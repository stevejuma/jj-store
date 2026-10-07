//! Device compute capabilities; shared inference grants no repository access.
use super::*;
use std::collections::BTreeMap;
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
#[schemars(rename = "ComputeSettings")]
pub struct Settings {
    pub revision: u64,
    pub model: Option<String>,
    pub memory_gb: Option<u64>,
    pub sharing: bool,
    pub peers: BTreeMap<String, Peer>,
    pub relay_urls: Vec<String>,
    pub target_peer: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ComputePeer")]
pub struct Peer {
    pub name: String,
    pub invitation: serde_json::Value,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ComputeModel")]
pub struct Model {
    pub id: String,
    pub name: String,
    pub installed: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ComputeStatus")]
pub struct Status {
    pub hardware: serde_json::Value,
    pub supported: bool,
    pub state: String,
    pub settings: Settings,
    pub models: Vec<Model>,
    pub runtime_prepared: bool,
    pub invitation: Option<serde_json::Value>,
    pub identity: Option<String>,
    pub progress: Option<serde_json::Value>,
    pub error: Option<String>,
    pub operation: Option<Outcome>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ComputeCommand")]
pub struct Command {
    pub version: u32,
    pub request_id: RequestId,
    pub expected_revision: u64,
    pub action: Action,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(rename = "ComputeAction")]
pub enum Action {
    Configure {
        settings: Settings,
    },
    Prepare,
    Download,
    Import {
        path: String,
    },
    Remove {
        model: String,
    },
    Start,
    Stop,
    Pair {
        name: String,
        invitation: serde_json::Value,
    },
    Revoke {
        identity: String,
    },
}
