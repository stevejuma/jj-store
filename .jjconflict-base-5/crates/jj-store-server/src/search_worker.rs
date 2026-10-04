use crate::{Error, Result, search::Runner};
use jj_store_adapters::{self as a, CancellationToken};
use jj_store_model::RepositoryId;
use std::{
    collections::BTreeMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::{sync::Mutex, task::JoinHandle, time::timeout};
struct Worker {
    cancel: CancellationToken,
    task: JoinHandle<(Runner, a::Result<()>)>,
}
impl Drop for Worker {
    fn drop(&mut self) {
        self.cancel.cancel();
        self.task.abort();
    }
}
#[derive(Default)]
pub struct Control {
    workers: Mutex<BTreeMap<RepositoryId, Worker>>,
    closed: AtomicBool,
    cancel: CancellationToken,
    jobs: Mutex<Vec<JoinHandle<()>>>,
}
impl Control {
    pub(crate) fn closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }
    pub(crate) fn cancellation(&self) -> CancellationToken {
        self.cancel.clone()
    }
    pub(crate) async fn spawn(
        &self,
        job: impl std::future::Future<Output = ()> + Send + 'static,
    ) -> std::result::Result<(), jj_store_service::search_management::Error> {
        let mut jobs = self.jobs.lock().await;
        if self.closed() {
            return Err(jj_store_service::search_management::Error::Unavailable);
        }
        jobs.retain(|job| !job.is_finished());
        jobs.push(tokio::spawn(job));
        Ok(())
    }
    pub(crate) async fn start(&self, repo: RepositoryId, mut runner: Runner) {
        if self.closed.load(Ordering::SeqCst) {
            return;
        }
        let cancel = self.cancel.child_token();
        let token = cancel.clone();
        let task = tokio::spawn(async move {
            let result = runner.run(token).await;
            let drained = runner.wait_idle().await;
            (runner, result.and(drained))
        });
        self.workers
            .lock()
            .await
            .insert(repo, Worker { cancel, task });
    }
    pub(crate) async fn stop(&self, repo: RepositoryId) -> Result<Option<Runner>> {
        let mut workers = self.workers.lock().await;
        let Some(worker) = workers.get_mut(&repo) else {
            return Ok(None);
        };
        worker.cancel.cancel();
        // Leave an undrained owner registered on timeout; never race a replacement writer.
        let outcome = timeout(Duration::from_secs(5), &mut worker.task)
            .await
            .map_err(|_| Error::Service("search worker is still draining"))?;
        workers.remove(&repo);
        let (runner, _result) =
            outcome.map_err(|_| Error::Service("search worker stopped unexpectedly"))?;
        Ok(Some(runner))
    }
    async fn shutdown(&self) -> Result<()> {
        self.closed.store(true, Ordering::SeqCst);
        self.cancel.cancel();
        let mut jobs = self.jobs.lock().await;
        for job in jobs.iter_mut() {
            timeout(Duration::from_secs(5), job)
                .await
                .map_err(|_| Error::Service("search operation is still draining"))?
                .map_err(|_| Error::Service("search operation failed"))?;
        }
        jobs.clear();
        let repos = self
            .workers
            .lock()
            .await
            .keys()
            .copied()
            .collect::<Vec<_>>();
        for repo in repos {
            self.stop(repo).await?;
        }
        Ok(())
    }
}
pub struct RunningSearch(pub(crate) Arc<Control>);
impl RunningSearch {
    pub async fn shutdown(self) -> Result<()> {
        self.0.shutdown().await
    }
}

impl Drop for RunningSearch {
    fn drop(&mut self) {
        self.0.closed.store(true, Ordering::SeqCst);
        self.0.cancel.cancel();
        if let Ok(mut workers) = self.0.workers.try_lock() {
            workers.clear();
        }
    }
}
