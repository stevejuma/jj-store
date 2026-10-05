//! Shared non-code provenance fields. Frozen V1 wire layouts stay unchanged.
use crate::activity::valid_label;
use crate::*;
use jj_store_model::ChangeId;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
const MAX_ANCHORS: usize = 256;
const MAX_REFS: usize = 4096;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CodeAssociationFactsV1 {
    pub association_id: AssociationId,
    pub checkpoint_id: Option<AgentCheckpointId>,
    pub session_id: AgentSessionId,
    pub turn_id: Option<TurnId>,
    pub tool_call_ids: BTreeSet<ToolCallId>,
    pub scope_id: Option<ProvenanceScopeId>,
    pub relation: AssociationRelation,
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
impl CodeAssociationFactsV1 {
    pub fn validate(&self) -> Result<()> {
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
        let correction_valid = match (self.supersedes, &self.correction_reason) {
            (None, None) => true,
            (Some(value), Some(reason)) => {
                value.kind == AgentObjectKind::Association && valid_label(reason, 1024)
            }
            _ => false,
        };
        if self.tool_call_ids.len() > MAX_REFS
            || self.changes.len() > MAX_ANCHORS
            || self.paths.len() > MAX_REFS
            || self.evidence.len() > MAX_REFS
            || invalid_path
            || wrong_evidence
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
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentCheckpointFactsV1 {
    pub checkpoint_id: AgentCheckpointId,
    pub session_id: AgentSessionId,
    pub turn_id: Option<TurnId>,
    pub scope_id: Option<ProvenanceScopeId>,
    pub session_snapshot: AgentObjectRef,
    pub transcript_manifest: AgentObjectRef,
    pub event_batches: Vec<AgentObjectRef>,
    pub tools: BTreeMap<ToolCallId, AgentObjectRef>,
    pub associations: Vec<AgentObjectRef>,
    pub evidence_method: EvidenceMethod,
    pub source_trust: SourceTrust,
    pub exclusive_write_scope: bool,
    pub coverage: CoverageV1,
    pub capture_status: CaptureStatusV1,
    pub execution_outcome: ExecutionOutcomeV1,
    pub supersedes: Option<AgentObjectRef>,
}
impl AgentCheckpointFactsV1 {
    pub fn validate(&self) -> Result<()> {
        let complete = self.capture_status != CaptureStatusV1::Complete
            || self.coverage.completeness == CompletenessV1::Complete;
        if self.session_snapshot.kind != AgentObjectKind::SessionSnapshot
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
}
