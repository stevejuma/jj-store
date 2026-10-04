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
