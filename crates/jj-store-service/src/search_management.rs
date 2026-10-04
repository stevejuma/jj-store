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
    #[serde(default = "poll_millis")]
    pub poll_millis: u64,
    #[serde(default = "timeout_seconds")]
    pub timeout_seconds: u64,
    #[serde(default = "snapshot_every")]
    pub snapshot_every: u64,
    #[serde(default = "max_text_bytes")]
    pub max_text_bytes: usize,
    #[serde(default = "max_files")]
    pub max_files: usize,
    #[serde(default = "max_work")]
    pub max_work: usize,
    #[serde(default = "max_output_bytes")]
    pub max_output_bytes: usize,
}
fn poll_millis() -> u64 {
    1000
}
fn timeout_seconds() -> u64 {
    60
}
fn snapshot_every() -> u64 {
    32
}
fn max_text_bytes() -> usize {
    1024 * 1024
}
fn max_files() -> usize {
    10_000
}
fn max_work() -> usize {
    1_000_000
}
fn max_output_bytes() -> usize {
    64 * 1024 * 1024
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
            poll_millis: poll_millis(),
            timeout_seconds: timeout_seconds(),
            snapshot_every: snapshot_every(),
            max_text_bytes: max_text_bytes(),
            max_files: max_files(),
            max_work: max_work(),
            max_output_bytes: max_output_bytes(),
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
