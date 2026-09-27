//! Shared, non-queuing admission across native graphs, content and file streams.
use super::{Error, GraphExecutor, Limits, Result, cache, content};
use serde::Serialize;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

/// Clone once per serving instance, not once per repository or route.
#[derive(Clone)]
pub struct Workers {
    capacity: usize,
    permits: Arc<Semaphore>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Status {
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
        })
    }
    pub fn status(&self) -> Status {
        Status {
            capacity: self.capacity,
            in_use: self.capacity - self.permits.available_permits(),
        }
    }
    pub(crate) fn acquire(&self) -> Result<OwnedSemaphorePermit> {
        self.permits
            .clone()
            .try_acquire_owned()
            .map_err(|_| Error::Busy)
    }
}
/// The serving composition creates this once and shares its clones. Standalone
/// executors remain useful for isolated callers with an independently owned budget.
#[derive(Clone)]
pub struct Executors {
    graph: GraphExecutor,
    content: content::ContentReader,
    workers: Workers,
}
impl Executors {
    pub fn new(
        root: PathBuf,
        graph: Limits,
        content: content::Limits,
        cache: cache::Policy,
        capacity: usize,
    ) -> Result<Self> {
        let workers = Workers::new(capacity)?;
        Ok(Self {
            graph: GraphExecutor::with_workers(root, graph, cache, workers.clone())?,
            content: content::ContentReader::with_workers(content, workers.clone())?,
            workers,
        })
    }
    pub fn graph(&self) -> &GraphExecutor {
        &self.graph
    }
    pub fn content(&self) -> &content::ContentReader {
        &self.content
    }
    pub fn workers(&self) -> &Workers {
        &self.workers
    }
}
