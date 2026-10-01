//! In-process LocalTrusted collaboration facade. Caller composition owns folder
//! admission; filesystem permissions remain the local access boundary.
mod actor;
mod work;
mod work_pages;
pub use work::*;
pub use work_pages::LocalWorkPage;
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum LocalProfile {
    LocalJjGit,
}
use jj_store_jj::local_metadata::{self as metadata, MetadataStore};
use jj_store_model as m;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct LocalRepository {
    pub(crate) metadata: MetadataStore,
    pub(crate) repo_path: PathBuf,
}
impl LocalRepository {
    pub fn new(repo_path: &Path, metadata: MetadataStore) -> Self {
        Self {
            repo_path: repo_path.to_path_buf(),
            metadata,
        }
    }
    pub fn metadata_store(&self) -> &MetadataStore {
        &self.metadata
    }
    pub fn repository(&self) -> crate::work::Result<Option<m::RepositoryId>> {
        Ok(self
            .metadata
            .read()
            .map_err(map_metadata)?
            .map(|s| s.repository()))
    }
    pub(crate) fn proposed_repository() -> crate::work::Result<m::RepositoryId> {
        let mut bytes = [0; 16];
        getrandom::fill(&mut bytes).map_err(|_| crate::work::Error::Unavailable)?;
        Ok(m::RepositoryId::new(bytes))
    }
}
pub(crate) fn map_metadata(error: metadata::Error) -> crate::work::Error {
    use crate::work::Error;
    match error {
        metadata::Error::Stale => Error::Stale,
        metadata::Error::IntentConflict => Error::IntentConflict,
        metadata::Error::Limit => Error::Limit,
        metadata::Error::Invalid(_) => Error::Corrupt,
        _ => Error::Unavailable,
    }
}
