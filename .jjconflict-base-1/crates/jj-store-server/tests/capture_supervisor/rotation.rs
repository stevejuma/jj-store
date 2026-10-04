use super::*;

#[tokio::test(flavor = "current_thread")]
async fn compact_replaced_source_releases_tracking_without_admitting_historical_starts() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    f.message("inactive-a", "unadmitted original A must remain untouched");
    let mut settings = config(&f);
    settings.max_sources = 1;
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| {
        s.tracked == 1 && s.observed_sources == 1 && s.resident_source_frames == 0
    })
    .await;
    assert!(f.sources.lookup_admitted(&f.qualified()).unwrap().is_none());
    let replacement = f.transcript.with_extension("replacement");
    fs::write(&replacement, format!("{}\n{}\n{}\n",
        json!({"type":"session_meta","payload":{"id":"00000000-0000-0000-0000-000000000002","cwd":f.workspace(B),"cli_version":"0.156.1","source":"cli","originator":"codex_cli_rs"}}),
        json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"historical"}}),
        json!({"type":"response_item","payload":{"type":"message","id":"history-b","role":"assistant","content":[{"type":"output_text","text":"replacement B eligible backfill"}]}}),
    )).unwrap();
    fs::rename(replacement, &f.transcript).unwrap();
    // The compact old owner must release its one tracking slot before a new
    // physical/header qualification can be armed. This changes no durable state.
    settle(&mut worker, |s| s.tracked == 0).await;
    assert_eq!(worker.status().resident_source_frames, 0);
    assert_eq!(worker.status().running_sources, 0);
    assert!(f.spool.pending().unwrap().is_empty());
    settle(&mut worker, |s| {
        s.tracked == 1
            && s.observed_sources == 1
            && s.awaiting_activity == 1
            && s.resident_source_frames == 0
    })
    .await;
    assert!(!worker.status().source_limit_reached);
    assert!(f.sources.lookup_admitted(&f.qualified()).unwrap().is_none());
    assert!(f.spool.pending().unwrap().is_empty());
    f.append(json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"fresh"}}));
    f.message("fresh-b", &format!("fresh replacement B body {SECRET}"));
    settle(&mut worker, |s| s.caught_up == 1).await;
    assert_eq!(worker.status().tracked, 1);
    assert!(f.spool.pending().unwrap().is_empty());
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    for repository in [A, B] {
        let read = f
            .host
            .endpoint(repository)
            .unwrap()
            .bind_local_trusted(
                f.authority.principal(),
                jj_store_service::auth::Permission::Read,
            )
            .unwrap()
            .into_session();
        let page = pager.begin(read.clone(), 10).await.unwrap();
        if repository == A {
            assert!(page.sessions.is_empty());
        } else {
            assert_eq!(page.sessions.len(), 1);
            let detail = pager
                .detail(read, page.sessions[0].session_id)
                .await
                .unwrap();
            let body = serde_json::to_string(&detail).unwrap();
            assert!(body.contains("replacement B eligible backfill"));
            assert!(body.contains("fresh replacement B body"));
            assert!(!body.contains("unadmitted original A"));
            assert!(!body.contains(SECRET));
        }
    }
    worker.shutdown().await.unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn replacement_cannot_evict_an_exact_unacknowledged_noncompact_frame() {
    let integration = serde_json::from_value(json!("codex")).unwrap();
    let f = Fixture::new(1).await;
    let mut settings = config(&f);
    settings.max_sources = 1;
    settings.max_pending_bytes = 1;
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| {
        s.tracked == 1 && s.observed_sources == 1 && s.resident_source_frames == 0
    })
    .await;
    f.append(json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"admitted"}}));
    f.message("unacknowledged", &format!("retain old raw frame {SECRET}"));
    settle(&mut worker, |s| {
        s.resident_source_frames == 1 && s.unavailable > 0
    })
    .await;
    let source = f.sources.lookup_admitted(&f.qualified()).unwrap().unwrap();
    let registry = fs::read(f.root.path().join("sources/sources.json")).unwrap();
    assert!(
        f.spool
            .latest_reader_progress(integration, source)
            .unwrap()
            .is_none()
    );
    let replacement = f.transcript.with_extension("replacement");
    fs::write(&replacement, format!("{}\n",json!({"type":"session_meta","payload":{"id":"00000000-0000-0000-0000-000000000002","cwd":f.workspace(B),"cli_version":"0.156.1","source":"cli","originator":"codex_cli_rs"}}))).unwrap();
    fs::rename(replacement, &f.transcript).unwrap();
    f.append(json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"replacement"}}));
    f.message("new-body", "replacement must not evict old frame");
    tokio::time::sleep(Duration::from_millis(1050)).await;
    settle(&mut worker, |s| s.source_limit_reached).await;
    worker.pause();
    quiesce(&mut worker).await;
    assert_eq!(worker.status().resident_source_frames, 1);
    worker.resume().unwrap();
    for _ in 0..16 {
        quantum(&mut worker).await;
        quiesce(&mut worker).await;
        assert_eq!(worker.status().tracked, 1);
        assert_eq!(worker.status().resident_source_frames, 1);
        assert_eq!(worker.status().caught_up, 0);
        assert!(worker.status().running_sources <= 1);
        assert!(f.spool.pending().unwrap().is_empty());
        assert!(
            f.spool
                .latest_reader_progress(integration, source)
                .unwrap()
                .is_none()
        );
        assert!(f.sources.lookup_admitted(&f.qualified()).unwrap().is_none());
        assert_eq!(
            fs::read(f.root.path().join("sources/sources.json")).unwrap(),
            registry
        );
    }
    worker.shutdown().await.unwrap();
    assert_eq!(worker.status().resident_source_frames, 0);
}

