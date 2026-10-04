//! Native commit storage; every data operation is bound to repository authority.
use crate::{
    codec::{NativeCodec, NativeSymlink},
    objects::{NativeObjects, UploadStatus},
    operation_stores::AuthorityResult,
    roots,
};
use futures::{AsyncRead, stream::BoxStream};
use jj_lib::{
    backend::*,
    index::Index,
    object_id::ObjectId,
    repo_path::{RepoPath, RepoPathBuf},
};
use jj_store_model as m;
use std::{
    collections::BTreeSet,
    fmt,
    future::Future as _,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::{Duration, SystemTime},
};
use tokio::sync::Semaphore;
use tokio_util::compat::{FuturesAsyncReadCompatExt as _, TokioAsyncReadCompatExt as _};
mod staged_copies;
use staged_copies::StagedCopies;

/// Bound to one authenticated repository session; no implicit permissive mode.
#[async_trait::async_trait]
pub trait BackendAuthority: Send + Sync {
    async fn authorize_read(&self) -> AuthorityResult<()>;
    /// Optional request policy for logical tree traversal, independent of codec
    /// support. Ordinary native sessions retain their existing read permission.
    async fn authorize_tree_read(&self, _path: &str) -> AuthorityResult<()> {
        self.authorize_read().await
    }
    async fn authorize_write(&self) -> AuthorityResult<()>;
    /// Complete retained, manifest-selected copy inventory, or an explicit error.
    async fn copy_ids(&self, limit: usize) -> AuthorityResult<BTreeSet<m::CopyId>>;
    async fn copy_family(
        &self,
        _seeds: &BTreeSet<m::CopyId>,
        _required: &BTreeSet<m::CopyId>,
        _limits: crate::query::copy_index::LookupLimits,
        _deadline: tokio::time::Instant,
    ) -> AuthorityResult<Option<crate::query::copy_index::Family>> {
        Ok(None)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct BackendLimits {
    pub concurrency: usize,
    pub max_nodes: usize,
    pub max_work: usize,
    pub max_bytes: usize,
    pub timeout: Duration,
}
impl Default for BackendLimits {
    fn default() -> Self {
        Self {
            concurrency: 16,
            max_nodes: 100_000,
            max_work: 1_000_000,
            max_bytes: 64 * 1024 * 1024,
            timeout: Duration::from_secs(30),
        }
    }
}
pub struct NativeBackend {
    runtime: crate::runtime::RuntimeContext,
    objects: Arc<NativeObjects>,
    pub(crate) authority: Arc<dyn BackendAuthority>,
    pub(crate) limits: BackendLimits,
    permits: Semaphore,
    pub(crate) staged_copies: Mutex<StagedCopies>,
    root: CommitId,
    root_change: ChangeId,
    empty: TreeId,
}
impl fmt::Debug for NativeBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeBackend").finish_non_exhaustive()
    }
}
impl NativeBackend {
    pub fn new(
        objects: Arc<NativeObjects>,
        authority: Arc<dyn BackendAuthority>,
        limits: BackendLimits,
    ) -> AuthorityResult<Self> {
        if limits.concurrency == 0
            || limits.concurrency > 256
            || limits.max_nodes == 0
            || limits.max_work == 0
            || limits.max_bytes == 0
            || limits.timeout.is_zero()
        {
            return Err("invalid backend limits".into());
        }
        Ok(Self {
            runtime: crate::runtime::RuntimeContext::current()?,
            objects,
            authority,
            limits,
            permits: Semaphore::new(limits.concurrency),
            staged_copies: Mutex::new(StagedCopies::default()),
            root: CommitId::from_bytes(roots::ROOT_COMMIT.as_bytes()),
            root_change: ChangeId::from_bytes(roots::ROOT_CHANGE.as_bytes()),
            empty: roots::empty_tree_id(),
        })
    }
    async fn read<T: NativeCodec>(&self, key: m::ObjectKey) -> BackendResult<T> {
        self.objects.get(key).await.map_err(|e| {
            let object_type = format!("{:?}", key.kind());
            let hash = key
                .as_bytes()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect();
            if matches!(
                e,
                crate::objects::Error::Store(jj_store_object::Error::NotFound)
            ) {
                BackendError::ObjectNotFound {
                    object_type,
                    hash,
                    source: e.into(),
                }
            } else {
                BackendError::ReadObject {
                    object_type,
                    hash,
                    source: e.into(),
                }
            }
        })
    }
    async fn write<T: NativeCodec>(&self, value: &T) -> BackendResult<m::ObjectKey> {
        let (key, status) = self.objects.put(value).await.map_err(other)?;
        acknowledged(status)?;
        Ok(key)
    }
}
pub(crate) fn other(e: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> BackendError {
    BackendError::Other(e.into())
}
pub(crate) fn id_bytes(id: &impl ObjectId, kind: &'static str) -> BackendResult<[u8; 64]> {
    id.as_bytes()
        .try_into()
        .map_err(|_| BackendError::InvalidHashLength {
            expected: 64,
            actual: id.as_bytes().len(),
            object_type: kind.to_owned(),
            hash: id.hex(),
        })
}
fn acknowledged(status: UploadStatus) -> BackendResult<()> {
    match status {
        UploadStatus::Stored | UploadStatus::AlreadyPresent => Ok(()),
        UploadStatus::Unknown => Err(other(
            "object upload outcome unknown; native write not acknowledged",
        )),
    }
}
/// Keep the private spool alive until the caller drops the stream, on all OSes.
struct FileReader {
    runtime: crate::runtime::RuntimeContext,
    reader: Pin<Box<dyn AsyncRead + Send>>,
    _spool: jj_store_object::StagedUpload,
}
impl AsyncRead for FileReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let runtime = self.runtime.clone();
        // JJ may read returned files through its foreign synchronous executor.
        // This reader needs the same per-poll runtime/cooperative boundary as
        // native futures and copy streams, including genuine Pending I/O.
        std::pin::pin!(runtime.run(std::future::poll_fn(|cx| {
            self.reader.as_mut().poll_read(cx, buf)
        })))
        .poll(cx)
    }
}

