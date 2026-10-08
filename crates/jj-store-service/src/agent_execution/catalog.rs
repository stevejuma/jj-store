//! Read-only negotiated choices; discovery never submits a prompt or binds code.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, JsonSchema)]
#[schemars(rename = "AgentExecutionCatalog")]
pub struct Catalog {
    #[serde(default)]
    pub status: Option<String>,
    pub runner: String,
    pub facts: serde_json::Value,
}

/// Preview-only secret values never enter the installation journal or responses.
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentCatalogPreview")]
pub struct Preview {
    pub version: u32,
    pub runner: String,
    pub preferences: super::Preferences,
    pub options: super::settings::RuntimeOptions,
    #[serde(default)]
    pub secrets: std::collections::BTreeMap<String, String>,
}
