use super::agent_session_timing::{codex_record, observed_at_millis};
use jj_store_wal::publication::{
    AgentEventV1, SanitizedObservationBatchV1, SourceInstanceId, ToolCallId, ToolState,
    normalized_event_id,
};
use std::collections::BTreeMap;

#[derive(Default)]
pub(super) struct ToolTimings(BTreeMap<ToolCallId, Span>);

struct Span {
    source: SourceInstanceId,
    normalizer: String,
    sequence: Option<u64>,
    start: Option<i64>,
    end: Option<i64>,
    last_time: Option<i64>,
    invalid: bool,
}

impl ToolTimings {
    pub(super) fn observe(
        &mut self,
        evidence: &SanitizedObservationBatchV1,
        normalizer: &str,
        event: &AgentEventV1,
    ) {
        let AgentEventV1::ToolStateChanged {
            event_id,
            tool_call_id,
            state,
            ..
        } = event
        else {
            return;
        };
        let observation = evidence.observations.iter().find(|observation| {
            normalized_event_id(
                evidence.source_instance_id,
                evidence.integration,
                observation.source_sequence,
                "tool",
            ) == *event_id
        });
        let span = self.0.entry(*tool_call_id).or_insert_with(|| Span {
            source: evidence.source_instance_id,
            normalizer: normalizer.into(),
            sequence: None,
            start: None,
            end: None,
            last_time: None,
            invalid: false,
        });
        let Some(observation) = observation else {
            span.invalid = true;
            return;
        };
        if span.source != evidence.source_instance_id
            || span.normalizer != normalizer
            || span
                .sequence
                .is_some_and(|value| observation.source_sequence <= value)
        {
            span.invalid = true;
        }
        span.sequence = Some(observation.source_sequence);
        let Some(time) = observed_at_millis(evidence, normalizer, observation) else {
            span.invalid = true;
            return;
        };
        if span.last_time.is_some_and(|previous| time < previous) {
            span.invalid = true;
        }
        span.last_time = Some(time);
        // A result record proves an end without claiming a successful outcome.
        let start = if let Some(value) = codex_record(evidence, normalizer, observation) {
            if value["record"]["type"] != "response_item" {
                span.invalid = true;
                return;
            }
            match value["record"]["payload"]["type"].as_str() {
                Some("function_call" | "custom_tool_call") => true,
                Some("function_call_output" | "custom_tool_call_output") => false,
                _ => {
                    span.invalid = true;
                    return;
                }
            }
        } else {
            match state {
                ToolState::Pending | ToolState::Running => true,
                ToolState::Succeeded
                | ToolState::Failed
                | ToolState::Cancelled
                | ToolState::Interrupted => false,
                _ => {
                    span.invalid = true;
                    return;
                }
            }
        };
        if start {
            if span.end.is_some() || span.start.is_some_and(|value| time < value) {
                span.invalid = true;
            }
            span.start.get_or_insert(time);
        } else {
            if span.end.is_some() || span.start.is_none_or(|value| time < value) {
                span.invalid = true;
            }
            span.end = Some(time);
        }
    }

    pub(super) fn duration(&self, tool: ToolCallId) -> Option<u64> {
        let span = self.0.get(&tool)?;
        if span.invalid {
            return None;
        }
        u64::try_from(span.end?.checked_sub(span.start?)?).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jj_store_wal::publication::{
        AgentSessionId, CaptureBatchId, IntegrationId, ObservationId, SanitizedMediaType,
        SanitizedObservationV1, SanitizedPayloadV1, SanitizedRepresentation, SourceKind,
        SourceTrust,
    };

    fn sample(
        sequence: u64,
        time: Option<i64>,
        state: ToolState,
    ) -> (SanitizedObservationBatchV1, AgentEventV1) {
        let source = SourceInstanceId::new([1; 16]);
        (
            SanitizedObservationBatchV1 {
                schema_version: 1,
                batch_id: CaptureBatchId::new([2; 16]),
                integration: IntegrationId::Cursor,
                source_instance_id: source,
                sanitizer_version: "test/v1".into(),
                representation: SanitizedRepresentation::SanitizedNative,
                observations: vec![SanitizedObservationV1 {
                    observation_id: ObservationId::new([3; 16]),
                    source_sequence: sequence,
                    occurred_at_millis: time,
                    source_kind: SourceKind::Hook,
                    source_trust: SourceTrust::LocalUserHook,
                    payload: SanitizedPayloadV1::new(SanitizedMediaType::Json, b"{}").unwrap(),
                }],
            },
            AgentEventV1::ToolStateChanged {
                event_id: normalized_event_id(source, IntegrationId::Cursor, sequence, "tool"),
                tool_call_id: ToolCallId::new([4; 16]),
                session_id: AgentSessionId::new([5; 16]),
                turn_id: None,
                state,
            },
        )
    }

    #[test]
    fn explicit_hook_span_preserves_failed_state_and_rejects_incomplete_or_ambiguous_evidence() {
        let id = ToolCallId::new([4; 16]);
        let start = sample(0, Some(1000), ToolState::Running);
        let end = sample(1, Some(1321), ToolState::Failed);
        let mut timings = ToolTimings::default();
        timings.observe(&start.0, "cursor@3.22.7/v1", &start.1);
        assert_eq!(timings.duration(id), None);
        timings.observe(&end.0, "cursor@3.22.7/v1", &end.1);
        assert_eq!(timings.duration(id), Some(321));
        for variant in 0..8 {
            let mut timings = ToolTimings::default();
            let mut end = sample(1, Some(1321), ToolState::Failed);
            match variant {
                0 => end.0.observations[0].occurred_at_millis = None,
                1 => end.0.observations[0].occurred_at_millis = Some(999),
                2 => end.0.source_instance_id = SourceInstanceId::new([8; 16]),
                3 => end.0.observations[0].source_sequence = 2,
                _ => {}
            }
            timings.observe(&start.0, "cursor@3.22.7/v1", &start.1);
            if variant == 7 {
                let intermediate = sample(1, Some(1500), ToolState::Running);
                timings.observe(&intermediate.0, "cursor@3.22.7/v1", &intermediate.1);
                end = sample(2, Some(1321), ToolState::Failed);
            }
            timings.observe(
                &end.0,
                if variant == 4 {
                    "other/v1"
                } else {
                    "cursor@3.22.7/v1"
                },
                &end.1,
            );
            if variant == 5 {
                let repeated = sample(2, Some(1400), ToolState::Succeeded);
                timings.observe(&repeated.0, "cursor@3.22.7/v1", &repeated.1);
            }
            if variant == 6 {
                let restarted = sample(2, Some(1400), ToolState::Running);
                timings.observe(&restarted.0, "cursor@3.22.7/v1", &restarted.1);
            }
            assert_eq!(timings.duration(id), None, "variant {variant}");
        }
    }
}
