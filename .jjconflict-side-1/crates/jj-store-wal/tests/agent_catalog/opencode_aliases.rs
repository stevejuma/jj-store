use super::native_results::{selected, session, tool};
use super::*;
use jj_agent::{AgentEntityRefV1, ToolState};
use serde_json::json;

const NATIVE: &str = "ses_capture";
fn capture(id: u8, source: u8, kind: SourceKind, native: &str) -> SanitizedObservationBatchV1 {
    let payload = match kind {
        SourceKind::Hook => {
            json!({"hook":"tool.execute.before","input":{"sessionID":native,"callID":"call_capture","tool":"bash","args":{"command":"test"}}})
        }
        _ => {
            json!({"capture_format":"opencode-snapshot/v1","product_version":"1.18.30","session_id":native,"record":{"type":"message","info":{"id":"msg_capture","sessionID":native,"role":"assistant"},"parts":[{"id":"prt_capture","sessionID":native,"messageID":"msg_capture","type":"tool","callID":"call_capture","tool":"bash","state":{"status":"completed","input":{"command":"test"},"output":"recorded result"}}]}})
        }
    };
    let mut batch = single_capture(id, 1, payload);
    batch.integration = IntegrationId::OpenCode;
    batch.source_instance_id = SourceInstanceId::new([source; 16]);
    batch.observations[0].source_kind = kind;
    batch.observations[0].source_trust = match kind {
        SourceKind::Hook => SourceTrust::LocalUserHook,
        SourceKind::NativeApi => SourceTrust::ApprovedHarnessApi,
        _ => SourceTrust::LocalTranscript,
    };
    batch
}
async fn delta(coordinator: &Coordinator, batch: &SanitizedObservationBatchV1) -> AgentDeltaV1 {
    coordinator
        .stage_agent_capture(
            &selected(coordinator).await,
            &NormalizerProfileV1::opencode_1_18_30(),
            batch,
            123,
        )
        .await
        .unwrap()
}
async fn publish(coordinator: &Coordinator, batch: &SanitizedObservationBatchV1) {
    publish_transcript_edits(
        coordinator,
        batch.batch_id.as_bytes()[0],
        delta(coordinator, batch).await.edits,
    )
    .await;
}

#[tokio::test]
async fn opencode_hook_api_and_export_adopt_one_selected_session_message_and_tool() {
    let coordinator = fixture().await;
    publish(&coordinator, &capture(210, 211, SourceKind::Hook, NATIVE)).await;
    let old = selected(&coordinator).await;
    let before = session(&old).await;
    let original_tool = tool(&old, &before).await;
    assert_eq!(original_tool.state, ToolState::Running);
    for (id, source, kind) in [
        (212, 213, SourceKind::NativeApi),
        (214, 215, SourceKind::SessionExport),
    ] {
        let batch = capture(id, source, kind, NATIVE);
        publish(&coordinator, &batch).await;
        let current = selected(&coordinator).await;
        let after = session(&current).await;
        assert_eq!(after.session_id, before.session_id);
        let next = tool(&current, &after).await;
        assert_eq!(next.tool_call_id, original_tool.tool_call_id);
        assert_eq!(next.state, ToolState::Succeeded);
        for alias in jj_agent::normalize_batch(&NormalizerProfileV1::opencode_1_18_30(), &batch)
            .unwrap()
            .aliases
        {
            let reference = current
                .lookup(AgentCatalogKind::Aliases, alias.catalog_key().unwrap())
                .await
                .unwrap()
                .unwrap();
            let AgentObjectV1::Alias(adopted) = current.read_object(reference).await.unwrap()
            else {
                panic!("alias")
            };
            match adopted.target {
                AgentEntityRefV1::Session { session_id } => {
                    assert_eq!(session_id, before.session_id)
                }
                AgentEntityRefV1::Tool {
                    session_id,
                    tool_call_id,
                } => {
                    assert_eq!(session_id, before.session_id);
                    assert_eq!(tool_call_id, original_tool.tool_call_id);
                }
                AgentEntityRefV1::Message { session_id, .. } => {
                    assert_eq!(session_id, before.session_id)
                }
                _ => panic!("unexpected alias"),
            }
        }
    }
    let current = selected(&coordinator).await;
    let after = session(&current).await;
    assert_eq!(after.evidence.len(), 3);
    assert_eq!(tool(&current, &after).await.evidence.len(), 3);
    assert_eq!(after.coverage.gaps, 0);
    assert_eq!(
        current
            .list_catalog(AgentCatalogKind::Sources, None, 10)
            .await
            .unwrap()
            .selections
            .len(),
        3
    );
    let mut messages = Vec::new();
    for reference in &after.event_batches {
        let AgentObjectV1::EventBatch(batch) = current.read_object(*reference).await.unwrap()
        else {
            panic!("events")
        };
        for event in batch.events {
            if let jj_agent::AgentEventV1::MessageCompleted { message_id, .. } = event {
                messages.push(message_id);
            }
        }
    }
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0], messages[1]);
    assert_eq!(session(&old).await, before);
    assert_eq!(tool(&old, &before).await, original_tool);
}

