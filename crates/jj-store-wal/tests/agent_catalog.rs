use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectRef,
    AgentObjectV1, AgentRelationPageV1, AgentRelationSubjectV1, AgentSessionId,
    AgentSessionSnapshotV1, AssociationGranularity, AssociationId, AssociationRelation,
    CodeAnchorV1, CodeAssociationV1, CompletenessV1, CoverageV1, EvidenceMethod, IntegrationId,
    SanitizedMediaType, SanitizedPayloadV1, SanitizedRepresentation, SessionState,
    SourceInstanceId, SourceTrust, TranscriptChunkV1, TranscriptManifestV1, TranscriptPageV1,
    TranscriptStreamV1,
};
use jj_store_model::{EpochId, ObjectKey, OperationId, RepositoryId};
use jj_store_object::memory::MemoryStore;
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication::{Coordinator, Limits, PublicationGuard, Request, Snapshot},
};
use std::{collections::BTreeMap, collections::BTreeSet, sync::Arc};

const REPOSITORY: RepositoryId = RepositoryId::new([1; 16]);

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
        .stage_agent_object(&AgentObjectV1::TranscriptPage(TranscriptPageV1 {
            schema_version: 1,
            source_instance_id: source_id,
            source_generation: 4,
            chunks: vec![second_chunk],
            next: None,
        }))
        .await
        .unwrap();
    let chunk_page = coordinator
        .stage_agent_object(&AgentObjectV1::TranscriptPage(TranscriptPageV1 {
            schema_version: 1,
            source_instance_id: source_id,
            source_generation: 4,
            chunks: vec![first_chunk],
            next: Some(second_page),
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
