use super::*;

#[tokio::test]
async fn tool_duration_uses_exact_call_and_result_evidence_without_inventing_status() {
    for profile in [
        NormalizerProfileV1::codex_0_156_1(),
        NormalizerProfileV1::codex_desktop_0_159_2(),
    ] {
        let fixture = fixture().await;
        let mut source = transcript_batch(&profile);
        source.observations.clear();
        let pairs = [
            (
                "complete",
                Some("2024-01-02T03:05:00Z"),
                Some("2024-01-02T03:05:01.250Z"),
            ),
            ("missing", Some("2024-01-02T03:05:00Z"), None),
            ("malformed", Some("bad date"), Some("2024-01-02T03:05:01Z")),
            (
                "backwards",
                Some("2024-01-02T03:05:02Z"),
                Some("2024-01-02T03:05:01Z"),
            ),
            (
                "zero",
                Some("2024-01-02T03:05:00Z"),
                Some("2024-01-02T03:05:00Z"),
            ),
        ];
        for (call, start, end) in pairs {
            for (timestamp, payload) in [
                (
                    start,
                    serde_json::json!({"type":"function_call", "call_id":call, "name":call,"arguments":"{}"}),
                ),
                (
                    end,
                    serde_json::json!({"type":"function_call_output", "call_id":call, "output":"recorded output"}),
                ),
            ] {
                let index = source.observations.len();
                let mut record = serde_json::json!({"type":"response_item", "payload":payload});
                if let Some(timestamp) = timestamp {
                    record["timestamp"] = timestamp.into();
                }
                source.observations.push(SanitizedObservationV1 {
                    observation_id: ObservationId::new([100 + index as u8;16]),
                    source_sequence: index as u64 + 1, occurred_at_millis:None,
                    source_kind:SourceKind::Transcript, source_trust:SourceTrust::LocalTranscript,
                    payload:SanitizedPayloadV1::new(SanitizedMediaType::Json, &serde_json::to_vec(&serde_json::json!({"capture_format":"codex-jsonl/v1", "product_version":profile.product_version, "session_id":"native-tool-timing", "record":record})).unwrap()).unwrap(),
                });
            }
        }
        let request = fixture
            .session
            .prepare_agent_capture(
                m::RequestId::new([111; 16]),
                m::PublicationId::new([112; 16]),
                &profile,
                &source,
                b"tool duration".to_vec(),
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
        let activity = serde_json::to_value(&detail.activity).unwrap();
        let tools = activity
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| v["type"] == "tool")
            .map(|v| &v["tool"])
            .collect::<Vec<_>>();
        assert_eq!(tools.len(), 5);
        let tool = |name: &str| *tools.iter().find(|v| v["native_name"] == name).unwrap();
        assert_eq!(tool("complete")["duration_millis"].as_u64(), Some(1250));
        assert_eq!(tool("complete")["state"], "unknown");
        for name in ["missing", "malformed", "backwards"] {
            assert!(tool(name)["duration_millis"].is_null());
        }
        assert_eq!(tool("zero")["duration_millis"].as_u64(), Some(0));
        assert_eq!(detail.snapshot, page.snapshot);
        assert_eq!(
            pager.begin(fixture.session, 10).await.unwrap().snapshot,
            page.snapshot
        );
    }
}

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

#[tokio::test]
async fn opencode_snapshot_timing_and_tool_span_use_native_evidence() {
    let fixture = fixture().await;
    let profile = NormalizerProfileV1::opencode_1_18_30();
    let mut source = transcript_batch(&profile);
    source.integration = IntegrationId::OpenCode;
    source.observations.truncate(1);
    let observation = &mut source.observations[0];
    observation.source_kind = SourceKind::NativeApi;
    observation.source_trust = SourceTrust::ApprovedHarnessApi;
    observation.payload = SanitizedPayloadV1::new(SanitizedMediaType::Json, &serde_json::to_vec(&serde_json::json!({
        "capture_format":"opencode-snapshot/v1", "product_version":"1.18.30", "session_id":"ses_timing",
        "record":{"type":"message", "info":{"id":"msg_timing","sessionID":"ses_timing","role":"assistant","time":{"created":1000,"completed":3000}},
            "parts":[{"id":"prt_timing","messageID":"msg_timing","sessionID":"ses_timing","type":"tool","callID":"call_timing","tool":"read",
                "state":{"status":"completed","input":{},"output":"done","time":{"start":1100,"end":1350}}}]}
    })).unwrap()).unwrap();
    let request = fixture
        .session
        .prepare_agent_capture(
            m::RequestId::new([87; 16]),
            m::PublicationId::new([88; 16]),
            &profile,
            &source,
            b"native OpenCode timing".to_vec(),
        )
        .await
        .unwrap();
    fixture.session.publish_agent_delta(&request).await.unwrap();
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    let page = pager.begin(fixture.session.clone(), 10).await.unwrap();
    assert_eq!(page.sessions.len(), 1);
    assert_eq!(page.sessions[0].first_activity_at_millis, Some(1000));
    assert_eq!(page.sessions[0].last_activity_at_millis, Some(3000));
    let detail = pager
        .detail(fixture.session.clone(), page.sessions[0].session_id)
        .await
        .unwrap();
    let activity = serde_json::to_value(&detail.activity).unwrap();
    let message = activity
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["type"] == "message")
        .unwrap();
    assert_eq!(message["occurred_at_millis"].as_i64(), Some(1000));
    let tool = activity
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["type"] == "tool")
        .unwrap();
    assert_eq!(tool["tool"]["duration_millis"].as_u64(), Some(250));
    assert_eq!(detail.snapshot, page.snapshot);
    assert_eq!(
        pager.begin(fixture.session, 10).await.unwrap().snapshot,
        page.snapshot
    );
}
