use super::*;
use jj_store_service::{auth::Permission, collaboration::AgentSessionPager};
use std::{fs, io::Write as _};

#[tokio::test(flavor = "current_thread")]
async fn explicit_rescan_discovers_before_cooldown_but_preserves_activity_admission() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let path = f.root.path().join("server.toml");
    let mut config = jj_store_server::config::Config::load(&path).unwrap();
    config.agent_capture.rescan_seconds = 30;
    config.agent_capture.max_readers = 1;
    config.agent_capture.max_sources = 4;
    config.agent_capture.max_pending_bytes = f.spool.pending_capacity();
    config.agent_capture.harnesses[0].roots = vec![f.transcript.parent().unwrap().to_owned()];
    fs::write(path, toml::to_string(&config).unwrap()).unwrap();
    let prepared = CodexCaptureSupervisor::prepare(
        config.agent_capture.clone(),
        CodexSupervisorLocations {
            home: None,
            codex_home: None,
        },
        CodexSupervisorResources {
            authority: Arc::new(
                jj_store_server::capture_authority::CaptureAuthority::prepare(&config)
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
    .unwrap();
    let mut runtime = RunningCodexCapture::start(prepared).unwrap();
    let control = runtime.controls();
    wait(&runtime, |s| {
        s.status.observed_sources == 1 && s.status.resident_source_frames == 0
    })
    .await;
    let catalog = fs::read(f.root.path().join("sources/sources.json")).unwrap();
    let original = fs::read_to_string(&f.transcript).unwrap();
    let added = f.transcript.parent().unwrap().join("added.jsonl");
    let header = original.replace(
        "00000000-0000-0000-0000-000000000001",
        "00000000-0000-0000-0000-000000000002",
    );
    let history = json!({"type":"response_item","payload":{"type":"message","id":"history","role":"assistant","content":[{"type":"output_text","text":format!("rescan historical body {SECRET}")}]}});
    fs::write(
        &added,
        format!(
            "{header}{}\n{history}\n",
            json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"already-past"}})
        ),
    )
    .unwrap();
    tokio::time::sleep(Duration::from_millis(80)).await;
    assert_eq!(control.snapshot().unwrap().status.observed_sources, 1);
    let began = std::time::Instant::now();
    for _ in 0..100 {
        control.rescan().unwrap();
    }
    tokio::time::timeout(Duration::from_secs(2), async {
        while control.snapshot().unwrap().status.observed_sources != 2 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    assert!(began.elapsed() < Duration::from_secs(2));
    wait(&runtime, |s| {
        s.status.awaiting_activity == 2 && s.status.resident_source_frames == 0
    })
    .await;
    assert_eq!(
        fs::read(f.root.path().join("sources/sources.json")).unwrap(),
        catalog
    );
    assert!(f.spool.pending().unwrap().is_empty());
    let pager = AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    let native = f
        .host
        .endpoint(A)
        .unwrap()
        .bind_local_trusted(f.authority.principal(), Permission::Read)
        .unwrap()
        .into_session();
    assert!(
        pager
            .begin(native.clone(), 10)
            .await
            .unwrap()
            .sessions
            .is_empty()
    );
    let growth = json!({"type":"response_item","payload":{"type":"message","id":"growth","role":"assistant","content":[{"type":"output_text","text":"rescan growth alone"}]}});
    let mut file = fs::OpenOptions::new().append(true).open(&added).unwrap();
    writeln!(file, "{growth}").unwrap();
    control.rescan().unwrap();
    tokio::time::sleep(Duration::from_millis(80)).await;
    assert!(
        pager
            .begin(native.clone(), 10)
            .await
            .unwrap()
            .sessions
            .is_empty()
    );
    writeln!(
        file,
        "{}",
        json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"new-start"}})
    )
    .unwrap();
    wait(&runtime, |s| s.status.caught_up == 1).await;
    let page = pager.begin(native.clone(), 10).await.unwrap();
    assert_eq!(page.sessions.len(), 1);
    let body = serde_json::to_string(
        &pager
            .detail(native, page.sessions[0].session_id)
            .await
            .unwrap(),
    )
    .unwrap();
    assert!(
        body.contains("rescan historical body")
            && body.contains("rescan growth alone")
            && !body.contains(SECRET)
    );
    control.pause().unwrap();
    wait(&runtime, |s| s.phase == CaptureWorkerPhase::Paused).await;
    assert!(control.rescan().is_err());
    assert!(!control.snapshot().unwrap().status.discovery.rescan_pending);
    runtime.shutdown().await.unwrap();
    assert!(control.rescan().is_err());
    assert!(!control.snapshot().unwrap().status.discovery.rescan_pending);
}
