//! Owned device registration contracts. Physical JJ discovery belongs to composition.
use crate::local_repository::LocalRepository;
pub use jj_store_jj::local_browser as code;
use jj_store_model::{RepositoryId, RequestId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("invalid local repository folder")]
    Invalid,
    #[error("local repository profile is unsupported")]
    Unsupported,
    #[error("local repository is unavailable")]
    Unavailable,
    #[error("local repository identity changed")]
    IdentityChanged,
    #[error("local registration revision changed")]
    Stale,
    #[error("local registration conflicts with another binding")]
    Conflict,
    #[error("local registration resource limit")]
    Limit,
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LocalFolder {
    pub path: String,
    pub metadata: String,
    pub repository: Option<RepositoryId>,
}
pub trait FolderProvider: Send + Sync {
    /// No initialization, checkout snapshot, reconciliation or program execution.
    fn inspect(&self, path: &str) -> Result<LocalFolder>;
    fn open(&self, folder: &LocalFolder) -> Result<LocalRepository>;
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct InspectFolder {
    pub path: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RegisterFolder {
    pub version: u32,
    pub request_id: RequestId,
    pub expected_revision: u64,
    pub path: String,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Availability {
    Ready,
    Unavailable,
    IdentityChanged,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LocalBinding {
    pub repository: RepositoryId,
    pub name: String,
    pub path: String,
    pub generation: u64,
    pub availability: Availability,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct LocalBindings {
    pub version: u32,
    pub revision: u64,
    pub bindings: Vec<LocalBinding>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RegistrationOutcome {
    Committed {
        request_id: RequestId,
        binding: LocalBinding,
        revision: u64,
    },
    OutcomeUnknown {
        request_id: RequestId,
    },
    NotKnown {
        request_id: RequestId,
    },
}
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RemoveBinding {
    pub version: u32,
    pub expected_revision: u64,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReconnectFolder {
    pub version: u32,
    pub expected_revision: u64,
    pub path: String,
}

/// Installation-owned creation, independent of physical workspace attachment.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateNativeRepository {
    pub version: u32,
    pub request_id: RequestId,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum NativeCreationOutcome {
    Committed {
        request_id: RequestId,
        repository: RepositoryId,
        name: String,
    },
    OutcomeUnknown {
        request_id: RequestId,
    },
    NotKnown {
        request_id: RequestId,
    },
}

pub struct HintWatch {
    pub receiver: tokio::sync::mpsc::Receiver<()>,
    pub retained: Box<dyn Send>,
}
#[async_trait::async_trait]
pub trait LocalRepositories: Send + Sync {
    fn watch(&self, _repository: RepositoryId) -> Result<Option<HintWatch>> {
        Ok(None)
    }
    fn freshness(&self) -> Option<tokio::sync::broadcast::Receiver<crate::freshness::Change>> {
        None
    }
    fn conversion(&self) -> bool {
        false
    }
    async fn preview_conversion(
        &self,
        _intent: crate::conversion::contracts::ConversionIntent,
    ) -> Result<crate::conversion::contracts::ConversionPreview> {
        Err(Error::Unsupported)
    }
    async fn start_conversion(
        &self,
        _request: crate::conversion::contracts::StartConversion,
    ) -> Result<crate::conversion::contracts::ConversionOutcome> {
        Err(Error::Unsupported)
    }
    async fn conversion_status(
        &self,
        _request: RequestId,
    ) -> Result<crate::conversion::contracts::ConversionOutcome> {
        Err(Error::Unsupported)
    }
    fn native_creation(&self) -> bool {
        false
    }
    async fn create_native(
        &self,
        _request: CreateNativeRepository,
    ) -> Result<NativeCreationOutcome> {
        Err(Error::Unsupported)
    }
    async fn native_creation_status(&self, _request: RequestId) -> Result<NativeCreationOutcome> {
        Err(Error::Unsupported)
    }
    fn inspect(&self, path: &str) -> Result<LocalFolder>;
    fn list(&self) -> Result<LocalBindings>;
    fn folder(&self, repository: RepositoryId) -> Result<(LocalFolder, u64)>;
    fn open(&self, repository: RepositoryId) -> Result<LocalRepository>;
    async fn register(&self, request: RegisterFolder) -> Result<RegistrationOutcome>;
    async fn status(&self, request: RequestId) -> Result<RegistrationOutcome>;
    fn remove(&self, repository: RepositoryId, request: RemoveBinding) -> Result<LocalBindings>;
    fn reconnect(
        &self,
        repository: RepositoryId,
        request: ReconnectFolder,
    ) -> Result<LocalBindings>;
}
