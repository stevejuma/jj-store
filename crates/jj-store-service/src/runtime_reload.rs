//! Explicit saved-runtime activation; no process restart or repository authority.
use crate::auth::BearerToken;
use jj_store_model::{PrincipalId, RequestId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub enum Caller {
    Local(PrincipalId),
    Token(BearerToken),
}
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "RuntimeReloadDomain")]
pub enum Domain {
    Service,
    Protection,
    Sync,
    Capture,
    RemoteAgents,
    Search,
    Agents,
    Compute,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "RuntimeReloadCommand")]
pub struct Command {
    pub version: u32,
    pub request_id: RequestId,
    #[schemars(regex(pattern = "^[0-9a-f]{64}$"))]
    pub expected_revision: String,
    #[schemars(length(min = 1, max = 8))]
    pub domains: Vec<Domain>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "RuntimeReloadState")]
pub enum State {
    Running,
    Applied,
    Current,
    Deferred,
    Failed,
    Unknown,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "RuntimeReloadDomainResult")]
pub struct DomainResult {
    pub domain: Domain,
    pub state: State,
    pub reason: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "RuntimeReloadOutcome")]
pub struct Outcome {
    pub request_id: RequestId,
    pub generation: u64,
    pub state: State,
    pub results: Vec<DomainResult>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "RuntimeReloadStatus")]
pub struct Status {
    pub original: Option<Command>,
    pub outcome: Option<Outcome>,
    pub saved_revision: String,
    pub supported: Vec<Domain>,
    pub pending: Vec<Domain>,
    pub generation: u64,
}
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("configuration reload denied")]
    Denied,
    #[error("invalid reload intent")]
    Invalid,
    #[error("saved configuration changed")]
    Stale,
    #[error("configuration reload busy")]
    Busy,
    #[error("reload intent conflict")]
    Conflict,
    #[error("reload unavailable")]
    Unavailable,
    #[error("reload outcome unknown")]
    Unknown,
}
#[async_trait::async_trait]
pub trait Manager: Send + Sync {
    fn freshness(&self) -> Option<tokio::sync::broadcast::Receiver<crate::freshness::Change>> {
        None
    }
    async fn status(&self, caller: Caller) -> Result<Status, Error>;
    async fn reload(&self, caller: Caller, command: Command) -> Result<Outcome, Error>;
    async fn outcome(&self, caller: Caller, command: Command) -> Result<Outcome, Error>;
}
