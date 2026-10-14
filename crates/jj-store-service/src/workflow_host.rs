//! Operator-owned host configuration shared by settings and the native CLI.
mod environment;
pub use environment::{Environment, HostSettings};
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    sync::{Arc, RwLock},
};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowHostConfiguration")]
pub struct HostConfiguration {
    pub version: u32,
    pub host: String,
    pub principal: m::PrincipalId,
    pub home: PathBuf,
    pub state_dir: PathBuf,
    pub jj_executable: PathBuf,
    pub allowed_owners: BTreeSet<m::PrincipalId>,
    #[serde(default)]
    pub labels: BTreeSet<String>,
    pub sources: Vec<Source>,
    #[serde(default)]
    pub commands: BTreeMap<String, Command>,
    #[serde(default)]
    pub agents: BTreeSet<String>,
    pub agent_endpoint: Option<AgentEndpoint>,
    #[serde(default = "poll")]
    pub poll_millis: u64,
}
fn poll() -> u64 {
    1000
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowHostSource")]
pub struct Source {
    pub repository: m::RepositoryId,
    pub workspace: PathBuf,
    pub target_id: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowHostCommand")]
pub struct Command {
    pub program: PathBuf,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub environment: BTreeMap<String, String>,
    /// Destination variable to operator environment reference; values are never persisted.
    #[serde(default)]
    pub secrets: BTreeMap<String, String>,
    #[serde(default)]
    pub artifacts: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowHostAgentEndpoint")]
pub struct AgentEndpoint {
    pub url: String,
    pub local: bool,
    pub token_environment: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ManagedWorkflowHost")]
pub struct ManagedHost {
    pub enabled: bool,
    pub configuration: HostSettings,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkflowConfiguration {
    pub admitted: BTreeSet<m::PrincipalId>,
    pub local: Option<ManagedHost>,
}
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowHostRuntimeStatus")]
pub struct RuntimeStatus {
    /// Local connection identity for explicit admission, never a credential.
    #[serde(default)]
    pub principal: Option<m::PrincipalId>,
    pub available: bool,
    pub running: bool,
    pub busy: bool,
    pub message: Option<String>,
}
#[async_trait::async_trait]
pub trait Runtime: Send + Sync {
    fn status(&self) -> RuntimeStatus;
    async fn reload(
        &self,
        value: Option<ManagedHost>,
    ) -> Result<(crate::runtime_reload::State, Option<String>), crate::runtime_reload::Error>;
}
pub type RuntimeSlot = Arc<RwLock<Option<Arc<dyn Runtime>>>>;
pub fn name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
}
fn path(value: &std::path::Path) -> bool {
    value.is_absolute()
        && value
            .to_str()
            .is_some_and(|v| v.len() <= 4096 && !v.chars().any(char::is_control))
}
fn relative(value: &str) -> bool {
    !value.starts_with('/')
        && !value.contains('\\')
        && value
            .split('/')
            .all(|p| !matches!(p, "" | "." | ".." | ".jj" | ".git"))
}
impl HostConfiguration {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.version != 1
            || !name(&self.host)
            || self.allowed_owners.is_empty()
            || self.allowed_owners.len() > 64
            || self.sources.is_empty()
            || self.sources.len() > 32
            || self.commands.len() > 32
            || self.agents.len() > 32
            || self.labels.len() > 32
            || !(100..=60000).contains(&self.poll_millis)
            || !path(&self.home)
            || !path(&self.state_dir)
            || !path(&self.jj_executable)
        {
            return Err("Invalid workflow host configuration");
        }
        let mut ids = BTreeSet::new();
        if self
            .sources
            .iter()
            .any(|s| !path(&s.workspace) || !ids.insert(s.repository))
            || self.labels.iter().chain(&self.agents).any(|v| !name(v))
            || (!self.agents.is_empty() && self.agent_endpoint.is_none())
        {
            return Err("Invalid workflow source, labels or agents");
        }
        for (key, c) in &self.commands {
            if !name(key)
                || !path(&c.program)
                || c.args.len() > 64
                || c.args.iter().any(|v| v.len() > 4096 || v.contains('\0'))
                || c.environment.len() + c.secrets.len() > 64
                || c.environment.iter().any(|(k, v)| {
                    !name(k) || v.len() > 4096 || v.contains('\0') || k.starts_with("JJ_WORKFLOW_")
                })
                || c.secrets
                    .iter()
                    .any(|(k, v)| !name(k) || !name(v) || k.starts_with("JJ_WORKFLOW_"))
                || c.artifacts.len() > 32
                || c.artifacts.iter().any(|v| !relative(v))
            {
                return Err("Invalid named workflow executor");
            }
        }
        if let Some(a) = &self.agent_endpoint {
            let u = url::Url::parse(&a.url).map_err(|_| "Invalid admitted agent endpoint")?;
            if u.path() != "/"
                || !u.username().is_empty()
                || u.password().is_some()
                || u.query().is_some()
                || u.fragment().is_some()
                || if a.local {
                    u.scheme() != "http"
                        || !matches!(u.host_str(), Some("127.0.0.1" | "[::1]" | "::1"))
                        || u.port().is_none()
                        || a.token_environment.is_some()
                } else {
                    u.scheme() != "https" || a.token_environment.as_ref().is_none_or(|v| !name(v))
                }
            {
                return Err("Invalid admitted agent endpoint");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn managed_environment_requires_no_repository_fields() {
        let host: ManagedHost = serde_json::from_value(serde_json::json!({
            "enabled": false,
            "configuration": {"version":2,"host":"builder","labels":["linux"],"commands":{"build":{"program":"/bin/sh"}}}
        })).expect("environment-only host configuration");
        assert!(!host.enabled);
        host.configuration.validate().unwrap();
    }
    #[test]
    fn managed_environment_defaults_and_bounds_preserve_authority() {
        let environment: Environment = serde_json::from_value(
            serde_json::json!({"version":2,"host":"builder","agents":["agent"]}),
        )
        .unwrap();
        environment.validate().unwrap();
        assert!(environment.state_dir.is_none());
        assert!(environment.jj_executable.is_none());
        assert!(environment.allowed_owners.is_empty());
        assert_eq!(environment.poll_millis, 1000);
        for patch in [
            serde_json::json!({"version":3}),
            serde_json::json!({"poll_millis":0}),
            serde_json::json!({"state_dir":"relative"}),
            serde_json::json!({"commands":{"build":{"program":"relative"}}}),
        ] {
            let mut value = serde_json::json!({"version":2,"host":"builder"});
            value
                .as_object_mut()
                .unwrap()
                .extend(patch.as_object().unwrap().clone());
            assert!(
                serde_json::from_value::<Environment>(value)
                    .unwrap()
                    .validate()
                    .is_err()
            );
        }
        let unknown = serde_json::json!({"version":2,"host":"builder","sources":[]});
        assert!(serde_json::from_value::<Environment>(unknown).is_err());
    }
}
