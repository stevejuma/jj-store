use super::*;
use jj_store_server::capture_source::{CaptureSourceStep, CodexCaptureSource};

#[tokio::test]
async fn full_captured_queue_drains_ordered_native_heads_without_capacity_increase() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    f.message("first", &format!("first cumulative evidence {SECRET}"));
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
    f.message("second", "second cumulative evidence");
    assert_eq!(
        source.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let mut queue = f.spool.pending().unwrap();
    queue.sort_by_key(|entry| entry.batch.observations[0].source_sequence);
    assert_eq!(queue.len(), 2);
    let progress = f
        .spool
        .latest_reader_progress(source.partition().integration, source.partition().source)
        .unwrap();
    let budget: u64 = fs::read_dir(f.root.path().join("spool/pending"))
        .unwrap()
        .map(|entry| entry.unwrap().metadata().unwrap().len())
        .sum();
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), budget).unwrap();
    f.message("third", "third evidence withheld under pressure");
    assert!(source.ingest_step(&mut f.context()).await.is_err());
    assert_eq!(f.spool.pending().unwrap().len(), 2);
    assert_eq!(
        f.spool
            .latest_reader_progress(source.partition().integration, source.partition().source)
            .unwrap(),
        progress
    );
    // A later same-partition request is not prepared against an earlier root.
    assert!(
        source
            .publish_step(&queue[1], &mut f.context())
            .await
            .is_err()
    );
    assert!(f.spool.pending().unwrap().iter().all(|entry| matches!(
        entry.state,
        jj_store_capture::agent_capture::PendingCaptureStateV1::Captured
    )));
    assert!(
        matches!(source.publish_step(&queue[0], &mut f.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed==1)
    );
    assert_eq!(
        source.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    assert!(
        matches!(source.publish_step(&queue[1], &mut f.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed==1)
    );
    let third = f.spool.pending().unwrap().remove(0);
    assert!(
        matches!(source.publish_step(&third, &mut f.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed==1)
    );
    assert!(f.spool.pending().unwrap().is_empty());
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
    assert_eq!(page.sessions.len(), 1);
    let detail = pager
        .detail(session, page.sessions[0].session_id)
        .await
        .unwrap();
    let body = serde_json::to_string(&detail).unwrap();
    for evidence in [
        "first cumulative evidence",
        "second cumulative evidence",
        "third evidence withheld under pressure",
    ] {
        assert!(body.contains(evidence));
    }
    assert!(!body.contains(SECRET));
    assert!(!body.contains(f.root.path().to_str().unwrap()));
}
