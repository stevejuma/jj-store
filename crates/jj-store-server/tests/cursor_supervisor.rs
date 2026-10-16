#![cfg(unix)]
#[path = "capture_source/fixture.rs"]
#[allow(dead_code)]
mod fixture;
use fixture::*;
use jj_store_capture::{cursor_binding, profile::CaptureProfile};
use jj_store_server::{
    capture_config::{AgentCaptureConfig, HarnessConfig},
    capture_runtime::RunningCodexCapture,
    capture_supervisor::{
        CodexCaptureSupervisor, CodexSupervisorLocations, CodexSupervisorResources,
    },
    capture_worker::CaptureWorkerPhase,
};
use serde_json::json;
use std::{fs, io::Write, sync::Arc, time::Duration};
fn prepare(f: &Fixture, p: CaptureProfile) -> CodexCaptureSupervisor {
    CodexCaptureSupervisor::prepare(
        AgentCaptureConfig {
            enabled: true,
            max_readers: 1,
            max_sources: 4,
            rescan_seconds: 1,
            max_pending_bytes: f.spool.pending_capacity(),
            harnesses: vec![HarnessConfig {
                profile: p,
                product_version: p.product_version().into(),
                roots: vec![f.transcript.parent().unwrap().to_owned()],
                endpoint: None,
            }],
        },
        CodexSupervisorLocations {
            home: None,
            codex_home: None,
        },
        CodexSupervisorResources {
            authority: Arc::new(
                jj_store_server::capture_authority::CaptureAuthority::prepare(
                    &jj_store_server::config::Config::load(&f.root.path().join("server.toml"))
                        .unwrap(),
                )
                .unwrap()
                .unwrap(),
            ),
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
fn bind(f: &Fixture, p: CaptureProfile, event: &str) {
    let path = f.transcript.canonicalize().unwrap();
    cursor_binding::write_hook(&serde_json::to_vec(&json!({"cursor_version":p.product_version(),"workspace_roots":[f.workspace(A)],"conversation_id":"12345678-1234-1234-1234-123456789abc","transcript_path":path,"hook_event_name":event})).unwrap(),&f.workspace(A),&[path.parent().unwrap().to_owned()]).unwrap();
}
fn append(f: &Fixture, text: &str) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&f.transcript)
        .unwrap();
    writeln!(
        file,
        "{}",
        json!({"role":"assistant","message":{"content":[{"type":"text","text":text}]}})
    )
    .unwrap();
}
async fn details(f: &Fixture) -> Vec<serde_json::Value> {
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
    let mut out = vec![];
    for s in page.sessions {
        out.push(
            serde_json::to_value(pager.detail(native.clone(), s.session_id).await.unwrap())
                .unwrap(),
        );
    }
    out
}
async fn wait_message(f: &Fixture, r: &RunningCodexCapture, text: &str) {
    tokio::time::timeout(Duration::from_secs(15), async {
        while !serde_json::to_string(&details(f).await)
            .unwrap()
            .contains(text)
        {
            assert!(r.snapshot().status.running_sources <= 1);
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap_or_else(|_| panic!("capture did not publish: {:?}", r.snapshot()));
}
async fn wait_phase(r: &RunningCodexCapture, phase: CaptureWorkerPhase) {
    tokio::time::timeout(Duration::from_secs(10), async {
        while r.snapshot().phase != phase {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
}
#[tokio::test]
async fn linked_cursor_products_backfill_pause_follow_final_flush_and_restart_without_duplicate_messages()
 {
    for p in [
        CaptureProfile::CursorDesktopTranscriptV1,
        CaptureProfile::CursorCliTranscriptV1,
    ] {
        let f = Fixture::new(4 * 1024 * 1024).await;
        fs::write(&f.transcript, "").unwrap();
        append(&f, "Cursor fixture backfill");
        bind(&f, p, "beforeSubmitPrompt");
        let mut r = RunningCodexCapture::start(prepare(&f, p)).unwrap();
        wait_message(&f, &r, "Cursor fixture backfill").await;
        assert_eq!(details(&f).await.len(), 1);
        assert!(
            serde_json::to_string(&details(&f).await)
                .unwrap()
                .contains("cursor")
        );
        r.pause().unwrap();
        wait_phase(&r, CaptureWorkerPhase::Paused).await;
        append(&f, "Cursor paused tail");
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(
            !serde_json::to_string(&details(&f).await)
                .unwrap()
                .contains("Cursor paused tail")
        );
        r.resume().unwrap();
        wait_message(&f, &r, "Cursor paused tail").await;
        r.shutdown().await.unwrap();
        bind(&f, p, "sessionEnd");
        append(&f, "Cursor delayed final flush");
        let mut r = RunningCodexCapture::start(prepare(&f, p)).unwrap();
        wait_message(&f, &r, "Cursor delayed final flush").await;
        r.shutdown().await.unwrap();
        let sessions = details(&f).await;
        assert_eq!(sessions.len(), 1);
        // Discovery excerpts may repeat message text; only activity records
        // represent published messages.
        let messages = sessions[0]["activity"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|activity| activity["type"] == "message")
            .map(|activity| activity["content"]["content"].as_str().unwrap())
            .collect::<Vec<_>>();
        for text in [
            "Cursor fixture backfill",
            "Cursor paused tail",
            "Cursor delayed final flush",
        ] {
            assert_eq!(
                messages.iter().filter(|body| body.contains(text)).count(),
                1
            );
        }
        assert!(f.spool.pending().unwrap().is_empty());
    }
}
#[tokio::test]
async fn inactive_unlinked_and_disabled_cursor_profiles_never_enroll_or_publish() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let p = CaptureProfile::CursorCliTranscriptV1;
    fs::write(&f.transcript, "").unwrap();
    append(&f, "Cursor withheld fixture");
    bind(&f, p, "stop");
    let mut r = RunningCodexCapture::start(prepare(&f, p)).unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        while r.snapshot().status.observed_sources != 1 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    assert!(details(&f).await.is_empty());
    assert!(f.spool.pending().unwrap().is_empty());
    r.shutdown().await.unwrap();
    f.catalog(&[]);
    bind(&f, p, "beforeSubmitPrompt");
    let mut r = RunningCodexCapture::start(prepare(&f, p)).unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        while r.snapshot().status.unavailable == 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    assert!(details(&f).await.is_empty());
    assert!(f.spool.pending().unwrap().is_empty());
    r.shutdown().await.unwrap();
    let disabled = AgentCaptureConfig::default();
    assert!(
        CodexCaptureSupervisor::prepare(
            disabled,
            CodexSupervisorLocations {
                home: None,
                codex_home: None
            },
            CodexSupervisorResources {
                authority: Arc::new(
                    jj_store_server::capture_authority::CaptureAuthority::prepare(
                        &jj_store_server::config::Config::load(&f.root.path().join("server.toml"))
                            .unwrap()
                    )
                    .unwrap()
                    .unwrap()
                ),
                host: f.host.clone(),
                admission: f.admission.clone(),
                sources: f.sources.clone(),
                spool: f.spool.fork_handle(),
                privacy: f.privacy()
            },
            Duration::from_secs(10)
        )
        .unwrap()
        .is_none()
    );
}
