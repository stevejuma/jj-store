//! Bounded current metadata + physical read orchestration. No mutation capability.
use super::*;
use jj_store_jj::{local_browser as code, query::Cancellation};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Semaphore;

#[derive(Clone)]
pub struct Observer {
    pub(super) reader: code::Executor,
    concurrency: u32,
    pub(super) capacity: Arc<Semaphore>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Observation {
    pub current: code::working_tree::CurrentState,
    pub projection: Projection,
}
pub(super) struct Cancel(pub(super) Cancellation);
impl Drop for Cancel {
    fn drop(&mut self) {
        self.0.cancel();
    }
}
impl Observer {
    pub fn new(reader: code::Executor, concurrency: usize) -> Result<Self> {
        if !(1..=64).contains(&concurrency) {
            return Err(Error::Limit);
        }
        Ok(Self {
            reader,
            concurrency: concurrency as u32,
            capacity: Arc::new(Semaphore::new(concurrency)),
        })
    }
    /// Waits for canceled blocking scans to release their actual worker leases.
    pub async fn drain(&self) -> Result<()> {
        let _permit = self
            .capacity
            .clone()
            .acquire_many_owned(self.concurrency)
            .await
            .map_err(|_| Error::Unavailable)?;
        Ok(())
    }
    /// Trusted composition supplies the registered canonical root and access owner.
    pub async fn observe(
        &self,
        access: Arc<dyn code::Access>,
        root: PathBuf,
        operation: jj_store_model::OperationId,
    ) -> Result<Observation> {
        let permit = Arc::new(
            self.capacity
                .clone()
                .try_acquire_owned()
                .map_err(|_| Error::Busy)?,
        );
        let cancellation = Cancellation::default();
        let _cancel = Cancel(cancellation.clone());
        let deadline = Instant::now() + Duration::from_secs(8);
        let run = async {
            let before = self
                .reader
                .working_tree_current(access.clone(), root.clone(), operation)
                .await
                .map_err(query_error)?;
            let scan_access = access.clone();
            let scan_cancel = cancellation.clone();
            let scan_root = root.clone();
            let scan_permit = permit.clone();
            let runtime = tokio::runtime::Handle::current();
            let physical = tokio::task::spawn_blocking(move || {
                // Outer observation and blocking scan both hold admission. Cancellation
                // cannot admit another scan before this worker actually exits.
                let _permit = scan_permit;
                scan(&scan_root, scan_access.repository(), scan_access.generation(),
                    Limits::default(), |_| {
                        check(&scan_cancel, deadline)?;
                        runtime.block_on(async {
                            tokio::select! { biased;
                                _ = scan_cancel.cancelled() => Err(Error::Cancelled),
                                value = tokio::time::timeout(Duration::from_millis(500), scan_access.authorize()) => value.map_err(|_| Error::Limit)?.map_err(query_error),
                            }
                        })
                    },
                )
            }).await.map_err(|_| Error::Unavailable)??;
            let after = self
                .reader
                .working_tree_current(access.clone(), root, operation)
                .await
                .map_err(query_error)?;
            if before != after || after.physical_workspace != physical.workspace.0 {
                return Err(Error::Stale);
            }
            let projection = project_sparse(
                physical,
                after.baseline.clone(),
                after.base_ignores.clone(),
                after.checkout.sparse.clone(),
                || check(&cancellation, deadline),
            )?;
            access.authorize().await.map_err(query_error)?;
            check(&cancellation, deadline)?;
            Ok(Observation {
                current: after,
                projection,
            })
        };
        tokio::time::timeout_at(tokio::time::Instant::from_std(deadline), run)
            .await
            .map_err(|_| Error::Limit)?
    }
}
pub(super) fn check(cancellation: &Cancellation, deadline: Instant) -> Result<()> {
    if cancellation.is_cancelled() {
        return Err(Error::Cancelled);
    }
    if Instant::now() >= deadline {
        return Err(Error::Limit);
    }
    Ok(())
}
pub(super) fn query_error(error: code::Error) -> Error {
    match error {
        code::Error::Canceled => Error::Cancelled,
        code::Error::Denied => Error::Denied,
        code::Error::EpochChanged => Error::Stale,
        code::Error::Limit => Error::Limit,
        code::Error::Busy => Error::Busy,
        code::Error::Invalid => Error::Unsupported,
        _ => Error::Unavailable,
    }
}
