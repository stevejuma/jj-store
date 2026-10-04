use jj_agent::*;
use serde_json::json;
const SESSION: &str = "00000000-0000-0000-0000-000000000001";
fn batch(records: Vec<serde_json::Value>) -> SanitizedObservationBatchV1 {
    SanitizedObservationBatchV1 {
        schema_version:1, batch_id:CaptureBatchId::new([1;16]),integration:IntegrationId::Codex,
        source_instance_id:SourceInstanceId::new([2;16]),sanitizer_version:"test/v1".into(),
        representation:SanitizedRepresentation::SanitizedNative,
        observations:records.into_iter().enumerate().map(|(i,record)| SanitizedObservationV1 {
            observation_id:ObservationId::new([i as u8+1;16]), source_sequence:i as u64+1,
            occurred_at_millis:None,source_kind:SourceKind::Transcript,source_trust:SourceTrust::LocalTranscript,
            payload:SanitizedPayloadV1::new(SanitizedMediaType::Json,&serde_json::to_vec(&json!({
                "capture_format":"codex-jsonl/v1", "product_version":"0.156.1", "session_id":SESSION, "record":record
            })).unwrap()).unwrap(),
        }).collect(),
    }
}
#[test]
fn native_codex_transcripts_project_messages_and_lifecycle_without_inventing_execution() {
    let batch = batch(vec![
        json!({"type":"session_meta","payload":{"id":SESSION,"cli_version":"0.156.1"}}),
        json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"turn-1"}}),
        json!({"type":"response_item","payload":{"type":"message","id":"message-1","role":"user","content":[{"type":"input_text","text":"Please inspect the code"}]}}),
        json!({"type":"response_item","payload":{"type":"message","id":"message-2","role":"assistant","content":[{"type":"output_text","text":"I will inspect it"}]}}),
        json!({"type":"response_item","payload":{"type":"function_call","call_id":"call-1","name":"functions.exec_command","arguments":"{\"cmd\":\"pwd\"}"}}),
        json!({"type":"event_msg","payload":{"type":"task_complete","turn_id":"turn-1"}}),
    ]);
    let normalized = normalize_batch(&NormalizerProfileV1::codex_0_156_1(), &batch).unwrap();
    assert_eq!(
        normalized
            .records
            .iter()
            .filter(|r| matches!(r, NormalizedActivityRecordV1::Message(_)))
            .count(),
        2
    );
    let tools = normalized.reduce_tools();
    assert_eq!(tools.tools.len(), 1);
    assert_eq!(tools.tools[0].state, ToolState::Pending);
    assert_eq!(tools.tools[0].kind, ToolKind::Execute);
    assert!(normalized.records.iter().any(
        |r| matches!(r,NormalizedActivityRecordV1::SessionState(v) if v.state==SessionState::Active)
    ));
    assert!(normalized.records.iter().any(
        |r| matches!(r,NormalizedActivityRecordV1::SessionState(v) if v.state==SessionState::Idle)
    ));
    assert_eq!(
        normalize_batch(&NormalizerProfileV1::codex_0_156_1(), &batch).unwrap(),
        normalized
    );
}
#[test]
fn metadata_unknown_records_and_unmatched_results_do_not_invent_activity() {
    let batch = batch(vec![
        json!({"type":"session_meta","payload":{"id":SESSION}}),
        json!({"type":"future_event","payload":{"text":"retained native evidence"}}),
        json!({"type":"response_item","payload":{"type":"function_call_output","call_id":"unmatched","output":"finished"}}),
    ]);
    let normalized = normalize_batch(&NormalizerProfileV1::codex_0_156_1(), &batch).unwrap();
    assert!(normalized.records.is_empty());
    assert!(normalized.aliases.is_empty());
    assert_eq!(normalized.diagnostics.len(), 3);
}
#[test]
fn native_message_identity_survives_updates_and_missing_ids_use_source_positions() {
    let mut first = batch(vec![
        json!({"type":"response_item","payload":{"type":"message","id":"native-message","role":"assistant","content":[{"type":"output_text","text":"first"}]}}),
    ]);
    let profile = NormalizerProfileV1::codex_0_156_1();
    let before = normalize_batch(&profile, &first).unwrap();
    first.observations[0].source_sequence = 2;
    let after = normalize_batch(&profile, &first).unwrap();
    let [NormalizedActivityRecordV1::Message(before)] = &before.records[..] else {
        panic!("message")
    };
    let [NormalizedActivityRecordV1::Message(after)] = &after.records[..] else {
        panic!("message")
    };
    assert_eq!(before.message_id, after.message_id);
    let missing = batch(vec![
        json!({"type":"response_item","payload":{"type":"message","id":null,"role":"assistant","content":[{"type":"output_text","text":"same text"}]}}),
        json!({"type":"response_item","payload":{"type":"message","role":"assistant","content":[{"type":"output_text","text":"same text"}]}}),
    ]);
    let normalized = normalize_batch(&profile, &missing).unwrap();
    let [
        NormalizedActivityRecordV1::Message(first),
        NormalizedActivityRecordV1::Message(second),
    ] = &normalized.records[..]
    else {
        panic!("two messages")
    };
    assert_ne!(first.message_id, second.message_id);
}
#[test]
fn desktop_native_profile_does_not_authorize_legacy_hooks_or_untrusted_envelopes() {
    let mut native = batch(vec![
        json!({"type":"event_msg","payload":{"type":"task_started"}}),
    ]);
    let value = json!({"capture_format":"codex-jsonl/v1","product_version":"0.159.2","session_id":SESSION,"record":{"type":"event_msg","payload":{"type":"task_started"}}});
    native.observations[0].payload = SanitizedPayloadV1::new(
        SanitizedMediaType::Json,
        &serde_json::to_vec(&value).unwrap(),
    )
    .unwrap();
    let desktop = NormalizerProfileV1::codex_desktop_0_159_2();
    assert_eq!(normalize_batch(&desktop, &native).unwrap().records.len(), 1);
    assert!(
        normalize_batch(&NormalizerProfileV1::codex_0_156_1(), &native)
            .unwrap()
            .records
            .is_empty()
    );
    native.observations[0].source_trust = SourceTrust::UntrustedAssertion;
    assert!(
        normalize_batch(&desktop, &native)
            .unwrap()
            .records
            .is_empty()
    );
    native.observations[0].source_trust = SourceTrust::LocalUserHook;
    native.observations[0].source_kind = SourceKind::Hook;
    native.observations[0].payload = SanitizedPayloadV1::new(
        SanitizedMediaType::Json,
        br#"{"hook_event_name":"SessionStart","session_id":"hook-session"}"#,
    )
    .unwrap();
    assert!(
        normalize_batch(&desktop, &native)
            .unwrap()
            .records
            .is_empty()
    );
}
#[test]
fn missing_message_ids_cannot_collide_with_real_native_aliases() {
    let input = batch(vec![
        json!({"type":"response_item","payload":{"type":"message","role":"assistant","content":[{"type":"output_text","text":"without native id"}]}}),
        json!({"type":"response_item","payload":{"type":"message","id":"source-record/1","role":"assistant","content":[{"type":"output_text","text":"with native id"}]}}),
    ]);
    let output = normalize_batch(&NormalizerProfileV1::codex_0_156_1(), &input).unwrap();
    let [
        NormalizedActivityRecordV1::Message(first),
        NormalizedActivityRecordV1::Message(second),
    ] = &output.records[..]
    else {
        panic!("messages")
    };
    assert_ne!(first.message_id, second.message_id);
    assert_eq!(
        output
            .aliases
            .iter()
            .filter(|v| v.native_kind == NativeEntityKind::Message)
            .count(),
        1
    );
}
#[test]
fn unrelated_hook_metadata_preserves_legacy_codex_interpretation() {
    let mut input = batch(vec![json!({})]);
    input.observations[0].source_kind = SourceKind::Hook;
    input.observations[0].source_trust = SourceTrust::LocalUserHook;
    input.observations[0].payload = SanitizedPayloadV1::new(
        SanitizedMediaType::Json,
        br#"{"hook_event_name":"SessionStart","session_id":"legacy-session"}"#,
    )
    .unwrap();
    let before = normalize_batch(&NormalizerProfileV1::codex_0_156_1(), &input).unwrap();
    input.observations[0].payload=SanitizedPayloadV1::new(SanitizedMediaType::Json,br#"{"hook_event_name":"SessionStart","session_id":"legacy-session","capture_format":"unrelated-metadata"}"#).unwrap();
    assert_eq!(
        normalize_batch(&NormalizerProfileV1::codex_0_156_1(), &input).unwrap(),
        before
    );
}

#[test]
fn native_tool_results_enrich_known_calls_without_claiming_success() {
    let input = batch(vec![
        json!({"type":"response_item","payload":{"type":"function_call","call_id":"call-1","name":"functions.exec_command","arguments":"{\"cmd\":\"pwd\"}"}}),
        json!({"type":"response_item","payload":{"type":"function_call_output","call_id":"call-1","status":"completed","output":"Process exited with code 0"}}),
        json!({"type":"response_item","payload":{"type":"custom_tool_call","call_id":"call-2","name":"apply_patch","input":"patch input"}}),
        json!({"type":"response_item","payload":{"type":"custom_tool_call_output","call_id":"call-2","output":{"success":true,"text":"patch accepted"}}}),
    ]);
    let output = normalize_batch(&NormalizerProfileV1::codex_0_156_1(), &input).unwrap();
    let reduced = output.reduce_tools();
    assert_eq!(reduced.tools.len(), 2);
    let shell = reduced
        .tools
        .iter()
        .find(|v| v.native_name == "functions.exec_command")
        .unwrap();
    assert_eq!(shell.kind, ToolKind::Execute);
    assert_eq!(shell.state, ToolState::Unknown);
    assert_eq!(shell.input.as_ref().unwrap().bytes(), br#"{"cmd":"pwd"}"#);
    assert_eq!(
        shell.output.as_ref().unwrap().bytes(),
        b"Process exited with code 0"
    );
    let patch = reduced
        .tools
        .iter()
        .find(|v| v.native_name == "apply_patch")
        .unwrap();
    assert_eq!(patch.kind, ToolKind::Edit);
    assert_eq!(patch.state, ToolState::Unknown);
    assert_eq!(patch.input.as_ref().unwrap().bytes(), b"patch input");
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(patch.output.as_ref().unwrap().bytes())
            .unwrap(),
        json!({"success":true,"text":"patch accepted"})
    );
    assert!(output.diagnostics.is_empty());
    assert_eq!(
        normalize_batch(&NormalizerProfileV1::codex_0_156_1(), &input).unwrap(),
        output
    );
    let mut desktop = input.clone();
    for observation in &mut desktop.observations {
        let mut value: serde_json::Value =
            serde_json::from_slice(observation.payload.bytes()).unwrap();
        value["product_version"] = json!("0.159.2");
        observation.payload = SanitizedPayloadV1::new(
            SanitizedMediaType::Json,
            &serde_json::to_vec(&value).unwrap(),
        )
        .unwrap();
    }
    let desktop = normalize_batch(&NormalizerProfileV1::codex_desktop_0_159_2(), &desktop).unwrap();
    assert_eq!(desktop.reduce_tools(), reduced);
    assert!(desktop.diagnostics.is_empty());
}

#[test]
fn malformed_results_and_foreign_sessions_cannot_invent_calls_or_block_later_records() {
    let mut input = batch(vec![
        json!({"type":"response_item","payload":{"type":"function_call_output","call_id":"absent-output"}}),
        json!({"type":"response_item","payload":{"type":"function_call_output","call_id":"x".repeat(1024),"output":"oversized alias"}}),
        json!({"type":"response_item","payload":{"type":"function_call_output","call_id":"known","output":"foreign session"}}),
        json!({"type":"response_item","payload":{"type":"function_call","call_id":"known","name":"read","arguments":"{}"}}),
        json!({"type":"response_item","payload":{"type":"function_call_output","call_id":"known","output":"same session"}}),
    ]);
    let mut foreign: serde_json::Value =
        serde_json::from_slice(input.observations[2].payload.bytes()).unwrap();
    foreign["session_id"] = json!("00000000-0000-0000-0000-000000000002");
    input.observations[2].payload = SanitizedPayloadV1::new(
        SanitizedMediaType::Json,
        &serde_json::to_vec(&foreign).unwrap(),
    )
    .unwrap();
    let output = normalize_batch(&NormalizerProfileV1::codex_0_156_1(), &input).unwrap();
    assert_eq!(output.diagnostics.len(), 3);
    let reduced = output.reduce_tools();
    assert_eq!(reduced.tools.len(), 1);
    assert_eq!(
        reduced.tools[0].output.as_ref().unwrap().bytes(),
        b"same session"
    );
    assert_eq!(output.records.len(), 2);
    assert_eq!(
        output
            .aliases
            .iter()
            .filter(|v| v.native_kind == NativeEntityKind::Session)
            .count(),
        1
    );
}

#[test]
fn native_result_candidates_are_transient_and_reject_wrong_resolution_identity() {
    let input = batch(vec![
        json!({"type":"response_item","payload":{"type":"function_call_output","call_id":"known","output":"recorded response"}}),
    ]);
    let mut candidates =
        normalize_batch_candidates(&NormalizerProfileV1::codex_0_156_1(), &input).unwrap();
    assert!(candidates.activity.records.is_empty());
    assert!(candidates.activity.aliases.is_empty());
    assert_eq!(candidates.tool_results.len(), 1);
    let wrong_call = batch(vec![
        json!({"type":"response_item","payload":{"type":"function_call","call_id":"different-call","name":"read","arguments":"{}"}}),
    ]);
    let known = normalize_batch(&NormalizerProfileV1::codex_0_156_1(), &wrong_call)
        .unwrap()
        .reduce_tools()
        .tools
        .remove(0);
    assert!(
        candidates
            .activity
            .resolve_tool_result(&candidates.tool_results[0], Some(&known))
            .is_err()
    );
    assert!(candidates.activity.records.is_empty());
    assert!(candidates.activity.aliases.is_empty());
}
