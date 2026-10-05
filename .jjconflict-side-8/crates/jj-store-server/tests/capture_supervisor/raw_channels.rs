use super::*;
use jj_store_server::capture_source::{CaptureSourceStep, CodexCaptureSource};

#[tokio::test(flavor = "current_thread")]
async fn raw_channel_reservation_survives_pause_and_resumes_after_queue_relief() {
    let mut f = Fixture::new(8 * 1024 * 1024).await;
    f.message("a-first", "first accepted A evidence");
    let active = f.start();
    let mut source = CodexCaptureSource::acquire_active(
        f.sources.clone(),
        active,
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .into_pipeline();
    assert_eq!(
        source.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    f.message("a-second", "second accepted A evidence");
    assert_eq!(
        source.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let partition = source.partition();
    drop(source);
    let retained = f.spool.pending().unwrap();
    assert_eq!(retained.len(), 2);
    let original = f
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap()
        .unwrap();
    let capacity: u64 = fs::read_dir(f.root.path().join("spool/pending"))
        .unwrap()
        .map(|e| e.unwrap().metadata().unwrap().len())
        .sum();
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), capacity).unwrap();
    f.append(json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}}));
    f.message("raw-b", &format!("retained exact B raw evidence {SECRET}"));
    f.catalog(&[B]);
    let mut settings = config(&f);
    settings.max_pending_bytes = capacity;
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| {
        s.ingress_capacity_refusals == 1 && s.resident_source_frames == 1
    })
    .await;
    worker.pause();
    quiesce(&mut worker).await;
    assert!(
        f.sources.acquire_admitted(f.qualified()).is_err(),
        "automatic parked raw allocation must exclude another reader of its channel"
    );
    assert_eq!(f.spool.pending().unwrap(), retained);
    assert_eq!(
        f.spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap(),
        Some(original.clone())
    );
    f.catalog(&[A, B]);
    worker.resume().unwrap();
    settle(&mut worker, |s| s.caught_up == 1).await;
    let accepted = f
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap()
        .unwrap();
    assert_eq!(accepted.cursor.generation, original.cursor.generation);
    assert_eq!(accepted.last_sequence, original.last_sequence + 2);
    assert_eq!(f.spool.pending_capacity(), capacity);
    assert!(f.spool.pending().unwrap().is_empty());
    drop(f.sources.acquire_admitted(f.qualified()).unwrap().unwrap());
    let read = f
        .host
        .endpoint(B)
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
    assert_eq!(page.sessions.len(), 1);
    let body = serde_json::to_string(
        &pager
            .detail(read, page.sessions[0].session_id)
            .await
            .unwrap(),
    )
    .unwrap();
    assert!(body.contains("retained exact B raw evidence"));
    assert!(!body.contains("accepted A evidence"));
    assert!(!body.contains(SECRET));
    worker.shutdown().await.unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn shutdown_releases_raw_channel_without_acknowledging_tail_and_restart_recovers() {
    let mut f = Fixture::new(1).await;
    f.message("pending", &format!("unacknowledged raw tail {SECRET}"));
    let active = f.start();
    // Retained native eligibility permits automatic backfill after this owner exits.
    let source = f.sources.acquire_active(active).unwrap();
    let id = source.source_instance_id();
    drop(source);
    let mut settings = config(&f);
    settings.max_pending_bytes = 1;
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| {
        s.ingress_capacity_refusals == 1 && s.resident_source_frames == 1
    })
    .await;
    worker.pause();
    quiesce(&mut worker).await;
    assert!(f.sources.acquire_admitted(f.qualified()).is_err());
    assert!(f.spool.pending().unwrap().is_empty());
    assert!(
        f.spool
            .latest_reader_progress(serde_json::from_value(json!("codex")).unwrap(), id)
            .unwrap()
            .is_none()
    );
    let registry = fs::read(f.root.path().join("sources/sources.json")).unwrap();
    worker.shutdown().await.unwrap();
    assert_eq!(worker.status().resident_source_frames, 0);
    assert_eq!(worker.status().running_sources, 0);
    assert_eq!(
        fs::read(f.root.path().join("sources/sources.json")).unwrap(),
        registry
    );
    assert_eq!(
        f.sources
            .acquire_admitted(f.qualified())
            .unwrap()
            .unwrap()
            .source_instance_id(),
        id
    );
    assert!(f.spool.pending().unwrap().is_empty());
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    let mut worker = supervisor(&f, config(&f));
    settle(&mut worker, |s| s.caught_up == 1).await;
    let accepted = f
        .spool
        .latest_reader_progress(serde_json::from_value(json!("codex")).unwrap(), id)
        .unwrap()
        .unwrap();
    assert_eq!(accepted.cursor.generation, 1);
    assert_eq!(accepted.last_sequence, 3); // header, message and fresh activity
    assert!(f.spool.pending().unwrap().is_empty());
    assert_eq!(f.sources.lookup_admitted(&f.qualified()).unwrap(), Some(id));
    worker.shutdown().await.unwrap();
}
