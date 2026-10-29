//! Filesystem admission uses default server settings, never repository executable config.
use jj_store_service::{
    local_browser::{Error, FolderProvider, LocalFolder, Result},
    local_repository::LocalRepository,
};
use std::path::Path;

pub struct LocalFolders;
impl FolderProvider for LocalFolders {
    fn inspect(&self, path: &str) -> Result<LocalFolder> {
        let source = jj_store_jj::local_repository::inspect_workspace(Path::new(path))
            .map_err(map_discovery)?;
        Ok(LocalFolder {
            path: source.path.to_str().ok_or(Error::Invalid)?.into(),
            metadata: source.metadata.to_str().ok_or(Error::Invalid)?.into(),
            repository: source.repository,
        })
    }

    fn open(&self, folder: &LocalFolder) -> Result<LocalRepository> {
        if self.inspect(&folder.path)? != *folder {
            return Err(Error::IdentityChanged);
        }
        let path = Path::new(&folder.metadata);
        let store = jj_store_jj::local_repository::default_metadata_store(path)
            .map_err(|_| Error::Unavailable)?;
        Ok(LocalRepository::new(path, store))
    }
}

fn map_discovery(error: jj_store_jj::local_repository::DiscoveryError) -> Error {
    use jj_store_jj::local_repository::DiscoveryError as D;
    match error {
        D::Invalid => Error::Invalid,
        D::Unsupported => Error::Unsupported,
        D::Unavailable => Error::Unavailable,
        D::IdentityChanged => Error::IdentityChanged,
        D::Limit => Error::Limit,
    }
}
