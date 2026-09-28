use super::{cursor_tool, string, value};
use crate::{
    AgentEntityRefV1, AgentSessionId, IntegrationId, MessageId, MessageRole, NativeEntityKind,
    NormalizationBatchV1, NormalizationDiagnosticKind, NormalizedActivityRecordV1,
    NormalizedFileObservationV1, NormalizedMessageRecordV1, NormalizedSessionStateRecordV1,
    NormalizerProfileV1, Result, SanitizedObservationBatchV1, SessionState, TurnId, alias, scoped,
    stable_id, text_payload,
};

pub(crate) fn normalize(
    profile: &NormalizerProfileV1,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
) -> Result<()> {
    for observation in &batch.observations {
        let Some(value) = value(observation) else {
            invalid(output, observation.source_sequence, "cursor-json");
            continue;
        };
        match string(&value, &["cursor_version"]) {
            None => {
                invalid(output, observation.source_sequence, "cursor-version");
                continue;
            }
            Some(version) if version != profile.product_version => {
                output.diagnostic(
                    NormalizationDiagnosticKind::EmbeddedVersionMismatch,
                    "cursor-version",
                    Some(observation.source_sequence),
                );
                continue;
            }
            Some(_) => {}
        }
        let (Some(event), Some(native_session)) = (
            string(&value, &["hook_event_name"]),
            string(&value, &["conversation_id"]),
        ) else {
            invalid(output, observation.source_sequence, "cursor-envelope");
            continue;
        };
        let session_id = session(batch, output, native_session);
        let turn = string(&value, &["generation_id"])
            .map(|native| turn_id(batch, output, native_session, native));
        match event {
            "sessionStart" => state(
                output,
                observation.source_sequence,
                session_id,
                SessionState::Active,
            ),
            "sessionEnd" => state(
                output,
                observation.source_sequence,
                session_id,
                SessionState::Ended,
            ),
            "stop" => state(
                output,
                observation.source_sequence,
                session_id,
                SessionState::Idle,
            ),
            "beforeSubmitPrompt" => message(
                &value,
                observation.source_sequence,
                batch,
                output,
                native_session,
                session_id,
                turn,
                MessageRole::User,
                "prompt",
            )?,
            "afterAgentResponse" => message(
                &value,
                observation.source_sequence,
                batch,
                output,
                native_session,
                session_id,
                turn,
                MessageRole::Assistant,
                "text",
            )?,
            "preToolUse" | "postToolUse" | "postToolUseFailure" => cursor_tool::normalize(
                event,
                &value,
                observation.source_sequence,
                batch,
                output,
                native_session,
                session_id,
                turn,
            )?,
            "afterFileEdit" => {
                let Some(path) = string(&value, &["file_path"]) else {
                    invalid(output, observation.source_sequence, "cursor-file");
                    continue;
                };
                output
                    .records
                    .push(NormalizedActivityRecordV1::FileObservation(
                        NormalizedFileObservationV1 {
                            source_sequence: observation.source_sequence,
                            session_id,
                            path: path.into(),
                        },
                    ));
            }
            _ => output.diagnostic(
                NormalizationDiagnosticKind::UnsupportedEvent,
                "cursor-event",
                Some(observation.source_sequence),
            ),
        }
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
fn state(
    output: &mut NormalizationBatchV1,
    source_sequence: u64,
    session_id: AgentSessionId,
    state: SessionState,
) {
    output
        .records
        .push(NormalizedActivityRecordV1::SessionState(
            NormalizedSessionStateRecordV1 {
                source_sequence,
                session_id,
                state,
            },
        ));
}

fn session(
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
    native: &str,
) -> AgentSessionId {
    let id = AgentSessionId::new(stable_id(
        batch.source_instance_id,
        IntegrationId::Cursor,
        "session",
        native,
    ));
    output.alias(alias(
        IntegrationId::Cursor,
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
        IntegrationId::Cursor,
        "turn",
        &native,
    ));
    let session_id = session(batch, output, native_session);
    output.alias(alias(
        IntegrationId::Cursor,
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
fn message(
    value: &serde_json::Value,
    sequence: u64,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
    native_session: &str,
    session_id: AgentSessionId,
    turn_id: Option<TurnId>,
    role: MessageRole,
    field: &str,
) -> Result<()> {
    let Some(content) = string(value, &[field]) else {
        invalid(output, sequence, "cursor-message");
        return Ok(());
    };
    let native = scoped(
        native_session,
        &format!(
            "{}/{field}",
            string(value, &["generation_id"]).unwrap_or("unknown")
        ),
    );
    let message_id = MessageId::new(stable_id(
        batch.source_instance_id,
        IntegrationId::Cursor,
        "message",
        &native,
    ));
    output.alias(alias(
        IntegrationId::Cursor,
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
            turn_id,
            message_id,
            role,
            content: text_payload(content)?,
        },
    ));
    Ok(())
}
