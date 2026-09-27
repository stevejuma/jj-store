//! Manifest-pinned application reads. This capability exposes no publication API.
use crate::native::NativeSession;
use jj_store_model as m;
use jj_store_wal::{self as wal, publication::Manifest};
use serde::Serialize;
use std::{collections::BTreeSet, sync::Arc, time::Duration};
use tokio::time::{Instant, timeout_at};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("read is not authorized")]
    Denied,
    #[error("object is not in the selected committed inventory")]
    NotCommitted,
    #[error("repository epoch changed; select a new snapshot")]
    EpochChanged,
    #[error("read session resource limit exceeded")]
    Limit,
    #[error("read authority is unavailable")]
    Unavailable,
}
pub type Result<T> = std::result::Result<T, Error>;
impl From<wal::Error> for Error {
    fn from(error: wal::Error) -> Self {
        match error {
            wal::Error::Denied => Self::Denied,
            wal::Error::Limit(_) => Self::Limit,
            _ => Self::Unavailable,
        }
    }
}
/// Request-local bounds, capped by the trusted native session configuration.
#[derive(Clone, Copy)]
pub struct ReadLimits {
    pub max_objects: usize,
    pub timeout: Duration,
}
impl Default for ReadLimits {
    fn default() -> Self {
        Self {
            max_objects: 100_000,
            timeout: Duration::from_secs(30),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct HeadSnapshot {
    pub version: u32,
    pub repo_id: m::RepositoryId,
    pub epoch: m::EpochId,
    pub published_seq: m::WalPosition,
    pub manifest_revision: u64,
    pub heads: BTreeSet<m::OperationId>,
}
impl From<&Manifest> for HeadSnapshot {
    fn from(manifest: &Manifest) -> Self {
        Self {
            version: 1,
            repo_id: manifest.repository,
            epoch: manifest.epoch,
            published_seq: manifest.published,
            manifest_revision: manifest.revision,
            heads: manifest.heads.clone(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ReadSnapshot {
    #[serde(flatten)]
    pub authority: HeadSnapshot,
    pub operation_id: m::OperationId,
}
/// Ambiguity is a response, never an invitation to reconcile during a read.
pub enum OpenRead {
    SelectHead(HeadSnapshot),
    Pinned(CommittedRead),
}
/// Private construction binds a checked inventory and exact operation to one
/// authorized request. The DTO is information, not a deserializable capability.
pub struct CommittedRead {
    session: Arc<NativeSession>,
    snapshot: ReadSnapshot,
    inventory: BTreeSet<m::ObjectKey>,
    deadline: Instant,
}
impl NativeSession {
    pub async fn open_read(
        self: &Arc<Self>,
        operation: Option<m::OperationId>,
        limits: ReadLimits,
    ) -> Result<OpenRead> {
        if limits.max_objects == 0
            || limits.max_objects > self.limits.inventory_nodes
            || limits.timeout.is_zero()
            || limits.timeout > self.limits.timeout
        {
            return Err(Error::Limit);
        }
        let deadline = Instant::now()
            .checked_add(limits.timeout)
            .ok_or(Error::Limit)?;
        within_deadline(deadline, async {
            // snapshot() verifies current permission both before and after
            // reading the manifest; no object/catalog read precedes it.
            let base = self.snapshot().await?;
            let manifest = base.manifest();
            let selected = match operation {
                Some(id) => id,
                None if manifest.heads.len() == 1 => *manifest.heads.first().unwrap(),
                None => return Ok(OpenRead::SelectHead(manifest.into())),
            };
            let inventory: BTreeSet<_> = self
                .coordinator
                .inventory(&base, limits.max_objects)
                .await?
                .into_iter()
                .map(|node| node.key)
                .collect();
            let read = CommittedRead {
                session: self.clone(),
                snapshot: ReadSnapshot {
                    authority: manifest.into(),
                    operation_id: selected,
                },
                inventory,
                deadline,
            };
            read.authorize_object(m::ObjectKey::Operation(selected))
                .await?;
            Ok(OpenRead::Pinned(read))
        })
        .await
    }
}
impl CommittedRead {
    /// Check current policy and epoch without replacing the captured snapshot.
    /// Later publications in the same epoch do not change this read's view.
    pub async fn authorize(&self) -> Result<()> {
        if Instant::now() >= self.deadline {
            return Err(Error::Limit);
        }
        within_deadline(self.deadline, async {
            let current = self.session.snapshot().await?;
            if current.manifest().epoch != self.snapshot.authority.epoch {
                return Err(Error::EpochChanged);
            }
            Ok(())
        })
        .await
    }
    pub async fn snapshot(&self) -> Result<&ReadSnapshot> {
        self.authorize().await?;
        Ok(&self.snapshot)
    }
    /// Guard before loading an object and again before returning its result.
    /// Knowing an ID or uploading it cannot make it part of this read session.
    pub async fn authorize_object(&self, key: m::ObjectKey) -> Result<()> {
        self.authorize().await?;
        if !self.inventory.contains(&key) {
            return Err(Error::NotCommitted);
        }
        Ok(())
    }
}

// Tokio polls the operation before its timer. Enforce both boundaries so an
// immediately ready storage response cannot succeed after an elapsed deadline.
async fn within_deadline<T>(
    deadline: Instant,
    future: impl std::future::Future<Output = Result<T>>,
) -> Result<T> {
    if Instant::now() >= deadline {
        return Err(Error::Limit);
    }
    let result = timeout_at(deadline, future)
        .await
        .map_err(|_| Error::Limit)?;
    if Instant::now() >= deadline {
        return Err(Error::Limit);
    }
    result
}

impl CommittedRead {
    /// Execute through the owned JJ boundary; the caller supplies trusted cache
    /// and worker limits, never user-selected local paths or store credentials.
    pub async fn graph(
        self: &Arc<Self>,
        executor: &jj_store_jj::query::GraphExecutor,
        request: jj_store_jj::query::Request,
        page: jj_store_jj::query::Page,
    ) -> jj_store_jj::query::Result<jj_store_jj::query::GraphPage> {
        self.authorize().await.map_err(query_error)?;
        let result = executor
            .execute(
                self.session.object_store.clone(),
                self.clone(),
                request,
                page,
            )
            .await?;
        self.authorize().await.map_err(query_error)?;
        Ok(result)
    }
}
fn query_error(error: Error) -> jj_store_jj::query::Error {
    use jj_store_jj::query::Error as Q;
    match error {
        Error::Denied => Q::Denied,
        Error::EpochChanged => Q::EpochChanged,
        Error::NotCommitted => Q::NotCommitted,
        Error::Limit => Q::Limit,
        Error::Unavailable => Q::Unavailable,
    }
}
#[async_trait::async_trait]
impl jj_store_jj::query::ReadAccess for CommittedRead {
    fn repository(&self) -> m::RepositoryId {
        self.snapshot.authority.repo_id
    }
    fn epoch(&self) -> m::EpochId {
        self.snapshot.authority.epoch
    }
    fn operation(&self) -> m::OperationId {
        self.snapshot.operation_id
    }
    fn inventory(&self) -> &BTreeSet<m::ObjectKey> {
        &self.inventory
    }
    async fn authorize(&self) -> jj_store_jj::query::Result<()> {
        self.authorize().await.map_err(query_error)
    }
}
