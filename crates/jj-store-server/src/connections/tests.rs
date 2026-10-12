use super::*;
#[test]
fn origins_require_tls_and_reject_credentials_and_paths() {
    assert_eq!(
        origin("https://store.example/", false).unwrap(),
        "https://store.example"
    );
    for target in [
        "http://store.example",
        "https://user:secret@store.example",
        "https://store.example/path",
        "https://store.example?token=secret",
        "https://store.example#fragment",
        "file:///tmp/foo",
    ] {
        assert!(origin(target, false).is_err(), "{target}");
    }
    assert!(origin("http://localhost:9000", true).is_err());
    assert!(origin("http://127.0.0.1:9000", true).is_ok());
}
#[tokio::test]
async fn registry_restart_retains_hints_without_credentials() {
    let root = tempfile::tempdir().unwrap();
    let manager = Manager::open(root.path().into(), false).unwrap();
    let connection = Connection {
        id: "a".repeat(32),
        name: "Build host".into(),
        origin: "https://store.example".into(),
        installation: "b".repeat(32),
    };
    let mut saved = Saved::default();
    saved
        .connections
        .insert(connection.id.clone(), connection.clone());
    manager.persist(&saved).unwrap();
    let restarted = Manager::open(root.path().into(), false).unwrap();
    assert_eq!(restarted.list().await.unwrap(), vec![connection.clone()]);
    assert!(restarted.0.state.lock().await.sessions.is_empty());
    restarted
        .edit(Edit::Rename {
            id: connection.id.clone(),
            name: "Remote".into(),
        })
        .await
        .unwrap();
    assert_eq!(restarted.list().await.unwrap()[0].name, "Remote");
    restarted
        .edit(Edit::Remove { id: connection.id })
        .await
        .unwrap();
    assert!(restarted.list().await.unwrap().is_empty());
}
#[tokio::test]
async fn gateway_binds_sessions_handles_binary_and_refuses_identity_replacement() {
    use axum::{
        Router,
        response::IntoResponse,
        routing::{get, post},
    };
    let token = BearerToken::generate().unwrap();
    let credential = token.expose();
    let expected = credential.to_string();
    let installation = Arc::new(tokio::sync::RwLock::new("c".repeat(32)));
    let identity = installation.clone();
    let router=Router::new()
        .route("/v1/bootstrap",get(move||{let identity=identity.clone();async move{axum::Json(serde_json::json!({"version":1,"access_mode":"authenticated","installation_id":identity.read().await.clone()}))}}))
        .route("/v1/directory/profile",get(move|headers:axum::http::HeaderMap|{let expected=expected.clone();async move{if headers.get("authorization").is_some_and(|v|v.as_bytes()==format!("Bearer {expected}").as_bytes()){axum::Json(serde_json::json!({"principal":"d".repeat(32)})).into_response()}else{axum::http::StatusCode::FORBIDDEN.into_response()}}}))
        .route("/v1/repos",get(||async{axum::Json(serde_json::json!({"repositories":[]}))}))
        .route("/v1/directory/media",post(|headers:axum::http::HeaderMap,body:axum::body::Bytes|async move{assert!(headers.get("origin").is_none());assert!(headers.get("cookie").is_none());assert!(headers.get("sec-fetch-site").is_none());assert!(headers.get("authorization").is_some());body}));
    let observed = installation.clone();
    let router = router.layer(axum::middleware::from_fn(
        move |request: axum::extract::Request, next: axum::middleware::Next| {
            let observed = observed.clone();
            async move {
                let id = observed.read().await.clone();
                let mut response = if request
                    .headers()
                    .get("x-jj-store-target-installation")
                    .is_some_and(|v| v.as_bytes() != id.as_bytes())
                {
                    axum::http::StatusCode::CONFLICT.into_response()
                } else {
                    next.run(request).await
                };
                response
                    .headers_mut()
                    .insert("x-jj-store-installation", id.parse().unwrap());
                response
            }
        },
    ));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
    let root = tempfile::tempdir().unwrap();
    let manager = Manager::open(root.path().into(), true).unwrap();
    let connected = manager
        .connect(Connect {
            id: None,
            name: "Remote".into(),
            origin: origin.clone(),
            token: credential.to_string(),
        })
        .await
        .unwrap();
    assert!(
        !String::from_utf8(std::fs::read(root.path().join("connections.json")).unwrap())
            .unwrap()
            .contains(credential.as_str())
    );
    let make = |path: &str, handle: &str, body: Vec<u8>| Forward {
        connection: connected.connection.id.clone(),
        handle: handle.into(),
        method: if body.is_empty() { "GET" } else { "POST" }.into(),
        path: path.into(),
        headers: vec![
            ("origin".into(), "https://local.invalid".into()),
            ("cookie".into(), "wrong".into()),
        ],
        body,
    };
    let mut response = manager
        .forward(make(
            "directory/media",
            &connected.handle,
            vec![0, 1, 255, 128],
        ))
        .await
        .unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(
        response.chunks.recv().await.unwrap().unwrap(),
        vec![0, 1, 255, 128]
    );
    assert!(matches!(
        manager
            .forward(make("repos", &"e".repeat(32), vec![]))
            .await,
        Err(Error::Denied)
    ));
    assert!(matches!(
        manager
            .forward(make("local/agents", &connected.handle, vec![]))
            .await,
        Err(Error::Invalid)
    ));
    *installation.write().await = "f".repeat(32);
    assert!(matches!(
        manager
            .connect(Connect {
                id: Some(connected.connection.id.clone()),
                name: "Remote".into(),
                origin,
                token: credential.to_string()
            })
            .await,
        Err(Error::Identity)
    ));
    assert!(matches!(
        manager
            .forward(make("repos", &connected.handle, vec![]))
            .await,
        Err(Error::Identity)
    ));
    manager.disconnect(&connected.handle).await.unwrap();
    assert!(matches!(
        manager
            .forward(make("repos", &connected.handle, vec![]))
            .await,
        Err(Error::Denied)
    ));
    server.abort();
}
