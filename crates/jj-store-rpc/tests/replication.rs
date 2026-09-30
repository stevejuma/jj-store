use jj_agent::{
    CaptureBatchId, IntegrationId, NormalizerProfileV1, ObservationId, SanitizedMediaType,
    SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
    SanitizedRepresentation, SourceInstanceId, SourceKind, SourceTrust,
};
use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::{Download, ObjectStore, StagedUpload, WriteOutcome, memory::MemoryStore};
use jj_store_replication::{
    ApplyOriginsTransportRequestV1, EndpointFailure, EndpointResult, FeedCursorV1, FeedRequestV1,
    NegotiateReplicaV1, OfferOriginsRequestV1, OriginClosureRequestV1, PeerCheckpointStore,
    ReplicaCapabilitiesV1, ReplicationEndpoint, ReplicationWorker, WorkerLimits,
};
use jj_store_rpc::{
    client::{NativeClient, Options},
    server::{Limits as RpcLimits, NativeRpc},
};
use jj_store_service::{
    auth::{BearerToken, Permission, Scope, SystemClock, TokenPolicy},
    events::{EventKind, Limits as EventLimits, Subscription},
    native::{NativeSession, SessionLimits},
    transport::NativeEndpoint,
};
use jj_store_wal::{
    self as wal,
    inventory::Representation,
    publication::{
        ApplyOriginsOutcomeV1, Coordinator, Limits, ManifestSnapshot, PublicationGuard, Request,
        Snapshot,
    },
};
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

const REPOSITORY: m::RepositoryId = m::RepositoryId::new([41; 16]);

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

struct RunningReplica {
    client: NativeClient,
    session: Arc<NativeSession>,
    stop: Option<tokio::sync::oneshot::Sender<()>>,
    task: tokio::task::JoinHandle<()>,
}

impl RunningReplica {
    async fn stop(mut self) {
        let _ = self.stop.take().unwrap().send(());
        tokio::time::timeout(Duration::from_secs(5), self.task)
            .await
            .unwrap()
            .unwrap();
    }
}

async fn initialize(
    store: Arc<dyn ObjectStore>,
    epoch: m::EpochId,
    replica: m::ReplicaId,
    writer_epoch: m::WriterEpoch,
    principal: m::PrincipalId,
) -> (BearerToken, Arc<NativeObjects>) {
    let mut policy = TokenPolicy::new(Scope::Repository(REPOSITORY));
    let token = policy.issue(principal, Permission::Write, None).unwrap();
    let objects = Arc::new(NativeObjects::new(store.clone(), REPOSITORY, 4 * 1024 * 1024).unwrap());
    Coordinator::new(
        store,
        REPOSITORY,
        objects.clone(),
        Arc::new(InitializationOnly),
        Limits::default(),
    )
    .unwrap()
    .initialize_replica(epoch, policy.encode().unwrap(), replica, writer_epoch)
    .await
    .unwrap();
    (token, objects)
}

