//! Owned local maintenance contracts. The private capture is never browser input.
use jj_store_model::{EpochId, RecordDigest, RepositoryId, RequestId, WalPosition};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StoragePosition {
    pub repository: RepositoryId,
    pub epoch: EpochId,
    pub position: WalPosition,
    pub revision: u64,
    pub digest: RecordDigest,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StorageLimits {
    pub entries: usize,
    pub bytes: u64,
    pub timeout_millis: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StorageAction {
    Check,
    PrepareCheckpoint,
    PublishCheckpoint,
    CancelCheckpoint,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StorageKind {
    Check,
    Checkpoint,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StoragePhase {
    Prepared,
    Running,
    Complete,
    Incomplete,
    Failed,
    Interrupted,
    Unknown,
    Canceled,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointSelection {
    NotCovered,
    Selected,
    Covered,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StorageCommand {
    pub version: u32,
    pub request_id: RequestId,
    pub action: StorageAction,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StorageOperation {
    pub version: u32,
    pub request_id: RequestId,
    pub kind: StorageKind,
    pub phase: StoragePhase,
    pub target: StoragePosition,
    pub limits: StorageLimits,
    pub selection: Option<CheckpointSelection>,
    pub message: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StorageOverview {
    pub version: u32,
    pub ready: bool,
    pub current: StoragePosition,
    pub checkpoint: Option<StoragePosition>,
    pub limits: StorageLimits,
    pub latest: Option<StorageOperation>,
}
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("invalid maintenance command")]
    Invalid,
    #[error("maintenance request has different intent")]
    Conflict,
    #[error("another maintenance operation needs attention")]
    Busy,
    #[error("maintenance journal is full")]
    Limit,
    #[error("maintenance operation not found")]
    NotFound,
    #[error("storage maintenance unavailable")]
    Unavailable,
    #[error("integrity check exceeded its configured budget")]
    Incomplete,
    #[error("selected storage data is missing or invalid")]
    Corrupt,
    #[error("checkpoint publication outcome is unknown")]
    Unknown,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StorageCapture {
    pub position: StoragePosition,
    pub limits: StorageLimits,
    pub(crate) checkpoint: jj_store_wal::publication::Checkpoint,
}
#[async_trait::async_trait]
pub trait StorageBackend: Send + Sync {
    async fn overview(&self, repo: RepositoryId) -> Result<StorageOverview, Error>;
    async fn capture(&self, repo: RepositoryId) -> Result<StorageCapture, Error>;
    async fn check(&self, repo: RepositoryId, target: &StorageCapture) -> Result<(), Error>;
    async fn publish(&self, repo: RepositoryId, target: &StorageCapture) -> Result<(), Error>;
    async fn selection(
        &self,
        repo: RepositoryId,
        target: &StorageCapture,
    ) -> Result<CheckpointSelection, Error>;
}
#[async_trait::async_trait]
pub trait StorageManager: Send + Sync {
    async fn overview(&self, repo: RepositoryId) -> Result<StorageOverview, Error>;
    async fn command(
        &self,
        repo: RepositoryId,
        command: StorageCommand,
    ) -> Result<StorageOperation, Error>;
    async fn operation(
        &self,
        repo: RepositoryId,
        request: RequestId,
    ) -> Result<StorageOperation, Error>;
}
