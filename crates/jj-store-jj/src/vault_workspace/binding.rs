use super::*;
use jj_lib::{
    ref_name::WorkspaceName,
    workspace_store::{SimpleWorkspaceStore, WorkspaceStore as _},
};
use std::io::{Read as _, Write as _};
fn saved(binding: &Binding) -> Result<StoredBinding> {
    let mut bytes = Vec::new();
    File::open(binding.state.join("vault.json"))?
        .take(16_385)
        .read_to_end(&mut bytes)?;
    if bytes.len() > 16_384 {
        return Err(Error::Binding);
    }
    serde_json::from_slice(&bytes).map_err(|_| Error::Binding)
}
pub(super) fn lease(binding: &Binding) -> Result<File> {
    let parent = binding.state.parent().ok_or(Error::Binding)?;
    let mut directory = fs::DirBuilder::new();
    directory.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt as _;
        directory.mode(0o700);
    }
    directory.create(parent)?;
    if fs::canonicalize(parent)? != parent {
        return Err(Error::Binding);
    }
    let path = parent.join(format!(".{}-backup.lock", binding.vault_id));
    if fs::symlink_metadata(&path).is_ok_and(|m| m.is_symlink() || !m.is_file()) {
        return Err(Error::Binding);
    }
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let lease = options.open(path)?;
    lease.try_lock().map_err(|_| Error::Busy)?;
    Ok(lease)
}
impl Binding {
    pub(super) fn verify_saved(&self) -> Result<()> {
        self.validate()?;
        let stored = saved(self)?;
        if stored.version != 1
            || stored.binding != *self
            || stored.root_identity != root_identity(&self.root)?
        {
            return Err(Error::Binding);
        }
        Ok(())
    }
    /// Explicit host action after selecting the same physical vault at a new
    /// location. A different inode cannot inherit the old vault's history.
    pub fn relink(&self) -> Result<()> {
        self.validate()?;
        if !self.state.join("vault.json").exists() {
            return Ok(());
        }
        let _lease = lease(self)?;
        let stored = saved(self)?;
        if stored.version != 1
            || stored.binding.vault_id != self.vault_id
            || stored.binding.state != self.state
            || stored.root_identity != root_identity(&self.root)?
        {
            return Err(Error::Binding);
        }
        if stored.binding.root == self.root {
            return Ok(());
        }
        SimpleWorkspaceStore::load(&self.metadata())
            .map_err(jj)?
            .add(WorkspaceName::DEFAULT, &self.root)
            .map_err(jj)?;
        let mut staged = tempfile::NamedTempFile::new_in(&self.state)?;
        staged.write_all(
            &serde_json::to_vec(&StoredBinding {
                version: 1,
                binding: self.clone(),
                root_identity: stored.root_identity,
            })
            .map_err(jj)?,
        )?;
        staged.as_file().sync_all()?;
        staged.persist(self.state.join("vault.json")).map_err(jj)?;
        File::open(&self.state)?.sync_all()?;
        self.verify_saved()
    }
}