async fn start(
    store: Arc<dyn ObjectStore>,
    objects: Arc<NativeObjects>,
    token: BearerToken,
) -> RunningReplica {
    let endpoint = NativeEndpoint::new(
        REPOSITORY,
        store,
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap();
    let session = endpoint.bind(token.clone()).unwrap().into_session();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(async move {
        NativeRpc::new(vec![endpoint], RpcLimits::default())
            .unwrap()
            .serve(listener, None, true, async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let client = NativeClient::connect(
        &format!("http://{address}"),
        REPOSITORY,
        token,
        Options {
            insecure_loopback: true,
            timeout: Duration::from_secs(5),
            object_bytes: 4 * 1024 * 1024,
            ..Options::default()
        },
    )
    .await
    .unwrap();
    // Retain the native objects in the caller so restart uses the exact source.
    let _ = objects;
    RunningReplica {
        client,
        session,
        stop: Some(stop),
        task,
    }
}

#[derive(Clone, Default)]
struct ProjectionState(Arc<Mutex<Option<jj_store_adapters::Checkpoint>>>);

struct RecordingProjection(ProjectionState);

#[async_trait::async_trait]
impl jj_store_adapters::Projection for RecordingProjection {
    async fn recover(
        &mut self,
        context: jj_store_adapters::Context,
    ) -> jj_store_adapters::Result<Option<jj_store_adapters::Checkpoint>> {
        context.check()?;
        Ok(self.0.0.lock().unwrap().clone())
    }

    async fn bootstrap(
        &mut self,
        snapshot: jj_store_adapters::Snapshot,
        identity: &jj_store_adapters::Identity,
        context: jj_store_adapters::Context,
    ) -> jj_store_adapters::Result<jj_store_adapters::Checkpoint> {
        context.check()?;
        let checkpoint = jj_store_adapters::Checkpoint {
            version: 1,
            identity: identity.clone(),
            source: snapshot.position,
            artifact: m::RecordDigest::new([91; 32]),
        };
        *self.0.0.lock().unwrap() = Some(checkpoint.clone());
        Ok(checkpoint)
    }

    async fn apply(
        &mut self,
        batch: jj_store_adapters::Batch,
        identity: &jj_store_adapters::Identity,
        context: jj_store_adapters::Context,
    ) -> jj_store_adapters::Result<jj_store_adapters::Checkpoint> {
        context.check()?;
        let checkpoint = jj_store_adapters::Checkpoint {
            version: 1,
            identity: identity.clone(),
            source: batch.snapshot.position,
            artifact: m::RecordDigest::new([92; 32]),
        };
        *self.0.0.lock().unwrap() = Some(checkpoint.clone());
        Ok(checkpoint)
    }
}

fn batch() -> SanitizedObservationBatchV1 {
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([51; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([52; 16]),
        sanitizer_version: "mongodb-kingfisher@2.7.0/builtin-medium/v1".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![SanitizedObservationV1 {
            observation_id: ObservationId::new([53; 16]),
            source_sequence: 1,
            occurred_at_millis: Some(1_725_000_000_000),
            source_kind: SourceKind::Hook,
            source_trust: SourceTrust::LocalUserHook,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                br#"{"hook_event_name":"SessionStart","session_id":"replicated"}"#,
            )
            .unwrap(),
        }],
    }
}

#[derive(Clone)]
struct RpcEndpoint {
    client: NativeClient,
    fail_apply_once: Arc<AtomicBool>,
    apply_ids: Arc<Mutex<Vec<(m::RequestId, m::PublicationId)>>>,
}

impl RpcEndpoint {
    fn new(client: NativeClient) -> Self {
        Self {
            client,
            fail_apply_once: Arc::new(AtomicBool::new(false)),
            apply_ids: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn fail_next_apply_before_send(&self) {
        self.fail_apply_once.store(true, Ordering::SeqCst);
    }

    fn apply_ids(&self) -> Vec<(m::RequestId, m::PublicationId)> {
        self.apply_ids.lock().unwrap().clone()
    }
}

fn endpoint_error(operation: &'static str, status: tonic::Status) -> EndpointFailure {
    use tonic::Code;
    if matches!(
        status.code(),
        Code::Cancelled
            | Code::Unknown
            | Code::DeadlineExceeded
            | Code::ResourceExhausted
            | Code::Aborted
            | Code::Internal
            | Code::Unavailable
    ) {
        EndpointFailure::transient(operation)
    } else {
        EndpointFailure::permanent(operation)
    }
}

#[async_trait::async_trait]
impl ReplicationEndpoint for RpcEndpoint {
    fn repository(&self) -> m::RepositoryId {
        self.client.repository()
    }

    async fn negotiate(
        &self,
        request: &jj_store_replication::NegotiateReplicaV1,
    ) -> EndpointResult<jj_store_replication::ReplicaNegotiationV1> {
        self.client
            .negotiate_replica(request)
            .await
            .map_err(|status| endpoint_error("negotiate", status))
    }

    async fn feed(
        &self,
        request: &jj_store_replication::FeedRequestV1,
    ) -> EndpointResult<jj_store_replication::FeedResponseV1> {
        self.client
            .committed_feed(request)
            .await
            .map_err(|status| endpoint_error("feed", status))
    }

    async fn offer(
        &self,
        request: &jj_store_replication::OfferOriginsRequestV1,
    ) -> EndpointResult<jj_store_replication::OfferOriginsResponseV1> {
        self.client
            .offer_origins(request)
            .await
            .map_err(|status| endpoint_error("offer", status))
    }

    async fn closure(
        &self,
        request: &jj_store_replication::OriginClosureRequestV1,
    ) -> EndpointResult<jj_store_replication::OriginClosureResponseV1> {
        self.client
            .origin_closure(request)
            .await
            .map_err(|status| endpoint_error("closure", status))
    }

    async fn read_native(&self, key: m::ObjectKey) -> EndpointResult<Download> {
        self.client
            .read(key)
            .await
            .map_err(|status| endpoint_error("read native", status))
    }

    async fn write_native(
        &self,
        key: m::ObjectKey,
        upload: &StagedUpload,
    ) -> EndpointResult<WriteOutcome> {
        self.client
            .upload(key, upload)
            .await
            .map_err(|status| endpoint_error("write native", status))
    }

    async fn read_control(
        &self,
        reference: jj_store_wal::records::RecordRef,
    ) -> EndpointResult<Vec<u8>> {
        self.client
            .read_control_record(reference)
            .await
            .map_err(|status| endpoint_error("read control", status))
    }

    async fn write_control(
        &self,
        reference: jj_store_wal::records::RecordRef,
        bytes: Vec<u8>,
    ) -> EndpointResult<WriteOutcome> {
        self.client
            .write_control_record(reference, bytes)
            .await
            .map_err(|status| endpoint_error("write control", status))
    }

    async fn apply(
        &self,
        request: &jj_store_replication::ApplyOriginsTransportRequestV1,
    ) -> EndpointResult<jj_store_replication::ApplyOriginsTransportResponseV1> {
        self.apply_ids
            .lock()
            .unwrap()
            .push((request.request_id, request.publication_id));
        if self.fail_apply_once.swap(false, Ordering::SeqCst) {
            return Err(EndpointFailure::transient("apply before send"));
        }
        self.client
            .apply_origins(request)
            .await
            .map_err(|status| endpoint_error("apply", status))
    }
}

#[tokio::test]
async fn authenticated_rpc_applies_a_closure_and_checkpoints_only_after_ack() {
    let source_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    let destination_memory = Arc::new(MemoryStore::default());
    let destination_store: Arc<dyn ObjectStore> = destination_memory.clone();
    let source_replica = m::ReplicaId::new([42; 16]);
    let destination_replica = m::ReplicaId::new([43; 16]);
    let principal = m::PrincipalId::new([44; 16]);
    let (source_token, source_objects) = initialize(
        source_store.clone(),
        m::EpochId::new([45; 16]),
        source_replica,
        m::WriterEpoch::new([46; 16]),
        principal,
    )
    .await;
    let (destination_token, destination_objects) = initialize(
        destination_store.clone(),
        m::EpochId::new([47; 16]),
        destination_replica,
        m::WriterEpoch::new([48; 16]),
        principal,
    )
    .await;
    let source = start(source_store, source_objects, source_token).await;
    let destination = start(
        destination_store.clone(),
        destination_objects.clone(),
        destination_token.clone(),
    )
    .await;
    let mut events = Subscription::open(
        destination.session.clone(),
        None,
        EventLimits {
            poll_interval: Duration::from_secs(5),
            ..EventLimits::default()
        },
    )
    .await
    .unwrap();
    assert!(matches!(
        events.next().await.unwrap().unwrap().kind,
        EventKind::Ready { .. }
    ));
    let projection_state = ProjectionState::default();
    let projection_root = tempfile::tempdir().unwrap();
    let mut projection = jj_store_adapters::Runner::new(
        destination.session.adapter_source(),
        jj_store_adapters::Identity {
            name: "replication-import-test".into(),
            implementation: "test".into(),
            schema: 1,
            config: m::RecordDigest::new([90; 32]),
        },
        RecordingProjection(projection_state.clone()),
        projection_root.path(),
        jj_store_adapters::Limits::default(),
    )
    .unwrap();
    assert_eq!(
        projection
            .step(jj_store_adapters::CancellationToken::new())
            .await
            .unwrap()
            .applied
            .unwrap()
            .source
            .position,
        m::WalPosition(0)
    );

    let prepared = source
        .client
        .prepare_agent_capture(
            m::RequestId::new([54; 16]),
            m::PublicationId::new([55; 16]),
            &NormalizerProfileV1::codex_0_156_1(),
            &batch(),
            b"captured locally".to_vec(),
        )
        .await
        .unwrap();
    source.client.publish_agent_delta(&prepared).await.unwrap();

    let negotiation = source
        .client
        .negotiate_replica(&NegotiateReplicaV1 {
            schema_version: 1,
            repository: REPOSITORY,
            peer: destination_replica,
            capabilities: ReplicaCapabilitiesV1::default(),
        })
        .await
        .unwrap();
    assert_eq!(negotiation.replica, source_replica);
    let feed = source
        .client
        .committed_feed(&FeedRequestV1 {
            schema_version: 1,
            source_replica,
            log_epoch: negotiation.log_epoch,
            after: m::WalPosition(0),
            upper_bound: negotiation.feed_upper_bound,
            limit: 16,
        })
        .await
        .unwrap()
        .page;
    assert_eq!(feed.origins.len(), 1);
    let offers = feed
        .origins
        .iter()
        .map(|item| wal::publication::OriginOfferV1 {
            origin_key: item.envelope.publication.origin_key,
            origin_digest: item.envelope.origin_digest,
        })
        .collect::<Vec<_>>();
    assert!(
        destination
            .client
            .offer_origins(&OfferOriginsRequestV1 {
                schema_version: 1,
                offers: offers.clone(),
            })
            .await
            .unwrap()
            .results
            .iter()
            .all(|result| matches!(
                result,
                wal::publication::OriginOfferResultV1::Missing { .. }
            ))
    );

    let closure = source
        .client
        .origin_closure(&OriginClosureRequestV1 {
            schema_version: 1,
            origin_key: offers[0].origin_key,
            max_entries: 10_000,
        })
        .await
        .unwrap()
        .closure;
    for node in &closure.native {
        if matches!(node.representation, Representation::Stored { .. }) {
            let download = source.client.read(node.key).await.unwrap();
            let upload =
                jj_store_object::StagedUpload::from_reader(download.reader, download.metadata.len)
                    .await
                    .unwrap();
            assert!(matches!(
                destination.client.upload(node.key, &upload).await.unwrap(),
                WriteOutcome::Applied(_) | WriteOutcome::PreconditionFailed
            ));
        }
    }
    for reference in &closure.control {
        let bytes = source.client.read_control_record(*reference).await.unwrap();
        assert!(matches!(
            destination
                .client
                .write_control_record(*reference, bytes)
                .await
                .unwrap(),
            WriteOutcome::Applied(_) | WriteOutcome::PreconditionFailed
        ));
    }

    let checkpoints = PeerCheckpointStore::new(
        destination_store,
        REPOSITORY,
        destination_replica,
        source_replica,
    )
    .unwrap();
    assert!(checkpoints.load().await.unwrap().is_none());
    let apply = ApplyOriginsTransportRequestV1 {
        schema_version: 1,
        request_id: m::RequestId::new([56; 16]),
        publication_id: m::PublicationId::new([57; 16]),
        origins: vec![closure.envelope],
    };
    let first = destination.client.apply_origins(&apply).await.unwrap();
    let ApplyOriginsOutcomeV1::Applied(first_result) = &first.outcome else {
        panic!("authenticated import must return a durable acknowledgement");
    };
    assert!(first_result.publication_position.is_some());
    assert_eq!(first_result.receipts[0].origin_key, offers[0].origin_key);
    let changed = tokio::time::timeout(Duration::from_secs(1), events.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert!(matches!(changed.kind, EventKind::Changed));
    assert_eq!(changed.cursor.position, m::WalPosition(1));
    let health = projection
        .step(jj_store_adapters::CancellationToken::new())
        .await
        .unwrap();
    assert_eq!(
        health.repository_position.unwrap().position,
        m::WalPosition(1)
    );
    assert_eq!(health.applied.unwrap().source.position, m::WalPosition(1));
    assert_eq!(
        projection_state
            .0
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .source
            .position,
        m::WalPosition(1)
    );

    let mut checkpoint = checkpoints.empty();
    checkpoint.revision = 1;
    checkpoint.pulled = Some(FeedCursorV1 {
        issuer: source_replica,
        log_epoch: feed.log_epoch,
        through: feed.through,
        observed_upper_bound: feed.upper_bound,
    });
    let saved = checkpoints.save(None, checkpoint).await.unwrap();

    destination.stop().await;
    let restarted = start(destination_memory, destination_objects, destination_token).await;
    let replay = restarted.client.apply_origins(&apply).await.unwrap();
    assert_eq!(replay, first);
    assert_eq!(
        checkpoints.load().await.unwrap().unwrap().checkpoint,
        saved.checkpoint
    );
    assert!(
        restarted
            .client
            .offer_origins(&OfferOriginsRequestV1 {
                schema_version: 1,
                offers,
            })
            .await
            .unwrap()
            .results
            .iter()
            .all(|result| matches!(result, wal::publication::OriginOfferResultV1::Known { .. }))
    );

    source.stop().await;
    restarted.stop().await;
}

#[tokio::test]
async fn bounded_worker_retries_with_stable_identity_and_converges_both_directions() {
    let local_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    let peer_store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
    let local_replica = m::ReplicaId::new([61; 16]);
    let peer_replica = m::ReplicaId::new([62; 16]);
    let principal = m::PrincipalId::new([63; 16]);
    let (local_token, local_objects) = initialize(
        local_store.clone(),
        m::EpochId::new([64; 16]),
        local_replica,
        m::WriterEpoch::new([65; 16]),
        principal,
    )
    .await;
    let (peer_token, peer_objects) = initialize(
        peer_store.clone(),
        m::EpochId::new([66; 16]),
        peer_replica,
        m::WriterEpoch::new([67; 16]),
        principal,
    )
    .await;
    let local = start(local_store.clone(), local_objects, local_token).await;
    let peer = start(peer_store, peer_objects, peer_token).await;

    let local_prepared = local
        .client
        .prepare_agent_capture(
            m::RequestId::new([68; 16]),
            m::PublicationId::new([69; 16]),
            &NormalizerProfileV1::codex_0_156_1(),
            &batch(),
            b"local capture".to_vec(),
        )
        .await
        .unwrap();
    local
        .client
        .publish_agent_delta(&local_prepared)
        .await
        .unwrap();
    let mut peer_batch = batch();
    peer_batch.batch_id = CaptureBatchId::new([70; 16]);
    peer_batch.source_instance_id = SourceInstanceId::new([71; 16]);
    peer_batch.observations[0].observation_id = ObservationId::new([72; 16]);
    let peer_prepared = peer
        .client
        .prepare_agent_capture(
            m::RequestId::new([73; 16]),
            m::PublicationId::new([74; 16]),
            &NormalizerProfileV1::codex_0_156_1(),
            &peer_batch,
            b"peer capture".to_vec(),
        )
        .await
        .unwrap();
    peer.client
        .publish_agent_delta(&peer_prepared)
        .await
        .unwrap();

    let local_negotiation = local
        .client
        .negotiate_replica(&NegotiateReplicaV1 {
            schema_version: 1,
            repository: REPOSITORY,
            peer: peer_replica,
            capabilities: ReplicaCapabilitiesV1::default(),
        })
        .await
        .unwrap();
    let local_origin = local
        .client
        .committed_feed(&FeedRequestV1 {
            schema_version: 1,
            source_replica: local_replica,
            log_epoch: local_negotiation.log_epoch,
            after: m::WalPosition(0),
            upper_bound: local_negotiation.feed_upper_bound,
            limit: 16,
        })
        .await
        .unwrap()
        .page
        .origins[0]
        .envelope
        .clone();
    let peer_negotiation = peer
        .client
        .negotiate_replica(&NegotiateReplicaV1 {
            schema_version: 1,
            repository: REPOSITORY,
            peer: local_replica,
            capabilities: ReplicaCapabilitiesV1::default(),
        })
        .await
        .unwrap();
    let peer_origin = peer
        .client
        .committed_feed(&FeedRequestV1 {
            schema_version: 1,
            source_replica: peer_replica,
            log_epoch: peer_negotiation.log_epoch,
            after: m::WalPosition(0),
            upper_bound: peer_negotiation.feed_upper_bound,
            limit: 16,
        })
        .await
        .unwrap()
        .page
        .origins[0]
        .envelope
        .clone();

    let local_endpoint = RpcEndpoint::new(local.client.clone());
    let peer_endpoint = RpcEndpoint::new(peer.client.clone());
    local_endpoint.fail_next_apply_before_send();
    let checkpoints =
        PeerCheckpointStore::new(local_store, REPOSITORY, local_replica, peer_replica).unwrap();
    let mut worker = ReplicationWorker::new(
        REPOSITORY,
        local_replica,
        peer_replica,
        local_endpoint.clone(),
        peer_endpoint,
        checkpoints.clone(),
        WorkerLimits {
            poll_interval: Duration::from_millis(10),
            ..WorkerLimits::default()
        },
    )
    .unwrap();
    let barrier = worker.capture_barrier().await.unwrap();

    assert!(worker.step().await.is_err());
    assert!(checkpoints.load().await.unwrap().is_none());
    let pushed = worker.step().await.unwrap();
    assert_eq!(pushed.direction, jj_store_replication::Direction::Push);
    assert_eq!(pushed.transferred_origins, 1);
    let pulled = worker.step().await.unwrap();
    assert_eq!(pulled.direction, jj_store_replication::Direction::Pull);
    assert_eq!(pulled.transferred_origins, 1);
    let ids = local_endpoint.apply_ids();
    assert_eq!(ids.len(), 2);
    assert_eq!(ids[0], ids[1]);
    let status = worker.handle().status();
    assert!(barrier.satisfied_by(&status));
    assert_eq!(status.checkpoint_revision, 2);

    let local_offer = local
        .client
        .offer_origins(&OfferOriginsRequestV1 {
            schema_version: 1,
            offers: vec![wal::publication::OriginOfferV1 {
                origin_key: peer_origin.publication.origin_key,
                origin_digest: peer_origin.origin_digest,
            }],
        })
        .await
        .unwrap();
    assert!(matches!(
        local_offer.results.as_slice(),
        [wal::publication::OriginOfferResultV1::Known { .. }]
    ));
    let peer_offer = peer
        .client
        .offer_origins(&OfferOriginsRequestV1 {
            schema_version: 1,
            offers: vec![wal::publication::OriginOfferV1 {
                origin_key: local_origin.publication.origin_key,
                origin_digest: local_origin.origin_digest,
            }],
        })
        .await
        .unwrap();
    assert!(matches!(
        peer_offer.results.as_slice(),
        [wal::publication::OriginOfferResultV1::Known { .. }]
    ));

    local.stop().await;
    peer.stop().await;
}
