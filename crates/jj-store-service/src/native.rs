//! Repository-bound authority used by in-process JJ stores and RPC handlers.
use crate::{
    auth::{BearerToken, Clock, Permission, Principal},
    guard::{AuthenticatedGuard, RepositoryAccess, SemanticPolicy},
};
use jj_store_jj::{
    backend::BackendAuthority,
    factories::NativeAuthority,
    objects::NativeObjects,
    operation_stores::{AuthorityResult, OperationAuthority},
    policy::{NativePolicy, PolicyLimits, Protection},
};
use jj_store_model as m;
use jj_store_object::ObjectStore;
use jj_store_wal::{self as wal, inventory::DependencySource as _, publication::*};
use sha2::{Digest as _, Sha256};
use std::{collections::BTreeSet, sync::Arc, time::Duration};

#[derive(Clone, Copy)]
pub struct SessionLimits {
    pub inventory_nodes: usize,
    pub timeout: Duration,
    pub publication: Limits,
    pub policy: PolicyLimits,
}
impl Default for SessionLimits {
    fn default() -> Self {
        Self {
            inventory_nodes: 100_000,
            timeout: Duration::from_secs(30),
            publication: Limits::default(),
            policy: PolicyLimits::default(),
        }
    }
}
struct NativeSemantic {
    store: Arc<dyn ObjectStore>,
    native: NativePolicy,
    inventory_nodes: usize,
}
#[async_trait::async_trait]
impl SemanticPolicy for NativeSemantic {
    async fn validate(
        &self,
        before: &ManifestSnapshot,
        _principal: Principal,
        request: &Request,
        after: &Snapshot,
    ) -> wal::Result<()> {
        if request.transition.is_none() {
            return Ok(());
        }
        let committed = before
            .inventory(self.store.clone(), self.inventory_nodes)
            .await?
            .into_iter()
            .map(|node| node.key)
            .collect();
        self.native
            .validate(before, request, after, &committed)
            .await
    }
}
/// No caller-authored actor and no permissive semantic-policy parameter. Trusted
/// server construction supplies object access and configured protection rules.
pub struct NativeSession {
    repository: m::RepositoryId,
    access: Arc<RepositoryAccess>,
    coordinator: Coordinator,
    limits: SessionLimits,
}
impl NativeSession {
    pub fn new(
        store: Arc<dyn ObjectStore>,
        objects: Arc<NativeObjects>,
        token: BearerToken,
        clock: Arc<dyn Clock>,
        protection: Protection,
        limits: SessionLimits,
    ) -> wal::Result<Self> {
        if limits.inventory_nodes == 0 || limits.timeout.is_zero() {
            return Err(wal::Error::Limit("native session limits"));
        }
        let repository = objects.scope().repository();
        let access = Arc::new(RepositoryAccess::new(
            repository,
            store.clone(),
            token,
            clock,
        ));
        let semantic = Arc::new(NativeSemantic {
            store: store.clone(),
            native: NativePolicy::new(objects.clone(), protection, limits.policy)?,
            inventory_nodes: limits.inventory_nodes,
        });
        let guard = Arc::new(AuthenticatedGuard::new(access.clone(), semantic));
        let coordinator = Coordinator::new(store, repository, objects, guard, limits.publication)?;
        Ok(Self {
            repository,
            access,
            coordinator,
            limits,
        })
    }
    pub async fn principal(&self, permission: Permission) -> wal::Result<Principal> {
        self.access.authorize_current(permission).await
    }
    pub async fn snapshot(&self) -> wal::Result<ManifestSnapshot> {
        self.principal(Permission::Read).await?;
        let snapshot = self.coordinator.read().await?;
        self.principal(Permission::Read).await?;
        Ok(snapshot)
    }
    pub async fn publish_request(&self, request: &Request) -> wal::Result<PublishOutcome> {
        self.coordinator.publish(request).await
    }
    /// Authentication selects the principal; deterministic transition identity
    /// survives process loss without persisting credentials or volatile counters.
    pub async fn transition_request(
        &self,
        old: BTreeSet<m::OperationId>,
        new: m::OperationId,
    ) -> wal::Result<Request> {
        if old.is_empty() || old.len() > 256 {
            return Err(wal::Error::Limit("native transition heads"));
        }
        let principal = self.principal(Permission::Write).await?;
        let id = |domain: &[u8]| {
            let mut hash = Sha256::new_with_prefix(domain);
            hash.update(self.repository.as_bytes());
            hash.update(principal.id().as_bytes());
            hash.update((old.len() as u64).to_be_bytes());
            for id in &old {
                hash.update(id.as_bytes());
            }
            hash.update(new.as_bytes());
            let full = hash.finalize();
            let mut id = [0; 16];
            id.copy_from_slice(&full[..16]);
            id
        };
        Ok(Request {
            principal: principal.id(),
            request_id: m::RequestId::new(id(b"jj-store/native-request/v1")),
            publication_id: m::PublicationId::new(id(b"jj-store/native-publication/v1")),
            transition: Some(Transition::Native {
                remove: old,
                add: new,
            }),
            settings: None,
            result: new.as_bytes().to_vec(),
            kind: SemanticKind::OperationPublished,
        })
    }
    pub(crate) async fn inventory(&self) -> wal::Result<BTreeSet<m::ObjectKey>> {
        tokio::time::timeout(self.limits.timeout, async {
            let base = self.snapshot().await?;
            let inventory = self
                .coordinator
                .inventory(&base, self.limits.inventory_nodes)
                .await?;
            self.principal(Permission::Read).await?;
            Ok(inventory.into_iter().map(|node| node.key).collect())
        })
        .await
        .map_err(|_| wal::Error::Limit("native inventory time"))?
    }
}
#[async_trait::async_trait]
impl BackendAuthority for NativeSession {
    async fn authorize_read(&self) -> AuthorityResult<()> {
        self.principal(Permission::Read).await?;
        Ok(())
    }
    async fn authorize_write(&self) -> AuthorityResult<()> {
        self.principal(Permission::Write).await?;
        Ok(())
    }
    async fn copy_ids(&self, limit: usize) -> AuthorityResult<BTreeSet<m::CopyId>> {
        let ids: BTreeSet<_> = self
            .inventory()
            .await?
            .into_iter()
            .filter_map(|key| {
                if let m::ObjectKey::Copy(id) = key {
                    Some(id)
                } else {
                    None
                }
            })
            .collect();
        if ids.len() > limit {
            return Err(wal::Error::Limit("native copy inventory").into());
        }
        Ok(ids)
    }
}
#[async_trait::async_trait]
impl OperationAuthority for NativeSession {
    async fn authorize_read(&self) -> AuthorityResult<()> {
        BackendAuthority::authorize_read(self).await
    }
    async fn authorize_write(&self) -> AuthorityResult<()> {
        BackendAuthority::authorize_write(self).await
    }
    async fn heads(&self) -> AuthorityResult<BTreeSet<m::OperationId>> {
        Ok(self.snapshot().await?.manifest().heads.clone())
    }
    async fn operation_ids(&self, limit: usize) -> AuthorityResult<BTreeSet<m::OperationId>> {
        let ids: BTreeSet<_> = self
            .inventory()
            .await?
            .into_iter()
            .filter_map(|key| {
                if let m::ObjectKey::Operation(id) = key {
                    Some(id)
                } else {
                    None
                }
            })
            .collect();
        if ids.len() > limit {
            return Err(wal::Error::Limit("native operation inventory").into());
        }
        Ok(ids)
    }
    async fn publish(
        &self,
        old: BTreeSet<m::OperationId>,
        new: m::OperationId,
    ) -> AuthorityResult<()> {
        let request = self.transition_request(old, new).await?;
        match self.coordinator.publish(&request).await? {
            PublishOutcome::Committed(_) => Ok(()),
            PublishOutcome::OutcomeUnknown { .. } => Err(wal::Error::OutcomeUnknown.into()),
        }
    }
}
impl NativeAuthority for NativeSession {
    fn repository(&self) -> m::RepositoryId {
        self.repository
    }
}

