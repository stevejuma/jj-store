use super::*;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::time::Instant;
pub(super) struct Budget {
    pub cancellation: tokio_util::sync::CancellationToken,
    pub limits: crate::query::Limits,
    deadline: Instant,
    work: AtomicU64,
    bytes: AtomicU64,
    metadata: AtomicU64,
    output: AtomicU64,
    byte_limit: u64,
    metadata_limit: u64,
}
impl Budget {
    pub fn new(limits: Limits) -> Self {
        Self {
            cancellation: tokio_util::sync::CancellationToken::new(),
            limits: crate::query::Limits {
                timeout: limits.timeout,
                max_work: limits.max_work,
                max_output_bytes: limits.max_output_bytes,
                ..Default::default()
            },
            deadline: Instant::now() + limits.timeout,
            work: AtomicU64::new(0),
            bytes: AtomicU64::new(0),
            metadata: AtomicU64::new(0),
            output: AtomicU64::new(0),
            byte_limit: 64 * 1024 * 1024,
            metadata_limit: 100_000,
        }
    }
    pub(super) fn preparation(
        limits: &preparation::Limits,
        cancellation: &crate::query::Cancellation,
    ) -> Self {
        let mut budget = Self::new(Limits {
            timeout: limits.timeout,
            max_work: limits.max_work,
            ..Limits::default()
        });
        budget.byte_limit = limits.max_read_bytes;
        budget.metadata_limit = limits.max_metadata_records;
        budget.cancellation = cancellation.token();
        budget
    }
    pub fn check(&self) -> Result<()> {
        if self.cancellation.is_cancelled() {
            return Err(Error::Canceled);
        }
        if Instant::now() >= self.deadline {
            return Err(Error::Limit);
        }
        Ok(())
    }
    fn charge(&self, counter: &AtomicU64, amount: u64, limit: u64) -> Result<()> {
        self.check()?;
        counter
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
                n.checked_add(amount).filter(|n| *n <= limit)
            })
            .map_err(|_| Error::Limit)?;
        Ok(())
    }
    pub fn work(&self, n: usize) -> Result<()> {
        self.charge(&self.work, n as u64, self.limits.max_work as u64)
    }
    pub fn metadata(&self, value: &impl Serialize) -> Result<()> {
        self.charge(&self.metadata, 1, self.metadata_limit)?;
        self.charge(
            &self.bytes,
            serde_json::to_vec(value)
                .map_err(|_| Error::Unavailable)?
                .len() as u64,
            self.byte_limit,
        )
    }
    pub fn read_bytes(&self, n: usize) -> Result<()> {
        self.charge(&self.bytes, n as u64, self.byte_limit)
    }
    pub fn output(&self, value: &impl Serialize) -> Result<()> {
        self.charge(
            &self.output,
            serde_json::to_vec(value)
                .map_err(|_| Error::Unavailable)?
                .len() as u64,
            self.limits.max_output_bytes as u64,
        )
    }
    pub fn complete_output(&self, value: &impl Serialize) -> Result<()> {
        self.check()?;
        if serde_json::to_vec(value)
            .map_err(|_| Error::Unavailable)?
            .len()
            > self.limits.max_output_bytes
        {
            return Err(Error::Limit);
        }
        Ok(())
    }
    pub async fn until<T>(&self, future: impl std::future::Future<Output = T>) -> Result<T> {
        self.check()?;
        tokio::select! { biased;
            _ = self.cancellation.cancelled() => Err(Error::Canceled),
            result = tokio::time::timeout_at(self.deadline, future) => result.map_err(|_| Error::Limit),
        }
    }
    pub async fn authorize(&self, pin: &Pin) -> Result<()> {
        self.work(1)?;
        self.until(pin.binding.check()).await?
    }
    pub fn stats(&self) -> Stats {
        Stats {
            metadata_nodes: self.metadata.load(Ordering::Relaxed),
            object_bytes: self.bytes.load(Ordering::Relaxed),
            work: self.work.load(Ordering::Relaxed),
            ..Default::default()
        }
    }
}
