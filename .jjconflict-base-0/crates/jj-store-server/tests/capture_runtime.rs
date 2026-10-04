#![cfg(unix)]
#[path = "capture_runtime/controls.rs"]
mod controls;
#[path = "capture_runtime/draining.rs"]
mod draining;
#[path = "capture_source/fixture.rs"]
#[allow(dead_code)]
mod fixture;
#[path = "capture_runtime/rescan.rs"]
mod rescan;
#[path = "capture_runtime/storage.rs"]
mod storage;
use fixture::*;
use jj_store_capture::profile::CaptureProfile;
use jj_store_server::{
    capture_config::{AgentCaptureConfig, HarnessConfig},
    capture_runtime::RunningCodexCapture,
    capture_supervisor::{
        CodexCaptureSupervisor, CodexSupervisorLocations, CodexSupervisorResources,
    },
    capture_worker::CaptureWorkerPhase,
};
use serde_json::json;
use std::{sync::Arc, time::Duration};

fn supervisor(f: &Fixture) -> CodexCaptureSupervisor {
    let config = AgentCaptureConfig {
        enabled: true,
        max_readers: 1,
        max_sources: 4,
        rescan_seconds: 1,
        max_pending_bytes: f.spool.pending_capacity(),
        harnesses: vec![HarnessConfig {
            profile: CaptureProfile::CodexCliJsonlV1,
            product_version: "0.156.1".into(),
            roots: vec![f.transcript.parent().unwrap().to_owned()],
            endpoint: None,
        }],
    };
    let config_file =
        jj_store_server::config::Config::load(&f.root.path().join("server.toml")).unwrap();
    CodexCaptureSupervisor::prepare(
        config,
        CodexSupervisorLocations {
            home: None,
            codex_home: None,
        },
        CodexSupervisorResources {
            authority: Arc::new(
                jj_store_server::capture_authority::CaptureAuthority::prepare(&config_file)
                    .unwrap()
                    .unwrap(),
            ),
            host: f.host.clone(),
            admission: f.admission.clone(),
            sources: f.sources.clone(),
            spool: f.spool.fork_handle(),
            privacy: f.privacy(),
        },
        Duration::from_secs(2),
    )
    .unwrap()
    .unwrap()
}
fn runtime(f: &Fixture) -> RunningCodexCapture {
    RunningCodexCapture::start(supervisor(f)).unwrap()
}
async fn wait(
    r: &RunningCodexCapture,
    predicate: impl Fn(jj_store_server::capture_runtime::CaptureRuntimeSnapshot) -> bool,
) {
    tokio::time::timeout(Duration::from_secs(30), async {
        while !predicate(r.snapshot()) {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap_or_else(|_| panic!("runtime did not settle: {:?}", r.snapshot()));
}

#[tokio::test(flavor = "current_thread")]
async fn owned_runtime_drives_discovery_active_capture_pause_and_terminal_shutdown() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    f.message("history", "eligible historical backfill");
    let mut r = runtime(&f);
    wait(&r, |s| {
        s.status.observed_sources == 1 && s.status.resident_source_frames == 0
    })
    .await;
    assert!(f.spool.pending().unwrap().is_empty());
    let active = f.start();
    drop(active);
    wait(&r, |s| s.status.caught_up == 1).await;
    let source = f.sources.lookup_admitted(&f.qualified()).unwrap().unwrap();
    let integration = serde_json::from_value(json!("codex")).unwrap();
    let before = f
        .spool
        .latest_reader_progress(integration, source)
        .unwrap()
        .unwrap();
    r.pause().unwrap();
    wait(&r, |s| s.phase == CaptureWorkerPhase::Paused).await;
    f.message("paused", "capture after internal resume");
    tokio::time::sleep(Duration::from_millis(80)).await;
    assert_eq!(
        f.spool.latest_reader_progress(integration, source).unwrap(),
        Some(before.clone())
    );
    r.resume().unwrap();
    wait(&r, |s| {
        s.status.caught_up == 1
            && f.spool
                .latest_reader_progress(integration, source)
                .unwrap()
                .is_some_and(|p| p.last_sequence > before.last_sequence)
    })
    .await;
    r.shutdown().await.unwrap();
    assert_eq!(r.snapshot().phase, CaptureWorkerPhase::Stopped);
    assert_eq!(r.snapshot().status.resident_source_frames, 0);
    assert!(r.resume().is_err());
    assert!(r.pause().is_err());
    r.shutdown().await.unwrap();
    drop(f.sources.acquire_admitted(f.qualified()).unwrap().unwrap());
}

#[tokio::test(flavor = "current_thread")]
async fn runtime_shutdown_does_not_acknowledge_a_backpressured_raw_frame() {
    let f = Fixture::new(1).await;
    f.message("raw", "raw data must stay unacknowledged");
    drop(f.sources.acquire_active(f.start()).unwrap());
    let mut r = runtime(&f);
    wait(&r, |s| {
        s.status.ingress_capacity_refusals == 1 && s.status.resident_source_frames == 1
    })
    .await;
    r.pause().unwrap();
    wait(&r, |s| s.phase == CaptureWorkerPhase::Paused).await;
    assert!(f.sources.acquire_admitted(f.qualified()).is_err());
    assert!(f.spool.pending().unwrap().is_empty());
    let id = f.sources.lookup_admitted(&f.qualified()).unwrap().unwrap();
    assert!(
        f.spool
            .latest_reader_progress(serde_json::from_value(json!("codex")).unwrap(), id)
            .unwrap()
            .is_none()
    );
    r.shutdown().await.unwrap();
    assert_eq!(r.snapshot().phase, CaptureWorkerPhase::Stopped);
    assert!(f.spool.pending().unwrap().is_empty());
    drop(f.sources.acquire_admitted(f.qualified()).unwrap().unwrap());
}

#[tokio::test(flavor = "current_thread")]
async fn failed_driver_marks_controls_terminal_and_never_repolls_a_completed_join() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let mut supervisor = supervisor(&f);
    supervisor.shutdown().await.unwrap();
    let mut r = RunningCodexCapture::start(supervisor).unwrap();
    assert!(r.shutdown().await.is_ok()); // stop before dispatch is also safe
    assert!(r.resume().is_err());
    assert!(r.shutdown().await.is_ok());
    let mut supervisor = self::supervisor(&f);
    supervisor.shutdown().await.unwrap();
    let mut r = RunningCodexCapture::start(supervisor).unwrap();
    // The driver must observe the failed dispatch independently of an explicit stop.
    tokio::time::timeout(Duration::from_secs(10), async {
        while r.resume().is_ok() {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    assert_eq!(r.snapshot().phase, CaptureWorkerPhase::Stopped);
    assert!(r.shutdown().await.is_err());
    assert!(r.shutdown().await.is_err());
    assert!(r.pause().is_err());
    assert!(f.spool.pending().unwrap().is_empty());
}
