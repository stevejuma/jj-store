use super::*;
mod selection;
use crate::{
    backend::BackendAuthority,
    factories::NativeAuthority,
    operation_stores::{AuthorityResult, OperationAuthority},
};
use jj_store_object::{
    self as storage, ByteRange, Condition, Download, Key, Metadata, StagedUpload, WriteOutcome,
};
pub(super) use selection::select_operation;
use std::{
    future::Future as _,
    io,
    pin::Pin,
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
    task::{Context, Poll},
};
use tokio::{
    io::{AsyncRead, AsyncReadExt as _, ReadBuf},
    time::{Instant, Sleep},
};

use tokio_util::sync::{CancellationToken, WaitForCancellationFutureOwned};

#[cfg(test)]
mod kinds_tests;

pub(crate) struct Budget {
    pub cancellation: CancellationToken,
    pub limits: Limits,
    pub deadline: Instant,
    reads: AtomicU64,
    metadata_nodes: AtomicU64,
    predicate_bytes: AtomicU64,
    predicate_lines: AtomicU64,
    bytes: AtomicU64,
    work: AtomicU64,
    output: AtomicU64,
    failure: Mutex<Option<Error>>,
    index: Mutex<Option<super::index::Admission>>,
    construction_memory: Mutex<Option<Vec<super::memory::Lease>>>,
    pub(super) index_memory: Arc<super::memory::Indexes>,
}
impl Budget {
    pub fn new(limits: Limits) -> Result<Self> {
        Ok(Self {
            cancellation: CancellationToken::new(),
            limits,
            deadline: Instant::now()
                .checked_add(limits.timeout)
                .ok_or(Error::Limit)?,
            reads: AtomicU64::new(0),
            metadata_nodes: AtomicU64::new(0),
            predicate_bytes: AtomicU64::new(0),
            predicate_lines: AtomicU64::new(0),
            bytes: AtomicU64::new(0),
            work: AtomicU64::new(0),
            output: AtomicU64::new(0),
            failure: Mutex::new(None),
            index: Mutex::new(None),
            construction_memory: Mutex::new(None),
            index_memory: Arc::default(),
        })
    }
    pub fn fail(&self, error: Error) -> Error {
        let mut failure = self.failure.lock().expect("query failure lock");
        failure.get_or_insert(error).clone()
    }
    pub fn failure(&self) -> Option<Error> {
        self.failure.lock().expect("query failure lock").clone()
    }
    pub fn check(&self) -> Result<()> {
        if self.cancellation.is_cancelled() {
            return Err(self.fail(Error::Canceled));
        }
        if let Some(error) = self.failure() {
            return Err(error);
        }
        if Instant::now() >= self.deadline {
            return Err(self.fail(Error::Limit));
        }
        Ok(())
    }
    fn charge(&self, counter: &AtomicU64, amount: u64, limit: u64) -> Result<()> {
        self.check()?;
        counter
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
                n.checked_add(amount).filter(|n| *n <= limit)
            })
            .map_err(|_| self.fail(Error::Limit))?;
        Ok(())
    }
    fn predicate_file(&self, framed_bytes: u64) -> Result<()> {
        let bytes = framed_bytes
            .checked_sub(m::envelope::HEADER_LEN as u64)
            .ok_or_else(|| self.fail(Error::Corrupt))?;
        if bytes > self.limits.predicates.max_file_bytes {
            return Err(self.fail(Error::Limit));
        }
        self.charge(
            &self.predicate_bytes,
            bytes,
            self.limits.predicates.max_total_file_bytes,
        )?;
        self.predicate_lines(1)
    }
    fn predicate_lines(&self, added: u64) -> Result<()> {
        self.check()?;
        let old = self
            .predicate_lines
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
                n.checked_add(added)
                    .filter(|n| *n <= self.limits.predicates.max_lines)
            })
            .map_err(|_| self.fail(Error::Limit))?;
        let total = old + added;
        self.work((total * total - old * old) as usize)
    }
    pub fn metadata_node(&self) -> Result<()> {
        self.charge(&self.metadata_nodes, 1, self.limits.metadata.max_nodes)
    }
    pub fn work(&self, amount: usize) -> Result<()> {
        self.charge(&self.work, amount as u64, self.limits.max_work as u64)
    }
    pub fn read_bytes(&self, amount: usize) -> Result<()> {
        self.charge(&self.bytes, amount as u64, self.limits.max_object_bytes)
    }
    pub fn start_code_index(&self) -> Result<()> {
        *self.construction_memory.lock().unwrap() = Some(vec![]);
        self.start_index(false)
    }
    pub fn start_index(&self, inline: bool) -> Result<()> {
        *self.index.lock().expect("index admission") = Some(super::index::Admission::new(inline));
        // The virtual root is indexed without a physical object-store read.
        self.index_commit(crate::roots::ROOT_COMMIT)
    }
    pub fn finish_initial_index(&self) {
        if let Some(admission) = self.index.lock().expect("index admission").as_mut() {
            admission.finish_initial_load();
        }
    }
    fn index_commit(&self, id: m::CommitId) -> Result<()> {
        self.check()?;
        if let Some(admission) = self.index.lock().expect("index admission").as_mut() {
            if admission.is_new(id)
                && admission.len() % 256 == 0
                && let Some(leases) = self.construction_memory.lock().unwrap().as_mut()
            {
                leases.push(super::memory::Lease::reserve(256 * 4096).map_err(|e| self.fail(e))?);
            }
            admission.admit(id, self.limits).map_err(|e| self.fail(e))?;
        }
        Ok(())
    }
    pub async fn until<T>(&self, future: impl std::future::Future<Output = T>) -> Result<T> {
        self.check()?;
        let result = tokio::select! {
            biased;
            _ = self.cancellation.cancelled() => Err(self.fail(Error::Canceled)),
            result = tokio::time::timeout_at(self.deadline, future) =>
                result.map_err(|_| self.fail(Error::Limit)),
        };
        self.check()?;
        result
    }
    pub async fn authorize(&self, access: &dyn ReadAccess) -> Result<()> {
        self.work(1)?;
        self.until(access.authorize())
            .await?
            .map_err(|error| self.fail(error))?;
        self.check()
    }
    pub fn output(&self, value: &impl Serialize) -> Result<()> {
        struct Sink<'a>(&'a Budget);
        impl io::Write for Sink<'_> {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                self.0
                    .charge(
                        &self.0.output,
                        bytes.len() as u64,
                        self.0.limits.max_output_bytes as u64,
                    )
                    .map_err(io::Error::other)?;
                Ok(bytes.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        serde_json::to_writer(Sink(self), value)
            .map_err(|_| self.failure().unwrap_or(Error::Unavailable))
    }
    pub fn complete_output(&self, value: &impl Serialize) -> Result<()> {
        // Incremental checks bound retained nodes; final accounting also includes
        // the response envelope and continuation, without charging nodes twice.
        self.output.store(0, Ordering::Relaxed);
        self.output(value)
    }
    pub fn stats(&self) -> Stats {
        Stats {
            metadata_nodes: self.metadata_nodes.load(Ordering::Relaxed),
            predicate_file_bytes: self.predicate_bytes.load(Ordering::Relaxed),
            predicate_lines: self.predicate_lines.load(Ordering::Relaxed),
            object_reads: self.reads.load(Ordering::Relaxed),
            object_bytes: self.bytes.load(Ordering::Relaxed),
            work: self.work.load(Ordering::Relaxed),
        }
    }
}
pub(crate) struct ReadOnlyAuthority {
    pub access: Arc<dyn ReadAccess>,
    pub budget: Arc<Budget>,
}
#[async_trait::async_trait]
impl BackendAuthority for ReadOnlyAuthority {
    async fn copy_family(
        &self,
        seeds: &BTreeSet<m::CopyId>,
        required: &BTreeSet<m::CopyId>,
        limits: copy_index::LookupLimits,
        deadline: Instant,
    ) -> AuthorityResult<Option<copy_index::Family>> {
        let deadline = deadline.min(self.budget.deadline);
        if Instant::now() >= deadline {
            return Err(self.budget.fail(Error::Limit).into());
        }
        self.budget.authorize(self.access.as_ref()).await?;
        let family = self
            .budget
            .until(async {
                tokio::time::timeout_at(
                    deadline,
                    self.access.copy_family(seeds, required, limits, deadline),
                )
                .await
                .map_err(|_| Error::Limit)?
            })
            .await?
            .map_err(|error| self.budget.fail(error))?;
        if let Some(family) = &family {
            if family.ids.len() > limits.max_nodes
                || family.work > limits.max_work
                || family
                    .ids
                    .len()
                    .checked_mul(512)
                    .is_none_or(|bytes| bytes > limits.max_bytes)
            {
                return Err(self.budget.fail(Error::Limit).into());
            }
            if !required.is_subset(&family.ids) {
                return Err(self.budget.fail(Error::NotCommitted).into());
            }
            self.budget.work(family.work)?;
        }
        self.budget.authorize(self.access.as_ref()).await?;
        if Instant::now() >= deadline {
            return Err(self.budget.fail(Error::Limit).into());
        }
        Ok(family)
    }
    async fn authorize_tree_read(&self, path: &str) -> AuthorityResult<()> {
        self.budget.authorize(self.access.as_ref()).await?;
        if path.len() > 4096 || path.split('/').filter(|name| !name.is_empty()).count() > 64 {
            return Err(self.budget.fail(Error::Limit).into());
        }
        Ok(())
    }
    async fn authorize_read(&self) -> AuthorityResult<()> {
        Ok(self.budget.authorize(self.access.as_ref()).await?)
    }
    async fn authorize_write(&self) -> AuthorityResult<()> {
        Err(Error::Denied.into())
    }
    async fn copy_ids(&self, limit: usize) -> AuthorityResult<BTreeSet<m::CopyId>> {
        self.budget.authorize(self.access.as_ref()).await?;
        let keys = self
            .budget
            .until(
                self.access
                    .inventory_kind(InventoryKind::Copies, limit, self.budget.deadline),
            )
            .await?
            .map_err(|error| self.budget.fail(error))?;
        let ids: BTreeSet<_> = keys
            .iter()
            .filter_map(|key| {
                if let m::ObjectKey::Copy(id) = key {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect();
        if ids.len() > limit {
            return Err(self.budget.fail(Error::Limit).into());
        }
        self.budget.authorize(self.access.as_ref()).await?;
        Ok(ids)
    }
}
#[async_trait::async_trait]
impl OperationAuthority for ReadOnlyAuthority {
    async fn authorize_read(&self) -> AuthorityResult<()> {
        BackendAuthority::authorize_read(self).await
    }
    async fn authorize_write(&self) -> AuthorityResult<()> {
        Err(Error::Denied.into())
    }
    async fn heads(&self) -> AuthorityResult<BTreeSet<m::OperationId>> {
        self.budget.authorize(self.access.as_ref()).await?;
        Ok(BTreeSet::from([self.access.operation()]))
    }
    async fn operation_ids(&self, limit: usize) -> AuthorityResult<BTreeSet<m::OperationId>> {
        self.budget.authorize(self.access.as_ref()).await?;
        let keys = self
            .budget
            .until(self.access.inventory_kind(
                InventoryKind::Operations,
                limit,
                self.budget.deadline,
            ))
            .await?
            .map_err(|error| self.budget.fail(error))?;
        let ids: BTreeSet<_> = keys
            .iter()
            .filter_map(|key| {
                if let m::ObjectKey::Operation(id) = key {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect();
        if ids.len() > limit {
            return Err(self.budget.fail(Error::Limit).into());
        }
        self.budget.authorize(self.access.as_ref()).await?;
        Ok(ids)
    }
    async fn publish(&self, _: BTreeSet<m::OperationId>, _: m::OperationId) -> AuthorityResult<()> {
        Err(Error::Denied.into())
    }
}
impl NativeAuthority for ReadOnlyAuthority {
    fn repository(&self) -> m::RepositoryId {
        self.access.repository()
    }
}
pub(crate) struct ReadStore {
    staged: Option<Arc<Mutex<BTreeSet<m::ObjectKey>>>>,
    predicate_files: bool,
    inner: Arc<dyn ObjectStore>,
    access: Arc<dyn ReadAccess>,
    budget: Arc<Budget>,
}
impl ReadStore {
    pub fn new(
        inner: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        budget: Arc<Budget>,
        predicate_files: bool,
    ) -> Self {
        Self {
            staged: None,
            predicate_files,
            inner,
            access,
            budget,
        }
    }
    pub(crate) fn with_staged(mut self, staged: Arc<Mutex<BTreeSet<m::ObjectKey>>>) -> Self {
        self.staged = Some(staged);
        self
    }
    async fn validate(&self, key: &Key) -> storage::Result<m::ObjectKey> {
        self.budget
            .authorize(self.access.as_ref())
            .await
            .map_err(|_| storage::Error::Invalid("query authorization"))?;
        let parts: Vec<_> = key.as_str().split('/').collect();
        let object = (|| {
            let ["repos", _, "objects", "v1", kind, _, id] = parts.as_slice() else {
                return None;
            };
            let id: m::CommitId = id.parse().ok()?;
            m::ObjectKey::from_parts(kind.parse().ok()?, *id.as_bytes())
        })()
        .filter(|id| id.storage_path(self.access.repository()) == key.as_str());
        let Some(object) = object else {
            self.budget.fail(Error::NotCommitted);
            return Err(storage::Error::Invalid("query inventory"));
        };
        let staged = self.staged.as_ref().is_some_and(|staged| {
            staged
                .lock()
                .expect("command staged objects")
                .contains(&object)
        });
        let committed = if staged {
            true
        } else {
            self.budget
                .until(self.access.contains(object))
                .await
                .and_then(|result| result)
                .map_err(|error| {
                    self.budget.fail(error);
                    storage::Error::Invalid("query inventory")
                })?
        };
        if !committed {
            self.budget.fail(Error::NotCommitted);
            return Err(storage::Error::Invalid("query inventory"));
        }
        Ok(object)
    }
}
#[async_trait::async_trait]
impl ObjectStore for ReadStore {
    async fn head(&self, key: &Key) -> storage::Result<Metadata> {
        self.validate(key).await?;
        let metadata = self
            .budget
            .until(self.inner.head(key))
            .await
            .map_err(|_| storage::Error::Limit)??;
        self.validate(key).await?;
        Ok(metadata)
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> storage::Result<Download> {
        let object = self.validate(key).await?;
        if let m::ObjectKey::Commit(id) = object {
            self.budget
                .index_commit(id)
                .map_err(|_| storage::Error::Limit)?;
        }
        if range.is_some() {
            return Err(storage::Error::Range);
        }
        let download = self
            .budget
            .until(self.inner.get(key, None))
            .await
            .map_err(|_| storage::Error::Limit)??;
        self.budget
            .charge(
                &self.budget.bytes,
                download.metadata.len,
                self.budget.limits.max_object_bytes,
            )
            .map_err(|_| storage::Error::Limit)?;
        let predicate_file = self.predicate_files && matches!(object, m::ObjectKey::File(_));
        if predicate_file {
            self.budget
                .predicate_file(download.metadata.len)
                .map_err(|_| storage::Error::Limit)?;
        }
        self.budget.reads.fetch_add(1, Ordering::Relaxed);
        let reader = DeadlineReader {
            predicate_file,
            header_remaining: m::envelope::HEADER_LEN,
            inner: Box::pin(
                download
                    .reader
                    .take(download.metadata.len.saturating_add(1)),
            ),
            remaining: download.metadata.len,
            sleep: Box::pin(tokio::time::sleep_until(self.budget.deadline)),
            cancelled: Box::pin(self.budget.cancellation.clone().cancelled_owned()),
            budget: self.budget.clone(),
        };
        let download = Download {
            metadata: download.metadata,
            reader: Box::pin(reader),
        };
        if matches!(object, m::ObjectKey::File(_)) {
            return Ok(download);
        }
        // Buffer bounded metadata once for a non-allocating collection scan.
        // NativeObjects still verifies the full canonical native identity.
        let metadata = download.metadata.clone();
        let bytes = download
            .read_bounded(crate::codec::MAX_METADATA_BYTES + m::envelope::HEADER_LEN as u64)
            .await?;
        super::admission::check(object, &bytes, &self.budget).map_err(|error| {
            if error == Error::Limit {
                storage::Error::Limit
            } else {
                storage::Error::Response("native metadata admission")
            }
        })?;
        Ok(Download {
            metadata,
            reader: Box::pin(std::io::Cursor::new(bytes)),
        })
    }
    async fn put(&self, _: &Key, _: Condition, _: &StagedUpload) -> storage::Result<WriteOutcome> {
        self.budget.fail(Error::Denied);
        Err(storage::Error::Invalid("read-only query"))
    }
}
struct DeadlineReader {
    predicate_file: bool,
    header_remaining: usize,
    inner: storage::Reader,
    remaining: u64,
    sleep: Pin<Box<Sleep>>,
    cancelled: Pin<Box<WaitForCancellationFutureOwned>>,
    budget: Arc<Budget>,
}
impl AsyncRead for DeadlineReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        if let Err(error) = self.budget.check() {
            return Poll::Ready(Err(io::Error::other(error)));
        }
        if self.cancelled.as_mut().poll(cx).is_ready() {
            return Poll::Ready(Err(io::Error::other(self.budget.fail(Error::Canceled))));
        }
        if self.sleep.as_mut().poll(cx).is_ready() {
            return Poll::Ready(Err(io::Error::other(self.budget.fail(Error::Limit))));
        }
        let before = buf.filled().len();
        let poll = self.inner.as_mut().poll_read(cx, buf);
        let size = (buf.filled().len() - before) as u64;
        if size > self.remaining {
            return Poll::Ready(Err(io::Error::other(self.budget.fail(Error::Unavailable))));
        }
        self.remaining -= size;
        if self.predicate_file {
            let skip = self.header_remaining.min(size as usize);
            self.header_remaining -= skip;
            let lines = buf.filled()[before + skip..]
                .iter()
                .filter(|&&b| b == b'\n')
                .count() as u64;
            if let Err(error) = self.budget.predicate_lines(lines) {
                return Poll::Ready(Err(io::Error::other(error)));
            }
        }
        if let Err(error) = self.budget.check() {
            return Poll::Ready(Err(io::Error::other(error)));
        }
        poll
    }
}
