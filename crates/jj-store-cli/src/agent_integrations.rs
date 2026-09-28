use crate::{
    agent_capture_files::{restrict_directory, sync_dir},
    agent_probe,
};
use jj_agent::{DeviceId, IntegrationId};
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_model::RepositoryId;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::Write as _,
    path::{Path, PathBuf},
};

const INTEGRATIONS: [IntegrationId; 3] = [
    IntegrationId::Codex,
    IntegrationId::Cursor,
    IntegrationId::OpenCode,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HarnessSupportV1 {
    Absent,
    Supported,
    UnsupportedVersion,
    ProbeFailed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationStateV1 {
    Disabled,
    Enabled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationSetupV1 {
    NotInstalled,
    Installed,
    Drifted,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrationStatusV1 {
    pub integration: IntegrationId,
    pub expected_version: String,
    pub executable: Option<PathBuf>,
    pub detected_version: Option<String>,
    pub support: HarnessSupportV1,
    pub state: IntegrationStateV1,
    pub setup: IntegrationSetupV1,
}

impl IntegrationStatusV1 {
    pub(crate) fn detected(
        integration: IntegrationId,
        expected: &str,
        executable: Option<PathBuf>,
        detected_version: Option<String>,
        support: HarnessSupportV1,
    ) -> Self {
        Self {
            integration,
            expected_version: expected.into(),
            executable,
            detected_version,
            support,
            state: IntegrationStateV1::Disabled,
            setup: IntegrationSetupV1::NotInstalled,
        }
    }

    fn validate(&self, key: IntegrationId) -> AuthorityResult<()> {
        if self.integration != key
            || !INTEGRATIONS.contains(&key)
            || self.expected_version.is_empty()
            || self.expected_version.len() > 128
            || self
                .detected_version
                .as_ref()
                .is_some_and(|value| value.is_empty() || value.len() > 128)
            || (self.support == HarnessSupportV1::Supported
                && (self.detected_version.as_deref() != Some(&self.expected_version)
                    || self.executable.is_none()))
        {
            return Err("invalid agent integration status".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentDeviceStatusV1 {
    pub schema_version: u32,
    pub device_id: DeviceId,
    pub repository: RepositoryId,
    pub workspace: PathBuf,
    pub integrations: BTreeMap<IntegrationId, IntegrationStatusV1>,
}

impl AgentDeviceStatusV1 {
    pub fn integration(&self, integration: IntegrationId) -> Option<&IntegrationStatusV1> {
        self.integrations.get(&integration)
    }

    pub(crate) fn validate(&self) -> AuthorityResult<()> {
        let expected = INTEGRATIONS.into_iter().collect::<BTreeSet<_>>();
        if self.schema_version != 1
            || self.device_id == DeviceId::new([0; 16])
            || self.integrations.keys().copied().collect::<BTreeSet<_>>() != expected
        {
            return Err("invalid agent device registry".into());
        }
        for (key, value) in &self.integrations {
            value.validate(*key)?;
        }
        Ok(())
    }
}

pub struct AgentIntegrationRegistry {
    directory: PathBuf,
    status: AgentDeviceStatusV1,
    _lock: File,
}

impl AgentIntegrationRegistry {
    pub fn open(
        directory: &Path,
        workspace: &Path,
        repository: RepositoryId,
    ) -> AuthorityResult<Self> {
        let directory = jj_store_server::policy::outside_checkout(directory)?;
        fs::create_dir_all(&directory)?;
        restrict_directory(&directory)?;
        let lock = jj_store_server::policy::exclusive_lock(&directory.join("registry.lock"))?;
        let path = directory.join("registry.json");
        let workspace = workspace.canonicalize()?;
        let status = if path.exists() {
            let value: AgentDeviceStatusV1 =
                serde_json::from_slice(&jj_store_server::policy::read_regular(&path, 1024 * 1024)?)
                    .map_err(|_| "invalid agent device registry")?;
            value.validate()?;
            if value.repository != repository || value.workspace != workspace {
                return Err("agent registry belongs to another repository or workspace".into());
            }
            value
        } else {
            initial_status(repository, workspace)?
        };
        let registry = Self {
            directory,
            status,
            _lock: lock,
        };
        registry.save()?;
        Ok(registry)
    }

    pub fn status(&self) -> &AgentDeviceStatusV1 {
        &self.status
    }

    pub(crate) fn state_directory(&self) -> &Path {
        &self.directory
    }

    pub(crate) fn set_setup(
        &mut self,
        integration: IntegrationId,
        setup: IntegrationSetupV1,
    ) -> AuthorityResult<()> {
        self.status
            .integrations
            .get_mut(&integration)
            .ok_or("unsupported agent integration")?
            .setup = setup;
        self.save()
    }

    pub fn refresh(&mut self, search: &[PathBuf]) -> AuthorityResult<AgentDeviceStatusV1> {
        for integration in INTEGRATIONS {
            let state = self
                .status
                .integrations
                .get(&integration)
                .ok_or("missing integration registry entry")?
                .state;
            let setup = self
                .status
                .integrations
                .get(&integration)
                .ok_or("missing integration registry entry")?
                .setup;
            let mut status = agent_probe::probe(integration, search)?;
            status.state = state;
            status.setup = setup;
            self.status.integrations.insert(integration, status);
        }
        self.status.validate()?;
        self.save()?;
        Ok(self.status.clone())
    }

    pub fn refresh_environment(&mut self) -> AuthorityResult<AgentDeviceStatusV1> {
        self.refresh(&agent_probe::environment_search_paths())
    }

    pub fn enable(
        &mut self,
        integrations: &[IntegrationId],
        dry_run: bool,
    ) -> AuthorityResult<AgentDeviceStatusV1> {
        if integrations.is_empty() || integrations.len() > INTEGRATIONS.len() {
            return Err("select at least one supported agent integration".into());
        }
        let mut status = self.status.clone();
        let mut unique = BTreeSet::new();
        for integration in integrations {
            if !unique.insert(*integration) {
                return Err("duplicate agent integration".into());
            }
            let entry = status
                .integrations
                .get_mut(integration)
                .ok_or("unsupported agent integration")?;
            if entry.support != HarnessSupportV1::Supported {
                return Err("agent integration version is not supported".into());
            }
            entry.state = IntegrationStateV1::Enabled;
        }
        status.validate()?;
        if !dry_run {
            self.status = status.clone();
            self.save()?;
        }
        Ok(status)
    }

    fn save(&self) -> AuthorityResult<()> {
        self.status.validate()?;
        let mut temporary = tempfile::NamedTempFile::new_in(&self.directory)?;
        temporary.write_all(&serde_json::to_vec_pretty(&self.status)?)?;
        temporary.as_file().sync_all()?;
        temporary
            .persist(self.directory.join("registry.json"))
            .map_err(|error| error.error)?;
        sync_dir(&self.directory)
    }
}

fn initial_status(
    repository: RepositoryId,
    workspace: PathBuf,
) -> AuthorityResult<AgentDeviceStatusV1> {
    let mut device = [0; 16];
    getrandom::fill(&mut device)?;
    let integrations = INTEGRATIONS
        .into_iter()
        .map(|integration| {
            let expected = match integration {
                IntegrationId::Codex => "0.156.1",
                IntegrationId::Cursor => "3.22.7",
                IntegrationId::OpenCode => "1.18.30",
                IntegrationId::OwnedController => unreachable!(),
            };
            (
                integration,
                IntegrationStatusV1::detected(
                    integration,
                    expected,
                    None,
                    None,
                    HarnessSupportV1::Absent,
                ),
            )
        })
        .collect();
    let status = AgentDeviceStatusV1 {
        schema_version: 1,
        device_id: DeviceId::new(device),
        repository,
        workspace,
        integrations,
    };
    status.validate()?;
    Ok(status)
}
