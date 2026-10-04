use super::*;

#[tokio::test(flavor = "current_thread")]
async fn runtime_handoff_preserves_settled_pause_until_explicit_resume() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let sources = std::fs::read(f.root.path().join("sources/sources.json")).unwrap();
    let mut prepared = supervisor(&f);
    prepared.pause();
    let mut r = RunningCodexCapture::start(prepared).unwrap();
    let control = r.controls();
    f.message(
        "paused-start",
        &format!("body present during paused startup {SECRET}"),
    );
    drop(f.start()); // This start precedes the worker's first arming boundary.
    tokio::time::sleep(Duration::from_millis(80)).await;
    assert_eq!(
        control.snapshot().unwrap().phase,
        CaptureWorkerPhase::Paused
    );
    assert_eq!(control.snapshot().unwrap().status.tracked, 0);
    assert_eq!(control.snapshot().unwrap().status.observed_sources, 0);
    assert_eq!(
        std::fs::read(f.root.path().join("sources/sources.json")).unwrap(),
        sources
    );
    assert!(f.spool.pending().unwrap().is_empty());
    assert!(f.sources.lookup_admitted(&f.qualified()).unwrap().is_none());
    control.resume().unwrap();
    wait(&r, |s| {
        s.status.observed_sources == 1 && s.status.resident_source_frames == 0
    })
    .await;
    assert_eq!(control.snapshot().unwrap().status.awaiting_activity, 1);
    assert_eq!(control.snapshot().unwrap().status.caught_up, 0);
    assert!(f.sources.lookup_admitted(&f.qualified()).unwrap().is_none());
    f.message("fresh-start", "body after resumed arming");
    drop(f.start());
    wait(&r, |s| s.status.caught_up == 1).await;
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    let native = f
        .host
        .endpoint(A)
        .unwrap()
        .bind_local_trusted(
            f.authority.principal(),
            jj_store_service::auth::Permission::Read,
        )
        .unwrap()
        .into_session();
    let page = pager.begin(native.clone(), 10).await.unwrap();
    assert_eq!(page.sessions.len(), 1);
    let detail = pager
        .detail(native, page.sessions[0].session_id)
        .await
        .unwrap();
    let body = serde_json::to_string(&detail).unwrap();
    assert!(
        body.contains("body present during paused startup")
            && body.contains("body after resumed arming")
            && !body.contains(SECRET)
    );
    r.shutdown().await.unwrap();
    assert!(control.resume().is_err());
}

#[tokio::test(flavor = "current_thread")]
async fn shared_control_handles_follow_pause_resume_and_terminal_owner() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let mut r = runtime(&f);
    let control = r.controls();
    let second = control.clone();
    wait(&r, |s| {
        s.status.observed_sources == 1 && s.status.resident_source_frames == 0
    })
    .await;
    control.pause().unwrap();
    wait(&r, |s| s.phase == CaptureWorkerPhase::Paused).await;
    assert_eq!(second.snapshot().unwrap().phase, CaptureWorkerPhase::Paused);
    f.message("controlled", &format!("shared controlled capture {SECRET}"));
    drop(f.start());
    tokio::time::sleep(Duration::from_millis(80)).await;
    assert!(f.spool.pending().unwrap().is_empty());
    assert!(f.sources.lookup_admitted(&f.qualified()).unwrap().is_none());
    // Dropping a management clone cannot stop the sole drain owner.
    drop(second);
    control.resume().unwrap();
    wait(&r, |s| s.status.caught_up == 1).await;
    let snapshot = control.snapshot().unwrap();
    assert_eq!(snapshot.status.caught_up, 1);
    assert_eq!(snapshot.status.pending_batches, 0);
    assert!(f.sources.lookup_admitted(&f.qualified()).unwrap().is_some());
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    let native = f
        .host
        .endpoint(A)
        .unwrap()
        .bind_local_trusted(
            f.authority.principal(),
            jj_store_service::auth::Permission::Read,
        )
        .unwrap()
        .into_session();
    let page = pager.begin(native.clone(), 10).await.unwrap();
    assert_eq!(page.sessions.len(), 1);
    let detail = pager
        .detail(native, page.sessions[0].session_id)
        .await
        .unwrap();
    let body = serde_json::to_string(&detail).unwrap();
    assert!(body.contains("shared controlled capture") && !body.contains(SECRET));
    r.shutdown().await.unwrap();
    assert!(control.pause().is_err() && control.resume().is_err());
    assert_eq!(
        control.snapshot().unwrap().phase,
        CaptureWorkerPhase::Stopped
    );
    drop(r);
    let terminal = control.clone();
    assert!(terminal.resume().is_err());
    assert_eq!(
        terminal.snapshot().unwrap().phase,
        CaptureWorkerPhase::Stopped
    );
}
