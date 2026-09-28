use super::*;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectV1,
    AgentRelationPageV1, AgentRelationSubjectV1, AgentSessionId, AgentSessionSnapshotV1,
    AssociationGranularity, AssociationId, AssociationRelation, CaptureBatchId, CodeAnchorV1,
    CodeAssociationV1, CompletenessV1, CoverageV1, EvidenceMethod, IntegrationId,
    NormalizerProfileV1, ObservationId, SanitizedMediaType, SanitizedObservationBatchV1,
    SanitizedObservationV1, SanitizedPayloadV1, SanitizedRepresentation, SessionState,
    SourceInstanceId, SourceKind, SourceTrust, TranscriptChunkV1, TranscriptManifestV1,
    TranscriptPageV1, TranscriptStreamV1,
};
use jj_store_http::{Access, Config, HttpApi};
use jj_store_service::administration::{OperatorPolicies, RepositoryHost};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use tower::ServiceExt as _;

struct NoOperators;
#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        panic!("LocalTrusted collaboration read accessed operator policy")
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

fn association(seed: u8, session_id: AgentSessionId) -> CodeAssociationV1 {
    CodeAssociationV1 {
        schema_version: 1,
        association_id: AssociationId::new([seed; 16]),
        checkpoint_id: None,
        session_id,
        turn_id: None,
        tool_call_ids: BTreeSet::new(),
        scope_id: None,
        relation: AssociationRelation::ObservedDuringInterval,
        before: CodeAnchorV1 {
            operations: BTreeSet::from([m::OperationId::new([0; 64])]),
            commits: BTreeSet::new(),
        },
        after: CodeAnchorV1::default(),
        changes: BTreeSet::new(),
        paths: vec![],
        granularity: AssociationGranularity::Operation,
        evidence_method: EvidenceMethod::HookBoundary,
        source_trust: SourceTrust::LocalUserHook,
        exclusive_write_scope: false,
        evidence: vec![],
        coverage: CoverageV1 {
            completeness: CompletenessV1::Complete,
            gaps: 0,
            redacted_fields: 0,
            truncated: false,
        },
        created_at_millis: i64::from(seed),
        supersedes: None,
        correction_reason: None,
    }
}

