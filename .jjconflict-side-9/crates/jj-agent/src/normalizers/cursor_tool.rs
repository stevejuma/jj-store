use super::{cursor::invalid, string, tool_kind};
use crate::{
    AgentEntityRefV1, AgentSessionId, IntegrationId, NativeEntityKind, NormalizationBatchV1,
    NormalizedActivityRecordV1, NormalizedToolRecordV1, PermissionState, Result,
    SanitizedObservationBatchV1, ToolCallId, ToolState, TurnId, alias, json_payload, scoped,
    stable_id,
};

#[allow(clippy::too_many_arguments)]
pub(super) fn normalize(
    event: &str,
    value: &serde_json::Value,
    sequence: u64,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
    native_session: &str,
    session_id: AgentSessionId,
    turn_id: Option<TurnId>,
) -> Result<()> {
    let (Some(native_call), Some(native_name)) = (
        string(value, &["tool_use_id"]),
        string(value, &["tool_name"]),
    ) else {
        invalid(output, sequence, "cursor-tool");
        return Ok(());
    };
    let native = scoped(native_session, native_call);
    let tool_call_id = ToolCallId::new(stable_id(
        batch.source_instance_id,
        IntegrationId::Cursor,
        "tool",
        &native,
    ));
    output.alias(alias(
        IntegrationId::Cursor,
        batch.source_instance_id,
        NativeEntityKind::Tool,
        native,
        AgentEntityRefV1::Tool {
            session_id,
            tool_call_id,
        },
    ));
    let state = match event {
        "preToolUse" => ToolState::Running,
        "postToolUseFailure"
            if value
                .get("is_interrupt")
                .and_then(serde_json::Value::as_bool)
                == Some(true) =>
        {
            ToolState::Interrupted
        }
        "postToolUseFailure" => ToolState::Failed,
        _ => ToolState::Succeeded,
    };
    let output_payload = value
        .get("tool_output")
        .or_else(|| value.get("error_message"));
    output
        .records
        .push(NormalizedActivityRecordV1::Tool(NormalizedToolRecordV1 {
            source_sequence: sequence,
            tool_call_id,
            session_id,
            turn_id,
            native_name: native_name.into(),
            kind: tool_kind(native_name),
            state,
            capture_state: crate::CaptureState::Observed,
            permission_state: PermissionState::NotObserved,
            input: value.get("tool_input").map(json_payload).transpose()?,
            output: output_payload.map(json_payload).transpose()?,
            conflicting_terminal_states: Vec::new(),
        }));
    Ok(())
}
