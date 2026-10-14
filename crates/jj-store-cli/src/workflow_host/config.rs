use super::*;
pub use jj_store_service::workflow_host::Command;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Config(pub jj_store_service::workflow_host::HostConfiguration);
impl std::ops::Deref for Config {
    type Target = jj_store_service::workflow_host::HostConfiguration;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        jj_store_server::policy::outside_checkout(path)?;
        let value: Self =
            serde_json::from_slice(&jj_store_server::policy::read_regular(path, 64 * 1024)?)?;
        value.validate()?;
        Ok(value)
    }
    pub fn validate(&self) -> Result<()> {
        self.0.validate()?;
        if !self.jj_executable.is_file() {
            return Err("Workflow JJ executable is unavailable".into());
        }
        jj_store_server::policy::outside_checkout(&self.state_dir)?;
        for source in &self.sources {
            if connection(&source.workspace)?.repository != source.repository {
                return Err("Workflow source binding differs".into());
            }
        }
        connection(&self.home)?;
        if self
            .commands
            .values()
            .any(|command| !command.program.is_file())
        {
            return Err("Workflow executor program is unavailable".into());
        }
        if self
            .forgejo
            .as_ref()
            .is_some_and(|f| !f.program.is_file() || !f.docker_program.is_file())
        {
            return Err("Forgejo runner or Docker executable unavailable".into());
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
                    w::Executor::GithubWorkflow { .. } => self.forgejo.is_some(),
                })
    }
}
