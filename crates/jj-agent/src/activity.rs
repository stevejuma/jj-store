use crate::{
    AgentObjectKind, AgentObjectRef, AgentSessionId, Error, ExecutionAttemptId, Result, ToolCallId,
    TurnId,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationId {
    Codex,
    Cursor,
    OpenCode,
    OwnedController,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompletenessV1 {
    Complete,
    Partial,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CoverageV1 {
    pub completeness: CompletenessV1,
    pub gaps: u32,
    pub redacted_fields: u32,
    pub truncated: bool,
}
impl CoverageV1 {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.completeness == CompletenessV1::Complete && (self.gaps != 0 || self.truncated) {
            return Err(Error::Invalid("complete agent coverage has gaps"));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SessionState {
    Discovered,
    Active,
    Idle,
    Ended,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentSessionSnapshotV1 {
    pub schema_version: u32,
    pub session_id: AgentSessionId,
    pub integration: IntegrationId,
    pub state: SessionState,
    pub parent_session_id: Option<AgentSessionId>,
    pub evidence: Vec<AgentObjectRef>,
    pub event_batches: Vec<AgentObjectRef>,
    pub transcripts: Vec<AgentObjectRef>,
    pub tools: BTreeMap<ToolCallId, AgentObjectRef>,
    pub coverage: CoverageV1,
}
impl AgentSessionSnapshotV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1
            || self.evidence.len() > 4096
            || self.event_batches.len() > 4096
            || self.transcripts.len() > 64
            || self.tools.len() > 16_384
            || self
                .evidence
                .iter()
                .any(|v| v.kind != AgentObjectKind::ObservationBatch)
            || self
                .event_batches
                .iter()
                .any(|v| v.kind != AgentObjectKind::EventBatch)
            || self
                .transcripts
                .iter()
                .any(|v| v.kind != AgentObjectKind::TranscriptManifest)
            || self
                .tools
                .values()
                .any(|v| v.kind != AgentObjectKind::ToolSnapshot)
        {
            return Err(Error::Invalid("agent session snapshot"));
        }
        self.coverage.validate()
    }
    pub fn dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        Ok(self
            .evidence
            .iter()
            .chain(&self.event_batches)
            .chain(&self.transcripts)
            .chain(self.tools.values())
            .copied()
            .collect())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ToolKind {
    Read,
    Edit,
    Delete,
    Move,
    Search,
    Execute,
    Fetch,
    Plan,
    Delegate,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ToolState {
    Pending,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    Interrupted,
    Skipped,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ToolInvocationSnapshotV1 {
    pub schema_version: u32,
    pub tool_call_id: ToolCallId,
    pub session_id: AgentSessionId,
    pub turn_id: Option<TurnId>,
    pub native_name: String,
    pub kind: ToolKind,
    pub state: ToolState,
    pub attempts: BTreeSet<ExecutionAttemptId>,
    pub input: Option<AgentObjectRef>,
    pub output: Option<AgentObjectRef>,
    pub evidence: Vec<AgentObjectRef>,
}
impl ToolInvocationSnapshotV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1
            || !valid_label(&self.native_name, 256)
            || self.attempts.len() > 64
            || self.evidence.len() > 1024
            || [self.input, self.output]
                .into_iter()
                .flatten()
                .any(|v| v.kind != AgentObjectKind::Content)
            || self
                .evidence
                .iter()
                .any(|v| v.kind != AgentObjectKind::ObservationBatch)
        {
            return Err(Error::Invalid("tool snapshot"));
        }
        Ok(())
    }
    pub fn dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        Ok([self.input, self.output]
            .into_iter()
            .flatten()
            .chain(self.evidence.iter().copied())
            .collect())
    }
}

pub(crate) fn valid_label(value: &str, max: usize) -> bool {
    !value.is_empty() && value.len() <= max && !value.chars().any(|c| c.is_control())
}
