#[path = "../../jj-store-wal/tests/support/review.rs"]
mod review_support;

use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectRef,
    AgentObjectV1, AgentSessionId, AgentSessionSnapshotV1, CompletenessV1, CoverageV1,
    IntegrationId, SessionState,
};
use jj_review::{ReviewCatalogKey, ReviewCatalogKind, ReviewDeltaV1};
use jj_store_model::*;
use jj_store_object::{
    Condition, Download, Key, ObjectStore, StagedUpload, WriteOutcome, memory::MemoryStore,
};
use jj_store_replication::{
    EndpointFailure, EndpointResult, PeerCheckpointStore, ReplicationEndpoint, ReplicationWorker,
    TransferLimits, WorkerLimits, transfer_closures,
};
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, InventoryError, Representation, SourceScope, VerifiedNode},
    publication::*,
};
use std::{collections::BTreeSet, sync::Arc};

const REPOSITORY: RepositoryId = RepositoryId::new([1; 16]);

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

    async fn authorize_replication_seed(
        &self,
        _: &ManifestSnapshot,
        _: &SeedReplicationRequestV1,
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
        if key != ObjectKey::Operation(OperationId::new([0; 64])) {
            return Err(InventoryError::Source("unexpected native object".into()));
        }
        Ok(VerifiedNode {
            key,
            representation: Representation::Virtual,
            dependencies: BTreeSet::new(),
        })
    }
}

struct Replica {
    coordinator: Coordinator,
    store: Arc<dyn ObjectStore>,
}

#[derive(Clone)]
struct LocalEndpoint {
    replica: ReplicaId,
    coordinator: Coordinator,
    store: Arc<dyn ObjectStore>,
    feed_lower_bound: WalPosition,
}

fn endpoint_error(operation: &'static str) -> impl FnOnce(wal::Error) -> EndpointFailure {
    move |_| EndpointFailure::permanent(operation)
}

#[async_trait::async_trait]
impl ReplicationEndpoint for LocalEndpoint {
    fn repository(&self) -> RepositoryId {
        REPOSITORY
    }

    async fn negotiate(
        &self,
        request: &NegotiateReplicaV1,
    ) -> EndpointResult<ReplicaNegotiationV1> {
        if request.repository != REPOSITORY || request.peer == self.replica {
            return Err(EndpointFailure::permanent("negotiate"));
        }
        let snapshot = self
            .coordinator
            .read()
            .await
            .map_err(endpoint_error("negotiate"))?;
        Ok(ReplicaNegotiationV1 {
            schema_version: 1,
            repository: REPOSITORY,
            replica: self.replica,
            log_epoch: snapshot.manifest().epoch,
            manifest_digest: snapshot.digest(),
            feed_lower_bound: self.feed_lower_bound,
            feed_upper_bound: snapshot.manifest().published,
            bootstrap: self
                .coordinator
                .bootstrap_checkpoint(&snapshot)
                .await
                .map_err(endpoint_error("negotiate"))?,
            capabilities: ReplicaCapabilitiesV1::default(),
        })
    }

    async fn feed(&self, request: &FeedRequestV1) -> EndpointResult<FeedResponseV1> {
        if request.after < self.feed_lower_bound {
            return Err(EndpointFailure::permanent("feed horizon"));
        }
        let snapshot = self
            .coordinator
            .read()
            .await
            .map_err(endpoint_error("feed"))?;
        Ok(FeedResponseV1 {
            schema_version: 1,
            page: self
                .coordinator
                .committed_origin_feed_at(
                    &snapshot,
                    request.source_replica,
                    request.log_epoch,
                    request.after,
                    request.upper_bound,
                    request.limit,
                )
                .await
                .map_err(endpoint_error("feed"))?,
        })
    }

    async fn offer(
        &self,
        request: &OfferOriginsRequestV1,
    ) -> EndpointResult<OfferOriginsResponseV1> {
        let snapshot = self
            .coordinator
            .read()
            .await
            .map_err(endpoint_error("offer"))?;
        Ok(OfferOriginsResponseV1 {
            schema_version: 1,
            results: self
                .coordinator
                .offer_origins(&snapshot, &request.offers)
                .await
                .map_err(endpoint_error("offer"))?,
        })
    }

    async fn closure(
        &self,
        request: &OriginClosureRequestV1,
    ) -> EndpointResult<OriginClosureResponseV1> {
        let snapshot = self
            .coordinator
            .read()
            .await
            .map_err(endpoint_error("closure"))?;
        Ok(OriginClosureResponseV1 {
            schema_version: 1,
            closure: self
                .coordinator
                .origin_closure(&snapshot, request.origin_key, request.max_entries)
                .await
                .map_err(endpoint_error("closure"))?,
        })
    }

