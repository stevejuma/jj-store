//! Filesystem admission uses default server settings, never repository executable config.
use jj_lib::{
    config::StackedConfig,
    settings::UserSettings,
    workspace::{DefaultWorkspaceLoaderFactory, WorkspaceLoaderFactory as _},
};
use jj_store_service::{
    local_browser::{Error, FolderProvider, LocalFolder, Result},
    local_repository::LocalRepository,
};
use std::path::Path;

pub struct LocalFolders;
impl FolderProvider for LocalFolders {
    fn inspect(&self, path: &str) -> Result<LocalFolder> {
        if path.len() > 4096 || path.contains('\0') || !Path::new(path).is_absolute() {
            return Err(Error::Invalid);
        }
        let root = Path::new(path)
            .canonicalize()
            .map_err(|_| Error::Unavailable)?;
        for component in [".jj", ".jj/working_copy", ".jj/repo"] {
            let info =
                std::fs::symlink_metadata(root.join(component)).map_err(|_| Error::Invalid)?;
            if info.file_type().is_symlink() {
                return Err(Error::Invalid);
            }
        }
        let repo_file = root.join(".jj/repo");
        if repo_file.is_file()
            && std::fs::metadata(&repo_file)
                .map_err(|_| Error::Invalid)?
                .len()
                > 4096
        {
            return Err(Error::Limit);
        }
        let loader = DefaultWorkspaceLoaderFactory
            .create(&root)
            .map_err(|_| Error::Invalid)?;
        let metadata = loader
            .repo_path()
            .canonicalize()
            .map_err(|_| Error::Unavailable)?;
        if std::fs::read_to_string(metadata.join("store/type"))
            .map_err(|_| Error::Invalid)?
            .trim()
            != "git"
        {
            return Err(Error::Unsupported);
        }
        let settings = UserSettings::from_config(StackedConfig::with_defaults())
            .map_err(|_| Error::Unavailable)?;
        let store = jj_store_jj::local_repository::metadata_store(&settings, &metadata)
            .map_err(|_| Error::Unsupported)?;
        let selected = store.identity().map_err(|_| Error::Invalid)?;
        Ok(LocalFolder {
            path: root.to_str().ok_or(Error::Invalid)?.into(),
            metadata: metadata.to_str().ok_or(Error::Invalid)?.into(),
            repository: selected.map(|s| s.repository),
        })
    }
    fn open(&self, folder: &LocalFolder) -> Result<LocalRepository> {
        if self.inspect(&folder.path)? != *folder {
            return Err(Error::IdentityChanged);
        }
        let settings = UserSettings::from_config(StackedConfig::with_defaults())
            .map_err(|_| Error::Unavailable)?;
        let path = Path::new(&folder.metadata);
        let store = jj_store_jj::local_repository::metadata_store(&settings, path)
            .map_err(|_| Error::Unavailable)?;
        Ok(LocalRepository::new(path, store))
    }
}
