use super::*;
use jj_store_capture::profile::CaptureProfile;
use jj_store_server::capture_config::HarnessConfig;
use reqwest::{Client, Method, StatusCode};
use serde_json::{Value, json};

async fn sample(client: &Client, base: &str, samples: &mut Vec<Value>) -> Value {
    let (status, data) =
        http::request(client, base, Method::GET, "/v1/local/capture", None, None).await;
    assert_eq!(status, StatusCode::OK);
    samples.push(json!({"path":"/v1/local/capture","method":"get","status":200,"media":"application/json","data":data}));
    data
}

#[tokio::test(flavor = "current_thread")]
async fn harness_coverage_is_bounded_cached_and_contains_no_roots() {
    let mut f = Fixture::new(true).await;
    let desktop = f.source.root.path().join("desktop-transcripts");
    fs::create_dir(&desktop).unwrap();
    f.config.agent_capture.harnesses.push(HarnessConfig {
        profile: CaptureProfile::CodexDesktopJsonlV1,
        product_version: "0.159.2".into(),
        roots: vec![desktop.clone()],
        endpoint: None,
    });
    f.config.agent_capture.harnesses.reverse(); // Public grouping is independent of configuration order.
    f.save(&f.config);
    f.manager = Manager::new(
        &f.source.root.path().join("server.toml"),
        &f.config,
        &f.identity,
        f.binding.clone(),
    )
    .unwrap();
    let mut runtime = f.runtime();
    tokio::time::timeout(Duration::from_secs(30), async {
        while runtime.snapshot().status.discovery.available_roots != 2
            || runtime.snapshot().status.observed_sources != 1
            || runtime.snapshot().status.resident_source_frames != 0
        {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let api = jj_store_http::HttpApi::new(
        f.source.host.clone(),
        f.source.root.path().join("coverage-reads"),
        jj_store_http::Config {
            origin: base.clone(),
            access: jj_store_http::Access::LocalTrusted {
                principal: f.actor,
                permission: Permission::Admin,
            },
            local_capture: Some(Arc::new(f.manager.clone())),
            ..Default::default()
        },
    )
    .unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(async move {
        axum::serve(listener, api.router())
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let client = Client::new();
    let mut samples = Vec::new();
    let status = sample(&client, &base, &mut samples).await;
    let coverage = status["runtime"]["harnesses"]
        .as_array()
        .expect("missing per-harness coverage");
    assert_eq!(coverage.len(), 2);
    for field in [
        "roots",
        "unchecked_roots",
        "available_roots",
        "unavailable_roots",
    ] {
        let sum: u64 = coverage.iter().map(|g| g[field].as_u64().unwrap()).sum();
        assert_eq!(sum, status["runtime"]["discovery"][field].as_u64().unwrap());
    }
    for field in [
        "tracked",
        "observed_sources",
        "awaiting_activity",
        "caught_up",
        "running_sources",
        "degraded_sources",
    ] {
        let sum: u64 = coverage.iter().map(|g| g[field].as_u64().unwrap()).sum();
        assert_eq!(sum, status["runtime"][field].as_u64().unwrap());
    }
    for (index, profile, version, tracked) in [
        (0, "codex-cli-jsonl-v1", "0.156.1", 1),
        (1, "codex-desktop-jsonl-v1", "0.159.2", 0),
    ] {
        let item = &coverage[index];
        assert_eq!(item["profile"], profile);
        assert_eq!(item["product_version"], version);
        assert_eq!(item["roots"], 1);
        assert_eq!(item["available_roots"], 1);
        assert_eq!(item["unchecked_roots"], 0);
        assert_eq!(item["unavailable_roots"], 0);
        assert_eq!(item["tracked"], tracked);
        assert_eq!(item["observed_sources"], tracked);
        assert_eq!(item["awaiting_activity"], tracked);
        assert_eq!(item["caught_up"], 0);
        assert_eq!(item["degraded_sources"], 0);
    }
    assert!(
        !status
            .to_string()
            .contains(f.source.root.path().to_str().unwrap())
    );
    f.manager
        .control(f.actor, f.command(1, 0, CaptureControlAction::Pause))
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(30), async {
        while runtime.snapshot().phase
            != jj_store_server::capture_worker::CaptureWorkerPhase::Paused
        {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    let bytes = f.bytes();
    let before = sample(&client, &base, &mut samples).await["runtime"]["harnesses"].clone();
    let cli = f.source.transcript.parent().unwrap();
    let saved_cli = f.source.root.path().join("retained-cli");
    let saved_desktop = f.source.root.path().join("retained-desktop");
    fs::rename(cli, &saved_cli).unwrap();
    fs::rename(&desktop, &saved_desktop).unwrap();
    let cached = sample(&client, &base, &mut samples).await;
    assert_eq!(
        cached["runtime"]["harnesses"], before,
        "status read roots while paused"
    );
    assert_eq!(f.bytes(), bytes);
    assert!(f.source.spool.pending().unwrap().is_empty());
    fs::rename(saved_cli, cli).unwrap();
    fs::rename(saved_desktop, desktop).unwrap();
    runtime.shutdown().await.unwrap();
    let stopped = sample(&client, &base, &mut samples).await;
    assert_eq!(stopped["runtime"]["phase"], "stopped");
    assert!(samples.iter().all(|s| {
        !s.to_string()
            .contains(f.source.root.path().to_str().unwrap())
    }));
    if let Some(path) = std::env::var_os("JJ_STORE_CAPTURE_COVERAGE_CONTRACT_CAPTURE") {
        fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
    let _ = stop.send(());
    task.await.unwrap();
}
