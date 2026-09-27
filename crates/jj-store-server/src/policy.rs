//! Trusted local operator policy, separate from repository publication state.
use crate::{Error, Result};
use jj_store_model::{PrincipalId, TokenId};
use jj_store_service::{
    administration::OperatorPolicies,
    auth::{self, BearerToken, Permission, Scope, TokenPolicy},
};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read as _, Write as _},
    path::{Path, PathBuf},
};

/// Resolve existing ancestors before checking checkout boundaries. This also
/// works for a not-yet-created state directory, without creating it first.
pub fn outside_checkout(path: &Path) -> Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()?.join(path)
    };
    if fs::symlink_metadata(&absolute).is_ok_and(|m| m.file_type().is_symlink()) {
        return Err(Error::Config("control paths must not be symlinks"));
    }
    let mut ancestor = absolute.as_path();
    let mut suffix = Vec::new();
    while !ancestor.exists() {
        suffix.push(
            ancestor
                .file_name()
                .ok_or(Error::Config("control path"))?
                .to_owned(),
        );
        ancestor = ancestor.parent().ok_or(Error::Config("control path"))?;
    }
    let mut resolved = ancestor.canonicalize()?;
    for component in suffix.into_iter().rev() {
        resolved.push(component);
    }
    for parent in resolved.ancestors() {
        if parent.join(".jj").exists() || parent.join(".git").exists() {
            return Err(Error::Config(
                "credentials and process state must be outside checkouts",
            ));
        }
    }
    Ok(resolved)
}
pub fn read_regular(path: &Path, limit: u64) -> Result<Vec<u8>> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(Error::Config("expected regular configuration file"));
    }
    let file = File::open(path)?;
    if !file.metadata()?.is_file() {
        return Err(Error::Config("expected regular configuration file"));
    }
    let mut bytes = Vec::new();
    file.take(limit + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err(Error::Config("configuration byte limit"));
    }
    Ok(bytes)
}
pub fn exclusive_lock(path: &Path) -> Result<File> {
    let path = outside_checkout(path)?;
    if let Ok(metadata) = fs::symlink_metadata(&path)
        && !metadata.is_file()
    {
        return Err(Error::Config("expected regular lock file"));
    }
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let file = options.open(path)?;
    file.try_lock()
        .map_err(|_| Error::Config("another process holds this state or policy lock"))?;
    Ok(file)
}
#[derive(Clone)]
pub struct PolicyFile {
    path: PathBuf,
}
impl PolicyFile {
    pub fn new(path: PathBuf) -> Result<Self> {
        Ok(Self {
            path: outside_checkout(&path)?,
        })
    }
    pub fn read(&self) -> Result<TokenPolicy> {
        TokenPolicy::decode(&read_regular(&self.path, 64 * 1024)?, Scope::Operator)
            .map_err(|_| Error::Config("invalid operator policy"))
    }
    fn lock(&self) -> Result<File> {
        let mut name = self.path.as_os_str().to_owned();
        name.push(".lock");
        exclusive_lock(Path::new(&name))
    }
    fn persist(&self, policy: &TokenPolicy) -> Result<()> {
        let parent = self
            .path
            .parent()
            .ok_or(Error::Config("operator policy path"))?;
        let bytes = policy
            .encode()
            .map_err(|_| Error::Config("operator policy bounds"))?;
        let mut temp = tempfile::NamedTempFile::new_in(parent)?;
        temp.write_all(&bytes)?;
        temp.as_file().sync_all()?;
        temp.persist(&self.path)
            .map_err(|error| Error::Io(error.error))?;
        #[cfg(unix)]
        File::open(parent)?.sync_all()?;
        Ok(())
    }
    pub fn initialize(&self, principal: PrincipalId, token: &BearerToken) -> Result<()> {
        let _lock = self.lock()?;
        let mut policy = TokenPolicy::new(Scope::Operator);
        policy
            .grant(token, principal, Permission::CreateRepository, None)
            .map_err(|_| Error::Config("operator grant"))?;
        if self.path.exists() {
            if self
                .read()?
                .encode()
                .map_err(|_| Error::Config("operator policy"))?
                == policy
                    .encode()
                    .map_err(|_| Error::Config("operator policy"))?
            {
                return Ok(());
            }
            return Err(Error::Config(
                "operator policy already exists with different grants",
            ));
        }
        self.persist(&policy)
    }
    pub fn grant(&self, principal: PrincipalId, token: &BearerToken) -> Result<()> {
        let _lock = self.lock()?;
        let mut policy = self.read()?;
        policy
            .grant(token, principal, Permission::CreateRepository, None)
            .map_err(|_| Error::Config("operator grant conflicts or exceeds limit"))?;
        self.persist(&policy)
    }
    pub fn revoke(&self, token: TokenId) -> Result<bool> {
        let _lock = self.lock()?;
        let mut policy = self.read()?;
        let removed = policy.revoke(token);
        if removed {
            self.persist(&policy)?;
        }
        Ok(removed)
    }
}
#[async_trait::async_trait]
impl OperatorPolicies for PolicyFile {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.read())
            .await
            .map_err(|_| auth::Error::Policy)?
            .map_err(|_| auth::Error::Policy)
    }
}
