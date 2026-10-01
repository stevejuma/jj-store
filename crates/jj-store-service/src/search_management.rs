//! Owned local configuration for rebuildable text search.
use jj_store_model::RepositoryId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SearchScope {
    Bookmark { name: String },
    Workspace { name: String },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SearchConfiguration {
    pub enabled: bool,
    pub index_source: bool,
    pub scopes: BTreeMap<String, SearchScope>,
    pub include_prefixes: BTreeSet<String>,
    pub exclude_prefixes: BTreeSet<String>,
    pub index_bytes: u64,
}
impl Default for SearchConfiguration {
    fn default() -> Self {
        Self {
            enabled: false,
            index_source: false,
            scopes: BTreeMap::new(),
            include_prefixes: BTreeSet::new(),
            exclude_prefixes: BTreeSet::new(),
            index_bytes: 512 * 1024 * 1024,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SearchOperation {
    #[default]
    Idle,
    Applying,
    Rebuilding,
    Retrying,
    Failed,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LocalSearchSettings {
    pub version: u32,
    pub revision: String,
    pub desired: SearchConfiguration,
    pub effective: SearchConfiguration,
    pub operation: SearchOperation,
    pub error: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SaveSearchSettings {
    pub version: u32,
    pub expected_revision: String,
    pub configuration: SearchConfiguration,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SearchAction {
    Apply,
    Retry,
    Rebuild,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SearchCommand {
    pub version: u32,
    pub expected_revision: String,
    pub action: SearchAction,
}
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("search configuration changed; refresh settings")]
    Conflict,
    #[error("invalid search configuration")]
    Invalid,
    #[error("search operation in progress")]
    Busy,
    #[error("search management unavailable")]
    Unavailable,
    #[error("search save outcome unknown; refresh settings")]
    Unknown,
}
#[async_trait::async_trait]
pub trait SearchManager: Send + Sync {
    async fn status(&self, repository: RepositoryId) -> Result<LocalSearchSettings, Error>;
    async fn save(
        &self,
        repository: RepositoryId,
        request: SaveSearchSettings,
    ) -> Result<LocalSearchSettings, Error>;
    async fn control(
        &self,
        repository: RepositoryId,
        request: SearchCommand,
    ) -> Result<LocalSearchSettings, Error>;
}
