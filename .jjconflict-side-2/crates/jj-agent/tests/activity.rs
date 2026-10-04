use jj_agent::{
    AgentEventBatchV1, AgentEventId, AgentEventV1, AgentObjectId, AgentObjectKind, AgentObjectRef,
    AgentObjectV1, AgentSessionId, AgentSessionSnapshotV1, CaptureBatchId, CompletenessV1,
    CoverageV1, IntegrationId, MessageId, MessageRole, PermissionState, SanitizedMediaType,
    SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
    SanitizedRepresentation, SessionState, SourceInstanceId, SourceKind, SourceTrust, ToolCallId,
    ToolInvocationSnapshotV1, ToolKind, ToolState, TranscriptChunkV1, TranscriptManifestV1,
    TranscriptStreamV1, TranscriptTailPageV2, TurnId, decode_object, encode_object,
};
use std::collections::{BTreeMap, BTreeSet};

fn reference(kind: AgentObjectKind, byte: u8) -> AgentObjectRef {
    AgentObjectRef::new(kind, AgentObjectId::new([byte; 32]))
}

fn coverage() -> CoverageV1 {
    CoverageV1 {
        completeness: CompletenessV1::Partial,
        gaps: 1,
        redacted_fields: 2,
        truncated: false,
    }
}

#[test]
fn session_snapshots_separate_evidence_events_tools_and_transcripts() {
    let evidence = reference(AgentObjectKind::ObservationBatch, 1);
    let events = reference(AgentObjectKind::EventBatch, 2);
    let transcript = reference(AgentObjectKind::TranscriptManifest, 3);
    let tool = reference(AgentObjectKind::ToolSnapshot, 4);
    let session = AgentSessionSnapshotV1 {
        schema_version: 1,
        session_id: AgentSessionId::new([1; 16]),
        integration: IntegrationId::Codex,
        state: SessionState::Active,
        parent_session_id: None,
        evidence: vec![evidence],
        event_batches: vec![events],
        transcripts: vec![transcript],
        tools: BTreeMap::from([(ToolCallId::new([2; 16]), tool)]),
        coverage: coverage(),
    };
    assert_eq!(
        session.dependencies().unwrap(),
        BTreeSet::from([evidence, events, transcript, tool])
    );
    let object = AgentObjectV1::SessionSnapshot(session);
    assert_eq!(
        decode_object(&encode_object(&object).unwrap()).unwrap(),
        object
    );
}

