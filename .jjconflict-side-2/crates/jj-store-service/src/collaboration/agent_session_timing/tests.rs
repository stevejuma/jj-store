use super::*;
use jj_store_wal::publication::{
    CaptureBatchId, ObservationId, SanitizedPayloadV1, SourceInstanceId,
};

fn evidence(timestamp: serde_json::Value) -> SanitizedObservationBatchV1 {
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([1; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([2; 16]),
        sanitizer_version: "test/v1".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![SanitizedObservationV1 {
            observation_id: ObservationId::new([3; 16]),
            source_sequence: 1,
            occurred_at_millis: None,
            source_kind: SourceKind::Transcript,
            source_trust: SourceTrust::LocalTranscript,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                &serde_json::to_vec(&serde_json::json!({
                    "capture_format":"codex-jsonl/v1", "product_version":"0.156.1",
                    "record":{"timestamp":timestamp}
                }))
                .unwrap(),
            )
            .unwrap(),
        }],
    }
}

fn time(evidence: &SanitizedObservationBatchV1, normalizer: &str) -> Option<i64> {
    observed_at_millis(evidence, normalizer, &evidence.observations[0])
}

#[test]
fn native_timestamp_requires_qualified_profile_and_transcript_source() {
    let source = evidence(serde_json::json!("2024-01-02T03:04:05.123Z"));
    assert_eq!(time(&source, "codex@0.156.1/v1"), Some(1_704_164_645_123));
    for profile in [
        "codex@0.159.2/v1",
        "codex@0.156.1/v2",
        "codex@9.9.9/v1",
        "cursor@3.22.7/v1",
    ] {
        assert_eq!(time(&source, profile), None);
    }
    let mut hook = source.clone();
    hook.observations[0].source_kind = SourceKind::Hook;
    hook.observations[0].source_trust = SourceTrust::LocalUserHook;
    assert_eq!(time(&hook, "codex@0.156.1/v1"), None);
    let mut wrong_integration = source.clone();
    wrong_integration.integration = IntegrationId::OpenCode;
    assert_eq!(time(&wrong_integration, "codex@0.156.1/v1"), None);
    let mut omitted = source;
    omitted.observations[0].payload =
        SanitizedPayloadV1::new(SanitizedMediaType::Json, br#"{"capture_omitted":true}"#).unwrap();
    assert_eq!(time(&omitted, "codex@0.156.1/v1"), None);
}

#[test]
fn malformed_native_timestamps_remain_unknown_and_explicit_time_wins() {
    for timestamp in [
        serde_json::json!(null),
        serde_json::json!(1_704_164_645_123i64),
        serde_json::json!("bad date"),
        serde_json::json!("2024-01-02T03:04:05"),
        serde_json::json!("2024-02-30T03:04:05Z"),
    ] {
        assert_eq!(time(&evidence(timestamp), "codex@0.156.1/v1"), None);
    }
    let mut source = evidence(serde_json::json!("2024-01-02T03:04:05.123Z"));
    source.observations[0].occurred_at_millis = Some(123);
    assert_eq!(time(&source, "codex@0.156.1/v1"), Some(123));
}
