use jj_store_model::PrincipalId;
use jj_store_service::agent_execution::Setup;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::PathBuf};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Configuration {
    pub acp_executable: PathBuf,
    pub jj_executable: PathBuf,
    pub agents: Vec<Agent>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Agent {
    pub setup: Setup,
    pub principals: BTreeSet<PrincipalId>,
}
impl Configuration {
    pub fn validate(&self) -> crate::Result<()> {
        if !cfg!(unix)
            || !self.acp_executable.is_absolute()
            || !self.jj_executable.is_absolute()
            || self.agents.is_empty()
            || self.agents.len() > 64
        {
            return Err(crate::Error::Config("remote agent configuration"));
        }
        let mut ids = BTreeSet::new();
        for agent in &self.agents {
            super::Supervisor::validate_remote_setup(&agent.setup)
                .map_err(|_| crate::Error::Config("remote agent runtime options"))?;
            if agent.setup.id.trim().is_empty()
                || agent.setup.id.len() > 64
                || agent.setup.id.chars().any(char::is_control)
                || !ids.insert(&agent.setup.id)
                || agent.principals.is_empty()
                || agent.principals.len() > 128
                || agent.setup.name.trim().is_empty()
                || agent.setup.name.len() > 128
                || agent.setup.instructions.len() > 32768
                || !matches!(agent.setup.runner.as_str(), "codex" | "cursor" | "local")
                || agent.setup.options.inherit_defaults
            {
                return Err(crate::Error::Config("remote agent definition or allowlist"));
            }
        }
        Ok(())
    }
    pub(super) fn allows(&self, id: &str, principal: PrincipalId) -> bool {
        self.agents
            .iter()
            .any(|a| a.setup.id == id && a.principals.contains(&principal))
    }
    pub(super) fn setups(&self, principal: PrincipalId) -> Vec<Setup> {
        self.agents
            .iter()
            .filter(|a| a.principals.contains(&principal))
            .map(|a| a.setup.clone())
            .collect()
    }
}
