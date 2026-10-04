use super::*;

#[tokio::test]
async fn codex_transcript_timing_recovers_existing_sanitized_native_timestamps() {
    for profile in [
        NormalizerProfileV1::codex_0_156_1(),
        NormalizerProfileV1::codex_desktop_0_159_2(),
    ] {
        let fixture = fixture().await;
        let source = transcript_batch(&profile);
        assert!(
            source
                .observations
                .iter()
                .all(|v| v.occurred_at_millis.is_none())
        );
        let request = fixture
            .session
            .prepare_agent_capture(
                m::RequestId::new([81; 16]),
                m::PublicationId::new([82; 16]),
                &profile,
                &source,
                b"accepted timestamped transcript".to_vec(),
            )
            .await
            .unwrap();
        assert!(matches!(
            fixture.session.publish_agent_delta(&request).await.unwrap(),
            PublishOutcome::Committed(_)
        ));
        let pager =
            AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
        let page = pager.begin(fixture.session.clone(), 10).await.unwrap();
        assert_eq!(page.sessions.len(), 1);
        let summary = &page.sessions[0];
        assert_eq!(summary.first_activity_at_millis, Some(1_704_164_645_123));
        assert_eq!(summary.last_activity_at_millis, Some(1_704_164_770_456));
        let detail = pager
            .detail(fixture.session.clone(), summary.session_id)
            .await
            .unwrap();
        assert_eq!(&detail.session, summary);
        // Timing is a read projection; the selected repository state stays pinned.
        assert_eq!(detail.snapshot, page.snapshot);
        assert_eq!(
            pager.begin(fixture.session, 10).await.unwrap().snapshot,
            page.snapshot
        );
    }
}

fn transcript_batch(profile: &NormalizerProfileV1) -> SanitizedObservationBatchV1 {
    let records = [
        serde_json::json!({"timestamp":"2024-01-02T00:00:00Z", "type":"session_meta", "payload":{}}),
        serde_json::json!({"timestamp":"2024-01-02T03:04:05.123Z", "type":"event_msg", "payload":{"type":"task_started"}}),
        serde_json::json!({"timestamp":"2024-01-02T04:06:10.456+01:00", "type":"event_msg", "payload":{"type":"task_complete"}}),
        serde_json::json!({"timestamp":"2024-01-02T03:05:00Z", "type":"response_item", "payload":{"type":"message", "role":"user", "content":[{"type":"input_text", "text":"Fix the parser"}]}}),
        serde_json::json!({"timestamp":"2099-01-01T00:00:00Z", "type":"unsupported", "payload":{}}),
    ];
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([71; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([72; 16]),
        sanitizer_version: "mongodb-kingfisher@2.7.0/builtin-medium/transcript-paths/v1".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: records.into_iter().enumerate().map(|(index, record)| SanitizedObservationV1 {
            observation_id: ObservationId::new([index as u8 + 73; 16]),
            source_sequence: index as u64 + 1,
            occurred_at_millis: None,
            source_kind: SourceKind::Transcript,
            source_trust: SourceTrust::LocalTranscript,
            payload: SanitizedPayloadV1::new(SanitizedMediaType::Json, &serde_json::to_vec(&serde_json::json!({
                "capture_format":"codex-jsonl/v1", "product_version":profile.product_version,
                "session_id":"native-transcript-timing", "record":record,
            })).unwrap()).unwrap(),
        }).collect(),
    }
}

#[tokio::test]
async fn message_timing_matches_selected_evidence_in_activity_and_transcript_pages() {
    for profile in [
        NormalizerProfileV1::codex_0_156_1(),
        NormalizerProfileV1::codex_desktop_0_159_2(),
    ] {
        let fixture = fixture().await;
        let mut source = transcript_batch(&profile);
        let template = source.observations[3].clone();
        for (i, timestamp) in [
            Some("2024-01-02T04:05:02.456+01:00"),
            Some("bad date"),
            None,
        ]
        .into_iter()
        .enumerate()
        {
            let mut observation = template.clone();
            observation.observation_id = ObservationId::new([90 + i as u8; 16]);
            observation.source_sequence = 6 + i as u64;
            let mut record = serde_json::json!({"type":"response_item", "payload":{"type":"message", "role":"assistant", "content":[{"type":"output_text", "text":format!("Response {i}")}]}});
            if let Some(timestamp) = timestamp {
                record["timestamp"] = timestamp.into();
            }
            observation.payload = SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                &serde_json::to_vec(&serde_json::json!({
                    "capture_format":"codex-jsonl/v1", "product_version":profile.product_version,
                    "session_id":"native-transcript-timing", "record":record,
                }))
                .unwrap(),
            )
            .unwrap();
            source.observations.push(observation);
        }
        let request = fixture
            .session
            .prepare_agent_capture(
                m::RequestId::new([85; 16]),
                m::PublicationId::new([86; 16]),
                &profile,
                &source,
                b"message timing".to_vec(),
            )
            .await
            .unwrap();
        fixture.session.publish_agent_delta(&request).await.unwrap();
        let pager =
            AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
        let page = pager.begin(fixture.session.clone(), 10).await.unwrap();
        let detail = pager
            .detail(fixture.session.clone(), page.sessions[0].session_id)
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
            vec![Some(1_704_164_700_000), Some(1_704_164_702_456), None, None]
        );
        let stream = &detail.transcripts[0];
        let query = AgentTranscriptRequest {
            session_id: detail.session.session_id,
            transcript_id: stream.transcript_id,
            source_instance_id: stream.source_instance_id,
            source_generation: stream.source_generation,
            limit: 2,
            max_bytes: 1024,
        };
        let first = pager
            .begin_transcript(fixture.session.clone(), query)
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
        let last = pager
            .resume_transcript(
                fixture.session.clone(),
                first.next_cursor.as_ref().unwrap(),
                query,
            )
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
        assert_eq!(first.snapshot, last.snapshot);
        assert_eq!(detail.snapshot, page.snapshot);
        assert_eq!(
            pager.begin(fixture.session, 10).await.unwrap().snapshot,
            page.snapshot
        );
    }
}
