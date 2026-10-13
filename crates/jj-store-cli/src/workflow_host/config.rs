use super::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub repository: m::RepositoryId,
    pub workspace: PathBuf,
    pub target_id: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentEndpoint {
    pub url: String,
    pub local: bool,
    pub token_environment: Option<String>,
}
fn name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
}
pub(super) fn connection(workspace: &Path) -> Result<crate::binding::Connection> {
    let path = workspace.join(".jj/repo");
    let cache = if path.is_file() {
        let bytes = crate::binding::bounded_read(&path, 16 * 1024)?;
        let target = std::str::from_utf8(&bytes)?.trim();
        workspace.join(".jj").join(target)
    } else {
        path
    };
    crate::binding::Connection::read(&cache)
}
pub(super) fn relative(value: &str) -> bool {
    !value.starts_with('/')
        && !value.contains('\\')
        && value
            .split('/')
            .all(|p| !matches!(p, "" | "." | ".." | ".jj" | ".git"))
}
impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        jj_store_server::policy::outside_checkout(path)?;
        let value: Self =
            serde_json::from_slice(&jj_store_server::policy::read_regular(path, 64 * 1024)?)?;
        value.validate()?;
        Ok(value)
    }
    pub fn validate(&self) -> Result<()> {
        if self.version != 1
            || !name(&self.host)
            || self.allowed_owners.is_empty()
            || self.allowed_owners.len() > 64
            || self.sources.is_empty()
            || self.sources.len() > 32
            || self.commands.len() > 32
            || self.agents.len() > 32
            || self.labels.len() > 32
            || !(100..=60_000).contains(&self.poll_millis)
            || !self.home.is_absolute()
            || !self.jj_executable.is_absolute()
            || !self.jj_executable.is_file()
        {
            return Err("Invalid workflow host configuration".into());
        }
        jj_store_server::policy::outside_checkout(&self.state_dir)?;
        let mut repositories = BTreeSet::new();
        for source in &self.sources {
            if !source.workspace.is_absolute() || !repositories.insert(source.repository) {
                return Err("Invalid or duplicate workflow source".into());
            }
            let connection = connection(&source.workspace)?;
            if connection.repository != source.repository {
                return Err("Workflow source binding differs".into());
            }
        }
        connection(&self.home)?;
        for (key, command) in &self.commands {
            if !name(key)
                || !command.program.is_absolute()
                || !command.program.is_file()
                || command.args.len() > 64
                || command
                    .args
                    .iter()
                    .any(|v| v.len() > 4096 || v.contains('\0'))
                || command.environment.len() + command.secrets.len() > 64
                || command.environment.iter().any(|(k, v)| {
                    !name(k) || v.len() > 4096 || v.contains('\0') || k.starts_with("JJ_WORKFLOW_")
                })
                || command
                    .secrets
                    .iter()
                    .any(|(k, v)| !name(k) || !name(v) || k.starts_with("JJ_WORKFLOW_"))
                || command.artifacts.len() > 32
                || command.artifacts.iter().any(|p| !relative(p))
            {
                return Err("Invalid named workflow executor".into());
            }
        }
        if self.labels.iter().chain(&self.agents).any(|v| !name(v))
            || (!self.agents.is_empty() && self.agent_endpoint.is_none())
        {
            return Err("Invalid workflow labels or agents".into());
        }
        if let Some(agent) = &self.agent_endpoint {
            let url = url::Url::parse(&agent.url)?;
            if url.path() != "/"
                || !url.username().is_empty()
                || url.password().is_some()
                || url.query().is_some()
                || url.fragment().is_some()
                || if agent.local {
                    url.scheme() != "http"
                        || !matches!(url.host_str(), Some("127.0.0.1" | "[::1]" | "::1"))
                        || url.port().is_none()
                        || agent.token_environment.is_some()
                } else {
                    url.scheme() != "https"
                        || agent.token_environment.as_ref().is_none_or(|v| !name(v))
                }
            {
                return Err("Invalid admitted agent endpoint".into());
            }
        }
        Ok(())
    }
    pub(super) fn eligible(&self, definition: &w::DefinitionVersion) -> bool {
        self.allowed_owners.contains(&definition.owner)
            && definition.definition.labels.is_subset(&self.labels)
            && self
                .sources
                .iter()
                .any(|s| s.repository == definition.definition.repository)
            && definition
                .definition
                .actions
                .iter()
                .all(|a| match &a.executor {
                    w::Executor::Command { name } => self.commands.contains_key(name),
                    w::Executor::Agent { agent, .. } => self.agents.contains(agent),
                })
    }
}
