//! Request-bound authentication shared by both publication transports.
pub use crate::access::{AccessPolicy, RepositoryAccess};
use crate::{
    access::map_auth,
    auth::{Permission, Principal, Scope, TokenPolicy},
};
use jj_store_wal::{
    self as wal,
    publication::{DomainRequest, ManifestSnapshot, PublicationGuard, Request, Snapshot},
};
use std::sync::Arc;
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
    async fn authorize_domain(
        &self,
        snapshot: &ManifestSnapshot,
        request: &DomainRequest,
    ) -> wal::Result<()> {
        let principal = self.access.authorize(snapshot, Permission::Write).await?;
        if request.principal != principal.id() {
            return Err(wal::Error::Denied);
        }
        Ok(())
    }
    async fn authorize_replication(
        &self,
        snapshot: &ManifestSnapshot,
        request: &jj_store_wal::publication::ApplyOriginsRequestV1,
    ) -> wal::Result<()> {
        let principal = self.access.authorize(snapshot, Permission::Write).await?;
        if request.principal != principal.id() {
            return Err(wal::Error::Denied);
        }
        Ok(())
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
