//! Explicit LocalTrusted installation administration; no general object deletion port.
use jj_store_model::{EpochId, RepositoryId, RequestId};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeleteRepository {
    pub version: u32,
    pub request_id: RequestId,
    pub expected_epoch: EpochId,
    pub expected_installation: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DeletionState {
    Pending,
    Deleted,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeletionOutcome {
    pub version: u32,
    pub repository: RepositoryId,
    pub request_id: RequestId,
    pub state: DeletionState,
}
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown storage writes require managed storage restart")]
    Uncertain,
    #[error("repository epoch changed before deletion")]
    Stale,
    #[error("versioned bucket deletion is unsupported")]
    Unsupported,
    #[error("repository deletion is unavailable")]
    Unavailable,
    #[error("repository deletion intent conflicts")]
    Conflict,
    #[error("invalid repository deletion request")]
    Invalid,
}
#[async_trait::async_trait]
pub trait RepositoryDeletion: Send + Sync {
    async fn delete(
        &self,
        repository: RepositoryId,
        request: DeleteRepository,
    ) -> Result<DeletionOutcome, Error>;
    async fn status(
        &self,
        repository: RepositoryId,
        request: RequestId,
    ) -> Result<DeletionOutcome, Error>;
}
