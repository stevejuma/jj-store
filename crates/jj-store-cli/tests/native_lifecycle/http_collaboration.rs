use super::*;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use jj_agent::{
    CaptureBatchId, IntegrationId, NormalizerProfileV1, ObservationId, SanitizedMediaType,
    SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
    SanitizedRepresentation, SourceInstanceId, SourceKind, SourceTrust,
};
use jj_store_http::{Access, Config, HttpApi};
use jj_store_service::administration::{OperatorPolicies, RepositoryHost};
use serde_json::Value;
use tower::ServiceExt as _;

struct NoOperators;
#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        panic!("LocalTrusted collaboration read accessed operator policy")
    }
}

fn batch(seed: u8, native_session: &str) -> SanitizedObservationBatchV1 {
    let payload = serde_json::json!({
        "hook_event_name": "SessionStart",
        "session_id": native_session,
    });
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
            occurred_at_millis: Some(1_725_000_000_000 + i64::from(seed)),
            source_kind: SourceKind::Hook,
            source_trust: SourceTrust::LocalUserHook,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                &serde_json::to_vec(&payload).unwrap(),
            )
            .unwrap(),
        }],
    }
}

async fn publish(session: &NativeSession, seed: u8, native_session: &str) {
    let request = session
        .prepare_agent_capture(
            m::RequestId::new([seed + 3; 16]),
            m::PublicationId::new([seed + 4; 16]),
            &NormalizerProfileV1::codex_0_156_1(),
            &batch(seed, native_session),
            b"accepted".to_vec(),
        )
        .await
        .unwrap();
    assert!(matches!(
        session.publish_agent_delta(&request).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
}

fn request(path: &str) -> Request<Body> {
    Request::builder()
        .uri(format!("/v1/repos/{REPO}/{path}"))
        .header("host", "127.0.0.1:7443")
        .header("x-jj-store-protocol", "1")
        .body(Body::empty())
        .unwrap()
}

async fn json(router: &axum::Router, path: &str) -> (StatusCode, Value) {
    let response = router.clone().oneshot(request(path)).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (status, serde_json::from_slice(&bytes).unwrap())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn local_trusted_http_lists_root_pinned_agent_sessions_without_bearer_credentials() {
    let fixture = Fixture::new().await;
    publish(&fixture.session, 51, "http-session-1").await;
    publish(&fixture.session, 61, "http-session-2").await;
    let host = RepositoryHost::new(
        fixture.store.clone(),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap();
    let cache = tempfile::tempdir().unwrap();
    let router = HttpApi::new(
        host,
        cache.path().to_owned(),
        Config {
            access: Access::LocalTrusted {
                principal: ACTOR,
                permission: Permission::Admin,
            },
            ..Config::default()
        },
    )
    .unwrap()
    .router();

    let (status, first) = json(&router, "agent/sessions?limit=1").await;
    assert_eq!(status, StatusCode::OK, "{first}");
    assert_eq!(first["version"], 1);
    assert_eq!(first["snapshot"]["repo_id"], REPO.to_string());
    assert!(first["snapshot"]["agent_root"].is_string());
    assert_eq!(first["sessions"].as_array().unwrap().len(), 1);
    let session_id = first["sessions"][0]["session_id"].as_str().unwrap();
    let cursor = first["next_cursor"].as_str().unwrap();

    let (status, second) = json(&router, &format!("agent/sessions?limit=1&cursor={cursor}")).await;
    assert_eq!(status, StatusCode::OK, "{second}");
    assert_eq!(second["snapshot"], first["snapshot"]);
    assert_eq!(second["sessions"].as_array().unwrap().len(), 1);
    assert!(second["next_cursor"].is_null());

    let (status, detail) = json(&router, &format!("agent/sessions/{session_id}")).await;
    assert_eq!(status, StatusCode::OK, "{detail}");
    assert_eq!(detail["snapshot"], first["snapshot"]);
    assert_eq!(detail["session"], first["sessions"][0]);
    assert!(detail["evidence"].is_array());
    assert!(detail["event_batches"].is_array());
    assert!(detail["transcripts"].is_array());
    assert!(detail["tools"].is_array());

    let (status, missing) = json(&router, &format!("agent/sessions/{}", "99".repeat(16))).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{missing}");
    if let Some(path) = std::env::var_os("JJ_STORE_COLLABORATION_CONTRACT_CAPTURE") {
        let pages = [first, second].map(|data| {
            serde_json::json!({
                "path": "/v1/repos/{repo}/agent/sessions",
                "method": "get",
                "status": 200,
                "media": "application/json",
                "data": data,
            })
        });
        let mut samples = pages.to_vec();
        samples.push(serde_json::json!({
            "path": "/v1/repos/{repo}/agent/sessions/{session}",
            "method": "get",
            "status": 200,
            "media": "application/json",
            "data": detail,
        }));
        std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
}
