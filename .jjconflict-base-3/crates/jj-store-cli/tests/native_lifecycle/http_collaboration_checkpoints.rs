use super::*;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentCheckpointId,
    AgentCheckpointVersionV1, AgentDeltaV1, AgentObjectRef, AgentObjectV1, AgentSessionId,
    AgentSessionSnapshotV1, CaptureStatusV1, CodeAnchorV1, CodeRelation, CompletenessV1,
    CoverageV1, EvidenceMethod, ExecutionOutcomeV1, IntegrationId, SessionState, SourceTrust,
    TranscriptManifestV1,
};
use jj_store_http::{Access, Config, HttpApi};
use jj_store_service::administration::{OperatorPolicies, RepositoryHost};
use serde_json::Value;
use std::collections::BTreeMap;
use tower::ServiceExt as _;

struct NoOperators;
#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        panic!("LocalTrusted checkpoint read accessed operator policy")
    }
}

struct NoPublication;
#[async_trait::async_trait]
impl PublicationGuard for NoPublication {
    async fn authorize(
        &self,
        _: &ManifestSnapshot,
        _: &wal::publication::Request,
    ) -> wal::Result<()> {
        Err(wal::Error::Denied)
    }

    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &wal::publication::Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Err(wal::Error::Denied)
    }
}

fn coverage() -> CoverageV1 {
    CoverageV1 {
        completeness: CompletenessV1::Complete,
        gaps: 0,
        redacted_fields: 0,
        truncated: false,
    }
}

async fn support(
    staging: &Coordinator,
    seed: u8,
) -> (AgentSessionId, AgentObjectRef, AgentObjectRef) {
    let session_id = AgentSessionId::new([seed; 16]);
    let transcript = staging
        .stage_agent_object(&AgentObjectV1::TranscriptManifest(TranscriptManifestV1 {
            schema_version: 1,
            session_id,
            streams: vec![],
            coverage: coverage(),
        }))
        .await
        .unwrap();
    let session = staging
        .stage_agent_object(&AgentObjectV1::SessionSnapshot(AgentSessionSnapshotV1 {
            schema_version: 1,
            session_id,
            integration: IntegrationId::Codex,
            state: SessionState::Ended,
            parent_session_id: None,
            evidence: vec![],
            event_batches: vec![],
            transcripts: vec![transcript],
            tools: BTreeMap::new(),
            coverage: coverage(),
        }))
        .await
        .unwrap();
    (session_id, session, transcript)
}

async fn stage_version(
    staging: &Coordinator,
    checkpoint_id: AgentCheckpointId,
    support: (AgentSessionId, AgentObjectRef, AgentObjectRef),
    outcome: ExecutionOutcomeV1,
    supersedes: Option<AgentObjectRef>,
) -> AgentObjectRef {
    let anchor = CodeAnchorV1 {
        operations: BTreeSet::from([m::OperationId::new([0; 64])]),
        commits: BTreeSet::new(),
    };
    staging
        .stage_agent_object(&AgentObjectV1::CheckpointVersion(
            AgentCheckpointVersionV1 {
                schema_version: 1,
                checkpoint_id,
                session_id: support.0,
                turn_id: None,
                scope_id: None,
                session_snapshot: support.1,
                transcript_manifest: support.2,
                event_batches: vec![],
                tools: BTreeMap::new(),
                before: anchor.clone(),
                after: anchor,
                code_relation: CodeRelation::NoChange,
                associations: vec![],
                evidence_method: EvidenceMethod::ControlledScope,
                source_trust: SourceTrust::AuthenticatedController,
                exclusive_write_scope: true,
                coverage: coverage(),
                capture_status: CaptureStatusV1::Complete,
                execution_outcome: outcome,
                supersedes,
            },
        ))
        .await
        .unwrap()
}

