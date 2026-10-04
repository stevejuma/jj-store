//! Explicit sanitized source envelope; native evidence is never executed.
use super::{codex_ids as codex, string, tool_kind};
use crate::{
    AgentEntityRefV1, CaptureState, IntegrationId, MessageId, MessageRole, NativeEntityKind,
    NormalizationBatchV1, NormalizationDiagnosticKind, NormalizedActivityRecordV1,
    NormalizedMessageRecordV1, NormalizedSessionStateRecordV1, NormalizedToolRecordV1,
    NormalizerProfileV1, PermissionState, Result, SanitizedObservationBatchV1,
    SanitizedObservationV1, SessionState, SourceKind, SourceTrust, ToolCallId,
    ToolResultObservation, ToolState, activity::valid_label, alias, json_payload, scoped,
    stable_id, text_payload,
};
use serde_json::Value;
pub(super) fn record(
    profile: &NormalizerProfileV1,
    batch: &SanitizedObservationBatchV1,
    observation: &SanitizedObservationV1,
    envelope: &Value,
    output: &mut NormalizationBatchV1,
    tool_results: &mut Vec<ToolResultObservation>,
) -> Result<()> {
    let sequence = observation.source_sequence;
    let Some(native_session) = string(envelope, &["session_id"]).filter(|v| valid_label(v, 1024))
    else {
        invalid(output, sequence, "codex-transcript-session");
        return Ok(());
    };
    if envelope["capture_format"] != "codex-jsonl/v1"
        || envelope["product_version"].as_str() != Some(&profile.product_version)
        || observation.source_kind != SourceKind::Transcript
        || observation.source_trust != SourceTrust::LocalTranscript
    {
        invalid(output, sequence, "codex-transcript-envelope");
        return Ok(());
    }
    let Some(record) = envelope.get("record").filter(|v| v.is_object()) else {
        invalid(output, sequence, "codex-transcript-record");
        return Ok(());
    };
    let payload = &record["payload"];
    match (record["type"].as_str(), payload["type"].as_str()) {
        (Some("event_msg"), Some(event @ ("task_started" | "task_complete" | "turn_aborted"))) => {
            let session_id = codex::session(batch, output, native_session);
            output
                .records
                .push(NormalizedActivityRecordV1::SessionState(
                    NormalizedSessionStateRecordV1 {
                        source_sequence: sequence,
                        session_id,
                        state: if event == "task_started" {
                            SessionState::Active
                        } else {
                            SessionState::Idle
                        },
                    },
                ));
        }
        (Some("response_item"), Some("message")) => {
            message(payload, sequence, batch, output, native_session)?
        }
        (Some("response_item"), Some("function_call" | "custom_tool_call")) => {
            tool(payload, sequence, batch, output, native_session)?
        }
        (Some("response_item"), Some("function_call_output" | "custom_tool_call_output")) => {
            super::codex_tool_result::record(
                payload,
                sequence,
                batch,
                output,
                native_session,
                tool_results,
            )?;
        }
        _ => output.diagnostic(
            NormalizationDiagnosticKind::UnsupportedEvent,
            "codex-transcript-event",
            Some(sequence),
        ),
    }
    Ok(())
}
pub(super) fn invalid(output: &mut NormalizationBatchV1, sequence: u64, code: &str) {
    output.diagnostic(
        NormalizationDiagnosticKind::InvalidRecord,
        code,
        Some(sequence),
    );
}
fn message(
    payload: &Value,
    sequence: u64,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
    native_session: &str,
) -> Result<()> {
    let role = match payload["role"].as_str() {
        Some("user") => MessageRole::User,
        Some("assistant") => MessageRole::Assistant,
        _ => {
            invalid(output, sequence, "codex-transcript-message-role");
            return Ok(());
        }
    };
    let Some(content) = payload["content"].as_array() else {
        invalid(output, sequence, "codex-transcript-message-content");
        return Ok(());
    };
    let mut texts = Vec::new();
    for item in content {
        match (item["type"].as_str(), item["text"].as_str()) {
            (Some("input_text" | "output_text"), Some(text)) => texts.push(text),
            _ => output.diagnostic(
                NormalizationDiagnosticKind::UnsupportedEvent,
                "codex-transcript-message-block",
                Some(sequence),
            ),
        }
    }
    if texts.is_empty() {
        invalid(output, sequence, "codex-transcript-message-text");
        return Ok(());
    }
    let (native_item, identity_kind, native_alias) = match payload.get("id") {
        Some(Value::String(id)) if valid_label(id, 1024) => (id.clone(), "message", true),
        Some(Value::Null) | None => (format!("source-record/{sequence}"), "source-message", false),
        _ => {
            invalid(output, sequence, "codex-transcript-message-id");
            return Ok(());
        }
    };
    let native = scoped(native_session, &native_item);
    let session_id = codex::session(batch, output, native_session);
    let message_id = MessageId::new(stable_id(
        batch.source_instance_id,
        IntegrationId::Codex,
        identity_kind,
        &native,
    ));
    if native_alias {
        output.alias(alias(
            IntegrationId::Codex,
            batch.source_instance_id,
            NativeEntityKind::Message,
            native,
            AgentEntityRefV1::Message {
                session_id,
                message_id,
            },
        ));
    }
    let turn_id = string(payload, &["turn_id"])
        .filter(|v| valid_label(v, 1024))
        .map(|v| codex::turn_id(batch, output, native_session, v));
    output.records.push(NormalizedActivityRecordV1::Message(
        NormalizedMessageRecordV1 {
            source_sequence: sequence,
            session_id,
            turn_id,
            message_id,
            role,
            content: text_payload(&texts.join("\n"))?,
        },
    ));
    Ok(())
}
fn tool(
    payload: &Value,
    sequence: u64,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
    native_session: &str,
) -> Result<()> {
    let (Some(call), Some(name)) = (
        string(payload, &["call_id"]).filter(|v| valid_label(v, 1024)),
        string(payload, &["name"]).filter(|v| valid_label(v, 256)),
    ) else {
        invalid(output, sequence, "codex-transcript-tool-call");
        return Ok(());
    };
    let input = payload.get("arguments").or_else(|| payload.get("input"));
    let input = input
        .map(|v| match v {
            Value::String(text) => match serde_json::from_str::<Value>(text) {
                Ok(value) => json_payload(&value),
                Err(_) => text_payload(text),
            },
            _ => json_payload(v),
        })
        .transpose()?;
    let native = scoped(native_session, call);
    if !valid_label(&native, 1024) {
        invalid(output, sequence, "codex-transcript-tool-call-alias");
        return Ok(());
    }
    let session_id = codex::session(batch, output, native_session);
    let tool_call_id = ToolCallId::new(stable_id(
        batch.source_instance_id,
        IntegrationId::Codex,
        "tool",
        &native,
    ));
    output.alias(alias(
        IntegrationId::Codex,
        batch.source_instance_id,
        NativeEntityKind::Tool,
        native,
        AgentEntityRefV1::Tool {
            session_id,
            tool_call_id,
        },
    ));
    let stripped = name.strip_prefix("functions.").unwrap_or(name);
    let kind = if stripped == "exec_command" {
        crate::ToolKind::Execute
    } else {
        tool_kind(stripped)
    };
    output
        .records
        .push(NormalizedActivityRecordV1::Tool(NormalizedToolRecordV1 {
            source_sequence: sequence,
            tool_call_id,
            session_id,
            turn_id: None,
            native_name: name.into(),
            kind,
            state: ToolState::Pending,
            capture_state: CaptureState::Observed,
            permission_state: PermissionState::NotObserved,
            input,
            output: None,
            conflicting_terminal_states: Vec::new(),
        }));
    Ok(())
}
