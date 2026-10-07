use jj_agent::*;
use serde_json::json;
fn batch(sequence: u64, mut payload: serde_json::Value) -> SanitizedObservationBatchV1 {
    payload["controllerSessionId"] = json!("app-session");
    payload["requestId"] = json!("turn-one");
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([sequence as u8; 16]),
        integration: IntegrationId::OwnedController,
        source_instance_id: SourceInstanceId::new([7; 16]),
        sanitizer_version: "test-v1".into(),
        representation: SanitizedRepresentation::CanonicalProducer,
        observations: vec![SanitizedObservationV1 {
            observation_id: ObservationId::new([sequence as u8; 16]),
            source_sequence: sequence,
            occurred_at_millis: Some(1000),
            source_kind: SourceKind::Controller,
            source_trust: SourceTrust::AuthenticatedController,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                &serde_json::to_vec(&payload).unwrap(),
            )
            .unwrap(),
        }],
    }
}
#[test]
fn controller_turns_preserve_identity_and_emit_one_final_assistant_message() {
    let profile = NormalizerProfileV1::acp_controller_v2();
    let chunk=normalize_batch(&profile,&batch(1,json!({"type":"activity","update":{"sessionUpdate":"agent_message_chunk","content":{"text":"part"}}}))).unwrap();
    assert!(chunk.records.is_empty());
    let final_batch = batch(
        2,
        json!({"type":"record","record":{"status":"completed"},"assistantText":"complete"}),
    );
    let first = normalize_batch(&profile, &final_batch).unwrap();
    let replay = normalize_batch(&profile, &final_batch).unwrap();
    assert_eq!(first, replay);
    assert_eq!(first.records.len(), 2);
    assert!(
        matches!(&first.records[1],NormalizedActivityRecordV1::Message(message) if message.role==MessageRole::Assistant)
    );
}
#[test]
fn controller_tool_updates_keep_exact_turn_and_call_identity() {
    let profile = NormalizerProfileV1::acp_controller_v2();
    let first=normalize_batch(&profile,&batch(1,json!({"type":"activity","toolSnapshot":{"toolCallId":"tool","title":"Read","kind":"read","status":"in_progress","rawOutput":null,"content":[]}}))).unwrap();
    let second=normalize_batch(&profile,&batch(2,json!({"type":"activity","toolSnapshot":{"toolCallId":"tool","title":"Read","kind":"read","status":"completed","rawOutput":{"formatted_output":"captured file"}}}))).unwrap();
    let NormalizedActivityRecordV1::Tool(call) = &first.records[0] else {
        panic!()
    };
    let NormalizedActivityRecordV1::Tool(result) = &second.records[0] else {
        panic!()
    };
    assert_eq!(call.tool_call_id, result.tool_call_id);
    assert_eq!(call.turn_id, result.turn_id);
    assert_eq!(result.state, ToolState::Succeeded);
    assert!(call.output.is_none());
    assert!(result.output.is_some());
    let mut combined = first.clone();
    combined.records.extend(second.records);
    assert!(combined.reduce_tools().tools[0].output.is_some());
}
#[test]
fn controller_unknown_or_unusable_turns_never_claim_idle() {
    let profile = NormalizerProfileV1::acp_controller_v2();
    for record in [
        json!({"type":"record", "record":{"status":"outcome_unknown"}, "sessionUsable":false}),
        json!({"type":"record", "record":{"status":"failed"}, "sessionUsable":false}),
        json!({"type":"record", "record":{"status":"timed_out"}}),
    ] {
        let normalized = normalize_batch(&profile, &batch(1, record)).unwrap();
        assert!(
            matches!(&normalized.records[0], NormalizedActivityRecordV1::SessionState(state) if state.state == SessionState::Unknown)
        );
    }
}
#[test]
fn controller_multiline_tool_titles_remain_publishable_without_losing_evidence() {
    let title = format!("Write file\n{}", "📝".repeat(150));
    let observation = batch(
        1,
        json!({"type":"activity","toolSnapshot":{"toolCallId":"cursor-write", "title":title, "status":"completed", "kind":"edit"}}),
    );
    let normalized =
        normalize_batch(&NormalizerProfileV1::acp_controller_v2(), &observation).unwrap();
    let NormalizedActivityRecordV1::Tool(tool) = &normalized.records[0] else {
        panic!()
    };
    let snapshot = ToolInvocationSnapshotV1 {
        schema_version: 1,
        tool_call_id: tool.tool_call_id,
        session_id: tool.session_id,
        turn_id: tool.turn_id,
        native_name: tool.native_name.clone(),
        kind: tool.kind,
        state: tool.state,
        attempts: Default::default(),
        input: None,
        output: None,
        evidence: vec![],
    };
    snapshot.validate().unwrap();
    assert!(tool.native_name.len() <= 256);
    assert!(observation.observations[0].payload.bytes().len() > title.len());
}
#[test]
fn controller_multiline_tool_ids_preserve_exact_identity_in_valid_aliases() {
    let call = "write:/workspace/file\nargument";
    let input = batch(
        1,
        json!({"type":"activity","toolSnapshot":{"toolCallId":call,"title":"Write file","kind":"edit","status":"completed"}}),
    );
    let normalized = normalize_batch(&NormalizerProfileV1::acp_controller_v2(), &input).unwrap();
    for alias in &normalized.aliases {
        alias.validate().unwrap();
    }
    let tool = normalized
        .aliases
        .iter()
        .find(|a| a.native_kind == NativeEntityKind::Tool)
        .unwrap();
    let parts: Vec<String> =
        serde_json::from_str(tool.native_id.strip_prefix("json:").unwrap()).unwrap();
    assert_eq!(parts, ["app-session/turn-one", call]);
    assert_eq!(
        normalize_batch(&NormalizerProfileV1::acp_controller_v2(), &input).unwrap(),
        normalized
    );
}

