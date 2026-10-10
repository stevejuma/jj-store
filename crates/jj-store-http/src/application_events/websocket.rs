//! One-way freshness delivery; the only client message is bounded authentication.
use super::*;
use axum::extract::ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade};
use futures::{SinkExt, StreamExt};

pub(crate) const PROTOCOL: &str = "jj-store-events-v1";
const HANDSHAKE_LIMIT: usize = 8192;
const IO_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Hello {
    version: u32,
    token: Option<String>,
    after: Option<String>,
}

// These checks supplement the common authority, size, body and method checks.
pub(crate) fn admission(request: &axum::extract::Request) -> Result<()> {
    let headers = request.headers();
    if request.method() != axum::http::Method::GET
        || request.uri().query().is_some()
        || headers.get_all(header::ORIGIN).iter().count() != 1
        || headers
            .get_all(header::SEC_WEBSOCKET_PROTOCOL)
            .iter()
            .count()
            != 1
        || headers
            .get(header::SEC_WEBSOCKET_PROTOCOL)
            .is_none_or(|v| v != PROTOCOL)
        || headers.contains_key(header::AUTHORIZATION)
        || headers.contains_key(header::COOKIE)
        || headers.contains_key(PROTOCOL_HEADER)
    {
        return Err(Error::denied());
    }
    Ok(())
}

pub(super) async fn upgrade(
    ExtractState(state): ExtractState<Arc<State>>,
    ws: WebSocketUpgrade,
) -> Result<Response> {
    let permit = state
        .event_slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| Error::busy())?;
    Ok(ws
        .protocols([PROTOCOL])
        .max_message_size(HANDSHAKE_LIMIT)
        .max_frame_size(HANDSHAKE_LIMIT)
        .on_upgrade(move |mut socket| async move {
            // Unlike HTTP response-body admission, this permit owns the upgraded socket.
            let _permit = permit;
            let result =
                tokio::time::timeout(state.config.event_lifetime, run(&state, &mut socket)).await;
            let code = match result {
                Ok(Ok(())) | Err(_) => 1000,
                Ok(Err(error)) => match error.into_response().status() {
                    StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => 4403,
                    StatusCode::BAD_REQUEST | StatusCode::PAYLOAD_TOO_LARGE => 4400,
                    _ => 1013,
                },
            };
            let _ = tokio::time::timeout(
                IO_TIMEOUT,
                socket.send(Message::Close(Some(CloseFrame {
                    code,
                    reason: "freshness closed".into(),
                }))),
            )
            .await;
        })
        .into_response())
}

async fn run(state: &Arc<State>, socket: &mut WebSocket) -> Result<()> {
    let mut subscription = tokio::time::timeout(IO_TIMEOUT, async {
        let Some(Ok(Message::Text(text))) = socket.recv().await else {
            return Err(Error::invalid());
        };
        let hello: Hello = serde_json::from_str(&text).map_err(|_| Error::invalid())?;
        if hello.version != 1
            || hello.token.as_ref().is_some_and(|t| t.len() > 4096)
            || hello.after.as_ref().is_some_and(|c| c.len() > 128)
            || (matches!(state.config.access, Access::LocalTrusted { .. }) && hello.token.is_some())
        {
            return Err(Error::invalid());
        }
        let mut headers = HeaderMap::new();
        if let Some(token) = hello.token {
            let mut value = format!("Bearer {token}")
                .parse::<axum::http::HeaderValue>()
                .map_err(|_| Error::invalid())?;
            value.set_sensitive(true);
            headers.insert(header::AUTHORIZATION, value);
        }
        Subscription::open(state.clone(), &headers, hello.after.as_deref()).await
    })
    .await
    .map_err(|_| Error::timeout())??;
    let (mut sender, mut receiver) = socket.split();
    loop {
        let delivery = {
            // Preparation may advance replay state before awaiting authorization.
            // Ping/Pong must not cancel it and silently discard a queued event.
            let delivery = subscription.next();
            tokio::pin!(delivery);
            loop {
                tokio::select! {
                    incoming = receiver.next() => match incoming {
                        None | Some(Ok(Message::Close(_))) => return Ok(()),
                        Some(Ok(Message::Ping(_) | Message::Pong(_))) => continue,
                        _ => return Err(Error::invalid()),
                    },
                    delivery = &mut delivery => break delivery?,
                }
            }
        };
        let text = match delivery {
            Some(Delivery::Heartbeat) => r#"{"version":1,"type":"heartbeat"}"#.into(),
            Some(Delivery::Event(frame)) => {
                serde_json::to_string(&frame).map_err(|_| Error::unavailable())?
            }
            None => return Ok(()),
        };
        tokio::time::timeout(IO_TIMEOUT, sender.send(Message::Text(text.into())))
            .await
            .map_err(|_| Error::timeout())?
            .map_err(|_| Error::unavailable())?;
    }
}

pub(super) fn document(document: &mut contract::Document) {
    document.application_websocket::<Hello, subscription::Frame>();
}

#[cfg(test)]
mod tests;
