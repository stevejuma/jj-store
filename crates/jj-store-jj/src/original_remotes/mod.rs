//! Explicit original-repository writer. Immutable HistoryReader grants do not authorize it.
mod counts;
mod fetch;
mod objects;
mod push;
mod settings;
use crate::{
    local_browser,
    vault_workspace::{Binding, RemoteTarget, RemoteTransfer},
};
use jj_lib::{
    config::StackedConfig,
    default_backend_factories::{default_backend_factories, default_working_copy_factories},
    git,
    object_id::ObjectId as _,
    repo::{ReadonlyRepo, Repo as _},
    settings::UserSettings,
    workspace::Workspace,
};
pub use push::PushReview;
use serde::Serialize;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Original repository binding is invalid or changed")]
    Binding,
    #[error("Original repository changed; refresh before continuing")]
    Stale,
    #[error("Original repository mutation is not admitted")]
    Denied,
    #[error("This original repository profile is unsupported")]
    Unsupported,
    #[error("Original repository transfer exceeds its bounded profile")]
    Limit,
    #[error("Fetch and reconcile the selected original bookmark before publishing")]
    Diverged,
    #[error("Original repository operation failed")]
    Native,
    #[error("Original repository transport failed; observe a pending push before retrying")]
    Transport,
}
pub type Result<T> = std::result::Result<T, Error>;

