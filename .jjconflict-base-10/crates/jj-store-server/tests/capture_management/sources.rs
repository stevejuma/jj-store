use super::*;
use reqwest::{Client, Method, StatusCode};
use serde_json::{Value, json};

async fn get(client: &Client, base: &str, path: &str, samples: &mut Vec<Value>) -> Value {
    let (code, data) = http::request(client, base, Method::GET, path, None, None).await;
    assert_eq!(code, StatusCode::OK, "{data}");
    samples.push(json!({"path":"/v1/local/capture/sources","method":"get","status":200,"media":"application/json","data":data}));
    data
}
#[tokio::test(flavor = "current_thread")]
async fn source_pages_are_bounded_cached_and_hold_no_admission_proofs() {
    let f = Fixture::new(true).await;
    let mut header: Value =
        serde_json::from_slice(&fs::read(&f.source.transcript).unwrap()).unwrap();
    for n in [2, 3] {
        header["payload"]["id"] = json!(format!("00000000-0000-0000-0000-{n:012}"));
        fs::write(
            f.source
                .transcript
                .parent()
                .unwrap()
                .join(format!("source-{n}.jsonl")),
            format!("{header}\n"),
        )
        .unwrap();
    }
    let mut runtime = f.runtime();
    tokio::time::timeout(Duration::from_secs(30), async {
        while runtime.snapshot().status.observed_sources != 3
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
        f.source.root.path().join("source-reads"),
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
    let mut samples = vec![];
    let first = get(
        &client,
        &base,
        "/v1/local/capture/sources?limit=1",
        &mut samples,
    )
    .await;
    assert_eq!(first["total"], 3);
    assert_eq!(first["sources"].as_array().unwrap().len(), 1);
    let mut rows = first["sources"].as_array().unwrap().clone();
    let mut cursor = first["next_cursor"].clone();
    while let Some(value) = cursor.as_str() {
        let page = get(
            &client,
            &base,
            &format!("/v1/local/capture/sources?limit=1&cursor={value}"),
            &mut samples,
        )
        .await;
        assert_eq!(page["runtime_generation"], first["runtime_generation"]);
        rows.extend(page["sources"].as_array().unwrap().iter().cloned());
        cursor = page["next_cursor"].clone();
    }
    assert_eq!(rows.len(), 3);
    for row in &rows {
        assert_eq!(row["selector"].as_str().unwrap().len(), 49);
        assert_eq!(row["profile"], "codex-cli-jsonl-v1");
        assert_eq!(row["product_version"], "0.156.1");
        assert_eq!(row["observed"], true);
        assert_eq!(row["awaiting_activity"], true);
        assert!(row["last_bound_repository"].is_null());
        assert_eq!(row["caught_up"], false);
    }
    let ids: std::collections::BTreeSet<_> = rows
        .iter()
        .map(|r| r["selector"].as_str().unwrap())
        .collect();
    assert_eq!(ids.len(), 3);
    let policy = f.bytes();
    let generation = first["runtime_generation"].as_str().unwrap();
    for (query, expected, code) in [
        (
            "limit=0".into(),
            StatusCode::BAD_REQUEST,
            "invalid_capture_control",
        ),
        (
            "limit=101".into(),
            StatusCode::BAD_REQUEST,
            "invalid_capture_control",
        ),
        (
            "cursor=invalid".into(),
            StatusCode::BAD_REQUEST,
            "invalid_capture_control",
        ),
        (
            format!("cursor={generation}:0000000000000000"),
            StatusCode::BAD_REQUEST,
            "invalid_capture_control",
        ),
        (
            format!("cursor={}:0000000000000001", "ff".repeat(16)),
            StatusCode::CONFLICT,
            "capture_source_selection_changed",
        ),
        (
            format!("cursor={generation}:ffffffffffffffff"),
            StatusCode::CONFLICT,
            "capture_source_selection_changed",
        ),
        (
            "actor=spoof".into(),
            StatusCode::BAD_REQUEST,
            "invalid_request",
        ),
    ] {
        let (status, data) = http::request(
            &client,
            &base,
            Method::GET,
            &format!("/v1/local/capture/sources?{query}"),
            None,
            None,
        )
        .await;
        assert_eq!(status, expected, "{query}: {data}");
        assert_eq!(data["code"], code);
        samples.push(json!({"path":"/v1/local/capture/sources","method":"get","status":status.as_u16(),"media":"application/json","data":data}));
        assert_eq!(f.bytes(), policy);
    }
    assert!(
        f.source
            .sources
            .lookup_admitted(&f.source.qualified())
            .unwrap()
            .is_none()
    );
    assert!(f.source.spool.pending().unwrap().is_empty());
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
    let before = get(&client, &base, "/v1/local/capture/sources", &mut samples).await;
    let bytes = f.bytes();
    let transcripts = f.source.transcript.parent().unwrap();
    let saved = f.source.root.path().join("held-transcripts");
    fs::rename(transcripts, &saved).unwrap();
    let cached = get(&client, &base, "/v1/local/capture/sources", &mut samples).await;
    assert_eq!(cached, before);
    assert_eq!(f.bytes(), bytes);
    assert!(f.source.spool.pending().unwrap().is_empty());
    fs::rename(saved, transcripts).unwrap();
    let original = transcripts.join("source-2.jsonl");
    let bytes = fs::read(&original).unwrap();
    fs::rename(&original, f.source.root.path().join("retired-transcript")).unwrap();
    fs::write(&original, bytes).unwrap();
    f.manager
        .control(f.actor, f.command(2, 1, CaptureControlAction::Resume))
        .await
        .unwrap();
    let replaced = tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            let page = f
                .manager
                .sources(f.actor, CaptureSourceQuery::default())
                .await
                .unwrap();
            let current: std::collections::BTreeSet<_> =
                page.sources.iter().map(|r| r.selector.as_str()).collect();
            if current.len() == 3 && current.intersection(&ids).count() == 2 {
                assert!(
                    current
                        .difference(&ids)
                        .all(|id| *id > *ids.last().unwrap())
                );
                break page;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    assert_eq!(replaced.runtime_generation.as_deref(), Some(generation));
    assert!(f.source.spool.pending().unwrap().is_empty());
    runtime.shutdown().await.unwrap();
    let stopped = get(&client, &base, "/v1/local/capture/sources", &mut samples).await;
    assert_eq!(stopped["phase"], "stopped");
    assert_eq!(stopped["runtime_generation"], first["runtime_generation"]);
    assert!(samples.iter().all(|sample| {
        let value = sample.to_string();
        !value.contains(f.source.root.path().to_str().unwrap())
            && !value.contains("00000000-0000-0000-0000-")
    }));
    if let Some(path) = std::env::var_os("JJ_STORE_CAPTURE_SOURCES_CONTRACT_CAPTURE") {
        fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
    let _ = stop.send(());
    task.await.unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn source_pages_recheck_authority_installation_configuration_and_physical_journal() {
    let disabled = Fixture::new(false).await;
    let page = disabled
        .manager
        .sources(disabled.actor, CaptureSourceQuery::default())
        .await
        .unwrap();
    assert!(!page.enabled && page.total == 0 && page.runtime_generation.is_none());
    assert!(!disabled.config.state_dir.join("agent-capture").exists());
    let f = Fixture::new(true).await;
    let initial = f.bytes();
    assert_eq!(
        f.manager
            .sources(f.actor, CaptureSourceQuery::default())
            .await,
        Err(Error::Unavailable)
    );
    let mut runtime = f.runtime();
    runtime.pause().unwrap();
    tokio::time::timeout(Duration::from_secs(30), async {
        while runtime.snapshot().phase
            != jj_store_server::capture_worker::CaptureWorkerPhase::Paused
        {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    assert!(
        f.manager
            .sources(f.actor, CaptureSourceQuery::default())
            .await
            .is_ok()
    );
    assert_eq!(
        f.manager
            .sources(PrincipalId::new([9; 16]), CaptureSourceQuery::default())
            .await,
        Err(Error::Denied)
    );
    let mut changed = f.config.clone();
    let Access::LocalTrusted { permission, .. } = &mut changed.access else {
        unreachable!();
    };
    *permission = Permission::Write;
    f.save(&changed);
    assert_eq!(
        f.manager
            .sources(f.actor, CaptureSourceQuery::default())
            .await,
        Err(Error::Denied)
    );
    changed = f.config.clone();
    changed.agent_capture.harnesses[0]
        .roots
        .push(f.source.root.path().join("other"));
    f.save(&changed);
    assert_eq!(
        f.manager
            .sources(f.actor, CaptureSourceQuery::default())
            .await,
        Err(Error::Unavailable)
    );
    f.save(&f.config);
    let identity_path = f.source.root.path().join("installation.json");
    let mut foreign = f.identity.clone();
    foreign.installation = "cd".repeat(16);
    fs::write(&identity_path, serde_json::to_vec(&foreign).unwrap()).unwrap();
    assert_eq!(
        f.manager
            .sources(f.actor, CaptureSourceQuery::default())
            .await,
        Err(Error::Denied)
    );
    fs::write(&identity_path, serde_json::to_vec(&f.identity).unwrap()).unwrap();
    let control = f.config.state_dir.join("agent-capture/control");
    let held = f.source.root.path().join("held-control");
    fs::rename(&control, &held).unwrap();
    assert_eq!(
        f.manager
            .sources(f.actor, CaptureSourceQuery::default())
            .await,
        Err(Error::Unavailable)
    );
    fs::create_dir(&control).unwrap();
    use std::os::unix::fs::PermissionsExt as _;
    fs::set_permissions(&control, fs::Permissions::from_mode(0o700)).unwrap();
    for entry in fs::read_dir(&held).unwrap() {
        let entry = entry.unwrap();
        fs::copy(entry.path(), control.join(entry.file_name())).unwrap();
    }
    assert_eq!(
        f.manager
            .sources(f.actor, CaptureSourceQuery::default())
            .await,
        Err(Error::Unavailable)
    );
    changed = f.config.clone();
    changed.agent_capture.enabled = false;
    f.save(&changed);
    assert!(
        !f.manager
            .sources(f.actor, CaptureSourceQuery::default())
            .await
            .unwrap()
            .enabled
    );
    fs::remove_dir_all(&control).unwrap();
    fs::rename(&held, &control).unwrap();
    f.save(&f.config);
    assert!(
        f.manager
            .sources(f.actor, CaptureSourceQuery::default())
            .await
            .is_ok()
    );
    assert_eq!(f.bytes(), initial);
    assert!(f.source.spool.pending().unwrap().is_empty());
    runtime.shutdown().await.unwrap();
}
