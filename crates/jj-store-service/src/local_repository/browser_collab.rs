//! Bounded current-companion presentation using shared domain rows and local pins.
use super::*;
use crate::{
    collab_pages::{Items, Query},
    collab_reads::*,
};
use jj_collab::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LocalInboxSelection {
    pub snapshot: LocalCollabSnapshot,
    pub actor: Option<m::PrincipalId>,
}
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LocalInboxThreadDetail {
    pub snapshot: LocalCollabSnapshot,
    pub thread: ThreadSummaryV1,
}
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LocalInboxCollabPage {
    pub snapshot: LocalCollabSnapshot,
    pub items: Items,
    pub next_after: Option<String>,
}
fn bounded<T: Serialize>(value: &T) -> LocalResult<()> {
    if serde_json::to_vec(value)
        .map_err(|_| LocalError::Corrupt)?
        .len()
        > 4 * 1024 * 1024
    {
        return Err(LocalError::Limit);
    }
    Ok(())
}
impl LocalRepository {
    pub async fn inbox_selection(&self) -> LocalResult<LocalInboxSelection> {
        let pin = self.metadata.read()?;
        self.check_identity(pin.as_ref())?;
        Ok(LocalInboxSelection {
            snapshot: self.thread_store_at(pin.as_ref()).await?.snapshot,
            actor: self.actor()?,
        })
    }
    async fn inbox_threads(
        &self,
        expected: Option<MetadataCommit>,
    ) -> LocalResult<LocalThreadStore> {
        let pin = self.metadata.read()?;
        self.check_identity(pin.as_ref())?;
        if expected != pin.as_ref().map(|p| p.commit()) {
            return Err(LocalError::Stale);
        }
        self.thread_store_at(pin.as_ref()).await
    }
    pub async fn inbox_thread(
        &self,
        expected: Option<MetadataCommit>,
        id: ThreadId,
    ) -> LocalResult<LocalInboxThreadDetail> {
        let store = self.inbox_threads(expected).await?;
        let reference = *store.state.threads.get(&id).ok_or(LocalError::NotFound)?;
        let CollabObjectV1::ThreadVersion(value) = store.object(reference)? else {
            return Err(LocalError::Corrupt);
        };
        if value.status == ThreadStatusV1::Tombstoned {
            return Err(LocalError::NotFound);
        }
        Ok(LocalInboxThreadDetail {
            snapshot: store.snapshot,
            thread: ThreadSummaryV1 {
                reference,
                value,
                directory: None,
            },
        })
    }
    pub async fn inbox_page(
        &self,
        expected: Option<MetadataCommit>,
        query: &Query,
        after: Option<String>,
    ) -> LocalResult<LocalInboxCollabPage> {
        let limit = match query {
            Query::Threads { limit }
            | Query::Events { limit, .. }
            | Query::Emojis { limit }
            | Query::Reactions { limit, .. } => *limit,
        };
        if !(1..=256).contains(&limit) {
            return Err(LocalError::Invalid);
        }
        let store = self.inbox_threads(expected).await?;
        let snapshot = store.snapshot.clone();
        let (items, next_after) = match query {
            Query::Threads { .. } => {
                let after: Option<ThreadId> = after
                    .map(|a| a.parse().map_err(|_| LocalError::Invalid))
                    .transpose()?;
                let mut threads = Vec::new();
                for (id, reference) in &store.state.threads {
                    if after.is_some_and(|a| *id <= a) {
                        continue;
                    }
                    let CollabObjectV1::ThreadVersion(value) = store.object(*reference)? else {
                        return Err(LocalError::Corrupt);
                    };
                    if value.status == ThreadStatusV1::Tombstoned {
                        continue;
                    }
                    threads.push(ThreadSummaryV1 {
                        reference: *reference,
                        value,
                        directory: None,
                    });
                    if threads.len() > limit as usize {
                        break;
                    }
                }
                let next = (threads.len() > limit as usize)
                    .then(|| threads[limit as usize - 1].value.thread_id.to_string());
                threads.truncate(limit as usize);
                (Items::Threads { threads }, next)
            }
            Query::Events { thread_id, .. } => {
                let reference = *store
                    .state
                    .threads
                    .get(thread_id)
                    .ok_or(LocalError::NotFound)?;
                let CollabObjectV1::ThreadVersion(thread) = store.object(reference)? else {
                    return Err(LocalError::Corrupt);
                };
                if thread.status == ThreadStatusV1::Tombstoned {
                    return Err(LocalError::NotFound);
                }
                let wanted: Option<CollabObjectRef> = after
                    .map(|s| serde_json::from_str(&s).map_err(|_| LocalError::Invalid))
                    .transpose()?;
                let mut cursor = Some(thread.event);
                let mut seen = BTreeSet::new();
                while wanted.is_some() && cursor != wanted {
                    let current = cursor.ok_or(LocalError::Stale)?;
                    if !seen.insert(current) || seen.len() > 4096 {
                        return Err(LocalError::Limit);
                    }
                    let CollabObjectV1::ThreadEvent(event) = store.object(current)? else {
                        return Err(LocalError::Corrupt);
                    };
                    if event.thread_id != *thread_id {
                        return Err(LocalError::Corrupt);
                    }
                    cursor = event.previous;
                }
                let mut events = Vec::new();
                while let Some(reference) = cursor {
                    if events.len() == limit as usize {
                        break;
                    }
                    if !seen.insert(reference) || seen.len() > 4096 {
                        return Err(LocalError::Limit);
                    }
                    let CollabObjectV1::ThreadEvent(value) = store.object(reference)? else {
                        return Err(LocalError::Corrupt);
                    };
                    if value.thread_id != *thread_id {
                        return Err(LocalError::Corrupt);
                    }
                    let message = if let ThreadEventKindV1::Message { revision } = value.event {
                        let CollabObjectV1::HumanMessageRevision(message) =
                            store.object(revision)?
                        else {
                            return Err(LocalError::Corrupt);
                        };
                        if message.thread_id != *thread_id {
                            return Err(LocalError::Corrupt);
                        }
                        Some(message)
                    } else {
                        None
                    };
                    let link = if let ThreadEventKindV1::Link { version } = value.event {
                        let CollabObjectV1::ThreadLink(link) = store.object(version)? else {
                            return Err(LocalError::Corrupt);
                        };
                        if link.thread_id != *thread_id {
                            return Err(LocalError::Corrupt);
                        }
                        Some(link)
                    } else {
                        None
                    };
                    cursor = value.previous;
                    events.push(ThreadEventItemV1 {
                        reference,
                        value,
                        message,
                        link,
                    });
                }
                (
                    Items::Events {
                        thread_id: *thread_id,
                        events,
                    },
                    cursor
                        .map(|r| serde_json::to_string(&r).map_err(|_| LocalError::Corrupt))
                        .transpose()?,
                )
            }
            Query::Emojis { .. } => {
                let pin = self.metadata.read()?;
                if expected != pin.as_ref().map(|p| p.commit()) {
                    return Err(LocalError::Stale);
                }
                let repository = pin
                    .as_ref()
                    .map_or(m::RepositoryId::new([0; 16]), |p| p.repository());
                let reactions = self.reaction_store_at(pin.as_ref(), repository).await?;
                let after = after
                    .map(|a| a.parse().map_err(|_| LocalError::Invalid))
                    .transpose()?;
                let page = reactions.emojis(after, limit)?;
                let versions = page
                    .definitions
                    .iter()
                    .map(|d| {
                        Ok((
                            d.emoji_id,
                            CollabObjectV1::CustomEmojiVersion(d.clone()).reference()?,
                        ))
                    })
                    .collect::<LocalResult<BTreeMap<_, _>>>()?;
                (
                    Items::Emojis {
                        definitions: page.definitions,
                        versions,
                    },
                    page.next_id.map(|i| i.to_string()),
                )
            }
            Query::Reactions { target, .. } => {
                let after: Option<m::PrincipalId> = after
                    .map(|a| a.parse().map_err(|_| LocalError::Invalid))
                    .transpose()?;
                let viewer = self.actor()?;
                let mut cursor = None;
                let mut actors = Vec::new();
                let mut aggregates = BTreeMap::<ReactionValueV1, (u32, bool)>::new();
                let mut conflict_count = 0;
                let mut revision = None;
                for _ in 0..16 {
                    let page = self.reaction_query(target, cursor, 256, expected).await?;
                    revision = Some(page.revision);
                    for row in page.rows {
                        if row.conflict {
                            conflict_count += 1;
                        }
                        if let Some(value) = &row.value {
                            let a = aggregates.entry(value.clone()).or_default();
                            a.0 += 1;
                            a.1 |= Some(row.actor) == viewer;
                        }
                        if after.is_none_or(|a| row.actor > a) {
                            actors.push(ReactionActorRowV1 {
                                actor: row.actor,
                                active: row.active,
                                value: row.value,
                                conflict: row.conflict,
                            });
                        }
                    }
                    cursor = page.next_actor;
                    if cursor.is_none() {
                        break;
                    }
                }
                if cursor.is_some() {
                    return Err(LocalError::Limit);
                }
                let next = (actors.len() > limit as usize)
                    .then(|| actors[limit as usize - 1].actor.to_string());
                actors.truncate(limit as usize);
                let reactions = aggregates
                    .into_iter()
                    .map(|(value, (count, viewer_reacted))| ReactionAggregateV1 {
                        value,
                        count,
                        viewer_reacted,
                    })
                    .collect();
                (
                    Items::Reactions {
                        target: target.clone(),
                        revision: revision.ok_or(LocalError::Corrupt)?,
                        reactions,
                        conflict_count,
                        actors,
                    },
                    next,
                )
            }
        };
        let result = LocalInboxCollabPage {
            snapshot,
            items,
            next_after,
        };
        bounded(&result)?;
        // A concurrent companion mutation cannot silently change an active read pin.
        self.inbox_threads(expected).await?;
        Ok(result)
    }
}
impl LocalRepository {
    pub async fn inbox_asset(&self, id: CollabObjectId) -> LocalResult<ImageAssetV1> {
        let pin = self.metadata.read()?;
        self.check_identity(pin.as_ref())?;
        let expected = pin.as_ref().map(|p| p.commit());
        let repository = pin
            .as_ref()
            .map_or(m::RepositoryId::new([0; 16]), |p| p.repository());
        let reactions = self.reaction_store_at(pin.as_ref(), repository).await?;
        let CollabObjectV1::ImageAsset(asset) = reactions.object(CollabObjectRef {
            kind: CollabObjectKind::ImageAsset,
            id,
        })?
        else {
            return Err(LocalError::Corrupt);
        };
        self.inbox_threads(expected).await?;
        Ok(asset)
    }
}
