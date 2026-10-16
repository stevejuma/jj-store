use crate::{
    Error, Result,
    installation::Identity,
    registry_persistence::{lock, persist},
};
use jj_store_model::{RepositoryId, TokenId};
use jj_store_service::registry::RepositoryEntry;
use serde::{Deserialize, Serialize};
use std::path::Path;

const MAX_REPOSITORIES: usize = 4096;
const MAX_TOKENS: usize = 16_384;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct TokenEntry {
    pub(crate) id: TokenId,
    pub(crate) repositories: Vec<RepositoryId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub version: u32,
    pub installation: String,
    pub generation: u64,
    pub repositories: Vec<RepositoryEntry>,
    pending_repositories: Vec<RepositoryEntry>,
    pub(crate) tokens: Vec<TokenEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    deleted_repositories: Vec<RepositoryId>,
}

impl Catalog {
    pub(crate) fn remove_entry(&mut self, repository: RepositoryId) -> Result<()> {
        if self.deleted_repositories.binary_search(&repository).is_ok() {
            return Ok(());
        }
        if let Err(index) = self.deleted_repositories.binary_search(&repository) {
            self.deleted_repositories.insert(index, repository);
        }
        self.repositories.retain(|e| e.id != repository);
        self.pending_repositories.retain(|e| e.id != repository);
        for token in &mut self.tokens {
            token.repositories.retain(|r| *r != repository);
        }
        self.tokens.retain(|t| !t.repositories.is_empty());
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or(Error::Config("registry generation"))?;
        Ok(())
    }
    pub fn empty(identity: &Identity) -> Self {
        Self {
            version: 2,
            installation: identity.installation.clone(),
            generation: 1,
            repositories: Vec::new(),
            pending_repositories: Vec::new(),
            tokens: Vec::new(),
            deleted_repositories: Vec::new(),
        }
    }

    pub fn load(path: &Path, identity: &Identity) -> Result<Self> {
        let catalog: Self = serde_json::from_slice(&crate::registry_persistence::read(path)?)
            .map_err(|_| Error::Config("invalid repository registry"))?;
        catalog.validate(identity)?;
        Ok(catalog)
    }

    pub fn register(
        path: &Path,
        identity: &Identity,
        repository: RepositoryId,
        name: String,
    ) -> Result<Self> {
        let _lock = lock(path)?;
        let mut catalog = Self::load(path, identity)?;
        catalog.finalize_entry(repository, name)?;
        persist(path, &catalog)?;
        Ok(catalog)
    }

    pub(crate) fn reserve_entry(&mut self, repository: RepositoryId, name: String) -> Result<bool> {
        if self.deleted_repositories.binary_search(&repository).is_ok() {
            return Err(Error::Config("deleted repository identity"));
        }
        if !valid_name(&name)
            || self
                .repositories
                .iter()
                .chain(&self.pending_repositories)
                .any(|entry| {
                    (entry.name == name && entry.id != repository)
                        || (entry.id == repository && entry.name != name)
                })
        {
            return Err(Error::Config("repository alias"));
        }
        if self
            .repositories
            .binary_search_by_key(&repository, |entry| entry.id)
            .is_ok()
            || self
                .pending_repositories
                .binary_search_by_key(&repository, |entry| entry.id)
                .is_ok()
        {
            return Ok(false);
        }
        let index = self
            .pending_repositories
            .partition_point(|entry| entry.id < repository);
        self.pending_repositories.insert(
            index,
            RepositoryEntry {
                id: repository,
                name,
            },
        );
        Ok(true)
    }

    pub(crate) fn finalize_entry(
        &mut self,
        repository: RepositoryId,
        name: String,
    ) -> Result<bool> {
        if let Ok(index) = self
            .pending_repositories
            .binary_search_by_key(&repository, |entry| entry.id)
        {
            if self.pending_repositories[index].name != name {
                return Err(Error::Config("repository alias"));
            }
            self.pending_repositories.remove(index);
        }
        self.register_entry(repository, name)
    }

