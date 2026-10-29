//! Shared non-queuing interactive admission and one reserved preparation worker.
use super::{Error, Result};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

/// Clone once per serving instance, not once per repository or route.
#[derive(Clone)]
pub struct Workers {
    capacity: usize,
    permits: Arc<Semaphore>,
    preparation: Arc<Semaphore>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Status {
    /// Total capacity: configured interactive capacity plus one preparation slot.
    pub capacity: usize,
    /// Includes live verified-file handles and canceled jobs that have not exited.
    pub in_use: usize,
}
impl Workers {
    pub fn new(capacity: usize) -> Result<Self> {
        if !(1..=64).contains(&capacity) {
            return Err(Error::Limit);
        }
        Ok(Self {
            capacity,
            permits: Arc::new(Semaphore::new(capacity)),
            preparation: Arc::new(Semaphore::new(1)),
        })
    }
    pub fn status(&self) -> Status {
        Status {
            capacity: self.capacity + 1,
            in_use: self.capacity - self.permits.available_permits() + 1
                - self.preparation.available_permits(),
        }
    }
    pub(crate) fn acquire(&self) -> Result<OwnedSemaphorePermit> {
        self.permits
            .clone()
            .try_acquire_owned()
            .map_err(|_| Error::Busy)
    }
    /// Metadata observation yields to foreground work and leaves one read slot
    /// available when the serving capacity permits it. No waiting queue.
    pub(crate) fn acquire_observation(&self) -> Result<OwnedSemaphorePermit> {
        let mut permits = self
            .permits
            .clone()
            .try_acquire_many_owned(self.capacity.min(2) as u32)
            .map_err(|_| Error::Busy)?;
        Ok(permits.split(1).expect("one admitted observation permit"))
    }
    /// Non-queuing admission shared by native graph, local graph and source
    /// preparation. Move the lease into the actual job until it exits.
    pub fn acquire_preparation(&self) -> Result<OwnedSemaphorePermit> {
        self.preparation
            .clone()
            .try_acquire_owned()
            .map_err(|_| Error::Busy)
    }
}
