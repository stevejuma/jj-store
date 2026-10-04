//! Pinned native API/export observations; metadata never implies execution.
use super::{opencode, opencode_tools, string};
use crate::{
    AgentEntityRefV1, IntegrationId, MessageId, MessageRole, NativeEntityKind,
    NormalizationBatchV1, NormalizationDiagnosticKind, NormalizedActivityRecordV1,
    NormalizedMessageRecordV1, NormalizedSessionStateRecordV1, NormalizerProfileV1, Result,
    SanitizedObservationBatchV1, SanitizedObservationV1, SessionState, SourceKind, SourceTrust,
    alias, json_payload, scoped, stable_id,
};
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub(super) fn normalize(
    profile: &NormalizerProfileV1,
    batch: &SanitizedObservationBatchV1,
    observation: &SanitizedObservationV1,
    value: &Value,
    output: &mut NormalizationBatchV1,
) -> Result<()> {
    let sequence = observation.source_sequence;
    let api = (observation.source_kind, observation.source_trust)
        == (SourceKind::NativeApi, SourceTrust::ApprovedHarnessApi);
    let export = (observation.source_kind, observation.source_trust)
        == (SourceKind::SessionExport, SourceTrust::LocalTranscript);
    let Some(native_session) = string(value, &["session_id"]).filter(|id| native_id(id, "ses"))
    else {
        opencode::invalid(output, sequence, "opencode-snapshot-session");
        return Ok(());
    };
    if (!api && !export)
        || string(value, &["product_version"]) != Some(profile.product_version.as_str())
    {
        opencode::invalid(output, sequence, "opencode-snapshot-profile");
        return Ok(());
    }
    let record = &value["record"];
    match string(record, &["type"]) {
        Some("session_status") if api => {
            let state = match string(record, &["status", "type"]) {
                Some("busy" | "retry") => SessionState::Active,
                Some("idle") => SessionState::Idle,
                _ => {
                    opencode::invalid(output, sequence, "opencode-snapshot-status");
                    return Ok(());
                }
            };
            let session_id = opencode::session(batch, output, native_session);
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
        Some("message") => message(record, native_session, sequence, batch, output)?,
        _ => output.diagnostic(
            NormalizationDiagnosticKind::UnsupportedEvent,
            "opencode-snapshot-kind",
            Some(sequence),
        ),
    }
    Ok(())
}
fn native_id(value: &str, prefix: &str) -> bool {
    value.starts_with(prefix)
        && value.len() > prefix.len()
        && value.len() <= 128
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
}
fn message(
    record: &Value,
    native_session: &str,
    sequence: u64,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
) -> Result<()> {
    let info = &record["info"];
    let Some(native_message) = string(info, &["id"]).filter(|id| native_id(id, "msg")) else {
        opencode::invalid(output, sequence, "opencode-snapshot-message");
        return Ok(());
    };
    let role = match string(info, &["role"]) {
        Some("user") => MessageRole::User,
        Some("assistant") => MessageRole::Assistant,
        _ => {
            opencode::invalid(output, sequence, "opencode-snapshot-role");
            return Ok(());
        }
    };
    let Some(parts) = record.get("parts").and_then(Value::as_array) else {
        opencode::invalid(output, sequence, "opencode-snapshot-parts");
        return Ok(());
    };
    let mut ids = BTreeSet::new();
    if string(info, &["sessionID"]) != Some(native_session)
        || parts.len() > 256
        || parts.iter().any(|part| {
            string(part, &["sessionID"]) != Some(native_session)
                || string(part, &["messageID"]) != Some(native_message)
                || !string(part, &["id"]).is_some_and(|id| native_id(id, "prt") && ids.insert(id))
        })
    {
        opencode::invalid(output, sequence, "opencode-snapshot-owner");
        return Ok(());
    }
    let content = json_payload(&Value::Array(parts.clone()))?;
    let session_id = opencode::session(batch, output, native_session);
    let native = scoped(native_session, native_message);
    let message_id = MessageId::new(stable_id(
        batch.source_instance_id,
        IntegrationId::OpenCode,
        "message",
        &native,
    ));
    output.alias(alias(
        IntegrationId::OpenCode,
        batch.source_instance_id,
        NativeEntityKind::Message,
        native,
        AgentEntityRefV1::Message {
            session_id,
            message_id,
        },
    ));
    output.records.push(NormalizedActivityRecordV1::Message(
        NormalizedMessageRecordV1 {
            source_sequence: sequence,
            session_id,
            turn_id: None,
            message_id,
            role,
            content,
        },
    ));
    for part in parts {
        match string(part, &["type"]) {
            Some("tool") => tool(part, sequence, batch, output)?,
            Some("text" | "reasoning") if part.get("text").is_some_and(Value::is_string) => {}
            Some(
                "file" | "subtask" | "step-start" | "step-finish" | "snapshot" | "patch" | "agent"
                | "retry" | "compaction",
            ) => {}
            _ => output.diagnostic(
                NormalizationDiagnosticKind::UnsupportedEvent,
                "opencode-snapshot-part",
                Some(sequence),
            ),
        }
    }
    Ok(())
}
fn tool(
    part: &Value,
    sequence: u64,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
) -> Result<()> {
    let label = |name: &str| {
        string(part, &[name])
            .is_some_and(|s| !s.is_empty() && s.len() <= 512 && !s.chars().any(char::is_control))
    };
    let status = string(part, &["state", "status"]);
    if !label("callID")
        || !label("tool")
        || !part.pointer("/state/input").is_some_and(Value::is_object)
        || !matches!(status, Some("pending" | "running" | "completed" | "error"))
        || (status == Some("completed")
            && !part.pointer("/state/output").is_some_and(Value::is_string))
        || (status == Some("error") && !part.pointer("/state/error").is_some_and(Value::is_string))
    {
        opencode::invalid(output, sequence, "opencode-snapshot-tool");
        return Ok(());
    }
    let mut declared = part.clone();
    let state = declared["state"].as_object_mut().expect("validated state");
    if status != Some("completed") {
        state.remove("output");
    }
    if status != Some("error") {
        state.remove("error");
    }
    opencode_tools::tool_part(&json!({"part":declared}), sequence, batch, output)
}
