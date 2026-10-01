//! Device-local management contracts, independent of transport and JJ discovery.
use jj_store_model::RepositoryId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MAX_WORKSPACES: usize = 64;
pub const MAX_PATH_BYTES: usize = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceProfile {
    Native,
    LocalJjGit,
}

/// Internal discovery proof. Never supplied by the browser.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerifiedWorkspace {
    pub path: String,
    pub metadata: String,
    pub repository: RepositoryId,
    pub profile: WorkspaceProfile,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceAvailability {
    Ready,
    Unavailable,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceLink {
    pub id: String,
    pub path: String,
    pub profile: WorkspaceProfile,
    pub availability: WorkspaceAvailability,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceLinks {
    pub version: u32,
    pub revision: u64,
    pub links: Vec<WorkspaceLink>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LinkWorkspace {
    pub version: u32,
    pub expected_revision: u64,
    pub path: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("invalid workspace binding")]
    Invalid,
    #[error("workspace profile is not supported for local management")]
    UnsupportedProfile,
    #[error("workspace links changed; refresh before retrying")]
    StaleRevision,
    #[error("workspace management resource limit")]
    Limit,
    #[error("workspace management is unavailable")]
    Unavailable,
}

pub trait WorkspaceVerifier: Send + Sync {
    /// Read existing, bounded metadata only. Must not load/snapshot a working copy,
    /// initialize metadata, reconcile heads or execute programs.
    fn inspect(&self, path: &str) -> Result<VerifiedWorkspace, Error>;
}

#[async_trait::async_trait]
pub trait WorkspaceManager: Send + Sync {
    async fn list(&self, repository: RepositoryId) -> Result<WorkspaceLinks, Error>;
    async fn link(
        &self,
        repository: RepositoryId,
        request: LinkWorkspace,
    ) -> Result<WorkspaceLinks, Error>;
}
