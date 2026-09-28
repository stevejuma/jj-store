use crate::*;
use std::{
    future::Future,
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};
use tokio::{
    sync::{OwnedSemaphorePermit, Semaphore},
    time::Instant,
};

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub page: usize,
    pub inventory_nodes: usize,
    pub requests: u64,
    pub bytes: u64,
    pub object_bytes: u64,
    pub timeout: Duration,
    pub poll: Duration,
    pub max_backoff: Duration,
    /// Zero disables remote projection snapshots. Otherwise publish after at
    /// most this many committed source positions, plus one startup check.
    pub snapshot_every: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            page: 32,
            inventory_nodes: 100_000,
            requests: 100_000,
            bytes: 256 * 1024 * 1024,
            object_bytes: 16 * 1024 * 1024,
            timeout: Duration::from_secs(60),
            poll: Duration::from_secs(1),
            max_backoff: Duration::from_secs(30),
            snapshot_every: 0,
        }
    }
}
impl Limits {
    pub(crate) fn validate(&self) -> Result<()> {
        if !(1..=1024).contains(&self.page)
            || !(1..=1_000_000).contains(&self.inventory_nodes)
            || !(1..=1_000_000).contains(&self.requests)
            || self.bytes == 0
            || self.bytes > 2 * 1024 * 1024 * 1024
            || self.object_bytes == 0
            || self.object_bytes > self.bytes
            || self.timeout.is_zero()
            || self.timeout > Duration::from_secs(300)
            || self.poll < Duration::from_millis(10)
            || self.max_backoff < self.poll
            || self.max_backoff > Duration::from_secs(60)
            || self.snapshot_every > 1_000_000
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
}
pub(crate) struct Lease(std::fs::File);
impl Lease {
    pub(crate) fn acquire(root: &Path, repo: m::RepositoryId, name: &str) -> Result<Arc<Self>> {
        let directory = root.join("adapters").join(repo.to_string()).join(name);
        std::fs::create_dir_all(&directory).map_err(|_| Error::Unavailable)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700))
                .map_err(|_| Error::Unavailable)?;
        }
        let mut options = std::fs::OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        let file = options
            .open(directory.join("writer.lock"))
            .map_err(|_| Error::Unavailable)?;
        file.try_lock().map_err(|_| Error::Busy)?;
        Ok(Arc::new(Self(file)))
    }
}
impl Drop for Lease {
    fn drop(&mut self) {
        let _ = self.0.unlock();
    }
}
struct Work {
    _lease: Arc<Lease>,
    _permit: OwnedSemaphorePermit,
    requests: AtomicU64,
    bytes: AtomicU64,
    limits: Limits,
    deadline: Instant,
    cancel: CancellationToken,
}
/// Clones retained by blocking work also retain writer ownership and admission.
/// Projection workers must retain this context until all destination work exits.
#[derive(Clone)]
pub struct Context(Arc<Work>);
impl Context {
    pub(crate) fn new(
        lease: Arc<Lease>,
        slots: Arc<Semaphore>,
        limits: Limits,
        cancel: CancellationToken,
    ) -> Result<Self> {
        let permit = slots.try_acquire_owned().map_err(|_| Error::Busy)?;
        Ok(Self(Arc::new(Work {
            _lease: lease,
            _permit: permit,
            requests: AtomicU64::new(0),
            bytes: AtomicU64::new(0),
            limits,
            deadline: Instant::now() + limits.timeout,
            cancel,
        })))
    }
    pub fn check(&self) -> Result<()> {
        if self.0.cancel.is_cancelled() {
            return Err(Error::Canceled);
        }
        if Instant::now() >= self.0.deadline {
            return Err(Error::Limit);
        }
        Ok(())
    }
    pub async fn until<T>(&self, work: impl Future<Output = Result<T>>) -> Result<T> {
        self.check()?;
        let result = tokio::select! {
            biased;
            _=self.0.cancel.cancelled()=>Err(Error::Canceled),
            result=tokio::time::timeout_at(self.0.deadline, work)=>result.map_err(|_|Error::Limit)?,
        };
        self.check()?;
        result
    }
    pub(crate) fn limits(&self) -> Limits {
        self.0.limits
    }
    fn charge(&self, count: &AtomicU64, n: u64, max: u64) -> Result<()> {
        self.check()?;
        count
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |old| {
                old.checked_add(n).filter(|n| *n <= max)
            })
            .map_err(|_| Error::Limit)?;
        Ok(())
    }
    pub(crate) fn request(&self) -> Result<()> {
        self.charge(&self.0.requests, 1, self.0.limits.requests)
    }
    pub(crate) fn bytes(&self, n: u64) -> Result<()> {
        self.charge(&self.0.bytes, n, self.0.limits.bytes)
    }
}
