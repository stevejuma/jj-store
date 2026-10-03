use jj_store_model::RepositoryId;
use jj_store_rpc::server::{ReplicationControlError, ReplicationControls};
use jj_store_service::sync_management::{
    Error, LocalSyncStatus, LocalSyncWait, SyncAction, SyncCommand, SyncManager,
};
use std::sync::Arc;

pub struct LocalSync(pub Arc<dyn ReplicationControls>);
fn failure(error: ReplicationControlError) -> Error {
    match error {
        ReplicationControlError::NotConfigured => Error::NotConfigured,
        ReplicationControlError::Invalid => Error::Invalid,
        _ => Error::Unavailable,
    }
}
fn configured(worker: jj_store_service::replication_control::StatusV1) -> LocalSyncStatus {
    LocalSyncStatus {
        version: 1,
        configured: true,
        worker: Some(worker),
    }
}
#[async_trait::async_trait]
impl SyncManager for LocalSync {
    async fn capture(
        &self,
        repository: RepositoryId,
    ) -> Result<jj_store_service::replication_control::BarrierV1, Error> {
        self.0.capture_barrier(repository).await.map_err(failure)
    }
    async fn wait(
        &self,
        repository: RepositoryId,
        request: jj_store_service::replication_control::WaitRequestV1,
    ) -> Result<LocalSyncWait, Error> {
        if request.version != 1
            || request.barrier.version != 1
            || request.barrier.repository != repository
            || !(1..=1000).contains(&request.timeout_millis)
        {
            return Err(Error::Invalid);
        }
        match self.0.wait(repository, &request).await {
            Ok(outcome) => Ok(LocalSyncWait {
                version: 1,
                barrier: outcome.barrier,
                reached: true,
                status: outcome.status,
            }),
            Err(ReplicationControlError::Timeout) => Ok(LocalSyncWait {
                version: 1,
                barrier: request.barrier,
                reached: false,
                status: self.0.status(repository).await.map_err(failure)?,
            }),
            Err(error) => Err(failure(error)),
        }
    }

    async fn status(&self, repository: RepositoryId) -> Result<LocalSyncStatus, Error> {
        match self.0.status(repository).await {
            Ok(worker) => Ok(configured(worker)),
            Err(ReplicationControlError::NotConfigured) => Ok(LocalSyncStatus {
                version: 1,
                configured: false,
                worker: None,
            }),
            Err(error) => Err(failure(error)),
        }
    }
    async fn control(
        &self,
        repository: RepositoryId,
        command: SyncCommand,
    ) -> Result<LocalSyncStatus, Error> {
        if command.version != 1 {
            return Err(Error::Invalid);
        }
        let worker = match command.action {
            SyncAction::Pause => self.0.set_paused(repository, true).await,
            SyncAction::Resume => self.0.set_paused(repository, false).await,
            SyncAction::Retry => self.0.wake(repository).await,
        }
        .map_err(failure)?;
        Ok(configured(worker))
    }
}
