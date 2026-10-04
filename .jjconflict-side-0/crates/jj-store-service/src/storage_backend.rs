use crate::{
    administration::RepositoryHost, auth::Permission, native::NativeSession, storage_management::*,
};
use jj_store_model::{PrincipalId, RepositoryId};
use jj_store_wal::{
    self as wal,
    publication::{Checkpoint, CheckpointCoverage},
};
use std::sync::Arc;
pub struct NativeStorageBackend {
    pub host: RepositoryHost,
    pub principal: PrincipalId,
}
impl NativeStorageBackend {
    fn session(&self, repo: RepositoryId) -> Result<Arc<NativeSession>, Error> {
        self.host
            .endpoint(repo)
            .and_then(|e| e.bind_local_trusted(self.principal, Permission::Admin))
            .map(|e| e.into_session())
            .map_err(|_| Error::Unavailable)
    }
}
fn position(target: &Checkpoint) -> StoragePosition {
    StoragePosition {
        repository: target.captured.repository,
        epoch: target.captured.epoch,
        position: target.captured.published,
        revision: target.captured.revision,
        digest: target.captured_digest,
    }
}
fn limits(session: &NativeSession) -> StorageLimits {
    StorageLimits {
        entries: session.limits.inventory_nodes,
        bytes: session.limits.publication.inventory.max_new_bytes,
        timeout_millis: session
            .limits
            .publication
            .timeout
            .as_millis()
            .min(u64::MAX as u128) as u64,
    }
}
fn failure(e: wal::Error) -> Error {
    match e {
        wal::Error::Limit(_) | wal::Error::Inventory(wal::inventory::InventoryError::Limit(_)) => {
            Error::Incomplete
        }
        wal::Error::OutcomeUnknown => Error::Unknown,
        wal::Error::Corrupt(_)
        | wal::Error::Json(_)
        | wal::Error::Storage(jj_store_object::Error::NotFound) => Error::Corrupt,
        _ => Error::Unavailable,
    }
}
#[async_trait::async_trait]
impl StorageBackend for NativeStorageBackend {
    async fn overview(&self, repo: RepositoryId) -> Result<StorageOverview, Error> {
        let s = self.session(repo)?;
        s.principal(Permission::Admin).await.map_err(failure)?;
        let snapshot = s.coordinator.read().await.map_err(failure)?;
        let checkpoint = s
            .coordinator
            .selected_checkpoint(&snapshot)
            .await
            .map_err(failure)?;
        let current = Checkpoint {
            captured: snapshot.manifest().clone(),
            captured_digest: snapshot.digest(),
        };
        Ok(StorageOverview {
            version: 1,
            ready: true,
            current: position(&current),
            checkpoint: checkpoint.as_ref().map(position),
            limits: limits(&s),
            latest: None,
        })
    }
    async fn capture(&self, repo: RepositoryId) -> Result<StorageCapture, Error> {
        let s = self.session(repo)?;
        s.principal(Permission::Admin).await.map_err(failure)?;
        let checkpoint = s.coordinator.capture_maintenance().await.map_err(failure)?;
        Ok(StorageCapture {
            position: position(&checkpoint),
            limits: limits(&s),
            checkpoint,
        })
    }
    async fn check(&self, repo: RepositoryId, target: &StorageCapture) -> Result<(), Error> {
        let s = self.session(repo)?;
        s.principal(Permission::Admin).await.map_err(failure)?;
        s.coordinator
            .recover_target(
                &target.checkpoint,
                target.limits.entries.min(s.limits.inventory_nodes),
            )
            .await
            .map_err(failure)?;
        Ok(())
    }
    async fn publish(&self, repo: RepositoryId, target: &StorageCapture) -> Result<(), Error> {
        let s = self.session(repo)?;
        s.principal(Permission::Admin).await.map_err(failure)?;
        s.coordinator
            .checkpoint_target(&target.checkpoint)
            .await
            .map_err(failure)?;
        Ok(())
    }
    async fn selection(
        &self,
        repo: RepositoryId,
        target: &StorageCapture,
    ) -> Result<CheckpointSelection, Error> {
        let s = self.session(repo)?;
        s.principal(Permission::Admin).await.map_err(failure)?;
        Ok(
            match s
                .coordinator
                .checkpoint_coverage(&target.checkpoint)
                .await
                .map_err(failure)?
            {
                CheckpointCoverage::NotCovered => CheckpointSelection::NotCovered,
                CheckpointCoverage::Selected => CheckpointSelection::Selected,
                CheckpointCoverage::Covered => CheckpointSelection::Covered,
            },
        )
    }
}

impl StorageCapture {
    pub fn validate(&self, repository: RepositoryId) -> Result<(), Error> {
        if self.position != position(&self.checkpoint)
            || self.position.repository != repository
            || self.limits.entries == 0
            || self.limits.bytes == 0
            || self.limits.timeout_millis == 0
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
}
