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
    replication::NativeOperationHeadReducer,
};
use jj_store_model as m;
use jj_store_object::ObjectStore;
use jj_store_wal::publication as replication;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplicaDescriptor {
    pub replica: m::ReplicaId,
    pub log_epoch: m::EpochId,
    pub feed_upper_bound: m::WalPosition,
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
    pub(crate) events: crate::events::Hints,
    pub(crate) repository: m::RepositoryId,
    pub(crate) object_store: Arc<dyn ObjectStore>,
    access: Arc<RepositoryAccess>,
    pub(crate) clock: Arc<dyn Clock>,
    pub(crate) coordinator: Coordinator,
    pub(crate) limits: SessionLimits,
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
        let access = Arc::new(RepositoryAccess::authenticated(
            repository,
            store.clone(),
            token,
            clock.clone(),
        ));
        Self::with_access(store, objects, access, clock, protection, limits)
    }
    pub fn new_local_trusted(
        store: Arc<dyn ObjectStore>,
        objects: Arc<NativeObjects>,
        principal: m::PrincipalId,
        permission: Permission,
        clock: Arc<dyn Clock>,
        protection: Protection,
        limits: SessionLimits,
    ) -> wal::Result<Self> {
        if limits.inventory_nodes == 0 || limits.timeout.is_zero() {
            return Err(wal::Error::Limit("native session limits"));
        }
        let repository = objects.scope().repository();
        let access = Arc::new(
            RepositoryAccess::local_trusted(repository, store.clone(), principal, permission)
                .map_err(|_| wal::Error::Denied)?,
        );
        Self::with_access(store, objects, access, clock, protection, limits)
    }
    fn with_access(
        store: Arc<dyn ObjectStore>,
        objects: Arc<NativeObjects>,
        access: Arc<RepositoryAccess>,
        clock: Arc<dyn Clock>,
        protection: Protection,
        limits: SessionLimits,
    ) -> wal::Result<Self> {
        let repository = objects.scope().repository();
        let semantic = Arc::new(NativeSemantic {
            store: store.clone(),
            native: NativePolicy::new(objects.clone(), protection, limits.policy)?,
            inventory_nodes: limits.inventory_nodes,
        });
        let guard = Arc::new(AuthenticatedGuard::new(access.clone(), semantic));
        let operation_heads = Arc::new(NativeOperationHeadReducer::new(objects.clone()));
        let coordinator = Coordinator::new_with_head_reducer(
            store.clone(),
            repository,
            objects,
            guard,
            operation_heads,
            limits.publication,
        )?;
        Ok(Self {
            events: crate::events::Hints::default(),
            repository,
            object_store: store,
            access,
            clock,
            coordinator,
            limits,
        })
    }
    pub async fn principal(&self, permission: Permission) -> wal::Result<Principal> {
        match self.access.authorize_current(permission).await {
            Err(wal::Error::Storage(jj_store_object::Error::NotFound)) => Err(wal::Error::Denied),
            result => result,
        }
    }
    pub async fn snapshot(&self) -> wal::Result<ManifestSnapshot> {
        self.principal(Permission::Read).await?;
        let snapshot = self.coordinator.read().await?;
        self.principal(Permission::Read).await?;
        Ok(snapshot)
    }
    pub async fn publish_request(&self, request: &Request) -> wal::Result<PublishOutcome> {
        let result = self.coordinator.publish(request).await?;
        if matches!(result, PublishOutcome::Committed(_)) {
            self.events.published(self.repository);
        }
        Ok(result)
    }

    /// Explicit administrative upgrade for a format-2 repository created
    /// before replication identity existed. The supplied IDs belong to this
    /// durable installation and are never inferred from workspace metadata.
    pub async fn seed_replication(
        &self,
        request_id: m::RequestId,
        publication_id: m::PublicationId,
        replica: m::ReplicaId,
        writer_epoch: m::WriterEpoch,
    ) -> wal::Result<SeedReplicationOutcomeV1> {
        let principal = self.principal(Permission::Admin).await?;
        let outcome = self
            .coordinator
            .seed_replication(&SeedReplicationRequestV1 {
                schema_version: 1,
                principal: principal.id(),
                request_id,
                publication_id,
                replica,
                writer_epoch,
            })
            .await?;
        self.principal(Permission::Admin).await?;
        if matches!(outcome, SeedReplicationOutcomeV1::Seeded(_)) {
            self.events.published(self.repository);
        }
        Ok(outcome)
    }

    pub async fn negotiate_replica(
        &self,
        request: &replication::NegotiateReplicaV1,
    ) -> wal::Result<replication::ReplicaNegotiationV1> {
        self.principal(Permission::Write).await?;
        self.coordinator.ensure_origin_replication().await?;
        if request.schema_version != 1
            || request.repository != self.repository
            || !replication::ReplicaCapabilitiesV1::default().compatible_with(&request.capabilities)
        {
            return Err(wal::Error::Corrupt("replication negotiation"));
        }
        self.principal(Permission::Write).await?;
        let snapshot = self.coordinator.read().await?;
        let root = self
            .coordinator
            .read_replication_root(
                snapshot
                    .manifest()
                    .replication_root
                    .ok_or(wal::Error::Corrupt("replication root missing"))?,
            )
            .await?;
        if request.peer == root.local_stream.replica {
            return Err(wal::Error::Corrupt("replication peer identity"));
        }
        self.principal(Permission::Write).await?;
        Ok(replication::ReplicaNegotiationV1 {
            schema_version: 1,
            repository: self.repository,
            replica: root.local_stream.replica,
            log_epoch: snapshot.manifest().epoch,
            manifest_digest: snapshot.digest(),
            feed_lower_bound: jj_store_model::WalPosition(0),
            feed_upper_bound: snapshot.manifest().published,
            bootstrap: self.coordinator.bootstrap_checkpoint(&snapshot).await?,
            capabilities: replication::ReplicaCapabilitiesV1::default(),
        })
    }

    /// Read the local issuer identity without inventing a peer or opening a
    /// transport. Server composition uses this before contacting an upstream.
    pub async fn replica_descriptor(&self) -> wal::Result<ReplicaDescriptor> {
        self.principal(Permission::Write).await?;
        let snapshot = self.coordinator.read().await?;
        let root = self
            .coordinator
            .read_replication_root(
                snapshot
                    .manifest()
                    .replication_root
                    .ok_or(wal::Error::Corrupt("replication root missing"))?,
            )
            .await?;
        self.principal(Permission::Write).await?;
        Ok(ReplicaDescriptor {
            replica: root.local_stream.replica,
            log_epoch: snapshot.manifest().epoch,
            feed_upper_bound: snapshot.manifest().published,
        })
    }

    pub async fn replication_feed(
        &self,
        request: &replication::FeedRequestV1,
    ) -> wal::Result<replication::FeedResponseV1> {
        if request.schema_version != 1 {
            return Err(wal::Error::Corrupt("replication feed request"));
        }
        self.principal(Permission::Write).await?;
        let snapshot = self.coordinator.read().await?;
        let page = self
            .coordinator
            .committed_origin_feed_at(
                &snapshot,
                request.source_replica,
                request.log_epoch,
                request.after,
                request.upper_bound,
                request.limit,
            )
            .await?;
        self.principal(Permission::Write).await?;
        Ok(replication::FeedResponseV1 {
            schema_version: 1,
            page,
        })
    }

    pub async fn offer_origins(
        &self,
        request: &replication::OfferOriginsRequestV1,
    ) -> wal::Result<replication::OfferOriginsResponseV1> {
        if request.schema_version != 1 {
            return Err(wal::Error::Corrupt("replication origin offers"));
        }
        self.principal(Permission::Write).await?;
        let snapshot = self.coordinator.read().await?;
        let results = self
            .coordinator
            .offer_origins(&snapshot, &request.offers)
            .await?;
        self.principal(Permission::Write).await?;
        Ok(replication::OfferOriginsResponseV1 {
            schema_version: 1,
            results,
        })
    }

    pub async fn origin_closure(
        &self,
        request: &replication::OriginClosureRequestV1,
    ) -> wal::Result<replication::OriginClosureResponseV1> {
        if request.schema_version != 1
            || request.max_entries == 0
            || request.max_entries > self.limits.inventory_nodes
        {
            return Err(wal::Error::Limit("replication origin closure"));
        }
        self.principal(Permission::Write).await?;
        let snapshot = self.coordinator.read().await?;
        let closure = self
            .coordinator
            .origin_closure(&snapshot, request.origin_key, request.max_entries)
            .await?;
        self.principal(Permission::Write).await?;
        Ok(replication::OriginClosureResponseV1 {
            schema_version: 1,
            closure,
        })
    }

    pub async fn apply_origins(
        &self,
        request: &replication::ApplyOriginsTransportRequestV1,
    ) -> wal::Result<replication::ApplyOriginsTransportResponseV1> {
        if request.schema_version != 1 {
            return Err(wal::Error::Corrupt("replication apply request"));
        }
        let principal = self.principal(Permission::Write).await?;
        let guarded = ApplyOriginsRequestV1 {
            schema_version: 1,
            principal: principal.id(),
            request_id: request.request_id,
            publication_id: request.publication_id,
            origins: request.origins.clone(),
        };
        let outcome = self.coordinator.apply_origins(&guarded).await?;
        if matches!(
            &outcome,
            ApplyOriginsOutcomeV1::Applied(result) if result.publication_position.is_some()
        ) {
            self.events.published(self.repository);
        }
        Ok(replication::ApplyOriginsTransportResponseV1 {
            schema_version: 1,
            outcome,
        })
    }

    pub async fn read_replication_record(
        &self,
        reference: wal::records::RecordRef,
    ) -> wal::Result<Vec<u8>> {
        self.principal(Permission::Write).await?;
        let bytes = self.coordinator.read_replication_record(reference).await?;
        self.principal(Permission::Write).await?;
        Ok(bytes)
    }

    pub async fn write_replication_record(
        &self,
        reference: wal::records::RecordRef,
        bytes: &[u8],
    ) -> wal::Result<jj_store_object::WriteOutcome> {
        self.principal(Permission::Write).await?;
        let outcome = self
            .coordinator
            .write_replication_record(reference, bytes)
            .await?;
        self.principal(Permission::Write).await?;
        Ok(outcome)
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
        match self.publish_request(&request).await? {
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
    initialize_repository(store, objects, initial_policy, epoch, limits).await
}

pub async fn create_repository_local_trusted(
    store: Arc<dyn ObjectStore>,
    objects: Arc<NativeObjects>,
    permission: Permission,
    epoch: m::EpochId,
    limits: Limits,
) -> wal::Result<ManifestSnapshot> {
    if permission != Permission::Admin {
        return Err(wal::Error::Denied);
    }
    let policy =
        crate::auth::TokenPolicy::new(crate::auth::Scope::Repository(objects.scope().repository()));
    initialize_repository(store, objects, &policy, epoch, limits).await
}

async fn initialize_repository(
    store: Arc<dyn ObjectStore>,
    objects: Arc<NativeObjects>,
    initial_policy: &crate::auth::TokenPolicy,
    epoch: m::EpochId,
    limits: Limits,
) -> wal::Result<ManifestSnapshot> {
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
    let mut replica = [0; 16];
    let mut writer_epoch = [0; 16];
    getrandom::fill(&mut replica).map_err(|_| wal::Error::Entropy)?;
    getrandom::fill(&mut writer_epoch).map_err(|_| wal::Error::Entropy)?;
    coordinator
        .initialize_replica(
            epoch,
            settings,
            m::ReplicaId::new(replica),
            m::WriterEpoch::new(writer_epoch),
        )
        .await
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
