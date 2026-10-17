//! Resolve ordinary JJ repositories without loading/reconciling operation heads.
mod discovery;
use crate::local_metadata::{Error, Limits, MetadataStore, Result};
pub use discovery::{DiscoveryError, WorkspaceInfo, inspect_workspace};
use jj_lib::{git_backend::GitBackend, settings::UserSettings};
use std::path::Path;

pub fn metadata_store(settings: &UserSettings, repo_path: &Path) -> Result<MetadataStore> {
    let path = repo_path.join("store");
    if std::fs::read_to_string(path.join("type"))?.trim() != "git" {
        return Err(Error::Invalid(
            "requires an ordinary Git-backed JJ repository",
        ));
    }
    let backend = GitBackend::load(settings, &path).map_err(Error::other)?;
    MetadataStore::open(backend.git_repo_path(), Limits::default())
}

/// Default-only settings for hosts that consume owned interfaces, without importing JJ.
pub fn default_metadata_store(repo_path: &Path) -> Result<MetadataStore> {
    let settings = UserSettings::from_config(jj_lib::config::StackedConfig::with_defaults())
        .map_err(Error::other)?;
    metadata_store(&settings, repo_path)
}
