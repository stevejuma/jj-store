use crate::{
    AgentEventId, AgentObjectKind, AgentObjectRef, AgentSessionId, CaptureBatchId, Error,
    IntegrationId, MessageId, PermissionState, Result, SessionState, SourceInstanceId, ToolCallId,
    ToolState, TurnId, activity::valid_label,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

const MAX_SANITIZED_PAYLOAD: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SanitizedMediaType {
    Json,
    Text,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SanitizedPayloadV1 {
    media_type: SanitizedMediaType,
    bytes: Vec<u8>,
}
impl SanitizedPayloadV1 {
    pub fn new(media_type: SanitizedMediaType, bytes: &[u8]) -> Result<Self> {
        if bytes.len() > MAX_SANITIZED_PAYLOAD {
            return Err(Error::Limit);
        }
        let bytes = match media_type {
            SanitizedMediaType::Json => {
                let value: serde_json::Value = serde_json::from_slice(bytes)?;
                serde_json::to_vec(&value)?
            }
            SanitizedMediaType::Text => {
                std::str::from_utf8(bytes)
                    .map_err(|_| Error::Invalid("sanitized payload utf-8"))?;
                bytes.to_vec()
            }
        };
        Ok(Self { media_type, bytes })
    }
    pub fn media_type(&self) -> SanitizedMediaType {
        self.media_type
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub(crate) fn validate(&self) -> Result<()> {
        if &Self::new(self.media_type, &self.bytes)? != self {
            return Err(Error::Invalid("noncanonical sanitized payload"));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SanitizedRepresentation {
    SanitizedNative,
    CanonicalProducer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    Hook,
    Plugin,
    NativeApi,
    SessionExport,
    Transcript,
    Controller,
    Manual,
    WorkspaceObservation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SourceTrust {
    AuthenticatedController,
    LocalUserHook,
    LocallyTrustedPlugin,
    ApprovedHarnessApi,
    LocalTranscript,
    ImportedArchive,
    UntrustedAssertion,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SanitizedObservationV1 {
    pub observation_id: crate::ObservationId,
    pub source_sequence: u64,
    pub occurred_at_millis: Option<i64>,
    pub source_kind: SourceKind,
    pub source_trust: SourceTrust,
    pub payload: SanitizedPayloadV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SanitizedObservationBatchV1 {
    pub schema_version: u32,
    pub batch_id: CaptureBatchId,
    pub integration: IntegrationId,
    pub source_instance_id: SourceInstanceId,
    pub sanitizer_version: String,
    pub representation: SanitizedRepresentation,
    pub observations: Vec<SanitizedObservationV1>,
}
impl SanitizedObservationBatchV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1
            || !valid_label(&self.sanitizer_version, 128)
            || self.observations.is_empty()
            || self.observations.len() > 256
        {
            return Err(Error::Invalid("sanitized observation batch"));
        }
        let mut previous = None;
        let mut ids = BTreeSet::new();
        for observation in &self.observations {
            if previous.is_some_and(|value| observation.source_sequence <= value)
                || !ids.insert(observation.observation_id)
            {
                return Err(Error::Invalid("sanitized observation order"));
            }
            observation.payload.validate()?;
            previous = Some(observation.source_sequence);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    User,
    Assistant,
    System,
    Tool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum AgentEventV1 {
    SessionStateChanged {
        event_id: AgentEventId,
        session_id: AgentSessionId,
        state: SessionState,
    },
    MessageCompleted {
        event_id: AgentEventId,
        session_id: AgentSessionId,
        turn_id: Option<TurnId>,
        message_id: MessageId,
        role: MessageRole,
        content: AgentObjectRef,
    },
    ToolStateChanged {
        event_id: AgentEventId,
        session_id: AgentSessionId,
        turn_id: Option<TurnId>,
        tool_call_id: ToolCallId,
        state: ToolState,
    },
    PermissionObserved {
        event_id: AgentEventId,
        session_id: AgentSessionId,
        tool_call_id: Option<ToolCallId>,
        state: PermissionState,
    },
    FileObserved {
        event_id: AgentEventId,
        session_id: AgentSessionId,
        path: String,
    },
    CaptureGap {
        event_id: AgentEventId,
        session_id: AgentSessionId,
        missing_from: u64,
        missing_to: u64,
    },
}
impl AgentEventV1 {
    fn id(&self) -> AgentEventId {
        match self {
            Self::SessionStateChanged { event_id, .. }
            | Self::MessageCompleted { event_id, .. }
            | Self::ToolStateChanged { event_id, .. }
            | Self::PermissionObserved { event_id, .. }
            | Self::FileObserved { event_id, .. }
            | Self::CaptureGap { event_id, .. } => *event_id,
        }
    }
    fn validate(&self) -> Result<()> {
        match self {
            Self::MessageCompleted { content, .. } if content.kind != AgentObjectKind::Content => {
                Err(Error::Invalid("message content kind"))
            }
            Self::CaptureGap {
                missing_from,
                missing_to,
                ..
            } if missing_from >= missing_to => Err(Error::Invalid("capture gap")),
            Self::FileObserved { path, .. } if !valid_label(path, 4096) => {
                Err(Error::Invalid("file observation path"))
            }
            _ => Ok(()),
        }
    }
    fn dependencies(&self) -> impl Iterator<Item = AgentObjectRef> {
        match self {
            Self::MessageCompleted { content, .. } => Some(*content),
            _ => None,
        }
        .into_iter()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentEventBatchV1 {
    pub schema_version: u32,
    pub normalizer_version: String,
    pub evidence: AgentObjectRef,
    pub events: Vec<AgentEventV1>,
}
impl AgentEventBatchV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1
            || !valid_label(&self.normalizer_version, 128)
            || self.evidence.kind != AgentObjectKind::ObservationBatch
            || self.events.is_empty()
            || self.events.len() > 4096
        {
            return Err(Error::Invalid("agent event batch"));
        }
        let mut ids = BTreeSet::new();
        for event in &self.events {
            event.validate()?;
            if !ids.insert(event.id()) {
                return Err(Error::Invalid("duplicate agent event"));
            }
        }
        Ok(())
    }
    pub fn dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        Ok(std::iter::once(self.evidence)
            .chain(self.events.iter().flat_map(AgentEventV1::dependencies))
            .collect())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentContentV1 {
    pub schema_version: u32,
    pub payload: SanitizedPayloadV1,
}
impl AgentContentV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1 {
            return Err(Error::Invalid("agent content schema"));
        }
        self.payload.validate()
    }
}
