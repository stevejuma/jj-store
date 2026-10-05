use super::*;
use jj_agent::{ToolInvocationSnapshotV1, ToolKind, ToolState};
use serde_json::{Value, json};

pub(super) const SESSION: &str = "00000000-0000-0000-0000-000000000001";

pub(super) fn native(
    id: u8,
    sequence: u64,
    session: &str,
    payload: Value,
) -> SanitizedObservationBatchV1 {
    let mut batch = single_capture(
        id,
        sequence,
        json!({
            "capture_format":"codex-jsonl/v1", "product_version":"0.156.1",
            "session_id":session, "record":{"type":"response_item","payload":payload},
        }),
    );
    batch.observations[0].source_kind = SourceKind::Transcript;
    batch.observations[0].source_trust = SourceTrust::LocalTranscript;
    batch
}

pub(super) async fn selected(
    coordinator: &Coordinator,
) -> jj_store_wal::publication::AgentStoreSnapshot {
    let current = coordinator.read().await.unwrap();
    coordinator
        .agent_store(current.manifest().agent_root.unwrap())
        .await
        .unwrap()
}

pub(super) async fn session(
    store: &jj_store_wal::publication::AgentStoreSnapshot,
) -> AgentSessionSnapshotV1 {
    let page = store.list_sessions(None, 10).await.unwrap();
    assert_eq!(page.selections.len(), 1);
    let AgentObjectV1::SessionSnapshot(session) =
        store.read_object(page.selections[0].1).await.unwrap()
    else {
        panic!("session")
    };
    session
}

pub(super) async fn tool(
    store: &jj_store_wal::publication::AgentStoreSnapshot,
    session: &AgentSessionSnapshotV1,
) -> ToolInvocationSnapshotV1 {
    assert_eq!(session.tools.len(), 1);
    let AgentObjectV1::ToolSnapshot(tool) = store
        .read_object(*session.tools.values().next().unwrap())
        .await
        .unwrap()
    else {
        panic!("tool")
    };
    tool
}

