use super::*;
use std::{fs, io::Write as _, path::Path};
pub(super) const INTERNAL_PREFIX: &str = "jj-store/workspaces/";
#[derive(Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Config {
    pub version: u32,
    pub revision: u64,
    pub namespace: String,
    pub active: Option<String>,
    pub workspaces: Vec<SavedWorkspace>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SavedWorkspace {
    pub id: String,
    pub name: String,
    pub target: String,
    pub applied: Vec<String>,
    pub merge_ref: String,
    pub wip_ref: String,
}
impl Config {
    pub fn active(&self) -> Option<&SavedWorkspace> {
        self.workspaces
            .iter()
            .find(|w| Some(&w.id) == self.active.as_ref())
    }
    pub fn active_mut(&mut self) -> Option<&mut SavedWorkspace> {
        self.workspaces
            .iter_mut()
            .find(|w| Some(&w.id) == self.active.as_ref())
    }
    pub fn validate_name(&self, name: &str, except: Option<&str>) -> Result<()> {
        if name.trim().is_empty()
            || name.len() > 128
            || name.chars().any(char::is_control)
            || self
                .workspaces
                .iter()
                .any(|w| Some(w.id.as_str()) != except && w.name == name.trim())
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    pub fn new_workspace(
        &self,
        name: &str,
        target: &str,
        settings: &jj_lib::settings::UserSettings,
    ) -> Result<SavedWorkspace> {
        self.validate_name(name, None)?;
        if self.workspaces.len() >= 64 {
            return Err(Error::Invalid);
        }
        let id = settings.get_rng().new_change_id(16).hex();
        let prefix = format!("{INTERNAL_PREFIX}{}/{id}", self.namespace);
        Ok(SavedWorkspace {
            id,
            name: name.trim().into(),
            target: target.into(),
            applied: vec![],
            merge_ref: format!("{prefix}/merge"),
            wip_ref: format!("{prefix}/wip"),
        })
    }
}
pub(super) struct Storage {
    directory: PathBuf,
    _lock: fs::File,
}
impl Storage {
    pub fn read_only(root: &Path) -> Result<Config> {
        Self::read_at(&root.join(".jj/jj-store"))
    }
    pub fn open(root: &Path) -> Result<Self> {
        let directory = root.join(".jj/jj-store");
        if let Ok(metadata) = fs::symlink_metadata(&directory)
            && (!metadata.is_dir() || metadata.is_symlink())
        {
            return Err(Error::Invalid);
        }
        fs::create_dir_all(&directory).map_err(jj)?;
        let lock_path = directory.join("stacks.lock");
        if fs::symlink_metadata(&lock_path).is_ok_and(|m| m.is_symlink()) {
            return Err(Error::Invalid);
        }
        let lock = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(lock_path)
            .map_err(jj)?;
        lock.try_lock().map_err(|_| Error::Busy)?;
        Ok(Self {
            directory,
            _lock: lock,
        })
    }
    pub fn read(&self) -> Result<Config> {
        Self::read_at(&self.directory)
    }
    fn read_at(directory: &Path) -> Result<Config> {
        let path = directory.join("stacks.toml");
        let metadata = match fs::symlink_metadata(&path) {
            Ok(value) => value,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Config {
                    version: 1,
                    ..Default::default()
                });
            }
            Err(error) => return Err(jj(error)),
        };
        if !metadata.is_file() || metadata.is_symlink() || metadata.len() > 1024 * 1024 {
            return Err(Error::Invalid);
        }
        let config: Config = toml::from_str(&fs::read_to_string(path).map_err(jj)?).map_err(jj)?;
        if config.version != 1
            || config.workspaces.len() > 64
            || (config.active.is_some() && config.active().is_none())
        {
            return Err(Error::Invalid);
        }
        let mut ids = std::collections::BTreeSet::new();
        for w in &config.workspaces {
            if !ids.insert(&w.id)
                || w.id.len() != 32
                || !w.id.bytes().all(|b| b.is_ascii_hexdigit())
                || w.applied.len() > 4096
                || w.merge_ref != format!("{INTERNAL_PREFIX}{}/{}/merge", config.namespace, w.id)
                || w.wip_ref != format!("{INTERNAL_PREFIX}{}/{}/wip", config.namespace, w.id)
            {
                return Err(Error::Invalid);
            }
        }
        Ok(config)
    }
    pub fn save(&self, config: &Config) -> Result<()> {
        let bytes = toml::to_string_pretty(config).map_err(jj)?;
        if bytes.len() > 1024 * 1024 {
            return Err(Error::Invalid);
        }
        let mut file = tempfile::NamedTempFile::new_in(&self.directory).map_err(jj)?;
        file.write_all(bytes.as_bytes()).map_err(jj)?;
        file.as_file().sync_all().map_err(jj)?;
        file.persist(self.directory.join("stacks.toml"))
            .map_err(jj)?;
        fs::File::open(&self.directory)
            .map_err(jj)?
            .sync_all()
            .map_err(jj)
    }
}
