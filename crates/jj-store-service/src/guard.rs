//! Request-bound authentication shared by both publication transports.
use crate::auth::{self, BearerToken, Clock, Permission, Principal, Scope, TokenPolicy};
use jj_store_model::RepositoryId;
use jj_store_object::ObjectStore;
use jj_store_wal::{
    self as wal,
    publication::{ManifestSnapshot, PublicationGuard, Request, Snapshot},
};
use std::sync::Arc;

pub struct RepositoryAccess {
    repository: RepositoryId,
    store: Arc<dyn ObjectStore>,
    token: BearerToken,
    clock: Arc<dyn Clock>,
}
impl RepositoryAccess {
    pub fn new(
        repository: RepositoryId,
        store: Arc<dyn ObjectStore>,
        token: BearerToken,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            repository,
            store,
            token,
            clock,
        }
    }
    /// Read/search/preview requests authenticate against current policy even
    /// when the content snapshot or index generation was selected in the past.
    pub async fn authorize_current(&self, required: Permission) -> wal::Result<Principal> {
        let snapshot = ManifestSnapshot::load(self.store.clone(), self.repository).await?;
        self.authorize(&snapshot, required).await
    }
    async fn authorize(
        &self,
        snapshot: &ManifestSnapshot,
        required: Permission,
    ) -> wal::Result<Principal> {
        if snapshot.manifest().repository != self.repository {
            return Err(wal::Error::Denied);
        }
        let data = snapshot.settings(self.store.clone()).await?;
        let policy =
            TokenPolicy::decode(&data, Scope::Repository(self.repository)).map_err(map_auth)?;
        policy
            .authorize(
                &self.token,
                required,
                self.clock.unix_seconds().map_err(map_auth)?,
            )
            .map_err(map_auth)
    }
}
fn map_auth(error: auth::Error) -> wal::Error {
    match error {
        auth::Error::Policy | auth::Error::Limit => {
            wal::Error::Corrupt("repository authentication policy")
        }
        _ => wal::Error::Denied,
    }
}
/// Native/application policy is mandatory. Authentication is not semantic
/// approval; this crate intentionally supplies no allow-all implementation.
#[async_trait::async_trait]
pub trait SemanticPolicy: Send + Sync {
    async fn validate(
        &self,
        before: &ManifestSnapshot,
        principal: Principal,
        request: &Request,
        after: &Snapshot,
    ) -> wal::Result<()>;
}
pub struct AuthenticatedGuard {
    access: Arc<RepositoryAccess>,
    semantic: Arc<dyn SemanticPolicy>,
}
impl AuthenticatedGuard {
    pub fn new(
        access: impl Into<Arc<RepositoryAccess>>,
        semantic: Arc<dyn SemanticPolicy>,
    ) -> Self {
        Self {
            access: access.into(),
            semantic,
        }
    }
    async fn actor(
        &self,
        snapshot: &ManifestSnapshot,
        request: &Request,
    ) -> wal::Result<Principal> {
        let required = if request.settings.is_some() {
            Permission::Admin
        } else {
            Permission::Write
        };
        let principal = self.access.authorize(snapshot, required).await?;
        if request.principal != principal.id() {
            return Err(wal::Error::Denied);
        }
        Ok(principal)
    }
}
#[async_trait::async_trait]
impl PublicationGuard for AuthenticatedGuard {
    async fn authorize(&self, snapshot: &ManifestSnapshot, request: &Request) -> wal::Result<()> {
        self.actor(snapshot, request).await.map(|_| ())
    }
    async fn validate_transition(
        &self,
        before: &ManifestSnapshot,
        request: &Request,
        after: &Snapshot,
    ) -> wal::Result<()> {
        let principal = self.actor(before, request).await?;
        if let Some(data) = &request.settings {
            TokenPolicy::decode(data, Scope::Repository(before.manifest().repository))
                .map_err(map_auth)?;
        }
        self.semantic
            .validate(before, principal, request, after)
            .await
    }
}
