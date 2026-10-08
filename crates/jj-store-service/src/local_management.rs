//! Device-local management contracts, independent of transport and JJ discovery.
use jj_store_model::RepositoryId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MAX_WORKSPACES: usize = 64;
pub const MAX_PATH_BYTES: usize = 4096;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FolderSelection {
    pub version: u32,
    pub path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ModelFileSelection {
    pub version: u32,
    pub path: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PickerError {
    #[error("folder picker is unavailable")]
    Unavailable,
    #[error("folder picker is already open")]
    Busy,
    #[error("folder picker timed out")]
    Timeout,
    #[error("folder picker returned invalid output")]
    Invalid,
}

#[async_trait::async_trait]
pub trait FolderPicker: Send + Sync {
    /// Explicit OS interaction only; selection never saves or verifies a link.
    async fn pick(&self) -> Result<FolderSelection, PickerError>;
    fn model_files_supported(&self) -> bool {
        false
    }
    /// Fixed GGUF dialog only; selection is a draft, never an import command.
    async fn pick_model_file(&self) -> Result<ModelFileSelection, PickerError> {
        Err(PickerError::Unavailable)
    }
}

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

/// Device-local capture admission witness; it confers no publication authority.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerifiedCaptureWorkspace {
    /// Original absolute source directory, retained to recheck aliases and newly
    /// introduced inner repository boundaries. Device-only, never an AgentStore body.
    pub directory: String,
    pub workspace: VerifiedWorkspace,
    pub binding_digest: [u8; 32],
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

    /// Resolve a session directory to its nearest repository boundary and retain
    /// an exact physical/native connection witness. Root-only implementations
    /// cannot be used for automatic capture without qualifying this capability.
    fn inspect_capture_directory(&self, _path: &str) -> Result<VerifiedCaptureWorkspace, Error> {
        Err(Error::Unavailable)
    }
}

#[async_trait::async_trait]
pub trait WorkspaceManager: Send + Sync {
    /// Bounded link and physical binding comparison; no repository/history reads.
    fn freshness_witness(&self) -> Option<String> {
        None
    }
    fn freshness(&self) -> Option<tokio::sync::broadcast::Receiver<crate::freshness::Change>> {
        None
    }
    async fn list(&self, repository: RepositoryId) -> Result<WorkspaceLinks, Error>;
    async fn link(
        &self,
        repository: RepositoryId,
        request: LinkWorkspace,
    ) -> Result<WorkspaceLinks, Error>;
}
