//! Request-bound access policies shared by native RPC and browser transports.
use crate::auth::{self, AccessPrincipal, BearerToken, Clock, Permission, Scope, TokenPolicy};
use jj_store_model::{PrincipalId, RepositoryId};
use jj_store_object::ObjectStore;
use jj_store_wal::{self as wal, publication::ManifestSnapshot};
use std::sync::Arc;

#[async_trait::async_trait]
pub trait AccessPolicy: Send + Sync {
    fn repository(&self) -> RepositoryId;
    async fn authorize(
        &self,
        snapshot: &ManifestSnapshot,
        required: Permission,
    ) -> wal::Result<AccessPrincipal>;
}

/// Additional caller and repository scope enforced at each read and publication.
/// Implementations hold credentials only in memory; they never replace repository grants.
#[async_trait::async_trait]
pub trait Delegation: Send + Sync {
    fn credential(&self, repository: RepositoryId) -> auth::Result<BearerToken>;
    async fn principal(
        &self,
        repository: RepositoryId,
        required: Permission,
    ) -> auth::Result<PrincipalId>;
}

struct AuthenticatedAccessPolicy {
    repository: RepositoryId,
    store: Arc<dyn ObjectStore>,
    token: BearerToken,
    clock: Arc<dyn Clock>,
    delegation: Option<Arc<dyn Delegation>>,
}

#[async_trait::async_trait]
impl AccessPolicy for AuthenticatedAccessPolicy {
    fn repository(&self) -> RepositoryId {
        self.repository
    }

    async fn authorize(
        &self,
        snapshot: &ManifestSnapshot,
        required: Permission,
    ) -> wal::Result<AccessPrincipal> {
        if snapshot.manifest().repository != self.repository {
            return Err(wal::Error::Denied);
        }
        let data = snapshot.settings(self.store.clone()).await?;
        let policy =
            TokenPolicy::decode(&data, Scope::Repository(self.repository)).map_err(map_auth)?;
        let principal = policy
            .authorize(
                &self.token,
                required,
                self.clock.unix_seconds().map_err(map_auth)?,
            )
            .map_err(map_auth)?;
        if let Some(delegation) = &self.delegation {
            let caller = delegation
                .principal(self.repository, required)
                .await
                .map_err(map_auth)?;
            if caller != principal.id() {
                return Err(wal::Error::Denied);
            }
        }
        Ok(principal)
    }
}

struct LocalTrustedAccessPolicy {
    repository: RepositoryId,
    principal: PrincipalId,
    permission: Permission,
}

#[async_trait::async_trait]
impl AccessPolicy for LocalTrustedAccessPolicy {
    fn repository(&self) -> RepositoryId {
        self.repository
    }

    async fn authorize(
        &self,
        snapshot: &ManifestSnapshot,
        required: Permission,
    ) -> wal::Result<AccessPrincipal> {
        if snapshot.manifest().repository != self.repository {
            return Err(wal::Error::Denied);
        }
        AccessPrincipal::local(
            self.principal,
            Scope::Repository(self.repository),
            self.permission,
            required,
        )
        .map_err(map_auth)
    }
}

pub struct RepositoryAccess {
    repository: RepositoryId,
    store: Arc<dyn ObjectStore>,
    policy: Arc<dyn AccessPolicy>,
}

impl RepositoryAccess {
    pub fn new(
        repository: RepositoryId,
        store: Arc<dyn ObjectStore>,
        token: BearerToken,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self::authenticated(repository, store, token, clock)
    }

    pub fn authenticated(
        repository: RepositoryId,
        store: Arc<dyn ObjectStore>,
        token: BearerToken,
        clock: Arc<dyn Clock>,
    ) -> Self {
        let policy = Arc::new(AuthenticatedAccessPolicy {
            repository,
            store: store.clone(),
            token,
            clock,
            delegation: None,
        });
        Self {
            repository,
            store,
            policy,
        }
    }

    pub fn delegated(
        repository: RepositoryId,
        store: Arc<dyn ObjectStore>,
        clock: Arc<dyn Clock>,
        delegation: Arc<dyn Delegation>,
    ) -> auth::Result<Self> {
        let token = delegation.credential(repository)?;
        let policy = Arc::new(AuthenticatedAccessPolicy {
            repository,
            store: store.clone(),
            token,
            clock,
            delegation: Some(delegation),
        });
        Ok(Self {
            repository,
            store,
            policy,
        })
    }

    pub fn local_trusted(
        repository: RepositoryId,
        store: Arc<dyn ObjectStore>,
        principal: PrincipalId,
        permission: Permission,
    ) -> auth::Result<Self> {
        AccessPrincipal::local(
            principal,
            Scope::Repository(repository),
            permission,
            Permission::Read,
        )?;
        Ok(Self {
            repository,
            store,
            policy: Arc::new(LocalTrustedAccessPolicy {
                repository,
                principal,
                permission,
            }),
        })
    }

    pub async fn authorize_current(&self, required: Permission) -> wal::Result<AccessPrincipal> {
        let snapshot = ManifestSnapshot::load(self.store.clone(), self.repository).await?;
        self.authorize(&snapshot, required).await
    }

    pub(crate) async fn authorize(
        &self,
        snapshot: &ManifestSnapshot,
        required: Permission,
    ) -> wal::Result<AccessPrincipal> {
        self.policy.authorize(snapshot, required).await
    }
}

pub(crate) fn map_auth(error: auth::Error) -> wal::Error {
    match error {
        auth::Error::Policy | auth::Error::Limit => {
            wal::Error::Corrupt("repository authentication policy")
        }
        _ => wal::Error::Denied,
    }
}
