//! Read-only negotiated choices; discovery never submits a prompt or binds code.
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, JsonSchema)]
#[schemars(rename = "AgentExecutionCatalog")]
pub struct Catalog {
    pub runner: String,
    pub facts: serde_json::Value,
}
