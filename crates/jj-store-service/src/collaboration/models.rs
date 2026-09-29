use jj_store_model as m;
use jj_store_wal::publication::{
    AgentCheckpointId, AgentEventId, AgentObjectId, AgentObjectRef, AgentRelationSubjectV1,
    AgentSessionId, AgentSessionSnapshotV1, AssociationId, AssociationRelation, CodeAssociationV1,
    CompletenessV1, IntegrationId, MessageId, MessageRole, PermissionState, SanitizedMediaType,
    SanitizedRepresentation, SessionState, SourceInstanceId, ToolCallId, ToolKind, ToolState,
    TranscriptStreamV1, TurnId,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type AgentSessionIdentity = AgentSessionId;
pub type AgentTranscriptIdentity = AgentObjectId;
pub type AgentSourceInstanceIdentity = SourceInstanceId;
pub type AgentToolIdentity = ToolCallId;
pub type AgentCheckpointIdentity = AgentCheckpointId;

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
    pub first_activity_at_millis: Option<i64>,
    pub last_activity_at_millis: Option<i64>,
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
pub struct AgentContent {
    pub media_type: SanitizedMediaType,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentCodeAssociation {
    pub association_id: AssociationId,
    pub object_id: AgentObjectId,
    pub session_id: AgentSessionId,
    pub tool_call_ids: Vec<ToolCallId>,
    pub relation: AssociationRelation,
    pub before_operations: Vec<m::OperationId>,
    pub after_operations: Vec<m::OperationId>,
    pub before_commits: Vec<m::CommitId>,
    pub after_commits: Vec<m::CommitId>,
    pub changes: Vec<m::ChangeId>,
    pub paths: Vec<String>,
    pub created_at_millis: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentToolActivity {
    pub tool_call_id: ToolCallId,
    pub snapshot_id: AgentObjectId,
    pub turn_id: Option<TurnId>,
    pub native_name: String,
    pub kind: ToolKind,
    pub state: ToolState,
    pub input: Option<AgentContent>,
    pub output: Option<AgentContent>,
    pub observed_paths: Vec<String>,
    pub code: Vec<AgentCodeAssociation>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AgentActivity {
    SessionState {
        sequence: u64,
        event_id: AgentEventId,
        state: SessionState,
    },
    Message {
        sequence: u64,
        event_id: AgentEventId,
        turn_id: Option<TurnId>,
        message_id: MessageId,
        role: MessageRole,
        content: AgentContent,
    },
    Tool {
        sequence: u64,
        event_id: AgentEventId,
        tool: AgentToolActivity,
    },
    Permission {
        sequence: u64,
        event_id: AgentEventId,
        tool_call_id: Option<ToolCallId>,
        state: PermissionState,
    },
    FileObserved {
        sequence: u64,
        event_id: AgentEventId,
        path: String,
    },
    CaptureGap {
        sequence: u64,
        event_id: AgentEventId,
        missing_from: u64,
        missing_to: u64,
    },
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "subject", rename_all = "snake_case", deny_unknown_fields)]
pub enum AgentRelationSubject {
    Session {
        session_id: AgentSessionId,
    },
    Tool {
        session_id: AgentSessionId,
        tool_call_id: ToolCallId,
    },
    Operation {
        operation_id: m::OperationId,
    },
    Revision {
        commit_id: m::CommitId,
    },
    Checkpoint {
        checkpoint_id: AgentCheckpointId,
    },
}

impl AgentRelationSubject {
    pub(super) fn wal(self) -> AgentRelationSubjectV1 {
        match self {
            Self::Session { session_id } => AgentRelationSubjectV1::Session { session_id },
            Self::Tool {
                session_id,
                tool_call_id,
            } => AgentRelationSubjectV1::Tool {
                session_id,
                tool_call_id,
            },
            Self::Operation { operation_id } => AgentRelationSubjectV1::Operation { operation_id },
            Self::Revision { commit_id } => AgentRelationSubjectV1::Revision { commit_id },
            Self::Checkpoint { checkpoint_id } => {
                AgentRelationSubjectV1::Checkpoint { checkpoint_id }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AgentRelationKind {
    Checkpoint,
    Association,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentRelationLink {
    pub kind: AgentRelationKind,
    pub object_id: AgentObjectId,
    pub association: Option<AgentCodeAssociation>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentRelationPage {
    pub version: u32,
    pub snapshot: AgentReadSnapshot,
    pub subject: AgentRelationSubject,
    pub relations: Vec<AgentRelationLink>,
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
    pub activity: Vec<AgentActivity>,
    pub activity_truncated: bool,
    pub code: Vec<AgentCodeAssociation>,
    pub code_truncated: bool,
}

pub(super) fn summary(
    reference: AgentObjectRef,
    session: &AgentSessionSnapshotV1,
    timing: super::agent_session_timing::SessionTiming,
) -> AgentSessionSummary {
    AgentSessionSummary {
        session_id: session.session_id,
        snapshot_id: reference.id,
        integration: session.integration,
        state: session.state,
        parent_session_id: session.parent_session_id,
        first_activity_at_millis: timing.first_activity_at_millis,
        last_activity_at_millis: timing.last_activity_at_millis,
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

pub(super) struct AgentSessionDetailContent {
    pub timing: super::agent_session_timing::SessionTiming,
    pub transcripts: Vec<AgentTranscriptDescriptor>,
    pub activity: Vec<AgentActivity>,
    pub activity_truncated: bool,
    pub code: Vec<AgentCodeAssociation>,
    pub code_truncated: bool,
}

pub(super) fn detail(
    snapshot: AgentReadSnapshot,
    reference: AgentObjectRef,
    session: AgentSessionSnapshotV1,
    content: AgentSessionDetailContent,
) -> AgentSessionDetail {
    AgentSessionDetail {
        version: 1,
        snapshot,
        session: summary(reference, &session, content.timing),
        evidence: session.evidence.iter().map(|value| value.id).collect(),
        event_batches: session.event_batches.iter().map(|value| value.id).collect(),
        transcripts: content.transcripts,
        tools: session
            .tools
            .iter()
            .map(|(tool_call_id, value)| AgentToolLink {
                tool_call_id: *tool_call_id,
                snapshot_id: value.id,
            })
            .collect(),
        activity: content.activity,
        activity_truncated: content.activity_truncated,
        code: content.code,
        code_truncated: content.code_truncated,
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

pub(super) fn code_association(
    object_id: AgentObjectId,
    value: &CodeAssociationV1,
) -> AgentCodeAssociation {
    AgentCodeAssociation {
        association_id: value.association_id,
        object_id,
        session_id: value.session_id,
        tool_call_ids: value.tool_call_ids.iter().copied().collect(),
        relation: value.relation,
        before_operations: value.before.operations.iter().copied().collect(),
        after_operations: value.after.operations.iter().copied().collect(),
        before_commits: value.before.commits.iter().copied().collect(),
        after_commits: value.after.commits.iter().copied().collect(),
        changes: value.changes.iter().copied().collect(),
        paths: value.paths.clone(),
        created_at_millis: value.created_at_millis,
    }
}
