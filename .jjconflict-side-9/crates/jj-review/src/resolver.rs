use crate::{AnchorId, ArtifactAnchorV1, ArtifactTargetV1, ByteOffset, Error, Result};
use jj_agent::{AgentObjectRef, AgentSessionId, BlockId};
use jj_store_model::{ChangeId, CommitId, FileId, OperationId};
use serde::{Deserialize, Serialize};

const DEFAULT_MAX_BYTES: usize = 8 * 1024 * 1024;
const MAX_RESOLVER_BYTES: usize = 64 * 1024 * 1024;
const DEFAULT_MAX_CANDIDATES: usize = 8;
const MAX_RESOLVER_CANDIDATES: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RetainedTextV1 {
    Available(Vec<u8>),
    Missing,
    Redacted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CodeLineageKindV1 {
    Predecessor,
    Rename,
    Copy,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatedCodeLineageV1 {
    pub from_operation_id: OperationId,
    pub from_commit_id: CommitId,
    pub from_path: String,
    pub from_file_id: FileId,
    pub to_operation_id: OperationId,
    pub to_commit_id: CommitId,
    pub to_path: String,
    pub to_file_id: FileId,
    pub relation: CodeLineageKindV1,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeTargetArtifactV1 {
    pub path: String,
    pub file_id: FileId,
    pub text: RetainedTextV1,
    pub lineage: Option<ValidatedCodeLineageV1>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodePlacementTargetV1 {
    pub operation_id: OperationId,
    pub commit_id: CommitId,
    pub change_id: Option<ChangeId>,
    pub artifacts: Vec<CodeTargetArtifactV1>,
    pub lineage_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AgentBlockLineageV1 {
    pub from_block_version: AgentObjectRef,
    pub from_content: AgentObjectRef,
    pub to_block_version: AgentObjectRef,
    pub to_content: AgentObjectRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentPlacementTargetV1 {
    pub session_id: AgentSessionId,
    pub block_id: BlockId,
    pub block_version: AgentObjectRef,
    pub content: AgentObjectRef,
    pub representation_version: String,
    pub sanitizer_version: String,
    pub normalizer_version: String,
    pub text: RetainedTextV1,
    pub lineage: Option<AgentBlockLineageV1>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExplicitPlacementTargetV1 {
    Code(CodePlacementTargetV1),
    Agent(AgentPlacementTargetV1),
    Revision {
        operation_id: OperationId,
        commit_id: CommitId,
    },
    Operation(OperationId),
    Repository,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnchorResolutionRequestV1 {
    pub original_text: RetainedTextV1,
    pub target: ExplicitPlacementTargetV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PlacementKindV1 {
    Exact,
    MappedUnchanged,
    CandidateModified,
    Deleted,
    Ambiguous,
    Unresolved,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "target", rename_all = "snake_case")]
pub enum PlacementCandidateV1 {
    CodeText {
        operation_id: OperationId,
        commit_id: CommitId,
        path: String,
        file_id: FileId,
        start_byte: ByteOffset,
        end_byte: ByteOffset,
    },
    AgentText {
        session_id: AgentSessionId,
        block_id: BlockId,
        block_version: AgentObjectRef,
        content: AgentObjectRef,
        start_byte: ByteOffset,
        end_byte: ByteOffset,
    },
    Revision {
        operation_id: OperationId,
        commit_id: CommitId,
    },
    Operation {
        operation_id: OperationId,
    },
    Repository,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PlacementEvidenceV1 {
    ExactIdentity,
    ValidatedLineage,
    UnchangedBytes,
    ModifiedBytes,
    TargetDeleted,
    MultipleCandidates,
    MissingLineage,
    BudgetExhausted,
    MissingContent,
    RedactedContent,
    UnsupportedTarget,
    RetainedAgentPrefix,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PlacementResultV1 {
    pub schema_version: u32,
    pub anchor_id: AnchorId,
    pub placement: PlacementKindV1,
    pub candidates: Vec<PlacementCandidateV1>,
    pub evidence: Vec<PlacementEvidenceV1>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResolverBudgetV1 {
    pub max_bytes: usize,
    pub max_candidates: usize,
}
impl Default for ResolverBudgetV1 {
    fn default() -> Self {
        Self {
            max_bytes: DEFAULT_MAX_BYTES,
            max_candidates: DEFAULT_MAX_CANDIDATES,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AnchorResolverV1 {
    budget: ResolverBudgetV1,
}
impl AnchorResolverV1 {
    pub fn new(budget: ResolverBudgetV1) -> Result<Self> {
        if budget.max_bytes == 0
            || budget.max_bytes > MAX_RESOLVER_BYTES
            || budget.max_candidates == 0
            || budget.max_candidates > MAX_RESOLVER_CANDIDATES
        {
            return Err(Error::Invalid("anchor resolver budget"));
        }
        Ok(Self { budget })
    }

    pub fn resolve(
        &self,
        anchor: &ArtifactAnchorV1,
        request: &AnchorResolutionRequestV1,
    ) -> Result<PlacementResultV1> {
        anchor.validate()?;
        let anchor_id = anchor.anchor_id()?;
        match (&anchor.target, &request.target) {
            (ArtifactTargetV1::CodeText(original), ExplicitPlacementTargetV1::Code(target)) => {
                crate::resolver_code::resolve_code_text(
                    anchor_id,
                    original,
                    &request.original_text,
                    target,
                    self.budget,
                )
            }
            (ArtifactTargetV1::AgentText(original), ExplicitPlacementTargetV1::Agent(target)) => {
                crate::resolver_agent::resolve_agent_text(
                    anchor_id,
                    original,
                    &request.original_text,
                    target,
                    self.budget,
                )
            }
            (
                ArtifactTargetV1::Revision {
                    operation_id,
                    commit_id,
                    ..
                },
                ExplicitPlacementTargetV1::Revision {
                    operation_id: target_operation,
                    commit_id: target_commit,
                },
            ) if operation_id == target_operation && commit_id == target_commit => Ok(result(
                anchor_id,
                PlacementKindV1::Exact,
                vec![PlacementCandidateV1::Revision {
                    operation_id: *operation_id,
                    commit_id: *commit_id,
                }],
                PlacementEvidenceV1::ExactIdentity,
            )),
            (
                ArtifactTargetV1::Operation { operation_id },
                ExplicitPlacementTargetV1::Operation(target),
            ) if operation_id == target => Ok(result(
                anchor_id,
                PlacementKindV1::Exact,
                vec![PlacementCandidateV1::Operation {
                    operation_id: *operation_id,
                }],
                PlacementEvidenceV1::ExactIdentity,
            )),
            (ArtifactTargetV1::Repository, ExplicitPlacementTargetV1::Repository) => Ok(result(
                anchor_id,
                PlacementKindV1::Exact,
                vec![PlacementCandidateV1::Repository],
                PlacementEvidenceV1::ExactIdentity,
            )),
            _ => Ok(result(
                anchor_id,
                PlacementKindV1::Unresolved,
                Vec::new(),
                PlacementEvidenceV1::UnsupportedTarget,
            )),
        }
    }
}

pub(crate) fn result(
    anchor_id: AnchorId,
    placement: PlacementKindV1,
    candidates: Vec<PlacementCandidateV1>,
    evidence: PlacementEvidenceV1,
) -> PlacementResultV1 {
    PlacementResultV1 {
        schema_version: 1,
        anchor_id,
        placement,
        candidates,
        evidence: vec![evidence],
    }
}

pub(crate) fn unavailable_evidence(value: &RetainedTextV1) -> Option<PlacementEvidenceV1> {
    match value {
        RetainedTextV1::Available(_) => None,
        RetainedTextV1::Missing => Some(PlacementEvidenceV1::MissingContent),
        RetainedTextV1::Redacted => Some(PlacementEvidenceV1::RedactedContent),
    }
}
