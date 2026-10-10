//! Installation-local ChatGPT registrations; credentials never cross this contract.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentChatGptAccount")]
pub struct Account {
    pub id: String,
    pub label: String,
    pub email: Option<String>,
    pub connected: bool,
    pub plan_enabled: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentChatGptAttempt")]
pub struct Attempt {
    pub id: String,
    pub account: String,
    pub state: String,
    pub error: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "AgentChatGptStatus")]
pub struct Status {
    pub supported: bool,
    pub accounts: Vec<Account>,
    pub attempts: Vec<Attempt>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(rename = "AgentChatGptCommand")]
pub enum Command {
    Connect {
        request_id: super::RequestId,
        account: Option<String>,
    },
    Cancel {
        attempt: String,
    },
    SignOut {
        account: String,
    },
}