async fn publish_transcript(
    fixture: &Fixture,
) -> (
    AgentSessionId,
    AgentObjectId,
    SourceInstanceId,
    [AgentObjectId; 2],
) {
    let staging = Coordinator::new(
        fixture.store.clone(),
        REPO,
        fixture.objects.clone(),
        Arc::new(NoPublication),
        Limits::default(),
    )
    .unwrap();
    let source = SourceInstanceId::new([71; 16]);
    let session_id = AgentSessionId::new([72; 16]);
    let first = staging
        .stage_agent_object(&AgentObjectV1::TranscriptChunk(TranscriptChunkV1 {
            schema_version: 1,
            source_instance_id: source,
            source_generation: 5,
            sequence: 10,
            payload: SanitizedPayloadV1::new(SanitizedMediaType::Text, b"first\n").unwrap(),
        }))
        .await
        .unwrap();
    let second = staging
        .stage_agent_object(&AgentObjectV1::TranscriptChunk(TranscriptChunkV1 {
            schema_version: 1,
            source_instance_id: source,
            source_generation: 5,
            sequence: 11,
            payload: SanitizedPayloadV1::new(SanitizedMediaType::Text, b"second\n").unwrap(),
        }))
        .await
        .unwrap();
    let tail = staging
        .stage_agent_object(&AgentObjectV1::TranscriptPage(TranscriptPageV1 {
            schema_version: 1,
            source_instance_id: source,
            source_generation: 5,
            chunks: vec![second],
            next: None,
        }))
        .await
        .unwrap();
    let head = staging
        .stage_agent_object(&AgentObjectV1::TranscriptPage(TranscriptPageV1 {
            schema_version: 1,
            source_instance_id: source,
            source_generation: 5,
            chunks: vec![first],
            next: Some(tail),
        }))
        .await
        .unwrap();
    let transcript = staging
        .stage_agent_object(&AgentObjectV1::TranscriptManifest(TranscriptManifestV1 {
            schema_version: 1,
            session_id,
            streams: vec![TranscriptStreamV1 {
                source_instance_id: source,
                source_generation: 5,
                source_format: "codex-jsonl".into(),
                decoder_version: "codex-0.156.1/v1".into(),
                sanitizer_version: "mongodb-kingfisher@2.7.0/builtin-medium/v1".into(),
                representation: SanitizedRepresentation::SanitizedNative,
                ordered_chunk_root: head,
                logical_byte_length: 13,
                record_count: 2,
                gaps: 0,
            }],
            coverage: CoverageV1 {
                completeness: CompletenessV1::Complete,
                gaps: 0,
                redacted_fields: 0,
                truncated: false,
            },
        }))
        .await
        .unwrap();
    let snapshot = staging
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
            coverage: CoverageV1 {
                completeness: CompletenessV1::Complete,
                gaps: 0,
                redacted_fields: 0,
                truncated: false,
            },
        }))
        .await
        .unwrap();
    let first_association = staging
        .stage_agent_object(&AgentObjectV1::Association(association(75, session_id)))
        .await
        .unwrap();
    let second_association = staging
        .stage_agent_object(&AgentObjectV1::Association(association(76, session_id)))
        .await
        .unwrap();
    let subject = AgentRelationSubjectV1::Session { session_id };
    let relation_tail = staging
        .stage_agent_object(&AgentObjectV1::RelationPage(AgentRelationPageV1 {
            schema_version: 1,
            subject: subject.clone(),
            selections: vec![second_association],
            next: None,
        }))
        .await
        .unwrap();
    let relation_head = staging
        .stage_agent_object(&AgentObjectV1::RelationPage(AgentRelationPageV1 {
            schema_version: 1,
            subject: subject.clone(),
            selections: vec![first_association],
            next: Some(relation_tail),
        }))
        .await
        .unwrap();
    let request = AgentDeltaRequest::new(
        ACTOR,
        m::RequestId::new([73; 16]),
        m::PublicationId::new([74; 16]),
        AgentDeltaV1 {
            schema_version: 1,
            edits: vec![
                AgentCatalogEditV1 {
                    catalog: AgentCatalogKind::Sessions,
                    key: AgentCatalogKey::derive("session", session_id.as_bytes()).unwrap(),
                    expected: None,
                    replacement: Some(snapshot),
                },
                AgentCatalogEditV1 {
                    catalog: AgentCatalogKind::Relations,
                    key: subject.catalog_key().unwrap(),
                    expected: None,
                    replacement: Some(relation_head),
                },
            ],
        },
        b"published transcript fixture".to_vec(),
    )
    .unwrap();
    assert!(matches!(
        fixture.session.publish_agent_delta(&request).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    (
        session_id,
        transcript.id,
        source,
        [first_association.id, second_association.id],
    )
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

    let (transcript_session, transcript_id, source, associations) =
        publish_transcript(&fixture).await;
    let (status, transcript_detail) =
        json(&router, &format!("agent/sessions/{transcript_session}")).await;
    assert_eq!(status, StatusCode::OK, "{transcript_detail}");
    assert_eq!(
        transcript_detail["transcripts"][0]["transcript_id"],
        transcript_id.to_string()
    );
    assert_eq!(
        transcript_detail["transcripts"][0]["source_format"],
        "codex-jsonl"
    );
    let transcript_path = format!(
        "agent/sessions/{transcript_session}/transcripts/{transcript_id}?source={source}&generation=5&limit=1&max_bytes=1024"
    );
    let (status, transcript_first) = json(&router, &transcript_path).await;
    assert_eq!(status, StatusCode::OK, "{transcript_first}");
    assert_eq!(transcript_first["chunks"][0]["content"], "first\n");
    let transcript_cursor = transcript_first["next_cursor"].as_str().unwrap();
    publish(&fixture.session, 81, "later-http-session").await;
    let (status, transcript_second) = json(
        &router,
        &format!("{transcript_path}&cursor={transcript_cursor}"),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{transcript_second}");
    assert_eq!(transcript_second["snapshot"], transcript_first["snapshot"]);
    assert_eq!(transcript_second["chunks"][0]["content"], "second\n");
    assert!(transcript_second["next_cursor"].is_null());

    let relation_path = format!("agent/associations?session={transcript_session}&limit=1");
    let (status, relation_first) = json(&router, &relation_path).await;
    assert_eq!(status, StatusCode::OK, "{relation_first}");
    assert_eq!(relation_first["relations"][0]["kind"], "association");
    assert_eq!(
        relation_first["relations"][0]["object_id"],
        associations[0].to_string()
    );
    let relation_cursor = relation_first["next_cursor"].as_str().unwrap();
    publish(&fixture.session, 91, "relation-later-session").await;
    let (status, relation_second) = json(
        &router,
        &format!("{relation_path}&cursor={relation_cursor}"),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{relation_second}");
    assert_eq!(relation_second["snapshot"], relation_first["snapshot"]);
    assert_eq!(
        relation_second["relations"][0]["object_id"],
        associations[1].to_string()
    );
    assert!(relation_second["next_cursor"].is_null());
    for (query, subject) in [
        (
            format!("tool_session={transcript_session}&tool={}", "00".repeat(16)),
            "tool",
        ),
        (format!("operation={}", "00".repeat(64)), "operation"),
        (format!("commit={}", "00".repeat(64)), "revision"),
        (format!("checkpoint={}", "00".repeat(16)), "checkpoint"),
    ] {
        let (status, empty) = json(&router, &format!("agent/associations?{query}")).await;
        assert_eq!(status, StatusCode::OK, "{empty}");
        assert_eq!(empty["subject"]["subject"], subject);
        assert!(empty["relations"].as_array().unwrap().is_empty());
    }
    let (status, invalid) = json(
        &router,
        &format!(
            "agent/associations?session={transcript_session}&operation={}",
            "00".repeat(64)
        ),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{invalid}");

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
        samples.push(serde_json::json!({
            "path": "/v1/repos/{repo}/agent/sessions/{session}",
            "method": "get",
            "status": 200,
            "media": "application/json",
            "data": transcript_detail,
        }));
        for data in [transcript_first, transcript_second] {
            samples.push(serde_json::json!({
                "path": "/v1/repos/{repo}/agent/sessions/{session}/transcripts/{transcript}",
                "method": "get",
                "status": 200,
                "media": "application/json",
                "data": data,
            }));
        }
        for data in [relation_first, relation_second] {
            samples.push(serde_json::json!({
                "path": "/v1/repos/{repo}/agent/associations",
                "method": "get",
                "status": 200,
                "media": "application/json",
                "data": data,
            }));
        }
        std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
}
