use super::{codex_transcript::invalid, string};
use crate::{
    AgentEntityRefV1, AgentSessionId, IntegrationId, NativeEntityKind, NormalizationBatchV1,
    Result, SanitizedObservationBatchV1, ToolCallId, ToolResultObservation, activity::valid_label,
    alias, json_payload, scoped, stable_id, text_payload,
};
use serde_json::Value;

pub(super) fn record(
    payload: &Value,
    sequence: u64,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
    native_session: &str,
    results: &mut Vec<ToolResultObservation>,
) -> Result<()> {
    let (Some(call), Some(content)) = (
        string(payload, &["call_id"]).filter(|v| valid_label(v, 1024)),
        payload.get("output"),
    ) else {
        invalid(output, sequence, "codex-transcript-tool-result");
        return Ok(());
    };
    let content = match content {
        Value::String(text) => text_payload(text)?,
        value => json_payload(value)?,
    };
    let native = scoped(native_session, call);
    if !valid_label(&native, 1024) {
        invalid(output, sequence, "codex-transcript-tool-result-alias");
        return Ok(());
    }
    let session_id = AgentSessionId::new(stable_id(
        batch.source_instance_id,
        IntegrationId::Codex,
        "session",
        native_session,
    ));
    let tool_call_id = ToolCallId::new(stable_id(
        batch.source_instance_id,
        IntegrationId::Codex,
        "tool",
        &native,
    ));
    results.push(ToolResultObservation {
        source_sequence: sequence,
        session_alias: alias(
            IntegrationId::Codex,
            batch.source_instance_id,
            NativeEntityKind::Session,
            native_session.into(),
            AgentEntityRefV1::Session { session_id },
        ),
        tool_alias: alias(
            IntegrationId::Codex,
            batch.source_instance_id,
            NativeEntityKind::Tool,
            native,
            AgentEntityRefV1::Tool {
                session_id,
                tool_call_id,
            },
        ),
        output: content,
    });
    Ok(())
}
