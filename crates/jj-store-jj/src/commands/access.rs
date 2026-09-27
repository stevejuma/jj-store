use super::*;
use crate::{
    backend::BackendAuthority,
    factories::NativeAuthority,
    operation_stores::{AuthorityResult, OperationAuthority},
    query::access::{ReadOnlyAuthority, ReadStore},
};
use jj_store_object::{
    self as storage, ByteRange, Condition, Download, Key, Metadata, StagedUpload, WriteOutcome,
};
use std::{
    collections::BTreeSet,
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

pub(super) struct Access(pub Arc<dyn MutationAccess>);
#[async_trait::async_trait]
impl ReadAccess for Access {
    fn repository(&self) -> m::RepositoryId {
        self.0.repository()
    }
    fn epoch(&self) -> m::EpochId {
        self.0.epoch()
    }
    fn operation(&self) -> m::OperationId {
        self.0.operation()
    }
    fn inventory(&self) -> &BTreeSet<m::ObjectKey> {
        self.0.inventory()
    }
    async fn authorize(&self) -> query::Result<()> {
        self.0.authorize_write().await
    }
}
pub(super) struct Authority(pub ReadOnlyAuthority);
#[async_trait::async_trait]
impl BackendAuthority for Authority {
    async fn authorize_read(&self) -> AuthorityResult<()> {
        BackendAuthority::authorize_read(&self.0).await
    }
    async fn authorize_tree_read(&self, path: &str) -> AuthorityResult<()> {
        self.0.authorize_tree_read(path).await
    }
    async fn authorize_write(&self) -> AuthorityResult<()> {
        BackendAuthority::authorize_read(&self.0).await
    }
    async fn copy_ids(&self, limit: usize) -> AuthorityResult<BTreeSet<m::CopyId>> {
        self.0.copy_ids(limit).await
    }
}
#[async_trait::async_trait]
impl OperationAuthority for Authority {
    async fn authorize_read(&self) -> AuthorityResult<()> {
        OperationAuthority::authorize_read(&self.0).await
    }
    async fn authorize_write(&self) -> AuthorityResult<()> {
        OperationAuthority::authorize_read(&self.0).await
    }
    async fn heads(&self) -> AuthorityResult<BTreeSet<m::OperationId>> {
        self.0.heads().await
    }
    async fn operation_ids(&self, limit: usize) -> AuthorityResult<BTreeSet<m::OperationId>> {
        self.0.operation_ids(limit).await
    }
    async fn publish(&self, _: BTreeSet<m::OperationId>, _: m::OperationId) -> AuthorityResult<()> {
        Err(query::Error::Denied.into())
    }
}
impl NativeAuthority for Authority {
    fn repository(&self) -> m::RepositoryId {
        self.0.repository()
    }
}
pub(super) struct Store {
    read: ReadStore,
    inner: Arc<dyn ObjectStore>,
    access: Arc<dyn ReadAccess>,
    budget: Arc<Budget>,
    staged: Arc<Mutex<BTreeSet<m::ObjectKey>>>,
    bytes: AtomicU64,
    limits: Limits,
}
impl Store {
    pub fn new(
        inner: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        budget: Arc<Budget>,
        limits: Limits,
    ) -> Self {
        let staged = Arc::new(Mutex::new(BTreeSet::new()));
        Self {
            read: ReadStore::new(inner.clone(), access.clone(), budget.clone(), false)
                .with_staged(staged.clone()),
            inner,
            access,
            budget,
            staged,
            bytes: AtomicU64::new(0),
            limits,
        }
    }
    pub fn stats(&self) -> (usize, u64) {
        (
            self.staged.lock().expect("command staged objects").len(),
            self.bytes.load(Ordering::Relaxed),
        )
    }
}
#[async_trait::async_trait]
impl ObjectStore for Store {
    async fn head(&self, key: &Key) -> storage::Result<Metadata> {
        self.read.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> storage::Result<Download> {
        self.read.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> storage::Result<WriteOutcome> {
        self.budget
            .authorize(self.access.as_ref())
            .await
            .map_err(|_| storage::Error::Invalid("command authorization"))?;
        if !matches!(condition, Condition::Absent) {
            return Err(storage::Error::Invalid("command immutable staging"));
        }
        let parts: Vec<_> = key.as_str().split('/').collect();
        let object = (|| {
            let ["repos", _, "objects", "v1", kind, _, id] = parts.as_slice() else {
                return None;
            };
            let id: m::CommitId = id.parse().ok()?;
            m::ObjectKey::from_parts(kind.parse().ok()?, *id.as_bytes())
        })()
        .filter(|id| id.storage_path(self.access.repository()) == key.as_str())
        .ok_or(storage::Error::Invalid("command native key"))?;
        self.bytes
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |bytes| {
                bytes
                    .checked_add(upload.len())
                    .filter(|total| *total <= self.limits.max_staged_bytes)
            })
            .map_err(|_| {
                self.budget.fail(query::Error::Limit);
                storage::Error::Limit
            })?;
        {
            let mut staged = self.staged.lock().expect("command staged objects");
            if staged.len() >= self.limits.max_staged_objects && !staged.contains(&object) {
                self.budget.fail(query::Error::Limit);
                return Err(storage::Error::Limit);
            }
            // Also admit collision/unknown-outcome verification of our own exact
            // attempted native content. NativeObjects checks identity and bytes.
            staged.insert(object);
        }
        let result = self
            .budget
            .until(self.inner.put(key, condition, upload))
            .await
            .map_err(|_| storage::Error::Limit)??;
        self.budget
            .authorize(self.access.as_ref())
            .await
            .map_err(|_| storage::Error::Invalid("command authorization"))?;
        Ok(result)
    }
}