    async fn read_native(&self, key: ObjectKey) -> EndpointResult<Download> {
        self.store
            .get(&Key::new(key.storage_path(REPOSITORY)).unwrap(), None)
            .await
            .map_err(|_| EndpointFailure::permanent("read native"))
    }

    async fn write_native(
        &self,
        key: ObjectKey,
        upload: &StagedUpload,
    ) -> EndpointResult<WriteOutcome> {
        self.store
            .put(
                &Key::new(key.storage_path(REPOSITORY)).unwrap(),
                Condition::Absent,
                upload,
            )
            .await
            .map_err(|_| EndpointFailure::permanent("write native"))
    }

    async fn read_control(&self, reference: wal::records::RecordRef) -> EndpointResult<Vec<u8>> {
        self.coordinator
            .read_replication_record(reference)
            .await
            .map_err(endpoint_error("read control"))
    }

    async fn write_control(
        &self,
        reference: wal::records::RecordRef,
        bytes: Vec<u8>,
    ) -> EndpointResult<WriteOutcome> {
        self.coordinator
            .write_replication_record(reference, &bytes)
            .await
            .map_err(endpoint_error("write control"))
    }

    async fn apply(
        &self,
        request: &ApplyOriginsTransportRequestV1,
    ) -> EndpointResult<ApplyOriginsTransportResponseV1> {
        Ok(ApplyOriginsTransportResponseV1 {
            schema_version: 1,
            outcome: self
                .coordinator
                .apply_origins(&ApplyOriginsRequestV1 {
                    schema_version: 1,
                    principal: PrincipalId::new([90; 16]),
                    request_id: request.request_id,
                    publication_id: request.publication_id,
                    origins: request.origins.clone(),
                })
                .await
                .map_err(endpoint_error("apply"))?,
        })
    }
}

impl Replica {
    async fn legacy(seed: u8) -> Self {
        let store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
        let coordinator = coordinator(store.clone());
        coordinator
            .initialize(EpochId::new([seed; 16]), vec![seed])
            .await
            .unwrap();
        Self { coordinator, store }
    }

    async fn writable(seed: u8) -> Self {
        let store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
        let coordinator = coordinator(store.clone());
        coordinator
            .initialize_replica(
                EpochId::new([seed; 16]),
                vec![seed],
                ReplicaId::new([seed.wrapping_add(1); 16]),
                WriterEpoch::new([seed.wrapping_add(2); 16]),
            )
            .await
            .unwrap();
        Self { coordinator, store }
    }
}

