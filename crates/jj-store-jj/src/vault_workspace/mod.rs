//! Host-owned vault checkpoints with independently located JJ working-copy state.
//! Call from a bounded native worker. Never invoke through immutable read APIs.
mod batch_plan;
mod batch_restore;
mod binding;
mod capture;
mod content;
mod initialize;
mod remote;
mod restore;
mod restore_io;
mod scan;
pub use batch_restore::{VaultRestorePreview, VaultRestoreRequest};
pub use content::FileContents;
use jj_lib::{
    config::{ConfigLayer, ConfigSource, StackedConfig},
    default_backend_factories::default_backend_factories,
    local_working_copy::LocalWorkingCopy,
    object_id::ObjectId as _,
    repo::{ReadonlyRepo, Repo as _, RepoLoader},
    settings::UserSettings,
    workspace::Workspace,
};
pub use remote::{
    FetchedHead, GitCredentials, PushIntent, PushOutcome, RemoteTarget, RemoteTransfer,
};
pub use restore::{RestorePreview, RestoreRequest};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The vault backup binding is invalid or changed")]
    Binding,
    #[error("Another vault backup command is running")]
    Busy,
    #[error("Vault backup IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("Vault backup JJ: {0}")]
    Jj(String),
}
pub type Result<T> = std::result::Result<T, Error>;
fn jj(error: impl std::fmt::Display) -> Error {
    Error::Jj(error.to_string())
}

/// Native composition supplies these paths after vault admission.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    pub vault_id: String,
    pub root: PathBuf,
    /// Contains `repo` and `working_copy`; normally outside the vault.
    pub state: PathBuf,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoredBinding {
    version: u32,
    binding: Binding,
    root_identity: String,
}
fn root_identity(root: &Path) -> Result<String> {
    let metadata = fs::symlink_metadata(root)?;
    if !metadata.is_dir() || metadata.is_symlink() {
        return Err(Error::Binding);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        Ok(format!("{}:{}", metadata.dev(), metadata.ino()))
    }
    #[cfg(not(unix))]
    {
        Ok(fs::canonicalize(root)?.to_string_lossy().into_owned())
    }
}
impl Binding {
    pub fn validate(&self) -> Result<()> {
        if self.vault_id.is_empty()
            || self.vault_id.len() > 128
            || !self
                .vault_id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
            || !self.root.is_absolute()
            || !self.state.is_absolute()
            || fs::canonicalize(&self.root)? != self.root
            || !fs::symlink_metadata(&self.root)?.is_dir()
            || (self.state.starts_with(&self.root) && self.state != self.root.join(".jj"))
        {
            return Err(Error::Binding);
        }
        Ok(())
    }
    pub fn metadata(&self) -> PathBuf {
        self.state.join("repo")
    }
    pub fn working_copy(&self) -> PathBuf {
        self.state.join("working_copy")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Checkpoint {
    pub operation: String,
    pub commit: String,
    pub change_id: String,
    pub description: String,
    pub timestamp_millis: i64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Capture {
    pub changed: bool,
    pub checkpoint: Option<Checkpoint>,
    pub operation: String,
    pub working_copy: String,
}

/// Holds an inter-process command lease. Readers use separate immutable handles.
pub struct Session {
    binding: Binding,
    workspace: Workspace,
    repo: Arc<ReadonlyRepo>,
    _lease: File,
}
impl Session {
    pub async fn open(binding: Binding) -> Result<Self> {
        binding.validate()?;
        let lease = binding::lease(&binding)?;
        let settings = settings()?;
        initialize::ensure(&binding, &settings).await?;
        binding.verify_saved()?;
        let loader = RepoLoader::init_from_file_system(
            &settings,
            &binding.metadata(),
            &default_backend_factories(),
        )
        .map_err(jj)?;
        // A managed backup repository owns its head. Concurrent CLI forks must be
        // resolved explicitly rather than reconciling them as a side effect.
        let heads = loader.op_heads_store().get_op_heads().await.map_err(jj)?;
        let [operation] = heads.as_slice() else {
            return Err(Error::Binding);
        };
        let selected = loader.load_operation(operation).await.map_err(jj)?;
        let repo = loader.load_at(&selected).await.map_err(jj)?;
        let working_copy = LocalWorkingCopy::load(
            repo.store().clone(),
            binding.root.clone(),
            binding.working_copy(),
            &settings,
        )
        .map_err(jj)?;
        let workspace = Workspace::new(
            &binding.root,
            binding.metadata(),
            Box::new(working_copy),
            loader,
        )
        .map_err(jj)?;
        Ok(Self {
            binding,
            workspace,
            repo,
            _lease: lease,
        })
    }
    pub fn binding(&self) -> &Binding {
        &self.binding
    }
    pub fn operation(&self) -> String {
        self.repo.op_id().hex()
    }
    pub fn checkpoint(&self) -> Result<Option<Checkpoint>> {
        let target = self
            .repo
            .view()
            .get_local_bookmark(jj_lib::ref_name::RefName::new("lapis-backup"));
        target
            .as_normal()
            .map(|id| {
                self.repo
                    .store()
                    .get_commit(id)
                    .map(|c| checkpoint(&c, &self.repo))
                    .map_err(jj)
            })
            .transpose()
    }
}
fn checkpoint(commit: &jj_lib::commit::Commit, repo: &ReadonlyRepo) -> Checkpoint {
    Checkpoint {
        operation: repo.op_id().hex(),
        commit: commit.id().hex(),
        change_id: commit.change_id().hex(),
        description: commit.description().to_owned(),
        timestamp_millis: commit.committer().timestamp.timestamp.0,
    }
}
fn settings() -> Result<UserSettings> {
    let mut config = StackedConfig::with_defaults();
    config.add_layer(ConfigLayer::parse(ConfigSource::CommandArg,
        "[user]\nname = 'Lapis Notes'\nemail = 'backup@local.invalid'\n[signing]\nbehavior = 'drop'\n").map_err(jj)?);
    UserSettings::from_config(config).map_err(jj)
}
