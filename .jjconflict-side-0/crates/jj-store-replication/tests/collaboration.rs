use jj_store_model::*;
use jj_store_object::{ObjectStore, memory::MemoryStore};
use jj_store_replication::{TransferLimits, transfer_closures};
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, InventoryError, SourceScope, VerifiedNode},
    publication::*,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

const REPOSITORY: RepositoryId = RepositoryId::new([21; 16]);

struct Permit;

#[async_trait::async_trait]
impl PublicationGuard for Permit {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }

    async fn authorize_domain(&self, _: &ManifestSnapshot, _: &DomainRequest) -> wal::Result<()> {
        Ok(())
    }

    async fn authorize_replication(
        &self,
        _: &ManifestSnapshot,
        _: &ApplyOriginsRequestV1,
    ) -> wal::Result<()> {
        Ok(())
    }

    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
struct VirtualSource;

#[async_trait::async_trait]
impl DependencySource for VirtualSource {
    fn scope(&self) -> SourceScope {
        SourceScope::new(REPOSITORY)
    }

    async fn load_verified(&self, key: ObjectKey, _: u64) -> wal::inventory::Result<VerifiedNode> {
        if key == ObjectKey::Operation(OperationId::new([0; 64])) {
            Ok(VerifiedNode {
                key,
                representation: wal::inventory::Representation::Virtual,
                dependencies: Default::default(),
            })
        } else {
            Err(InventoryError::Source("unexpected native object".into()))
        }
    }
}

fn coordinator(store: Arc<dyn ObjectStore>) -> Coordinator {
    coordinator_with_guard(store, Arc::new(Permit))
}

fn coordinator_with_guard(
    store: Arc<dyn ObjectStore>,
    guard: Arc<dyn PublicationGuard>,
) -> Coordinator {
    Coordinator::new(
        store,
        REPOSITORY,
        Arc::new(VirtualSource),
        guard,
        Limits::default(),
    )
    .unwrap()
}

#[derive(Default)]
struct ImportOnlyGuard {
    replication_checks: AtomicUsize,
}

#[async_trait::async_trait]
impl PublicationGuard for ImportOnlyGuard {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        panic!("origin import invoked ordinary publication authorization")
    }

    async fn authorize_domain(&self, _: &ManifestSnapshot, _: &DomainRequest) -> wal::Result<()> {
        panic!("origin import invoked authored domain authorization")
    }

    async fn authorize_collaboration(
        &self,
        _: &ManifestSnapshot,
        _: &CollaborationRequest,
    ) -> wal::Result<()> {
        panic!("origin import invoked collaboration command authorization")
    }

    async fn authorize_replication(
        &self,
        _: &ManifestSnapshot,
        _: &ApplyOriginsRequestV1,
    ) -> wal::Result<()> {
        self.replication_checks.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        panic!("origin import invoked native command semantics")
    }
}

async fn publish_alias(coordinator: &Coordinator, value: u8) -> (AgentCatalogKey, AgentObjectRef) {
    publish_alias_version(coordinator, value, value, value).await
}

