//! Installation-local operational configuration; no repository or worker mutation.
use jj_store_model::{CommitId, PrincipalId, RepositoryId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ServiceConfiguration {
    pub concurrent_requests: usize,
    pub request_seconds: u64,
    pub max_blob_bytes: u64,
    pub inventory_nodes: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProtectionConfiguration {
    pub bookmarks: BTreeSet<String>,
    pub commits: BTreeSet<CommitId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SyncLimits {
    pub max_batch_origins: usize,
    pub max_closure_entries: usize,
    pub max_objects_per_step: usize,
    pub max_bytes_per_step: u64,
    pub max_native_object_bytes: u64,
    pub poll_millis: u64,
    pub retry_initial_millis: u64,
    pub retry_max_seconds: u64,
    pub circuit_failures: usize,
    pub circuit_cooldown_seconds: u64,
    pub timeout_seconds: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SyncConfiguration {
    pub repository: RepositoryId,
    pub upstream: String,
    pub insecure_loopback: bool,
    pub token_environment: String,
    pub ca_certificate: Option<String>,
    pub limits: SyncLimits,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ServerConfiguration {
    pub service: ServiceConfiguration,
    pub protection: ProtectionConfiguration,
    pub sync: Vec<SyncConfiguration>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConfiguredValue {
    pub section: String,
    pub name: String,
    pub saved: String,
    pub effective: String,
    pub reason: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LocalServerSettings {
    pub version: u32,
    pub revision: String,
    pub desired: ServerConfiguration,
    pub effective: ServerConfiguration,
    pub restart_required: bool,
    pub sync_defaults: SyncLimits,
    pub inventory: Vec<ConfiguredValue>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "section",
    content = "configuration",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ServerSettingsUpdate {
    Service(ServiceConfiguration),
    Protection(ProtectionConfiguration),
    Sync(Vec<SyncConfiguration>),
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SaveServerSettings {
    pub version: u32,
    #[schemars(regex(pattern = "^[0-9a-f]{64}$"))]
    pub expected_revision: String,
    pub update: ServerSettingsUpdate,
}
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("local administrator required")]
    Denied,
    #[error("invalid server settings")]
    Invalid,
    #[error("server configuration changed")]
    Stale,
    #[error("configuration editor busy")]
    Busy,
    #[error("server settings unavailable")]
    Unavailable,
    #[error("save outcome unknown")]
    Unknown,
}
#[async_trait::async_trait]
pub trait ServerSettingsManager: Send + Sync {
    async fn status(&self, actor: PrincipalId) -> Result<LocalServerSettings, Error>;
    async fn save(
        &self,
        actor: PrincipalId,
        request: SaveServerSettings,
    ) -> Result<LocalServerSettings, Error>;
}
