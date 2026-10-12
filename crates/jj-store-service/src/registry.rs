//! Installation-owned discovery metadata. Repository policies remain authority.
use crate::auth::Permission;
use jj_store_model::{RepositoryId, TokenId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryEntry {
    pub id: RepositoryId,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegistrySnapshot {
    pub generation: u64,
    pub repositories: Vec<RepositoryEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EffectivePermission {
    Read,
    Write,
    Admin,
}

impl TryFrom<Permission> for EffectivePermission {
    type Error = Error;

    fn try_from(value: Permission) -> Result<Self, Self::Error> {
        match value {
            Permission::Read => Ok(Self::Read),
            Permission::Write => Ok(Self::Write),
            Permission::Admin => Ok(Self::Admin),
            Permission::CreateRepository
            | Permission::ManageIdentities
            | Permission::ManageAgents => Err(Error::Invalid),
        }
    }
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum Error {
    #[error("repository registry is unavailable")]
    Unavailable,
    #[error("repository alias conflicts with another repository")]
    Conflict,
    #[error("repository registry input is invalid")]
    Invalid,
}

#[async_trait::async_trait]
pub trait RepositoryRegistry: Send + Sync {
    /// A token ID narrows discovery candidates only. Callers must reauthorize.
    async fn snapshot(&self, token: Option<TokenId>) -> Result<RegistrySnapshot, Error>;
    /// Reserve a unique alias before authoritative repository publication.
    async fn reserve(&self, repository: RepositoryId, name: String) -> Result<(), Error>;
    /// Make a reserved repository discoverable after publication is recovered.
    async fn register(
        &self,
        repository: RepositoryId,
        name: String,
        initial_token: Option<TokenId>,
    ) -> Result<(), Error>;
    /// Update discovery candidates after an authoritative policy mutation.
    async fn token_candidate(
        &self,
        repository: RepositoryId,
        token: TokenId,
        granted: bool,
    ) -> Result<(), Error>;
}
