//! Bounded, manifest-root-pinned collaboration reads.
mod agent_cursors;

use crate::{
    auth::{Clock, Permission},
    native::NativeSession,
    pagination,
};
use agent_cursors::{AgentCursorRegistry, AgentPin};
use jj_store_model as m;
use jj_store_wal::{
    self as wal,
    publication::{
        AgentCatalogKey, AgentObjectId, AgentObjectRef, AgentObjectV1, AgentSessionId,
        CompletenessV1, Domain, IntegrationId, SessionState,
    },
};
use schemars::JsonSchema;
use serde::Serialize;
use std::sync::Arc;

const MAX_PAGE: usize = 256;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("collaboration read is not authorized")]
    Denied,
    #[error("invalid collaboration cursor or changed page size")]
    InvalidCursor,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentReadSnapshot {
    pub repo_id: m::RepositoryId,
    pub epoch: m::EpochId,
    pub published_seq: m::WalPosition,
    pub manifest_revision: u64,
    pub agent_root: m::RecordDigest,
    pub agent_state_version: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentCoverage {
    pub completeness: CompletenessV1,
    pub gaps: u32,
    pub redacted_fields: u32,
    pub truncated: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentSessionSummary {
    pub session_id: AgentSessionId,
    pub snapshot_id: AgentObjectId,
    pub integration: IntegrationId,
    pub state: SessionState,
    pub parent_session_id: Option<AgentSessionId>,
    pub coverage: AgentCoverage,
    pub evidence_count: usize,
    pub event_batch_count: usize,
    pub transcript_count: usize,
    pub tool_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentSessionPage {
    pub version: u32,
    pub snapshot: AgentReadSnapshot,
    pub sessions: Vec<AgentSessionSummary>,
    pub next_cursor: Option<String>,
}

pub struct AgentSessionPager {
    cursors: AgentCursorRegistry,
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
        authorize(&session).await?;
        let selected = session.snapshot().await.map_err(map_wal)?;
        let manifest = selected.manifest();
        let domain = manifest.domain_state(Domain::Agent).ok_or(Error::Corrupt)?;
        let snapshot = AgentReadSnapshot {
            repo_id: manifest.repository,
            epoch: manifest.epoch,
            published_seq: manifest.published,
            manifest_revision: manifest.revision,
            agent_root: domain.root.digest,
            agent_state_version: domain.version,
        };
        let store = session
            .coordinator
            .agent_store(domain.root)
            .await
            .map_err(map_wal)?;
        let pin = AgentPin {
            snapshot: snapshot.clone(),
            store,
        };
        self.cursors.validate_pin(&pin)?;
        let page = pin
            .store
            .list_sessions(None, limit)
            .await
            .map_err(map_wal)?;
        let sessions = resolve(&pin.store, page.selections).await?;
        authorize_epoch(&session, snapshot.epoch).await?;
        let next_cursor = page
            .next_after
            .map(|after| self.cursors.retain(pin, limit, after))
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
        authorize_epoch(&session, entry.pin.snapshot.epoch).await?;
        let page = entry
            .pin
            .store
            .list_sessions(Some(claims.after), limit)
            .await
            .map_err(map_wal)?;
        let sessions = resolve(&entry.pin.store, page.selections).await?;
        authorize_epoch(&session, entry.pin.snapshot.epoch).await?;
        let next_cursor = self.cursors.next(&claims, &entry, page.next_after)?;
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
        rows.push(AgentSessionSummary {
            session_id: session.session_id,
            snapshot_id: reference.id,
            integration: session.integration,
            state: session.state,
            parent_session_id: session.parent_session_id,
            coverage: AgentCoverage {
                completeness: session.coverage.completeness,
                gaps: session.coverage.gaps,
                redacted_fields: session.coverage.redacted_fields,
                truncated: session.coverage.truncated,
            },
            evidence_count: session.evidence.len(),
            event_batch_count: session.event_batches.len(),
            transcript_count: session.transcripts.len(),
            tool_count: session.tools.len(),
        });
    }
    Ok(rows)
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