#[async_trait::async_trait]
impl Backend for NativeBackend {
    fn name(&self) -> &str {
        "jj-store"
    }
    fn commit_id_length(&self) -> usize {
        64
    }
    fn change_id_length(&self) -> usize {
        16
    }
    fn root_commit_id(&self) -> &CommitId {
        &self.root
    }
    fn root_change_id(&self) -> &ChangeId {
        &self.root_change
    }
    fn empty_tree_id(&self) -> &TreeId {
        &self.empty
    }
    fn concurrency(&self) -> usize {
        self.limits.concurrency
    }

    async fn read_file(
        &self,
        _path: &RepoPath,
        id: &FileId,
    ) -> BackendResult<Pin<Box<dyn AsyncRead + Send>>> {
        self.runtime
            .run(async {
                let _permit = self.permits.acquire().await.map_err(other)?;
                self.authority.authorize_read().await.map_err(other)?;
                let spool = self
                    .objects
                    .get_file(m::FileId::new(id_bytes(id, "file")?))
                    .await
                    .map_err(other)?;
                let reader = Box::pin(spool.reader().map_err(other)?.compat());
                Ok(Box::pin(FileReader {
                    runtime: self.runtime.clone(),
                    reader,
                    _spool: spool,
                }) as Pin<Box<dyn AsyncRead + Send>>)
            })
            .await
    }
    async fn write_file(
        &self,
        _path: &RepoPath,
        contents: &mut (dyn AsyncRead + Send + Unpin),
    ) -> BackendResult<FileId> {
        self.runtime
            .run(async {
                let _permit = self.permits.acquire().await.map_err(other)?;
                self.authority.authorize_write().await.map_err(other)?;
                let (id, status) = self
                    .objects
                    .put_file(contents.compat())
                    .await
                    .map_err(other)?;
                acknowledged(status)?;
                Ok(FileId::from_bytes(id.as_bytes()))
            })
            .await
    }
    async fn read_symlink(&self, _path: &RepoPath, id: &SymlinkId) -> BackendResult<String> {
        self.runtime
            .run(async {
                let _permit = self.permits.acquire().await.map_err(other)?;
                self.authority.authorize_read().await.map_err(other)?;
                Ok(self
                    .read::<NativeSymlink>(m::ObjectKey::Symlink(m::SymlinkId::new(id_bytes(
                        id, "symlink",
                    )?)))
                    .await?
                    .0)
            })
            .await
    }
    async fn write_symlink(&self, _path: &RepoPath, target: &str) -> BackendResult<SymlinkId> {
        self.runtime
            .run(async {
                let _permit = self.permits.acquire().await.map_err(other)?;
                self.authority.authorize_write().await.map_err(other)?;
                Ok(SymlinkId::from_bytes(
                    self.write(&NativeSymlink(target.to_owned()))
                        .await?
                        .as_bytes(),
                ))
            })
            .await
    }
    async fn read_copy(&self, id: &CopyId) -> BackendResult<CopyHistory> {
        self.runtime
            .run(async {
                let _permit = self.permits.acquire().await.map_err(other)?;
                self.authority.authorize_read().await.map_err(other)?;
                self.read(m::ObjectKey::Copy(m::CopyId::new(id_bytes(id, "copy")?)))
                    .await
            })
            .await
    }
    async fn write_copy(&self, copy: &CopyHistory) -> BackendResult<CopyId> {
        self.runtime
            .run(async {
                let _permit = self.permits.acquire().await.map_err(other)?;
                self.authority.authorize_write().await.map_err(other)?;
                let id = m::CopyId::from_slice(copy.key().as_bytes()).map_err(other)?;
                let parents = copy
                    .parents
                    .iter()
                    .filter(|id| !id.as_bytes().is_empty())
                    .map(|id| id_bytes(id, "copy").map(m::CopyId::new))
                    .collect::<BackendResult<BTreeSet<_>>>()?;
                {
                    let staged = self
                        .staged_copies
                        .lock()
                        .map_err(|_| other("copy lock poisoned"))?;
                    staged.admit(id, &parents, self.limits)?;
                }
                let key = self.write(copy).await?;
                let mut staged = self
                    .staged_copies
                    .lock()
                    .map_err(|_| other("copy lock poisoned"))?;
                staged.insert(id, parents, self.limits)?;
                Ok(CopyId::from_bytes(key.as_bytes()))
            })
            .await
    }
    async fn get_related_copies(&self, id: &CopyId) -> BackendResult<Vec<RelatedCopy>> {
        self.runtime
            .run(async {
                let deadline = tokio::time::Instant::now() + self.limits.timeout;
                tokio::time::timeout_at(deadline, crate::copy_query::related(self, id, deadline))
                    .await
                    .map_err(|_| other("copy query deadline exceeded"))?
            })
            .await
    }
    async fn read_tree(&self, path: &RepoPath, id: &TreeId) -> BackendResult<Tree> {
        self.runtime
            .run(async {
                let _permit = self.permits.acquire().await.map_err(other)?;
                self.authority
                    .authorize_tree_read(path.as_internal_file_string())
                    .await
                    .map_err(other)?;
                let key = m::ObjectKey::Tree(m::TreeId::new(id_bytes(id, "tree")?));
                if *id == self.empty {
                    return Ok(Tree::default());
                }
                self.read(key).await
            })
            .await
    }
    async fn write_tree(&self, _path: &RepoPath, contents: &Tree) -> BackendResult<TreeId> {
        self.runtime
            .run(async {
                let _permit = self.permits.acquire().await.map_err(other)?;
                self.authority.authorize_write().await.map_err(other)?;
                Ok(TreeId::from_bytes(self.write(contents).await?.as_bytes()))
            })
            .await
    }
    async fn read_commit(&self, id: &CommitId) -> BackendResult<Commit> {
        self.runtime
            .run(async {
                let _permit = self.permits.acquire().await.map_err(other)?;
                self.authority.authorize_read().await.map_err(other)?;
                let key = m::ObjectKey::Commit(m::CommitId::new(id_bytes(id, "commit")?));
                if *id == self.root {
                    return Ok(roots::commit());
                }
                self.read(key).await
            })
            .await
    }
    async fn write_commit(
        &self,
        mut contents: Commit,
        sign_with: Option<&mut SigningFn>,
    ) -> BackendResult<(CommitId, Commit)> {
        self.runtime
            .run(async {
                let _permit = self.permits.acquire().await.map_err(other)?;
                self.authority.authorize_write().await.map_err(other)?;
                if contents.parents.is_empty() {
                    return Err(other("only the virtual root may have no parents"));
                }
                if let Some(sign) = sign_with {
                    if contents.secure_sig.is_some() {
                        return Err(other("commit already has a signature"));
                    }
                    let mut data = b"jj-store/native-commit-signature/v1\0".to_vec();
                    data.extend(contents.encode_payload().map_err(other)?);
                    let sig = sign(&data).map_err(other)?;
                    contents.secure_sig = Some(SecureSig { data, sig });
                }
                let key = self.write(&contents).await?;
                Ok((CommitId::from_bytes(key.as_bytes()), contents))
            })
            .await
    }
    fn get_copy_records(
        &self,
        paths: Option<&[RepoPathBuf]>,
        root: &CommitId,
        head: &CommitId,
    ) -> BackendResult<BoxStream<'_, BackendResult<CopyRecord>>> {
        id_bytes(root, "commit")?;
        id_bytes(head, "commit")?;
        crate::copy_query::records(self, paths, root, head)
            .map(|stream| self.runtime.stream(stream))
    }
    fn gc(&self, _index: &dyn Index, _keep_newer: SystemTime) -> BackendResult<()> {
        Err(BackendError::Unsupported(
            "destructive GC is disabled".to_owned(),
        ))
    }
}

