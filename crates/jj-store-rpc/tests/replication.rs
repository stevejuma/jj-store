use jj_agent::{
    CaptureBatchId, IntegrationId, NormalizerProfileV1, ObservationId, SanitizedMediaType,
    SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
    SanitizedRepresentation, SourceInstanceId, SourceKind, SourceTrust,
};
use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::{ObjectStore, WriteOutcome, memory::MemoryStore};
use jj_store_replication::{
    ApplyOriginsTransportRequestV1, FeedCursorV1, FeedRequestV1, NegotiateReplicaV1,
    OfferOriginsRequestV1, OriginClosureRequestV1, PeerCheckpointStore, ReplicaCapabilitiesV1,
};
use jj_store_rpc::{
    client::{NativeClient, Options},
    server::{Limits as RpcLimits, NativeRpc},
};
use jj_store_service::{
    auth::{BearerToken, Permission, Scope, SystemClock, TokenPolicy},
    native::SessionLimits,
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
use std::{sync::Arc, time::Duration};

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
        stop: Some(stop),
        task,
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
