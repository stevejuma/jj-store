use super::*;
use axum::{
    Router,
    extract::ws::{Message, WebSocketUpgrade},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
};

#[tokio::test]
async fn gateway_freshness_uses_registered_origin_and_session_authentication() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    let expected_origin = origin.clone();
    let token = BearerToken::generate().unwrap();
    let expected_token = token.expose().to_string();
    let router = Router::new()
        .route("/v1/bootstrap", get(|| async {
            axum::Json(serde_json::json!({"version":1,"access_mode":"authenticated","installation_id":"c".repeat(32)}))
        }))
        .route("/v1/directory/profile", get(|| async {
            axum::Json(serde_json::json!({"principal":"d".repeat(32)}))
        }))
        .route("/v1/events/ws", get(move |headers: HeaderMap, ws: WebSocketUpgrade| {
            let origin = expected_origin.clone();
            let token = expected_token.clone();
            async move {
                if headers.get("origin").is_none_or(|v| v.as_bytes() != origin.as_bytes()) {
                    return StatusCode::FORBIDDEN.into_response();
                }
                assert!(!headers.contains_key("authorization"));
                assert!(!headers.contains_key("cookie"));
                assert!(!headers.contains_key("x-jj-store-protocol"));
                assert_eq!(headers["x-jj-store-target-installation"], "c".repeat(32));
                let mut response = ws.protocols(["jj-store-events-v1"]).on_upgrade(move |mut socket| async move {
                    let Some(Ok(Message::Text(frame))) = socket.recv().await else { panic!("authentication frame") };
                    let hello: serde_json::Value = serde_json::from_str(&frame).unwrap();
                    assert_eq!(hello["version"], 1);
                    assert_eq!(hello["token"], token);
                    assert_eq!(hello["after"], "retained-cursor");
                    socket.send(Message::Text("{\"type\":\"ready\"}".into())).await.unwrap();
                    let _ = socket.recv().await;
                }).into_response();
                response.headers_mut().insert("x-jj-store-installation", "c".repeat(32).parse().unwrap());
                response
            }
        }));
    let server = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
    let root = tempfile::tempdir().unwrap();
    let manager = Manager::open(root.path().into(), true).unwrap();
    let connected = manager
        .connect(Connect {
            id: None,
            name: "Remote".into(),
            origin,
            token: token.expose().to_string(),
        })
        .await
        .unwrap();
    let mut frames = manager
        .events(
            &connected.connection.id,
            &connected.handle,
            Some("retained-cursor".into()),
        )
        .await
        .unwrap();
    let frame = tokio::time::timeout(std::time::Duration::from_secs(5), frames.recv())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(frame, "{\"type\":\"ready\"}");
    manager.disconnect(&connected.handle).await.unwrap();
    assert!(
        tokio::time::timeout(std::time::Duration::from_secs(5), frames.recv())
            .await
            .unwrap()
            .is_none()
    );
    server.abort();
}
