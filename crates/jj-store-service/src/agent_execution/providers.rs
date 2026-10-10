//! Owned provider capabilities shared by validation, launch and browser forms.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentProvider")]
pub struct Provider {
    pub id: String,
    pub label: String,
    pub runtime_provider: String,
    pub endpoint_env: String,
    pub default_endpoint: Option<String>,
    pub endpoint_required: bool,
    pub credential_env: Option<String>,
    pub model_required: bool,
    pub managed: bool,
    pub guidance: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
}

pub fn canonical(id: Option<&str>) -> &str {
    match id {
        None => "local",
        Some("openai") => "openai-compat",
        Some("databricks-v2") => "databricks_v2",
        Some(id) => id,
    }
}
pub fn get(id: Option<&str>) -> Option<Provider> {
    catalog().into_iter().find(|p| p.id == canonical(id))
}
pub fn managed(id: Option<&str>) -> bool {
    matches!(canonical(id), "local" | "paired")
}

pub fn catalog() -> Vec<Provider> {
    [
        ("local", "This machine", "openai", "OPENAI_COMPAT_BASE_URL", None, None, true, false),
        ("paired", "JJ Store shared compute", "openai", "OPENAI_COMPAT_BASE_URL", None, None, true, false),
        ("anthropic", "Anthropic", "anthropic", "ANTHROPIC_BASE_URL", Some("https://api.anthropic.com"), Some("ANTHROPIC_API_KEY"), false, false),
        ("chatgpt-plan", "ChatGPT plan", "chatgpt-plan", "", Some("https://api.openai.com/v1"), None, false, false),
        ("openai-api", "OpenAI", "openai", "OPENAI_COMPAT_BASE_URL", Some("https://api.openai.com/v1"), Some("OPENAI_COMPAT_API_KEY"), false, false),
        ("openai-compat", "OpenAI-compatible", "openai-compat", "OPENAI_COMPAT_BASE_URL", None, Some("OPENAI_COMPAT_API_KEY"), false, true),
        ("openrouter", "OpenRouter", "openrouter", "OPENROUTER_BASE_URL", Some("https://openrouter.ai/api/v1"), Some("OPENROUTER_API_KEY"), false, false),
        ("databricks", "Databricks", "databricks", "DATABRICKS_HOST", None, Some("DATABRICKS_TOKEN"), false, true),
        ("databricks_v2", "Databricks v2", "databricks_v2", "DATABRICKS_HOST", None, Some("DATABRICKS_TOKEN"), false, true),
    ].into_iter().map(|(id,label,runtime,endpoint,default_endpoint,credential,managed,required)| Provider {
        id:id.into(), label:label.into(), runtime_provider:runtime.into(), endpoint_env:endpoint.into(),
        default_endpoint:default_endpoint.map(str::to_owned), endpoint_required:required,
        credential_env:credential.map(str::to_owned), model_required:!managed, managed,
        authentication: (id=="chatgpt-plan").then(||"oauth".into()),
        available: (id=="chatgpt-plan").then_some(cfg!(target_os="macos")),
        guidance: match id {
            "chatgpt-plan" => "Connect a ChatGPT account in Settings → Agents → Defaults. Uses your eligible ChatGPT plan allowance.",
            "local" => "Prepare and start a model in Settings → Compute.",
            "paired" => "Uses the trusted inference host selected in Settings → Compute → Sharing.",
            "databricks" | "databricks_v2" => "Set the workspace URL. Use a DATABRICKS_TOKEN credential reference in Advanced or the agent's existing Databricks sign-in. Model discovery never opens sign-in.",
            _ => "Add the provider's credential as a server environment reference or device Keychain secret in Advanced. Model discovery uses this draft.",
        }.into(),
    }).collect()
}