#[test]
fn sent_context_preserves_human_text_snapshot_and_message_identity_after_delivery() {
    let mut input = json!({"type":"prompt","text":"Instructions plus hello","userText":"hello","sentContext":{"version":1,"blocks":[{"title":"Pinned","text":"revision-one"}],"delivery":"prepared"}});
    let first = normalize_batch(
        &NormalizerProfileV1::acp_controller_v2(),
        &batch(1, input.clone()),
    )
    .unwrap();
    input["type"] = json!("context_delivery");
    input["sentContext"]["delivery"] = json!("sent");
    let second =
        normalize_batch(&NormalizerProfileV1::acp_controller_v2(), &batch(2, input)).unwrap();
    let NormalizedActivityRecordV1::Message(prepared) = &first.records[0] else {
        panic!()
    };
    let NormalizedActivityRecordV1::Message(sent) = &second.records[0] else {
        panic!()
    };
    assert_eq!(prepared.message_id, sent.message_id);
    let data: serde_json::Value = serde_json::from_slice(sent.content.bytes()).unwrap();
    assert_eq!(data["text"], "hello");
    assert_eq!(data["context"]["blocks"][0]["text"], "revision-one");
    assert_eq!(data["context"]["delivery"], "sent");
}

#[test]
fn controller_tool_outputs_retain_raw_file_bytes_and_structured_evidence() {
    let raw = json!({"formatted_output":"export const answer = 42;\n","exit_code":0});
    let content = json!([{ "type":"terminal", "terminalId":"captured" }]);
    let normalized = normalize_batch(&NormalizerProfileV1::acp_controller_v2(), &batch(9,json!({"type":"activity","toolSnapshot":{"toolCallId":"read","title":"Read file","kind":"read","status":"completed","rawOutput":raw,"content":content}}))).unwrap();
    let tool = normalized
        .records
        .iter()
        .find_map(|record| {
            if let NormalizedActivityRecordV1::Tool(tool) = record {
                Some(tool)
            } else {
                None
            }
        })
        .unwrap();
    let value: serde_json::Value =
        serde_json::from_slice(tool.output.as_ref().unwrap().bytes()).unwrap();
    assert_eq!(value["rawOutput"], raw);
    assert_eq!(value["content"], content);
}