async fn publish_alias_version(
    coordinator: &Coordinator,
    identity: u8,
    target: u8,
    request_value: u8,
) -> (AgentCatalogKey, AgentObjectRef) {
    let alias = AgentAliasV1 {
        schema_version: 1,
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([identity; 16]),
        native_kind: NativeEntityKind::Session,
        native_id: format!("native-session-{identity}"),
        target: AgentEntityRefV1::Session {
            session_id: AgentSessionId::new([target; 16]),
        },
    };
    let key = alias.catalog_key().unwrap();
    let replacement = coordinator
        .stage_agent_object(&AgentObjectV1::Alias(alias))
        .await
        .unwrap();
    let request = AgentDeltaRequest::new(
        PrincipalId::new([1; 16]),
        RequestId::new([request_value; 16]),
        PublicationId::new([request_value; 16]),
        AgentDeltaV1 {
            schema_version: 1,
            edits: vec![AgentCatalogEditV1 {
                catalog: AgentCatalogKind::Aliases,
                key,
                expected: None,
                replacement: Some(replacement),
            }],
        },
        vec![request_value],
    )
    .unwrap();
    assert!(matches!(
        coordinator.publish_agent_delta(&request).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    (key, replacement)
}

fn create_thread(value: u8) -> ReviewCommandRequest {
    ReviewCommandRequest::new(
        PrincipalId::new([2; 16]),
        RequestId::new([value; 16]),
        PublicationId::new([value; 16]),
        ReviewCommandV1::CreateThread {
            anchor: Box::new(ArtifactAnchorV1 {
                schema_version: 1,
                repository_id: REPOSITORY,
                target: ArtifactTargetV1::Repository,
                created_context: CreatedContextV1 {
                    publication_position: None,
                    code_state_version: None,
                    agent_root: None,
                },
                sensitivity: SensitivityV1::Repository,
            }),
            body: ReviewCommentBodyV1 {
                schema_version: 1,
                format: ReviewBodyFormatV1::SafeMarkdown,
                text: format!("offline review {value}"),
                sanitizer_version: "test-v1".into(),
            },
        },
    )
    .unwrap()
}

#[tokio::test]
async fn imported_agent_and_review_facts_reduce_without_replaying_commands() {
    let source_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    let source = coordinator(source_store.clone());
    source
        .initialize_replica(
            EpochId::new([1; 16]),
            b"source".to_vec(),
            ReplicaId::new([2; 16]),
            WriterEpoch::new([3; 16]),
        )
        .await
        .unwrap();
    let first_alias = publish_alias(&source, 10).await;
    let second_alias = publish_alias(&source, 11).await;
    assert!(matches!(
        source
            .publish_review_command(&create_thread(12))
            .await
            .unwrap(),
        PublishOutcome::Committed(_)
    ));

    let source_snapshot = source.read().await.unwrap();
    let feed = source
        .committed_origin_feed(&source_snapshot, WalPosition(0), 16)
        .await
        .unwrap();
    assert_eq!(feed.origins.len(), 3);
    let mut closures = Vec::new();
    for item in &feed.origins {
        closures.push(
            source
                .origin_closure(
                    &source_snapshot,
                    item.envelope.publication.origin_key,
                    10_000,
                )
                .await
                .unwrap(),
        );
    }

    let invalid_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    let invalid = coordinator(invalid_store.clone());
    invalid
        .initialize_replica(
            EpochId::new([14; 16]),
            b"invalid-destination".to_vec(),
            ReplicaId::new([15; 16]),
            WriterEpoch::new([16; 16]),
        )
        .await
        .unwrap();
    transfer_closures(
        source_store.clone(),
        invalid_store,
        REPOSITORY,
        &closures,
        TransferLimits::default(),
    )
    .await
    .unwrap();
    let mut changed = feed.origins[1].envelope.clone();
    changed
        .publication
        .domain_facts
        .as_mut()
        .unwrap()
        .agent
        .as_mut()
        .unwrap()
        .edits[0]
        .replacement = Some(first_alias.1);
    changed.origin_digest = changed.publication.digest().unwrap();
    assert!(matches!(
        invalid
            .apply_origins(&ApplyOriginsRequestV1 {
                schema_version: 1,
                principal: PrincipalId::new([17; 16]),
                request_id: RequestId::new([18; 16]),
                publication_id: PublicationId::new([18; 16]),
                origins: vec![feed.origins[0].envelope.clone(), changed],
            })
            .await,
        Err(wal::Error::Corrupt("origin agent facts selection"))
    ));
    assert_eq!(
        invalid.read().await.unwrap().manifest().published,
        WalPosition(0)
    );

    let destination_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    let import_guard = Arc::new(ImportOnlyGuard::default());
    let destination = coordinator_with_guard(destination_store.clone(), import_guard.clone());
    destination
        .initialize_replica(
            EpochId::new([4; 16]),
            b"destination".to_vec(),
            ReplicaId::new([5; 16]),
            WriterEpoch::new([6; 16]),
        )
        .await
        .unwrap();
    transfer_closures(
        source_store,
        destination_store,
        REPOSITORY,
        &closures,
        TransferLimits::default(),
    )
    .await
    .unwrap();
    let request = ApplyOriginsRequestV1 {
        schema_version: 1,
        principal: PrincipalId::new([7; 16]),
        request_id: RequestId::new([8; 16]),
        publication_id: PublicationId::new([8; 16]),
        origins: feed
            .origins
            .iter()
            .map(|item| item.envelope.clone())
            .collect(),
    };
    assert!(matches!(
        destination.apply_origins(&request).await.unwrap(),
        ApplyOriginsOutcomeV1::Applied(_)
    ));

    let selected = destination.read().await.unwrap();
    let domains = selected.manifest().domains().unwrap();
    assert_eq!(domains.agent.version, 1);
    assert_eq!(domains.review.version, 1);
    let agent = destination.agent_store(domains.agent.root).await.unwrap();
    assert_eq!(
        agent
            .lookup(AgentCatalogKind::Aliases, first_alias.0)
            .await
            .unwrap(),
        Some(first_alias.1)
    );
    assert_eq!(
        agent
            .lookup(AgentCatalogKind::Aliases, second_alias.0)
            .await
            .unwrap(),
        Some(second_alias.1)
    );
    let review = destination.review_store(domains.review.root).await.unwrap();
    assert_eq!(
        review
            .list_threads(None, 16)
            .await
            .unwrap()
            .selections
            .len(),
        1
    );
    assert_eq!(selected.manifest().published, WalPosition(1));
    assert!(import_guard.replication_checks.load(Ordering::SeqCst) >= 2);
}

#[tokio::test]
async fn concurrent_replicas_merge_independent_agent_and_review_facts() {
    let first_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    let first = coordinator(first_store.clone());
    first
        .initialize_replica(
            EpochId::new([31; 16]),
            b"first".to_vec(),
            ReplicaId::new([32; 16]),
            WriterEpoch::new([33; 16]),
        )
        .await
        .unwrap();
    let first_alias = publish_alias(&first, 40).await;
    first
        .publish_review_command(&create_thread(41))
        .await
        .unwrap();

    let second_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    let second = coordinator(second_store.clone());
    second
        .initialize_replica(
            EpochId::new([34; 16]),
            b"second".to_vec(),
            ReplicaId::new([35; 16]),
            WriterEpoch::new([36; 16]),
        )
        .await
        .unwrap();
    let second_alias = publish_alias(&second, 42).await;
    second
        .publish_review_command(&create_thread(43))
        .await
        .unwrap();

    let first_snapshot = first.read().await.unwrap();
    let first_feed = first
        .committed_origin_feed(&first_snapshot, WalPosition(0), 16)
        .await
        .unwrap();
    let second_snapshot = second.read().await.unwrap();
    let second_feed = second
        .committed_origin_feed(&second_snapshot, WalPosition(0), 16)
        .await
        .unwrap();

    let destination_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    let destination = coordinator(destination_store.clone());
    destination
        .initialize_replica(
            EpochId::new([37; 16]),
            b"destination".to_vec(),
            ReplicaId::new([38; 16]),
            WriterEpoch::new([39; 16]),
        )
        .await
        .unwrap();

    for (source, store, snapshot, feed) in [
        (&first, &first_store, &first_snapshot, &first_feed),
        (&second, &second_store, &second_snapshot, &second_feed),
    ] {
        let mut closures = Vec::new();
        for item in &feed.origins {
            closures.push(
                source
                    .origin_closure(snapshot, item.envelope.publication.origin_key, 10_000)
                    .await
                    .unwrap(),
            );
        }
        transfer_closures(
            store.clone(),
            destination_store.clone(),
            REPOSITORY,
            &closures,
            TransferLimits::default(),
        )
        .await
        .unwrap();
    }

    let origins = second_feed
        .origins
        .iter()
        .rev()
        .chain(first_feed.origins.iter().rev())
        .map(|item| item.envelope.clone())
        .collect();
    destination
        .apply_origins(&ApplyOriginsRequestV1 {
            schema_version: 1,
            principal: PrincipalId::new([44; 16]),
            request_id: RequestId::new([45; 16]),
            publication_id: PublicationId::new([45; 16]),
            origins,
        })
        .await
        .unwrap();

    let selected = destination.read().await.unwrap();
    let domains = selected.manifest().domains().unwrap();
    let agent = destination.agent_store(domains.agent.root).await.unwrap();
    for (key, reference) in [first_alias, second_alias] {
        assert_eq!(
            agent.lookup(AgentCatalogKind::Aliases, key).await.unwrap(),
            Some(reference)
        );
    }
    let review = destination.review_store(domains.review.root).await.unwrap();
    assert_eq!(
        review
            .list_threads(None, 16)
            .await
            .unwrap()
            .selections
            .len(),
        2
    );
    assert_eq!(
        review
            .list_catalog(ReviewCatalogKind::Statuses, None, 16)
            .await
            .unwrap()
            .selections
            .len(),
        1
    );
    assert_eq!(selected.manifest().published, WalPosition(1));
}

#[tokio::test]
async fn concurrent_agent_versions_are_retained_as_an_explicit_conflict() {
    let first_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    let first = coordinator(first_store.clone());
    first
        .initialize_replica(
            EpochId::new([51; 16]),
            b"first".to_vec(),
            ReplicaId::new([52; 16]),
            WriterEpoch::new([53; 16]),
        )
        .await
        .unwrap();
    let first_version = publish_alias_version(&first, 60, 61, 62).await;

    let second_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    let second = coordinator(second_store.clone());
    second
        .initialize_replica(
            EpochId::new([54; 16]),
            b"second".to_vec(),
            ReplicaId::new([55; 16]),
            WriterEpoch::new([56; 16]),
        )
        .await
        .unwrap();
    let second_version = publish_alias_version(&second, 60, 63, 64).await;
    assert_eq!(first_version.0, second_version.0);
    assert_ne!(first_version.1, second_version.1);

    let first_snapshot = first.read().await.unwrap();
    let first_feed = first
        .committed_origin_feed(&first_snapshot, WalPosition(0), 8)
        .await
        .unwrap();
    let second_snapshot = second.read().await.unwrap();
    let second_feed = second
        .committed_origin_feed(&second_snapshot, WalPosition(0), 8)
        .await
        .unwrap();
    let destination_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    let destination = coordinator(destination_store.clone());
    destination
        .initialize_replica(
            EpochId::new([57; 16]),
            b"destination".to_vec(),
            ReplicaId::new([58; 16]),
            WriterEpoch::new([59; 16]),
        )
        .await
        .unwrap();
    for (source, store, snapshot, feed) in [
        (&first, &first_store, &first_snapshot, &first_feed),
        (&second, &second_store, &second_snapshot, &second_feed),
    ] {
        let closure = source
            .origin_closure(
                snapshot,
                feed.origins[0].envelope.publication.origin_key,
                10_000,
            )
            .await
            .unwrap();
        transfer_closures(
            store.clone(),
            destination_store.clone(),
            REPOSITORY,
            &[closure],
            TransferLimits::default(),
        )
        .await
        .unwrap();
    }
    destination
        .apply_origins(&ApplyOriginsRequestV1 {
            schema_version: 1,
            principal: PrincipalId::new([65; 16]),
            request_id: RequestId::new([66; 16]),
            publication_id: PublicationId::new([66; 16]),
            origins: vec![
                second_feed.origins[0].envelope.clone(),
                first_feed.origins[0].envelope.clone(),
            ],
        })
        .await
        .unwrap();

    let selected = destination.read().await.unwrap();
    let domains = selected.manifest().domains().unwrap();
    let agent = destination.agent_store(domains.agent.root).await.unwrap();
    let conflict_ref = agent
        .lookup(AgentCatalogKind::Aliases, first_version.0)
        .await
        .unwrap()
        .unwrap();
    let AgentObjectV1::CatalogConflict(conflict) = agent.read_object(conflict_ref).await.unwrap()
    else {
        panic!("concurrent alias should select a conflict object");
    };
    assert_eq!(
        conflict.alternatives,
        [first_version.1, second_version.1]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
    );
}