fn coordinator(store: Arc<dyn ObjectStore>) -> Coordinator {
    Coordinator::new(
        store,
        REPOSITORY,
        Arc::new(VirtualSource),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap()
}

async fn publish_session(replica: &Replica, seed: u8) -> (AgentCatalogKey, AgentObjectRef) {
    let session_id = AgentSessionId::new([seed; 16]);
    let key = AgentCatalogKey::derive("session", session_id.as_bytes()).unwrap();
    let replacement = replica
        .coordinator
        .stage_agent_object(&AgentObjectV1::SessionSnapshot(AgentSessionSnapshotV1 {
            schema_version: 1,
            session_id,
            integration: IntegrationId::Codex,
            state: SessionState::Ended,
            parent_session_id: None,
            evidence: Vec::new(),
            event_batches: Vec::new(),
            transcripts: Vec::new(),
            tools: Default::default(),
            coverage: CoverageV1 {
                completeness: CompletenessV1::Complete,
                gaps: 0,
                redacted_fields: 0,
                truncated: false,
            },
        }))
        .await
        .unwrap();
    let outcome = replica
        .coordinator
        .publish_agent_delta(
            &AgentDeltaRequest::new(
                PrincipalId::new([40; 16]),
                RequestId::new([seed; 16]),
                PublicationId::new([seed; 16]),
                AgentDeltaV1 {
                    schema_version: 1,
                    edits: vec![AgentCatalogEditV1 {
                        catalog: AgentCatalogKind::Sessions,
                        key,
                        expected: None,
                        replacement: Some(replacement),
                    }],
                },
                Vec::new(),
            )
            .unwrap(),
        )
        .await
        .unwrap();
    assert!(matches!(outcome, PublishOutcome::Committed(_)));
    (key, replacement)
}

async fn publish_legacy_review(replica: &Replica, seed: u8) -> ReviewCatalogKey {
    let snapshot = replica.coordinator.read().await.unwrap();
    let state = snapshot.manifest().domain_state(Domain::Review).unwrap();
    let store = replica.coordinator.review_store(state.root).await.unwrap();
    let thread = review_support::stage_thread(&replica.coordinator, seed, None).await;
    let root = replica
        .coordinator
        .stage_review_delta(
            &store,
            &ReviewDeltaV1 {
                schema_version: 1,
                edits: review_support::edits(&thread),
            },
        )
        .await
        .unwrap();
    let outcome = replica
        .coordinator
        .publish_domain(
            &DomainRequest::new(
                PrincipalId::new([41; 16]),
                RequestId::new([seed; 16]),
                PublicationId::new([seed; 16]),
                Domain::Review,
                state,
                root,
                Vec::new(),
            )
            .unwrap(),
        )
        .await
        .unwrap();
    assert!(matches!(outcome, PublishOutcome::Committed(_)));
    ReviewCatalogKey::derive("thread", thread.thread_id.as_bytes()).unwrap()
}

async fn transfer_origins(source: &Replica, destination: &Replica, origins: &[OriginEnvelopeV1]) {
    let snapshot = source.coordinator.read().await.unwrap();
    let mut closures = Vec::new();
    for origin in origins {
        closures.push(
            source
                .coordinator
                .origin_closure(&snapshot, origin.publication.origin_key, 20_000)
                .await
                .unwrap(),
        );
    }
    transfer_closures(
        source.store.clone(),
        destination.store.clone(),
        REPOSITORY,
        &closures,
        TransferLimits::default(),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn seed_checkpoint_plus_tail_merges_three_roots_without_replacing_local_authority() {
    let source = Replica::legacy(2).await;
    let (baseline_session_key, baseline_session) = publish_session(&source, 10).await;
    let thread_key = publish_legacy_review(&source, 20).await;
    let before_seed = source.coordinator.read().await.unwrap();
    assert_eq!(before_seed.manifest().format, 2);
    assert!(before_seed.manifest().replication_root.is_none());

    let seed_request = SeedReplicationRequestV1 {
        schema_version: 1,
        principal: PrincipalId::new([42; 16]),
        request_id: RequestId::new([30; 16]),
        publication_id: PublicationId::new([30; 16]),
        replica: ReplicaId::new([31; 16]),
        writer_epoch: WriterEpoch::new([32; 16]),
    };
    let SeedReplicationOutcomeV1::Seeded(seed) = source
        .coordinator
        .seed_replication(&seed_request)
        .await
        .unwrap()
    else {
        panic!("seed outcome should be durable")
    };
    assert_eq!(seed.receipt.origin_key.sequence, 1);
    assert_eq!(
        source
            .coordinator
            .seed_replication(&seed_request)
            .await
            .unwrap(),
        SeedReplicationOutcomeV1::Seeded(seed)
    );
    let pinned = source.coordinator.read().await.unwrap();
    let checkpoint = source
        .coordinator
        .bootstrap_checkpoint(&pinned)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(checkpoint.through, seed.receipt.accepted.position);
    assert_eq!(checkpoint.source_manifest, pinned.digest());

    let (tail_session_key, tail_session) = publish_session(&source, 11).await;
    let current = source.coordinator.read().await.unwrap();
    let tail = source
        .coordinator
        .committed_origin_feed_at(
            &current,
            checkpoint.source_replica,
            checkpoint.log_epoch,
            checkpoint.through,
            current.manifest().published,
            16,
        )
        .await
        .unwrap();
    assert_eq!(tail.origins.len(), 1);

    let destination = Replica::writable(50).await;
    let (local_session_key, local_session) = publish_session(&destination, 60).await;
    let local_before = destination.coordinator.read().await.unwrap();
    let local_root_before = destination
        .coordinator
        .read_replication_root(local_before.manifest().replication_root.unwrap())
        .await
        .unwrap();

    let seed_snapshot = source.coordinator.read().await.unwrap();
    let seed_closure = source
        .coordinator
        .origin_closure(&seed_snapshot, checkpoint.seed.origin_key, 20_000)
        .await
        .unwrap();
    assert_eq!(
        seed_closure.envelope.origin_digest,
        checkpoint.seed.origin_digest
    );
    assert!(seed_closure.envelope.publication.is_seed_baseline());
    let mut origins = vec![seed_closure.envelope];
    origins.extend(tail.origins.into_iter().map(|item| item.envelope));
    transfer_origins(&source, &destination, &origins).await;
    let ApplyOriginsOutcomeV1::Applied(applied) = destination
        .coordinator
        .apply_origins(&ApplyOriginsRequestV1 {
            schema_version: 1,
            principal: PrincipalId::new([70; 16]),
            request_id: RequestId::new([71; 16]),
            publication_id: PublicationId::new([72; 16]),
            origins,
        })
        .await
        .unwrap()
    else {
        panic!("bootstrap import should commit")
    };
    assert_eq!(applied.receipts.len(), 2);

    let restored = destination.coordinator.recover(512).await.unwrap();
    assert_ne!(restored.digest(), current.digest());
    let local_root_after = destination
        .coordinator
        .read_replication_root(restored.manifest().replication_root.unwrap())
        .await
        .unwrap();
    assert_eq!(
        local_root_after.local_stream,
        local_root_before.local_stream
    );
    assert_eq!(
        local_root_after.local_sequence,
        local_root_before.local_sequence
    );

    let agent = destination
        .coordinator
        .agent_store(restored.manifest().domains().unwrap().agent.root)
        .await
        .unwrap();
    for (key, expected) in [
        (baseline_session_key, baseline_session),
        (tail_session_key, tail_session),
        (local_session_key, local_session),
    ] {
        assert_eq!(
            agent.lookup(AgentCatalogKind::Sessions, key).await.unwrap(),
            Some(expected)
        );
    }
    let review = destination
        .coordinator
        .review_store(restored.manifest().domains().unwrap().review.root)
        .await
        .unwrap();
    assert!(
        review
            .lookup(ReviewCatalogKind::Threads, thread_key)
            .await
            .unwrap()
            .is_some()
    );
}

#[tokio::test]
async fn worker_recovers_a_retained_feed_gap_from_seed_then_replays_the_pinned_tail() {
    let source = Replica::legacy(80).await;
    let seed_request = SeedReplicationRequestV1 {
        schema_version: 1,
        principal: PrincipalId::new([81; 16]),
        request_id: RequestId::new([82; 16]),
        publication_id: PublicationId::new([82; 16]),
        replica: ReplicaId::new([83; 16]),
        writer_epoch: WriterEpoch::new([84; 16]),
    };
    let SeedReplicationOutcomeV1::Seeded(seed) = source
        .coordinator
        .seed_replication(&seed_request)
        .await
        .unwrap()
    else {
        panic!("seed must commit")
    };
    let (tail_key, tail_session) = publish_session(&source, 85).await;

    let destination = Replica::writable(90).await;
    let (pending_key, pending_session) = publish_session(&destination, 94).await;
    let destination_before = destination.coordinator.read().await.unwrap();
    let destination_root = destination
        .coordinator
        .read_replication_root(destination_before.manifest().replication_root.unwrap())
        .await
        .unwrap();

    let source_endpoint = LocalEndpoint {
        replica: seed_request.replica,
        coordinator: source.coordinator.clone(),
        store: source.store.clone(),
        feed_lower_bound: seed.receipt.accepted.position,
    };
    let destination_endpoint = LocalEndpoint {
        replica: destination_root.local_stream.replica,
        coordinator: destination.coordinator.clone(),
        store: destination.store.clone(),
        feed_lower_bound: WalPosition(0),
    };
    let checkpoint_store = PeerCheckpointStore::new(
        destination.store.clone(),
        REPOSITORY,
        destination_root.local_stream.replica,
        seed_request.replica,
    )
    .unwrap();
    let mut worker = ReplicationWorker::new(
        REPOSITORY,
        destination_root.local_stream.replica,
        seed_request.replica,
        destination_endpoint,
        source_endpoint,
        checkpoint_store.clone(),
        WorkerLimits::default(),
    )
    .unwrap();
    let outcome = worker.step().await.unwrap();
    assert!(outcome.caught_up);
    assert_eq!(outcome.offered_origins, 2);
    assert_eq!(outcome.transferred_origins, 2);
    assert_eq!(
        outcome.through,
        source
            .coordinator
            .read()
            .await
            .unwrap()
            .manifest()
            .published
    );
    let checkpoint = checkpoint_store.load().await.unwrap().unwrap();
    assert_eq!(checkpoint.checkpoint.revision, 2);
    assert_eq!(
        checkpoint.checkpoint.pulled.unwrap().through,
        outcome.through
    );

    let restored = destination.coordinator.recover(512).await.unwrap();
    let root = destination
        .coordinator
        .read_replication_root(restored.manifest().replication_root.unwrap())
        .await
        .unwrap();
    assert_eq!(root.local_stream, destination_root.local_stream);
    assert_eq!(root.local_sequence, destination_root.local_sequence);
    let agent = destination
        .coordinator
        .agent_store(restored.manifest().domains().unwrap().agent.root)
        .await
        .unwrap();
    for (key, expected) in [(pending_key, pending_session), (tail_key, tail_session)] {
        assert_eq!(
            agent.lookup(AgentCatalogKind::Sessions, key).await.unwrap(),
            Some(expected)
        );
    }
}
