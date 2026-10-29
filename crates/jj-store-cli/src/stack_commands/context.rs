use super::*;
use jj_lib::{
    config::StackedConfig,
    default_backend_factories::{default_backend_factories, default_working_copy_factories},
    settings::UserSettings,
    ui_path::RepoPathUiConverter,
    workspace::Workspace,
};
use jj_store_model::RepositoryId;
use sha2::{Digest as _, Sha256};

pub(super) struct LocalAccess {
    root: PathBuf,
    metadata: PathBuf,
    repository: RepositoryId,
}
#[async_trait::async_trait]
impl local_browser::Access for LocalAccess {
    fn repository(&self) -> RepositoryId {
        self.repository
    }
    fn generation(&self) -> u64 {
        1
    }
    fn metadata_path(&self) -> &Path {
        &self.metadata
    }
    async fn authorize(&self) -> local_browser::Result<()> {
        let settings = UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None)
            .map_err(|_| local_browser::Error::Unavailable)?;
        let workspace = Workspace::load(
            &settings,
            &self.root,
            &default_backend_factories(),
            &default_working_copy_factories(),
        )
        .map_err(|_| local_browser::Error::Unavailable)?;
        if workspace.workspace_root() != self.root
            || workspace.repo_path().canonicalize().ok().as_ref() != Some(&self.metadata)
            || fs::read_to_string(self.metadata.join("store/type"))
                .ok()
                .as_deref()
                .map(str::trim)
                != Some("git")
        {
            return Err(local_browser::Error::Unavailable);
        }
        Ok(())
    }
}
pub(super) struct Context {
    pub root: PathBuf,
    pub cwd: PathBuf,
    pub access: Arc<dyn local_browser::Access>,
    pub read: local_stacks::ReadSession,
    pub state: State,
}
impl Context {
    pub async fn open(directory: &Path) -> AuthorityResult<Self> {
        let cwd = directory.canonicalize()?;
        let workspace_directory = jj_cli::cli_util::find_workspace_dir(&cwd);
        let settings =
            UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None)?;
        let workspace = Workspace::load(
            &settings,
            workspace_directory,
            &default_backend_factories(),
            &default_working_copy_factories(),
        )?;
        let root = workspace.workspace_root().to_owned();
        let metadata = workspace.repo_path().canonicalize()?;
        if fs::read_to_string(metadata.join("store/type"))?.trim() != "git" {
            return Err("Stacks CLI supports ordinary Git-backed JJ workspaces only".into());
        }
        let hash = Sha256::digest(metadata.as_os_str().as_encoded_bytes());
        let repository = RepositoryId::new(hash[..16].try_into()?);
        let access: Arc<dyn local_browser::Access> = Arc::new(LocalAccess {
            root: root.clone(),
            metadata,
            repository,
        });
        let read = local_stacks::ReadSession::open(access.clone(), root.clone()).await?;
        let state = read.state().await?;
        Ok(Self {
            root,
            cwd,
            access,
            read,
            state,
        })
    }
    pub fn path(&self, value: &str) -> AuthorityResult<String> {
        let path = RepoPathUiConverter::Fs {
            cwd: self.cwd.clone(),
            base: self.root.clone(),
        }
        .parse_file_path(value)?;
        Ok(path.as_internal_file_string().to_owned())
    }
    pub fn active(&self) -> AuthorityResult<&local_stacks::NamedWorkspace> {
        self.state
            .workspaces
            .iter()
            .find(|w| Some(&w.id) == self.state.active.as_ref())
            .ok_or_else(|| "No active Stacks workspace; use stack init or workspace switch".into())
    }
    pub fn workspace(&self, value: &str) -> AuthorityResult<String> {
        self.state
            .workspaces
            .iter()
            .find(|w| w.id == value || w.name == value)
            .map(|w| w.id.clone())
            .ok_or_else(|| "Workspace name or ID is unavailable".into())
    }
    pub fn request(&self, command: Command) -> local_stacks::Request {
        local_stacks::Request {
            version: 1,
            binding_generation: 1,
            operation: Some(self.state.operation),
            revision: Some(self.state.revision),
            command,
        }
    }
    pub fn check_pin(
        &self,
        operation: Option<jj_store_model::OperationId>,
        revision: Option<u64>,
    ) -> AuthorityResult<()> {
        if operation.is_some_and(|op| op != self.state.operation)
            || revision.is_some_and(|rev| rev != self.state.revision)
        {
            return Err(local_stacks::Error::Stale.into());
        }
        Ok(())
    }
}
