use super::*;
use reqwest::{Client, Method, StatusCode};
use serde_json::json;

fn command(f: &Fixture, revision: u64) -> CaptureRescanCommand {
    CaptureRescanCommand {
        version: 1,
        generation: hex::encode(f.binding.as_ref().unwrap().generation),
        expected_revision: revision,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rescan_is_conditional_actor_free_and_preserves_device_policy() {
    let f = Fixture::new(true).await;
    let mut runtime = f.runtime();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let api = jj_store_http::HttpApi::new(
        f.source.host.clone(),
        f.source.root.path().join("rescan-reads"),
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
    let job = tokio::spawn(async move {
        axum::serve(listener, api.router())
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let client = Client::new();
    let path = "/v1/local/capture/rescan";
    let body = serde_json::to_value(command(&f, 0)).unwrap();
    let mut samples = Vec::new();
    let bytes = f.bytes();
    for _ in 0..2 {
        let (status, data) =
            http::request(&client, &base, Method::POST, path, Some(body.clone()), None).await;
        assert_eq!(
            status,
            StatusCode::OK,
            "rescan endpoint absent or unavailable: {data}"
        );
        assert_eq!(data["accepted"], true);
        assert_eq!(data["status"]["policy"]["revision"], 0);
        let coverage = &data["status"]["runtime"]["discovery"];
        assert_eq!(coverage["roots"], 1);
        assert_eq!(
            coverage["unchecked_roots"].as_u64().unwrap()
                + coverage["available_roots"].as_u64().unwrap()
                + coverage["unavailable_roots"].as_u64().unwrap(),
            1
        );
        samples.push(json!({"path":path,"method":"post","status":status.as_u16(),"media":"application/json","data":data}));
        assert_eq!(f.bytes(), bytes);
    }
    let mut cases = Vec::new();
    for field in ["actor", "principal", "request_id", "path"] {
        let mut forged = body.clone();
        forged[field] = json!("untrusted");
        cases.push((forged, StatusCode::BAD_REQUEST));
    }
    for field in ["version", "generation", "expected_revision"] {
        let mut missing = body.clone();
        missing.as_object_mut().unwrap().remove(field);
        cases.push((missing, StatusCode::BAD_REQUEST));
    }
    for generation in ["00".repeat(16), "AB".repeat(16), "ab".repeat(15)] {
        let mut invalid = body.clone();
        invalid["generation"] = json!(generation);
        cases.push((invalid, StatusCode::BAD_REQUEST));
    }
    let mut invalid = body.clone();
    invalid["version"] = json!(2);
    cases.push((invalid, StatusCode::BAD_REQUEST));
    let mut foreign = body.clone();
    foreign["generation"] = json!("ff".repeat(16));
    cases.push((foreign, StatusCode::CONFLICT));
    let mut stale = body.clone();
    stale["expected_revision"] = json!(1);
    cases.push((stale, StatusCode::CONFLICT));
    for (body, expected) in cases {
        let (status, data) =
            http::request(&client, &base, Method::POST, path, Some(body), None).await;
        assert_eq!(status, expected, "{data}");
        samples.push(json!({"path":path,"method":"post","status":status.as_u16(),"media":"application/json","data":data}));
        assert_eq!(f.bytes(), bytes);
    }
    assert_eq!(
        http::request(
            &client,
            &base,
            Method::POST,
            path,
            Some(body.clone()),
            Some("https://foreign.example")
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        http::request(
            &client,
            &base,
            Method::POST,
            &format!("{path}?actor=spoof"),
            Some(body.clone()),
            None
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let mut oversized = serde_json::to_vec(&body).unwrap();
    oversized.resize(4097, b' ');
    assert_eq!(
        client
            .post(format!("{base}{path}"))
            .header("x-jj-store-protocol", "1")
            .header("content-type", "application/json")
            .body(oversized)
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::PAYLOAD_TOO_LARGE
    );
    assert_eq!(f.bytes(), bytes);
    f.manager
        .control(f.actor, f.command(1, 0, CaptureControlAction::Pause))
        .await
        .unwrap();
    let paused = f.bytes();
    let (status, data) = http::request(
        &client,
        &base,
        Method::POST,
        path,
        Some(serde_json::to_value(command(&f, 1)).unwrap()),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(data["code"], "capture_paused");
    samples.push(json!({"path":path,"method":"post","status":status.as_u16(),"media":"application/json","data":data}));
    assert_eq!(f.bytes(), paused);
    f.manager
        .control(f.actor, f.command(2, 1, CaptureControlAction::Resume))
        .await
        .unwrap();
    let body = serde_json::to_value(command(&f, 2)).unwrap();
    let selected = f.bytes();
    for (enabled, permission, expected) in [
        (true, Permission::Write, StatusCode::FORBIDDEN),
        (false, Permission::Admin, StatusCode::CONFLICT),
    ] {
        let mut changed = f.config.clone();
        changed.agent_capture.enabled = enabled;
        let Access::LocalTrusted { permission: p, .. } = &mut changed.access else {
            unreachable!();
        };
        *p = permission;
        f.save(&changed);
        let (status, data) =
            http::request(&client, &base, Method::POST, path, Some(body.clone()), None).await;
        assert_eq!(status, expected);
        samples.push(json!({"path":path,"method":"post","status":status.as_u16(),"media":"application/json","data":data}));
        assert_eq!(f.bytes(), selected);
    }
    f.save(&f.config);
    runtime.shutdown().await.unwrap();
    let (status, data) = http::request(&client, &base, Method::POST, path, Some(body), None).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    samples.push(json!({"path":path,"method":"post","status":status.as_u16(),"media":"application/json","data":data}));
    assert_eq!(f.bytes(), selected);
    assert!(f.source.spool.pending().unwrap().is_empty());
    assert!(
        f.source
            .sources
            .lookup_admitted(&f.source.qualified())
            .unwrap()
            .is_none()
    );
    assert!(samples.iter().all(|s| {
        !s.to_string()
            .contains(f.source.transcript.to_str().unwrap())
    }));
    if let Some(path) = std::env::var_os("JJ_STORE_CAPTURE_RESCAN_CONTRACT_CAPTURE") {
        fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
    let _ = stop.send(());
    job.await.unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn rescan_denials_preserve_policy_and_never_initialize_missing_state() {
    let f = Fixture::new(false).await;
    let request = CaptureRescanCommand {
        version: 1,
        generation: "ab".repeat(16),
        expected_revision: 0,
    };
    assert_eq!(
        f.manager.rescan(f.actor, request).await,
        Err(Error::Disabled)
    );
    assert!(!f.config.state_dir.join("agent-capture").exists());
    let f = Fixture::new(true).await;
    let request = command(&f, 0);
    let bytes = f.bytes();
    assert_eq!(
        f.manager.rescan(f.actor, request.clone()).await,
        Err(Error::Unavailable)
    );
    assert_eq!(
        f.manager
            .rescan(PrincipalId::new([9; 16]), request.clone())
            .await,
        Err(Error::Denied)
    );
    let mut changed = f.config.clone();
    changed.agent_capture.rescan_seconds = 30;
    f.save(&changed);
    assert_eq!(
        f.manager.rescan(f.actor, request.clone()).await,
        Err(Error::Unavailable)
    );
    f.save(&f.config);
    let identity_path = f.source.root.path().join("installation.json");
    let mut foreign = f.identity.clone();
    foreign.installation = "cd".repeat(16);
    fs::write(&identity_path, serde_json::to_vec(&foreign).unwrap()).unwrap();
    assert_eq!(
        f.manager.rescan(f.actor, request.clone()).await,
        Err(Error::Denied)
    );
    fs::write(identity_path, serde_json::to_vec(&f.identity).unwrap()).unwrap();
    assert_eq!(f.bytes(), bytes);
    fs::remove_file(f.config.state_dir.join("agent-capture/control/state.json")).unwrap();
    assert_eq!(
        f.manager.rescan(f.actor, request).await,
        Err(Error::Unavailable)
    );
    assert!(
        !f.config
            .state_dir
            .join("agent-capture/control/state.json")
            .exists()
    );
    assert!(f.source.spool.pending().unwrap().is_empty());
}
