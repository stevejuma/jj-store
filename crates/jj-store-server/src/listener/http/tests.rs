use super::*;
use axum::{
    extract::ws::{Message, WebSocketUpgrade},
    routing::get,
};
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message as Wire};

#[tokio::test]
async fn production_listener_keeps_upgraded_sockets_alive_serves_http2_and_closes_on_shutdown() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let router = axum::Router::new()
        .route(
            "/ws",
            get(|ws: WebSocketUpgrade| async {
                ws.on_upgrade(|mut socket| async move {
                    while let Some(Ok(Message::Text(text))) = socket.recv().await {
                        if socket.send(Message::Text(text)).await.is_err() {
                            break;
                        }
                    }
                })
            }),
        )
        .route("/http", get(|| async { "ready" }));
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(serve(
        incoming(listener, None, 16, Duration::from_secs(30)),
        router,
        4,
        Duration::from_secs(1),
        async {
            let _ = stopped.await;
        },
    ));
    let (mut socket, _) = connect_async(format!("ws://{address}/ws")).await.unwrap();
    for message in ["first", "still alive"] {
        socket.send(Wire::Text(message.into())).await.unwrap();
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(2), socket.next())
                .await
                .unwrap()
                .unwrap()
                .unwrap()
                .to_text()
                .unwrap(),
            message
        );
    }
    let url = format!("http://{address}/http");
    for client in [
        reqwest::Client::new(),
        reqwest::Client::builder()
            .http2_prior_knowledge()
            .build()
            .unwrap(),
    ] {
        assert_eq!(
            client.get(&url).send().await.unwrap().text().await.unwrap(),
            "ready"
        );
    }
    stop.send(()).unwrap();
    tokio::time::timeout(Duration::from_secs(2), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let closed = tokio::time::timeout(Duration::from_secs(2), socket.next())
        .await
        .unwrap();
    assert!(closed.is_none() || matches!(closed, Some(Err(_)) | Some(Ok(Wire::Close(_)))));
}
