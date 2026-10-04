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

#[tokio::test(flavor = "current_thread")]
async fn configured_roots_retain_default_home_override_and_explicit_selection() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let home = f.root.path().join("configured-home");
    let codex_home = f.root.path().join("configured-codex-home");
    let explicit = f.root.path().join("explicit-cli-sessions");
    for (override_home, explicit_roots, expected) in [
        (None, vec![], home.join(".codex/sessions")),
        (
            Some(codex_home.clone()),
            vec![],
            codex_home.join("sessions"),
        ),
        (Some(codex_home.clone()), vec![explicit.clone()], explicit),
    ] {
        let mut config = crate::config::Config::load(&f.root.path().join("server.toml")).unwrap();
        config.agent_capture.max_pending_bytes = f.spool.pending_capacity();
        config.agent_capture.harnesses[0].roots = explicit_roots;
        let worker = CodexCaptureSupervisor::prepare(
            config.agent_capture.clone(),
            CodexSupervisorLocations {
                home: Some(home.clone()),
                codex_home: override_home,
            },
            CodexSupervisorResources {
                authority: Arc::new(CaptureAuthority::prepare(&config).unwrap().unwrap()),
                host: f.host.clone(),
                admission: f.admission.clone(),
                sources: f.sources.clone(),
                spool: f.spool.fork_handle(),
                privacy: f.privacy(),
            },
            Duration::from_secs(2),
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            &*worker.configured_roots(),
            &[(CaptureProfile::CodexCliJsonlV1, expected.clone())]
        );
        assert!(
            !expected.exists(),
            "configuration reporting must not create or inspect directory contents"
        );
    }
}
