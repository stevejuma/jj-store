use super::*;
use reqwest::{Client, Method, StatusCode};
use serde_json::{Value, json};

pub(super) async fn request(
    client: &Client,
    base: &str,
    method: Method,
    path: &str,
    body: Option<Value>,
    origin: Option<&str>,
) -> (StatusCode, Value) {
    let mut request = client
        .request(method, format!("{base}{path}"))
        .header("x-jj-store-protocol", "1");
    if let Some(body) = body {
        request = request
            .header("content-type", "application/json")
            .body(serde_json::to_vec(&body).unwrap());
    }
    if let Some(origin) = origin {
        request = request.header("origin", origin);
    }
    let response = request.send().await.unwrap();
    assert_eq!(response.headers()["cache-control"], "no-store");
    assert_eq!(response.headers()["x-content-type-options"], "nosniff");
    let status = response.status();
    (
        status,
        serde_json::from_slice(&response.bytes().await.unwrap()).unwrap(),
    )
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn local_capture_http_derives_actor_revalidates_authority_and_captures_contracts() {
    let f = Fixture::new(true).await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let api = jj_store_http::HttpApi::new(
        f.source.host.clone(),
        f.source.root.path().join("http-reads"),
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
    for (method, path, body, expected) in [
        (Method::GET, "/v1/local/capture", None, StatusCode::OK),
        (
            Method::POST,
            "/v1/local/capture/control",
            Some(serde_json::to_value(f.command(1, 0, CaptureControlAction::Pause)).unwrap()),
            StatusCode::OK,
        ),
        (
            Method::POST,
            "/v1/local/capture/control",
            Some(serde_json::to_value(f.command(2, 1, CaptureControlAction::Resume)).unwrap()),
            StatusCode::OK,
        ),
        (
            Method::POST,
            "/v1/local/capture/control",
            Some(serde_json::to_value(f.command(1, 0, CaptureControlAction::Pause)).unwrap()),
            StatusCode::OK,
        ),
        (
            Method::POST,
            "/v1/local/capture/control",
            Some(serde_json::to_value(f.command(1, 0, CaptureControlAction::Resume)).unwrap()),
            StatusCode::CONFLICT,
        ),
        (
            Method::POST,
            "/v1/local/capture/control",
            Some(serde_json::to_value(f.command(3, 0, CaptureControlAction::Pause)).unwrap()),
            StatusCode::CONFLICT,
        ),
    ] {
        let (status, data) = request(&client, &base, method.clone(), path, body, None).await;
        assert_eq!(status, expected, "{data}");
        samples.push(json!({"path":path,"method":method.as_str().to_lowercase(),"status":status.as_u16(),"media":"application/json","data":data}));
    }
    let settings = f.manager.settings(f.actor).await.unwrap();
    let mut desired = settings.desired.clone();
    desired.max_sources = 8;
    let save = SaveCaptureSettings {
        version: 1,
        expected_revision: settings.revision.clone(),
        configuration: desired,
    };
    for (method, body, expected) in [
        (Method::GET, None, StatusCode::OK),
        (
            Method::POST,
            Some(serde_json::to_value(&save).unwrap()),
            StatusCode::OK,
        ),
        (
            Method::POST,
            Some(serde_json::to_value(&save).unwrap()),
            StatusCode::CONFLICT,
        ),
    ] {
        let (status, data) = request(
            &client,
            &base,
            method.clone(),
            "/v1/local/capture/settings",
            body,
            None,
        )
        .await;
        assert_eq!(status, expected, "{data}");
        samples.push(json!({"path":"/v1/local/capture/settings","method":method.as_str().to_lowercase(),"status":status.as_u16(),"media":"application/json","data":data}));
    }
    let mut forged = serde_json::to_value(&save).unwrap();
    forged["actor"] = json!(f.actor);
    assert_eq!(
        request(
            &client,
            &base,
            Method::POST,
            "/v1/local/capture/settings",
            Some(forged),
            None
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            &client,
            &base,
            Method::POST,
            "/v1/local/capture/settings",
            Some(serde_json::to_value(&save).unwrap()),
            Some("https://foreign.example")
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    f.save(&f.config);
    let bytes = f.bytes();
    let mut forged = serde_json::to_value(f.command(3, 2, CaptureControlAction::Pause)).unwrap();
    forged["actor"] = json!(f.actor);
    assert_eq!(
        request(
            &client,
            &base,
            Method::POST,
            "/v1/local/capture/control",
            Some(forged),
            None
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            &client,
            &base,
            Method::GET,
            "/v1/local/capture",
            None,
            Some("https://foreign.example")
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(
            &client,
            &base,
            Method::GET,
            "/v1/local/capture?actor=spoof",
            None,
            None
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let mut runtime = f.runtime();
    let (status, data) =
        request(&client, &base, Method::GET, "/v1/local/capture", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(data["runtime"].is_object());
    samples.push(json!({"path":"/v1/local/capture","method":"get","status":status.as_u16(),"media":"application/json","data":data}));
    runtime.shutdown().await.unwrap();
    let mut changed = f.config.clone();
    let Access::LocalTrusted { permission, .. } = &mut changed.access else {
        unreachable!();
    };
    *permission = Permission::Write;
    f.save(&changed);
    let (status, data) =
        request(&client, &base, Method::GET, "/v1/local/capture", None, None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    samples.push(json!({"path":"/v1/local/capture","method":"get","status":status.as_u16(),"media":"application/json","data":data}));
    assert_eq!(f.bytes(), bytes);
    assert!(samples.iter().all(|s| {
        !s.to_string()
            .contains(f.source.transcript.to_str().unwrap())
    }));
    if let Some(path) = std::env::var_os("JJ_STORE_CAPTURE_MANAGEMENT_CONTRACT_CAPTURE") {
        fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
    let _ = stop.send(());
    task.await.unwrap();
}
