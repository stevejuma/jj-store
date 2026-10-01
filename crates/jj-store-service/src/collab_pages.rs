//! Query-bound native collaboration pages over bounded credential-free pins.
mod cursors;
use crate::{
    auth::{Clock, Permission},
    collab_mutations,
    collab_reads::*,
    native::NativeSession,
};
use jj_collab::{
    CollabObjectRef, CustomEmojiId, CustomEmojiVersionV1, ReactionTargetV1, TargetRevisionV1,
    ThreadId,
};
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("invalid collaboration cursor or changed query")]
    InvalidCursor,
    #[error("collaboration cursor expired or serving instance changed; refresh explicitly")]
    CursorReset,
    #[error("collaboration pagination is busy")]
    Busy,
    #[error("collaboration pagination resource limit exceeded")]
    Limit,
    #[error("collaboration pagination is unavailable")]
    Unavailable,
    #[error(transparent)]
    Read(#[from] collab_mutations::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy)]
pub struct Limits {
    pub snapshots: usize,
    pub objects: usize,
    /// Sum of retained canonical Thread/Reaction graph bytes; membership sets
    /// for lazy Agent/Review bodies are separately bounded by objects.
    pub encoded_collab_bytes: usize,
    pub concurrency: usize,
    pub ttl: Duration,
    pub timeout: Duration,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            snapshots: 64,
            objects: 1_000_000,
            encoded_collab_bytes: 256 * 1024 * 1024,
            concurrency: 8,
            ttl: Duration::from_secs(300),
            timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "CollabQuery")]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Query {
    Threads {
        limit: u32,
    },
    Events {
        thread_id: ThreadId,
        limit: u32,
    },
    Reactions {
        target: ReactionTargetV1,
        limit: u32,
    },
    Emojis {
        limit: u32,
    },
}
impl Query {
    fn validate(&self) -> Result<()> {
        let limit = match self {
            Self::Threads { limit }
            | Self::Events { limit, .. }
            | Self::Reactions { limit, .. }
            | Self::Emojis { limit } => *limit,
        };
        if !(1..=256).contains(&limit) {
            return Err(collab_mutations::Error::Invalid.into());
        }
        if let Self::Reactions { target, .. } = self {
            target
                .validate()
                .map_err(|_| collab_mutations::Error::Invalid)?;
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "CollabPageItems")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Items {
    Threads {
        threads: Vec<ThreadSummaryV1>,
    },
    Events {
        thread_id: ThreadId,
        events: Vec<ThreadEventItemV1>,
    },
    Reactions {
        target: ReactionTargetV1,
        revision: TargetRevisionV1,
        reactions: Vec<ReactionAggregateV1>,
        conflict_count: u32,
        actors: Vec<ReactionActorRowV1>,
    },
    Emojis {
        versions: std::collections::BTreeMap<CustomEmojiId, CollabObjectRef>,
        definitions: Vec<CustomEmojiVersionV1>,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "CollabPage")]
pub struct Page {
    pub snapshot: CollabReadSnapshotV1,
    pub items: Items,
    pub next_cursor: Option<String>,
}

pub struct CollabPager {
    registry: cursors::Registry,
    actors: Arc<dyn ReactionActorVisibility>,
}
impl CollabPager {
    pub fn new(
        limits: Limits,
        clock: Arc<dyn Clock>,
        actors: Arc<dyn ReactionActorVisibility>,
    ) -> Result<Self> {
        Ok(Self {
            registry: cursors::Registry::new(limits, clock)?,
            actors,
        })
    }
    pub async fn page(
        &self,
        session: Arc<NativeSession>,
        query: Query,
        cursor: Option<&str>,
    ) -> Result<Page> {
        let _permit = self.registry.acquire()?;
        let timeout = self.registry.timeout().min(session.limits.timeout);
        let start = tokio::time::Instant::now();
        let result = tokio::time::timeout(timeout, self.page_inner(session, query, cursor))
            .await
            .map_err(|_| Error::Limit)?;
        if start.elapsed() >= timeout {
            return Err(Error::Limit);
        }
        match result {
            Err(Error::Read(collab_mutations::Error::Stale)) if cursor.is_some() => {
                Err(Error::CursorReset)
            }
            result => result,
        }
    }
    async fn page_inner(
        &self,
        session: Arc<NativeSession>,
        query: Query,
        cursor: Option<&str>,
    ) -> Result<Page> {
        query.validate()?;
        let viewer = session
            .principal(Permission::Read)
            .await
            .map_err(collab_mutations::Error::from)?
            .id();
        let (pin, after, claims) = if let Some(token) = cursor {
            let (claims, pin) = self
                .registry
                .lookup(session.repository, viewer, &query, token)?;
            (pin, Some(claims.after.clone()), Some(claims))
        } else {
            let pin = Arc::new(PinnedCollabRead::open(session.clone(), self.actors.clone()).await?);
            self.registry.validate_pin(&pin)?;
            (pin, None, None)
        };
        let (items, next) = execute(&pin, session.clone(), &query, after).await?;
        let next_cursor = match (claims, next) {
            (Some(claims), next) => self.registry.next(&claims, next)?,
            (None, Some(next)) => Some(self.registry.retain(pin.clone(), viewer, query, next)?),
            (None, None) => None,
        };
        let page = Page {
            snapshot: pin.snapshot().clone(),
            items,
            next_cursor,
        };
        if serde_json::to_vec(&page)
            .map_err(|_| Error::Unavailable)?
            .len()
            > 4 * 1024 * 1024
        {
            return Err(Error::Limit);
        }
        pin.authorize(&session).await.map_err(|error| {
            if error == collab_mutations::Error::Stale {
                Error::CursorReset
            } else {
                error.into()
            }
        })?;
        Ok(page)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum Position {
    Thread { after: ThreadId },
    Event { before: CollabObjectRef },
    Actor { after: m::PrincipalId },
    Emoji { after: CustomEmojiId },
}
async fn execute(
    pin: &PinnedCollabRead,
    session: Arc<NativeSession>,
    query: &Query,
    position: Option<Position>,
) -> Result<(Items, Option<Position>)> {
    let mismatch = || Error::InvalidCursor;
    match query {
        Query::Threads { limit } => {
            let after = match position {
                None => None,
                Some(Position::Thread { after }) => Some(after),
                _ => return Err(mismatch()),
            };
            let page = pin.threads(session, after, *limit).await?;
            Ok((
                Items::Threads {
                    threads: page.threads,
                },
                page.next_id.map(|after| Position::Thread { after }),
            ))
        }
        Query::Events { thread_id, limit } => {
            let before = match position {
                None => None,
                Some(Position::Event { before }) => Some(before),
                _ => return Err(mismatch()),
            };
            let page = pin.events(session, *thread_id, before, *limit).await?;
            Ok((
                Items::Events {
                    thread_id: *thread_id,
                    events: page.events,
                },
                page.next_event.map(|before| Position::Event { before }),
            ))
        }
        Query::Reactions { target, limit } => {
            let after = match position {
                None => None,
                Some(Position::Actor { after }) => Some(after),
                _ => return Err(mismatch()),
            };
            let page = pin.reactions(session, target, after, *limit).await?;
            Ok((
                Items::Reactions {
                    target: page.target,
                    revision: page.revision,
                    reactions: page.reactions,
                    conflict_count: page.conflict_count,
                    actors: page.actors,
                },
                page.next_actor.map(|after| Position::Actor { after }),
            ))
        }
        Query::Emojis { limit } => {
            let after = match position {
                None => None,
                Some(Position::Emoji { after }) => Some(after),
                _ => return Err(mismatch()),
            };
            let page = pin.emojis(session, after, *limit).await?;
            Ok((
                Items::Emojis {
                    versions: page.versions,
                    definitions: page.definitions,
                },
                page.next_id.map(|after| Position::Emoji { after }),
            ))
        }
    }
}