/// Creation is a separate operator capability, checked before any object access.
/// Repository admin tokens alone cannot bootstrap repositories.
#[expect(clippy::too_many_arguments)]
pub async fn create_repository(
    store: Arc<dyn ObjectStore>,
    objects: Arc<NativeObjects>,
    operator_policy: &crate::auth::TokenPolicy,
    operator_token: &BearerToken,
    initial_policy: &crate::auth::TokenPolicy,
    epoch: m::EpochId,
    clock: &dyn Clock,
    limits: Limits,
) -> wal::Result<ManifestSnapshot> {
    use crate::auth::Scope;
    if operator_policy.scope() != Scope::Operator
        || initial_policy.scope() != Scope::Repository(objects.scope().repository())
    {
        return Err(wal::Error::Denied);
    }
    operator_policy
        .authorize(
            operator_token,
            Permission::CreateRepository,
            clock.unix_seconds().map_err(|_| wal::Error::Denied)?,
        )
        .map_err(|_| wal::Error::Denied)?;
    let settings = initial_policy
        .encode()
        .map_err(|_| wal::Error::Corrupt("initial repository policy"))?;
    let coordinator = Coordinator::new(
        store,
        objects.scope().repository(),
        objects,
        Arc::new(InitializationOnly),
        limits,
    )?;
    coordinator.initialize(epoch, settings).await
}
struct InitializationOnly;
#[async_trait::async_trait]
impl PublicationGuard for InitializationOnly {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Err(wal::Error::Denied)
    }
    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Err(wal::Error::Denied)
    }
}
