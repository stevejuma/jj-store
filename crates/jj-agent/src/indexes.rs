use crate::{
    AgentCatalogKey, AgentCheckpointId, AgentObjectKind, AgentObjectRef, AgentSessionId,
    AssociationId, CaptureBatchId, CoverageV1, Error, IntegrationId, MessageId, Result,
    SourceInstanceId, ToolCallId, TurnId, activity::valid_label,
};
use jj_store_model::{CommitId, OperationId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

const MAX_INDEX_SELECTIONS: usize = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NativeEntityKind {
    Session,
    Turn,
    Message,
    Tool,
    Checkpoint,
    Association,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "entity", rename_all = "snake_case", deny_unknown_fields)]
pub enum AgentEntityRefV1 {
    Session {
        session_id: AgentSessionId,
    },
    Tool {
        session_id: AgentSessionId,
        tool_call_id: ToolCallId,
    },
    Turn {
        session_id: AgentSessionId,
        turn_id: TurnId,
    },
    Message {
        session_id: AgentSessionId,
        message_id: MessageId,
    },
    Checkpoint {
        checkpoint_id: AgentCheckpointId,
    },
    Association {
        association_id: AssociationId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentAliasV1 {
    pub schema_version: u32,
    pub integration: IntegrationId,
    pub source_instance_id: SourceInstanceId,
    pub native_kind: NativeEntityKind,
    pub native_id: String,
    pub target: AgentEntityRefV1,
}
impl AgentAliasV1 {
    pub fn validate(&self) -> Result<()> {
        let target_matches = matches!(
            (self.native_kind, &self.target),
            (NativeEntityKind::Session, AgentEntityRefV1::Session { .. })
                | (NativeEntityKind::Turn, AgentEntityRefV1::Turn { .. })
                | (NativeEntityKind::Message, AgentEntityRefV1::Message { .. })
                | (NativeEntityKind::Tool, AgentEntityRefV1::Tool { .. })
                | (
                    NativeEntityKind::Checkpoint,
                    AgentEntityRefV1::Checkpoint { .. }
                )
                | (
                    NativeEntityKind::Association,
                    AgentEntityRefV1::Association { .. }
                )
        );
        if self.schema_version != 1 || !valid_label(&self.native_id, 1024) || !target_matches {
            return Err(Error::Invalid("agent alias"));
        }
        Ok(())
    }
    pub fn catalog_key(&self) -> Result<AgentCatalogKey> {
        self.validate()?;
        derive_key(
            "alias",
            &(
                self.integration,
                self.source_instance_id,
                self.native_kind,
                &self.native_id,
            ),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "subject", rename_all = "snake_case", deny_unknown_fields)]
pub enum AgentRelationSubjectV1 {
    Session {
        session_id: AgentSessionId,
    },
    Tool {
        session_id: AgentSessionId,
        tool_call_id: ToolCallId,
    },
    Operation {
        operation_id: OperationId,
    },
    Revision {
        commit_id: CommitId,
    },
    Checkpoint {
        checkpoint_id: AgentCheckpointId,
    },
}
impl AgentRelationSubjectV1 {
    pub fn catalog_key(&self) -> Result<AgentCatalogKey> {
        derive_key("relation", self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentRelationPageV1 {
    pub schema_version: u32,
    pub subject: AgentRelationSubjectV1,
    pub selections: Vec<AgentObjectRef>,
    pub next: Option<AgentObjectRef>,
}
impl AgentRelationPageV1 {
    pub fn validate(&self) -> Result<()> {
        let unique: BTreeSet<_> = self.selections.iter().collect();
        if self.schema_version != 1
            || self.selections.is_empty()
            || self.selections.len() > 256
            || unique.len() != self.selections.len()
            || self.selections.iter().any(|value| {
                !matches!(
                    value.kind,
                    AgentObjectKind::CheckpointVersion | AgentObjectKind::Association
                )
            })
            || self
                .next
                .is_some_and(|value| value.kind != AgentObjectKind::RelationPage)
        {
            return Err(Error::Invalid("agent relation page"));
        }
        Ok(())
    }
    pub fn dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        Ok(self.selections.iter().copied().chain(self.next).collect())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "position", rename_all = "snake_case", deny_unknown_fields)]
pub enum SourcePositionV1 {
    JsonLines {
        complete_byte_offset: u64,
        prefix_sha256: [u8; 32],
    },
    NativeSequence {
        sequence: u64,
    },
    Snapshot {
        generation_digest: [u8; 32],
        logical_records: u64,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureSourceReceiptV1 {
    pub batch_id: CaptureBatchId,
    pub first_sequence: u64,
    pub last_sequence: u64,
}
impl CaptureSourceReceiptV1 {
    fn validate(&self) -> Result<()> {
        if self.first_sequence > self.last_sequence {
            return Err(Error::Invalid("capture source receipt"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureSourceStateV1 {
    pub schema_version: u32,
    pub integration: IntegrationId,
    pub source_instance_id: SourceInstanceId,
    pub generation: u64,
    pub parser_version: String,
    pub native_session_key: Option<String>,
    pub position: SourcePositionV1,
    pub receipts: Vec<CaptureSourceReceiptV1>,
    pub coverage: CoverageV1,
}
impl CaptureSourceStateV1 {
    pub fn validate(&self) -> Result<()> {
        let mut batches = BTreeSet::new();
        let mut previous = None;
        if self.schema_version != 1
            || !valid_label(&self.parser_version, 128)
            || self
                .native_session_key
                .as_ref()
                .is_some_and(|value| !valid_label(value, 1024))
            || self.receipts.len() > MAX_INDEX_SELECTIONS
        {
            return Err(Error::Invalid("capture source state"));
        }
        for receipt in &self.receipts {
            receipt.validate()?;
            if !batches.insert(receipt.batch_id)
                || previous.is_some_and(|value| receipt.first_sequence <= value)
            {
                return Err(Error::Invalid("capture source receipt order"));
            }
            previous = Some(receipt.last_sequence);
        }
        self.coverage.validate()
    }
    pub fn catalog_key(&self) -> Result<AgentCatalogKey> {
        self.validate()?;
        derive_key("source", &(self.integration, self.source_instance_id))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentProjectionCatalogV1 {
    pub schema_version: u32,
    pub normalizer_version: String,
    pub activated_at_millis: i64,
    pub sessions: Vec<AgentObjectRef>,
    pub event_batches: Vec<AgentObjectRef>,
    pub source_states: Vec<AgentObjectRef>,
}
impl AgentProjectionCatalogV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1
            || !valid_label(&self.normalizer_version, 128)
            || self.sessions.len() > MAX_INDEX_SELECTIONS
            || self.event_batches.len() > MAX_INDEX_SELECTIONS
            || self.source_states.len() > MAX_INDEX_SELECTIONS
            || has_wrong_or_duplicate(&self.sessions, AgentObjectKind::SessionSnapshot)
            || has_wrong_or_duplicate(&self.event_batches, AgentObjectKind::EventBatch)
            || has_wrong_or_duplicate(&self.source_states, AgentObjectKind::SourceState)
        {
            return Err(Error::Invalid("agent projection catalog"));
        }
        Ok(())
    }
    pub fn catalog_key(&self) -> Result<AgentCatalogKey> {
        self.validate()?;
        AgentCatalogKey::derive("projection", self.normalizer_version.as_bytes())
    }
    pub fn dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        Ok(self
            .sessions
            .iter()
            .chain(&self.event_batches)
            .chain(&self.source_states)
            .copied()
            .collect())
    }
}

fn derive_key<T: Serialize + ?Sized>(namespace: &str, value: &T) -> Result<AgentCatalogKey> {
    AgentCatalogKey::derive(namespace, &serde_json::to_vec(value)?)
}
fn has_wrong_or_duplicate(values: &[AgentObjectRef], kind: AgentObjectKind) -> bool {
    values.iter().any(|value| value.kind != kind)
        || values.iter().collect::<BTreeSet<_>>().len() != values.len()
}
