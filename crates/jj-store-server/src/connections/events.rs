use super::*;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::{Message, client::IntoClientRequest};
impl Manager {
    pub(super) async fn follow(
        &self,
        id: &str,
        handle: &str,
        after: Option<String>,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<String>>> {
        if after.as_ref().is_some_and(|v| v.len() > 256) {
            return Err(Error::Invalid);
        }
        let (connection, session) = {
            let state = self.0.state.lock().await;
            let connection = state
                .saved
                .connections
                .get(id)
                .cloned()
                .ok_or(Error::Denied)?;
            let session = state
                .sessions
                .get(handle)
                .cloned()
                .filter(|s| s.connection == id && !*s.closed.borrow())
                .ok_or(Error::Denied)?;
            (connection, session)
        };
        let permit = self
            .0
            .slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| Error::Limit)?;
        let mut url = url::Url::parse(&format!("{}/v1/events/ws", connection.origin))
            .map_err(|_| Error::Invalid)?;
        url.set_scheme(if url.scheme() == "https" { "wss" } else { "ws" })
            .map_err(|_| Error::Invalid)?;
        let mut request = url
            .as_str()
            .into_client_request()
            .map_err(|_| Error::Invalid)?;
        // Use the registered remote authority, never the local browser's Origin.
        request.headers_mut().insert(
            "origin",
            connection.origin.parse().map_err(|_| Error::Invalid)?,
        );
        request.headers_mut().insert(
            "sec-websocket-protocol",
            "jj-store-events-v1".parse().map_err(|_| Error::Invalid)?,
        );
        request.headers_mut().insert(
            "x-jj-store-target-installation",
            connection
                .installation
                .parse()
                .map_err(|_| Error::Invalid)?,
        );
        let (mut socket, handshake) = tokio::time::timeout(
            std::time::Duration::from_secs(15),
            tokio_tungstenite::connect_async_with_config(
                request,
                Some(
                    tokio_tungstenite::tungstenite::protocol::WebSocketConfig::default()
                        .max_message_size(Some(1024 * 1024))
                        .max_frame_size(Some(1024 * 1024)),
                ),
                false,
            ),
        )
        .await
        .map_err(|_| Error::Unavailable)?
        .map_err(|_| Error::Unavailable)?;
        if handshake
            .headers()
            .get("x-jj-store-installation")
            .is_none_or(|v| v.as_bytes() != connection.installation.as_bytes())
        {
            session.closed.send_replace(true);
            return Err(Error::Identity);
        }
        if handshake
            .headers()
            .get("sec-websocket-protocol")
            .is_none_or(|v| v != "jj-store-events-v1")
        {
            return Err(Error::Invalid);
        }
        let authentication =
            serde_json::json!({"version":1,"token":session.token.expose().as_str(),"after":after});
        tokio::time::timeout(
            std::time::Duration::from_secs(10),
            socket.send(Message::Text(authentication.to_string().into())),
        )
        .await
        .map_err(|_| Error::Unavailable)?
        .map_err(|_| Error::Unavailable)?;
        let (sender, receiver) = tokio::sync::mpsc::channel(4);
        let mut closed = session.closed.subscribe();
        tokio::spawn(async move {
            let _permit = permit;
            let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(300);
            loop {
                let frame = tokio::select! {v=socket.next()=>v,_=closed.changed()=>break,_=sender.closed()=>break,_=tokio::time::sleep_until(deadline)=>break};
                match frame {
                    Some(Ok(Message::Text(text))) if text.len() <= 1024 * 1024 => {
                        tokio::select! {_ = closed.changed()=>break,_=tokio::time::sleep_until(deadline)=>break,result=sender.send(Ok(text.to_string()))=>if result.is_err(){break}}
                    }
                    Some(Ok(Message::Ping(bytes))) => {
                        if !matches!(
                            tokio::time::timeout(
                                std::time::Duration::from_secs(5),
                                socket.send(Message::Pong(bytes))
                            )
                            .await,
                            Ok(Ok(()))
                        ) {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(frame))) => {
                        if frame.is_some_and(|f| u16::from(f.code) == 4403) {
                            let _ = sender.try_send(Err(Error::Denied));
                        }
                        break;
                    }
                    None => break,
                    _ => {
                        let _ = sender.try_send(Err(Error::Unavailable));
                        break;
                    }
                }
            }
            let _ =
                tokio::time::timeout(std::time::Duration::from_secs(2), socket.close(None)).await;
        });
        Ok(receiver)
    }
}
