//! Qualified JSONL rows: source-derived identities never masquerade as native IDs.
use super::{
    cursor::{invalid, session},
    string, tool_kind,
};
use crate::{
    CaptureState, IntegrationId, MessageId, MessageRole, NormalizationBatchV1,
    NormalizedActivityRecordV1, NormalizedMessageRecordV1, NormalizedSessionStateRecordV1,
    NormalizedToolRecordV1, NormalizerProfileV1, PermissionState, Result,
    SanitizedObservationBatchV1, SanitizedObservationV1, SessionState, SourceKind, SourceTrust,
    ToolCallId, ToolState, json_payload, stable_id, text_payload,
};
use serde_json::Value;
pub(super) fn record(
    profile: &NormalizerProfileV1,
    batch: &SanitizedObservationBatchV1,
    observation: &SanitizedObservationV1,
    v: &Value,
    out: &mut NormalizationBatchV1,
) -> Result<()> {
    let seq = observation.source_sequence;
    let (Some(native), Some(key)) = (string(v, &["session_id"]), string(v, &["record_key"])) else {
        invalid(out, seq, "cursor-transcript-identity");
        return Ok(());
    };
    if v["capture_format"] != "cursor-jsonl/v1"
        || v["product_version"].as_str() != Some(&profile.product_version)
        || observation.source_kind != SourceKind::Transcript
        || observation.source_trust != SourceTrust::LocalTranscript
        || key.len() != 64
        || !key.bytes().all(|b| b.is_ascii_hexdigit())
    {
        invalid(out, seq, "cursor-transcript-envelope");
        return Ok(());
    }
    let record = &v["record"];
    let session_id = session(batch, out, native);
    if record["type"] == "turn_ended" {
        if matches!(
            record["status"].as_str(),
            Some("success" | "error" | "aborted")
        ) {
            out.records.push(NormalizedActivityRecordV1::SessionState(
                NormalizedSessionStateRecordV1 {
                    source_sequence: seq,
                    session_id,
                    state: SessionState::Idle,
                },
            ));
        } else {
            invalid(out, seq, "cursor-turn-status");
        }
        return Ok(());
    }
    let role = match record["role"].as_str() {
        Some("user") => MessageRole::User,
        Some("assistant") => MessageRole::Assistant,
        _ => {
            invalid(out, seq, "cursor-transcript-role");
            return Ok(());
        }
    };
    let Some(content) = record["message"]["content"].as_array() else {
        invalid(out, seq, "cursor-transcript-content");
        return Ok(());
    };
    let mut texts = Vec::new();
    for (index, part) in content.iter().enumerate() {
        match part["type"].as_str() {
            Some("text") => {
                if let Some(text) = part["text"].as_str() {
                    texts.push(text)
                } else {
                    invalid(out, seq, "cursor-transcript-text");
                }
            }
            Some("tool_use") => {
                let Some(name) = part["name"]
                    .as_str()
                    .filter(|n| !n.is_empty() && n.len() <= 1024)
                else {
                    invalid(out, seq, "cursor-transcript-tool");
                    continue;
                };
                let id = ToolCallId::new(stable_id(
                    batch.source_instance_id,
                    IntegrationId::Cursor,
                    "transcript-tool",
                    &format!("{native}/{key}/{index}"),
                ));
                out.records
                    .push(NormalizedActivityRecordV1::Tool(NormalizedToolRecordV1 {
                        source_sequence: seq,
                        tool_call_id: id,
                        session_id,
                        turn_id: None,
                        native_name: name.into(),
                        kind: tool_kind(name),
                        state: ToolState::Unknown,
                        capture_state: CaptureState::Partial,
                        permission_state: PermissionState::NotObserved,
                        input: part.get("input").map(json_payload).transpose()?,
                        output: None,
                        conflicting_terminal_states: vec![],
                    }));
            }
            _ => invalid(out, seq, "cursor-transcript-part"),
        }
    }
    if !texts.is_empty() {
        out.records.push(NormalizedActivityRecordV1::Message(
            NormalizedMessageRecordV1 {
                source_sequence: seq,
                session_id,
                turn_id: None,
                message_id: MessageId::new(stable_id(
                    batch.source_instance_id,
                    IntegrationId::Cursor,
                    "transcript-message",
                    &format!("{native}/{key}"),
                )),
                role,
                content: text_payload(&texts.join("\n\n"))?,
            },
        ));
    }
    Ok(())
}
