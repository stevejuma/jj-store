use super::*;

#[tokio::test]
async fn codex_transcript_timing_remains_companion_pinned() {
    let (_root, repo) = fixture();
    let mut source = batch();
    source.observations.truncate(2);
    for (i, observation) in source.observations.iter_mut().enumerate() {
        observation.occurred_at_millis = None;
        observation.source_kind = SourceKind::Transcript;
        observation.source_trust = SourceTrust::LocalTranscript;
        observation.payload = SanitizedPayloadV1::new(SanitizedMediaType::Json, &serde_json::to_vec(&serde_json::json!({
            "capture_format":"codex-jsonl/v1", "product_version":"0.156.1", "session_id":"native-local-timing",
            "record":{"timestamp":if i==0 { "2024-01-02T03:04:05.123Z" } else { "2024-01-02T04:06:10.456+01:00" },
                "type":"event_msg", "payload":{"type":if i==0 { "task_started" } else { "task_complete" }}}
        })).unwrap()).unwrap();
    }
    repo.apply_agent_command(&request(91, source.clone()))
        .await
        .unwrap();
    let pin = repo.metadata_store().read().unwrap().unwrap().commit();
    let page = repo.browser_agent_sessions(pin, 20, None).await.unwrap();
    assert_eq!(page.sessions.len(), 1);
    let row = &page.sessions[0];
    assert_eq!(row.first_activity_at_millis, Some(1_704_164_645_123));
    assert_eq!(row.last_activity_at_millis, Some(1_704_164_770_456));
    assert_eq!(
        repo.browser_agent_session(pin, row.session_id)
            .await
            .unwrap()
            .session,
        *row
    );

    source.batch_id = CaptureBatchId::new([92; 16]);
    source.observations.truncate(1);
    source.observations[0].observation_id = ObservationId::new([93; 16]);
    source.observations[0].source_sequence = 3;
    source.observations[0].payload = SanitizedPayloadV1::new(SanitizedMediaType::Json, br#"{"capture_format":"codex-jsonl/v1","product_version":"0.156.1","session_id":"native-local-timing","record":{"timestamp":"2024-01-02T03:10:00Z","type":"event_msg","payload":{"type":"task_complete"}}}"#).unwrap();
    repo.apply_agent_command(&request(92, source))
        .await
        .unwrap();
    let current = repo.metadata_store().read().unwrap().unwrap().commit();
    assert_eq!(
        repo.browser_agent_session(pin, row.session_id)
            .await
            .unwrap()
            .session,
        *row
    );
    assert_eq!(
        repo.browser_agent_session(current, row.session_id)
            .await
            .unwrap()
            .session
            .last_activity_at_millis,
        Some(1_704_165_000_000)
    );
}

#[tokio::test]
async fn message_timing_remains_companion_pinned_across_later_capture() {
    let (_root, repo) = fixture();
    let mut source = batch();
    let template = source.observations[0].clone();
    source.observations = (0..4).map(|i| {
        let mut observation = template.clone();
        observation.observation_id = ObservationId::new([100+i;16]);
        observation.source_sequence = u64::from(i)+1;
        observation.occurred_at_millis = if i==2 { Some(1_704_164_704_789) } else { None };
        observation.source_kind = SourceKind::Transcript;
        observation.source_trust = SourceTrust::LocalTranscript;
        let timestamp = match i {0=>Some("2024-01-02T03:05:00Z"), 1=>Some("2024-01-02T04:05:02.456+01:00"), 2=>Some("bad date"), _=>None};
        let mut record = serde_json::json!({"type":"response_item", "payload":{"type":"message", "role":if i==0 {"user"} else {"assistant"}, "content":[{"type":"output_text", "text":format!("Local message {i}")}]}});
        if let Some(timestamp) = timestamp {record["timestamp"] = timestamp.into();}
        observation.payload = SanitizedPayloadV1::new(SanitizedMediaType::Json, &serde_json::to_vec(&serde_json::json!({"capture_format":"codex-jsonl/v1", "product_version":"0.156.1", "session_id":"local-message-timing", "record":record})).unwrap()).unwrap();
        observation
    }).collect();
    repo.apply_agent_command(&request(94, source.clone()))
        .await
        .unwrap();
    let pin = repo.metadata_store().read().unwrap().unwrap().commit();
    let page = repo.browser_agent_sessions(pin, 20, None).await.unwrap();
    let detail = repo
        .browser_agent_session(pin, page.sessions[0].session_id)
        .await
        .unwrap();
    let messages = serde_json::to_value(&detail.activity)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| v["type"] == "message")
        .map(|v| v["occurred_at_millis"].as_i64())
        .collect::<Vec<_>>();
    assert_eq!(
        messages,
        vec![
            Some(1_704_164_700_000),
            Some(1_704_164_702_456),
            Some(1_704_164_704_789),
            None
        ]
    );
    let stream = &detail.transcripts[0];
    let query = LocalBrowserTranscriptRequest {
        session_id: detail.session.session_id,
        transcript_id: stream.transcript_id,
        source_instance_id: stream.source_instance_id,
        source_generation: stream.source_generation,
        limit: 2,
        max_bytes: 1024,
    };
    let first = repo
        .browser_agent_transcript(pin, query.clone(), None)
        .await
        .unwrap();
    let times = serde_json::to_value(&first.chunks)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["occurred_at_millis"].as_i64())
        .collect::<Vec<_>>();
    assert_eq!(times, messages[..2]);
    source.batch_id = CaptureBatchId::new([95; 16]);
    source.observations.truncate(1);
    source.observations[0].observation_id = ObservationId::new([96; 16]);
    source.observations[0].source_sequence = 5;
    source.observations[0].occurred_at_millis = Some(1_704_164_800_000);
    repo.apply_agent_command(&request(95, source))
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(
            repo.browser_agent_session(pin, detail.session.session_id)
                .await
                .unwrap()
        )
        .unwrap(),
        serde_json::to_value(&detail).unwrap()
    );
    let last = repo
        .browser_agent_transcript(pin, query, first.next_after_sequence)
        .await
        .unwrap();
    let times = serde_json::to_value(&last.chunks)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["occurred_at_millis"].as_i64())
        .collect::<Vec<_>>();
    assert_eq!(times, messages[2..]);
    assert_eq!(last.snapshot, first.snapshot);
    let current = repo.metadata_store().read().unwrap().unwrap().commit();
    let later = repo
        .browser_agent_session(current, detail.session.session_id)
        .await
        .unwrap();
    assert_eq!(
        later
            .activity
            .iter()
            .filter(|v| matches!(
                v,
                jj_store_service::collaboration::AgentActivity::Message { .. }
            ))
            .count(),
        5
    );
}
