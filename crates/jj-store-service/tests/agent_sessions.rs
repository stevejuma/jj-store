use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_service::{
    auth::{Permission, SystemClock},
    collaboration::{AgentActivity, AgentSessionPager, AgentTranscriptRequest, Error},
    native::{NativeSession, SessionLimits, create_repository_local_trusted},
    pagination,
};
use jj_store_wal::publication::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaRequest, AgentDeltaV1,
    AgentEventBatchV1, AgentEventId, AgentEventV1, AgentObjectV1, AgentSessionId,
    AgentSessionSnapshotV1, CaptureBatchId, CompletenessV1, Coordinator, CoverageV1, IntegrationId,
    MessageRole, NormalizerProfileV1, ObservationId, PublicationGuard, SanitizedMediaType,
    SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
    SanitizedRepresentation, SessionState, SourceInstanceId, SourceKind, SourceTrust, ToolKind,
    ToolState, TranscriptChunkV1, TranscriptManifestV1, TranscriptPageV1, TranscriptStreamV1,
};
use jj_store_wal::publication::{Limits, PublishOutcome};
use std::{collections::BTreeMap, collections::BTreeSet, sync::Arc};

const REPOSITORY: m::RepositoryId = m::RepositoryId::new([41; 16]);
const EPOCH: m::EpochId = m::EpochId::new([42; 16]);
const PRINCIPAL: m::PrincipalId = m::PrincipalId::new([43; 16]);

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

fn rich_cursor_batch() -> SanitizedObservationBatchV1 {
    let values: Vec<serde_json::Value> = serde_json::from_slice(include_bytes!(
        "../../jj-agent/tests/fixtures/cursor-3.22.7-hooks.json"
    ))
    .unwrap();
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([101; 16]),
        integration: IntegrationId::Cursor,
        source_instance_id: SourceInstanceId::new([102; 16]),
        sanitizer_version: "mongodb-kingfisher@2.7.0/builtin-medium/v1".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: values
            .into_iter()
            .enumerate()
            .map(|(index, value)| SanitizedObservationV1 {
                observation_id: ObservationId::new([index as u8 + 1; 16]),
                source_sequence: index as u64 + 1,
                occurred_at_millis: Some(1_725_000_000_000 + index as i64),
                source_kind: SourceKind::Hook,
                source_trust: SourceTrust::LocalUserHook,
                payload: SanitizedPayloadV1::new(
                    SanitizedMediaType::Json,
                    &serde_json::to_vec(&value).unwrap(),
                )
                .unwrap(),
            })
            .collect(),
    }
}

struct NoPublication;
#[async_trait::async_trait]
impl PublicationGuard for NoPublication {
    async fn authorize(
        &self,
        _: &jj_store_wal::publication::ManifestSnapshot,
        _: &jj_store_wal::publication::Request,
    ) -> jj_store_wal::Result<()> {
        Err(jj_store_wal::Error::Denied)
    }

    async fn validate_transition(
        &self,
        _: &jj_store_wal::publication::ManifestSnapshot,
        _: &jj_store_wal::publication::Request,
        _: &jj_store_wal::publication::Snapshot,
    ) -> jj_store_wal::Result<()> {
        Err(jj_store_wal::Error::Denied)
    }
}

struct Fixture {
    session: Arc<NativeSession>,
    staging: Coordinator,
}

