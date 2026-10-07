//! Installation lifecycle independent of repository/session selection.
use super::*;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentFleetCommand")]
pub struct Control {
    pub version: u32,
    pub request_id: RequestId,
    pub agent_id: String,
    pub start: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentFleetStatus")]
pub struct LiveAgent {
    pub agent_id: String,
    pub state: String,
    pub enabled: bool,
    pub generation: u64,
    pub error: Option<String>,
    #[serde(default)]
    pub restart_required: bool,
    #[serde(default)]
    pub inference_host: Option<String>,
}
