use super::*;
use crate::local_repository::LocalAgentSnapshot;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct LocalBrowserAgentPage {
    pub version: u32,
    pub snapshot: LocalAgentSnapshot,
    pub sessions: Vec<presentation::AgentSessionSummary>,
    pub next_after: Option<AgentCatalogKey>,
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct LocalBrowserAgentDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_feedback: Option<crate::agent_execution::Feedback>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_unavailable: Option<String>,
    pub version: u32,
    pub snapshot: LocalAgentSnapshot,
    pub session: presentation::AgentSessionSummary,
    pub evidence: Vec<AgentObjectId>,
    pub event_batches: Vec<AgentObjectId>,
    pub transcripts: Vec<presentation::AgentTranscriptDescriptor>,
    pub tools: Vec<presentation::AgentToolLink>,
    pub activity: Vec<presentation::AgentActivity>,
    pub activity_truncated: bool,
    pub code: Vec<LocalBrowserCodeAssociation>,
    pub code_truncated: bool,
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct LocalBrowserCodeAssociation {
    pub profile: m::portable::CodeProfileV1,
    pub source_repositories: Vec<m::RepositoryId>,
    pub evidence: Vec<m::portable::CodeEvidenceRefV1>,
    pub association_id: AssociationId,
    pub object_id: AgentObjectId,
    pub session_id: AgentSessionId,
    pub tool_call_ids: Vec<ToolCallId>,
    pub relation: AssociationRelation,
    pub before_operations: Vec<m::OperationId>,
    pub after_operations: Vec<m::OperationId>,
    pub before_commits: Vec<m::GitSha1Id>,
    pub after_commits: Vec<m::GitSha1Id>,
    pub changes: Vec<m::ChangeId>,
    pub paths: Vec<String>,
    pub created_at_millis: i64,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "subject", rename_all = "snake_case", deny_unknown_fields)]
pub enum LocalAgentSubject {
    Session {
        session_id: AgentSessionId,
    },
    Operation {
        repository: m::RepositoryId,
        operation_id: m::OperationId,
    },
    Revision {
        repository: m::RepositoryId,
        commit_id: m::GitSha1Id,
    },
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct LocalBrowserAgentRelations {
    pub version: u32,
    pub snapshot: LocalAgentSnapshot,
    pub subject: LocalAgentSubject,
    pub associations: Vec<LocalBrowserCodeAssociation>,
    pub next_after: Option<AgentCatalogKey>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LocalBrowserTranscriptRequest {
    pub session_id: AgentSessionId,
    pub transcript_id: AgentObjectId,
    pub source_instance_id: SourceInstanceId,
    pub source_generation: u64,
    pub limit: usize,
    pub max_bytes: usize,
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct LocalBrowserTranscriptPage {
    pub version: u32,
    pub snapshot: LocalAgentSnapshot,
    pub session_id: AgentSessionId,
    pub transcript_id: AgentObjectId,
    pub source_instance_id: SourceInstanceId,
    pub source_generation: u64,
    pub chunks: Vec<presentation::AgentTranscriptChunk>,
    pub next_after_sequence: Option<u64>,
}
