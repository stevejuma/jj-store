use super::{opencode, string, tool_kind};
use crate::{
    AgentEntityRefV1, IntegrationId, NativeEntityKind, NormalizationBatchV1,
    NormalizedActivityRecordV1, NormalizedPermissionRecordV1, NormalizedToolRecordV1,
    PermissionState, Result, SanitizedObservationBatchV1, ToolCallId, ToolState, alias,
    json_payload, scoped, stable_id,
};

pub(super) fn tool(
    value: &serde_json::Value,
    sequence: u64,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
) -> Result<()> {
    let (Some(hook), Some(native_name), Some(native_session), Some(native_call)) = (
        string(value, &["hook"]),
        string(value, &["input", "tool"]),
        string(value, &["input", "sessionID"]),
        string(value, &["input", "callID"]),
    ) else {
        opencode::invalid(output, sequence, "opencode-tool");
        return Ok(());
    };
    let state = if hook == "tool.execute.before" {
        ToolState::Running
    } else if value
        .pointer("/output/metadata/exit")
        .and_then(serde_json::Value::as_i64)
        .is_some_and(|exit| exit != 0)
        || value
            .pointer("/output/metadata/success")
            .and_then(serde_json::Value::as_bool)
            == Some(false)
    {
        ToolState::Failed
    } else {
        ToolState::Succeeded
    };
    let input = value
        .pointer("/input/args")
        .or_else(|| value.pointer("/output/args"));
    let result = (hook == "tool.execute.after")
        .then(|| value.get("output"))
        .flatten();
    push_tool(
        sequence,
        batch,
        output,
        native_session,
        native_call,
        native_name,
        state,
        input,
        result,
    )
}

pub(super) fn tool_part(
    properties: &serde_json::Value,
    sequence: u64,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
) -> Result<()> {
    let part = &properties["part"];
    let (Some(native_session), Some(native_call), Some(native_name), Some(status)) = (
        string(part, &["sessionID"]),
        string(part, &["callID"]),
        string(part, &["tool"]),
        string(part, &["state", "status"]),
    ) else {
        opencode::invalid(output, sequence, "opencode-tool-part");
        return Ok(());
    };
    let state = match status {
        "pending" => ToolState::Pending,
        "running" => ToolState::Running,
        "completed" => ToolState::Succeeded,
        "error" => ToolState::Failed,
        _ => ToolState::Unknown,
    };
    let input = part.pointer("/state/input");
    let result = part
        .pointer("/state/output")
        .or_else(|| part.pointer("/state/error"));
    push_tool(
        sequence,
        batch,
        output,
        native_session,
        native_call,
        native_name,
        state,
        input,
        result,
    )
}

#[allow(clippy::too_many_arguments)]
fn push_tool(
    sequence: u64,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
    native_session: &str,
    native_call: &str,
    native_name: &str,
    state: ToolState,
    input: Option<&serde_json::Value>,
    result: Option<&serde_json::Value>,
) -> Result<()> {
    let session_id = opencode::session(batch, output, native_session);
    let native = scoped(native_session, native_call);
    let tool_call_id = ToolCallId::new(stable_id(
        batch.source_instance_id,
        IntegrationId::OpenCode,
        "tool",
        &native,
    ));
    output.alias(alias(
        IntegrationId::OpenCode,
        batch.source_instance_id,
        NativeEntityKind::Tool,
        native,
        AgentEntityRefV1::Tool {
            session_id,
            tool_call_id,
        },
    ));
    output
        .records
        .push(NormalizedActivityRecordV1::Tool(NormalizedToolRecordV1 {
            source_sequence: sequence,
            tool_call_id,
            session_id,
            turn_id: None,
            native_name: native_name.into(),
            kind: tool_kind(native_name),
            state,
            capture_state: crate::CaptureState::Observed,
            permission_state: PermissionState::NotObserved,
            input: input.map(json_payload).transpose()?,
            output: result.map(json_payload).transpose()?,
            conflicting_terminal_states: Vec::new(),
        }));
    Ok(())
}

pub(super) fn permission(
    value: &serde_json::Value,
    sequence: u64,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
) {
    let Some(native_session) = string(value, &["input", "sessionID"]) else {
        opencode::invalid(output, sequence, "opencode-permission");
        return;
    };
    let session_id = opencode::session(batch, output, native_session);
    let tool_call_id = string(value, &["input", "callID"]).map(|native| {
        ToolCallId::new(stable_id(
            batch.source_instance_id,
            IntegrationId::OpenCode,
            "tool",
            &scoped(native_session, native),
        ))
    });
    output.records.push(NormalizedActivityRecordV1::Permission(
        NormalizedPermissionRecordV1 {
            source_sequence: sequence,
            session_id,
            tool_call_id,
            state: PermissionState::Requested,
        },
    ));
}
