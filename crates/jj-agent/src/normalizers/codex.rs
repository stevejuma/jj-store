use super::{string, tool_kind, value};
use crate::{
    AgentEntityRefV1, AgentSessionId, IntegrationId, MessageId, MessageRole, NativeEntityKind,
    NormalizationBatchV1, NormalizationDiagnosticKind, NormalizedActivityRecordV1,
    NormalizedMessageRecordV1, NormalizedPermissionRecordV1, NormalizedSessionStateRecordV1,
    NormalizedToolRecordV1, NormalizerProfileV1, PermissionState, Result,
    SanitizedObservationBatchV1, SessionState, ToolCallId, ToolState, TurnId, alias, json_payload,
    scoped, stable_id, text_payload,
};

pub(crate) fn normalize(
    _: &NormalizerProfileV1,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
) -> Result<()> {
    for observation in &batch.observations {
        let Some(value) = value(observation) else {
            output.diagnostic(
                NormalizationDiagnosticKind::InvalidRecord,
                "codex-json",
                Some(observation.source_sequence),
            );
            continue;
        };
        let (Some(event), Some(native_session)) = (
            string(&value, &["hook_event_name"]),
            string(&value, &["session_id"]),
        ) else {
            output.diagnostic(
                NormalizationDiagnosticKind::InvalidRecord,
                "codex-envelope",
                Some(observation.source_sequence),
            );
            continue;
        };
        let session_id = session(batch, output, native_session);
        let turn = string(&value, &["turn_id"])
            .map(|native| turn_id(batch, output, native_session, native));
        match event {
            "SessionStart" => output
                .records
                .push(NormalizedActivityRecordV1::SessionState(
                    NormalizedSessionStateRecordV1 {
                        source_sequence: observation.source_sequence,
                        session_id,
                        state: SessionState::Active,
                    },
                )),
            "SessionEnd" => output
                .records
                .push(NormalizedActivityRecordV1::SessionState(
                    NormalizedSessionStateRecordV1 {
                        source_sequence: observation.source_sequence,
                        session_id,
                        state: SessionState::Ended,
                    },
                )),
            "UserPromptSubmit" => {
                let Some(prompt) = string(&value, &["prompt"]) else {
                    output.diagnostic(
                        NormalizationDiagnosticKind::InvalidRecord,
                        "codex-prompt",
                        Some(observation.source_sequence),
                    );
                    continue;
                };
                let native = scoped(
                    native_session,
                    &format!("{}/user", string(&value, &["turn_id"]).unwrap_or("unknown")),
                );
                let message_id = MessageId::new(stable_id(
                    batch.source_instance_id,
                    IntegrationId::Codex,
                    "message",
                    &native,
                ));
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
                output.records.push(NormalizedActivityRecordV1::Message(
                    NormalizedMessageRecordV1 {
                        source_sequence: observation.source_sequence,
                        session_id,
                        turn_id: turn,
                        message_id,
                        role: MessageRole::User,
                        content: text_payload(prompt)?,
                    },
                ));
            }
            "PreToolUse" | "PostToolUse" => tool(
                event,
                &value,
                observation.source_sequence,
                batch,
                output,
                native_session,
                session_id,
                turn,
            )?,
            "PermissionRequest" => permission(
                &value,
                observation.source_sequence,
                batch,
                output,
                native_session,
                session_id,
            ),
            "Stop" => output
                .records
                .push(NormalizedActivityRecordV1::SessionState(
                    NormalizedSessionStateRecordV1 {
                        source_sequence: observation.source_sequence,
                        session_id,
                        state: SessionState::Idle,
                    },
                )),
            "Interrupt" => output
                .records
                .push(NormalizedActivityRecordV1::SessionState(
                    NormalizedSessionStateRecordV1 {
                        source_sequence: observation.source_sequence,
                        session_id,
                        state: SessionState::Idle,
                    },
                )),
            _ => output.diagnostic(
                NormalizationDiagnosticKind::UnsupportedEvent,
                "codex-event",
                Some(observation.source_sequence),
            ),
        }
    }
    Ok(())
}

fn session(
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
    native: &str,
) -> AgentSessionId {
    let id = AgentSessionId::new(stable_id(
        batch.source_instance_id,
        IntegrationId::Codex,
        "session",
        native,
    ));
    output.alias(alias(
        IntegrationId::Codex,
        batch.source_instance_id,
        NativeEntityKind::Session,
        native.into(),
        AgentEntityRefV1::Session { session_id: id },
    ));
    id
}

fn turn_id(
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
    native_session: &str,
    native: &str,
) -> TurnId {
    let native = scoped(native_session, native);
    let id = TurnId::new(stable_id(
        batch.source_instance_id,
        IntegrationId::Codex,
        "turn",
        &native,
    ));
    let session_id = session(batch, output, native_session);
    output.alias(alias(
        IntegrationId::Codex,
        batch.source_instance_id,
        NativeEntityKind::Turn,
        native,
        AgentEntityRefV1::Turn {
            session_id,
            turn_id: id,
        },
    ));
    id
}

#[allow(clippy::too_many_arguments)]
fn tool(
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
        output.diagnostic(
            NormalizationDiagnosticKind::InvalidRecord,
            "codex-tool",
            Some(sequence),
        );
        return Ok(());
    };
    let native = scoped(native_session, native_call);
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
    let response = value.get("tool_response");
    let state = if event == "PreToolUse" {
        ToolState::Running
    } else if response
        .and_then(|v| v.get("success"))
        .and_then(serde_json::Value::as_bool)
        == Some(false)
        || response
            .and_then(|v| v.get("exit_code"))
            .and_then(serde_json::Value::as_i64)
            .is_some_and(|code| code != 0)
    {
        ToolState::Failed
    } else {
        ToolState::Succeeded
    };
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
            output: response.map(json_payload).transpose()?,
            conflicting_terminal_states: Vec::new(),
        }));
    Ok(())
}

fn permission(
    value: &serde_json::Value,
    sequence: u64,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
    native_session: &str,
    session_id: AgentSessionId,
) {
    let tool_call_id = string(value, &["tool_use_id"]).map(|native| {
        ToolCallId::new(stable_id(
            batch.source_instance_id,
            IntegrationId::Codex,
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
