use super::models::{AgentCoverage, AgentToolLink};
use jj_store_model as m;
use jj_store_wal::publication::{
    AgentCheckpointId, AgentCheckpointVersionV1, AgentObjectId, AgentObjectRef, AgentSessionId,
    CaptureStatusV1, CodeAnchorV1, CodeRelation, EvidenceMethod, ExecutionOutcomeV1,
    ProvenanceScopeId, SourceTrust, TurnId,
};
use schemars::JsonSchema;
use serde::Serialize;
use std::collections::BTreeSet;

pub type AgentCheckpointVersionIdentity = AgentObjectId;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentCodeAnchor {
    pub operations: BTreeSet<m::OperationId>,
    pub commits: BTreeSet<m::CommitId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentCheckpointSummary {
    pub checkpoint_id: AgentCheckpointId,
    pub version_id: AgentObjectId,
    pub session_id: AgentSessionId,
    pub turn_id: Option<TurnId>,
    pub scope_id: Option<ProvenanceScopeId>,
    pub before: AgentCodeAnchor,
    pub after: AgentCodeAnchor,
    pub code_relation: CodeRelation,
    pub evidence_method: EvidenceMethod,
    pub source_trust: SourceTrust,
    pub exclusive_write_scope: bool,
    pub coverage: AgentCoverage,
    pub capture_status: CaptureStatusV1,
    pub execution_outcome: ExecutionOutcomeV1,
    pub event_batch_count: usize,
    pub tool_count: usize,
    pub association_count: usize,
    pub supersedes: Option<AgentObjectId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentCheckpointPage {
    pub version: u32,
    pub snapshot: super::AgentReadSnapshot,
    pub checkpoints: Vec<AgentCheckpointSummary>,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct AgentCheckpointDetail {
    pub version: u32,
    pub snapshot: super::AgentReadSnapshot,
    pub checkpoint: AgentCheckpointSummary,
    pub session_snapshot_id: AgentObjectId,
    pub transcript_id: AgentObjectId,
    pub event_batches: Vec<AgentObjectId>,
    pub tools: Vec<AgentToolLink>,
    pub associations: Vec<AgentObjectId>,
}

fn anchor(value: &CodeAnchorV1) -> AgentCodeAnchor {
    AgentCodeAnchor {
        operations: value.operations.clone(),
        commits: value.commits.clone(),
    }
}

pub(super) fn summary(
    reference: AgentObjectRef,
    value: &AgentCheckpointVersionV1,
) -> AgentCheckpointSummary {
    AgentCheckpointSummary {
        checkpoint_id: value.checkpoint_id,
        version_id: reference.id,
        session_id: value.session_id,
        turn_id: value.turn_id,
        scope_id: value.scope_id,
        before: anchor(&value.before),
        after: anchor(&value.after),
        code_relation: value.code_relation,
        evidence_method: value.evidence_method,
        source_trust: value.source_trust,
        exclusive_write_scope: value.exclusive_write_scope,
        coverage: AgentCoverage {
            completeness: value.coverage.completeness,
            gaps: value.coverage.gaps,
            redacted_fields: value.coverage.redacted_fields,
            truncated: value.coverage.truncated,
        },
        capture_status: value.capture_status,
        execution_outcome: value.execution_outcome,
        event_batch_count: value.event_batches.len(),
        tool_count: value.tools.len(),
        association_count: value.associations.len(),
        supersedes: value.supersedes.map(|reference| reference.id),
    }
}

pub(super) fn detail(
    snapshot: super::AgentReadSnapshot,
    reference: AgentObjectRef,
    value: AgentCheckpointVersionV1,
) -> AgentCheckpointDetail {
    let checkpoint = summary(reference, &value);
    AgentCheckpointDetail {
        version: 1,
        snapshot,
        checkpoint,
        session_snapshot_id: value.session_snapshot.id,
        transcript_id: value.transcript_manifest.id,
        event_batches: value
            .event_batches
            .into_iter()
            .map(|item| item.id)
            .collect(),
        tools: value
            .tools
            .into_iter()
            .map(|(tool_call_id, snapshot)| AgentToolLink {
                tool_call_id,
                snapshot_id: snapshot.id,
            })
            .collect(),
        associations: value.associations.into_iter().map(|item| item.id).collect(),
    }
}
