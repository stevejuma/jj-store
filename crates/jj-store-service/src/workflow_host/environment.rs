use super::*;
/// Managed execution settings; attachment paths come from installation discovery.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowRunnerEnvironment")]
pub struct Environment {
    pub version: u32,
    pub host: String,
    #[serde(default)]
    pub labels: BTreeSet<String>,
    #[serde(default)]
    pub commands: BTreeMap<String, Command>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forgejo: Option<Forgejo>,
    #[serde(default)]
    pub agents: BTreeSet<String>,
    pub agent_endpoint: Option<AgentEndpoint>,
    #[serde(default)]
    pub allowed_owners: BTreeSet<m::PrincipalId>,
    pub state_dir: Option<PathBuf>,
    pub jj_executable: Option<PathBuf>,
    #[serde(default = "poll")]
    pub poll_millis: u64,
}
impl Environment {
    pub fn bind(
        &self,
        principal: m::PrincipalId,
        home: PathBuf,
        state_dir: PathBuf,
        jj_executable: PathBuf,
        sources: Vec<Source>,
    ) -> HostConfiguration {
        HostConfiguration {
            version: 1,
            host: self.host.clone(),
            principal,
            home,
            state_dir,
            jj_executable,
            allowed_owners: if self.allowed_owners.is_empty() {
                [principal].into()
            } else {
                self.allowed_owners.clone()
            },
            labels: self.labels.clone(),
            sources,
            commands: self.commands.clone(),
            forgejo: self.forgejo.clone(),
            agents: self.agents.clone(),
            agent_endpoint: self.agent_endpoint.clone(),
            poll_millis: self.poll_millis,
        }
    }
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.version != 2
            || self.state_dir.as_deref().is_some_and(|p| !path(p))
            || self.jj_executable.as_deref().is_some_and(|p| !path(p))
        {
            return Err("Invalid runner environment or overrides");
        }
        let mut configuration = self.bind(
            m::PrincipalId::new([1; 16]),
            "/home".into(),
            "/state".into(),
            "/jj".into(),
            vec![Source {
                repository: m::RepositoryId::new([1; 16]),
                workspace: "/source".into(),
                target_id: None,
            }],
        );
        if configuration.agent_endpoint.is_none() && !configuration.agents.is_empty() {
            configuration.agent_endpoint = Some(AgentEndpoint {
                url: "http://127.0.0.1:1/".into(),
                local: true,
                token_environment: None,
            });
        }
        configuration.validate()
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, schemars::JsonSchema)]
#[serde(untagged)]
#[schemars(rename = "ManagedWorkflowHostConfiguration")]
pub enum HostSettings {
    Legacy(HostConfiguration),
    Registered(Environment),
}
impl From<HostConfiguration> for HostSettings {
    fn from(value: HostConfiguration) -> Self {
        Self::Legacy(value)
    }
}
impl HostSettings {
    pub fn validate(&self) -> Result<(), &'static str> {
        match self {
            Self::Legacy(v) => v.validate(),
            Self::Registered(v) => v.validate(),
        }
    }
    pub fn state_dir(&self) -> Option<&std::path::Path> {
        match self {
            Self::Legacy(v) => Some(&v.state_dir),
            Self::Registered(v) => v.state_dir.as_deref(),
        }
    }
    pub fn poll_millis(&self) -> u64 {
        match self {
            Self::Legacy(v) => v.poll_millis,
            Self::Registered(v) => v.poll_millis,
        }
    }
}