/// Native hosts separately admit mutation; read-only access cannot implement this implicitly.
#[async_trait::async_trait]
pub trait MutationAccess: local_browser::Access {
    async fn authorize_mutation(&self) -> local_browser::Result<()>;
}
#[derive(Clone, Debug, Serialize)]
pub struct Remote {
    pub name: String,
    /// Unsupported or credential-bearing existing endpoints are never returned verbatim.
    pub url: Option<String>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Bookmark {
    pub name: String,
    pub commit: Option<String>,
    pub conflicted: bool,
}
#[derive(Clone, Debug, Serialize)]
pub struct Snapshot {
    pub operation: String,
    pub remote_configuration: String,
    pub bookmarks: Vec<Bookmark>,
    pub remotes: Vec<Remote>,
    pub working_copy: String,
}

pub struct OriginalRepository {
    access: Arc<dyn MutationAccess>,
    root: PathBuf,
    workspace: Workspace,
    repo: Arc<ReadonlyRepo>,
}
impl OriginalRepository {
    pub async fn open(access: Arc<dyn MutationAccess>, root: PathBuf) -> Result<Self> {
        access.authorize().await.map_err(|_| Error::Denied)?;
        if !root.is_absolute() || root.canonicalize().map_err(|_| Error::Binding)? != root {
            return Err(Error::Binding);
        }
        let settings = UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None)
            .map_err(|_| Error::Native)?;
        let workspace = Workspace::load(
            &settings,
            &root,
            &default_backend_factories(),
            &default_working_copy_factories(),
        )
        .map_err(|_| Error::Unsupported)?;
        if workspace.workspace_root() != root
            || workspace
                .repo_path()
                .canonicalize()
                .map_err(|_| Error::Binding)?
                != access
                    .metadata_path()
                    .canonicalize()
                    .map_err(|_| Error::Binding)?
        {
            return Err(Error::Binding);
        }
        let loader = workspace.repo_loader();
        let heads = loader
            .op_heads_store()
            .get_op_heads()
            .await
            .map_err(|_| Error::Native)?;
        let [head] = heads.as_slice() else {
            return Err(Error::Stale);
        };
        let operation = loader
            .load_operation(head)
            .await
            .map_err(|_| Error::Native)?;
        let repo = loader
            .load_at(&operation)
            .await
            .map_err(|_| Error::Native)?;
        git::get_git_backend(repo.store()).map_err(|_| Error::Unsupported)?;
        access.authorize().await.map_err(|_| Error::Denied)?;
        Ok(Self {
            access,
            root,
            workspace,
            repo,
        })
    }
    /// Creates no storage. Hosts select its private parent; browser DTOs contain no paths.
    pub fn transport_binding(&self, private: &Path) -> Result<Binding> {
        if !private.is_absolute()
            || private.starts_with(&self.root)
            || private.canonicalize().map_err(|_| Error::Binding)? != private
            || !private.is_dir()
        {
            return Err(Error::Binding);
        }
        Ok(Binding {
            vault_id: format!("original-{}", self.access.repository()),
            root: self.root.clone(),
            state: private.join("original-transport"),
        })
    }
    pub async fn snapshot(&self) -> Result<Snapshot> {
        self.access.authorize().await.map_err(|_| Error::Denied)?;
        let backend = git::get_git_backend(self.repo.store()).map_err(|_| Error::Unsupported)?;
        let database = gix::open(backend.git_repo_path()).map_err(|_| Error::Native)?;
        let remotes = database
            .remote_names()
            .into_iter()
            .filter_map(|name| String::from_utf8(name.into()).ok())
            .map(jj_lib::ref_name::RemoteNameBuf::from)
            .map(|name| {
                let url = git::try_find_active_remote(&database, &name)
                    .ok()
                    .flatten()
                    .and_then(|remote| {
                        remote
                            .url(gix::remote::Direction::Fetch)
                            .map(ToString::to_string)
                    })
                    .filter(|url| {
                        RemoteTarget {
                            url: url.clone(),
                            branch: "main".into(),
                        }
                        .validate()
                        .is_ok()
                    });
                Remote {
                    name: name.as_str().into(),
                    url,
                }
            })
            .collect();
        let bookmarks = self
            .repo
            .view()
            .local_bookmarks()
            .map(|(name, target)| Bookmark {
                name: name.as_str().into(),
                commit: target.as_normal().map(|id| id.hex()),
                conflicted: !target.is_resolved(),
            })
            .collect();
        let working_copy = self
            .repo
            .view()
            .get_wc_commit_id(self.workspace.workspace_name())
            .ok_or(Error::Binding)?
            .hex();
        self.access.authorize().await.map_err(|_| Error::Denied)?;
        Ok(Snapshot {
            operation: self.repo.op_id().hex(),
            remote_configuration: self.configuration_revision()?,
            bookmarks,
            remotes,
            working_copy,
        })
    }
    async fn current(&self) -> Result<()> {
        self.access
            .authorize_mutation()
            .await
            .map_err(|_| Error::Denied)?;
        let heads = self
            .workspace
            .repo_loader()
            .op_heads_store()
            .get_op_heads()
            .await
            .map_err(|_| Error::Native)?;
        if heads.as_slice() != [self.repo.op_id().clone()] {
            return Err(Error::Stale);
        }
        Ok(())
    }
    fn validate_transport(&self, name: &str, transfer: &RemoteTransfer) -> Result<()> {
        let binding = transfer.native_binding();
        if binding.root != self.root
            || binding.state.starts_with(&self.root)
            || binding.state.canonicalize().map_err(|_| Error::Binding)? != binding.state
            || binding.vault_id != format!("original-{}", self.access.repository())
        {
            return Err(Error::Binding);
        }
        let target = transfer.native_target();
        target.validate().map_err(|_| Error::Binding)?;
        if name.is_empty() || name.len() > 128 || name.contains('/') || name == "git" {
            return Err(Error::Binding);
        }
        let backend = git::get_git_backend(self.repo.store()).map_err(|_| Error::Unsupported)?;
        let database = gix::open(backend.git_repo_path()).map_err(|_| Error::Native)?;
        let remote =
            git::try_find_active_remote(&database, jj_lib::ref_name::RemoteName::new(name))
                .map_err(|_| Error::Native)?
                .ok_or(Error::Binding)?;
        if remote
            .url(gix::remote::Direction::Fetch)
            .map(ToString::to_string)
            .as_deref()
            != Some(&target.url)
        {
            return Err(Error::Stale);
        }
        Ok(())
    }
}
