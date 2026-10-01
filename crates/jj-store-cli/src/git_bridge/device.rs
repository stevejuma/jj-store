//! Private device approvals and disposable attempt caches outside checkouts.
use super::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read as _, Write as _},
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Endpoint {
    pub remote: m::RequestId,
    pub name: String,
    pub url: String,
    pub approval: m::RequestId,
    pub fingerprint: m::RecordDigest,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Bindings {
    version: u32,
    repository: m::RepositoryId,
    endpoints: BTreeMap<m::RequestId, Endpoint>,
}
pub struct Device {
    root: PathBuf,
    repository: m::RepositoryId,
    _lock: File,
    bindings: Bindings,
}
pub struct Cache {
    pub root: tempfile::TempDir,
    pub bare: PathBuf,
}
mod approval;
pub use approval::{approved_url, id};
use approval::{fingerprint, private};
impl Device {
    pub fn open(repository: m::RepositoryId, checkout: &Path) -> Result<Self> {
        let base = match std::env::var_os("JJ_STORE_GIT_HOME") {
            Some(value) => {
                let path = PathBuf::from(value);
                if !path.is_absolute() {
                    return Err("Git device root must be absolute".into());
                }
                path
            }
            None => {
                let parent = dirs::data_local_dir()
                    .ok_or("Git device data root unavailable")?
                    .join("jj-store");
                fs::create_dir_all(&parent)?;
                parent.join("git-bridge")
            }
        };
        if base.starts_with(checkout) {
            return Err("Git bindings cannot reside in a checkout".into());
        }
        private(&base)?;
        let base = base.canonicalize()?;
        if base.starts_with(checkout.canonicalize()?) {
            return Err("Git bindings cannot reside in a checkout".into());
        }
        let root = base.join(repository.to_string());
        private(&root)?;
        let lock_path = root.join("device.lock");
        if lock_path.exists() && fs::symlink_metadata(&lock_path)?.file_type().is_symlink() {
            return Err("Git lock symlink".into());
        }
        let mut options = OpenOptions::new();
        options.create(true).read(true).write(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        let lock = options.open(lock_path)?;
        lock.try_lock().map_err(|_| "Git device is busy")?;
        let path = root.join("bindings.json");
        let bindings = if path.exists() {
            let metadata = fs::symlink_metadata(&path)?;
            if !metadata.is_file()
                || metadata.file_type().is_symlink()
                || metadata.len() > 1024 * 1024
            {
                return Err("Git bindings file type/limit".into());
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::{MetadataExt as _, PermissionsExt as _};
                if metadata.uid() != fs::metadata(&root)?.uid()
                    || metadata.permissions().mode() & 0o077 != 0
                {
                    return Err("Git bindings file ownership or permissions".into());
                }
            }
            let mut bytes = Vec::new();
            File::open(&path)?
                .take(1024 * 1024 + 1)
                .read_to_end(&mut bytes)?;
            serde_json::from_slice::<Bindings>(&bytes)?
        } else {
            Bindings {
                version: 1,
                repository,
                endpoints: BTreeMap::new(),
            }
        };
        if bindings.version != 1
            || bindings.repository != repository
            || bindings.endpoints.len() > 256
        {
            return Err("Git binding profile/repository/limit".into());
        }
        for (key, value) in &bindings.endpoints {
            if *key != value.remote
                || !valid_remote_name(&value.name)
                || value.fingerprint
                    != fingerprint(repository, value.remote, value.approval, &value.url)
                || approved_url(Path::new("/"), &value.url)? != value.url
            {
                return Err("Git endpoint binding integrity".into());
            }
        }
        Ok(Self {
            root,
            repository,
            _lock: lock,
            bindings,
        })
    }
    pub fn endpoint(&self, remote: m::RequestId) -> Result<&Endpoint> {
        self.bindings.endpoints.get(&remote).ok_or_else(|| {
            "Git remote has no approved binding on this device; use git remote set-url".into()
        })
    }
    pub fn bind(&mut self, remote: m::RequestId, name: &str, url: String) -> Result<Endpoint> {
        if !valid_remote_name(name)
            || self.bindings.endpoints.len() >= 256
                && !self.bindings.endpoints.contains_key(&remote)
        {
            return Err("Git endpoint name/limit".into());
        }
        let approval = id()?;
        let endpoint = Endpoint {
            remote,
            name: name.into(),
            fingerprint: fingerprint(self.repository, remote, approval, &url),
            approval,
            url,
        };
        self.bindings.endpoints.insert(remote, endpoint.clone());
        self.save()?;
        Ok(endpoint)
    }
    pub fn rename(&mut self, remote: m::RequestId, name: &str) -> Result<()> {
        if !valid_remote_name(name) {
            return Err("Git endpoint name".into());
        }
        if let Some(endpoint) = self.bindings.endpoints.get_mut(&remote) {
            endpoint.name = name.into();
            self.save()?;
        }
        Ok(())
    }
    fn save(&self) -> Result<()> {
        let bytes = serde_json::to_vec(&self.bindings)?;
        if bytes.len() > 1024 * 1024 {
            return Err("Git binding byte limit".into());
        }
        let mut file = tempfile::NamedTempFile::new_in(&self.root)?;
        file.write_all(&bytes)?;
        file.as_file().sync_all()?;
        file.persist(self.root.join("bindings.json"))?;
        File::open(&self.root)?.sync_all()?;
        Ok(())
    }
    pub fn cache(&self) -> Result<Cache> {
        let parent = self.root.join("cache");
        private(&parent)?;
        // The repository lock excludes live attempts. Native records, not these
        // abandoned disposable caches, retain accepted objects and outcomes.
        for (count, entry) in fs::read_dir(&parent)?.enumerate() {
            if count >= 256 {
                return Err("Git abandoned cache limit".into());
            }
            let entry = entry?;
            if !entry.file_name().to_string_lossy().starts_with("attempt-") {
                return Err("unexpected Git cache entry".into());
            }
            private(&entry.path())?;
            fs::remove_dir_all(entry.path())?;
        }
        let mut builder = tempfile::Builder::new();
        builder.prefix("attempt-");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            builder.permissions(fs::Permissions::from_mode(0o700));
        }
        let root = builder.tempdir_in(parent)?;
        let bare = root.path().join("objects.git");
        gix::init_bare(&bare)?;
        Ok(Cache { root, bare })
    }
}

#[cfg(test)]
mod tests;
