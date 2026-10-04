use jj_agent::*;
use serde_json::{Value, json};

const SESSION: &str = "ses_snapshot1";
fn batch(kind: SourceKind, trust: SourceTrust, records: Vec<Value>) -> SanitizedObservationBatchV1 {
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([1; 16]),
        integration: IntegrationId::OpenCode,
        source_instance_id: SourceInstanceId::new([2; 16]),
        sanitizer_version: "test-sanitized/v1".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: records
            .into_iter()
            .enumerate()
            .map(|(i, record)| SanitizedObservationV1 {
                observation_id: ObservationId::new([i as u8 + 1; 16]),
                source_sequence: i as u64 + 1,
                occurred_at_millis: None,
                source_kind: kind,
                source_trust: trust,
                payload: SanitizedPayloadV1::new(
                    SanitizedMediaType::Json,
                    &serde_json::to_vec(&record).unwrap(),
                )
                .unwrap(),
            })
            .collect(),
    }
}
fn envelope(record: Value) -> Value {
    json!({"capture_format":"opencode-snapshot/v1", "product_version":"1.18.30", "session_id":SESSION, "record":record})
}
fn message(role: &str, tool_status: &str) -> Value {
    envelope(
        json!({"type":"message", "info":{"id":"msg_snapshot1", "sessionID":SESSION, "role":role}, "parts":[
            {"id":"prt_text1", "sessionID":SESSION, "messageID":"msg_snapshot1", "type":"text", "text":"Inspect the recorded edit"},
            {"id":"prt_tool1", "sessionID":SESSION, "messageID":"msg_snapshot1", "type":"tool", "callID":"call_snapshot1", "tool":"edit", "state":{"status":tool_status,"input":{"path":"$WORKSPACE/src/main.rs"},"output":"Recorded edit","error":"Recorded failure"}}
        ]}),
    )
}
fn normalize(input: &SanitizedObservationBatchV1) -> NormalizationBatchV1 {
    normalize_batch(&NormalizerProfileV1::opencode_1_18_30(), input).unwrap()
}
#[test]
fn native_snapshots_keep_messages_tools_and_explicit_api_lifecycle() {
    let input = batch(
        SourceKind::NativeApi,
        SourceTrust::ApprovedHarnessApi,
        vec![
            envelope(json!({"type":"session_status","status":{"type":"busy"}})),
            message("assistant", "running"),
            message("assistant", "completed"),
            envelope(json!({"type":"session_status","status":{"type":"idle"}})),
        ],
    );
    let result = normalize(&input);
    assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
    assert_eq!(result.records.len(), 6);
    let tools: Vec<_> = result
        .records
        .iter()
        .filter_map(|r| match r {
            NormalizedActivityRecordV1::Tool(t) => Some(t),
            _ => None,
        })
        .collect();
    assert_eq!(tools[0].tool_call_id, tools[1].tool_call_id);
    assert_eq!(tools[0].state, ToolState::Running);
    assert!(tools[0].output.is_none());
    assert_eq!(tools[1].state, ToolState::Succeeded);
    assert!(tools[1].output.is_some());
    let messages: Vec<_> = result
        .records
        .iter()
        .filter_map(|r| match r {
            NormalizedActivityRecordV1::Message(m) => Some(m),
            _ => None,
        })
        .collect();
    assert_eq!(messages[0].message_id, messages[1].message_id);
    assert_eq!(messages[0].role, MessageRole::Assistant);
    assert!(
        std::str::from_utf8(messages[0].content.bytes())
            .unwrap()
            .contains("prt_text1")
    );
    assert!(
        matches!(&result.records[0], NormalizedActivityRecordV1::SessionState(s) if s.state == SessionState::Active)
    );
    assert!(
        matches!(&result.records[5], NormalizedActivityRecordV1::SessionState(s) if s.state == SessionState::Idle)
    );
    assert_eq!(normalize(&input), result);
}

#[test]
fn unsupported_parts_keep_safe_message_evidence_without_fictional_tools() {
    let mut value = message("assistant", "completed");
    value["record"]["parts"][1]["type"] = json!("future-part");
    let result = normalize(&batch(
        SourceKind::NativeApi,
        SourceTrust::ApprovedHarnessApi,
        vec![value],
    ));
    assert_eq!(result.records.len(), 1);
    assert!(matches!(
        &result.records[0],
        NormalizedActivityRecordV1::Message(_)
    ));
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(result.diagnostics[0].code, "opencode-snapshot-part");
}
#[test]
fn exported_snapshots_cannot_assert_running_and_unknown_roles_are_gaps() {
    let exported = batch(
        SourceKind::SessionExport,
        SourceTrust::LocalTranscript,
        vec![
            message("user", "error"),
            envelope(json!({"type":"session_status","status":{"type":"busy"}})),
        ],
    );
    let result = normalize(&exported);
    assert!(
        !result
            .records
            .iter()
            .any(|r| matches!(r, NormalizedActivityRecordV1::SessionState(_)))
    );
    assert_eq!(result.records.len(), 2);
    assert_eq!(result.diagnostics.len(), 1);
    let unknown = batch(
        SourceKind::NativeApi,
        SourceTrust::ApprovedHarnessApi,
        vec![message("future-role", "completed")],
    );
    assert!(normalize(&unknown).records.is_empty());
}
#[test]
fn mismatched_native_owners_versions_and_evidence_never_publish_aliases() {
    for mutate in [0, 1, 2, 3] {
        let mut value = message("assistant", "completed");
        match mutate {
            0 => value["record"]["info"]["sessionID"] = json!("ses_other"),
            1 => value["record"]["parts"][0]["messageID"] = json!("msg_other"),
            2 => value["product_version"] = json!("1.18.31"),
            _ => value["record"]["parts"][1]["id"] = json!("prt_text1"),
        }
        let result = normalize(&batch(
            SourceKind::NativeApi,
            SourceTrust::ApprovedHarnessApi,
            vec![value],
        ));
        assert!(result.records.is_empty());
        assert!(result.aliases.is_empty());
        assert!(!result.diagnostics.is_empty());
    }
    let forged = normalize(&batch(
        SourceKind::Hook,
        SourceTrust::LocalUserHook,
        vec![message("assistant", "completed")],
    ));
    assert!(forged.records.is_empty());
    assert!(forged.aliases.is_empty());
}
