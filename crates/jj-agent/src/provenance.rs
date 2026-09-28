use crate::{
    AgentCheckpointId, AgentObjectKind, AgentObjectRef, AgentSessionId, AssociationId,
    CompletenessV1, CoverageV1, Error, ProvenanceScopeId, Result, SourceTrust, ToolCallId, TurnId,
    activity::valid_label,
};
use jj_store_model::{ChangeId, CommitId, ObjectKey, OperationId};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

const MAX_ANCHORS: usize = 256;
const MAX_REFS: usize = 4096;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CodeAnchorV1 {
    pub operations: BTreeSet<OperationId>,
    pub commits: BTreeSet<CommitId>,
}
impl CodeAnchorV1 {
    fn validate(&self) -> Result<()> {
        if self.operations.len() > MAX_ANCHORS || self.commits.len() > MAX_ANCHORS {
            return Err(Error::Limit);
        }
        Ok(())
    }
    fn is_empty(&self) -> bool {
        self.operations.is_empty() && self.commits.is_empty()
    }
    fn native_dependencies(&self) -> BTreeSet<ObjectKey> {
        self.operations
            .iter()
            .copied()
            .map(ObjectKey::Operation)
            .chain(self.commits.iter().copied().map(ObjectKey::Commit))
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CodeRelation {
    Changed,
    NoChange,
    NotObserved,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssociationRelation {
    InitiatedRevision,
    RewroteRevision,
    ContributedToSnapshot,
    ObservedDuringInterval,
    ReadRevision,
    ValidatedRevision,
    ManualAssertion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssociationGranularity {
    Operation,
    Revision,
    SnapshotInterval,
    File,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceMethod {
    ControlledScope,
    HookBoundary,
    TranscriptRecovery,
    HistoricalImport,
    ManualAssertion,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CodeAssociationV1 {
    pub schema_version: u32,
    pub association_id: AssociationId,
    pub checkpoint_id: Option<AgentCheckpointId>,
    pub session_id: AgentSessionId,
    pub turn_id: Option<TurnId>,
    pub tool_call_ids: BTreeSet<ToolCallId>,
    pub scope_id: Option<ProvenanceScopeId>,
    pub relation: AssociationRelation,
    pub before: CodeAnchorV1,
    pub after: CodeAnchorV1,
    pub changes: BTreeSet<ChangeId>,
    pub paths: Vec<String>,
    pub granularity: AssociationGranularity,
    pub evidence_method: EvidenceMethod,
    pub source_trust: SourceTrust,
    pub exclusive_write_scope: bool,
    pub evidence: Vec<AgentObjectRef>,
    pub coverage: CoverageV1,
    pub created_at_millis: i64,
    pub supersedes: Option<AgentObjectRef>,
    pub correction_reason: Option<String>,
}
impl CodeAssociationV1 {
    pub fn validate(&self) -> Result<()> {
        self.before.validate()?;
        self.after.validate()?;
        let invalid_path = self.paths.iter().any(|path| {
            path.is_empty()
                || path.len() > 4096
                || path.starts_with('/')
                || path.contains('\\')
                || path.chars().any(char::is_control)
                || path
                    .split('/')
                    .any(|part| part.is_empty() || matches!(part, "." | ".."))
        });
        let wrong_evidence = self.evidence.iter().any(|value| {
            !matches!(
                value.kind,
                AgentObjectKind::ObservationBatch
                    | AgentObjectKind::EventBatch
                    | AgentObjectKind::Content
            )
        });
        let wrong_granularity = match self.granularity {
            AssociationGranularity::Operation => {
                self.before.operations.is_empty() && self.after.operations.is_empty()
            }
            AssociationGranularity::Revision => {
                self.before.commits.is_empty() && self.after.commits.is_empty()
            }
            AssociationGranularity::SnapshotInterval => false,
            AssociationGranularity::File => self.paths.is_empty(),
        };
        let correction_valid = match (self.supersedes, &self.correction_reason) {
            (None, None) => true,
            (Some(value), Some(reason)) => {
                value.kind == AgentObjectKind::Association && valid_label(reason, 1024)
            }
            _ => false,
        };
        if self.schema_version != 1
            || self.tool_call_ids.len() > MAX_REFS
            || self.changes.len() > MAX_ANCHORS
            || self.paths.len() > MAX_REFS
            || self.evidence.len() > MAX_REFS
            || (self.before.is_empty() && self.after.is_empty())
            || invalid_path
            || wrong_evidence
            || wrong_granularity
            || !correction_valid
            || (self.relation == AssociationRelation::ManualAssertion
                && self.evidence_method != EvidenceMethod::ManualAssertion)
        {
            return Err(Error::Invalid("code association"));
        }
        self.coverage.validate()
    }
    pub fn dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        Ok(self
            .evidence
            .iter()
            .copied()
            .chain(self.supersedes)
            .collect())
    }
    pub fn native_dependencies(&self) -> Result<BTreeSet<ObjectKey>> {
        self.validate()?;
        Ok(self
            .before
            .native_dependencies()
            .into_iter()
            .chain(self.after.native_dependencies())
            .collect())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CaptureStatusV1 {
    Complete,
    Partial,
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionOutcomeV1 {
    Succeeded,
    Failed,
    Cancelled,
    Interrupted,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentCheckpointVersionV1 {
    pub schema_version: u32,
    pub checkpoint_id: AgentCheckpointId,
    pub session_id: AgentSessionId,
    pub turn_id: Option<TurnId>,
    pub scope_id: Option<ProvenanceScopeId>,
    pub session_snapshot: AgentObjectRef,
    pub transcript_manifest: AgentObjectRef,
    pub event_batches: Vec<AgentObjectRef>,
    pub tools: BTreeMap<ToolCallId, AgentObjectRef>,
    pub before: CodeAnchorV1,
    pub after: CodeAnchorV1,
    pub code_relation: CodeRelation,
    pub associations: Vec<AgentObjectRef>,
    pub evidence_method: EvidenceMethod,
    pub source_trust: SourceTrust,
    pub exclusive_write_scope: bool,
    pub coverage: CoverageV1,
    pub capture_status: CaptureStatusV1,
    pub execution_outcome: ExecutionOutcomeV1,
    pub supersedes: Option<AgentObjectRef>,
}
impl AgentCheckpointVersionV1 {
    pub fn validate_code_relation(
        before: &CodeAnchorV1,
        after: &CodeAnchorV1,
        relation: CodeRelation,
    ) -> Result<()> {
        before.validate()?;
        after.validate()?;
        let valid = match relation {
            CodeRelation::Changed => !before.is_empty() && !after.is_empty() && before != after,
            CodeRelation::NoChange => !before.is_empty() && before == after,
            CodeRelation::NotObserved => before.is_empty() || after.is_empty(),
        };
        if !valid {
            return Err(Error::Invalid("checkpoint code relation"));
        }
        Ok(())
    }
    pub fn validate(&self) -> Result<()> {
        Self::validate_code_relation(&self.before, &self.after, self.code_relation)?;
        let complete = self.capture_status != CaptureStatusV1::Complete
            || self.coverage.completeness == CompletenessV1::Complete;
        if self.schema_version != 1
            || self.session_snapshot.kind != AgentObjectKind::SessionSnapshot
            || self.transcript_manifest.kind != AgentObjectKind::TranscriptManifest
            || self.event_batches.len() > MAX_REFS
            || self.tools.len() > MAX_REFS
            || self.associations.len() > MAX_REFS
            || self
                .event_batches
                .iter()
                .any(|value| value.kind != AgentObjectKind::EventBatch)
            || self
                .tools
                .values()
                .any(|value| value.kind != AgentObjectKind::ToolSnapshot)
            || self
                .associations
                .iter()
                .any(|value| value.kind != AgentObjectKind::Association)
            || self
                .supersedes
                .is_some_and(|value| value.kind != AgentObjectKind::CheckpointVersion)
            || !complete
        {
            return Err(Error::Invalid("agent checkpoint"));
        }
        self.coverage.validate()
    }
    pub fn dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        Ok([self.session_snapshot, self.transcript_manifest]
            .into_iter()
            .chain(self.event_batches.iter().copied())
            .chain(self.tools.values().copied())
            .chain(self.associations.iter().copied())
            .chain(self.supersedes)
            .collect())
    }
    pub fn native_dependencies(&self) -> Result<BTreeSet<ObjectKey>> {
        self.validate()?;
        Ok(self
            .before
            .native_dependencies()
            .into_iter()
            .chain(self.after.native_dependencies())
            .collect())
    }
}
