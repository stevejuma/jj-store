//! Installation-owned tooling, separate from session commands and presence.
use super::*;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentRuntimeStatus")]
pub struct Status {
    pub version: u32,
    pub runtimes: Vec<Runtime>,
    pub node_version: Option<String>,
    pub npm_available: bool,
    pub install_supported: bool,
    pub install_unavailable: Option<String>,
    pub pending: Option<Outcome>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentRuntime")]
pub struct Runtime {
    pub id: String,
    pub label: String,
    pub state: String,
    pub adapter_version: Option<String>,
    pub cli_version: Option<String>,
    pub managed: bool,
    pub action: Option<Action>,
    pub detail: String,
    pub guide_url: String,
    pub target_version: Option<String>,
    pub target_cli_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "AgentRuntimeAction")]
pub enum Action {
    Install,
    Update,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentRuntimeCommand")]
pub struct Command {
    pub version: u32,
    pub request_id: RequestId,
    pub runtime_id: String,
    pub action: Action,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentRuntimeOutcome")]
pub struct Outcome {
    pub version: u32,
    pub request_id: RequestId,
    pub runtime_id: String,
    pub state: String,
    pub error: Option<String>,
}
