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
pub fn id() -> Result<m::RequestId> {
    let mut bytes = [0; 16];
    getrandom::fill(&mut bytes)?;
    Ok(m::RequestId::new(bytes))
}
fn private(path: &Path) -> Result<()> {
    if path.exists() {
        let metadata = fs::symlink_metadata(path)?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err("Git private directory type".into());
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::{MetadataExt as _, PermissionsExt as _};
            if metadata.uid() != fs::metadata(dirs::home_dir().ok_or("device home missing")?)?.uid()
                || metadata.permissions().mode() & 0o077 != 0
            {
                return Err("Git private directory ownership or permissions".into());
            }
        }
    } else {
        let mut builder = fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt as _;
            builder.mode(0o700);
        }
        builder.create(path)?;
    }
    #[cfg(not(unix))]
    {
        return Err("native Git private transport currently requires Unix".into());
    }
    #[allow(unreachable_code)]
    Ok(())
}
fn fingerprint(
    repository: m::RepositoryId,
    remote: m::RequestId,
    approval: m::RequestId,
    url: &str,
) -> m::RecordDigest {
    let mut hash = Sha256::new_with_prefix(b"jj-store/git-device-binding/v1\0");
    hash.update(repository.as_bytes());
    hash.update(remote.as_bytes());
    hash.update(approval.as_bytes());
    hash.update(url.as_bytes());
    m::RecordDigest::new(hash.finalize().into())
}
pub fn approved_url(cwd: &Path, value: &str) -> Result<String> {
    if value.is_empty()
        || value.len() > 4096
        || value.starts_with('-')
        || value.chars().any(char::is_control)
    {
        return Err("invalid Git endpoint".into());
    }
    if let Ok(url) = url::Url::parse(value) {
        if !matches!(url.scheme(), "https" | "ssh")
            || url.host_str().is_none()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || (url.scheme() == "https" && !url.username().is_empty())
            || (url.scheme() == "ssh"
                && (url.username().starts_with('-')
                    || !url
                        .username()
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))))
            || url
                .host_str()
                .is_some_and(|v| v.starts_with('-') || v.chars().any(char::is_control))
        {
            return Err(
                "Git endpoint requires approved HTTPS/SSH without embedded credentials".into(),
            );
        }
        return Ok(url.to_string());
    }
    if value.contains(':') {
        return Err("use an explicit ssh:// Git endpoint".into());
    }
    Ok(cwd
        .join(value)
        .canonicalize()?
        .to_str()
        .ok_or("Git local endpoint encoding")?
        .into())
}
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
        let root = tempfile::Builder::new()
            .prefix("attempt-")
            .tempdir_in(parent)?;
        let bare = root.path().join("objects.git");
        gix::init_bare(&bare)?;
        Ok(Cache { root, bare })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn approved_endpoints_reject_credentials_helpers_and_ssh_options() {
        let directory = tempfile::tempdir().unwrap();
        assert_eq!(
            approved_url(directory.path(), ".").unwrap(),
            directory.path().canonicalize().unwrap().to_str().unwrap()
        );
        assert!(approved_url(directory.path(), "https://example.invalid/code.git").is_ok());
        assert!(approved_url(directory.path(), "ssh://git@example.invalid/code.git").is_ok());
        for value in [
            "https://user:secret@example.invalid/code.git",
            "https://user@example.invalid/code.git",
            "ssh://-oProxyCommand@example.invalid/code.git",
            "ssh://user%20-oProxyCommand@example.invalid/code.git",
            "ext::command",
            "file:///tmp/repo",
            "git@example.invalid:code.git",
            "https://example.invalid/repo?secret=x",
            "--upload-pack=command",
            "https://example.invalid/repo\n",
        ] {
            assert!(approved_url(directory.path(), value).is_err(), "{value}");
        }
        let repository = m::RepositoryId::new([1; 16]);
        let remote = m::RequestId::new([2; 16]);
        let approval = m::RequestId::new([3; 16]);
        assert_ne!(
            fingerprint(repository, remote, approval, "a"),
            fingerprint(repository, remote, approval, "b")
        );
        assert_ne!(
            fingerprint(repository, remote, approval, "a"),
            fingerprint(repository, remote, m::RequestId::new([4; 16]), "a")
        );
    }
    #[cfg(unix)]
    #[test]
    fn private_state_rejects_symlinks_and_public_permissions() {
        use std::os::unix::fs::{PermissionsExt as _, symlink};
        let parent = tempfile::tempdir().unwrap();
        let path = parent.path().join("private");
        private(&path).unwrap();
        let link = parent.path().join("link");
        symlink(&path, &link).unwrap();
        assert!(private(&link).is_err());
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(private(&path).is_err());
    }
}
