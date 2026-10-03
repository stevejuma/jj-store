use super::*;
use serde_json::json;

fn capture(id: u8, source: u8, kind: SourceKind) -> LocalAgentRequest {
    let payload = match kind {
        SourceKind::Hook => {
            json!({"hook":"tool.execute.before","input":{"sessionID":"ses_local","callID":"call_local","tool":"read","args":{"path":"recorded"}}})
        }
        _ => {
            json!({"capture_format":"opencode-snapshot/v1","product_version":"1.18.30","session_id":"ses_local","record":{"type":"message","info":{"id":"msg_local","sessionID":"ses_local","role":"assistant"},"parts":[{"id":"prt_local","sessionID":"ses_local","messageID":"msg_local","type":"tool","callID":"call_local","tool":"read","state":{"status":"completed","input":{"path":"recorded"},"output":"result"}}]}})
        }
    };
    let mut batch = batch();
    batch.integration = IntegrationId::OpenCode;
    batch.batch_id = CaptureBatchId::new([id; 16]);
    batch.source_instance_id = SourceInstanceId::new([source; 16]);
    batch.observations.truncate(1);
    let observation = &mut batch.observations[0];
    observation.observation_id = ObservationId::new([id; 16]);
    observation.source_sequence = 1;
    observation.source_kind = kind;
    observation.source_trust = match kind {
        SourceKind::Hook => SourceTrust::LocalUserHook,
        SourceKind::NativeApi => SourceTrust::ApprovedHarnessApi,
        _ => SourceTrust::LocalTranscript,
    };
    observation.payload = SanitizedPayloadV1::new(
        SanitizedMediaType::Json,
        &serde_json::to_vec(&payload).unwrap(),
    )
    .unwrap();
    LocalAgentRequest {
        version: 1,
        request_id: m::RequestId::new([id; 16]),
        command: LocalAgentCommand::Capture {
            profile: NormalizerProfileV1::opencode_1_18_30(),
            batch: Box::new(batch),
        },
    }
}

#[tokio::test]
async fn local_opencode_sources_share_selected_ids_across_reopen_and_exact_receipt_replay() {
    let (root, repo) = fixture();
    repo.apply_agent_command(&capture(210, 211, SourceKind::Hook))
        .await
        .unwrap();
    let old = repo.agent_store().await.unwrap();
    let (_, before) = session(&old).await;
    let id = *before.tools.keys().next().unwrap();
    repo.apply_agent_command(&capture(212, 213, SourceKind::NativeApi))
        .await
        .unwrap();
    drop(repo);
    let reopened = reopen(&root.path().join("repo"));
    let command = capture(214, 215, SourceKind::SessionExport);
    let receipt = reopened.apply_agent_command(&command).await.unwrap();
    assert_eq!(
        reopened.apply_agent_command(&command).await.unwrap(),
        receipt
    );
    assert_eq!(
        reopened.agent_command_status(command.request_id).unwrap(),
        receipt
    );
    let current = reopened.agent_store().await.unwrap();
    let (_, after) = session(&current).await;
    assert_eq!(after.session_id, before.session_id);
    assert_eq!(after.evidence.len(), 3);
    assert_eq!(after.tools.keys().copied().collect::<Vec<_>>(), vec![id]);
    let AgentObjectV1::ToolSnapshot(tool) = current.object(after.tools[&id]).await.unwrap() else {
        panic!("tool")
    };
    assert_eq!(tool.state, ToolState::Succeeded);
    assert_eq!(tool.evidence.len(), 3);
    assert_eq!(session(&old).await.1, before);
}
