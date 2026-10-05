//! Complete physical preconditions for metadata-only checkout completion.
use super::*;
use jj_store_jj::{
    local_browser as code,
    local_execution::{PhysicalGuard, PhysicalInventoryV1, RecoveryBinding},
    query::Cancellation,
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Semaphore;
struct Guard {
    access: Arc<dyn code::Access>,
    root: PathBuf,
    capacity: Arc<Semaphore>,
}
impl Observer {
    /// Trusted registration composition supplies the root and current access
    /// owner. Construction never scans, snapshots or grants a command capability.
    pub fn physical_guard(
        &self,
        access: Arc<dyn code::Access>,
        root: PathBuf,
    ) -> Arc<dyn PhysicalGuard> {
        Arc::new(Guard {
            access,
            root,
            capacity: self.capacity.clone(),
        })
    }
}
impl Guard {
    async fn read(
        &self,
        binding: &RecoveryBinding,
        expected: Option<RecordDigest>,
        capture: bool,
    ) -> code::Result<Option<PhysicalInventoryV1>> {
        self.access.authorize().await?;
        if binding.repository != self.access.repository()
            || binding.generation != self.access.generation()
            || expected == Some(RecordDigest::new([0; 32]))
        {
            return Err(code::Error::EpochChanged);
        }
        let permit = Arc::new(
            self.capacity
                .clone()
                .try_acquire_owned()
                .map_err(|_| code::Error::Busy)?,
        );
        let cancellation = Cancellation::default();
        let _cancel = observer::Cancel(cancellation.clone());
        let deadline = Instant::now() + Duration::from_secs(8);
        let access = self.access.clone();
        let root = self.root.clone();
        let worker_cancel = cancellation.clone();
        let worker_permit = permit.clone();
        let runtime = tokio::runtime::Handle::current();
        let read = async {
            let physical=tokio::task::spawn_blocking(move || {
                let _permit=worker_permit;
                scan(&root,access.repository(),access.generation(),Limits::default(), |_| {
                    observer::check(&worker_cancel,deadline)?;
                    runtime.block_on(async { tokio::select!{biased;
                        _=worker_cancel.cancelled()=>Err(Error::Cancelled),
                        value=tokio::time::timeout(Duration::from_millis(500),access.authorize())=>value.map_err(|_|Error::Limit)?.map_err(observer::query_error),
                    }})
                })
            }).await.map_err(|_|code::Error::Unavailable)?.map_err(query_error)?;
            if physical.workspace.0 != binding.physical_workspace
                || expected.is_some_and(|expected| physical.fingerprint != expected)
            {
                return Err(code::Error::EpochChanged);
            }
            let inventory = if capture {
                let inventory = physical.inventory().map_err(query_error)?;
                inventory.check_selection(binding, expected.unwrap_or(physical.fingerprint))?;
                Some(inventory)
            } else {
                None
            };
            self.access.authorize().await?;
            observer::check(&cancellation, deadline).map_err(query_error)?;
            Ok(inventory)
        };
        tokio::time::timeout_at(tokio::time::Instant::from_std(deadline), read)
            .await
            .map_err(|_| code::Error::Limit)?
    }
}
#[async_trait::async_trait]
impl PhysicalGuard for Guard {
    async fn validate(
        &self,
        binding: &RecoveryBinding,
        expected: RecordDigest,
    ) -> code::Result<()> {
        self.read(binding, Some(expected), false).await.map(|_| ())
    }
    async fn inventory(
        &self,
        binding: &RecoveryBinding,
        expected: RecordDigest,
    ) -> code::Result<PhysicalInventoryV1> {
        self.read(binding, Some(expected), true)
            .await?
            .ok_or(code::Error::Invalid)
    }
    async fn observe(&self, binding: &RecoveryBinding) -> code::Result<PhysicalInventoryV1> {
        self.read(binding, None, true)
            .await?
            .ok_or(code::Error::Invalid)
    }
}
fn query_error(error: Error) -> code::Error {
    match error {
        Error::Busy => code::Error::Busy,
        Error::Denied => code::Error::Denied,
        Error::Invalid | Error::Unsupported => code::Error::Invalid,
        Error::Limit => code::Error::Limit,
        Error::Stale => code::Error::EpochChanged,
        Error::Unavailable => code::Error::Unavailable,
        Error::Cancelled => code::Error::Canceled,
    }
}
