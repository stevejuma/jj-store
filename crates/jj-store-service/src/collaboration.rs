//! Bounded, manifest-root-pinned collaboration reads.
mod agent_checkpoints;
mod agent_cursors;
mod agent_details;
pub(crate) use agent_details::load_activity;
mod agent_relations;
mod agent_session_timing;
mod agent_tool_timing;
mod opencode_timing;
pub(crate) use agent_session_timing::resolve as session_timing;
mod agent_transcript_timing;
pub(crate) use agent_transcript_timing::present_chunks;
mod agent_transcripts;
mod checkpoint_models;
mod cursor_position;
mod models;
pub(crate) use models::{summary as session_summary, transcript_descriptor};
pub(crate) mod review_activity;
mod review_cursors;
mod review_models;
mod review_presentation;
mod review_replication;
pub(crate) mod review_threads;
pub use agent_relations::AgentRelationRequest;
pub use agent_transcripts::AgentTranscriptRequest;
pub use checkpoint_models::*;
pub use models::*;
pub use review_models::*;

use crate::{
    auth::{Clock, Permission},
    native::NativeSession,
    pagination,
};
use agent_cursors::{AgentCursorRegistry, AgentPin};
use cursor_position::Position;
use jj_store_model as m;
use jj_store_wal::{
    self as wal,
    publication::{AgentCatalogKey, AgentObjectRef, AgentObjectV1, Domain},
};
use std::sync::Arc;

const MAX_PAGE: usize = 256;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("collaboration read is not authorized")]
    Denied,
    #[error("invalid collaboration cursor or changed page size")]
    InvalidCursor,
    #[error("collaboration object was not found in the selected root")]
    NotFound,
    #[error("collaboration cursor expired or its serving instance changed")]
    CursorReset,
    #[error("collaboration read resource limit exceeded")]
    Limit,
    #[error("collaboration read is busy")]
    Busy,
    #[error("committed collaboration state is corrupt")]
    Corrupt,
    #[error("collaboration read is unavailable")]
    Unavailable,
}
pub type Result<T> = std::result::Result<T, Error>;

pub struct AgentSessionPager {
    cursors: AgentCursorRegistry,
}

pub struct ReviewThreadPager {
    cursors: review_cursors::ReviewCursorRegistry,
}

impl std::fmt::Debug for ReviewThreadPager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReviewThreadPager")
            .field("cursors", &self.cursors)
            .finish()
    }
}

impl ReviewThreadPager {
    pub fn new(limits: pagination::Limits, clock: Arc<dyn Clock>) -> Result<Self> {
        Ok(Self {
            cursors: review_cursors::ReviewCursorRegistry::new(limits, clock)?,
        })
    }
}

impl std::fmt::Debug for AgentSessionPager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AgentSessionPager")
            .field("cursors", &self.cursors)
            .finish()
    }
}

impl AgentSessionPager {
    pub fn new(limits: pagination::Limits, clock: Arc<dyn Clock>) -> Result<Self> {
        Ok(Self {
            cursors: AgentCursorRegistry::new(limits, clock)?,
        })
    }

    pub async fn begin(
        &self,
        session: Arc<NativeSession>,
        limit: usize,
    ) -> Result<AgentSessionPage> {
        valid_limit(limit)?;
        let _permit = self.cursors.acquire()?;
        within_timeout(self.cursors.timeout(), self.begin_page(session, limit)).await
    }

    async fn begin_page(
        &self,
        session: Arc<NativeSession>,
        limit: usize,
    ) -> Result<AgentSessionPage> {
        let pin = self.pin(&session).await?;
        let snapshot = pin.snapshot.clone();
        let page = pin
            .store
            .list_sessions(None, limit)
            .await
            .map_err(map_wal)?;
        let mut sessions = resolve(&pin.store, page.selections).await?;
        enrich(session.clone(), &mut sessions).await?;
        authorize_epoch(&session, snapshot.epoch).await?;
        let next_cursor = page
            .next_after
            .map(|after| {
                self.cursors
                    .retain(pin, limit, Position::Sessions { after })
            })
            .transpose()?;
        Ok(AgentSessionPage {
            version: 1,
            snapshot,
            sessions,
            next_cursor,
        })
    }

    pub async fn resume(
        &self,
        session: Arc<NativeSession>,
        cursor: &str,
        limit: usize,
    ) -> Result<AgentSessionPage> {
        valid_limit(limit)?;
        let _permit = self.cursors.acquire()?;
        within_timeout(
            self.cursors.timeout(),
            self.resume_page(session, cursor, limit),
        )
        .await
    }

