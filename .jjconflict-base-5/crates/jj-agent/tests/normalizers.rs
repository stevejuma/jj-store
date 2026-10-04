use jj_agent::{
    AgentSessionId, IntegrationId, NormalizationDiagnosticKind, NormalizedActivityRecordV1,
    NormalizerProfileV1, ObservationId, SanitizedMediaType, SanitizedObservationBatchV1,
    SanitizedObservationV1, SanitizedPayloadV1, SanitizedRepresentation, SourceInstanceId,
    SourceKind, SourceTrust, ToolKind, ToolState, normalize_batch,
};
use serde_json::{Value, json};

fn fixture(integration: IntegrationId, bytes: &[u8]) -> SanitizedObservationBatchV1 {
    let values: Vec<Value> = serde_json::from_slice(bytes).unwrap();
    fixture_values(integration, values)
}

fn fixture_values(integration: IntegrationId, values: Vec<Value>) -> SanitizedObservationBatchV1 {
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: jj_agent::CaptureBatchId::new([1; 16]),
        integration,
        source_instance_id: SourceInstanceId::new([2; 16]),
        sanitizer_version: "agent-redaction@0.1.0".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: values
            .into_iter()
            .enumerate()
            .map(|(index, value)| SanitizedObservationV1 {
                observation_id: ObservationId::new([index as u8 + 1; 16]),
                source_sequence: index as u64 + 1,
                occurred_at_millis: Some(1_725_000_000_000 + index as i64),
                source_kind: match integration {
                    IntegrationId::OpenCode => SourceKind::Plugin,
                    _ => SourceKind::Hook,
                },
                source_trust: match integration {
                    IntegrationId::OpenCode => SourceTrust::LocallyTrustedPlugin,
                    _ => SourceTrust::LocalUserHook,
                },
                payload: SanitizedPayloadV1::new(
                    SanitizedMediaType::Json,
                    &serde_json::to_vec(&value).unwrap(),
                )
                .unwrap(),
            })
            .collect(),
    }
}

fn tool_records(records: &[NormalizedActivityRecordV1]) -> Vec<&jj_agent::NormalizedToolRecordV1> {
    records
        .iter()
        .filter_map(|record| match record {
            NormalizedActivityRecordV1::Tool(value) => Some(value),
            _ => None,
        })
        .collect()
}

fn assert_semantic_tools(output: &jj_agent::NormalizationBatchV1) {
    let tools = tool_records(&output.records);
    assert!(
        tools
            .iter()
            .any(|tool| { tool.kind == ToolKind::Edit && tool.state == ToolState::Succeeded })
    );
    assert!(
        tools
            .iter()
            .any(|tool| tool.kind == ToolKind::Execute && tool.state == ToolState::Failed)
    );
    assert!(tools.iter().any(|tool| tool.kind == ToolKind::Read));
    assert!(output.diagnostics.is_empty(), "{:?}", output.diagnostics);
}

#[test]
fn pinned_harness_fixtures_normalize_to_common_activity() {
    let cases = [
        (
            NormalizerProfileV1::codex_0_156_1(),
            fixture(
                IntegrationId::Codex,
                include_bytes!("fixtures/codex-0.156.1-hooks.json"),
            ),
        ),
        (
            NormalizerProfileV1::cursor_3_22_7(),
            fixture(
                IntegrationId::Cursor,
                include_bytes!("fixtures/cursor-3.22.7-hooks.json"),
            ),
        ),
        (
            NormalizerProfileV1::opencode_1_18_30(),
            fixture(
                IntegrationId::OpenCode,
                include_bytes!("fixtures/opencode-1.18.30-plugin.json"),
            ),
        ),
    ];
    for (profile, batch) in cases {
        let first = normalize_batch(&profile, &batch).unwrap();
        let replay = normalize_batch(&profile, &batch).unwrap();
        assert_eq!(first, replay);
        assert_semantic_tools(&first);
        assert!(first.records.iter().any(|record| matches!(
            record,
            NormalizedActivityRecordV1::SessionState(value)
                if value.state == jj_agent::SessionState::Active
        )));
        assert!(!first.aliases.is_empty());
    }
}

#[test]
fn lifecycle_records_share_ids_and_reducer_never_regresses_terminal_state() {
    let batch = fixture(
        IntegrationId::OpenCode,
        include_bytes!("fixtures/opencode-1.18.30-plugin.json"),
    );
    let output = normalize_batch(&NormalizerProfileV1::opencode_1_18_30(), &batch).unwrap();
    let tools = tool_records(&output.records);
    let shell: Vec<_> = tools
        .into_iter()
        .filter(|tool| tool.native_name == "bash")
        .collect();
    assert!(shell.len() >= 2);
    assert!(
        shell
            .iter()
            .all(|tool| tool.tool_call_id == shell[0].tool_call_id)
    );
    let reduced = output.reduce_tools();
    let shell = reduced
        .tools
        .iter()
        .find(|tool| tool.tool_call_id == shell[0].tool_call_id)
        .unwrap();
    assert_eq!(shell.state, ToolState::Failed);
    assert!(shell.input.is_some());
    assert!(shell.output.is_some());
}

