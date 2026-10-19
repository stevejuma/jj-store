use super::{Account, Credential};
use crate::{Error, Result};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

/// Native custody is replaceable only by an injected test implementation.
pub trait SecretStore: Send + Sync {
    fn get(&self, id: &str) -> Result<Option<String>>;
    fn set(&self, id: &str, value: &str) -> Result<()>;
    fn remove(&self, id: &str) -> Result<()>;
}
pub struct NativeSecrets;
/// Host-owned namespace keeps independently installed applications separate.
pub struct ScopedNativeSecrets {
    service: &'static str,
}
impl ScopedNativeSecrets {
    pub const fn new(service: &'static str) -> Self {
        Self { service }
    }
}
impl SecretStore for NativeSecrets {
    fn get(&self, id: &str) -> Result<Option<String>> {
        ScopedNativeSecrets::new("md.lapis.jj-store.forge.v1").get(id)
    }
    fn set(&self, id: &str, value: &str) -> Result<()> {
        ScopedNativeSecrets::new("md.lapis.jj-store.forge.v1").set(id, value)
    }
    fn remove(&self, id: &str) -> Result<()> {
        ScopedNativeSecrets::new("md.lapis.jj-store.forge.v1").remove(id)
    }
}
#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
fn entry(service: &str, id: &str) -> Result<keyring::Entry> {
    keyring::Entry::new(service, id).map_err(|_| Error::State)
}
impl SecretStore for ScopedNativeSecrets {
    fn get(&self, id: &str) -> Result<Option<String>> {
        #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
        {
            match entry(self.service, id)?.get_password() {
                Ok(v) => Ok(Some(v)),
                Err(keyring::Error::NoEntry) => Ok(None),
                _ => Err(Error::State),
            }
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            let _ = id;
            Err(Error::Unsupported)
        }
    }
    fn set(&self, id: &str, value: &str) -> Result<()> {
        #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
        {
            entry(self.service, id)?
                .set_password(value)
                .map_err(|_| Error::State)
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            let _ = (id, value);
            Err(Error::Unsupported)
        }
    }
    fn remove(&self, id: &str) -> Result<()> {
        #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
        {
            match entry(self.service, id)?.delete_credential() {
                Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
                _ => Err(Error::State),
            }
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            let _ = id;
            Err(Error::Unsupported)
        }
    }
}

pub struct Store {
    path: PathBuf,
    _lock: File,
    pub accounts: Vec<Account>,
}
impl Store {
    pub fn open(root: &Path) -> Result<Self> {
        if root
            .ancestors()
            .any(|p| p.join(".jj").exists() || p.join(".git").exists())
        {
            return Err(Error::Invalid);
        }
        let mut builder = fs::DirBuilder::new();
        builder.recursive(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(root).map_err(|_| Error::State)?;
        let root = root.canonicalize().map_err(|_| Error::State)?;
        if root
            .ancestors()
            .any(|p| p.join(".jj").exists() || p.join(".git").exists())
        {
            return Err(Error::Invalid);
        }
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(root.join("accounts.lock"))
            .map_err(|_| Error::State)?;
        lock.try_lock().map_err(|_| Error::State)?;
        let path = root.join("accounts.json");
        let accounts: Vec<Account> = match fs::read(&path) {
            Ok(b) if b.len() <= 256 * 1024 => {
                serde_json::from_slice(&b).map_err(|_| Error::State)?
            }
            Ok(_) => return Err(Error::Limit),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => vec![],
            Err(_) => return Err(Error::State),
        };
        if accounts.len() > 64 {
            return Err(Error::Limit);
        }
        Ok(Self {
            path,
            _lock: lock,
            accounts,
        })
    }
    pub fn save(&self) -> Result<()> {
        let root = self.path.parent().ok_or(Error::State)?;
        let bytes = serde_json::to_vec(&self.accounts).map_err(|_| Error::State)?;
        if bytes.len() > 256 * 1024 {
            return Err(Error::Limit);
        }
        let mut file = tempfile::NamedTempFile::new_in(root).map_err(|_| Error::State)?;
        file.write_all(&bytes)
            .and_then(|_| file.as_file().sync_all())
            .map_err(|_| Error::State)?;
        file.persist(&self.path).map_err(|_| Error::State)?;
        // Unix permits directory fsync; opening a directory as a normal file
        // fails on Windows even after the atomic replacement has succeeded.
        #[cfg(unix)]
        File::open(root)
            .and_then(|f| f.sync_all())
            .map_err(|_| Error::Unknown)?;
        Ok(())
    }
    pub fn put(
        &mut self,
        mut account: Account,
        value: &Credential,
        secrets: &dyn SecretStore,
    ) -> Result<Account> {
        let old = self.accounts.iter().position(|a| a.id == account.id);
        if old.is_none() && self.accounts.len() == 64 {
            return Err(Error::Limit);
        }
        account.revision = old.map(|i| self.accounts[i].revision + 1).unwrap_or(1);
        let text = zeroize::Zeroizing::new(serde_json::to_string(value).map_err(|_| Error::State)?);
        secrets.set(&account.id, &text)?;
        match old {
            Some(i) => self.accounts[i] = account.clone(),
            None => self.accounts.push(account.clone()),
        }
        self.save()?;
        Ok(account)
    }
}
pub fn account_id(api: &str, user: &str, method: &super::Method) -> String {
    hex::encode(Sha256::digest(
        format!("{api}\n{user}\n{method:?}").as_bytes(),
    ))
}
pub fn root() -> Result<PathBuf> {
    std::env::var_os("JJ_STORE_FORGE_STATE_DIR")
        .map(PathBuf::from)
        .or_else(|| dirs::data_local_dir().map(|p| p.join("jj-store/forges")))
        .ok_or(Error::State)
        .map(|p| p.join("integrations"))
}