    fn register_entry(&mut self, repository: RepositoryId, name: String) -> Result<bool> {
        if self.deleted_repositories.binary_search(&repository).is_ok() {
            return Err(Error::Config("deleted repository identity"));
        }
        if !valid_name(&name)
            || self
                .repositories
                .iter()
                .any(|entry| entry.name == name && entry.id != repository)
        {
            return Err(Error::Config("repository alias"));
        }
        let changed = match self
            .repositories
            .binary_search_by_key(&repository, |entry| entry.id)
        {
            Ok(index) if self.repositories[index].name == name => false,
            Ok(index) => {
                self.repositories[index].name = name;
                true
            }
            Err(index) => {
                self.repositories.insert(
                    index,
                    RepositoryEntry {
                        id: repository,
                        name,
                    },
                );
                true
            }
        };
        if changed {
            self.increment_generation()?;
        }
        Ok(changed)
    }

    pub(crate) fn apply_token(
        &mut self,
        repository: RepositoryId,
        token: TokenId,
        grant: bool,
    ) -> Result<bool> {
        if self
            .repositories
            .binary_search_by_key(&repository, |entry| entry.id)
            .is_err()
        {
            return Err(Error::Config("repository registry entry"));
        }
        let changed = match self.tokens.binary_search_by_key(&token, |entry| entry.id) {
            Ok(index) => {
                let entry = &mut self.tokens[index];
                match entry.repositories.binary_search(&repository) {
                    Ok(position) if !grant => {
                        entry.repositories.remove(position);
                        true
                    }
                    Ok(_) => false,
                    Err(position) if grant => {
                        entry.repositories.insert(position, repository);
                        true
                    }
                    Err(_) => false,
                }
            }
            Err(index) if grant => {
                self.tokens.insert(
                    index,
                    TokenEntry {
                        id: token,
                        repositories: vec![repository],
                    },
                );
                true
            }
            Err(_) => false,
        };
        self.tokens.retain(|entry| !entry.repositories.is_empty());
        if changed {
            self.increment_generation()?;
        }
        Ok(changed)
    }

    fn increment_generation(&mut self) -> Result<()> {
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or(Error::Config("repository registry generation"))?;
        Ok(())
    }

    pub(crate) fn validate(&self, identity: &Identity) -> Result<()> {
        let all_repositories = self
            .repositories
            .iter()
            .chain(&self.pending_repositories)
            .collect::<Vec<_>>();
        let repositories_valid = all_repositories.len() <= MAX_REPOSITORIES
            && all_repositories.iter().all(|entry| valid_name(&entry.name))
            && self
                .repositories
                .windows(2)
                .all(|pair| pair[0].id < pair[1].id)
            && self
                .pending_repositories
                .windows(2)
                .all(|pair| pair[0].id < pair[1].id)
            && unique_names_and_ids(&all_repositories);
        let tokens_valid = self.tokens.len() <= MAX_TOKENS
            && self.tokens.windows(2).all(|pair| pair[0].id < pair[1].id)
            && self.tokens.iter().all(|entry| {
                !entry.repositories.is_empty()
                    && entry.repositories.windows(2).all(|pair| pair[0] < pair[1])
                    && entry.repositories.iter().all(|repository| {
                        self.repositories
                            .binary_search_by_key(repository, |item| item.id)
                            .is_ok()
                    })
            });
        if self.version != 2
            || self.installation != identity.installation
            || self.generation == 0
            || self.deleted_repositories.len() > MAX_REPOSITORIES
            || self.deleted_repositories.windows(2).any(|w| w[0] >= w[1])
            || all_repositories
                .iter()
                .any(|e| self.deleted_repositories.binary_search(&e.id).is_ok())
            || !repositories_valid
            || !tokens_valid
        {
            return Err(Error::Config("invalid repository registry"));
        }
        Ok(())
    }
}

fn unique_names_and_ids(entries: &[&RepositoryEntry]) -> bool {
    let mut names = entries
        .iter()
        .map(|entry| entry.name.as_str())
        .collect::<Vec<_>>();
    names.sort_unstable();
    let mut ids = entries.iter().map(|entry| entry.id).collect::<Vec<_>>();
    ids.sort_unstable();
    names.windows(2).all(|pair| pair[0] != pair[1]) && ids.windows(2).all(|pair| pair[0] != pair[1])
}

pub fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 256
        && name.trim() == name
        && !name.contains('\\')
        && !name.chars().any(char::is_control)
        && name
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}
