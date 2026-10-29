use super::*;
use jj_lib::{
    ref_name::WorkspaceName, simple_workspace_store::SimpleWorkspaceStore,
    workspace_store::WorkspaceStore as _,
};
use std::io::{Read as _, Write as _};
pub(super) fn saved(binding: &Binding) -> Result<StoredBinding> {
    let mut bytes = Vec::new();
    File::open(binding.state.join("vault.json"))?
        .take(512 * 1024 + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > 512 * 1024 {
        return Err(Error::Binding);
    }
    serde_json::from_slice(&bytes).map_err(|_| Error::Binding)
}
pub(super) fn lease(binding: &Binding) -> Result<File> {
    let parent = directory(binding)?;
    let mut directory = fs::DirBuilder::new();
    directory.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt as _;
        directory.mode(0o700);
    }
    directory.create(&parent)?;
    if fs::canonicalize(&parent)? != parent {
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
        if fence(self)?.exists() {
            return Err(Error::Busy);
        }
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
            || (stored.binding.state != self.state
                && !(stored.binding.state == stored.binding.root.join(".jj")
                    && self.state == self.root.join(".jj")))
            || stored.root_identity != root_identity(&self.root)?
        {
            return Err(Error::Binding);
        }
        if stored.binding.root == self.root && stored.binding.state == self.state {
            return Ok(());
        }
        SimpleWorkspaceStore::load(&self.metadata().join("workspace_store"))
            .map_err(jj)?
            .add(WorkspaceName::DEFAULT, &self.root)
            .map_err(jj)?;
        let mut previous_states = stored.previous_states;
        if stored.binding.state != self.state && !previous_states.contains(&stored.binding.state) {
            previous_states.push(stored.binding.state);
        }
        let mut staged = tempfile::NamedTempFile::new_in(&self.state)?;
        staged.write_all(
            &serde_json::to_vec(&StoredBinding {
                version: 1,
                binding: self.clone(),
                root_identity: stored.root_identity,
                previous_states,
                relocations: stored.relocations,
            })
            .map_err(jj)?,
        )?;
        staged.as_file().sync_all()?;
        staged.persist(self.state.join("vault.json")).map_err(jj)?;
        File::open(&self.state)?.sync_all()?;
        self.verify_saved()
    }
}

/// All root-profile control files are excluded from vault capture. Each parent
/// is checked before creating a child so aliases cannot create external state.
pub(super) fn private(root: &Path, name: &str) -> Result<PathBuf> {
    let mut path = root.to_owned();
    for part in [".lapis", name] {
        path.push(part);
        match fs::symlink_metadata(&path) {
            Ok(meta) if !meta.is_dir() || meta.is_symlink() => return Err(Error::Binding),
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let mut builder = fs::DirBuilder::new();
                #[cfg(unix)]
                {
                    use std::os::unix::fs::DirBuilderExt as _;
                    builder.mode(0o700);
                }
                builder.create(&path)?;
            }
            Err(e) => return Err(e.into()),
        }
        if fs::canonicalize(&path)? != path {
            return Err(Error::Binding);
        }
    }
    Ok(path)
}
pub(super) fn directory(binding: &Binding) -> Result<PathBuf> {
    if binding.state == binding.root.join(".jj") {
        private(&binding.root, "vault-history-locks")
    } else {
        Ok(binding.state.parent().ok_or(Error::Binding)?.to_owned())
    }
}
pub(super) fn fence(binding: &Binding) -> Result<PathBuf> {
    let parent = if binding.state == binding.root.join(".jj") {
        binding.root.join(".lapis/vault-history-locks")
    } else {
        binding.state.parent().ok_or(Error::Binding)?.to_owned()
    };
    Ok(parent.join(format!(".{}-storage-moving.json", binding.vault_id)))
}
pub(super) fn transport(binding: &Binding, exclusive: bool) -> Result<File> {
    let parent = directory(binding)?;
    if !parent.exists() || fs::canonicalize(&parent)? != parent {
        return Err(Error::Binding);
    }
    let path = parent.join(format!(".{}-transport.lock", binding.vault_id));
    if fs::symlink_metadata(&path).is_ok_and(|m| !m.is_file() || m.is_symlink()) {
        return Err(Error::Binding);
    }
    let file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .read(true)
        .open(path)?;
    if exclusive {
        file.try_lock().map_err(|_| Error::Busy)?;
    } else {
        file.try_lock_shared().map_err(|_| Error::Busy)?;
    }
    Ok(file)
}
impl Binding {
    pub(super) fn accepts_state(&self, state: &Path) -> bool {
        saved(self).is_ok_and(|stored| {
            self.state == state || stored.previous_states.iter().any(|old| old == state)
        })
    }
}