#[tokio::test]
async fn opencode_native_ids_are_case_sensitive_and_generic_labels_remain_source_scoped() {
    for (a, b) in [
        ("ses_capture", "ses_CAPTURE"),
        ("shared-label", "shared-label"),
    ] {
        let coordinator = fixture().await;
        publish(&coordinator, &capture(216, 217, SourceKind::Hook, a)).await;
        publish(&coordinator, &capture(218, 219, SourceKind::Hook, b)).await;
        assert_eq!(
            selected(&coordinator)
                .await
                .list_sessions(None, 10)
                .await
                .unwrap()
                .selections
                .len(),
            2
        );
    }
}

#[tokio::test]
async fn opencode_common_join_retains_selected_precondition_across_sources() {
    let coordinator = fixture().await;
    let batch = capture(220, 221, SourceKind::Hook, NATIVE);
    let edits = delta(&coordinator, &batch).await.edits;
    let key = jj_agent::AgentProjectionCatalogV1 {
        schema_version: 1,
        normalizer_version: "opencode-native-alias-join/v1".into(),
        activated_at_millis: 123,
        sessions: vec![],
        event_batches: vec![],
        source_states: vec![],
    }
    .catalog_key()
    .unwrap();
    assert!(
        edits
            .iter()
            .any(|e| e.catalog == AgentCatalogKind::Projections
                && e.key == key
                && e.expected.is_none())
    );
    publish_transcript_edits(&coordinator, 220, edits).await;
    let current = selected(&coordinator).await;
    let expected = current
        .lookup(AgentCatalogKind::Projections, key)
        .await
        .unwrap();
    assert!(expected.is_some());
    let next = delta(
        &coordinator,
        &capture(222, 223, SourceKind::SessionExport, NATIVE),
    )
    .await;
    assert!(
        next.edits
            .iter()
            .any(|e| e.catalog == AgentCatalogKind::Projections
                && e.key == key
                && e.expected == expected)
    );
}

#[tokio::test]
async fn opencode_ambiguous_prior_aliases_keep_receipts_and_withhold_session_edits() {
    let coordinator = fixture().await;
    let first = capture(224, 225, SourceKind::Hook, NATIVE);
    let second = capture(226, 227, SourceKind::NativeApi, NATIVE);
    let initial = selected(&coordinator).await;
    let mut legacy = coordinator
        .stage_agent_capture(
            &initial,
            &NormalizerProfileV1::opencode_1_18_30(),
            &second,
            123,
        )
        .await
        .unwrap();
    publish(&coordinator, &first).await;
    let current = selected(&coordinator).await;
    for edit in &mut legacy.edits {
        if edit.catalog == AgentCatalogKind::Projections {
            edit.expected = current.lookup(edit.catalog, edit.key).await.unwrap();
        }
    }
    publish_transcript_edits(&coordinator, 226, legacy.edits).await;
    let prior = selected(&coordinator)
        .await
        .list_sessions(None, 10)
        .await
        .unwrap();
    assert_eq!(prior.selections.len(), 2);
    let batch = capture(228, 229, SourceKind::SessionExport, NATIVE);
    let blocked = delta(&coordinator, &batch).await;
    assert!(blocked.edits.iter().all(|e| matches!(
        e.catalog,
        AgentCatalogKind::Sources | AgentCatalogKind::Projections
    )));
    publish_transcript_edits(&coordinator, 228, blocked.edits).await;
    let current = selected(&coordinator).await;
    assert_eq!(current.list_sessions(None, 10).await.unwrap(), prior);
    let key = jj_agent::CaptureSourceStateV1::source_catalog_key(
        IntegrationId::OpenCode,
        batch.source_instance_id,
    )
    .unwrap();
    let AgentObjectV1::SourceState(state) = current
        .read_object(
            current
                .lookup(AgentCatalogKind::Sources, key)
                .await
                .unwrap()
                .unwrap(),
        )
        .await
        .unwrap()
    else {
        panic!("source")
    };
    assert_eq!(state.coverage.gaps, 1);
    assert_eq!(state.receipts[0].batch_id, batch.batch_id);
}
