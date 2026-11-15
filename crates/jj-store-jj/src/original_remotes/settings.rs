//! Explicit remote save: one physical Git config transaction, no transport.
use super::*;
use sha2::{Digest, Sha256};
use std::{fs, io::Read as _};

impl OriginalRepository {
    fn configuration_path(&self) -> Result<PathBuf> {
        let backend = git::get_git_backend(self.repo.store()).map_err(|_| Error::Unsupported)?;
        Ok(backend.git_repo().common_dir().join("config"))
    }
    fn configuration_bytes(&self) -> Result<Vec<u8>> {
        let path = self.configuration_path()?;
        let metadata = fs::symlink_metadata(&path).map_err(|_| Error::Binding)?;
        if !metadata.is_file() || metadata.is_symlink() || metadata.len() > 256 * 1024 {
            return Err(Error::Binding);
        }
        let mut bytes = Vec::new();
        fs::File::open(path)
            .map_err(|_| Error::Binding)?
            .take(256 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| Error::Binding)?;
        if bytes.len() > 256 * 1024 {
            return Err(Error::Limit);
        }
        Ok(bytes)
    }
    pub(super) fn configuration_revision(&self) -> Result<String> {
        Ok(Sha256::digest(self.configuration_bytes()?)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect())
    }
    /// The host's explicit save supplies the exact discovery revision. Existing
    /// origin is never chosen or replaced implicitly. Both URLs are deliberately
    /// saved to the selected endpoint, avoiding inherited hidden push overrides.
    pub async fn save_remote(
        &self,
        expected: &str,
        name: &str,
        target: &RemoteTarget,
    ) -> Result<Snapshot> {
        self.current().await?;
        target.validate().map_err(|_| Error::Binding)?;
        if name.is_empty()
            || name.len() > 128
            || name == "git"
            || name.starts_with('-')
            || !name
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c))
        {
            return Err(Error::Binding);
        }
        self.configuration_bytes()?;
        let backend = git::get_git_backend(self.repo.store()).map_err(|_| Error::Unsupported)?;
        let database = gix::open(backend.git_repo_path()).map_err(|_| Error::Native)?;
        let path = self.configuration_path()?;
        let mut config = database.config_file_mut(&path).map_err(|_| Error::Stale)?;
        if self.configuration_revision()? != expected {
            return Err(Error::Stale);
        }
        let refspec = format!("+refs/heads/*:refs/remotes/{name}/*");
        let mut remote = database
            .remote_at(target.url.as_str())
            .map_err(|_| Error::Binding)?
            .with_refspecs([refspec.as_bytes()], gix::remote::Direction::Fetch)
            .map_err(|_| Error::Binding)?
            .with_push_url(target.url.as_str())
            .map_err(|_| Error::Binding)?;
        remote
            .save_as_to(name, &mut config)
            .map_err(|_| Error::Native)?;
        self.current().await?;
        config.commit().map_err(|_| Error::Native)?;
        fs::File::open(path.parent().ok_or(Error::Binding)?)
            .map_err(|_| Error::Native)?
            .sync_all()
            .map_err(|_| Error::Native)?;
        self.snapshot().await
    }
    pub(super) fn validate_push_transport(
        &self,
        name: &str,
        transfer: &RemoteTransfer,
    ) -> Result<()> {
        self.validate_transport(name, transfer)?;
        let backend = git::get_git_backend(self.repo.store()).map_err(|_| Error::Unsupported)?;
        let database = gix::open(backend.git_repo_path()).map_err(|_| Error::Native)?;
        let remote =
            git::try_find_active_remote(&database, jj_lib::ref_name::RemoteName::new(name))
                .map_err(|_| Error::Native)?
                .ok_or(Error::Binding)?;
        if remote
            .url(gix::remote::Direction::Push)
            .or_else(|| remote.url(gix::remote::Direction::Fetch))
            .map(ToString::to_string)
            .as_deref()
            != Some(&transfer.native_target().url)
        {
            return Err(Error::Stale);
        }
        Ok(())
    }
}