async fn publish(
    fixture: &Fixture,
    checkpoint_id: AgentCheckpointId,
    expected: Option<AgentObjectRef>,
    replacement: AgentObjectRef,
    seed: u8,
) {
    let request = AgentDeltaRequest::new(
        ACTOR,
        m::RequestId::new([seed; 16]),
        m::PublicationId::new([seed + 1; 16]),
        AgentDeltaV1 {
            schema_version: 1,
            edits: vec![AgentCatalogEditV1 {
                catalog: AgentCatalogKind::Checkpoints,
                key: AgentCatalogKey::derive("checkpoint", checkpoint_id.as_bytes()).unwrap(),
                expected,
                replacement: Some(replacement),
            }],
        },
        b"published HTTP checkpoint fixture".to_vec(),
    )
    .unwrap();
    assert!(matches!(
        fixture.session.publish_agent_delta(&request).await.unwrap(),
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
async fn local_trusted_http_pages_current_and_historical_agent_checkpoints() {
    let fixture = Fixture::new().await;
    let staging = Coordinator::new(
        fixture.store.clone(),
        REPO,
        fixture.objects.clone(),
        Arc::new(NoPublication),
        Limits::default(),
    )
    .unwrap();
    let first_id = AgentCheckpointId::new([101; 16]);
    let second_id = AgentCheckpointId::new([102; 16]);
    let first_support = support(&staging, 103).await;
    let second_support = support(&staging, 104).await;
    let first_v1 = stage_version(
        &staging,
        first_id,
        first_support,
        ExecutionOutcomeV1::Succeeded,
        None,
    )
    .await;
    let second_v1 = stage_version(
        &staging,
        second_id,
        second_support,
        ExecutionOutcomeV1::Succeeded,
        None,
    )
    .await;
    publish(&fixture, first_id, None, first_v1, 105).await;
    publish(&fixture, second_id, None, second_v1, 107).await;

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

    let (status, first) = json(&router, "agent/checkpoints?limit=1").await;
    assert_eq!(status, StatusCode::OK, "{first}");
    let cursor = first["next_cursor"].as_str().unwrap();
    let first_v2 = stage_version(
        &staging,
        first_id,
        first_support,
        ExecutionOutcomeV1::Failed,
        Some(first_v1),
    )
    .await;
    publish(&fixture, first_id, Some(first_v1), first_v2, 109).await;
    let (status, second) = json(
        &router,
        &format!("agent/checkpoints?limit=1&cursor={cursor}"),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{second}");
    assert_eq!(second["snapshot"], first["snapshot"]);

    let (status, detail) = json(&router, &format!("agent/checkpoints/{first_id}")).await;
    assert_eq!(status, StatusCode::OK, "{detail}");
    assert_eq!(detail["checkpoint"]["version_id"], first_v2.id.to_string());
    assert_eq!(detail["checkpoint"]["supersedes"], first_v1.id.to_string());
    let (status, historical) = json(
        &router,
        &format!("agent/checkpoints/{first_id}/versions/{}", first_v1.id),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{historical}");
    assert_eq!(
        historical["checkpoint"]["version_id"],
        first_v1.id.to_string()
    );
    let (status, missing) = json(
        &router,
        &format!("agent/checkpoints/{first_id}/versions/{}", second_v1.id),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{missing}");

    if let Some(path) = std::env::var_os("JJ_STORE_CHECKPOINT_CONTRACT_CAPTURE") {
        let samples = [
            ("/v1/repos/{repo}/agent/checkpoints", first),
            ("/v1/repos/{repo}/agent/checkpoints", second),
            ("/v1/repos/{repo}/agent/checkpoints/{checkpoint}", detail),
            (
                "/v1/repos/{repo}/agent/checkpoints/{checkpoint}/versions/{version}",
                historical,
            ),
        ]
        .map(|(path, data)| {
            serde_json::json!({
                "path": path,
                "method": "get",
                "status": 200,
                "media": "application/json",
                "data": data,
            })
        });
        std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
}
