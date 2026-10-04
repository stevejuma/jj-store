use crate::{
    AgentEntityRefV1, AgentSessionId, IntegrationId, NativeEntityKind, NormalizationBatchV1,
    SanitizedObservationBatchV1, TurnId, alias, scoped, stable_id,
};

pub(super) fn session(
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

pub(super) fn turn_id(
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