#[test]
fn sanitized_evidence_requires_canonical_payloads_and_source_order() {
    let payload = SanitizedPayloadV1::new(SanitizedMediaType::Json, br#"{"z":2,"a":1}"#).unwrap();
    assert_eq!(payload.bytes(), br#"{"a":1,"z":2}"#);
    assert!(SanitizedPayloadV1::new(SanitizedMediaType::Json, b"{").is_err());

    let observation = |sequence, byte| SanitizedObservationV1 {
        observation_id: jj_agent::ObservationId::new([byte; 16]),
        source_sequence: sequence,
        occurred_at_millis: None,
        source_kind: SourceKind::Hook,
        source_trust: SourceTrust::LocalUserHook,
        payload: payload.clone(),
    };
    let batch = SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([3; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([4; 16]),
        sanitizer_version: "agent-redaction@0.1.0".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![observation(1, 1), observation(2, 2)],
    };
    batch.validate().unwrap();
    let duplicate = SanitizedObservationBatchV1 {
        observations: vec![observation(1, 1), observation(1, 2)],
        ..batch
    };
    assert!(duplicate.validate().is_err());
}

#[test]
fn normalized_events_and_tool_snapshots_keep_typed_content_edges() {
    let content = reference(AgentObjectKind::Content, 5);
    let evidence = reference(AgentObjectKind::ObservationBatch, 6);
    let event = AgentEventV1::MessageCompleted {
        event_id: AgentEventId::new([7; 16]),
        session_id: AgentSessionId::new([8; 16]),
        turn_id: Some(TurnId::new([9; 16])),
        message_id: MessageId::new([10; 16]),
        role: MessageRole::Assistant,
        content,
    };
    let batch = AgentEventBatchV1 {
        schema_version: 1,
        normalizer_version: "codex@0.156.1/v1".into(),
        evidence,
        events: vec![
            event,
            AgentEventV1::PermissionObserved {
                event_id: AgentEventId::new([17; 16]),
                session_id: AgentSessionId::new([8; 16]),
                tool_call_id: Some(ToolCallId::new([11; 16])),
                state: PermissionState::Granted,
            },
            AgentEventV1::FileObserved {
                event_id: AgentEventId::new([18; 16]),
                session_id: AgentSessionId::new([8; 16]),
                path: "src/lib.rs".into(),
            },
        ],
    };
    assert_eq!(
        batch.dependencies().unwrap(),
        BTreeSet::from([evidence, content])
    );

    let tool = ToolInvocationSnapshotV1 {
        schema_version: 1,
        tool_call_id: ToolCallId::new([11; 16]),
        session_id: AgentSessionId::new([8; 16]),
        turn_id: Some(TurnId::new([9; 16])),
        native_name: "apply_patch".into(),
        kind: ToolKind::Edit,
        state: ToolState::Succeeded,
        attempts: BTreeSet::new(),
        input: Some(content),
        output: None,
        evidence: vec![evidence],
    };
    assert_eq!(
        tool.dependencies().unwrap(),
        BTreeSet::from([content, evidence])
    );
}

#[test]
fn transcript_manifests_reconstruct_from_their_own_chunk_page_closure() {
    let chunk_ref = reference(AgentObjectKind::TranscriptChunk, 12);
    let page_ref = reference(AgentObjectKind::TranscriptPage, 13);
    let prefix_page_ref = reference(AgentObjectKind::TranscriptPage, 16);
    let chunk = TranscriptChunkV1 {
        schema_version: 1,
        source_instance_id: SourceInstanceId::new([14; 16]),
        source_generation: 1,
        sequence: 0,
        payload: SanitizedPayloadV1::new(SanitizedMediaType::Text, b"hello\n").unwrap(),
    };
    assert!(chunk.dependencies().unwrap().is_empty());
    let page = TranscriptTailPageV2 {
        schema_version: 2,
        source_instance_id: SourceInstanceId::new([14; 16]),
        source_generation: 1,
        chunks: vec![chunk_ref],
        prefix: Some(prefix_page_ref),
    };
    assert_eq!(
        page.dependencies().unwrap(),
        BTreeSet::from([chunk_ref, prefix_page_ref])
    );
    let manifest = TranscriptManifestV1 {
        schema_version: 1,
        session_id: AgentSessionId::new([15; 16]),
        streams: vec![TranscriptStreamV1 {
            source_instance_id: SourceInstanceId::new([14; 16]),
            source_generation: 1,
            source_format: "codex-jsonl".into(),
            decoder_version: "codex@0.156.1/v1".into(),
            sanitizer_version: "agent-redaction@0.1.0".into(),
            representation: SanitizedRepresentation::SanitizedNative,
            ordered_chunk_root: page_ref,
            logical_byte_length: 6,
            record_count: 1,
            gaps: 0,
        }],
        coverage: coverage(),
    };
    assert_eq!(manifest.dependencies().unwrap(), BTreeSet::from([page_ref]));
}

#[test]
fn only_current_tail_pages_decode_and_retain_strong_prefix_dependencies() {
    let source = SourceInstanceId::new([20; 16]);
    let chunk = reference(AgentObjectKind::TranscriptChunk, 21);
    let prefix = reference(AgentObjectKind::TranscriptPage, 22);
    let old_bytes = format!(r#"{{"type":"transcript_page","value":{{"schema_version":1,"source_instance_id":"{source}","source_generation":1,"chunks":[{{"kind":"transcript_chunk","id":"{}"}}]}}}}"#, chunk.id).into_bytes();
    assert!(decode_object(&old_bytes).is_err());
    let mut tail = jj_agent::TranscriptTailPageV2 {
        schema_version: 2,
        source_instance_id: source,
        source_generation: 1,
        chunks: vec![chunk],
        prefix: Some(prefix),
    };
    assert_eq!(
        tail.dependencies().unwrap(),
        BTreeSet::from([chunk, prefix])
    );
    let object = AgentObjectV1::TranscriptTailPage(tail.clone());
    assert_eq!(object.kind(), AgentObjectKind::TranscriptPage);
    assert_eq!(
        decode_object(&encode_object(&object).unwrap()).unwrap(),
        object
    );
    tail.prefix = Some(chunk);
    assert!(tail.validate().is_err());
    tail.prefix = None;
    tail.schema_version = 1;
    assert!(tail.validate().is_err());
    tail.schema_version = 2;
    tail.source_generation = 0;
    assert!(tail.validate().is_err());
    tail.source_generation = 1;
    tail.chunks = vec![chunk; 257];
    assert!(tail.validate().is_err());
}
