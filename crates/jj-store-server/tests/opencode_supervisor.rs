#![cfg(unix)]
#[path = "opencode_source/fixture.rs"]
mod api;
#[path = "capture_source/fixture.rs"]
#[allow(dead_code)]
mod native;
use api::{Api, message};
use jj_store_capture::profile::CaptureProfile;
use jj_store_server::{
    capture_authority::CaptureAuthority,
    capture_config::{AgentCaptureConfig, HarnessConfig},
    capture_runtime::RunningCodexCapture,
    capture_supervisor::{
        CodexCaptureSupervisor, CodexSupervisorLocations, CodexSupervisorResources,
    },
};
use native::{A, Fixture};
use std::{sync::Arc, time::Duration};
fn prepare(f: &Fixture, endpoint: &str) -> CodexCaptureSupervisor {
    let config = jj_store_server::config::Config::load(&f.root.path().join("server.toml")).unwrap();
    CodexCaptureSupervisor::prepare(
        AgentCaptureConfig {
            enabled: true,
            max_readers: 1,
            max_sources: 4,
            rescan_seconds: 1,
            max_pending_bytes: f.spool.pending_capacity(),
            harnesses: vec![HarnessConfig {
                profile: CaptureProfile::OpencodeReadOnlyApiV1,
                product_version: "1.18.30".into(),
                roots: vec![],
                endpoint: Some(endpoint.into()),
            }],
        },
        CodexSupervisorLocations {
            home: None,
            codex_home: None,
        },
        CodexSupervisorResources {
            authority: Arc::new(CaptureAuthority::prepare(&config).unwrap().unwrap()),
            host: f.host.clone(),
            admission: f.admission.clone(),
            sources: f.sources.clone(),
            spool: f.spool.fork_handle(),
            privacy: f.privacy(),
        },
        Duration::from_secs(10),
    )
    .unwrap()
    .unwrap()
}
async fn recorded(f: &Fixture, text: &str) -> bool {
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
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        jj_store_service::pagination::Limits::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    let page = pager.begin(native.clone(), 10).await.unwrap();
    for session in page.sessions {
        if serde_json::to_string(
            &pager
                .detail(native.clone(), session.session_id)
                .await
                .unwrap(),
        )
        .unwrap()
        .contains(text)
        {
            return true;
        }
    }
    false
}
async fn wait_recorded(f: &Fixture, runtime: &RunningCodexCapture, text: &str) {
    tokio::time::timeout(Duration::from_secs(15), async {
        while !recorded(f, text).await {
            let status = runtime.snapshot().status;
            assert!(status.running_sources <= 1);
            assert!(status.resident_source_frames <= 2);
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
}
#[tokio::test]
async fn api_runtime_automatically_backfills_follows_pause_resume_and_offline_restart() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let directory = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(
        directory.clone(),
        vec![message(
            "msg_first",
            Some(&directory),
            "automatic native API capture",
        )],
        true,
    )
    .await;
    let mut runtime = RunningCodexCapture::start(prepare(&f, &api.endpoint)).unwrap();
    wait_recorded(&f, &runtime, "automatic native API capture").await;
    assert_eq!(
        runtime
            .snapshot()
            .status
            .harnesses
            .iter()
            .flatten()
            .filter(|h| h.profile == CaptureProfile::OpencodeReadOnlyApiV1)
            .count(),
        1
    );
    runtime.pause().unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        while runtime.snapshot().phase
            != jj_store_server::capture_worker::CaptureWorkerPhase::Paused
        {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    api.data
        .lock()
        .unwrap()
        .messages
        .push(message("msg_second", Some(&directory), "paused tail"));
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(!recorded(&f, "paused tail").await);
    runtime.resume().unwrap();
    wait_recorded(&f, &runtime, "paused tail").await;
    runtime.shutdown().await.unwrap();
    api.data.lock().unwrap().busy = false;
    api.data.lock().unwrap().messages.push(message(
        "msg_offline",
        Some(&directory),
        "ended while offline",
    ));
    let mut restarted = RunningCodexCapture::start(prepare(&f, &api.endpoint)).unwrap();
    wait_recorded(&f, &restarted, "ended while offline").await;
    restarted.shutdown().await.unwrap();
    assert!(f.spool.pending().unwrap().is_empty());
}
#[tokio::test]
async fn inactive_api_history_waits_for_activity_and_paused_preparation_has_no_acquisition() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let directory = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(
        directory.clone(),
        vec![message("msg_first", Some(&directory), "inactive history")],
        false,
    )
    .await;
    let mut supervisor = prepare(&f, &api.endpoint);
    supervisor.pause();
    let mut runtime = RunningCodexCapture::start(supervisor).unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(runtime.snapshot().status.tracked, 0);
    runtime.resume().unwrap();
    tokio::time::sleep(Duration::from_millis(150)).await;
    assert!(!recorded(&f, "inactive history").await);
    api.data.lock().unwrap().busy = true;
    wait_recorded(&f, &runtime, "inactive history").await;
    runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn codex_and_api_share_one_reader_without_starving_either_harness() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let directory = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(
        directory.clone(),
        vec![message("msg_api", Some(&directory), "shared API reader")],
        true,
    )
    .await;
    let settings = AgentCaptureConfig {
        enabled: true,
        max_readers: 1,
        max_sources: 4,
        rescan_seconds: 1,
        max_pending_bytes: f.spool.pending_capacity(),
        harnesses: vec![
            HarnessConfig {
                profile: CaptureProfile::CodexCliJsonlV1,
                product_version: "0.156.1".into(),
                roots: vec![f.transcript.parent().unwrap().to_owned()],
                endpoint: None,
            },
            HarnessConfig {
                profile: CaptureProfile::OpencodeReadOnlyApiV1,
                product_version: "1.18.30".into(),
                roots: vec![],
                endpoint: Some(api.endpoint.clone()),
            },
        ],
    };
    let config = jj_store_server::config::Config::load(&f.root.path().join("server.toml")).unwrap();
    let worker = CodexCaptureSupervisor::prepare(
        settings,
        CodexSupervisorLocations {
            home: None,
            codex_home: None,
        },
        CodexSupervisorResources {
            authority: Arc::new(CaptureAuthority::prepare(&config).unwrap().unwrap()),
            host: f.host.clone(),
            admission: f.admission.clone(),
            sources: f.sources.clone(),
            spool: f.spool.fork_handle(),
            privacy: f.privacy(),
        },
        Duration::from_secs(10),
    )
    .unwrap()
    .unwrap();
    let mut runtime = RunningCodexCapture::start(worker).unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        while runtime
            .snapshot()
            .status
            .harnesses
            .iter()
            .flatten()
            .find(|h| h.profile == CaptureProfile::CodexCliJsonlV1)
            .unwrap()
            .observed_sources
            == 0
        {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    f.append(serde_json::json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"shared"}}));
    f.message("shared-file", "shared Codex reader");
    tokio::time::timeout(Duration::from_secs(20), async {
        while !(recorded(&f, "shared API reader").await
            && recorded(&f, "shared Codex reader").await)
        {
            let status = runtime.snapshot().status;
            assert!(status.running_sources <= 1);
            assert!(status.resident_source_frames <= 2);
            assert!(status.tracked <= 4);
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        while runtime.snapshot().status.resident_source_frames != 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    runtime.shutdown().await.unwrap();
}
