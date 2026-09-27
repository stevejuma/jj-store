use super::*;
use crate::{
    backend::BackendAuthority,
    factories::NativeAuthority,
    operation_stores::{AuthorityResult, OperationAuthority},
};
use jj_store_object::{
    self as storage, ByteRange, Condition, Download, Key, Metadata, StagedUpload, WriteOutcome,
};
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

pub(super) struct Budget {
    pub limits: Limits,
    pub deadline: Instant,
    reads: AtomicU64,
    bytes: AtomicU64,
    work: AtomicU64,
    output: AtomicU64,
    failure: Mutex<Option<Error>>,
}
impl Budget {
    pub fn new(limits: Limits) -> Result<Self> {
        Ok(Self {
            limits,
            deadline: Instant::now()
                .checked_add(limits.timeout)
                .ok_or(Error::Limit)?,
            reads: AtomicU64::new(0),
            bytes: AtomicU64::new(0),
            work: AtomicU64::new(0),
            output: AtomicU64::new(0),
            failure: Mutex::new(None),
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
    pub fn work(&self, amount: usize) -> Result<()> {
        self.charge(&self.work, amount as u64, self.limits.max_work as u64)
    }
    pub async fn authorize(&self, access: &dyn ReadAccess) -> Result<()> {
        self.work(1)?;
        tokio::time::timeout_at(self.deadline, access.authorize())
            .await
            .map_err(|_| self.fail(Error::Limit))?
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
            object_reads: self.reads.load(Ordering::Relaxed),
            object_bytes: self.bytes.load(Ordering::Relaxed),
            work: self.work.load(Ordering::Relaxed),
        }
    }
}
pub(super) struct ReadOnlyAuthority {
    pub access: Arc<dyn ReadAccess>,
    pub budget: Arc<Budget>,
}
#[async_trait::async_trait]
impl BackendAuthority for ReadOnlyAuthority {
    async fn authorize_read(&self) -> AuthorityResult<()> {
        Ok(self.budget.authorize(self.access.as_ref()).await?)
    }
    async fn authorize_write(&self) -> AuthorityResult<()> {
        Err(Error::Denied.into())
    }
    async fn copy_ids(&self, limit: usize) -> AuthorityResult<BTreeSet<m::CopyId>> {
        self.budget.authorize(self.access.as_ref()).await?;
        let ids: BTreeSet<_> = self
            .access
            .inventory()
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
            return Err(Error::Limit.into());
        }
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
        let ids: BTreeSet<_> = self
            .access
            .inventory()
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
            return Err(Error::Limit.into());
        }
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
pub(super) struct ReadStore {
    inner: Arc<dyn ObjectStore>,
    access: Arc<dyn ReadAccess>,
    budget: Arc<Budget>,
}
impl ReadStore {
    pub fn new(
        inner: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        budget: Arc<Budget>,
    ) -> Self {
        Self {
            inner,
            access,
            budget,
        }
    }
    async fn validate(&self, key: &Key) -> storage::Result<()> {
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
        .filter(|id| {
            id.storage_path(self.access.repository()) == key.as_str()
                && self.access.inventory().contains(id)
        });
        if object.is_none() {
            self.budget.fail(Error::NotCommitted);
            return Err(storage::Error::Invalid("query inventory"));
        }
        Ok(())
    }
}
#[async_trait::async_trait]
impl ObjectStore for ReadStore {
    async fn head(&self, key: &Key) -> storage::Result<Metadata> {
        self.validate(key).await?;
        let metadata = tokio::time::timeout_at(self.budget.deadline, self.inner.head(key))
            .await
            .map_err(|_| {
                self.budget.fail(Error::Limit);
                storage::Error::Limit
            })??;
        self.validate(key).await?;
        Ok(metadata)
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> storage::Result<Download> {
        self.validate(key).await?;
        if range.is_some() {
            return Err(storage::Error::Range);
        }
        let download = tokio::time::timeout_at(self.budget.deadline, self.inner.get(key, None))
            .await
            .map_err(|_| {
                self.budget.fail(Error::Limit);
                storage::Error::Limit
            })??;
        self.budget
            .charge(
                &self.budget.bytes,
                download.metadata.len,
                self.budget.limits.max_object_bytes,
            )
            .map_err(|_| storage::Error::Limit)?;
        self.budget.reads.fetch_add(1, Ordering::Relaxed);
        let reader = DeadlineReader {
            inner: Box::pin(
                download
                    .reader
                    .take(download.metadata.len.saturating_add(1)),
            ),
            remaining: download.metadata.len,
            sleep: Box::pin(tokio::time::sleep_until(self.budget.deadline)),
            budget: self.budget.clone(),
        };
        Ok(Download {
            metadata: download.metadata,
            reader: Box::pin(reader),
        })
    }
    async fn put(&self, _: &Key, _: Condition, _: &StagedUpload) -> storage::Result<WriteOutcome> {
        self.budget.fail(Error::Denied);
        Err(storage::Error::Invalid("read-only query"))
    }
}
struct DeadlineReader {
    inner: storage::Reader,
    remaining: u64,
    sleep: Pin<Box<Sleep>>,
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
        if let Err(error) = self.budget.check() {
            return Poll::Ready(Err(io::Error::other(error)));
        }
        poll
    }
}
