use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectRef,
    AgentObjectV1, AgentRelationPageV1, AgentRelationSubjectV1, AgentSessionId,
    AgentSessionSnapshotV1, AssociationGranularity, AssociationId, AssociationRelation,
    CaptureBatchId, CodeAnchorV1, CodeAssociationV1, CompletenessV1, CoverageV1, EvidenceMethod,
    IntegrationId, NormalizedActivityRecordV1, NormalizerProfileV1, ObservationId,
    SanitizedMediaType, SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
    SanitizedRepresentation, SessionState, SourceInstanceId, SourceKind, SourceTrust,
    TranscriptChunkV1, TranscriptManifestV1, TranscriptStreamV1, TranscriptTailPageV2,
};
use jj_store_model::{EpochId, ObjectKey, OperationId, RepositoryId};
use jj_store_object::memory::MemoryStore;
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication::{AgentDeltaRequest, Coordinator, Limits, PublicationGuard, Request, Snapshot},
};
use std::{collections::BTreeMap, collections::BTreeSet, sync::Arc};

const REPOSITORY: RepositoryId = RepositoryId::new([1; 16]);

#[path = "agent_catalog/common_aliases.rs"]
mod common_aliases;
#[path = "agent_catalog/native_results.rs"]
mod native_results;
#[path = "agent_catalog/opencode_aliases.rs"]
mod opencode_aliases;
#[path = "agent_catalog/selected_tail.rs"]
mod selected_tail;

