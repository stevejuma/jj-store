use crate::{config, policy, search::Repository};
use jj_store_model::RepositoryId;
use jj_store_service::search_management::{Error, SearchConfiguration, SearchScope};
use jj_store_tantivy::native;
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

pub struct SettingsFile {
    path: PathBuf,
}
pub struct Snapshot {
    pub revision: String,
    pub entries: Vec<Repository>,
}
impl Snapshot {
    pub fn entry(&self, repo: RepositoryId) -> Option<&Repository> {
        self.entries.iter().find(|e| e.repository == repo)
    }
    pub fn configuration(&self, repo: RepositoryId) -> SearchConfiguration {
        self.entry(repo).map(configuration).unwrap_or_default()
    }
}
pub fn configuration(entry: &Repository) -> SearchConfiguration {
    SearchConfiguration {
        enabled: entry.enabled,
        index_source: entry.source.index_source,
        scopes: entry
            .source
            .scopes
            .iter()
            .map(|(id, scope)| {
                (
                    id.clone(),
                    match scope {
                        native::Scope::Bookmark { name } => {
                            SearchScope::Bookmark { name: name.clone() }
                        }
                        native::Scope::Workspace { name } => {
                            SearchScope::Workspace { name: name.clone() }
                        }
                    },
                )
            })
            .collect(),
        include_prefixes: entry.source.include_prefixes.clone(),
        exclude_prefixes: entry.source.exclude_prefixes.clone(),
        index_bytes: entry.index_bytes,
    }
}
pub fn update(
    entry: Option<&Repository>,
    repo: RepositoryId,
    desired: &SearchConfiguration,
) -> Result<Repository, Error> {
    let mut entry = entry.cloned().unwrap_or(Repository {
        repository: repo,
        enabled: false,
        token_environment: None,
        source: native::Config::default(),
        poll_millis: 1000,
        timeout_seconds: 60,
        index_bytes: 512 * 1024 * 1024,
        snapshot_every: 32,
    });
    entry.enabled = desired.enabled;
    entry.index_bytes = desired.index_bytes;
    entry.source.index_source = desired.index_source;
    entry.source.include_prefixes = desired.include_prefixes.clone();
    entry.source.exclude_prefixes = desired.exclude_prefixes.clone();
    entry.source.scopes = desired
        .scopes
        .iter()
        .map(|(id, scope)| {
            (
                id.clone(),
                match scope {
                    SearchScope::Bookmark { name } => {
                        native::Scope::Bookmark { name: name.clone() }
                    }
                    SearchScope::Workspace { name } => {
                        native::Scope::Workspace { name: name.clone() }
                    }
                },
            )
        })
        .collect();
    crate::search_config::validate(std::slice::from_ref(&entry), false)
        .map_err(|_| Error::Invalid)?;
    Ok(entry)
}
impl SettingsFile {
    pub fn new(path: &Path) -> crate::Result<Self> {
        Ok(Self {
            path: policy::outside_checkout(path)?,
        })
    }
    fn bytes(&self) -> Result<Vec<u8>, Error> {
        policy::read_regular(&self.path, 64 * 1024).map_err(|_| Error::Unavailable)
    }
    pub fn read(&self) -> Result<Snapshot, Error> {
        let bytes = self.bytes()?;
        let text = std::str::from_utf8(&bytes).map_err(|_| Error::Invalid)?;
        let mut config: config::Config = toml::from_str(text).map_err(|_| Error::Invalid)?;
        config
            .resolve_paths(self.path.parent().ok_or(Error::Invalid)?)
            .map_err(|_| Error::Invalid)?;
        config.validate().map_err(|_| Error::Invalid)?;
        if !matches!(
            config.access,
            config::Access::LocalTrusted {
                permission: jj_store_service::auth::Permission::Admin,
                ..
            }
        ) {
            return Err(Error::Unavailable);
        }
        Ok(Snapshot {
            revision: hex::encode(Sha256::digest(&bytes)),
            entries: config.search,
        })
    }
    pub fn lock(&self) -> Result<File, Error> {
        let mut path = self.path.as_os_str().to_owned();
        path.push(".settings.lock");
        policy::exclusive_lock(Path::new(&path)).map_err(|_| Error::Busy)
    }
    pub fn save(
        &self,
        repo: RepositoryId,
        expected: &str,
        desired: &SearchConfiguration,
    ) -> Result<(), Error> {
        let _lock = self.lock()?;
        let snapshot = self.read()?;
        if snapshot.revision != expected {
            return Err(Error::Conflict);
        }
        let entry = update(snapshot.entry(repo), repo, desired)?;
        let bytes = self.bytes()?;
        if hex::encode(Sha256::digest(&bytes)) != expected {
            return Err(Error::Conflict);
        }
        let mut doc: toml::Value =
            toml::from_str(std::str::from_utf8(&bytes).map_err(|_| Error::Invalid)?)
                .map_err(|_| Error::Invalid)?;
        let table = doc.as_table_mut().ok_or(Error::Invalid)?;
        let entries = table
            .entry("search")
            .or_insert_with(|| toml::Value::Array(Vec::new()))
            .as_array_mut()
            .ok_or(Error::Invalid)?;
        let value = toml::Value::try_from(&entry).map_err(|_| Error::Invalid)?;
        if let Some(index) = snapshot.entries.iter().position(|e| e.repository == repo) {
            entries[index] = value;
        } else {
            entries.push(value);
        }
        if entries.len() > 8 {
            return Err(Error::Invalid);
        }
        let output = toml::to_string(&doc).map_err(|_| Error::Invalid)?;
        if output.len() > 64 * 1024 {
            return Err(Error::Invalid);
        }
        let parent = self.path.parent().ok_or(Error::Invalid)?;
        let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|_| Error::Unavailable)?;
        temp.write_all(output.as_bytes())
            .and_then(|()| temp.as_file().sync_all())
            .map_err(|_| Error::Unavailable)?;
        // An editor that does not use the advisory lock is still detected before replacement.
        if self.read()?.revision != expected {
            return Err(Error::Conflict);
        }
        temp.persist(&self.path).map_err(|_| Error::Unknown)?;
        File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|_| Error::Unknown)?;
        Ok(())
    }
}
