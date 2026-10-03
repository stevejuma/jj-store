use super::{opencode_tools, string, value};
use crate::{
    AgentEntityRefV1, AgentSessionId, IntegrationId, MessageId, MessageRole, NativeEntityKind,
    NormalizationBatchV1, NormalizationDiagnosticKind, NormalizedActivityRecordV1,
    NormalizedMessageRecordV1, NormalizedSessionStateRecordV1, NormalizerProfileV1, Result,
    SanitizedObservationBatchV1, SessionState, alias, json_payload, scoped, stable_id,
};

pub(crate) fn normalize(
    profile: &NormalizerProfileV1,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
) -> Result<()> {
    for observation in &batch.observations {
        let Some(value) = value(observation) else {
            invalid(output, observation.source_sequence, "opencode-json");
            continue;
        };
        if string(&value, &["capture_format"]) == Some("opencode-snapshot/v1") {
            super::opencode_snapshot::normalize(profile, batch, observation, &value, output)?;
            continue;
        }
        if let Some(version) = string(&value, &["input", "event", "properties", "info", "version"])
            && version != profile.product_version
        {
            output.diagnostic(
                NormalizationDiagnosticKind::EmbeddedVersionMismatch,
                "opencode-version",
                Some(observation.source_sequence),
            );
            continue;
        }
        match string(&value, &["hook"]) {
            Some("event") => event(&value, observation.source_sequence, batch, output)?,
            Some("chat.message") => message(&value, observation.source_sequence, batch, output)?,
            Some("tool.execute.before") | Some("tool.execute.after") => {
                opencode_tools::tool(&value, observation.source_sequence, batch, output)?
            }
            Some("permission.ask") => {
                opencode_tools::permission(&value, observation.source_sequence, batch, output)
            }
            _ => output.diagnostic(
                NormalizationDiagnosticKind::UnsupportedEvent,
                "opencode-hook",
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

pub(super) fn session(
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
    native: &str,
) -> AgentSessionId {
    let id = AgentSessionId::new(stable_id(
        batch.source_instance_id,
        IntegrationId::OpenCode,
        "session",
        native,
    ));
    output.alias(alias(
        IntegrationId::OpenCode,
        batch.source_instance_id,
        NativeEntityKind::Session,
        native.into(),
        AgentEntityRefV1::Session { session_id: id },
    ));
    id
}

fn event(
    value: &serde_json::Value,
    sequence: u64,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
) -> Result<()> {
    let Some(event_type) = string(value, &["input", "event", "type"]) else {
        invalid(output, sequence, "opencode-event");
        return Ok(());
    };
    let properties = &value["input"]["event"]["properties"];
    match event_type {
        "session.created" | "session.updated" | "session.deleted" => {
            let Some(native_session) = string(properties, &["info", "id"]) else {
                invalid(output, sequence, "opencode-session");
                return Ok(());
            };
            let state = if event_type == "session.deleted" {
                SessionState::Ended
            } else {
                SessionState::Active
            };
            let session_id = session(batch, output, native_session);
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
        "session.idle" => {
            let Some(native_session) = string(properties, &["sessionID"]) else {
                invalid(output, sequence, "opencode-session");
                return Ok(());
            };
            let session_id = session(batch, output, native_session);
            output
                .records
                .push(NormalizedActivityRecordV1::SessionState(
                    NormalizedSessionStateRecordV1 {
                        source_sequence: sequence,
                        session_id,
                        state: SessionState::Idle,
                    },
                ));
        }
        "message.part.updated" => opencode_tools::tool_part(properties, sequence, batch, output)?,
        _ => output.diagnostic(
            NormalizationDiagnosticKind::UnsupportedEvent,
            "opencode-event-type",
            Some(sequence),
        ),
    }
    Ok(())
}

fn message(
    value: &serde_json::Value,
    sequence: u64,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
) -> Result<()> {
    let (Some(native_session), Some(native_message)) = (
        string(value, &["input", "sessionID"]),
        string(value, &["input", "messageID"]),
    ) else {
        invalid(output, sequence, "opencode-message");
        return Ok(());
    };
    let session_id = session(batch, output, native_session);
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
    let role = match string(value, &["output", "message", "role"]) {
        Some("assistant") => MessageRole::Assistant,
        Some("system") => MessageRole::System,
        Some("tool") => MessageRole::Tool,
        _ => MessageRole::User,
    };
    let Some(parts) = value.get("output").and_then(|v| v.get("parts")) else {
        invalid(output, sequence, "opencode-message-parts");
        return Ok(());
    };
    output.records.push(NormalizedActivityRecordV1::Message(
        NormalizedMessageRecordV1 {
            source_sequence: sequence,
            session_id,
            turn_id: None,
            message_id,
            role,
            content: json_payload(parts)?,
        },
    ));
    Ok(())
}