#[tokio::test]
async fn native_desktop_messages_publish_sanitized_evidence_without_asserting_execution() {
    let coordinator = fixture().await;
    let selected = coordinator.read().await.unwrap();
    let old = coordinator
        .agent_store(selected.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let profile = NormalizerProfileV1::codex_desktop_0_159_2();
    let mut batch = single_capture(
        139,
        1,
        serde_json::json!({
            "capture_format":"codex-jsonl/v1","product_version":"0.159.2",
            "session_id":"00000000-0000-0000-0000-000000000001",
            "record":{"type":"response_item","payload":{"type":"message","id":"native-message","role":"assistant","content":[{"type":"output_text","text":"Recorded response"}]}}
        }),
    );
    batch.observations[0].source_kind = SourceKind::Transcript;
    batch.observations[0].source_trust = SourceTrust::LocalTranscript;
    let delta = coordinator
        .stage_agent_capture(&old, &profile, &batch, 123)
        .await
        .unwrap();
    let session = delta
        .edits
        .iter()
        .find(|e| e.catalog == AgentCatalogKind::Sessions)
        .unwrap();
    let key = session.key;
    publish_transcript_edits(&coordinator, 139, delta.edits).await;
    assert!(
        old.lookup(AgentCatalogKind::Sessions, key)
            .await
            .unwrap()
            .is_none()
    );
    let selected = coordinator.read().await.unwrap();
    let store = coordinator
        .agent_store(selected.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let reference = store
        .lookup(AgentCatalogKind::Sessions, key)
        .await
        .unwrap()
        .unwrap();
    let AgentObjectV1::SessionSnapshot(session) = store.read_object(reference).await.unwrap()
    else {
        panic!("session")
    };
    assert_eq!(session.state, SessionState::Discovered);
    assert_eq!(session.evidence.len(), 1);
    assert_eq!(session.transcripts.len(), 1);
    assert!(session.tools.is_empty());
    let range = store
        .transcript_range(
            session.transcripts[0],
            batch.source_instance_id,
            1,
            None,
            10,
            1024,
        )
        .await
        .unwrap();
    assert_eq!(range.chunks.len(), 1);
    assert_eq!(range.chunks[0].payload.bytes(), b"Recorded response");
    let AgentObjectV1::ObservationBatch(evidence) =
        store.read_object(session.evidence[0]).await.unwrap()
    else {
        panic!("evidence")
    };
    assert_eq!(evidence, batch);
}

struct Source(SourceScope);
#[async_trait::async_trait]
impl DependencySource for Source {
    fn scope(&self) -> SourceScope {
        self.0
    }
    async fn load_verified(&self, key: ObjectKey, _: u64) -> wal::inventory::Result<VerifiedNode> {
        let expected = ObjectKey::Operation(OperationId::new([0; 64]));
        if key != expected {
            return Err(wal::inventory::InventoryError::Source(
                "unexpected fixture object".into(),
            ));
        }
        Ok(VerifiedNode {
            key,
            representation: Representation::Virtual,
            dependencies: BTreeSet::new(),
        })
    }
}

struct Permit;
#[async_trait::async_trait]
impl PublicationGuard for Permit {
    async fn authorize(
        &self,
        _: &wal::publication::ManifestSnapshot,
        _: &Request,
    ) -> wal::Result<()> {
        Ok(())
    }
    async fn validate_transition(
        &self,
        _: &wal::publication::ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize_domain(
        &self,
        _: &wal::publication::ManifestSnapshot,
        _: &wal::publication::DomainRequest,
    ) -> wal::Result<()> {
        Ok(())
    }
}

async fn fixture() -> Coordinator {
    let coordinator = Coordinator::new(
        Arc::new(MemoryStore::default()),
        REPOSITORY,
        Arc::new(Source(SourceScope::new(REPOSITORY))),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap();
    coordinator
        .initialize(EpochId::new([2; 16]), Vec::new())
        .await
        .unwrap();
    coordinator
}

fn codex_batch() -> SanitizedObservationBatchV1 {
    let values: Vec<serde_json::Value> = serde_json::from_slice(include_bytes!(
        "../../jj-agent/tests/fixtures/codex-0.156.1-hooks.json"
    ))
    .unwrap();
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([70; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([71; 16]),
        sanitizer_version: "agent-redaction@0.1.0".into(),
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

fn codex_followup_batch() -> SanitizedObservationBatchV1 {
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([75; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([71; 16]),
        sanitizer_version: "agent-redaction@0.1.0".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![SanitizedObservationV1 {
            observation_id: ObservationId::new([75; 16]),
            source_sequence: 11,
            occurred_at_millis: Some(1_725_000_000_011),
            source_kind: SourceKind::Hook,
            source_trust: SourceTrust::LocalUserHook,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                br#"{"hook_event_name":"UserPromptSubmit","session_id":"thr_demo","turn_id":"turn_2","prompt":"Explain the failure"}"#,
            )
            .unwrap(),
        }],
    }
}

fn session(byte: u8) -> AgentObjectV1 {
    AgentObjectV1::SessionSnapshot(AgentSessionSnapshotV1 {
        schema_version: 1,
        session_id: AgentSessionId::new([byte; 16]),
        integration: IntegrationId::Codex,
        state: SessionState::Active,
        parent_session_id: None,
        evidence: Vec::new(),
        event_batches: Vec::new(),
        transcripts: Vec::new(),
        tools: BTreeMap::new(),
        coverage: CoverageV1 {
            completeness: CompletenessV1::Complete,
            gaps: 0,
            redacted_fields: 0,
            truncated: false,
        },
    })
}

fn edit(
    key: AgentCatalogKey,
    expected: Option<AgentObjectRef>,
    replacement: AgentObjectRef,
) -> AgentCatalogEditV1 {
    AgentCatalogEditV1 {
        catalog: AgentCatalogKind::Sessions,
        key,
        expected,
        replacement: Some(replacement),
    }
}

#[tokio::test]
async fn normalized_capture_stages_one_root_pinned_publication_delta() {
    let coordinator = fixture().await;
    let selected = coordinator.read().await.unwrap();
    let root = selected.manifest().agent_root.unwrap();
    let pinned = coordinator.agent_store(root).await.unwrap();
    let batch = codex_batch();
    let profile = NormalizerProfileV1::codex_0_156_1();
    let normalized = jj_agent::normalize_batch(&profile, &batch).unwrap();
    let session_id = normalized
        .records
        .iter()
        .find_map(|record| match record {
            NormalizedActivityRecordV1::SessionState(value) => Some(value.session_id),
            _ => None,
        })
        .unwrap();

    let delta = coordinator
        .stage_agent_capture(&pinned, &profile, &batch, 1_725_000_000_000)
        .await
        .unwrap();
    assert!(
        delta
            .edits
            .iter()
            .any(|edit| edit.catalog == AgentCatalogKind::Sessions)
    );
    assert!(
        delta
            .edits
            .iter()
            .any(|edit| edit.catalog == AgentCatalogKind::Aliases)
    );
    assert!(
        delta
            .edits
            .iter()
            .any(|edit| edit.catalog == AgentCatalogKind::Sources)
    );
    assert!(
        delta
            .edits
            .iter()
            .any(|edit| edit.catalog == AgentCatalogKind::Projections)
    );

    let request = AgentDeltaRequest::new(
        jj_store_model::PrincipalId::new([72; 16]),
        jj_store_model::RequestId::new([73; 16]),
        jj_store_model::PublicationId::new([74; 16]),
        delta,
        b"captured codex batch".to_vec(),
    )
    .unwrap();
    let first = coordinator.publish_agent_delta(&request).await.unwrap();
    let current = coordinator.read().await.unwrap();
    let store = coordinator
        .agent_store(current.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let key = AgentCatalogKey::derive("session", session_id.as_bytes()).unwrap();
    let reference = store
        .lookup(AgentCatalogKind::Sessions, key)
        .await
        .unwrap()
        .unwrap();
    let AgentObjectV1::SessionSnapshot(session) = store.read_object(reference).await.unwrap()
    else {
        panic!("expected session snapshot");
    };
    assert_eq!(session.session_id, session_id);
    assert_eq!(session.evidence.len(), 1);
    assert_eq!(session.event_batches.len(), 1);
    assert_eq!(session.transcripts.len(), 1);
    assert_eq!(session.tools.len(), 3);
    let first_transcript = session.transcripts[0];
    let first_range = store
        .transcript_range(
            first_transcript,
            batch.source_instance_id,
            1,
            None,
            10,
            1024,
        )
        .await
        .unwrap();
    assert_eq!(first_range.chunks.len(), 1);
    assert_eq!(first_range.chunks[0].sequence, 2);
    assert_eq!(first_range.chunks[0].payload.bytes(), b"Update the parser");
    assert_eq!(
        coordinator.publish_agent_delta(&request).await.unwrap(),
        first
    );

    let followup = codex_followup_batch();
    let delta = coordinator
        .stage_agent_capture(&store, &profile, &followup, 1_725_000_000_011)
        .await
        .unwrap();
    let request = AgentDeltaRequest::new(
        jj_store_model::PrincipalId::new([76; 16]),
        jj_store_model::RequestId::new([77; 16]),
        jj_store_model::PublicationId::new([78; 16]),
        delta,
        b"captured codex followup".to_vec(),
    )
    .unwrap();
    coordinator.publish_agent_delta(&request).await.unwrap();
    let current = coordinator.read().await.unwrap();
    let store = coordinator
        .agent_store(current.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let reference = store
        .lookup(AgentCatalogKind::Sessions, key)
        .await
        .unwrap()
        .unwrap();
    let AgentObjectV1::SessionSnapshot(session) = store.read_object(reference).await.unwrap()
    else {
        panic!("expected session snapshot");
    };
    assert_eq!(session.evidence.len(), 2);
    assert_eq!(session.event_batches.len(), 2);
    assert_eq!(session.transcripts.len(), 1);
    assert_ne!(session.transcripts[0], first_transcript);
    let range = store
        .transcript_range(
            session.transcripts[0],
            batch.source_instance_id,
            1,
            None,
            10,
            1024,
        )
        .await
        .unwrap();
    assert_eq!(
        range
            .chunks
            .iter()
            .map(|chunk| (chunk.sequence, chunk.payload.bytes()))
            .collect::<Vec<_>>(),
        vec![
            (2, b"Update the parser".as_slice()),
            (11, b"Explain the failure".as_slice())
        ]
    );
}

#[tokio::test]
async fn persistent_catalog_splits_and_keeps_old_root_pinned() {
    let coordinator = fixture().await;
    let initial = coordinator.read().await.unwrap();
    let initial_root = initial.manifest().agent_root.unwrap();
    let pinned = coordinator.agent_store(initial_root).await.unwrap();
    let mut selections = Vec::new();
    let mut edits = Vec::new();
    for byte in 100..140 {
        let value = coordinator
            .stage_agent_object(&session(byte))
            .await
            .unwrap();
        let key = AgentCatalogKey::derive("session", &[byte; 16]).unwrap();
        selections.push((key, value));
        edits.push(edit(key, None, value));
    }
    let first_root = coordinator
        .stage_agent_delta(
            &pinned,
            &AgentDeltaV1 {
                schema_version: 1,
                edits,
            },
        )
        .await
        .unwrap();
    let first = coordinator.agent_store(first_root).await.unwrap();
    for (key, value) in &selections {
        assert_eq!(
            first
                .lookup(AgentCatalogKind::Sessions, *key)
                .await
                .unwrap(),
            Some(*value)
        );
    }
    let first_page = first.list_sessions(None, 10).await.unwrap();
    assert_eq!(first_page.selections.len(), 10);
    let second_page = first
        .list_sessions(first_page.next_after, 10)
        .await
        .unwrap();
    assert_eq!(second_page.selections.len(), 10);
    assert!(
        first_page
            .selections
            .iter()
            .all(|entry| !second_page.selections.contains(entry))
    );

    let replacement = coordinator.stage_agent_object(&session(200)).await.unwrap();
    let (key, previous) = selections[17];
    let second_root = coordinator
        .stage_agent_delta(
            &first,
            &AgentDeltaV1 {
                schema_version: 1,
                edits: vec![edit(key, Some(previous), replacement)],
            },
        )
        .await
        .unwrap();
    let second = coordinator.agent_store(second_root).await.unwrap();
    assert_eq!(
        first.lookup(AgentCatalogKind::Sessions, key).await.unwrap(),
        Some(previous)
    );
    assert_eq!(
        second
            .lookup(AgentCatalogKind::Sessions, key)
            .await
            .unwrap(),
        Some(replacement)
    );
    let (neighbor_key, neighbor) = selections[18];
    assert_eq!(
        second
            .lookup(AgentCatalogKind::Sessions, neighbor_key)
            .await
            .unwrap(),
        Some(neighbor)
    );
}

#[tokio::test]
async fn pinned_reads_page_catalog_relations_and_transcript_ranges() {
    let coordinator = fixture().await;
    let manifest = coordinator.read().await.unwrap();
    let base = coordinator
        .agent_store(manifest.manifest().agent_root.unwrap())
        .await
        .unwrap();

    let source_id = SourceInstanceId::new([21; 16]);
    let first_chunk = coordinator
        .stage_agent_object(&AgentObjectV1::TranscriptChunk(TranscriptChunkV1 {
            schema_version: 1,
            source_instance_id: source_id,
            source_generation: 4,
            sequence: 10,
            payload: SanitizedPayloadV1::new(SanitizedMediaType::Text, b"first\n").unwrap(),
        }))
        .await
        .unwrap();
    let second_chunk = coordinator
        .stage_agent_object(&AgentObjectV1::TranscriptChunk(TranscriptChunkV1 {
            schema_version: 1,
            source_instance_id: source_id,
            source_generation: 4,
            sequence: 11,
            payload: SanitizedPayloadV1::new(SanitizedMediaType::Text, b"second\n").unwrap(),
        }))
        .await
        .unwrap();
    let second_page = coordinator
        .stage_agent_object(&AgentObjectV1::TranscriptTailPage(TranscriptTailPageV2 {
            schema_version: 2,
            source_instance_id: source_id,
            source_generation: 4,
            chunks: vec![first_chunk],
            prefix: None,
        }))
        .await
        .unwrap();
    let chunk_page = coordinator
        .stage_agent_object(&AgentObjectV1::TranscriptTailPage(TranscriptTailPageV2 {
            schema_version: 2,
            source_instance_id: source_id,
            source_generation: 4,
            chunks: vec![second_chunk],
            prefix: Some(second_page),
        }))
        .await
        .unwrap();
    let transcript = coordinator
        .stage_agent_object(&AgentObjectV1::TranscriptManifest(TranscriptManifestV1 {
            schema_version: 1,
            session_id: AgentSessionId::new([22; 16]),
            streams: vec![TranscriptStreamV1 {
                source_instance_id: source_id,
                source_generation: 4,
                source_format: "codex-jsonl".into(),
                decoder_version: "codex-0.156.1/v1".into(),
                sanitizer_version: "agent-redaction@0.1.0".into(),
                representation: SanitizedRepresentation::SanitizedNative,
                ordered_chunk_root: chunk_page,
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
    let session_value = coordinator
        .stage_agent_object(&AgentObjectV1::SessionSnapshot(AgentSessionSnapshotV1 {
            schema_version: 1,
            session_id: AgentSessionId::new([22; 16]),
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

    let association = coordinator
        .stage_agent_object(&AgentObjectV1::Association(CodeAssociationV1 {
            schema_version: 1,
            association_id: AssociationId::new([23; 16]),
            checkpoint_id: None,
            session_id: AgentSessionId::new([22; 16]),
            turn_id: None,
            tool_call_ids: BTreeSet::new(),
            scope_id: None,
            relation: AssociationRelation::ObservedDuringInterval,
            before: CodeAnchorV1 {
                operations: BTreeSet::from([OperationId::new([0; 64])]),
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
                completeness: CompletenessV1::Partial,
                gaps: 1,
                redacted_fields: 0,
                truncated: false,
            },
            created_at_millis: 1,
            supersedes: None,
            correction_reason: None,
        }))
        .await
        .unwrap();
    let subject = AgentRelationSubjectV1::Session {
        session_id: AgentSessionId::new([22; 16]),
    };
    let relation = coordinator
        .stage_agent_object(&AgentObjectV1::RelationPage(AgentRelationPageV1 {
            schema_version: 1,
            subject: subject.clone(),
            selections: vec![association],
            next: None,
        }))
        .await
        .unwrap();
    let session_key = AgentCatalogKey::derive("session", &[22; 16]).unwrap();
    let root = coordinator
        .stage_agent_delta(
            &base,
            &AgentDeltaV1 {
                schema_version: 1,
                edits: vec![
                    edit(session_key, None, session_value),
                    AgentCatalogEditV1 {
                        catalog: AgentCatalogKind::Relations,
                        key: subject.catalog_key().unwrap(),
                        expected: None,
                        replacement: Some(relation),
                    },
                ],
            },
        )
        .await
        .unwrap();
    let pinned = coordinator.agent_store(root).await.unwrap();

    let sessions = pinned.list_sessions(None, 1).await.unwrap();
    assert_eq!(sessions.selections, vec![(session_key, session_value)]);
    assert_eq!(sessions.next_after, None);
    let relations = pinned.relations(&subject, None, 1).await.unwrap();
    assert_eq!(relations.selections, vec![association]);
    assert_eq!(relations.next, None);

    let first = pinned
        .transcript_range(transcript, source_id, 4, None, 1, 1024)
        .await
        .unwrap();
    assert_eq!(first.chunks[0].sequence, 10);
    assert_eq!(first.next_after_sequence, Some(10));
    let second = pinned
        .transcript_range(transcript, source_id, 4, first.next_after_sequence, 1, 1024)
        .await
        .unwrap();
    assert_eq!(second.chunks[0].sequence, 11);
    assert_eq!(second.next_after_sequence, None);
}

fn single_capture(
    id: u8,
    sequence: u64,
    payload: serde_json::Value,
) -> SanitizedObservationBatchV1 {
    let mut batch = codex_batch();
    batch.batch_id = CaptureBatchId::new([id; 16]);
    batch.observations = vec![SanitizedObservationV1 {
        observation_id: ObservationId::new([id; 16]),
        source_sequence: sequence,
        occurred_at_millis: Some(sequence as i64),
        source_kind: SourceKind::Hook,
        source_trust: SourceTrust::LocalUserHook,
        payload: SanitizedPayloadV1::new(
            SanitizedMediaType::Json,
            &serde_json::to_vec(&payload).unwrap(),
        )
        .unwrap(),
    }];
    batch
}

async fn publish_single_capture(coordinator: &Coordinator, batch: &SanitizedObservationBatchV1) {
    let current = coordinator.read().await.unwrap();
    let base = coordinator
        .agent_store(current.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let delta = coordinator
        .stage_agent_capture(&base, &NormalizerProfileV1::codex_0_156_1(), batch, 123)
        .await
        .unwrap();
    let id = batch.batch_id.as_bytes()[0];
    let request = AgentDeltaRequest::new(
        jj_store_model::PrincipalId::new([72; 16]),
        jj_store_model::RequestId::new([id; 16]),
        jj_store_model::PublicationId::new([id; 16]),
        delta,
        b"capture".to_vec(),
    )
    .unwrap();
    let accepted = coordinator.publish_agent_delta(&request).await.unwrap();
    assert_eq!(
        coordinator.publish_agent_delta(&request).await.unwrap(),
        accepted
    );
}

#[tokio::test]
async fn diagnostic_only_capture_retains_evidence_and_allows_later_source_progress() {
    let coordinator = fixture().await;
    let omitted = single_capture(81, 1, serde_json::json!({"status":"omitted"}));
    publish_single_capture(&coordinator, &omitted).await;
    let selected = coordinator.read().await.unwrap();
    let diagnostic_root = selected.manifest().agent_root.unwrap();
    let store = coordinator.agent_store(diagnostic_root).await.unwrap();
    assert!(
        store
            .list_catalog(AgentCatalogKind::Sessions, None, 10)
            .await
            .unwrap()
            .selections
            .is_empty()
    );
    let projections = store
        .list_catalog(AgentCatalogKind::Projections, None, 10)
        .await
        .unwrap();
    let AgentObjectV1::ProjectionCatalog(projection) = store
        .read_object(projections.selections[0].1)
        .await
        .unwrap()
    else {
        panic!("projection");
    };
    let AgentObjectV1::EventBatch(events) = store
        .read_object(projection.event_batches[0])
        .await
        .unwrap()
    else {
        panic!("events");
    };
    assert!(events.events.is_empty());
    assert_eq!(
        store.read_object(events.evidence).await.unwrap(),
        AgentObjectV1::ObservationBatch(omitted)
    );
    let unsupported = single_capture(
        82,
        2,
        serde_json::json!({"hook_event_name":"NewNativeEvent", "session_id":"capture_session"}),
    );
    publish_single_capture(&coordinator, &unsupported).await;
    publish_single_capture(
        &coordinator,
        &single_capture(
            83,
            3,
            serde_json::json!({"hook_event_name":"SessionStart", "session_id":"capture_session"}),
        ),
    )
    .await;
    let selected = coordinator.read().await.unwrap();
    let store = coordinator
        .agent_store(selected.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let sources = store
        .list_catalog(AgentCatalogKind::Sources, None, 10)
        .await
        .unwrap();
    let AgentObjectV1::SourceState(source) =
        store.read_object(sources.selections[0].1).await.unwrap()
    else {
        panic!("source");
    };
    assert_eq!(source.receipts.len(), 3);
    assert_eq!(source.coverage.gaps, 2);
    assert_eq!(
        source.position,
        jj_agent::SourcePositionV1::NativeSequence { sequence: 3 }
    );
    assert_eq!(
        store
            .list_catalog(AgentCatalogKind::Sessions, None, 10)
            .await
            .unwrap()
            .selections
            .len(),
        1
    );
}

#[tokio::test]
async fn separately_published_tool_updates_preserve_content_terminal_state_and_conflict_coverage() {
    let coordinator = fixture().await;
    for (id, sequence, payload) in [
        (
            84,
            1,
            serde_json::json!({"hook_event_name":"PostToolUse", "session_id":"capture_session", "tool_use_id":"call_1", "tool_name":"Bash", "tool_response":{"exit_code":1,"output":"failed output"}}),
        ),
        (
            85,
            2,
            serde_json::json!({"hook_event_name":"PreToolUse", "session_id":"capture_session", "tool_use_id":"call_1", "tool_name":"Bash", "tool_input":{"command":"test"}}),
        ),
        (
            86,
            3,
            serde_json::json!({"hook_event_name":"PostToolUse", "session_id":"capture_session", "tool_use_id":"call_1", "tool_name":"Bash", "tool_response":{"exit_code":0,"output":"contradictory output"}}),
        ),
    ] {
        publish_single_capture(&coordinator, &single_capture(id, sequence, payload)).await;
    }
    let selected = coordinator.read().await.unwrap();
    let store = coordinator
        .agent_store(selected.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let sessions = store
        .list_catalog(AgentCatalogKind::Sessions, None, 10)
        .await
        .unwrap();
    let AgentObjectV1::SessionSnapshot(session) =
        store.read_object(sessions.selections[0].1).await.unwrap()
    else {
        panic!("session");
    };
    let AgentObjectV1::ToolSnapshot(tool) = store
        .read_object(*session.tools.values().next().unwrap())
        .await
        .unwrap()
    else {
        panic!("tool");
    };
    assert_eq!(tool.state, jj_agent::ToolState::Failed);
    assert_eq!(tool.evidence.len(), 3);
    assert_eq!(session.coverage.gaps, 1);
    let AgentObjectV1::Content(input) = store.read_object(tool.input.unwrap()).await.unwrap()
    else {
        panic!("input");
    };
    let AgentObjectV1::Content(output) = store.read_object(tool.output.unwrap()).await.unwrap()
    else {
        panic!("output");
    };
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(input.payload.bytes()).unwrap(),
        serde_json::json!({"command":"test"})
    );
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(output.payload.bytes()).unwrap(),
        serde_json::json!({"exit_code":1,"output":"failed output"})
    );
}

#[tokio::test]
async fn diagnostic_only_capture_rejects_unsupported_normalizer_profile_without_selection() {
    let coordinator = fixture().await;
    let current = coordinator.read().await.unwrap();
    let before = current.manifest().clone();
    let base = coordinator
        .agent_store(before.agent_root.unwrap())
        .await
        .unwrap();
    let batch = single_capture(87, 1, serde_json::json!({"status":"omitted"}));
    let mut profile = NormalizerProfileV1::codex_0_156_1();
    profile.product_version = "unqualified".into();
    assert!(
        coordinator
            .stage_agent_capture(&base, &profile, &batch, 123)
            .await
            .is_err()
    );
    assert_eq!(coordinator.read().await.unwrap().manifest(), &before);
    publish_single_capture(&coordinator, &batch).await;
}

struct CountedAgentSnapshot<'a> {
    inner: &'a jj_store_wal::publication::AgentStoreSnapshot,
    pages: std::sync::atomic::AtomicUsize,
    chunks: std::sync::atomic::AtomicUsize,
}
#[async_trait::async_trait]
impl jj_store_wal::publication::AgentSnapshotRead for CountedAgentSnapshot<'_> {
    fn repository(&self) -> RepositoryId {
        self.inner.repository_id()
    }
    fn root(&self) -> &jj_agent::AgentRootV1 {
        self.inner.root()
    }
    fn transcript_boundary(
        &self,
        root: AgentObjectRef,
    ) -> Option<jj_store_wal::publication::TranscriptAppendBoundary> {
        self.inner.transcript_proofs().boundary(root)
    }
    async fn lookup(
        &self,
        kind: AgentCatalogKind,
        key: AgentCatalogKey,
    ) -> jj_store_core::Result<Option<AgentObjectRef>> {
        self.inner.lookup(kind, key).await.map_err(Into::into)
    }
    async fn read_object(&self, reference: AgentObjectRef) -> jj_store_core::Result<AgentObjectV1> {
        use std::sync::atomic::Ordering::Relaxed;
        match reference.kind {
            jj_agent::AgentObjectKind::TranscriptPage => {
                self.pages.fetch_add(1, Relaxed);
            }
            jj_agent::AgentObjectKind::TranscriptChunk => {
                self.chunks.fetch_add(1, Relaxed);
            }
            _ => {}
        }
        self.inner.read_object(reference).await.map_err(Into::into)
    }
}

#[tokio::test]
async fn transcript_append_reads_only_selected_tail_and_keeps_old_manifest_reconstructible() {
    use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
    let coordinator = fixture().await;
    let mut initial = codex_batch();
    initial.batch_id = CaptureBatchId::new([90; 16]);
    let template = initial.observations[0].clone();
    initial.observations = (1u64..=400).map(|sequence| {
        let mut value = template.clone();
        let mut id = [0; 16]; id[..8].copy_from_slice(&sequence.to_be_bytes());
        value.observation_id = ObservationId::new(id);
        value.source_sequence = sequence;
        value.payload = SanitizedPayloadV1::new(SanitizedMediaType::Json,
            &serde_json::to_vec(&serde_json::json!({"hook_event_name":"UserPromptSubmit", "session_id":"thr_demo", "turn_id":format!("turn_{sequence}"), "prompt":format!("message {sequence}")})).unwrap()).unwrap();
        value
    }).collect();
    for (part, observations) in initial.observations.chunks(200).enumerate() {
        let mut accepted = initial.clone();
        accepted.batch_id = CaptureBatchId::new([90 + part as u8 * 2; 16]);
        accepted.observations = observations.to_vec();
        publish_single_capture(&coordinator, &accepted).await;
    }
    let selected = coordinator.read().await.unwrap();
    let base = coordinator
        .agent_store(selected.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let sessions = base.list_sessions(None, 10).await.unwrap();
    let AgentObjectV1::SessionSnapshot(old_session) =
        base.read_object(sessions.selections[0].1).await.unwrap()
    else {
        panic!("session")
    };
    let old_manifest = old_session.transcripts[0];
    let counted = CountedAgentSnapshot {
        inner: &base,
        pages: AtomicUsize::new(0),
        chunks: AtomicUsize::new(0),
    };
    let next = single_capture(
        91,
        401,
        serde_json::json!({"hook_event_name":"UserPromptSubmit", "session_id":"thr_demo", "prompt":"new message"}),
    );
    let delta = jj_store_wal::publication::AgentProjector::new(&coordinator)
        .stage_agent_capture(&counted, &NormalizerProfileV1::codex_0_156_1(), &next, 124)
        .await
        .unwrap();
    assert!(
        counted.pages.load(Relaxed) <= 1 && counted.chunks.load(Relaxed) <= 1,
        "append read {} pages and {} old chunks",
        counted.pages.load(Relaxed),
        counted.chunks.load(Relaxed)
    );
    let request = AgentDeltaRequest::new(
        jj_store_model::PrincipalId::new([72; 16]),
        jj_store_model::RequestId::new([91; 16]),
        jj_store_model::PublicationId::new([91; 16]),
        delta,
        b"bounded append".to_vec(),
    )
    .unwrap();
    coordinator.publish_agent_delta(&request).await.unwrap();
    let old = base
        .transcript_range(
            old_manifest,
            initial.source_instance_id,
            1,
            Some(398),
            10,
            1024,
        )
        .await
        .unwrap();
    assert_eq!(
        old.chunks.iter().map(|c| c.sequence).collect::<Vec<_>>(),
        vec![399, 400]
    );
    let selected = coordinator.read().await.unwrap();
    let current = coordinator
        .agent_store(selected.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let sessions = current.list_sessions(None, 10).await.unwrap();
    let AgentObjectV1::SessionSnapshot(session) =
        current.read_object(sessions.selections[0].1).await.unwrap()
    else {
        panic!("session")
    };
    let range = current
        .transcript_range(
            session.transcripts[0],
            initial.source_instance_id,
            1,
            Some(398),
            10,
            1024,
        )
        .await
        .unwrap();
    assert_eq!(
        range.chunks.iter().map(|c| c.sequence).collect::<Vec<_>>(),
        vec![399, 400, 401]
    );
}

async fn publish_transcript_edits(
    coordinator: &Coordinator,
    id: u8,
    edits: Vec<AgentCatalogEditV1>,
) {
    let request = AgentDeltaRequest::new(
        jj_store_model::PrincipalId::new([72; 16]),
        jj_store_model::RequestId::new([id; 16]),
        jj_store_model::PublicationId::new([id; 16]),
        jj_agent::AgentDeltaV1 {
            schema_version: 1,
            edits,
        },
        b"current transcript fixture".to_vec(),
    )
    .unwrap();
    coordinator.publish_agent_delta(&request).await.unwrap();
}

#[tokio::test]
async fn tail_append_preserves_other_streams_and_reads_only_selected_tail() {
    use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
    let coordinator = fixture().await;
    let initial = codex_batch();
    publish_single_capture(&coordinator, &initial).await;
    let selected = coordinator.read().await.unwrap();
    let store = coordinator
        .agent_store(selected.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let session_entry = store.list_sessions(None, 10).await.unwrap().selections[0];
    let AgentObjectV1::SessionSnapshot(mut session) =
        store.read_object(session_entry.1).await.unwrap()
    else {
        panic!("session")
    };
    let AgentObjectV1::TranscriptManifest(mut manifest) =
        store.read_object(session.transcripts[0]).await.unwrap()
    else {
        panic!("manifest")
    };
    let other_source = jj_agent::SourceInstanceId::new([125; 16]);
    let payload =
        SanitizedPayloadV1::new(SanitizedMediaType::Json, br#"{"message":"other"}"#).unwrap();
    let other_chunk = coordinator
        .stage_agent_object(&AgentObjectV1::TranscriptChunk(TranscriptChunkV1 {
            schema_version: 1,
            source_instance_id: other_source,
            source_generation: 1,
            sequence: 1,
            payload: payload.clone(),
        }))
        .await
        .unwrap();
    let other_page = coordinator
        .stage_agent_object(&AgentObjectV1::TranscriptTailPage(TranscriptTailPageV2 {
            schema_version: 2,
            source_instance_id: other_source,
            source_generation: 1,
            chunks: vec![other_chunk],
            prefix: None,
        }))
        .await
        .unwrap();
    let mut other = manifest.streams[0].clone();
    other.source_instance_id = other_source;
    other.ordered_chunk_root = other_page;
    other.logical_byte_length = payload.bytes().len() as u64;
    other.record_count = 1;
    other.gaps = 0;
    manifest.streams.push(other);
    session.transcripts[0] = coordinator
        .stage_agent_object(&AgentObjectV1::TranscriptManifest(manifest))
        .await
        .unwrap();
    let session_ref = coordinator
        .stage_agent_object(&AgentObjectV1::SessionSnapshot(session))
        .await
        .unwrap();
    publish_transcript_edits(
        &coordinator,
        126,
        vec![AgentCatalogEditV1 {
            catalog: AgentCatalogKind::Sessions,
            key: session_entry.0,
            expected: Some(session_entry.1),
            replacement: Some(session_ref),
        }],
    )
    .await;
    let selected = coordinator.read().await.unwrap();
    let base = coordinator
        .agent_store(selected.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let followup = codex_followup_batch();
    let counted = CountedAgentSnapshot {
        inner: &base,
        pages: AtomicUsize::new(0),
        chunks: AtomicUsize::new(0),
    };
    let delta = jj_store_wal::publication::AgentProjector::new(&coordinator)
        .stage_agent_capture(
            &counted,
            &NormalizerProfileV1::codex_0_156_1(),
            &followup,
            125,
        )
        .await
        .unwrap();
    assert_eq!(counted.pages.load(Relaxed), 1);
    assert_eq!(counted.chunks.load(Relaxed), 1);
    publish_transcript_edits(&coordinator, 128, delta.edits).await;
    let selected = coordinator.read().await.unwrap();
    let current = coordinator
        .agent_store(selected.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let selected_session = current.list_sessions(None, 10).await.unwrap().selections[0];
    let AgentObjectV1::SessionSnapshot(session) =
        current.read_object(selected_session.1).await.unwrap()
    else {
        panic!("session")
    };
    let AgentObjectV1::TranscriptManifest(manifest) =
        current.read_object(session.transcripts[0]).await.unwrap()
    else {
        panic!("manifest")
    };
    assert_eq!(manifest.streams.len(), 2);
    assert_eq!(manifest.streams[1].ordered_chunk_root, other_page);
    let range = current
        .transcript_range(
            session.transcripts[0],
            initial.source_instance_id,
            1,
            None,
            10,
            1024,
        )
        .await
        .unwrap();
    assert_eq!(
        range.chunks.iter().map(|c| c.sequence).collect::<Vec<_>>(),
        vec![2, 11]
    );
    let other = current
        .transcript_range(session.transcripts[0], other_source, 1, None, 10, 1024)
        .await
        .unwrap();
    assert_eq!(other.chunks[0].payload, payload);
}

#[tokio::test]
async fn tail_append_ordering_cannot_be_inferred_from_an_unrelated_source_cursor() {
    let coordinator = fixture().await;
    let initial = codex_batch();
    publish_single_capture(&coordinator, &initial).await;
    let selected = coordinator.read().await.unwrap();
    let base = coordinator
        .agent_store(selected.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let (key, expected) = base.list_sessions(None, 10).await.unwrap().selections[0];
    let AgentObjectV1::SessionSnapshot(mut session) = base.read_object(expected).await.unwrap()
    else {
        panic!("session")
    };
    let AgentObjectV1::TranscriptManifest(mut manifest) =
        base.read_object(session.transcripts[0]).await.unwrap()
    else {
        panic!("manifest")
    };
    let chunk = coordinator
        .stage_agent_object(&AgentObjectV1::TranscriptChunk(TranscriptChunkV1 {
            schema_version: 1,
            source_instance_id: initial.source_instance_id,
            source_generation: 1,
            sequence: 1000,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                br#"{"message":"outside receipt interval"}"#,
            )
            .unwrap(),
        }))
        .await
        .unwrap();
    let page = coordinator
        .stage_agent_object(&AgentObjectV1::TranscriptTailPage(TranscriptTailPageV2 {
            schema_version: 2,
            source_instance_id: initial.source_instance_id,
            source_generation: 1,
            chunks: vec![chunk],
            prefix: None,
        }))
        .await
        .unwrap();
    manifest.streams[0].ordered_chunk_root = page;
    manifest.streams[0].record_count = 1;
    session.transcripts[0] = coordinator
        .stage_agent_object(&AgentObjectV1::TranscriptManifest(manifest))
        .await
        .unwrap();
    let replacement = coordinator
        .stage_agent_object(&AgentObjectV1::SessionSnapshot(session))
        .await
        .unwrap();
    publish_transcript_edits(
        &coordinator,
        129,
        vec![AgentCatalogEditV1 {
            catalog: AgentCatalogKind::Sessions,
            key,
            expected: Some(expected),
            replacement: Some(replacement),
        }],
    )
    .await;
    let before = coordinator.read().await.unwrap();
    let selected = coordinator
        .agent_store(before.manifest().agent_root.unwrap())
        .await
        .unwrap();
    assert!(
        coordinator
            .stage_agent_capture(
                &selected,
                &NormalizerProfileV1::codex_0_156_1(),
                &codex_followup_batch(),
                125
            )
            .await
            .is_err()
    );
    assert_eq!(
        coordinator.read().await.unwrap().manifest(),
        before.manifest()
    );
}
