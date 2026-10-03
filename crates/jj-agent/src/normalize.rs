use crate::{
    AgentAliasV1, AgentEntityRefV1, AgentSessionId, IntegrationId, MessageId, MessageRole,
    NativeEntityKind, Result, SanitizedPayloadV1, SessionState, SourceInstanceId, ToolCallId,
    ToolKind, ToolState, TurnId,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NormalizerProfileV1 {
    pub integration: IntegrationId,
    pub product_version: String,
    pub schema_version: u32,
}
impl NormalizerProfileV1 {
    pub fn codex_0_156_1() -> Self {
        Self::new(IntegrationId::Codex, "0.156.1")
    }
    pub fn codex_desktop_0_159_2() -> Self {
        Self::new(IntegrationId::Codex, "0.159.2")
    }
    pub fn cursor_3_22_7() -> Self {
        Self::new(IntegrationId::Cursor, "3.22.7")
    }
    pub fn opencode_1_18_30() -> Self {
        Self::new(IntegrationId::OpenCode, "1.18.30")
    }
    fn new(integration: IntegrationId, version: &str) -> Self {
        Self {
            integration,
            product_version: version.into(),
            schema_version: 1,
        }
    }
    pub fn identity(&self) -> String {
        let integration = match self.integration {
            IntegrationId::Codex => "codex",
            IntegrationId::Cursor => "cursor",
            IntegrationId::OpenCode => "opencode",
            IntegrationId::OwnedController => "owned-controller",
        };
        format!(
            "{integration}@{}/v{}",
            self.product_version, self.schema_version
        )
    }
    pub fn validate(&self) -> Result<()> {
        if self.supported() {
            Ok(())
        } else {
            Err(crate::Error::Invalid("normalizer profile"))
        }
    }
    pub(crate) fn supported(&self) -> bool {
        self.schema_version == 1
            && matches!(
                (self.integration, self.product_version.as_str()),
                (IntegrationId::Codex, "0.156.1")
                    | (IntegrationId::Codex, "0.159.2")
                    | (IntegrationId::Cursor, "3.22.7")
                    | (IntegrationId::OpenCode, "1.18.30")
            )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NormalizationDiagnosticKind {
    UnsupportedVersion,
    IntegrationMismatch,
    EmbeddedVersionMismatch,
    InvalidRecord,
    UnsupportedEvent,
    ConflictingTerminalState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NormalizationDiagnosticV1 {
    pub kind: NormalizationDiagnosticKind,
    pub code: String,
    pub source_sequence: Option<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CaptureState {
    Observed,
    Partial,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PermissionState {
    NotObserved,
    Requested,
    Granted,
    Denied,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NormalizedSessionStateRecordV1 {
    pub source_sequence: u64,
    pub session_id: AgentSessionId,
    pub state: SessionState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NormalizedMessageRecordV1 {
    pub source_sequence: u64,
    pub session_id: AgentSessionId,
    pub turn_id: Option<TurnId>,
    pub message_id: MessageId,
    pub role: MessageRole,
    pub content: SanitizedPayloadV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NormalizedToolRecordV1 {
    pub source_sequence: u64,
    pub tool_call_id: ToolCallId,
    pub session_id: AgentSessionId,
    pub turn_id: Option<TurnId>,
    pub native_name: String,
    pub kind: ToolKind,
    pub state: ToolState,
    pub capture_state: CaptureState,
    pub permission_state: PermissionState,
    pub input: Option<SanitizedPayloadV1>,
    pub output: Option<SanitizedPayloadV1>,
    pub conflicting_terminal_states: Vec<ToolState>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NormalizedPermissionRecordV1 {
    pub source_sequence: u64,
    pub session_id: AgentSessionId,
    pub tool_call_id: Option<ToolCallId>,
    pub state: PermissionState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NormalizedFileObservationV1 {
    pub source_sequence: u64,
    pub session_id: AgentSessionId,
    pub path: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum NormalizedActivityRecordV1 {
    SessionState(NormalizedSessionStateRecordV1),
    Message(NormalizedMessageRecordV1),
    Tool(NormalizedToolRecordV1),
    Permission(NormalizedPermissionRecordV1),
    FileObservation(NormalizedFileObservationV1),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NormalizationBatchV1 {
    pub schema_version: u32,
    pub profile: NormalizerProfileV1,
    pub aliases: Vec<AgentAliasV1>,
    pub records: Vec<NormalizedActivityRecordV1>,
    pub diagnostics: Vec<NormalizationDiagnosticV1>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReducedToolsV1 {
    pub tools: Vec<NormalizedToolRecordV1>,
    pub diagnostics: Vec<NormalizationDiagnosticV1>,
}

impl NormalizationBatchV1 {
    pub(crate) fn diagnostic(
        &mut self,
        kind: NormalizationDiagnosticKind,
        code: &str,
        sequence: Option<u64>,
    ) {
        self.diagnostics.push(NormalizationDiagnosticV1 {
            kind,
            code: code.into(),
            source_sequence: sequence,
        });
    }
    pub(crate) fn alias(&mut self, alias: AgentAliasV1) {
        if !self.aliases.contains(&alias) {
            self.aliases.push(alias);
        }
    }
}

pub(crate) fn stable_id(
    source: SourceInstanceId,
    integration: IntegrationId,
    kind: &str,
    native: &str,
) -> [u8; 16] {
    let mut hash = Sha256::new_with_prefix(b"jj-store/agent-normalized-id/v1\0");
    hash.update(source.as_bytes());
    hash.update(match integration {
        IntegrationId::Codex => b"codex".as_slice(),
        IntegrationId::Cursor => b"cursor".as_slice(),
        IntegrationId::OpenCode => b"opencode".as_slice(),
        IntegrationId::OwnedController => b"owned-controller".as_slice(),
    });
    hash.update((kind.len() as u64).to_be_bytes());
    hash.update(kind.as_bytes());
    hash.update((native.len() as u64).to_be_bytes());
    hash.update(native.as_bytes());
    hash.finalize()[..16].try_into().expect("sha256 prefix")
}

pub fn normalized_event_id(
    source: SourceInstanceId,
    integration: IntegrationId,
    source_sequence: u64,
    kind: &str,
) -> crate::AgentEventId {
    crate::AgentEventId::new(stable_id(
        source,
        integration,
        "event",
        &format!("{source_sequence}/{kind}"),
    ))
}

pub(crate) fn scoped(session: &str, native: &str) -> String {
    format!("{session}/{native}")
}

pub(crate) fn alias(
    integration: IntegrationId,
    source: SourceInstanceId,
    kind: NativeEntityKind,
    native: String,
    target: AgentEntityRefV1,
) -> AgentAliasV1 {
    AgentAliasV1 {
        schema_version: 1,
        integration,
        source_instance_id: source,
        native_kind: kind,
        native_id: native,
        target,
    }
}

pub(crate) fn json_payload(value: &serde_json::Value) -> Result<SanitizedPayloadV1> {
    SanitizedPayloadV1::new(crate::SanitizedMediaType::Json, &serde_json::to_vec(value)?)
}

pub(crate) fn text_payload(value: &str) -> Result<SanitizedPayloadV1> {
    SanitizedPayloadV1::new(crate::SanitizedMediaType::Text, value.as_bytes())
}
