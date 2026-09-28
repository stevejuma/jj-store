use jj_store_model as m;
use jj_store_wal::publication::{
    AgentObjectId, AgentObjectRef, AgentSessionId, AgentSessionSnapshotV1, CompletenessV1,
    IntegrationId, SanitizedMediaType, SanitizedRepresentation, SessionState, SourceInstanceId,
    ToolCallId, TranscriptStreamV1,
};
use schemars::JsonSchema;
use serde::Serialize;

pub type AgentSessionIdentity = AgentSessionId;
pub type AgentTranscriptIdentity = AgentObjectId;
pub type AgentSourceInstanceIdentity = SourceInstanceId;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentReadSnapshot {
    pub repo_id: m::RepositoryId,
    pub epoch: m::EpochId,
    pub published_seq: m::WalPosition,
    pub manifest_revision: u64,
    pub agent_root: m::RecordDigest,
    pub agent_state_version: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentCoverage {
    pub completeness: CompletenessV1,
    pub gaps: u32,
    pub redacted_fields: u32,
    pub truncated: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentSessionSummary {
    pub session_id: AgentSessionId,
    pub snapshot_id: AgentObjectId,
    pub integration: IntegrationId,
    pub state: SessionState,
    pub parent_session_id: Option<AgentSessionId>,
    pub coverage: AgentCoverage,
    pub evidence_count: usize,
    pub event_batch_count: usize,
    pub transcript_count: usize,
    pub tool_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentSessionPage {
    pub version: u32,
    pub snapshot: AgentReadSnapshot,
    pub sessions: Vec<AgentSessionSummary>,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentToolLink {
    pub tool_call_id: ToolCallId,
    pub snapshot_id: AgentObjectId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentTranscriptDescriptor {
    pub transcript_id: AgentObjectId,
    pub source_instance_id: SourceInstanceId,
    pub source_generation: u64,
    pub source_format: String,
    pub decoder_version: String,
    pub sanitizer_version: String,
    pub representation: SanitizedRepresentation,
    pub logical_byte_length: u64,
    pub record_count: u64,
    pub gaps: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentTranscriptChunk {
    pub sequence: u64,
    pub media_type: SanitizedMediaType,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentTranscriptPage {
    pub version: u32,
    pub snapshot: AgentReadSnapshot,
    pub session_id: AgentSessionId,
    pub transcript_id: AgentObjectId,
    pub source_instance_id: SourceInstanceId,
    pub source_generation: u64,
    pub chunks: Vec<AgentTranscriptChunk>,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentSessionDetail {
    pub version: u32,
    pub snapshot: AgentReadSnapshot,
    pub session: AgentSessionSummary,
    pub evidence: Vec<AgentObjectId>,
    pub event_batches: Vec<AgentObjectId>,
    pub transcripts: Vec<AgentTranscriptDescriptor>,
    pub tools: Vec<AgentToolLink>,
}

pub(super) fn summary(
    reference: AgentObjectRef,
    session: &AgentSessionSnapshotV1,
) -> AgentSessionSummary {
    AgentSessionSummary {
        session_id: session.session_id,
        snapshot_id: reference.id,
        integration: session.integration,
        state: session.state,
        parent_session_id: session.parent_session_id,
        coverage: AgentCoverage {
            completeness: session.coverage.completeness,
            gaps: session.coverage.gaps,
            redacted_fields: session.coverage.redacted_fields,
            truncated: session.coverage.truncated,
        },
        evidence_count: session.evidence.len(),
        event_batch_count: session.event_batches.len(),
        transcript_count: session.transcripts.len(),
        tool_count: session.tools.len(),
    }
}

pub(super) fn detail(
    snapshot: AgentReadSnapshot,
    reference: AgentObjectRef,
    session: AgentSessionSnapshotV1,
    transcripts: Vec<AgentTranscriptDescriptor>,
) -> AgentSessionDetail {
    AgentSessionDetail {
        version: 1,
        snapshot,
        session: summary(reference, &session),
        evidence: session.evidence.iter().map(|value| value.id).collect(),
        event_batches: session.event_batches.iter().map(|value| value.id).collect(),
        transcripts,
        tools: session
            .tools
            .iter()
            .map(|(tool_call_id, value)| AgentToolLink {
                tool_call_id: *tool_call_id,
                snapshot_id: value.id,
            })
            .collect(),
    }
}

pub(super) fn transcript_descriptor(
    transcript_id: AgentObjectId,
    stream: &TranscriptStreamV1,
) -> AgentTranscriptDescriptor {
    AgentTranscriptDescriptor {
        transcript_id,
        source_instance_id: stream.source_instance_id,
        source_generation: stream.source_generation,
        source_format: stream.source_format.clone(),
        decoder_version: stream.decoder_version.clone(),
        sanitizer_version: stream.sanitizer_version.clone(),
        representation: stream.representation,
        logical_byte_length: stream.logical_byte_length,
        record_count: stream.record_count,
        gaps: stream.gaps,
    }
}
