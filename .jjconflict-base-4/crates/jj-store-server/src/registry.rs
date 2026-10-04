//! Private installation repository discovery and token candidate index.
use crate::{
    Error, Result,
    installation::Identity,
    registry_persistence::{lock, persist},
};
use jj_store_model::{RepositoryId, TokenId};
use jj_store_service::registry::{self, RegistrySnapshot, RepositoryRegistry};
use std::path::PathBuf;

pub use crate::registry_catalog::{Catalog, valid_name};

#[derive(Clone)]
pub struct Registry {
    path: PathBuf,
    identity: Identity,
}

impl Registry {
    pub fn new(path: PathBuf, identity: Identity) -> Result<Self> {
        Catalog::load(&path, &identity)?;
        Ok(Self { path, identity })
    }
}

#[async_trait::async_trait]
impl RepositoryRegistry for Registry {
    async fn snapshot(
        &self,
        token: Option<TokenId>,
    ) -> std::result::Result<RegistrySnapshot, registry::Error> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
            let catalog = Catalog::load(&this.path, &this.identity)
                .map_err(|_| registry::Error::Unavailable)?;
            let repositories = match token {
                None => catalog.repositories,
                Some(token) => catalog
                    .tokens
                    .binary_search_by_key(&token, |entry| entry.id)
                    .ok()
                    .map(|index| {
                        catalog.tokens[index]
                            .repositories
                            .iter()
                            .filter_map(|id| {
                                catalog
                                    .repositories
                                    .binary_search_by_key(id, |entry| entry.id)
                                    .ok()
                                    .map(|index| catalog.repositories[index].clone())
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
            };
            Ok(RegistrySnapshot {
                generation: catalog.generation,
                repositories,
            })
        })
        .await
        .map_err(|_| registry::Error::Unavailable)?
    }

    async fn reserve(
        &self,
        repository: RepositoryId,
        name: String,
    ) -> std::result::Result<(), registry::Error> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
            let _lock = lock(&this.path)?;
            let mut catalog = Catalog::load(&this.path, &this.identity)?;
            if catalog.reserve_entry(repository, name)? {
                persist(&this.path, &catalog)?;
            }
            Ok::<_, Error>(())
        })
        .await
        .map_err(|_| registry::Error::Unavailable)?
        .map_err(map_registry_error)
    }

    async fn register(
        &self,
        repository: RepositoryId,
        name: String,
        initial_token: Option<TokenId>,
    ) -> std::result::Result<(), registry::Error> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
            let _lock = lock(&this.path)?;
            let mut catalog = Catalog::load(&this.path, &this.identity)?;
            let mut changed = catalog.finalize_entry(repository, name)?;
            if let Some(token) = initial_token {
                changed |= catalog.apply_token(repository, token, true)?;
            }
            if changed {
                persist(&this.path, &catalog)?;
            }
            Ok::<_, Error>(())
        })
        .await
        .map_err(|_| registry::Error::Unavailable)?
        .map_err(map_registry_error)
    }

    async fn token_candidate(
        &self,
        repository: RepositoryId,
        token: TokenId,
        granted: bool,
    ) -> std::result::Result<(), registry::Error> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
            let _lock = lock(&this.path)?;
            let mut catalog = Catalog::load(&this.path, &this.identity)?;
            if catalog.apply_token(repository, token, granted)? {
                persist(&this.path, &catalog)?;
            }
            Ok::<_, Error>(())
        })
        .await
        .map_err(|_| registry::Error::Unavailable)?
        .map_err(|_| registry::Error::Unavailable)
    }
}

fn map_registry_error(error: Error) -> registry::Error {
    match error {
        Error::Config("repository alias") => registry::Error::Conflict,
        _ => registry::Error::Unavailable,
    }
}
