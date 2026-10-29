//! Existing workspace discovery, independent of application services and executables.
use super::{MetadataStore, default_metadata_store};
use jj_lib::workspace::{DefaultWorkspaceLoaderFactory, WorkspaceLoaderFactory as _};
use jj_store_model::RepositoryId;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum DiscoveryError {
    #[error("invalid local repository folder")]
    Invalid,
    #[error("local repository profile is unsupported")]
    Unsupported,
    #[error("local repository is unavailable")]
    Unavailable,
    #[error("local repository identity changed")]
    IdentityChanged,
    #[error("local repository discovery resource limit")]
    Limit,
}
/// Native paths are observations, never an authorization capability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceInfo {
    pub path: PathBuf,
    pub metadata: PathBuf,
    pub git: PathBuf,
    pub repository: Option<RepositoryId>,
}
impl WorkspaceInfo {
    /// Recheck observed workspace paths and carrier identity before opening.
    pub fn open_metadata(&self) -> Result<MetadataStore, DiscoveryError> {
        if inspect_workspace(&self.path)? != *self {
            return Err(DiscoveryError::IdentityChanged);
        }
        default_metadata_store(&self.metadata).map_err(|_| DiscoveryError::Unavailable)
    }
}
/// Read only existing bounded workspace links. Never load a working copy,
/// reconcile heads, initialize identity or read repository executable settings.
pub fn inspect_workspace(path: &Path) -> Result<WorkspaceInfo, DiscoveryError> {
    let text = path.to_str().ok_or(DiscoveryError::Invalid)?;
    if text.len() > 4096 || text.contains('\0') || !path.is_absolute() {
        return Err(DiscoveryError::Invalid);
    }
    let root = path
        .canonicalize()
        .map_err(|_| DiscoveryError::Unavailable)?;
    for component in [".jj", ".jj/working_copy", ".jj/repo"] {
        let info =
            std::fs::symlink_metadata(root.join(component)).map_err(|_| DiscoveryError::Invalid)?;
        if info.file_type().is_symlink() {
            return Err(DiscoveryError::Invalid);
        }
    }
    let repo_file = root.join(".jj/repo");
    if repo_file.is_file()
        && std::fs::metadata(&repo_file)
            .map_err(|_| DiscoveryError::Invalid)?
            .len()
            > 4096
    {
        return Err(DiscoveryError::Limit);
    }
    let loader = DefaultWorkspaceLoaderFactory
        .create(&root)
        .map_err(|_| DiscoveryError::Invalid)?;
    let metadata = loader
        .repo_path()
        .canonicalize()
        .map_err(|_| DiscoveryError::Unavailable)?;
    if std::fs::read_to_string(metadata.join("store/type"))
        .map_err(|_| DiscoveryError::Invalid)?
        .trim()
        != "git"
    {
        return Err(DiscoveryError::Unsupported);
    }
    let store = default_metadata_store(&metadata).map_err(|_| DiscoveryError::Unsupported)?;
    let selected = store.identity().map_err(|_| DiscoveryError::Invalid)?;
    Ok(WorkspaceInfo {
        path: root,
        metadata,
        git: store
            .git_dir()
            .canonicalize()
            .map_err(|_| DiscoveryError::Unavailable)?,
        repository: selected.map(|s| s.repository),
    })
}