async fn fixture() -> Fixture {
    let store = Arc::new(MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), REPOSITORY, 1024 * 1024).unwrap());
    create_repository_local_trusted(
        store.clone(),
        objects.clone(),
        Permission::Admin,
        EPOCH,
        Limits::default(),
    )
    .await
    .unwrap();
    let staging = Coordinator::new(
        store.clone(),
        REPOSITORY,
        objects.clone(),
        Arc::new(NoPublication),
        Limits::default(),
    )
    .unwrap();
    let session = Arc::new(
        NativeSession::new_local_trusted(
            store,
            objects,
            PRINCIPAL,
            Permission::Admin,
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    Fixture { session, staging }
}

async fn publish(session: &NativeSession, seed: u8, native_session: &str) {
    let request = session
        .prepare_agent_capture(
            m::RequestId::new([seed + 3; 16]),
            m::PublicationId::new([seed + 4; 16]),
            &NormalizerProfileV1::codex_0_156_1(),
            &batch(seed, native_session),
            format!("accepted {native_session}").into_bytes(),
        )
        .await
        .unwrap();
    assert!(matches!(
        session.publish_agent_delta(&request).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
}

#[tokio::test]
async fn session_pages_retain_one_agent_root_across_later_publication() {
    let fixture = fixture().await;
    let session = fixture.session;
    publish(&session, 11, "native-session-1").await;
    publish(&session, 21, "native-session-2").await;
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();

    let first = pager.begin(session.clone(), 1).await.unwrap();
    assert_eq!(first.version, 1);
    assert_eq!(first.snapshot.repo_id, REPOSITORY);
    assert_eq!(first.snapshot.epoch, EPOCH);
    assert_eq!(first.sessions.len(), 1);
    let cursor = first.next_cursor.clone().unwrap();

    publish(&session, 31, "native-session-3").await;
    let second = pager.resume(session.clone(), &cursor, 1).await.unwrap();
    assert_eq!(second.snapshot, first.snapshot);
    assert_eq!(second.sessions.len(), 1);
    assert!(second.next_cursor.is_none());
    let pinned: BTreeSet<_> = first
        .sessions
        .iter()
        .chain(&second.sessions)
        .map(|row| row.session_id)
        .collect();
    assert_eq!(pinned.len(), 2);

    let current = pager.begin(session.clone(), 10).await.unwrap();
    assert!(current.snapshot.agent_state_version > first.snapshot.agent_state_version);
    assert_eq!(current.sessions.len(), 3);
    assert!(current.next_cursor.is_none());

    assert_eq!(
        pager.resume(session.clone(), &cursor, 2).await.unwrap_err(),
        Error::InvalidCursor
    );
    let mut tampered = cursor;
    let tail = tampered.pop().unwrap();
    tampered.push(if tail == 'A' { 'B' } else { 'A' });
    assert_eq!(
        pager.resume(session, &tampered, 1).await.unwrap_err(),
        Error::InvalidCursor
    );
}

#[tokio::test]
async fn session_detail_resolves_current_stable_identity_and_exact_links() {
    let fixture = fixture().await;
    let session = fixture.session;
    publish(&session, 41, "native-detail-session").await;
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    let page = pager.begin(session.clone(), 10).await.unwrap();
    let summary = page.sessions.first().unwrap();
    assert_eq!(summary.first_activity_at_millis, Some(1_725_000_000_041));
    assert_eq!(
        summary.last_activity_at_millis,
        summary.first_activity_at_millis
    );

    let detail = pager
        .detail(session.clone(), summary.session_id)
        .await
        .unwrap();
    assert_eq!(detail.version, 1);
    assert_eq!(detail.snapshot, page.snapshot);
    assert_eq!(&detail.session, summary);
    assert_eq!(detail.evidence.len(), summary.evidence_count);
    assert_eq!(detail.event_batches.len(), summary.event_batch_count);
    assert_eq!(detail.transcripts.len(), summary.transcript_count);
    assert_eq!(detail.tools.len(), summary.tool_count);

    assert_eq!(
        pager
            .detail(
                session,
                jj_store_wal::publication::AgentSessionId::new([99; 16])
            )
            .await
            .unwrap_err(),
        Error::NotFound
    );
}

#[tokio::test]
async fn session_detail_resolves_bounded_messages_and_final_tool_snapshots() {
    let fixture = fixture().await;
    let request = fixture
        .session
        .prepare_agent_capture(
            m::RequestId::new([103; 16]),
            m::PublicationId::new([104; 16]),
            &NormalizerProfileV1::cursor_3_22_7(),
            &rich_cursor_batch(),
            b"accepted rich cursor activity".to_vec(),
        )
        .await
        .unwrap();
    assert!(matches!(
        fixture.session.publish_agent_delta(&request).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    let page = pager.begin(fixture.session.clone(), 10).await.unwrap();
    let detail = pager
        .detail(fixture.session.clone(), page.sessions[0].session_id)
        .await
        .unwrap();

    assert_eq!(
        detail.session.first_activity_at_millis,
        Some(1_725_000_000_000)
    );
    assert_eq!(
        detail.session.last_activity_at_millis,
        Some(1_725_000_000_010)
    );
    assert!(!detail.activity_truncated);
    assert!(detail.activity.iter().any(|item| matches!(
        item,
        AgentActivity::Message { role: MessageRole::User, content, .. }
            if content.content == "Update the parser"
    )));
    assert!(detail.activity.iter().any(|item| matches!(
        item,
        AgentActivity::Message { role: MessageRole::Assistant, content, .. }
            if content.content == "Updated the parser."
    )));
    let reactions = jj_store_service::collab_reads::PinnedCollabRead::open(
        fixture.session.clone(),
        Arc::new(jj_store_service::collab_reads::RepositoryReactionActors(
            REPOSITORY,
        )),
    )
    .await
    .unwrap();
    for item in &detail.activity {
        if let AgentActivity::Message {
            event_id,
            message_id,
            reaction_target,
            ..
        } = item
        {
            let Some(jj_collab::ReactionTargetV1::AgentMessage {
                session_id,
                event_id: target_event,
                message_id: target_message,
                batch,
                content,
            }) = reaction_target
            else {
                panic!("missing exact message witness")
            };
            let read = reactions
                .reactions(
                    fixture.session.clone(),
                    reaction_target.as_ref().unwrap(),
                    None,
                    10,
                )
                .await
                .unwrap();
            assert_eq!(read.target, *reaction_target.as_ref().unwrap());
            assert!(read.reactions.is_empty());
            assert_eq!(*session_id, detail.session.session_id);
            assert_eq!(target_event, event_id);
            assert_eq!(target_message, message_id);
            assert_eq!(
                batch.kind,
                jj_store_wal::publication::AgentObjectKind::EventBatch
            );
            assert_eq!(
                content.kind,
                jj_store_wal::publication::AgentObjectKind::Content
            );
        }
    }
    let tool = detail
        .activity
        .iter()
        .find_map(|item| match item {
            AgentActivity::Tool { tool, .. } if tool.kind == ToolKind::Edit => Some(tool),
            _ => None,
        })
        .unwrap();
    assert_eq!(tool.state, ToolState::Succeeded);
    assert!(tool.input.is_some());
    assert!(tool.output.is_some());
    assert_eq!(tool.observed_paths, ["<workspace>/src/lib.rs"]);
}

#[tokio::test]
async fn session_detail_marks_activity_beyond_the_event_bound_truncated() {
    let fixture = fixture().await;
    let session_id = AgentSessionId::new([111; 16]);
    let evidence = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::ObservationBatch(batch(
            112,
            "bounded-session",
        )))
        .await
        .unwrap();
    let event = |index: u32| {
        let mut bytes = [0; 16];
        bytes[12..].copy_from_slice(&index.to_be_bytes());
        AgentEventV1::SessionStateChanged {
            event_id: AgentEventId::new(bytes),
            session_id,
            state: SessionState::Active,
        }
    };
    let first = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::EventBatch(AgentEventBatchV1 {
            schema_version: 1,
            normalizer_version: "bounded-test/v1".into(),
            evidence,
            events: (0..4_096).map(event).collect(),
        }))
        .await
        .unwrap();
    let overflow = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::EventBatch(AgentEventBatchV1 {
            schema_version: 1,
            normalizer_version: "bounded-test/v1".into(),
            evidence,
            events: vec![event(4_096)],
        }))
        .await
        .unwrap();
    let snapshot = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::SessionSnapshot(AgentSessionSnapshotV1 {
            schema_version: 1,
            session_id,
            integration: IntegrationId::Codex,
            state: SessionState::Active,
            parent_session_id: None,
            evidence: vec![evidence],
            event_batches: vec![first, overflow],
            transcripts: vec![],
            tools: BTreeMap::new(),
            coverage: CoverageV1 {
                completeness: CompletenessV1::Partial,
                gaps: 0,
                redacted_fields: 0,
                truncated: false,
            },
        }))
        .await
        .unwrap();
    let request = AgentDeltaRequest::new(
        PRINCIPAL,
        m::RequestId::new([113; 16]),
        m::PublicationId::new([114; 16]),
        AgentDeltaV1 {
            schema_version: 1,
            edits: vec![AgentCatalogEditV1 {
                catalog: AgentCatalogKind::Sessions,
                key: AgentCatalogKey::derive("session", session_id.as_bytes()).unwrap(),
                expected: None,
                replacement: Some(snapshot),
            }],
        },
        b"published bounded activity fixture".to_vec(),
    )
    .unwrap();
    assert!(matches!(
        fixture.session.publish_agent_delta(&request).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    let detail = pager.detail(fixture.session, session_id).await.unwrap();

    assert_eq!(detail.activity.len(), 4_096);
    assert!(detail.activity_truncated);
}

async fn publish_transcript(
    fixture: &Fixture,
) -> (
    AgentSessionId,
    jj_store_wal::publication::AgentObjectId,
    SourceInstanceId,
) {
    let source = SourceInstanceId::new([81; 16]);
    let session_id = AgentSessionId::new([82; 16]);
    let first = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::TranscriptChunk(TranscriptChunkV1 {
            schema_version: 1,
            source_instance_id: source,
            source_generation: 4,
            sequence: 10,
            payload: SanitizedPayloadV1::new(SanitizedMediaType::Text, b"first\n").unwrap(),
        }))
        .await
        .unwrap();
    let second = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::TranscriptChunk(TranscriptChunkV1 {
            schema_version: 1,
            source_instance_id: source,
            source_generation: 4,
            sequence: 11,
            payload: SanitizedPayloadV1::new(SanitizedMediaType::Text, b"second\n").unwrap(),
        }))
        .await
        .unwrap();
    let tail = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::TranscriptPage(TranscriptPageV1 {
            schema_version: 1,
            source_instance_id: source,
            source_generation: 4,
            chunks: vec![second],
            next: None,
        }))
        .await
        .unwrap();
    let head = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::TranscriptPage(TranscriptPageV1 {
            schema_version: 1,
            source_instance_id: source,
            source_generation: 4,
            chunks: vec![first],
            next: Some(tail),
        }))
        .await
        .unwrap();
    let transcript = fixture
        .staging
        .stage_agent_object(&AgentObjectV1::TranscriptManifest(TranscriptManifestV1 {
            schema_version: 1,
            session_id,
            streams: vec![TranscriptStreamV1 {
                source_instance_id: source,
                source_generation: 4,
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
    let snapshot = fixture
        .staging
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
    let delta = AgentDeltaV1 {
        schema_version: 1,
        edits: vec![AgentCatalogEditV1 {
            catalog: AgentCatalogKind::Sessions,
            key: AgentCatalogKey::derive("session", session_id.as_bytes()).unwrap(),
            expected: None,
            replacement: Some(snapshot),
        }],
    };
    let request = AgentDeltaRequest::new(
        PRINCIPAL,
        m::RequestId::new([83; 16]),
        m::PublicationId::new([84; 16]),
        delta,
        b"published transcript fixture".to_vec(),
    )
    .unwrap();
    assert!(matches!(
        fixture.session.publish_agent_delta(&request).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    (session_id, transcript.id, source)
}

#[tokio::test]
async fn transcript_pages_retain_exact_root_and_validate_stream_cursor() {
    let fixture = fixture().await;
    let (session_id, transcript_id, source_instance_id) = publish_transcript(&fixture).await;
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    let request = AgentTranscriptRequest {
        session_id,
        transcript_id,
        source_instance_id,
        source_generation: 4,
        limit: 1,
        max_bytes: 1024,
    };
    let detail = pager
        .detail(fixture.session.clone(), session_id)
        .await
        .unwrap();
    assert_eq!(detail.transcripts.len(), 1);
    assert_eq!(detail.transcripts[0].transcript_id, transcript_id);
    assert_eq!(detail.transcripts[0].source_format, "codex-jsonl");

    let first = pager
        .begin_transcript(fixture.session.clone(), request)
        .await
        .unwrap();
    assert_eq!(first.chunks[0].sequence, 10);
    assert_eq!(first.chunks[0].content, "first\n");
    let cursor = first.next_cursor.clone().unwrap();

    publish(&fixture.session, 91, "later-session").await;
    let second = pager
        .resume_transcript(fixture.session.clone(), &cursor, request)
        .await
        .unwrap();
    assert_eq!(second.snapshot, first.snapshot);
    assert_eq!(second.chunks[0].sequence, 11);
    assert_eq!(second.chunks[0].content, "second\n");
    assert!(second.next_cursor.is_none());

    assert_eq!(
        pager
            .resume_transcript(
                fixture.session,
                &cursor,
                AgentTranscriptRequest {
                    max_bytes: 2048,
                    ..request
                },
            )
            .await
            .unwrap_err(),
        Error::InvalidCursor
    );
}
