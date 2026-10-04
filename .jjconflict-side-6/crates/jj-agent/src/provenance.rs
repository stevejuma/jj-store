use crate::{
    AgentCheckpointFactsV1, AgentCheckpointId, AgentObjectRef, AgentSessionId, AssociationId,
    CodeAssociationFactsV1, CoverageV1, Error, ProvenanceScopeId, Result, SourceTrust, ToolCallId,
    TurnId,
};
use jj_store_model::{ChangeId, CommitId, ObjectKey, OperationId};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

const MAX_ANCHORS: usize = 256;

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
    pub fn facts(&self) -> CodeAssociationFactsV1 {
        CodeAssociationFactsV1 {
            association_id: self.association_id,
            checkpoint_id: self.checkpoint_id,
            session_id: self.session_id,
            turn_id: self.turn_id,
            tool_call_ids: self.tool_call_ids.clone(),
            scope_id: self.scope_id,
            relation: self.relation,
            changes: self.changes.clone(),
            paths: self.paths.clone(),
            granularity: self.granularity,
            evidence_method: self.evidence_method,
            source_trust: self.source_trust,
            exclusive_write_scope: self.exclusive_write_scope,
            evidence: self.evidence.clone(),
            coverage: self.coverage.clone(),
            created_at_millis: self.created_at_millis,
            supersedes: self.supersedes,
            correction_reason: self.correction_reason.clone(),
        }
    }
    pub fn validate(&self) -> Result<()> {
        self.before.validate()?;
        self.after.validate()?;
        self.facts().validate()?;
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
        if self.schema_version != 1
            || (self.before.is_empty() && self.after.is_empty())
            || wrong_granularity
        {
            return Err(Error::Invalid("code association"));
        }
        Ok(())
    }
    pub fn dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        self.facts().dependencies()
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
    pub fn facts(&self) -> AgentCheckpointFactsV1 {
        AgentCheckpointFactsV1 {
            checkpoint_id: self.checkpoint_id,
            session_id: self.session_id,
            turn_id: self.turn_id,
            scope_id: self.scope_id,
            session_snapshot: self.session_snapshot,
            transcript_manifest: self.transcript_manifest,
            event_batches: self.event_batches.clone(),
            tools: self.tools.clone(),
            associations: self.associations.clone(),
            evidence_method: self.evidence_method,
            source_trust: self.source_trust,
            exclusive_write_scope: self.exclusive_write_scope,
            coverage: self.coverage.clone(),
            capture_status: self.capture_status,
            execution_outcome: self.execution_outcome,
            supersedes: self.supersedes,
        }
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1 {
            return Err(Error::Invalid("agent checkpoint"));
        }
        Self::validate_code_relation(&self.before, &self.after, self.code_relation)?;
        self.facts().validate()
    }
    pub fn dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        self.facts().dependencies()
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
