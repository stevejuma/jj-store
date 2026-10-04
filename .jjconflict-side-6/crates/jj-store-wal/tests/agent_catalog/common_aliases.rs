use super::native_results::{SESSION, native, selected, session, tool};
use super::*;
use jj_agent::{AgentEntityRefV1, ToolState};
use serde_json::json;

fn hook(id: u8, source: u8, sequence: u64) -> SanitizedObservationBatchV1 {
    let mut batch = single_capture(
        id,
        sequence,
        json!({
            "hook_event_name":"PostToolUse", "session_id":SESSION, "turn_id":"native-turn",
            "tool_use_id":"native-call", "tool_name":"Bash", "tool_input":{"command":"test"},
            "tool_response":{"exit_code":1,"output":"failed"},
        }),
    );
    batch.source_instance_id = SourceInstanceId::new([source; 16]);
    batch
}

#[tokio::test]
async fn hook_and_transcript_channels_adopt_selected_native_session_and_tool_ids() {
    let coordinator = fixture().await;
    publish_single_capture(&coordinator, &hook(160, 161, 1)).await;
    let old = selected(&coordinator).await;
    let before = session(&old).await;
    let original_tool = tool(&old, &before).await;
    let mut transcript = native(
        162,
        1,
        SESSION,
        json!({
            "type":"function_call", "call_id":"native-call", "name":"functions.exec_command", "arguments":"{}",
        }),
    );
    transcript.source_instance_id = SourceInstanceId::new([163; 16]);
    publish_single_capture(&coordinator, &transcript).await;
    let current = selected(&coordinator).await;
    let after = session(&current).await;
    let enriched = tool(&current, &after).await;
    assert_eq!(after.session_id, before.session_id);
    assert_eq!(enriched.tool_call_id, original_tool.tool_call_id);
    assert_eq!(enriched.state, ToolState::Failed);
    assert_eq!(enriched.output, original_tool.output);
    assert_eq!(enriched.turn_id, original_tool.turn_id);
    assert_eq!(after.evidence.len(), 2);
    assert_eq!(after.coverage.gaps, 0);
    assert_eq!(session(&old).await, before);
    let normalized =
        jj_agent::normalize_batch(&NormalizerProfileV1::codex_0_156_1(), &transcript).unwrap();
    for mut alias in normalized.aliases {
        let reference = current
            .lookup(AgentCatalogKind::Aliases, alias.catalog_key().unwrap())
            .await
            .unwrap()
            .unwrap();
        let AgentObjectV1::Alias(selected_alias) = current.read_object(reference).await.unwrap()
        else {
            panic!("alias")
        };
        match &mut alias.target {
            AgentEntityRefV1::Session { session_id } => *session_id = before.session_id,
            AgentEntityRefV1::Tool {
                session_id,
                tool_call_id,
            } => {
                *session_id = before.session_id;
                *tool_call_id = original_tool.tool_call_id;
            }
            _ => panic!("unexpected alias"),
        }
        assert_eq!(selected_alias, alias);
    }
    let result = native(
        164,
        2,
        SESSION,
        json!({"type":"function_call_output", "call_id":"native-call", "output":"retained"}),
    );
    let mut result = result;
    result.source_instance_id = transcript.source_instance_id;
    publish_single_capture(&coordinator, &result).await;
    assert_eq!(
        session(&selected(&coordinator).await).await.evidence.len(),
        3
    );
}

