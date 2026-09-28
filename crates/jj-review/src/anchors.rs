use crate::{AnchorId, Result, TextSelectionV1};
use jj_agent::{AgentObjectRef, AgentSessionId, BlockId, MessageId, ToolAttemptId, ToolCallId};
use jj_store_model::{
    ChangeId, CommitId, FileId, ObjectKey, OperationId, RecordDigest, RepositoryId,
};
use serde::{Deserialize, Serialize};
use sha2::Digest as _;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SensitivityV1 {
    Public,
    Repository,
    Restricted,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CreatedContextV1 {
    pub publication_position: Option<u64>,
    pub code_state_version: Option<u64>,
    pub agent_root: Option<RecordDigest>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CodeValueKind {
    File,
    BinaryFile,
    EmptyFile,
    Symlink,
    Gitlink,
    ConflictTerm,
    ConflictArtifact,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DiffSide {
    Before,
    After,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonKind {
    ExplicitPair,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DiffContextV1 {
    pub before_operation_id: OperationId,
    pub before_commit_id: CommitId,
    pub before_path: Option<String>,
    pub after_operation_id: OperationId,
    pub after_commit_id: CommitId,
    pub after_path: Option<String>,
    pub side: DiffSide,
    pub comparison: ComparisonKind,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CodeTextAnchorV1 {
    pub operation_id: OperationId,
    pub commit_id: CommitId,
    pub change_id: Option<ChangeId>,
    pub path: String,
    pub file_id: FileId,
    pub value_kind: CodeValueKind,
    pub text: TextSelectionV1,
    pub diff_context: Option<DiffContextV1>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CodeArtifactAnchorV1 {
    pub operation_id: OperationId,
    pub commit_id: CommitId,
    pub change_id: Option<ChangeId>,
    pub path: String,
    pub object: ObjectKey,
    pub value_kind: CodeValueKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AgentContentPart {
    Input,
    Output,
    Message,
    Plan,
    ExposedSummary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SourceCoverageV1 {
    Complete,
    Partial,
    Redacted,
    Truncated,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AgentContentOwnerV1 {
    Tool {
        tool_call_id: ToolCallId,
        snapshot: AgentObjectRef,
        attempt_id: ToolAttemptId,
    },
    Message {
        message_id: MessageId,
        snapshot: AgentObjectRef,
    },
    Plan {
        block_id: BlockId,
        snapshot: AgentObjectRef,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentTextAnchorV1 {
    pub session_id: AgentSessionId,
    pub session_snapshot: AgentObjectRef,
    pub owner: AgentContentOwnerV1,
    pub block_id: BlockId,
    pub block_version: AgentObjectRef,
    pub part: AgentContentPart,
    pub content: AgentObjectRef,
    pub representation_version: String,
    pub sanitizer_version: String,
    pub normalizer_version: String,
    pub source_coverage: SourceCoverageV1,
    pub text: TextSelectionV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentArtifactAnchorV1 {
    pub session_id: AgentSessionId,
    pub session_snapshot: AgentObjectRef,
    pub owner: AgentContentOwnerV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "target", content = "value", rename_all = "snake_case")]
pub enum ArtifactTargetV1 {
    CodeText(Box<CodeTextAnchorV1>),
    CodeArtifact(Box<CodeArtifactAnchorV1>),
    AgentText(Box<AgentTextAnchorV1>),
    AgentArtifact(Box<AgentArtifactAnchorV1>),
    Revision {
        operation_id: OperationId,
        commit_id: CommitId,
        change_id: Option<ChangeId>,
    },
    Operation {
        operation_id: OperationId,
    },
    Repository,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ArtifactAnchorV1 {
    pub schema_version: u32,
    pub repository_id: RepositoryId,
    pub target: ArtifactTargetV1,
    pub created_context: CreatedContextV1,
    pub sensitivity: SensitivityV1,
}

impl ArtifactAnchorV1 {
    pub fn anchor_id(&self) -> Result<AnchorId> {
        Ok(AnchorId::new(
            sha2::Sha256::digest(crate::encode_anchor(self)?).into(),
        ))
    }
}
