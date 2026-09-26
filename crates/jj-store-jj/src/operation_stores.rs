//! Pinned native operation traits over checked objects and request authority.
use crate::{
    codec::NativeCodec as _,
    objects::{NativeObjects, UploadStatus},
    roots,
};
use jj_lib::{
    object_id::{HexPrefix, ObjectId as _, PrefixResolution},
    op_heads_store::{OpHeadsStore, OpHeadsStoreError, OpHeadsStoreLock},
    op_store::{self as o, OpStore, OpStoreError, OpStoreResult},
};
use jj_store_model as m;
use std::{
    collections::BTreeSet,
    fmt,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

pub type AuthorityError = Box<dyn std::error::Error + Send + Sync>;
pub type AuthorityResult<T> = Result<T, AuthorityError>;
/// Each implementation is bound to one authenticated repository session. Native
/// types stay in this crate; transports and services implement the owned-ID API.
#[async_trait::async_trait]
pub trait OperationAuthority: Send + Sync {
    async fn authorize_read(&self) -> AuthorityResult<()>;
    async fn authorize_write(&self) -> AuthorityResult<()>;
    async fn heads(&self) -> AuthorityResult<BTreeSet<m::OperationId>>;
    /// Return every retained published operation, or a limit error. Implementors
    /// must not silently truncate or infer commitment by listing object keys.
    async fn operation_ids(&self, limit: usize) -> AuthorityResult<BTreeSet<m::OperationId>>;
    /// Stable request/publication identities belong to this authority's context.
    /// Success means a selected durable receipt, never merely a staged candidate.
    async fn publish(
        &self,
        old: BTreeSet<m::OperationId>,
        new: m::OperationId,
    ) -> AuthorityResult<()>;
}
#[derive(Clone, Copy)]
pub struct OperationLimits {
    pub max_operations: usize,
    pub timeout: Duration,
}
impl Default for OperationLimits {
    fn default() -> Self {
        Self {
            max_operations: 100_000,
            timeout: Duration::from_secs(30),
        }
    }
}
pub struct NativeOpStore {
    objects: Arc<NativeObjects>,
    authority: Arc<dyn OperationAuthority>,
    root: o::OperationId,
    staged: Mutex<BTreeSet<m::OperationId>>,
    limits: OperationLimits,
}
impl fmt::Debug for NativeOpStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeOpStore").finish_non_exhaustive()
    }
}
impl NativeOpStore {
    pub fn new(
        objects: Arc<NativeObjects>,
        authority: Arc<dyn OperationAuthority>,
        limits: OperationLimits,
    ) -> AuthorityResult<Self> {
        if limits.max_operations == 0 || limits.timeout.is_zero() {
            return Err("invalid operation limits".into());
        }
        Ok(Self {
            objects,
            authority,
            root: o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes()),
            staged: Mutex::new(BTreeSet::new()),
            limits,
        })
    }
    async fn resolve_prefix(
        &self,
        prefix: &HexPrefix,
    ) -> OpStoreResult<PrefixResolution<o::OperationId>> {
        self.authority
            .authorize_read()
            .await
            .map_err(OpStoreError::Other)?;
        let mut ids = self
            .authority
            .operation_ids(self.limits.max_operations)
            .await
            .map_err(OpStoreError::Other)?;
        if ids.len() > self.limits.max_operations {
            return Err(op_error("operation inventory limit"));
        }
        ids.insert(roots::ROOT_OPERATION);
        let staged = self
            .staged
            .lock()
            .map_err(|_| op_error("staged operation lock poisoned"))?;
        ids.extend(staged.iter().copied());
        if ids.len() > self.limits.max_operations {
            return Err(op_error("operation inventory limit"));
        }
        let mut found = None;
        for id in ids {
            let id = o::OperationId::from_bytes(id.as_bytes());
            if prefix.matches(&id) {
                if found.is_some() {
                    return Ok(PrefixResolution::AmbiguousMatch);
                }
                found = Some(id);
            }
        }
        Ok(found.map_or(PrefixResolution::NoMatch, PrefixResolution::SingleMatch))
    }
}
fn op_error(message: &'static str) -> OpStoreError {
    OpStoreError::Other(message.into())
}
fn stored(status: UploadStatus) -> OpStoreResult<()> {
    match status {
        UploadStatus::Stored | UploadStatus::AlreadyPresent => Ok(()),
        UploadStatus::Unknown => Err(op_error(
            "object upload outcome unknown; publication has not been acknowledged",
        )),
    }
}
#[async_trait::async_trait]
impl OpStore for NativeOpStore {
    fn name(&self) -> &str {
        "jj-store"
    }
    fn root_operation_id(&self) -> &o::OperationId {
        &self.root
    }
    async fn read_view(&self, id: &o::ViewId) -> OpStoreResult<o::View> {
        self.authority
            .authorize_read()
            .await
            .map_err(OpStoreError::Other)?;
        let id = m::ViewId::from_slice(id.as_bytes()).map_err(|e| OpStoreError::Other(e.into()))?;
        if id == roots::ROOT_VIEW {
            return Ok(roots::view());
        }
        self.objects
            .get(m::ObjectKey::View(id))
            .await
            .map_err(|e| OpStoreError::Other(e.into()))
    }
    async fn write_view(&self, contents: &o::View) -> OpStoreResult<o::ViewId> {
        self.authority
            .authorize_write()
            .await
            .map_err(OpStoreError::Other)?;
        let (key, status) = self
            .objects
            .put(contents)
            .await
            .map_err(|e| OpStoreError::Other(e.into()))?;
        stored(status)?;
        Ok(o::ViewId::from_bytes(key.as_bytes()))
    }
    async fn read_operation(&self, id: &o::OperationId) -> OpStoreResult<o::Operation> {
        self.authority
            .authorize_read()
            .await
            .map_err(OpStoreError::Other)?;
        let id =
            m::OperationId::from_slice(id.as_bytes()).map_err(|e| OpStoreError::Other(e.into()))?;
        if id == roots::ROOT_OPERATION {
            return Ok(roots::operation());
        }
        self.objects
            .get(m::ObjectKey::Operation(id))
            .await
            .map_err(|e| OpStoreError::Other(e.into()))
    }
    async fn write_operation(&self, contents: &o::Operation) -> OpStoreResult<o::OperationId> {
        self.authority
            .authorize_write()
            .await
            .map_err(OpStoreError::Other)?;
        {
            let staged = self
                .staged
                .lock()
                .map_err(|_| op_error("staged operation lock poisoned"))?;
            let id = m::OperationId::from_slice(contents.key().as_bytes())
                .map_err(|e| OpStoreError::Other(e.into()))?;
            if staged.len() >= self.limits.max_operations && !staged.contains(&id) {
                return Err(op_error("staged operation limit"));
            }
        }
        let (key, status) = self
            .objects
            .put(contents)
            .await
            .map_err(|e| OpStoreError::Other(e.into()))?;
        stored(status)?;
        let id = m::OperationId::from_slice(key.as_bytes())
            .map_err(|e| OpStoreError::Other(e.into()))?;
        let mut staged = self
            .staged
            .lock()
            .map_err(|_| op_error("staged operation lock poisoned"))?;
        if staged.len() >= self.limits.max_operations && !staged.contains(&id) {
            return Err(op_error("staged operation limit"));
        }
        staged.insert(id);
        Ok(o::OperationId::from_bytes(id.as_bytes()))
    }
    async fn resolve_operation_id_prefix(
        &self,
        prefix: &HexPrefix,
    ) -> OpStoreResult<PrefixResolution<o::OperationId>> {
        tokio::time::timeout(self.limits.timeout, self.resolve_prefix(prefix))
            .await
            .map_err(|_| op_error("operation prefix query deadline"))?
    }
    async fn gc(&self, _: &[o::OperationId], _: SystemTime) -> OpStoreResult<()> {
        Err(op_error("destructive native operation GC is disabled"))
    }
}
pub struct NativeOpHeadsStore {
    authority: Arc<dyn OperationAuthority>,
}
impl NativeOpHeadsStore {
    pub fn new(authority: Arc<dyn OperationAuthority>) -> Self {
        Self { authority }
    }
}
impl fmt::Debug for NativeOpHeadsStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeOpHeadsStore").finish_non_exhaustive()
    }
}
struct CasCoordination;
impl OpHeadsStoreLock for CasCoordination {}
#[async_trait::async_trait]
impl OpHeadsStore for NativeOpHeadsStore {
    fn name(&self) -> &str {
        "jj-store"
    }
    async fn get_op_heads(&self) -> Result<Vec<o::OperationId>, OpHeadsStoreError> {
        self.authority
            .authorize_read()
            .await
            .map_err(OpHeadsStoreError::Read)?;
        let ids = self
            .authority
            .heads()
            .await
            .map_err(OpHeadsStoreError::Read)?;
        if ids.is_empty() || ids.len() > 256 {
            return Err(OpHeadsStoreError::Read(
                "invalid native operation heads".into(),
            ));
        }
        Ok(ids
            .into_iter()
            .map(|id| o::OperationId::from_bytes(id.as_bytes()))
            .collect())
    }
    async fn update_op_heads(
        &self,
        old: &[o::OperationId],
        new: &o::OperationId,
    ) -> Result<(), OpHeadsStoreError> {
        let error = |source| OpHeadsStoreError::Write {
            new_op_id: new.clone(),
            source,
        };
        self.authority.authorize_write().await.map_err(error)?;
        if old.len() > 256 {
            return Err(error("native head limit".into()));
        }
        let old = old
            .iter()
            .map(|id| m::OperationId::from_slice(id.as_bytes()))
            .collect::<Result<BTreeSet<_>, _>>()
            .map_err(|e| error(e.into()))?;
        let id = m::OperationId::from_slice(new.as_bytes()).map_err(|e| error(e.into()))?;
        self.authority.publish(old, id).await.map_err(error)
    }
    async fn lock(&self) -> Result<Box<dyn OpHeadsStoreLock + '_>, OpHeadsStoreError> {
        Ok(Box::new(CasCoordination))
    }
}