#[tokio::test]
async fn result_only_new_channel_adopts_a_published_call_without_new_session_or_lifecycle() {
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
    publish_single_capture(&coordinator, &hook(165, 166, 1)).await;
    let old = selected(&coordinator).await;
    let before = session(&old).await;
    let original_tool = tool(&old, &before).await;
    let mut result = native(
        167,
        1,
        SESSION,
        json!({
            "type":"function_call_output", "call_id":"native-call", "output":"recorded",
        }),
    );
    result.source_instance_id = SourceInstanceId::new([168; 16]);
    let delta = coordinator
        .stage_agent_capture(&old, &NormalizerProfileV1::codex_0_156_1(), &result, 123)
        .await
        .unwrap();
    let request = AgentDeltaRequest::new(
        jj_store_model::PrincipalId::new([72; 16]),
        jj_store_model::RequestId::new([167; 16]),
        jj_store_model::PublicationId::new([167; 16]),
        delta,
        b"joined result".to_vec(),
    )
    .unwrap();
    let accepted = coordinator.publish_agent_delta(&request).await.unwrap();
    let current = selected(&coordinator).await;
    let after = session(&current).await;
    assert_eq!(after.session_id, before.session_id);
    assert_eq!(after.state, before.state);
    assert_eq!(after.evidence.len(), 2);
    assert_eq!(after.coverage.gaps, 0);
    let next = tool(&current, &after).await;
    assert_eq!(next.tool_call_id, original_tool.tool_call_id);
    assert_eq!(next.state, ToolState::Failed);
    assert_eq!(next.output, original_tool.output);
    assert_eq!(next.evidence.len(), 2);
    let recovered = Coordinator::new(
        backing,
        REPOSITORY,
        Arc::new(Source(SourceScope::new(REPOSITORY))),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap();
    recovered.recover(100).await.unwrap();
    assert_eq!(
        recovered.publish_agent_delta(&request).await.unwrap(),
        accepted
    );
    let cold = selected(&recovered).await;
    assert_eq!(session(&cold).await, after);
    assert_eq!(tool(&cold, &after).await, next);
}

#[tokio::test]
async fn transcript_first_ids_survive_later_hook_channel_and_an_independent_source_cursor() {
    let coordinator = fixture().await;
    let mut first = native(
        169,
        1,
        SESSION,
        json!({
            "type":"function_call", "call_id":"native-call", "name":"functions.exec_command", "arguments":"{}",
        }),
    );
    first.source_instance_id = SourceInstanceId::new([170; 16]);
    publish_single_capture(&coordinator, &first).await;
    let old = selected(&coordinator).await;
    let before = session(&old).await;
    let original_tool = tool(&old, &before).await;
    publish_single_capture(&coordinator, &hook(171, 172, 1)).await;
    let current = selected(&coordinator).await;
    let after = session(&current).await;
    assert_eq!(after.session_id, before.session_id);
    let next = tool(&current, &after).await;
    assert_eq!(next.tool_call_id, original_tool.tool_call_id);
    assert_eq!(next.state, ToolState::Failed);
    assert_eq!(after.evidence.len(), 2);
    assert_eq!(
        current
            .list_catalog(AgentCatalogKind::Sources, None, 10)
            .await
            .unwrap()
            .selections
            .len(),
        2
    );
}

#[tokio::test]
async fn conflicting_legacy_native_sessions_withhold_activity_but_keep_source_progress() {
    let coordinator = fixture().await;
    let empty = selected(&coordinator).await;
    publish_single_capture(&coordinator, &hook(173, 174, 1)).await;
    let mut legacy = native(
        175,
        1,
        SESSION,
        json!({
            "type":"function_call", "call_id":"native-call", "name":"read", "arguments":"{}",
        }),
    );
    legacy.source_instance_id = SourceInstanceId::new([176; 16]);
    // Reconstruct pre-join legacy state from a retained empty root, without changing IDs.
    let mut delta = coordinator
        .stage_agent_capture(&empty, &NormalizerProfileV1::codex_0_156_1(), &legacy, 123)
        .await
        .unwrap();
    let current = selected(&coordinator).await;
    for edit in &mut delta.edits {
        if edit.catalog == AgentCatalogKind::Projections {
            edit.expected = current.lookup(edit.catalog, edit.key).await.unwrap();
        }
    }
    publish_transcript_edits(&coordinator, 175, delta.edits).await;
    let before = selected(&coordinator).await;
    let retained = before.list_sessions(None, 10).await.unwrap();
    assert_eq!(retained.selections.len(), 2);
    let blocked = hook(177, 178, 1);
    let delta = coordinator
        .stage_agent_capture(
            &before,
            &NormalizerProfileV1::codex_0_156_1(),
            &blocked,
            123,
        )
        .await
        .unwrap();
    assert_eq!(delta.edits.len(), 2);
    assert!(delta.edits.iter().all(|e| matches!(
        e.catalog,
        AgentCatalogKind::Sources | AgentCatalogKind::Projections
    )));
    publish_transcript_edits(&coordinator, 177, delta.edits).await;
    let after = selected(&coordinator).await;
    assert_eq!(after.list_sessions(None, 10).await.unwrap(), retained);
    let key = jj_agent::CaptureSourceStateV1::source_catalog_key(
        IntegrationId::Codex,
        blocked.source_instance_id,
    )
    .unwrap();
    let AgentObjectV1::SourceState(state) = after
        .read_object(
            after
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
    assert_eq!(state.receipts[0].batch_id, blocked.batch_id);
    let mut later = native(
        179,
        2,
        "00000000-0000-0000-0000-000000000002",
        json!({
            "type":"message","role":"assistant","content":[{"type":"output_text","text":"later valid record"}],
        }),
    );
    later.source_instance_id = blocked.source_instance_id;
    publish_single_capture(&coordinator, &later).await;
    assert_eq!(
        selected(&coordinator)
            .await
            .list_sessions(None, 10)
            .await
            .unwrap()
            .selections
            .len(),
        3
    );
}

#[tokio::test]
async fn non_uuid_labels_remain_source_scoped_and_never_join_by_clock_or_content() {
    let coordinator = fixture().await;
    for (id, source) in [(180, 181), (182, 183)] {
        let mut batch = single_capture(
            id,
            1,
            json!({
                "hook_event_name":"PreToolUse","session_id":"shared-label","tool_use_id":"same-call","tool_name":"read",
            }),
        );
        batch.source_instance_id = SourceInstanceId::new([source; 16]);
        publish_single_capture(&coordinator, &batch).await;
    }
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

#[tokio::test]
async fn common_message_and_turn_aliases_use_uuid_identity_and_keep_missing_ids_source_scoped() {
    let coordinator = fixture().await;
    let native_session = "abcdef00-0000-0000-0000-000000000001";
    for (id, source, session_key) in [
        (189, 190, native_session.to_owned()),
        (191, 192, native_session.to_uppercase()),
    ] {
        let mut batch = native(
            id,
            1,
            &session_key,
            json!({
                "type":"message","id":"message-id","turn_id":"turn-id","role":"assistant",
                "content":[{"type":"output_text","text":"same native message"}],
            }),
        );
        batch.source_instance_id = SourceInstanceId::new([source; 16]);
        publish_single_capture(&coordinator, &batch).await;
    }
    let current = selected(&coordinator).await;
    let retained = session(&current).await;
    let mut messages = Vec::new();
    for reference in &retained.event_batches {
        let AgentObjectV1::EventBatch(batch) = current.read_object(*reference).await.unwrap()
        else {
            panic!("batch")
        };
        for event in batch.events {
            if let jj_agent::AgentEventV1::MessageCompleted {
                session_id,
                turn_id,
                message_id,
                ..
            } = event
            {
                assert_eq!(session_id, retained.session_id);
                messages.push((turn_id, message_id));
            }
        }
    }
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0], messages[1]);
    assert!(messages[0].0.is_some());
    for (id, source) in [(193, 194), (195, 196)] {
        let mut batch = native(
            id,
            1,
            native_session,
            json!({
                "type":"message","role":"assistant","content":[{"type":"output_text","text":"no native id"}],
            }),
        );
        batch.source_instance_id = SourceInstanceId::new([source; 16]);
        publish_single_capture(&coordinator, &batch).await;
    }
    let current = selected(&coordinator).await;
    let retained = session(&current).await;
    let mut fallback = Vec::new();
    for reference in &retained.event_batches[2..] {
        let AgentObjectV1::EventBatch(batch) = current.read_object(*reference).await.unwrap()
        else {
            panic!("batch")
        };
        for event in batch.events {
            if let jj_agent::AgentEventV1::MessageCompleted { message_id, .. } = event {
                fallback.push(message_id);
            }
        }
    }
    assert_eq!(fallback.len(), 2);
    assert_ne!(fallback[0], fallback[1]);
    assert_ne!(fallback[0], messages[0].1);
}

#[tokio::test]
async fn conflicting_tool_aliases_never_choose_one_or_replace_accepted_aliases() {
    let coordinator = fixture().await;
    publish_single_capture(&coordinator, &hook(197, 198, 1)).await;
    let base = selected(&coordinator).await;
    let previous = session(&base).await;
    let mut conflicting_tool = tool(&base, &previous).await;
    conflicting_tool.tool_call_id = jj_agent::ToolCallId::new([199; 16]);
    let tool_ref = coordinator
        .stage_agent_object(&AgentObjectV1::ToolSnapshot(conflicting_tool.clone()))
        .await
        .unwrap();
    let mut next = previous.clone();
    next.tools.insert(conflicting_tool.tool_call_id, tool_ref);
    let session_ref = coordinator
        .stage_agent_object(&AgentObjectV1::SessionSnapshot(next))
        .await
        .unwrap();
    let alias = jj_agent::AgentAliasV1 {
        schema_version: 1,
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([200; 16]),
        native_kind: jj_agent::NativeEntityKind::Tool,
        native_id: format!("{SESSION}/native-call"),
        target: AgentEntityRefV1::Tool {
            session_id: previous.session_id,
            tool_call_id: conflicting_tool.tool_call_id,
        },
    };
    let alias_ref = coordinator
        .stage_agent_object(&AgentObjectV1::Alias(alias.clone()))
        .await
        .unwrap();
    let (key, expected) = base.list_sessions(None, 10).await.unwrap().selections[0];
    publish_transcript_edits(
        &coordinator,
        201,
        vec![
            AgentCatalogEditV1 {
                catalog: AgentCatalogKind::Sessions,
                key,
                expected: Some(expected),
                replacement: Some(session_ref),
            },
            AgentCatalogEditV1 {
                catalog: AgentCatalogKind::Aliases,
                key: alias.catalog_key().unwrap(),
                expected: None,
                replacement: Some(alias_ref),
            },
        ],
    )
    .await;
    let before = selected(&coordinator).await;
    let selected_session = session(&before).await;
    // Even an existing source cannot silently choose its preferred legacy tool.
    publish_single_capture(&coordinator, &hook(202, 198, 2)).await;
    assert_eq!(
        session(&selected(&coordinator).await).await,
        selected_session
    );
}

#[tokio::test]
async fn concurrent_cross_profile_preparations_cannot_publish_duplicate_native_sessions() {
    let coordinator = fixture().await;
    let pinned = selected(&coordinator).await;
    let first = hook(203, 204, 1);
    let mut second = native(
        205,
        1,
        SESSION,
        json!({
            "type":"function_call","call_id":"native-call","name":"read","arguments":"{}",
        }),
    );
    second.source_instance_id = SourceInstanceId::new([206; 16]);
    let mut envelope: serde_json::Value =
        serde_json::from_slice(second.observations[0].payload.bytes()).unwrap();
    envelope["product_version"] = "0.159.2".into();
    second.observations[0].payload = SanitizedPayloadV1::new(
        SanitizedMediaType::Json,
        &serde_json::to_vec(&envelope).unwrap(),
    )
    .unwrap();
    let profile = NormalizerProfileV1::codex_desktop_0_159_2();
    let staged = coordinator
        .stage_agent_capture(&pinned, &profile, &second, 123)
        .await
        .unwrap();
    let request = AgentDeltaRequest::new(
        jj_store_model::PrincipalId::new([72; 16]),
        jj_store_model::RequestId::new([207; 16]),
        jj_store_model::PublicationId::new([207; 16]),
        staged,
        b"parallel source".to_vec(),
    )
    .unwrap();
    publish_single_capture(&coordinator, &first).await;
    assert!(matches!(
        coordinator.publish_agent_delta(&request).await,
        Err(wal::Error::Stale)
    ));
    let current = selected(&coordinator).await;
    let before = session(&current).await;
    let delta = coordinator
        .stage_agent_capture(&current, &profile, &second, 124)
        .await
        .unwrap();
    publish_transcript_edits(&coordinator, 208, delta.edits).await;
    assert_eq!(
        session(&selected(&coordinator).await).await.session_id,
        before.session_id
    );
}
