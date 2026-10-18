use jj_agent::*;
use serde_json::{Value, json};

fn profile() -> NormalizerProfileV1 {
    NormalizerProfileV1 {
        integration: IntegrationId::OwnedController,
        product_version: "jj-native-capture-1".into(),
        schema_version: 1,
    }
}
fn envelope(frame: Value) -> Value {
    json!({
        "schema":1,"policy":"fixture-policy/v1",
        "binding":{"scope":"vault","worker":"worker","request":"open","target":"target","window":"main","admission":"generation"},
        "ordinal":4,"publication":{"kind":"event","sequence":3},"frames":[frame]
    })
}
fn batch(value: Value) -> SanitizedObservationBatchV1 {
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([1; 16]),
        integration: IntegrationId::OwnedController,
        source_instance_id: SourceInstanceId::new([2; 16]),
        sanitizer_version: "fixture-policy/v1".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![SanitizedObservationV1 {
            observation_id: ObservationId::new([3; 16]),
            source_sequence: 4,
            occurred_at_millis: None,
            source_kind: SourceKind::NativeApi,
            source_trust: SourceTrust::ApprovedHarnessApi,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                &serde_json::to_vec(&value).unwrap(),
            )
            .unwrap(),
        }],
    }
}
fn record() -> Value {
    json!({"version":3,"type":"record","sessionId":"worker","requestId":"turn","sequence":81,"record":{"status":"completed"},"assistantText":"safe answer"})
}

#[test]
fn native_capture_projects_complete_groups_and_retains_exact_source_progress() {
    let mut value = envelope(record());
    value["frames"] = json!([
        {"version":3,"type":"activity","sessionId":"worker","requestId":"turn","update":{"sessionUpdate":"agent_message_chunk","content":{"type":"text","text":"safe answer"}}},
        record()
    ]);
    let input = batch(value);
    let bytes = input.observations[0].payload.bytes().to_vec();
    let normalized = normalize_batch(&profile(), &input).unwrap();
    assert!(normalized.diagnostics.is_empty());
    assert_eq!(normalized.records.len(), 2);
    assert!(
        matches!(&normalized.records[0], NormalizedActivityRecordV1::SessionState(v) if v.source_sequence==4 && v.state==SessionState::Idle)
    );
    assert!(
        matches!(&normalized.records[1], NormalizedActivityRecordV1::Message(v) if v.source_sequence==4 && v.role==MessageRole::Assistant && v.content.bytes()==b"safe answer")
    );
    assert_eq!(input.observations[0].payload.bytes(), bytes);
    assert_eq!(normalize_batch(&profile(), &input).unwrap(), normalized);
}

#[test]
fn native_capture_rejects_mismatched_scope_progress_policy_and_group_shapes() {
    let source = envelope(record());
    for changed in [
        {
            let mut v = source.clone();
            v["schema"] = json!(2);
            v
        },
        {
            let mut v = source.clone();
            v["ordinal"] = json!(5);
            v
        },
        {
            let mut v = source.clone();
            v["policy"] = json!("other");
            v
        },
        {
            let mut v = source.clone();
            v["binding"]["worker"] = json!("other");
            v
        },
        {
            let mut v = source.clone();
            v["binding"]["extra"] = json!(true);
            v
        },
        {
            let mut v = source.clone();
            v["publication"]["sequence"] = json!(0);
            v
        },
        {
            let mut v = source.clone();
            v["frames"] = json!([]);
            v
        },
        {
            let mut v = source.clone();
            v["frames"] = json!([record(), record()]);
            v
        },
        {
            let mut v = source.clone();
            v["frames"][0]["version"] = json!(4);
            v
        },
        {
            let mut v = source.clone();
            v["frames"][0]["controllerSessionId"] = json!("forged");
            v
        },
    ] {
        assert!(normalize_batch(&profile(), &batch(changed)).is_err());
    }
}

#[test]
fn native_capture_bootstrap_closure_and_uncertain_errors_keep_distinct_states() {
    for (frame, publication, expected) in [
        (
            json!({"version":3,"type":"opened","sessionId":"worker","adapterSessionId":"provider","requestId":"native-open","sequence":2}),
            json!({"kind":"facts"}),
            SessionState::Idle,
        ),
        (
            json!({"version":3,"type":"session_closed","sessionId":"worker"}),
            json!({"kind":"event","sequence":3}),
            SessionState::Ended,
        ),
        (
            json!({"version":3,"type":"error","sessionId":"worker","requestId":"turn","code":"unknown"}),
            json!({"kind":"event","sequence":3}),
            SessionState::Unknown,
        ),
    ] {
        let mut value = envelope(frame);
        value["publication"] = publication;
        let normalized = normalize_batch(&profile(), &batch(value)).unwrap();
        assert!(normalized.diagnostics.is_empty());
        assert!(
            matches!(&normalized.records[..], [NormalizedActivityRecordV1::SessionState(v)] if v.state==expected)
        );
    }
}

#[test]
fn native_capture_deferred_prefix_does_not_repeat_its_old_tool_metadata() {
    let tool = json!({"toolCallId":"read","title":"Read","kind":"read","status":"completed"});
    let mut value = envelope(
        json!({"version":3,"type":"activity","sessionId":"worker","requestId":"turn","toolSnapshot":tool}),
    );
    value["frames"].as_array_mut().unwrap().insert(0, json!({
        "version":3,"type":"activity","sessionId":"worker","requestId":"old-turn",
        "toolSnapshot":{"toolCallId":"old","title":"Old tool","status":"in_progress"},
        "update":{"sessionUpdate":"agent_message_chunk","content":{"type":"text","text":"released prefix"}}
    }));
    let input = batch(value);
    let normalized = normalize_batch(&profile(), &input).unwrap();
    assert!(
        matches!(&normalized.records[..], [NormalizedActivityRecordV1::Tool(v)] if v.native_name=="Read" && v.state==ToolState::Succeeded)
    );
    assert!(
        std::str::from_utf8(input.observations[0].payload.bytes())
            .unwrap()
            .contains("old-turn")
    );
}

#[test]
fn native_capture_preserves_full_width_ordinal_and_native_cursors() {
    let mut value = envelope(record());
    value["ordinal"] = json!(u64::MAX);
    value["publication"]["sequence"] = json!(u64::MAX);
    value["frames"][0]["sequence"] = json!(u64::MAX);
    let mut input = batch(value);
    input.observations[0].source_sequence = u64::MAX;
    let normalized = normalize_batch(&profile(), &input).unwrap();
    assert!(
        matches!(&normalized.records[1],NormalizedActivityRecordV1::Message(v) if v.source_sequence==u64::MAX)
    );
    let retained: Value = serde_json::from_slice(input.observations[0].payload.bytes()).unwrap();
    assert_eq!(retained["publication"]["sequence"].as_u64(), Some(u64::MAX));
    assert_eq!(retained["frames"][0]["sequence"].as_u64(), Some(u64::MAX));
}