async fn read_agents(
    f: &Fixture,
    repository: jj_store_model::RepositoryId,
) -> (jj_store_service::collaboration::AgentReadSnapshot, String) {
    let read = f
        .host
        .endpoint(repository)
        .unwrap()
        .bind_local_trusted(
            f.authority.principal(),
            jj_store_service::auth::Permission::Read,
        )
        .unwrap()
        .into_session();
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    let page = pager.begin(read.clone(), 10).await.unwrap();
    let mut body = String::new();
    for session in page.sessions {
        body.push_str(
            &serde_json::to_string(
                &pager
                    .detail(read.clone(), session.session_id)
                    .await
                    .unwrap(),
            )
            .unwrap(),
        );
    }
    (page.snapshot, body)
}

#[tokio::test(flavor = "current_thread")]
async fn compact_retirement_preserves_exact_a_queue_while_fresh_replacement_b_delivers() {
    use jj_store_server::capture_source::{CaptureSourceStep, CodexCaptureSource};
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let mut settings = config(&f);
    settings.max_sources = 1;
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| {
        s.tracked == 1 && s.observed_sources == 1 && s.resident_source_frames == 0
    })
    .await;
    f.append(json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"old-start"}}));
    f.message("a-selected", "selected A before rotation");
    settle(&mut worker, |s| s.caught_up == 1).await;
    worker.pause();
    quiesce(&mut worker).await;
    f.message("a-pending", &format!("exact durable old A intent {SECRET}"));
    let mut independent = CodexCaptureSource::acquire_admitted(
        f.sources.clone(),
        f.qualified(),
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap()
    .into_pipeline();
    assert_eq!(
        independent.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    drop(independent);
    let queue = f.spool.pending().unwrap();
    assert_eq!(queue.len(), 1);
    let source = queue[0].batch.source_instance_id;
    let integration = queue[0].batch.integration;
    let cursor = f.spool.latest_reader_progress(integration, source).unwrap();
    let path = f
        .root
        .path()
        .join("spool/pending")
        .join(format!("{}.json", queue[0].batch.batch_id));
    let bytes = fs::read(&path).unwrap();
    let before_a = read_agents(&f, A).await;
    f.catalog(&[B]);
    let replacement = f.transcript.with_extension("replacement");
    fs::write(&replacement, format!("{}\n",json!({"type":"session_meta","payload":{"id":"00000000-0000-0000-0000-000000000002","cwd":f.workspace(B),"cli_version":"0.156.1","source":"cli","originator":"codex_cli_rs"}}))).unwrap();
    fs::rename(replacement, &f.transcript).unwrap();
    worker.resume().unwrap();
    settle(&mut worker, |s| s.tracked == 0).await;
    settle(&mut worker, |s| {
        s.tracked == 1
            && s.observed_sources == 1
            && s.awaiting_activity == 1
            && s.resident_source_frames == 0
    })
    .await;
    assert_eq!(f.spool.pending().unwrap(), queue);
    assert_eq!(fs::read(&path).unwrap(), bytes);
    assert_eq!(
        f.spool.latest_reader_progress(integration, source).unwrap(),
        cursor
    );
    f.append(json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"new-start"}}));
    f.message(
        "b-current",
        &format!("fresh B after compact retirement {SECRET}"),
    );
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            quantum(&mut worker).await;
            quiesce(&mut worker).await;
            let (_, body) = read_agents(&f, B).await;
            if body.contains("fresh B after compact retirement") {
                assert!(!body.contains("exact durable old A intent"));
                assert!(!body.contains(SECRET));
                break;
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(worker.status().tracked, 1);
    assert_eq!(worker.status().caught_up, 0);
    assert_eq!(f.spool.pending().unwrap(), queue);
    assert_eq!(fs::read(&path).unwrap(), bytes);
    assert_eq!(read_agents(&f, A).await, before_a);
    let current = f
        .spool
        .latest_reader_progress(integration, source)
        .unwrap()
        .unwrap();
    assert_eq!(current.repository, B);
    assert!(current.last_sequence > cursor.unwrap().last_sequence);
    worker.shutdown().await.unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn compact_header_inspection_error_retains_owner_until_exact_binding_recovers() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let header = fs::read(&f.transcript).unwrap();
    let mut settings = config(&f);
    settings.max_sources = 1;
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| {
        s.tracked == 1 && s.observed_sources == 1 && s.resident_source_frames == 0
    })
    .await;
    let registry = fs::read(f.root.path().join("sources/sources.json")).unwrap();
    fs::write(&f.transcript, b"unqualified header\n").unwrap();
    let before = worker.status().unavailable;
    settle(&mut worker, |s| s.unavailable > before).await;
    for _ in 0..8 {
        quantum(&mut worker).await;
        quiesce(&mut worker).await;
        assert_eq!(worker.status().tracked, 1);
        assert_eq!(worker.status().resident_source_frames, 0);
        assert_eq!(worker.status().awaiting_activity, 1);
        assert!(f.spool.pending().unwrap().is_empty());
        assert_eq!(
            fs::read(f.root.path().join("sources/sources.json")).unwrap(),
            registry
        );
    }
    fs::write(&f.transcript, header).unwrap();
    for _ in 0..16 {
        quantum(&mut worker).await;
        quiesce(&mut worker).await;
    }
    assert_eq!(worker.status().tracked, 1);
    assert!(f.sources.lookup_admitted(&f.qualified()).unwrap().is_none());
    f.append(
        json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"after-recovery"}}),
    );
    f.message(
        "recovered",
        &format!("fresh activity after header recovery {SECRET}"),
    );
    settle(&mut worker, |s| s.caught_up == 1).await;
    let (_, body) = read_agents(&f, A).await;
    assert!(body.contains("fresh activity after header recovery"));
    assert!(!body.contains(SECRET));
    worker.shutdown().await.unwrap();
}
