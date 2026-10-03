use super::*;

async fn until(
    worker: &mut CodexCaptureSupervisor,
    predicate: impl Fn(CodexSupervisorStatus) -> bool,
) {
    tokio::time::timeout(Duration::from_secs(30), async {
        while !predicate(worker.status()) {
            worker.start_step().unwrap();
            settle(worker).await;
        }
    })
    .await
    .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn source_degradation_is_cached_and_clears_after_successful_recovery() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    f.message("history", "qualified source before temporary revocation");
    let mut worker = supervisor(&f);
    until(&mut worker, |s| {
        s.observed_sources == 1 && s.resident_source_frames == 0
    })
    .await;
    drop(f.start());
    until(&mut worker, |s| s.caught_up == 1).await;
    assert_eq!(worker.status().degraded_sources, 0);
    f.catalog(&[]);
    f.message("tail", "withheld during source admission loss");
    until(&mut worker, |s| s.degraded_sources == 1).await;
    let observed = worker.status();
    assert!(observed.unavailable > 0);
    f.catalog(&[fixture::A, fixture::B]);
    for _ in 0..100 {
        assert_eq!(worker.status(), observed, "status performed source I/O");
    }
    until(&mut worker, |s| s.caught_up == 1 && s.degraded_sources == 0).await;
    worker.pause();
    settle(&mut worker).await;
    assert_eq!(
        worker.status().degraded_sources,
        0,
        "ordinary cancellation invented degradation"
    );
    assert!(worker.rescan().is_err());
    worker.shutdown().await.unwrap();
    assert_eq!(worker.status().degraded_sources, 0);
}
