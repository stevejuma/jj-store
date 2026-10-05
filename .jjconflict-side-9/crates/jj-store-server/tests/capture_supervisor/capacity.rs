use super::*;
use jj_store_server::capture_source::{CaptureSourceStep, CodexCaptureSource};

async fn native_agents(
    f: &Fixture,
) -> (jj_store_service::collaboration::AgentReadSnapshot, String) {
    let session = f
        .host
        .endpoint(A)
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
    let page = pager.begin(session.clone(), 10).await.unwrap();
    let mut body = String::new();
    for agent in page.sessions {
        body.push_str(
            &serde_json::to_string(
                &pager
                    .detail(session.clone(), agent.session_id)
                    .await
                    .unwrap(),
            )
            .unwrap(),
        );
    }
    (page.snapshot, body)
}

#[tokio::test(flavor = "current_thread")]
async fn ingress_capacity_status_preserves_cursor_and_clears_after_explicit_relief() {
    let mut f = Fixture::new(1).await;
    let mut settings = config(&f);
    settings.max_pending_bytes = 1;
    let mut worker = supervisor(&f, settings.clone());
    settle(&mut worker, |s| {
        s.observed_sources == 1 && s.resident_source_frames == 0
    })
    .await;
    f.append(json!({"type":"event_msg","payload":{"type":"task_started"}}));
    f.message("denied", &format!("capacity evidence retained {SECRET}"));
    settle(&mut worker, |s| s.ingress_capacity_refusals == 1).await;
    worker.pause();
    quiesce(&mut worker).await;
    let status = worker.status();
    assert_eq!(status.pending_capacity_bytes, 1);
    let required = status.ingress_required_capacity_bytes.unwrap();
    assert!(required > 1 && required <= 4 * 1024 * 1024);
    assert_eq!(status.delivery_required_capacity_bytes, None);
    assert_eq!(status.resident_source_frames, 1);
    let debug = format!("{status:?}");
    assert!(!debug.contains(SECRET));
    assert!(!debug.contains(f.root.path().to_str().unwrap()));
    assert!(f.spool.pending().unwrap().is_empty());
    let source = f.sources.lookup_admitted(&f.qualified()).unwrap().unwrap();
    let integration = serde_json::from_value(json!("codex")).unwrap();
    assert!(
        f.spool
            .latest_reader_progress(integration, source)
            .unwrap()
            .is_none()
    );
    let registry = fs::read(f.root.path().join("sources/sources.json")).unwrap();
    worker.resume().unwrap();
    settle(&mut worker, |s| s.ingress_capacity_refusals == 1).await;
    assert_eq!(
        worker.status().ingress_required_capacity_bytes,
        Some(required)
    );
    assert_eq!(
        fs::read(f.root.path().join("sources/sources.json")).unwrap(),
        registry
    );
    worker.shutdown().await.unwrap();
    assert_eq!(worker.status().ingress_capacity_refusals, 0);
    assert_eq!(worker.status().ingress_required_capacity_bytes, None);
    drop(worker);
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), required).unwrap();
    settings.max_pending_bytes = required;
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| s.caught_up == 1).await;
    assert_eq!(worker.status().pending_capacity_bytes, required);
    assert_eq!(worker.status().ingress_capacity_refusals, 0);
    assert_eq!(worker.status().ingress_required_capacity_bytes, None);
    assert_eq!(worker.status().delivery_required_capacity_bytes, None);
    assert!(f.spool.pending().unwrap().is_empty());
    assert!(
        f.spool
            .latest_reader_progress(integration, source)
            .unwrap()
            .is_some()
    );
    let (_, body) = native_agents(&f).await;
    assert!(body.contains("capacity evidence retained"));
    assert!(!body.contains(SECRET));
    worker.shutdown().await.unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn compact_preparation_status_reports_exact_relief_without_rewriting_queue() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    f.message("compact", &format!("compact preparation capacity {SECRET}"));
    let active = f.start();
    let mut pipeline = CodexCaptureSource::acquire_active(
        f.sources.clone(),
        active,
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .into_pipeline();
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let partition = pipeline.partition();
    drop(pipeline);
    let entry = f.spool.pending().unwrap().remove(0);
    let path = f
        .root
        .path()
        .join("spool/pending")
        .join(format!("{}.json", entry.batch.batch_id));
    let bytes = serde_json::to_vec(&entry).unwrap();
    fs::write(&path, &bytes).unwrap();
    let budget = bytes.len() as u64;
    let progress = f
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap();
    let before = native_agents(&f).await;
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), budget).unwrap();
    let mut settings = config(&f);
    settings.max_pending_bytes = budget;
    let mut worker = supervisor(&f, settings.clone());
    settle(&mut worker, |s| {
        s.delivery_required_capacity_bytes.is_some()
    })
    .await;
    worker.pause();
    quiesce(&mut worker).await;
    let status = worker.status();
    let required = status.delivery_required_capacity_bytes.unwrap();
    assert!(required > budget && required <= 4 * 1024 * 1024);
    assert_eq!(status.pending_capacity_bytes, budget);
    assert_eq!(status.ingress_capacity_refusals, 0);
    assert_eq!(status.ingress_required_capacity_bytes, None);
    assert_eq!(status.caught_up, 0);
    assert_eq!(fs::read(&path).unwrap(), bytes);
    assert_eq!(f.spool.pending().unwrap(), vec![entry.clone()]);
    assert_eq!(
        f.spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap(),
        progress
    );
    assert_eq!(native_agents(&f).await, before);
    worker.resume().unwrap();
    settle(&mut worker, |s| {
        s.delivery_required_capacity_bytes.is_some()
    })
    .await;
    worker.shutdown().await.unwrap();
    assert_eq!(worker.status().delivery_required_capacity_bytes, None);
    drop(worker);
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), budget).unwrap();
    let mut worker = supervisor(&f, settings.clone());
    settle(&mut worker, |s| {
        s.delivery_required_capacity_bytes == Some(required)
    })
    .await;
    assert_eq!(fs::read(&path).unwrap(), bytes);
    assert_eq!(native_agents(&f).await, before);
    worker.shutdown().await.unwrap();
    drop(worker);
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), required).unwrap();
    settings.max_pending_bytes = required;
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| s.caught_up == 1).await;
    assert_eq!(worker.status().delivery_required_capacity_bytes, None);
    assert_eq!(worker.status().ingress_capacity_refusals, 0);
    assert!(f.spool.pending().unwrap().is_empty());
    let (_, body) = native_agents(&f).await;
    assert!(body.contains("compact preparation capacity"));
    assert!(!body.contains(SECRET));
    worker.shutdown().await.unwrap();
}
