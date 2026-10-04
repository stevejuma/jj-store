#![cfg(unix)]
use jj_agent::{
    CaptureBatchId, IntegrationId, NormalizerProfileV1, ObservationId, SanitizedMediaType,
    SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
    SanitizedRepresentation, SourceInstanceId, SourceKind, SourceTrust,
};
use jj_store_jj::policy::Protection;
use jj_store_model as m;
use jj_store_object::memory::{Fault, MemoryStore};
use jj_store_rpc::{
    client::{NativeClient, Options},
    server::{Limits, NativeRpc},
};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    auth::{self, Permission, Scope, SystemClock, TokenPolicy},
    native::SessionLimits,
};
use jj_store_wal::publication::PublishOutcome;
use std::{os::unix::fs::MetadataExt as _, sync::Arc, time::Duration};
use tonic::{Code, transport::Server};

struct NoOperators;
#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(TokenPolicy::new(Scope::Operator))
    }
}

fn batch(seed: u8, session: &str) -> SanitizedObservationBatchV1 {
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([seed; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([seed + 1; 16]),
        sanitizer_version: "mongodb-kingfisher@2.7.0/builtin-medium/v1".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![SanitizedObservationV1 {
            observation_id: ObservationId::new([seed + 2; 16]),
            source_sequence: 1,
            occurred_at_millis: Some(1_725_000_000_000),
            source_kind: SourceKind::Hook,
            source_trust: SourceTrust::LocalUserHook,
            payload: SanitizedPayloadV1::new(SanitizedMediaType::Json, session.as_bytes()).unwrap(),
        }],
    }
}

#[tokio::test]
async fn local_peer_prepares_and_replays_exact_agent_publication() {
    let root = tempfile::tempdir().unwrap();
    let socket = root.path().join("native.sock");
    let owner_uid = std::fs::metadata(root.path()).unwrap().uid();
    let repository = m::RepositoryId::new([21; 16]);
    let epoch = m::EpochId::new([22; 16]);
    let principal = m::PrincipalId::new([23; 16]);
    let store = Arc::new(MemoryStore::default());
    let host = RepositoryHost::new(
        store.clone(),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        1024 * 1024,
    )
    .unwrap();
    let rpc = NativeRpc::hosted_local_trusted(
        host,
        Limits::default(),
        principal,
        Permission::Admin,
        owner_uid,
    )
    .unwrap();
    let listener = tokio::net::UnixListener::bind(&socket).unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        Server::builder()
            .add_service(rpc.service())
            .serve_with_incoming_shutdown(
                tokio_stream::wrappers::UnixListenerStream::new(listener),
                async {
                    let _ = stopped.await;
                },
            )
            .await
            .unwrap();
    });
    let client = NativeClient::connect_unix(
        &socket,
        repository,
        Options {
            timeout: Duration::from_secs(5),
            object_bytes: 1024 * 1024,
            ..Options::default()
        },
    )
    .await
    .unwrap();
    client.create_repository_local(epoch).await.unwrap();

    let request_id = m::RequestId::new([24; 16]);
    let publication_id = m::PublicationId::new([25; 16]);
    let result = b"capture accepted".to_vec();
    let prepared = client
        .prepare_agent_capture(
            request_id,
            publication_id,
            &NormalizerProfileV1::codex_0_156_1(),
            &batch(
                11,
                r#"{"hook_event_name":"SessionStart","session_id":"session-1"}"#,
            ),
            result.clone(),
        )
        .await
        .unwrap();
    assert_eq!(prepared.principal, principal);
    assert_eq!(prepared.request_id, request_id);
    assert_eq!(prepared.publication_id, publication_id);
    assert_eq!(prepared.result, result);

    let first = client.publish_agent_delta(&prepared).await.unwrap();
    let PublishOutcome::Committed(first_receipt) = first else {
        panic!("expected a committed agent publication");
    };
    assert_eq!(first_receipt.principal, principal);
    assert_eq!(first_receipt.request_id, request_id);
    assert_eq!(first_receipt.publication_id, publication_id);
    let replay = client.publish_agent_delta(&prepared).await.unwrap();
    assert_eq!(replay, PublishOutcome::Committed(first_receipt));

    let mut foreign = prepared.clone();
    foreign.principal = m::PrincipalId::new([99; 16]);
    assert_eq!(
        client
            .publish_agent_delta(&foreign)
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );

    let unknown_request = client
        .prepare_agent_capture(
            m::RequestId::new([34; 16]),
            m::PublicationId::new([35; 16]),
            &NormalizerProfileV1::codex_0_156_1(),
            &batch(
                31,
                r#"{"hook_event_name":"SessionStart","session_id":"session-2"}"#,
            ),
            b"second capture".to_vec(),
        )
        .await
        .unwrap();
    store.inject(Fault::BeforeWrite);
    assert_eq!(
        client.publish_agent_delta(&unknown_request).await.unwrap(),
        PublishOutcome::OutcomeUnknown {
            request_id: unknown_request.request_id,
            publication_id: unknown_request.publication_id,
        }
    );
    assert!(matches!(
        client.publish_agent_delta(&unknown_request).await.unwrap(),
        PublishOutcome::Committed(_)
    ));

    let _ = stop.send(());
    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
}
