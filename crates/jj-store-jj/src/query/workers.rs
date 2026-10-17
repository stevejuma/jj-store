//! Shared non-queuing interactive admission and one reserved preparation worker.
use super::{GraphExecutor, Limits, Result, cache, content};
pub use crate::read_support::workers::{Status, Workers};
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::Error;
    use std::sync::Arc;
    #[test]
    fn retained_query_clones_capture_updated_limits_for_new_jobs() {
        let seconds = Arc::new(std::sync::atomic::AtomicU64::new(30));
        let graph_seconds = seconds.clone();
        let content_seconds = seconds.clone();
        let root = tempfile::tempdir().unwrap();
        let executors = Executors::new(
            root.path().to_owned(),
            Limits::default(),
            content::Limits::default(),
            cache::Policy::default(),
            2,
        )
        .unwrap()
        .with_live_limits(
            move |mut l| {
                l.timeout = std::time::Duration::from_secs(
                    graph_seconds.load(std::sync::atomic::Ordering::SeqCst),
                );
                l
            },
            move |mut l| {
                l.timeout = std::time::Duration::from_secs(
                    content_seconds.load(std::sync::atomic::Ordering::SeqCst),
                );
                l
            },
        );
        let retained = executors.clone();
        let captured = retained.content().current_limits();
        seconds.store(7, std::sync::atomic::Ordering::SeqCst);
        assert_eq!(
            retained.graph().current_limits().timeout,
            std::time::Duration::from_secs(7)
        );
        assert_eq!(
            retained.content().current_limits().timeout,
            std::time::Duration::from_secs(7)
        );
        assert_eq!(captured.timeout, std::time::Duration::from_secs(30));
        let _permit = executors.workers.acquire().unwrap();
        assert_eq!(executors.workers.status(), retained.workers.status());
        assert_eq!(retained.workers.status().in_use, 1);
    }
    #[test]
    fn background_observation_preserves_foreground_admission() {
        let workers = Workers::new(2).unwrap();
        let observing = workers.acquire_observation().unwrap();
        assert!(matches!(workers.acquire_observation(), Err(Error::Busy)));
        let foreground = workers.acquire().unwrap();
        assert_eq!(workers.status().in_use, 2);
        drop(observing);
        assert!(matches!(workers.acquire_observation(), Err(Error::Busy)));
        drop(foreground);
        assert!(workers.acquire_observation().is_ok());
        assert!(Workers::new(1).unwrap().acquire_observation().is_ok());
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
    /// All retained executor clones share the serving owner's live limits. Running jobs keep their captured budget.
    pub fn with_live_limits(
        mut self,
        graph: impl Fn(Limits) -> Limits + Send + Sync + 'static,
        content: impl Fn(content::Limits) -> content::Limits + Send + Sync + 'static,
    ) -> Self {
        self.graph = self.graph.with_live_limits(graph);
        self.content = self
            .content
            .with_live_limits(content)
            .with_graph(self.graph.clone());
        self
    }

    pub fn new(
        root: PathBuf,
        graph: Limits,
        content: content::Limits,
        cache: cache::Policy,
        capacity: usize,
    ) -> Result<Self> {
        let workers = Workers::new(capacity)?;
        let graph = GraphExecutor::with_workers(root, graph, cache, workers.clone())?;
        Ok(Self {
            content: content::ContentReader::with_workers(content, workers.clone())?
                .with_graph(graph.clone()),
            graph,
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
