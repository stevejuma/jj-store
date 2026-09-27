//! Manifest-pinned application reads. This capability exposes no publication API.
use crate::native::NativeSession;
// Only project-owned contracts cross this transport-facing service boundary.
pub use jj_store_jj::{factories::PROFILE as NATIVE_PROFILE, query};
use jj_store_model as m;
pub use jj_store_object::ByteRange;
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
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
    inventory: Arc<BTreeSet<m::ObjectKey>>,
    deadline: Instant,
}
/// Immutable filtering information, never authorization. Only a checked native
/// read constructs it; retaining it retains neither a credential nor a session.
#[derive(Clone)]
pub struct VisibleView {
    snapshot: ReadSnapshot,
    commits: Arc<BTreeSet<m::CommitId>>,
}
impl VisibleView {
    pub fn snapshot(&self) -> &ReadSnapshot {
        &self.snapshot
    }
    pub fn commits(&self) -> &BTreeSet<m::CommitId> {
        &self.commits
    }
}
/// Only checked read sessions create these; no public or deserializable wire API.
/// A retained pin is credential-free and must be rebound to a fresh session.
#[derive(Clone)]
pub(crate) struct ReadPin {
    pub snapshot: ReadSnapshot,
    pub inventory: Arc<BTreeSet<m::ObjectKey>>,
}
impl ReadPin {
    pub async fn resume(
        &self,
        session: Arc<NativeSession>,
        limits: ReadLimits,
    ) -> Result<Arc<CommittedRead>> {
        if session.repository != self.snapshot.authority.repo_id {
            return Err(Error::Denied);
        }
        if self.inventory.len() > limits.max_objects {
            return Err(Error::Limit);
        }
        let deadline = session.read_deadline(limits)?;
        let read = Arc::new(CommittedRead {
            session,
            snapshot: self.snapshot.clone(),
            inventory: self.inventory.clone(),
            deadline,
        });
        read.authorize().await?;
        Ok(read)
    }
}
impl NativeSession {
    fn read_deadline(&self, limits: ReadLimits) -> Result<Instant> {
        if limits.max_objects == 0
            || limits.max_objects > self.limits.inventory_nodes
            || limits.timeout.is_zero()
            || limits.timeout > self.limits.timeout
        {
            return Err(Error::Limit);
        }
        Instant::now()
            .checked_add(limits.timeout)
            .ok_or(Error::Limit)
    }
    pub async fn open_read(
        self: &Arc<Self>,
        operation: Option<m::OperationId>,
        limits: ReadLimits,
    ) -> Result<OpenRead> {
        let deadline = self.read_deadline(limits)?;
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
                inventory: Arc::new(inventory),
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
    pub(crate) async fn pin(&self) -> Result<ReadPin> {
        self.authorize().await?;
        Ok(ReadPin {
            snapshot: self.snapshot.clone(),
            inventory: self.inventory.clone(),
        })
    }
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
    pub async fn search_visibility(
        self: &Arc<Self>,
        reader: &query::content::ContentReader,
    ) -> query::Result<VisibleView> {
        self.authorize().await.map_err(query_error)?;
        let commits = reader
            .visible_commits(self.session.object_store.clone(), self.clone())
            .await?;
        self.authorize().await.map_err(query_error)?;
        Ok(VisibleView {
            snapshot: self.snapshot.clone(),
            commits: Arc::new(commits),
        })
    }
    pub async fn prepare_index(
        self: &Arc<Self>,
        executor: &jj_store_jj::query::GraphExecutor,
        cancellation: jj_store_jj::query::Cancellation,
    ) -> jj_store_jj::query::Result<jj_store_jj::query::IndexReport> {
        self.authorize().await.map_err(query_error)?;
        let result = executor
            .prepare_index(
                self.session.object_store.clone(),
                self.clone(),
                cancellation,
            )
            .await?;
        self.authorize().await.map_err(query_error)?;
        Ok(result)
    }
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

impl CommittedRead {
    pub async fn object(
        self: &Arc<Self>,
        reader: &jj_store_jj::query::content::ContentReader,
        key: m::ObjectKey,
    ) -> jj_store_jj::query::Result<jj_store_jj::query::content::Object> {
        self.authorize().await.map_err(query_error)?;
        let value = reader
            .object(self.session.object_store.clone(), self.clone(), key)
            .await?;
        self.authorize().await.map_err(query_error)?;
        Ok(value)
    }
    pub async fn file(
        self: &Arc<Self>,
        reader: &jj_store_jj::query::content::ContentReader,
        id: m::FileId,
        range: Option<jj_store_object::ByteRange>,
    ) -> jj_store_jj::query::Result<jj_store_jj::query::content::VerifiedFile> {
        self.authorize().await.map_err(query_error)?;
        let file = reader
            .file(self.session.object_store.clone(), self.clone(), id, range)
            .await?;
        self.authorize().await.map_err(query_error)?;
        Ok(file)
    }
    pub async fn diff(
        self: &Arc<Self>,
        reader: &jj_store_jj::query::content::ContentReader,
        request: jj_store_jj::query::content::diff::Request,
    ) -> jj_store_jj::query::Result<jj_store_jj::query::content::diff::Response> {
        self.authorize().await.map_err(query_error)?;
        let result = reader
            .diff(self.session.object_store.clone(), self.clone(), request)
            .await?;
        self.authorize().await.map_err(query_error)?;
        Ok(result)
    }
    pub async fn conflicts(
        self: &Arc<Self>,
        reader: &jj_store_jj::query::content::ContentReader,
        request: jj_store_jj::query::content::conflicts::Request,
    ) -> jj_store_jj::query::Result<jj_store_jj::query::content::conflicts::Response> {
        self.authorize().await.map_err(query_error)?;
        let result = reader
            .conflicts(self.session.object_store.clone(), self.clone(), request)
            .await?;
        self.authorize().await.map_err(query_error)?;
        Ok(result)
    }
    pub async fn operation_diff(
        self: &Arc<Self>,
        reader: &jj_store_jj::query::content::ContentReader,
        request: jj_store_jj::query::content::operations::Request,
    ) -> jj_store_jj::query::Result<jj_store_jj::query::content::operations::Response> {
        self.authorize().await.map_err(query_error)?;
        let result = reader
            .operation_diff(self.session.object_store.clone(), self.clone(), request)
            .await?;
        self.authorize().await.map_err(query_error)?;
        Ok(result)
    }
    pub async fn references(
        self: &Arc<Self>,
        reader: &jj_store_jj::query::content::ContentReader,
        request: jj_store_jj::query::content::references::Request,
    ) -> jj_store_jj::query::Result<jj_store_jj::query::content::references::Page> {
        self.authorize().await.map_err(query_error)?;
        let result = reader
            .references(self.session.object_store.clone(), self.clone(), request)
            .await?;
        self.authorize().await.map_err(query_error)?;
        Ok(result)
    }
    pub async fn paths(
        self: &Arc<Self>,
        reader: &jj_store_jj::query::content::ContentReader,
        request: jj_store_jj::query::content::paths::Request,
    ) -> jj_store_jj::query::Result<jj_store_jj::query::content::paths::Page> {
        self.authorize().await.map_err(query_error)?;
        let result = reader
            .paths(self.session.object_store.clone(), self.clone(), request)
            .await?;
        self.authorize().await.map_err(query_error)?;
        Ok(result)
    }
    pub async fn operation_changes(
        self: &Arc<Self>,
        reader: &jj_store_jj::query::content::ContentReader,
        request: jj_store_jj::query::content::operation_changes::Request,
    ) -> jj_store_jj::query::Result<jj_store_jj::query::content::operation_changes::Page> {
        self.authorize().await.map_err(query_error)?;
        let result = reader
            .operation_changes(self.session.object_store.clone(), self.clone(), request)
            .await?;
        self.authorize().await.map_err(query_error)?;
        Ok(result)
    }
    pub async fn diff_summaries(
        self: &Arc<Self>,
        reader: &jj_store_jj::query::content::ContentReader,
        request: jj_store_jj::query::content::diff_summary::Request,
    ) -> jj_store_jj::query::Result<jj_store_jj::query::content::diff_summary::Page> {
        self.authorize().await.map_err(query_error)?;
        let result = reader
            .diff_summaries(self.session.object_store.clone(), self.clone(), request)
            .await?;
        self.authorize().await.map_err(query_error)?;
        Ok(result)
    }
    pub async fn tree(
        self: &Arc<Self>,
        reader: &jj_store_jj::query::content::ContentReader,
        request: jj_store_jj::query::content::TreeRequest,
    ) -> jj_store_jj::query::Result<jj_store_jj::query::content::TreePage> {
        self.authorize().await.map_err(query_error)?;
        let value = reader
            .tree(self.session.object_store.clone(), self.clone(), request)
            .await?;
        self.authorize().await.map_err(query_error)?;
        Ok(value)
    }
}
