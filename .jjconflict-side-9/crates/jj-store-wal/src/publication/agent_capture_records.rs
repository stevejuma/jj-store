use super::agent_capture_tools::merge_selected_tool;
use super::*;
use jj_agent::{
    AgentContentV1, AgentEventBatchV1, AgentEventV1, AgentObjectV1, AgentSessionId,
    NormalizationBatchV1, NormalizedActivityRecordV1, NormalizerProfileV1,
    SanitizedObservationBatchV1, SanitizedPayloadV1, ToolCallId, ToolInvocationSnapshotV1,
    TranscriptChunkV1, normalized_event_id,
};

pub(super) struct CaptureObjects {
    pub evidence: AgentObjectRef,
    pub event_batch: AgentObjectRef,
    pub tools: BTreeMap<AgentSessionId, BTreeMap<ToolCallId, AgentObjectRef>>,
    pub transcript_chunks: BTreeMap<AgentSessionId, Vec<TranscriptChunkSelection>>,
}

#[derive(Clone, Copy)]
pub(super) struct TranscriptChunkSelection {
    pub sequence: u64,
    pub bytes: u64,
    pub reference: AgentObjectRef,
}

pub(super) async fn stage_capture_objects(
    coordinator: &dyn AgentObjectIo,
    base: &dyn AgentSnapshotRead,
    profile: &NormalizerProfileV1,
    source: &SanitizedObservationBatchV1,
    normalized: &mut NormalizationBatchV1,
) -> Result<CaptureObjects> {
    let evidence = coordinator
        .stage_agent_object(&AgentObjectV1::ObservationBatch(source.clone()))
        .await?;
    let mut events = Vec::with_capacity(normalized.records.len());
    let mut transcript_chunks: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for record in &normalized.records {
        let (sequence, kind) = record_identity(record);
        let event_id = normalized_event_id(
            source.source_instance_id,
            source.integration,
            sequence,
            kind,
        );
        let event = match record {
            NormalizedActivityRecordV1::SessionState(value) => AgentEventV1::SessionStateChanged {
                event_id,
                session_id: value.session_id,
                state: value.state,
            },
            NormalizedActivityRecordV1::Message(value) => {
                let content = stage_content(coordinator, &value.content).await?;
                let reference = coordinator
                    .stage_agent_object(&AgentObjectV1::TranscriptChunk(TranscriptChunkV1 {
                        schema_version: 1,
                        source_instance_id: source.source_instance_id,
                        source_generation: 1,
                        sequence: value.source_sequence,
                        payload: value.content.clone(),
                    }))
                    .await?;
                transcript_chunks.entry(value.session_id).or_default().push(
                    TranscriptChunkSelection {
                        sequence: value.source_sequence,
                        bytes: value.content.bytes().len() as u64,
                        reference,
                    },
                );
                AgentEventV1::MessageCompleted {
                    event_id,
                    session_id: value.session_id,
                    turn_id: value.turn_id,
                    message_id: value.message_id,
                    role: value.role,
                    content,
                }
            }
            NormalizedActivityRecordV1::Tool(value) => AgentEventV1::ToolStateChanged {
                event_id,
                session_id: value.session_id,
                turn_id: value.turn_id,
                tool_call_id: value.tool_call_id,
                state: value.state,
            },
            NormalizedActivityRecordV1::Permission(value) => AgentEventV1::PermissionObserved {
                event_id,
                session_id: value.session_id,
                tool_call_id: value.tool_call_id,
                state: value.state,
            },
            NormalizedActivityRecordV1::FileObservation(value) => AgentEventV1::FileObserved {
                event_id,
                session_id: value.session_id,
                path: value.path.clone(),
            },
        };
        events.push(event);
    }
    let event_batch = coordinator
        .stage_agent_object(&AgentObjectV1::EventBatch(AgentEventBatchV1 {
            schema_version: 1,
            normalizer_version: profile.identity(),
            evidence,
            events,
        }))
        .await?;
    let mut tools: BTreeMap<_, BTreeMap<_, _>> = BTreeMap::new();
    let reduced = normalized.reduce_tools();
    normalized.diagnostics.extend(reduced.diagnostics);
    for tool in reduced.tools {
        let input = stage_optional_content(coordinator, tool.input.as_ref()).await?;
        let output = stage_optional_content(coordinator, tool.output.as_ref()).await?;
        let session_tools = tools.entry(tool.session_id).or_default();
        let mut snapshot = ToolInvocationSnapshotV1 {
            schema_version: 1,
            tool_call_id: tool.tool_call_id,
            session_id: tool.session_id,
            turn_id: tool.turn_id,
            native_name: tool.native_name,
            kind: tool.kind,
            state: tool.state,
            attempts: BTreeSet::new(),
            input,
            output,
            evidence: vec![evidence],
        };
        if merge_selected_tool(base, &mut snapshot).await? {
            normalized
                .diagnostics
                .push(jj_agent::NormalizationDiagnosticV1 {
                    kind: jj_agent::NormalizationDiagnosticKind::ConflictingTerminalState,
                    code: "tool-terminal-conflict".into(),
                    source_sequence: Some(tool.source_sequence),
                });
        }
        let reference = coordinator
            .stage_agent_object(&AgentObjectV1::ToolSnapshot(snapshot))
            .await?;
        session_tools.insert(tool.tool_call_id, reference);
    }
    Ok(CaptureObjects {
        evidence,
        event_batch,
        tools,
        transcript_chunks,
    })
}

async fn stage_optional_content(
    coordinator: &dyn AgentObjectIo,
    payload: Option<&SanitizedPayloadV1>,
) -> Result<Option<AgentObjectRef>> {
    match payload {
        Some(value) => Ok(Some(stage_content(coordinator, value).await?)),
        None => Ok(None),
    }
}

async fn stage_content(
    coordinator: &dyn AgentObjectIo,
    payload: &SanitizedPayloadV1,
) -> Result<AgentObjectRef> {
    coordinator
        .stage_agent_object(&AgentObjectV1::Content(AgentContentV1 {
            schema_version: 1,
            payload: payload.clone(),
        }))
        .await
}

fn record_identity(record: &NormalizedActivityRecordV1) -> (u64, &'static str) {
    match record {
        NormalizedActivityRecordV1::SessionState(value) => (value.source_sequence, "session"),
        NormalizedActivityRecordV1::Message(value) => (value.source_sequence, "message"),
        NormalizedActivityRecordV1::Tool(value) => (value.source_sequence, "tool"),
        NormalizedActivityRecordV1::Permission(value) => (value.source_sequence, "permission"),
        NormalizedActivityRecordV1::FileObservation(value) => (value.source_sequence, "file"),
    }
}
