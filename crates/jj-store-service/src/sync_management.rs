//! Local browser-facing controls for an already configured sync worker.
use jj_store_model::RepositoryId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LocalSyncStatus {
    pub version: u32,
    pub configured: bool,
    pub worker: Option<crate::replication_control::StatusV1>,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SyncAction {
    Pause,
    Resume,
    Retry,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SyncCommand {
    pub version: u32,
    pub action: SyncAction,
}
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("sync is not configured")]
    NotConfigured,
    #[error("invalid sync command")]
    Invalid,
    #[error("sync worker unavailable")]
    Unavailable,
}
#[async_trait::async_trait]
pub trait SyncManager: Send + Sync {
    async fn status(&self, repository: RepositoryId) -> Result<LocalSyncStatus, Error>;
    async fn control(
        &self,
        repository: RepositoryId,
        command: SyncCommand,
    ) -> Result<LocalSyncStatus, Error>;
}
