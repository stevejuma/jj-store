use super::*;
use serde_json::{Value, json};

fn native(id: u8, sequence: u64, payload: Value) -> SanitizedObservationBatchV1 {
    let mut batch = batch();
    batch.batch_id = CaptureBatchId::new([id; 16]);
    batch.observations.truncate(1);
    let observation = &mut batch.observations[0];
    observation.observation_id = ObservationId::new([id; 16]);
    observation.source_sequence = sequence;
    observation.source_kind = SourceKind::Transcript;
    observation.source_trust = SourceTrust::LocalTranscript;
    observation.payload = SanitizedPayloadV1::new(
        SanitizedMediaType::Json,
        &serde_json::to_vec(&json!({
            "capture_format":"codex-jsonl/v1", "product_version":"0.156.1",
            "session_id":"00000000-0000-0000-0000-000000000001",
            "record":{"type":"response_item","payload":payload},
        }))
        .unwrap(),
    )
    .unwrap();
    batch
}

#[tokio::test]
async fn local_native_result_enrichment_retains_selected_calls_receipts_and_pins_across_reopen() {
    let (root, repo) = fixture();
    repo.apply_agent_command(&request(150, native(150, 1, json!({
        "type":"custom_tool_call", "call_id":"patch-call", "name":"apply_patch", "input":"patch input",
    })))).await.unwrap();
    let old = repo.agent_store().await.unwrap();
    let (_, before) = session(&old).await;
    let AgentObjectV1::ToolSnapshot(previous) = old
        .object(*before.tools.values().next().unwrap())
        .await
        .unwrap()
    else {
        panic!("tool")
    };
    assert!(previous.output.is_none());
    let command = request(
        151,
        native(
            151,
            2,
            json!({
                "type":"custom_tool_call_output", "call_id":"patch-call", "output":"patch recorded", "status":"completed",
            }),
        ),
    );
    let accepted = repo.apply_agent_command(&command).await.unwrap();
    drop(repo);
    let reopened = reopen(&root.path().join("repo"));
    assert_eq!(
        reopened.apply_agent_command(&command).await.unwrap(),
        accepted
    );
    assert_eq!(
        reopened.agent_command_status(command.request_id).unwrap(),
        accepted
    );
    let current = reopened.agent_store().await.unwrap();
    let (_, after) = session(&current).await;
    let AgentObjectV1::ToolSnapshot(next) = current
        .object(*after.tools.values().next().unwrap())
        .await
        .unwrap()
    else {
        panic!("tool")
    };
    assert_eq!(after.session_id, before.session_id);
    assert_eq!(after.state, before.state);
    assert_eq!(after.evidence.len(), 2);
    assert_eq!(next.tool_call_id, previous.tool_call_id);
    assert_eq!(next.input, previous.input);
    assert_eq!(next.kind, ToolKind::Edit);
    assert_eq!(next.state, ToolState::Unknown);
    assert_eq!(next.evidence.len(), 2);
    let AgentObjectV1::Content(output) = current.object(next.output.unwrap()).await.unwrap() else {
        panic!("output")
    };
    assert_eq!(output.payload.bytes(), b"patch recorded");
    assert_eq!(session(&old).await.1, before);
}

#[tokio::test]
async fn local_common_alias_adoption_survives_reopen_receipt_replay_and_later_source_results() {
    let (root, repo) = fixture();
    let mut first = native(
        184,
        1,
        json!({
            "type":"custom_tool_call","call_id":"patch-call","name":"apply_patch","input":"patch input",
        }),
    );
    first.source_instance_id = SourceInstanceId::new([185; 16]);
    repo.apply_agent_command(&request(184, first))
        .await
        .unwrap();
    let old = repo.agent_store().await.unwrap();
    let (_, before) = session(&old).await;
    let mut result = native(
        186,
        1,
        json!({
            "type":"custom_tool_call_output","call_id":"patch-call","output":"patch recorded",
        }),
    );
    result.source_instance_id = SourceInstanceId::new([187; 16]);
    let command = request(186, result.clone());
    let accepted = repo.apply_agent_command(&command).await.unwrap();
    drop(repo);
    let reopened = reopen(&root.path().join("repo"));
    assert_eq!(
        reopened.apply_agent_command(&command).await.unwrap(),
        accepted
    );
    assert_eq!(
        reopened.agent_command_status(command.request_id).unwrap(),
        accepted
    );
    let current = reopened.agent_store().await.unwrap();
    let (_, after) = session(&current).await;
    assert_eq!(after.session_id, before.session_id);
    assert_eq!(
        after.tools.keys().collect::<Vec<_>>(),
        before.tools.keys().collect::<Vec<_>>()
    );
    assert_eq!(after.evidence.len(), 2);
    assert_eq!(after.coverage.gaps, 0);
    assert_eq!(session(&old).await.1, before);
    result.batch_id = CaptureBatchId::new([188; 16]);
    result.observations[0].observation_id = ObservationId::new([188; 16]);
    result.observations[0].source_sequence = 2;
    reopened
        .apply_agent_command(&request(188, result))
        .await
        .unwrap();
    assert_eq!(
        session(&reopened.agent_store().await.unwrap())
            .await
            .1
            .evidence
            .len(),
        3
    );
}