#[tokio::test]
async fn result_only_capture_enriches_selected_call_and_preserves_old_root() {
    let backing = Arc::new(MemoryStore::default());
    let coordinator = Coordinator::new(
        backing.clone(),
        REPOSITORY,
        Arc::new(Source(SourceScope::new(REPOSITORY))),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap();
    coordinator
        .initialize(EpochId::new([2; 16]), Vec::new())
        .await
        .unwrap();
    publish_single_capture(&coordinator, &native(140, 1, SESSION, json!({
        "type":"function_call", "call_id":"native-call", "name":"functions.exec_command", "arguments":"{\"cmd\":\"pwd\"}",
    }))).await;
    let old = selected(&coordinator).await;
    let previous = session(&old).await;
    let before = tool(&old, &previous).await;
    assert_eq!(before.state, ToolState::Pending);
    let result = native(
        141,
        2,
        SESSION,
        json!({
            "type":"function_call_output", "call_id":"native-call", "status":"completed", "output":"Process exited with code 0",
        }),
    );
    publish_single_capture(&coordinator, &result).await;
    let current = selected(&coordinator).await;
    let next = session(&current).await;
    let after = tool(&current, &next).await;
    assert_eq!(next.session_id, previous.session_id);
    assert_eq!(next.state, previous.state);
    assert_eq!(next.evidence.len(), 2);
    assert_eq!(next.event_batches.len(), 2);
    assert_eq!(next.coverage.gaps, 0);
    assert_eq!(after.tool_call_id, before.tool_call_id);
    assert_eq!(after.native_name, before.native_name);
    assert_eq!(after.kind, ToolKind::Execute);
    assert_eq!(after.input, before.input);
    assert_eq!(after.state, ToolState::Unknown);
    assert_eq!(after.evidence.len(), 2);
    let AgentObjectV1::Content(output) = current.read_object(after.output.unwrap()).await.unwrap()
    else {
        panic!("output")
    };
    assert_eq!(output.payload.bytes(), b"Process exited with code 0");
    assert_eq!(session(&old).await, previous);
    assert_eq!(tool(&old, &previous).await, before);
    let reopened = Coordinator::new(
        backing,
        REPOSITORY,
        Arc::new(Source(SourceScope::new(REPOSITORY))),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap();
    reopened.recover(100).await.unwrap();
    let recovered = selected(&reopened).await;
    assert_eq!(session(&recovered).await, next);
    assert_eq!(tool(&recovered, &next).await, after);
}

#[tokio::test]
async fn unmatched_results_keep_diagnostics_without_adopting_foreign_calls() {
    let coordinator = fixture().await;
    let base = selected(&coordinator).await;
    let staged = native(
        149,
        1,
        SESSION,
        json!({"type":"function_call", "call_id":"unmatched", "name":"read", "arguments":"{}"}),
    );
    coordinator
        .stage_agent_capture(&base, &NormalizerProfileV1::codex_0_156_1(), &staged, 123)
        .await
        .unwrap();
    let result = native(
        142,
        1,
        SESSION,
        json!({"type":"function_call_output", "call_id":"unmatched", "output":"recorded"}),
    );
    publish_single_capture(&coordinator, &result).await;
    assert!(
        selected(&coordinator)
            .await
            .list_sessions(None, 10)
            .await
            .unwrap()
            .selections
            .is_empty()
    );
    publish_single_capture(
        &coordinator,
        &native(
            143,
            2,
            SESSION,
            json!({"type":"function_call", "call_id":"known", "name":"read", "arguments":"{}"}),
        ),
    )
    .await;
    let known_root = selected(&coordinator).await;
    let known = session(&known_root).await;
    let mut foreign_source = native(
        144,
        3,
        "00000000-0000-0000-0000-000000000003",
        json!({"type":"function_call_output", "call_id":"known", "output":"from another source"}),
    );
    foreign_source.source_instance_id = SourceInstanceId::new([145; 16]);
    publish_single_capture(&coordinator, &foreign_source).await;
    let foreign_session = native(
        146,
        3,
        "00000000-0000-0000-0000-000000000002",
        json!({"type":"function_call_output", "call_id":"known", "output":"from another session"}),
    );
    publish_single_capture(&coordinator, &foreign_session).await;
    let current = selected(&coordinator).await;
    assert_eq!(session(&current).await, known);
    let sources = current
        .list_catalog(AgentCatalogKind::Sources, None, 10)
        .await
        .unwrap();
    assert_eq!(sources.selections.len(), 2);
    for (_, reference) in sources.selections {
        let AgentObjectV1::SourceState(state) = current.read_object(reference).await.unwrap()
        else {
            panic!("source")
        };
        assert!(state.coverage.gaps >= 1);
    }
}

#[tokio::test]
async fn native_results_preserve_selected_terminal_evidence_and_turn_identity() {
    let coordinator = fixture().await;
    let hook = single_capture(
        147,
        1,
        json!({
            "hook_event_name":"PostToolUse", "session_id":SESSION, "turn_id":"native-turn", "tool_use_id":"native-call", "tool_name":"Bash",
            "tool_input":{"command":"test"}, "tool_response":{"exit_code":1,"output":"failed"},
        }),
    );
    publish_single_capture(&coordinator, &hook).await;
    let old = selected(&coordinator).await;
    let previous = session(&old).await;
    let before = tool(&old, &previous).await;
    assert_eq!(before.state, ToolState::Failed);
    publish_single_capture(&coordinator, &native(148, 2, SESSION, json!({
        "type":"function_call_output", "call_id":"native-call", "output":{"status":"success","exit_code":0},
    }))).await;
    let current = selected(&coordinator).await;
    let next = session(&current).await;
    let after = tool(&current, &next).await;
    assert_eq!(after.state, ToolState::Failed);
    assert_eq!(after.native_name, "Bash");
    assert_eq!(after.kind, before.kind);
    assert_eq!(after.turn_id, before.turn_id);
    assert!(after.turn_id.is_some());
    assert_eq!(after.input, before.input);
    assert_eq!(after.output, before.output);
    assert_eq!(after.evidence.len(), 2);
    assert_eq!(next.coverage.gaps, 0);
    assert_eq!(next.state, previous.state);
}
