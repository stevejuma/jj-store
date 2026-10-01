use jj_store_model::RepositoryId;
use jj_store_rpc::server::{ReplicationControlError, ReplicationControls};
use jj_store_service::sync_management::{
    Error, LocalSyncStatus, SyncAction, SyncCommand, SyncManager,
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