#[test]
fn unsupported_versions_degrade_without_parsing_native_records() {
    let batch = fixture(
        IntegrationId::Codex,
        include_bytes!("fixtures/codex-0.156.1-hooks.json"),
    );
    let output = normalize_batch(
        &NormalizerProfileV1 {
            integration: IntegrationId::Codex,
            product_version: "0.157.0".into(),
            schema_version: 1,
        },
        &batch,
    )
    .unwrap();
    assert!(output.records.is_empty());
    assert!(output.aliases.is_empty());
    assert_eq!(
        output.diagnostics[0].kind,
        NormalizationDiagnosticKind::UnsupportedVersion
    );
}

#[test]
fn resumed_sessions_and_mixed_sources_keep_stable_ids() {
    let batch = fixture(
        IntegrationId::Codex,
        include_bytes!("fixtures/codex-0.156.1-hooks.json"),
    );
    let output = normalize_batch(&NormalizerProfileV1::codex_0_156_1(), &batch).unwrap();
    let sessions: Vec<AgentSessionId> = output
        .records
        .iter()
        .filter_map(|record| match record {
            NormalizedActivityRecordV1::SessionState(value) => Some(value.session_id),
            _ => None,
        })
        .collect();
    assert!(sessions.len() >= 3);
    assert!(sessions.iter().all(|session| *session == sessions[0]));
}

#[test]
fn cursor_file_observations_do_not_invent_tools_and_unknown_tools_stay_other() {
    let batch = fixture(
        IntegrationId::Cursor,
        include_bytes!("fixtures/cursor-3.22.7-hooks.json"),
    );
    let output = normalize_batch(&NormalizerProfileV1::cursor_3_22_7(), &batch).unwrap();
    let file_sequence = 9;
    assert!(output.records.iter().any(|record| matches!(
        record,
        NormalizedActivityRecordV1::FileObservation(value)
            if value.source_sequence == file_sequence
    )));
    assert!(
        !tool_records(&output.records)
            .iter()
            .any(|tool| tool.source_sequence == file_sequence)
    );

    let batch = fixture_values(
        IntegrationId::Cursor,
        vec![json!({
            "hook_event_name": "preToolUse",
            "conversation_id": "conv_demo",
            "generation_id": "gen_1",
            "cursor_version": "3.22.7",
            "tool_name": "future_native_tool",
            "tool_use_id": "future_1",
            "tool_input": {}
        })],
    );
    let output = normalize_batch(&NormalizerProfileV1::cursor_3_22_7(), &batch).unwrap();
    assert_eq!(tool_records(&output.records)[0].kind, ToolKind::Other);
}

#[test]
fn malformed_unknown_and_embedded_version_failures_are_safe_diagnostics() {
    let batch = fixture_values(
        IntegrationId::Cursor,
        vec![
            json!({"secret": "must-not-appear"}),
            json!({
                "hook_event_name": "futureEvent",
                "conversation_id": "conv_demo",
                "generation_id": "gen_1",
                "cursor_version": "3.22.7"
            }),
            json!({
                "hook_event_name": "sessionStart",
                "conversation_id": "conv_demo",
                "generation_id": "gen_1",
                "cursor_version": "3.23.0"
            }),
        ],
    );
    let output = normalize_batch(&NormalizerProfileV1::cursor_3_22_7(), &batch).unwrap();
    assert!(output.records.is_empty());
    assert_eq!(
        output
            .diagnostics
            .iter()
            .map(|value| (value.kind, value.source_sequence))
            .collect::<Vec<_>>(),
        vec![
            (NormalizationDiagnosticKind::InvalidRecord, Some(1)),
            (NormalizationDiagnosticKind::UnsupportedEvent, Some(2)),
            (
                NormalizationDiagnosticKind::EmbeddedVersionMismatch,
                Some(3)
            ),
        ]
    );
    let encoded = serde_json::to_string(&output.diagnostics).unwrap();
    assert!(!encoded.contains("must-not-appear"));
}

#[test]
fn reducer_keeps_conflicting_terminal_states_explicit() {
    let batch = fixture(
        IntegrationId::OpenCode,
        include_bytes!("fixtures/opencode-1.18.30-plugin.json"),
    );
    let mut output = normalize_batch(&NormalizerProfileV1::opencode_1_18_30(), &batch).unwrap();
    let mut conflicting = tool_records(&output.records)
        .into_iter()
        .find(|tool| tool.native_name == "bash" && tool.state == ToolState::Failed)
        .unwrap()
        .clone();
    conflicting.source_sequence = 100;
    conflicting.state = ToolState::Succeeded;
    output
        .records
        .push(NormalizedActivityRecordV1::Tool(conflicting));

    let reduced = output.reduce_tools();
    let tool = reduced
        .tools
        .iter()
        .find(|tool| tool.native_name == "bash")
        .unwrap();
    assert_eq!(tool.state, ToolState::Failed);
    assert_eq!(
        tool.conflicting_terminal_states,
        vec![ToolState::Failed, ToolState::Succeeded]
    );
    assert_eq!(
        reduced.diagnostics[0].kind,
        NormalizationDiagnosticKind::ConflictingTerminalState
    );
}
