//! Versioned ACP observations from an application-owned execution source.
use super::{string, tool_kind, value};
use crate::*;
pub(crate) fn normalize(
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
) -> Result<()> {
    for observation in &batch.observations {
        let Some(value) = value(observation) else {
            continue;
        };
        let Some(native) = string(&value, &["controllerSessionId"]) else {
            output.diagnostic(
                NormalizationDiagnosticKind::InvalidRecord,
                "controller-session",
                Some(observation.source_sequence),
            );
            continue;
        };
        let source = batch.source_instance_id;
        let integration = IntegrationId::OwnedController;
        let session_id = AgentSessionId::new(stable_id(source, integration, "session", native));
        output.alias(alias(
            integration,
            source,
            NativeEntityKind::Session,
            native.into(),
            AgentEntityRefV1::Session { session_id },
        ));
        let request = string(&value, &["requestId"]).unwrap_or("session");
        let turn_key = scoped(native, request);
        let turn_id = TurnId::new(stable_id(source, integration, "turn", &turn_key));
        output.alias(alias(
            integration,
            source,
            NativeEntityKind::Turn,
            turn_key.clone(),
            AgentEntityRefV1::Turn {
                session_id,
                turn_id,
            },
        ));
        let sequence = observation.source_sequence;
        let kind = string(&value, &["type"]).unwrap_or("");
        let state = match kind {
            "opened" => Some(SessionState::Idle),
            "started" => Some(SessionState::Active),
            "record" => Some(
                if matches!(
                    string(&value, &["record", "status"]),
                    Some("timed_out" | "outcome_unknown")
                ) || value["sessionUsable"] == false
                {
                    SessionState::Unknown
                } else {
                    SessionState::Idle
                },
            ),
            "closed" => Some(SessionState::Ended),
            _ => None,
        };
        if let Some(state) = state {
            output
                .records
                .push(NormalizedActivityRecordV1::SessionState(
                    NormalizedSessionStateRecordV1 {
                        source_sequence: sequence,
                        session_id,
                        state,
                    },
                ));
        }
        if matches!(kind, "prompt" | "context_delivery") {
            if let Some(text) = string(&value, &["text"]) {
                let rich = value
                    .get("sentContext")
                    .filter(|context| context["version"] == 1);
                let content = rich.map(|context| serde_json::json!({"schema":"jj-store.sent-context.v1","text":value["userText"],"context":context}).to_string());
                message(
                    output,
                    batch,
                    session_id,
                    turn_id,
                    &turn_key,
                    "user",
                    MessageRole::User,
                    content.as_deref().unwrap_or(text),
                    sequence,
                )?;
            }
        } else if kind == "record"
            && let Some(text) = string(&value, &["assistantText"]).filter(|text| !text.is_empty())
        {
            message(
                output,
                batch,
                session_id,
                turn_id,
                &turn_key,
                "assistant",
                MessageRole::Assistant,
                text,
                sequence,
            )?;
        }
        if kind == "activity"
            && let Some(tool) = value.get("toolSnapshot")
        {
            tool_record(
                output, batch, session_id, turn_id, &turn_key, tool, sequence,
            )?;
        }
        if matches!(kind, "permission" | "permission_resolved") {
            output.records.push(NormalizedActivityRecordV1::Permission(
                NormalizedPermissionRecordV1 {
                    source_sequence: sequence,
                    session_id,
                    tool_call_id: None,
                    state: if kind == "permission" {
                        PermissionState::Requested
                    } else {
                        match value["decisionKind"].as_str() {
                            Some("allow_once" | "allow_always") => PermissionState::Granted,
                            Some("reject_once" | "reject_always") => PermissionState::Denied,
                            _ if value.get("optionId").is_none_or(|v| v.is_null()) => {
                                PermissionState::Denied
                            }
                            _ => PermissionState::NotObserved,
                        }
                    },
                },
            ));
        }
    }
    Ok(())
}
#[allow(clippy::too_many_arguments)]
fn message(
    output: &mut NormalizationBatchV1,
    batch: &SanitizedObservationBatchV1,
    session_id: AgentSessionId,
    turn_id: TurnId,
    turn_key: &str,
    part: &str,
    role: MessageRole,
    text: &str,
    sequence: u64,
) -> Result<()> {
    let native = scoped(turn_key, part);
    let id = MessageId::new(stable_id(
        batch.source_instance_id,
        IntegrationId::OwnedController,
        "message",
        &native,
    ));
    output.alias(alias(
        IntegrationId::OwnedController,
        batch.source_instance_id,
        NativeEntityKind::Message,
        native,
        AgentEntityRefV1::Message {
            session_id,
            message_id: id,
        },
    ));
    output.records.push(NormalizedActivityRecordV1::Message(
        NormalizedMessageRecordV1 {
            source_sequence: sequence,
            session_id,
            turn_id: Some(turn_id),
            message_id: id,
            role,
            content: if role == MessageRole::User
                && serde_json::from_str::<serde_json::Value>(text)
                    .is_ok_and(|v| v["schema"] == "jj-store.sent-context.v1")
            {
                SanitizedPayloadV1::new(SanitizedMediaType::Json, text.as_bytes())?
            } else {
                text_payload(text)?
            },
        },
    ));
    Ok(())
}
fn tool_record(
    output: &mut NormalizationBatchV1,
    batch: &SanitizedObservationBatchV1,
    session_id: AgentSessionId,
    turn_id: TurnId,
    turn_key: &str,
    value: &serde_json::Value,
    sequence: u64,
) -> Result<()> {
    let Some(native_id) = string(value, &["toolCallId"]) else {
        return Ok(());
    };
    let native = scoped(turn_key, native_id);
    let tool_call_id = ToolCallId::new(stable_id(
        batch.source_instance_id,
        IntegrationId::OwnedController,
        "tool",
        &native,
    ));
    output.alias(alias(
        IntegrationId::OwnedController,
        batch.source_instance_id,
        NativeEntityKind::Tool,
        tool_alias_key(turn_key, native_id)?,
        AgentEntityRefV1::Tool {
            session_id,
            tool_call_id,
        },
    ));
    // ACP titles can be multiline commands. Keep the full title in the
    // observation; the immutable tool snapshot requires a bounded label.
    let name = tool_label(string(value, &["title"]).unwrap_or(native_id));
    let state = match string(value, &["status"]) {
        Some("completed") => ToolState::Succeeded,
        Some("failed") => ToolState::Failed,
        Some("pending") => ToolState::Unknown,
        _ => ToolState::Running,
    };
    output
        .records
        .push(NormalizedActivityRecordV1::Tool(NormalizedToolRecordV1 {
            source_sequence: sequence,
            tool_call_id,
            session_id,
            turn_id: Some(turn_id),
            native_name: name.clone(),
            kind: tool_kind(string(value, &["kind"]).unwrap_or(&name)),
            state,
            capture_state: CaptureState::Observed,
            permission_state: PermissionState::NotObserved,
            input: value.get("rawInput").map(json_payload).transpose()?,
            output: value.get("content").map(json_payload).transpose()?,
            conflicting_terminal_states: vec![],
        }));
    Ok(())
}
fn tool_alias_key(turn: &str, call: &str) -> Result<String> {
    let encoded = serde_json::to_string(&[turn, call])?;
    let key = format!("json:{encoded}");
    if key.len() <= 1024 {
        return Ok(key);
    }
    // Oversized opaque IDs remain exact in evidence. This documented key
    // derives from the full tuple rather than truncating or renaming the ID.
    use sha2::{Digest as _, Sha256};
    Ok(format!(
        "sha256:{}",
        hex::encode(Sha256::digest(encoded.as_bytes()))
    ))
}
fn tool_label(title: &str) -> String {
    let mut label = String::new();
    for c in title.chars().map(|c| if c.is_control() { ' ' } else { c }) {
        if label.len() + c.len_utf8() > 256 {
            break;
        }
        label.push(c);
    }
    if label.trim().is_empty() {
        "Tool call".into()
    } else {
        label
    }
}