    async fn resume_page(
        &self,
        session: Arc<NativeSession>,
        cursor: &str,
        limit: usize,
    ) -> Result<AgentSessionPage> {
        authorize(&session).await?;
        let (claims, entry) = self.cursors.lookup(session.repository, cursor, limit)?;
        let Position::Sessions { after } = claims.position else {
            return Err(Error::InvalidCursor);
        };
        authorize_epoch(&session, entry.pin.snapshot.epoch).await?;
        let page = entry
            .pin
            .store
            .list_sessions(Some(after), limit)
            .await
            .map_err(map_wal)?;
        let mut sessions = resolve(&entry.pin.store, page.selections).await?;
        enrich(session.clone(), &mut sessions).await?;
        authorize_epoch(&session, entry.pin.snapshot.epoch).await?;
        let next_cursor = self.cursors.next(
            &claims,
            &entry,
            page.next_after.map(|after| Position::Sessions { after }),
        )?;
        Ok(AgentSessionPage {
            version: 1,
            snapshot: entry.pin.snapshot,
            sessions,
            next_cursor,
        })
    }
}

async fn resolve(
    store: &wal::publication::AgentStoreSnapshot,
    selections: Vec<(AgentCatalogKey, AgentObjectRef)>,
) -> Result<Vec<AgentSessionSummary>> {
    let mut rows = Vec::with_capacity(selections.len());
    for (key, reference) in selections {
        let AgentObjectV1::SessionSnapshot(session) =
            store.read_object(reference).await.map_err(map_wal)?
        else {
            return Err(Error::Corrupt);
        };
        if session.catalog_key().map_err(|_| Error::Corrupt)? != key {
            return Err(Error::Corrupt);
        }
        let timing = agent_session_timing::resolve(store, &session).await?;
        rows.push(models::summary(reference, &session, timing));
    }
    Ok(rows)
}

async fn resolve_checkpoint(
    store: &wal::publication::AgentStoreSnapshot,
    key: AgentCatalogKey,
    reference: AgentObjectRef,
) -> Result<AgentCheckpointSummary> {
    let AgentObjectV1::CheckpointVersion(checkpoint) =
        store.read_object(reference).await.map_err(map_wal)?
    else {
        return Err(Error::Corrupt);
    };
    let expected = AgentCatalogKey::derive("checkpoint", checkpoint.checkpoint_id.as_bytes())
        .map_err(|_| Error::Corrupt)?;
    if expected != key {
        return Err(Error::Corrupt);
    }
    Ok(checkpoint_models::summary(reference, &checkpoint))
}

impl AgentSessionPager {
    async fn pin(&self, session: &NativeSession) -> Result<AgentPin> {
        authorize(session).await?;
        let selected = session.snapshot().await.map_err(map_wal)?;
        let manifest = selected.manifest();
        let domain = manifest.domain_state(Domain::Agent).ok_or(Error::Corrupt)?;
        let pin = AgentPin {
            snapshot: AgentReadSnapshot {
                repo_id: manifest.repository,
                epoch: manifest.epoch,
                published_seq: manifest.published,
                manifest_revision: manifest.revision,
                agent_root: domain.root.digest,
                agent_state_version: domain.version,
            },
            store: session
                .coordinator
                .agent_store(domain.root)
                .await
                .map_err(map_wal)?,
        };
        self.cursors.validate_pin(&pin)?;
        Ok(pin)
    }
}

fn valid_limit(limit: usize) -> Result<()> {
    if (1..=MAX_PAGE).contains(&limit) {
        Ok(())
    } else {
        Err(Error::Limit)
    }
}

async fn authorize(session: &NativeSession) -> Result<()> {
    session
        .principal(Permission::Read)
        .await
        .map(|_| ())
        .map_err(map_wal)
}

async fn authorize_epoch(session: &NativeSession, epoch: m::EpochId) -> Result<()> {
    let current = session.snapshot().await.map_err(map_wal)?;
    if current.manifest().epoch != epoch {
        return Err(Error::CursorReset);
    }
    Ok(())
}

fn map_wal(error: wal::Error) -> Error {
    match error {
        wal::Error::Denied => Error::Denied,
        wal::Error::Limit(_) => Error::Limit,
        wal::Error::Corrupt(_) | wal::Error::Stale => Error::Corrupt,
        _ => Error::Unavailable,
    }
}

async fn within_timeout<T>(
    duration: std::time::Duration,
    future: impl std::future::Future<Output = Result<T>>,
) -> Result<T> {
    let deadline = tokio::time::Instant::now()
        .checked_add(duration)
        .ok_or(Error::Limit)?;
    if tokio::time::Instant::now() >= deadline {
        return Err(Error::Limit);
    }
    let result = tokio::time::timeout_at(deadline, future)
        .await
        .map_err(|_| Error::Limit)?;
    if tokio::time::Instant::now() >= deadline {
        return Err(Error::Limit);
    }
    result
}

async fn enrich(session: Arc<NativeSession>, rows: &mut [AgentSessionSummary]) -> Result<()> {
    let pin = crate::collab_reads::PinnedCollabRead::open(
        session.clone(),
        Arc::new(crate::collab_reads::RepositoryMemberReactionActors),
    )
    .await
    .map_err(|_| Error::Unavailable)?;
    let discovery = pin
        .conversation_discovery(session)
        .await
        .map_err(|_| Error::Unavailable)?;
    for row in rows {
        if let Some((title, text)) = discovery.get(&row.session_id) {
            row.conversation_title = title.clone();
            row.discovery_text = Some(text.clone());
        }
    }
    Ok(())
}
