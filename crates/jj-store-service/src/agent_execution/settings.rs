//! Installation settings and secret references, independent of process implementation.
use super::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "source",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
#[schemars(rename = "AgentEnvironmentValue")]
pub enum EnvironmentValue {
    Literal(String),
    Environment(String),
    Keychain(String),
    Unset,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
#[schemars(rename = "AgentRuntimeOptions")]
pub struct RuntimeOptions {
    pub inherit_defaults: bool,
    pub provider: Option<String>,
    pub endpoint: Option<String>,
    pub environment: BTreeMap<String, EnvironmentValue>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
#[schemars(rename = "AgentSettings")]
pub struct Settings {
    pub revision: u64,
    pub runner: String,
    pub preferences: Preferences,
    pub options: RuntimeOptions,
    pub automatically_mention: bool,
    pub keep_awake: bool,
    pub keychain_supported: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            revision: 0,
            runner: "codex".into(),
            preferences: Preferences {
                model: None,
                effort: None,
                mode: None,
            },
            options: RuntimeOptions::default(),
            automatically_mention: false,
            keep_awake: false,
            keychain_supported: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentSettingsCommand")]
pub struct SettingsCommand {
    pub version: u32,
    pub request_id: RequestId,
    pub expected_revision: u64,
    pub settings: Settings,
}

/// Write-only secret input. Never retained in an installation journal.
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentSecretCommand")]
pub struct SecretCommand {
    pub id: String,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "AgentSecretStatus")]
pub struct SecretStatus {
    pub keychain_supported: bool,
}
