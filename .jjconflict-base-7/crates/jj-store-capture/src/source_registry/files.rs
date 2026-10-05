use crate::{CaptureResult, agent_capture_files::sync_dir, codex_discovery::PhysicalId};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    fs::{self, File, Metadata, OpenOptions},
    io::{Read as _, Write as _},
    path::{Path, PathBuf},
};
const MAX_BYTES: u64 = 2 * 1024 * 1024;
const MAX_FILES: usize = 8194;
#[derive(Clone)]
pub(super) struct PrivateDirectory {
    logical: PathBuf,
    physical: PathBuf,
    identity: PhysicalId,
    owner: u32,
}
pub(super) struct PrivateLock {
    directory: PrivateDirectory,
    path: PathBuf,
    file: File,
    identity: PhysicalId,
}
impl Drop for PrivateLock {
    fn drop(&mut self) {
        let _ = self.file.unlock();
    }
}
fn private(metadata: &Metadata, owner: u32, directory: bool) -> CaptureResult<PhysicalId> {
    if metadata.file_type().is_symlink()
        || (if directory {
            !metadata.is_dir()
        } else {
            !metadata.is_file()
        })
    {
        return Err("source registry requires regular private state".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt as _, PermissionsExt as _};
        if metadata.uid() != owner || metadata.permissions().mode() & 0o077 != 0 {
            return Err("source registry ownership or permissions".into());
        }
    }
    PhysicalId::from(metadata)
}
fn owner() -> CaptureResult<u32> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        // A fresh anonymous file is owned by the effective filesystem user.
        // Avoid environmental usernames and an unsafe getuid call.
        Ok(tempfile::tempfile()?.metadata()?.uid())
    }
    #[cfg(not(unix))]
    {
        Err("source registry platform is unqualified".into())
    }
}
impl PrivateDirectory {
    pub(super) fn open(path: &Path) -> CaptureResult<Self> {
        Self::open_impl(path, true)?.ok_or_else(|| "source registry directory unavailable".into())
    }
    pub(super) fn existing(path: &Path) -> CaptureResult<Option<Self>> {
        Self::open_impl(path, false)
    }
    fn open_impl(path: &Path, create: bool) -> CaptureResult<Option<Self>> {
        let physical = crate::path_policy::outside_checkout(path)?;
        let logical = if path.is_absolute() {
            path.to_owned()
        } else {
            std::env::current_dir()?.join(path)
        };
        if !physical.exists() {
            if !create {
                return Ok(None);
            }
            let mut builder = fs::DirBuilder::new();
            builder.recursive(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt as _;
                builder.mode(0o700);
            }
            builder.create(&physical)?;
            if let Some(parent) = physical.parent() {
                sync_dir(parent)?;
            }
        }
        let owner = owner()?;
        let identity = private(&fs::symlink_metadata(&physical)?, owner, true)?;
        let directory = Self {
            logical,
            physical,
            identity,
            owner,
        };
        directory.revalidate()?;
        Ok(Some(directory))
    }
    pub(super) fn revalidate(&self) -> CaptureResult<()> {
        if fs::symlink_metadata(&self.logical)?
            .file_type()
            .is_symlink()
            || fs::canonicalize(&self.logical)? != self.physical
            || private(&fs::symlink_metadata(&self.physical)?, self.owner, true)? != self.identity
        {
            return Err("source registry directory changed".into());
        }
        Ok(())
    }
    pub(super) fn is_empty(&self) -> CaptureResult<bool> {
        self.revalidate()?;
        match fs::read_dir(&self.physical)?.next() {
            Some(entry) => {
                entry?;
                Ok(false)
            }
            None => Ok(true),
        }
    }
    fn entry_count(&self) -> CaptureResult<usize> {
        let mut count = 0;
        for entry in fs::read_dir(&self.physical)? {
            entry?;
            count += 1;
            if count > MAX_FILES + 1 {
                return Err("source registry file capacity exhausted".into());
            }
        }
        Ok(count)
    }
    pub(super) fn lock(&self, name: &str) -> CaptureResult<PrivateLock> {
        self.revalidate()?;
        let path = self.physical.join(name);
        let mut options = OpenOptions::new();
        options.read(true).write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        let file = match fs::symlink_metadata(&path) {
            Ok(metadata) => {
                private(&metadata, self.owner, false)?;
                options.open(&path)?
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                if self.entry_count()? >= MAX_FILES {
                    return Err("source registry file capacity exhausted".into());
                }
                options.create_new(true).open(&path)?
            }
            Err(error) => return Err(error.into()),
        };
        let identity = private(&file.metadata()?, self.owner, false)?;
        let lease = PrivateLock {
            directory: self.clone(),
            path,
            file,
            identity,
        };
        lease
            .file
            .try_lock()
            .map_err(|_| "source registry or reader lease unavailable")?;
        lease.revalidate()?;
        Ok(lease)
    }
    pub(super) fn load<T: DeserializeOwned>(&self) -> CaptureResult<Option<T>> {
        self.revalidate()?;
        let path = self.physical.join("sources.json");
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        let identity = private(&metadata, self.owner, false)?;
        if metadata.len() > MAX_BYTES {
            return Err("source registry byte capacity exhausted".into());
        }
        let file = File::open(&path)?;
        if private(&file.metadata()?, self.owner, false)? != identity {
            return Err("source registry file changed".into());
        }
        let mut bytes = Vec::new();
        file.take(MAX_BYTES + 1).read_to_end(&mut bytes)?;
        if bytes.len() as u64 > MAX_BYTES {
            return Err("source registry byte capacity exhausted".into());
        }
        if private(&fs::symlink_metadata(&path)?, self.owner, false)? != identity {
            return Err("source registry file changed".into());
        }
        self.revalidate()?;
        serde_json::from_slice(&bytes)
            .map(Some)
            .map_err(|_| "invalid source registry record".into())
    }
    pub(super) fn persist(&self, value: &impl Serialize) -> CaptureResult<()> {
        self.revalidate()?;
        let bytes = serde_json::to_vec(value)?;
        if bytes.len() as u64 > MAX_BYTES {
            return Err("source registry byte capacity exhausted".into());
        }
        let mut temporary = tempfile::NamedTempFile::new_in(&self.physical)?;
        private(&temporary.as_file().metadata()?, self.owner, false)?;
        temporary.write_all(&bytes)?;
        temporary.as_file().sync_all()?;
        self.revalidate()?;
        temporary
            .persist(self.physical.join("sources.json"))
            .map_err(|e| e.error)?;
        sync_dir(&self.physical)?;
        self.revalidate()
    }
}
impl PrivateLock {
    pub(super) fn validate_peer(&self, other: &Self) -> CaptureResult<()> {
        other.revalidate()?;
        self.validate_directory(&other.directory)
    }
    pub(super) fn validate_directory(&self, directory: &PrivateDirectory) -> CaptureResult<()> {
        self.revalidate()?;
        directory.revalidate()?;
        if self.directory.logical != directory.logical
            || self.directory.physical != directory.physical
            || self.directory.identity != directory.identity
            || self.directory.owner != directory.owner
        {
            return Err("source registry lease directory changed".into());
        }
        Ok(())
    }
    pub(super) fn revalidate(&self) -> CaptureResult<()> {
        self.directory.revalidate()?;
        if private(&self.file.metadata()?, self.directory.owner, false)? != self.identity
            || private(
                &fs::symlink_metadata(&self.path)?,
                self.directory.owner,
                false,
            )? != self.identity
        {
            return Err("source registry lease changed".into());
        }
        Ok(())
    }
}
fn lock_name(name: &str, prefix: &str, digits: usize) -> bool {
    name.strip_prefix(prefix)
        .and_then(|v| v.strip_suffix(".lock"))
        .is_some_and(|v| {
            v.len() == digits
                && v.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reader_lease_explicitly_unlocks_inherited_description() {
        let private = tempfile::tempdir().unwrap();
        let directory = PrivateDirectory::open(&private.path().join("registry")).unwrap();
        let name = "source-11111111111111111111111111111111.lock";
        let lease = directory.lock(name).unwrap();
        let inherited = lease.file.try_clone().unwrap();
        assert!(directory.lock(name).is_err());
        drop(lease);
        assert!(directory.lock(name).is_ok());
        drop(inherited);
    }
}

mod contents;