#[cfg(test)]
mod runtime_reader_tests {
    use super::*;
    struct BudgetedReader {
        ready: bool,
    }
    impl AsyncRead for BudgetedReader {
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<std::io::Result<usize>> {
            if !self.ready {
                return Poll::Pending;
            }
            let mut budget = std::pin::pin!(tokio::task::consume_budget());
            std::task::ready!(budget.as_mut().poll(cx));
            buf[0] = 7;
            Poll::Ready(Ok(1))
        }
    }
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn native_file_reader_progress_does_not_require_foreign_executor_cooperative_reset() {
        let spool = jj_store_object::StagedUpload::from_bytes(b"", 0)
            .await
            .unwrap();
        let mut reader = Box::pin(FileReader {
            runtime: crate::runtime::RuntimeContext::current().unwrap(),
            reader: Box::pin(BudgetedReader { ready: true }),
            _spool: spool,
        });
        let mut context = Context::from_waker(futures::task::noop_waker_ref());
        let mut bytes = [0];
        for _ in 0..512 {
            assert!(matches!(
                reader.as_mut().poll_read(&mut context, &mut bytes),
                Poll::Ready(Ok(1))
            ));
            assert_eq!(bytes, [7]);
        }
        reader.as_mut().get_mut().reader = Box::pin(BudgetedReader { ready: false });
        assert!(
            reader
                .as_mut()
                .poll_read(&mut context, &mut bytes)
                .is_pending()
        );
    }
}
