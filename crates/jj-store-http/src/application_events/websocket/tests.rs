use super::*;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message as Wire, client::IntoClientRequest},
};
use tower::ServiceExt;

async fn server(
    access: Access,
    lifetime: Duration,
    slots: usize,
) -> (
    HttpApi,
    String,
    tokio::task::JoinHandle<()>,
    tempfile::TempDir,
) {
    let root = tempfile::tempdir().unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    let api = HttpApi::with_search(
        None,
        root.path().into(),
        Config {
            origin: origin.clone(),
            access,
            registry: Some(Arc::new(super::super::tests::Registry::default())),
            event_concurrency: slots,
            event_lifetime: lifetime,
            ..Default::default()
        },
        BTreeMap::new(),
    )
    .unwrap();
    let app = api.router();
    let task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    (api, origin, task, root)
}
fn local() -> Access {
    Access::LocalTrusted {
        principal: m::PrincipalId::new([1; 16]),
        permission: Permission::Admin,
    }
}
fn request(origin: &str) -> axum::http::Request<()> {
    let mut req = format!("{}/v1/events/ws", origin.replace("http:", "ws:"))
        .into_client_request()
        .unwrap();
    req.headers_mut()
        .insert(header::ORIGIN, origin.parse().unwrap());
    req.headers_mut()
        .insert(header::SEC_WEBSOCKET_PROTOCOL, PROTOCOL.parse().unwrap());
    req.headers_mut()
        .insert("sec-fetch-site", "same-origin".parse().unwrap());
    req.headers_mut()
        .insert("sec-fetch-mode", "websocket".parse().unwrap());
    req.headers_mut()
        .insert("sec-fetch-dest", "empty".parse().unwrap());
    req
}
type Client =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
async fn hello(origin: &str, after: Option<&str>) -> Client {
    let (mut socket, response) = connect_async(request(origin)).await.unwrap();
    assert_eq!(response.headers()[header::SEC_WEBSOCKET_PROTOCOL], PROTOCOL);
    socket
        .send(Wire::Text(
            serde_json::json!({"version":1,"token":null,"after":after})
                .to_string()
                .into(),
        ))
        .await
        .unwrap();
    socket
}
async fn read(socket: &mut Client) -> Wire {
    tokio::time::timeout(Duration::from_secs(2), socket.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap()
}
async fn json(socket: &mut Client) -> serde_json::Value {
    serde_json::from_str(read(socket).await.to_text().unwrap()).unwrap()
}
#[tokio::test]
async fn websocket_replays_resets_and_does_not_compete_with_http_admission() {
    let (api, origin, task, _root) = server(local(), Duration::from_secs(300), 16).await;
    let mut clients = Vec::new();
    for _ in 0..8 {
        let mut client = hello(&origin, None).await;
        assert_eq!(json(&mut client).await["event"]["type"], "ready");
        clients.push(client);
    }
    let mut first = clients.pop().unwrap();
    first
        .send(Wire::Ping(b"liveness".to_vec().into()))
        .await
        .unwrap();
    assert!(matches!(read(&mut first).await, Wire::Pong(_)));
    api.0.freshness.changed(Change {
        repository: None,
        domain: Domain::Configuration,
        witness: "one".into(),
        identity: None,
        audience: None,
    });
    let changed = json(&mut first).await;
    assert_eq!(changed["event"]["change"]["witness"], "one");
    let cursor = changed["id"].as_str().unwrap();
    api.0.freshness.changed(Change {
        repository: None,
        domain: Domain::Configuration,
        witness: "two".into(),
        identity: None,
        audience: None,
    });
    let mut replay = hello(&origin, Some(cursor)).await;
    assert_eq!(json(&mut replay).await["event"]["change"]["witness"], "two");
    assert_eq!(json(&mut replay).await["event"]["type"], "ready");
    let stale = format!("jse1:{}:{}:0", "00".repeat(16), "11".repeat(16));
    let mut reset = hello(&origin, Some(&stale)).await;
    assert_eq!(json(&mut reset).await["event"]["type"], "reset");
    assert!(matches!(read(&mut reset).await, Wire::Close(Some(c)) if c.code == 1000.into()));
    let response = api
        .router()
        .oneshot(
            axum::http::Request::builder()
                .uri("/v1/bootstrap")
                .header("host", origin.trim_start_matches("http://"))
                .header(PROTOCOL_HEADER, "1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    drop(clients);
    drop(first);
    drop(replay);
    drop(reset);
    tokio::time::sleep(Duration::from_millis(30)).await;
    assert_eq!(api.0.event_slots.available_permits(), 16);
    task.abort();
}
#[tokio::test]
async fn websocket_rejects_cross_origin_headers_queries_and_commands() {
    let (_api, origin, task, _root) = server(local(), Duration::from_secs(300), 16).await;
    for (name, value) in [
        ("origin", "http://evil.invalid"),
        ("origin", "null"),
        ("sec-fetch-site", "cross-site"),
        ("sec-fetch-mode", "cors"),
        ("sec-websocket-protocol", "other"),
        ("cookie", "ignored=1"),
        ("authorization", "Bearer forbidden"),
        (PROTOCOL_HEADER, "1"),
    ] {
        let mut req = request(&origin);
        req.headers_mut().insert(
            axum::http::HeaderName::from_bytes(name.as_bytes()).unwrap(),
            value.parse().unwrap(),
        );
        assert!(connect_async(req).await.is_err(), "{name}");
    }
    let mut req = request(&origin);
    req.headers_mut().remove(header::ORIGIN);
    assert!(connect_async(req).await.is_err());
    let mut req = request(&origin);
    *req.uri_mut() = format!("{}?token=no", req.uri()).parse().unwrap();
    assert!(connect_async(req).await.is_err());
    for payload in [
        r#"{"version":2,"token":null}"#,
        r#"{"version":1,"token":"no"}"#,
        r#"{"version":1,"unknown":true}"#,
        r#"{"version":1,"after":"bad"}"#,
    ] {
        let (mut socket, _) = connect_async(request(&origin)).await.unwrap();
        socket.send(Wire::Text(payload.into())).await.unwrap();
        assert!(matches!(read(&mut socket).await, Wire::Close(Some(c)) if c.code == 4400.into()));
    }
    let mut socket = hello(&origin, None).await;
    json(&mut socket).await;
    socket
        .send(Wire::Text(r#"{"command":"mutate"}"#.into()))
        .await
        .unwrap();
    assert!(matches!(read(&mut socket).await, Wire::Close(Some(c)) if c.code == 4400.into()));
    task.abort();
}
#[tokio::test]
async fn websocket_forged_bearer_receives_no_data_and_lifetime_releases_admission() {
    let (_, origin, task, _root) =
        server(Access::Authenticated, Duration::from_secs(300), 16).await;
    let (mut socket, _) = connect_async(request(&origin)).await.unwrap();
    let token = BearerToken::generate().unwrap();
    socket
        .send(Wire::Text(
            serde_json::json!({"version":1,"token":token.expose().as_str()})
                .to_string()
                .into(),
        ))
        .await
        .unwrap();
    assert!(matches!(read(&mut socket).await, Wire::Close(Some(c)) if c.code == 4403.into()));
    task.abort();
    let (api, origin, task, _root) = server(local(), Duration::from_millis(180), 1).await;
    let mut client = hello(&origin, None).await;
    json(&mut client).await;
    assert!(connect_async(request(&origin)).await.is_err());
    assert!(matches!(read(&mut client).await, Wire::Close(Some(c)) if c.code == 1000.into()));
    tokio::time::sleep(Duration::from_millis(30)).await;
    assert_eq!(api.0.event_slots.available_permits(), 1);
    task.abort();
}

#[tokio::test]
async fn websocket_silent_handshake_and_oversized_frames_release_their_slots() {
    let (api, origin, task, _root) = server(local(), Duration::from_secs(300), 1).await;
    let (mut silent, _) = connect_async(request(&origin)).await.unwrap();
    assert!(connect_async(request(&origin)).await.is_err());
    let close = tokio::time::timeout(Duration::from_secs(7), silent.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert!(matches!(close, Wire::Close(Some(c)) if c.code == 1013.into()));
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert_eq!(api.0.event_slots.available_permits(), 1);
    let (mut large, _) = connect_async(request(&origin)).await.unwrap();
    large
        .send(Wire::Text("x".repeat(HANDSHAKE_LIMIT + 1).into()))
        .await
        .unwrap();
    // The WebSocket parser enforces the frame bound before JSON or authorization.
    let _ = tokio::time::timeout(Duration::from_secs(2), large.next())
        .await
        .unwrap();
    drop(large);
    tokio::time::sleep(Duration::from_millis(30)).await;
    assert_eq!(api.0.event_slots.available_permits(), 1);
    task.abort();
}
