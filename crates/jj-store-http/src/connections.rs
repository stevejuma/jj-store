//! LocalTrusted-only connection registry and registered public API forwarding.
use crate::*;
use axum::{
    Json,
    extract::{DefaultBodyLimit, OriginalUri},
    routing::{any, get, post},
};
use jj_store_service::connections as c;
fn manager(state: &State) -> Result<&Arc<dyn c::Connections>> {
    if !matches!(
        state.config.access,
        Access::LocalTrusted {
            permission: Permission::Admin,
            ..
        }
    ) {
        return Err(Error::denied());
    }
    state
        .config
        .local_connections
        .as_ref()
        .ok_or_else(Error::unavailable)
}
fn failure(error: c::Error) -> Error {
    let (status, code) = match error {
        c::Error::Denied => (StatusCode::FORBIDDEN, "connection_denied"),
        c::Error::Invalid => (StatusCode::BAD_REQUEST, "invalid_connection"),
        c::Error::Identity => (StatusCode::CONFLICT, "connection_identity_changed"),
        c::Error::Limit => (StatusCode::TOO_MANY_REQUESTS, "connection_limit"),
        c::Error::Unavailable => (StatusCode::SERVICE_UNAVAILABLE, "connection_unavailable"),
    };
    Error::new(status, code)
}
pub(super) fn router() -> Router<Arc<State>> {
    Router::new()
        .route("/local/connections", get(list).post(edit))
        .route("/local/connections/connect", post(connect))
        .route("/local/connections/disconnect", post(disconnect))
        .route("/local/connections/{id}/api/{*path}", any(forward))
        .route("/local/connections/{id}/events", get(events))
        .layer(DefaultBodyLimit::max(32 * 1024 * 1024))
}
pub(super) fn document(document: &mut contract::Document) {
    document.local_endpoint::<routes::Empty, Vec<c::Connection>>(
        "get",
        "/local/connections",
        "local_connections",
    );
    document.local_endpoint::<routes::Empty, Vec<c::Connection>>(
        "post",
        "/local/connections",
        "local_connections_edit",
    );
    document.body::<c::Edit>("/v1/local/connections", "post");
    document.local_endpoint::<routes::Empty, c::Connected>(
        "post",
        "/local/connections/connect",
        "local_connections_connect",
    );
    document.body::<c::Connect>("/v1/local/connections/connect", "post");
    document.local_endpoint::<routes::Empty, serde_json::Value>(
        "post",
        "/local/connections/disconnect",
        "local_connections_disconnect",
    );
    document.body::<c::Disconnect>("/v1/local/connections/disconnect", "post");
}
async fn list(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(query)?;
    state.json(&manager(&state)?.list().await.map_err(failure)?)
}
async fn connect(
    ExtractState(state): ExtractState<Arc<State>>,
    Json(input): Json<c::Connect>,
) -> Result<Response> {
    state.json(&manager(&state)?.connect(input).await.map_err(failure)?)
}
async fn edit(
    ExtractState(state): ExtractState<Arc<State>>,
    Json(input): Json<c::Edit>,
) -> Result<Response> {
    state.json(&manager(&state)?.edit(input).await.map_err(failure)?)
}
async fn disconnect(
    ExtractState(state): ExtractState<Arc<State>>,
    Json(input): Json<c::Disconnect>,
) -> Result<Response> {
    manager(&state)?
        .disconnect(&input.handle)
        .await
        .map_err(failure)?;
    state.json(&serde_json::json!({"disconnected":true}))
}
async fn forward(
    ExtractState(state): ExtractState<Arc<State>>,
    Path((id, _path)): Path<(String, String)>,
    OriginalUri(uri): OriginalUri,
    request: axum::extract::Request,
) -> Result<Response> {
    let manager = manager(&state)?;
    let handle = request.headers().get_all("x-jj-store-connection-session");
    if handle.iter().count() != 1 {
        return Err(Error::denied());
    }
    let handle = handle
        .iter()
        .next()
        .and_then(|v| v.to_str().ok())
        .ok_or_else(Error::denied)?
        .to_owned();
    let prefix = format!("/v1/local/connections/{id}/api/");
    let raw = uri.path_and_query().ok_or_else(Error::invalid)?.as_str();
    let path = raw
        .strip_prefix(&prefix)
        .ok_or_else(Error::invalid)?
        .to_owned();
    let headers = request
        .headers()
        .iter()
        .filter_map(|(name, value)| {
            value
                .to_str()
                .ok()
                .map(|v| (name.as_str().into(), v.into()))
        })
        .collect();
    let method = request.method().as_str().to_owned();
    let body = axum::body::to_bytes(request.into_body(), 32 * 1024 * 1024)
        .await
        .map_err(|_| Error::limit())?
        .to_vec();
    let upstream = manager
        .forward(c::Forward {
            connection: id,
            handle,
            method,
            path,
            headers,
            body,
        })
        .await
        .map_err(failure)?;
    let mut response = Response::builder().status(upstream.status);
    for (name, value) in upstream.headers {
        response = response.header(name, value);
    }
    let stream = futures::stream::unfold(upstream.chunks, |mut receiver| async move {
        receiver
            .recv()
            .await
            .map(|value| (value.map_err(std::io::Error::other), receiver))
    });
    response
        .body(Body::from_stream(stream))
        .map_err(|_| Error::unavailable())
}
async fn events(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(id): Path<String>,
    upgrade: axum::extract::ws::WebSocketUpgrade,
) -> Result<Response> {
    let manager = manager(&state)?.clone();
    Ok(upgrade
        .protocols(["jj-store-events-v1"])
        .max_message_size(1024 * 1024)
        .on_upgrade(move |mut socket| async move {
            use axum::extract::ws::{CloseFrame, Message};
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Hello {
                version: u32,
                token: String,
                after: Option<String>,
            }
            let first = tokio::time::timeout(Duration::from_secs(10), socket.recv()).await;
            let hello = match first {
                Ok(Some(Ok(Message::Text(value)))) if value.len() <= 4096 => {
                    serde_json::from_str::<Hello>(&value).ok()
                }
                _ => None,
            };
            let Some(hello) = hello.filter(|h| h.version == 1) else {
                let _ = socket
                    .send(Message::Close(Some(CloseFrame {
                        code: 4400,
                        reason: "invalid connection handshake".into(),
                    })))
                    .await;
                return;
            };
            let mut frames = match manager.events(&id, &hello.token, hello.after).await {
                Ok(value) => value,
                Err(_) => {
                    let _ = socket
                        .send(Message::Close(Some(CloseFrame {
                            code: 4403,
                            reason: "connection denied".into(),
                        })))
                        .await;
                    return;
                }
            };
            loop {
                let frame = tokio::select! {frame=frames.recv()=>frame, _=socket.recv()=>break};
                match frame {
                    Some(Ok(text)) => {
                        if socket.send(Message::Text(text.into())).await.is_err() {
                            break;
                        }
                    }
                    Some(Err(_)) => {
                        let _ = socket
                            .send(Message::Close(Some(CloseFrame {
                                code: 4403,
                                reason: "connection unavailable".into(),
                            })))
                            .await;
                        return;
                    }
                    None => break,
                }
            }
            let _ = socket.send(Message::Close(None)).await;
        })
        .into_response())
}
