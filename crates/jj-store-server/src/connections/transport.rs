use super::*;
use futures::StreamExt;
const MAX_BODY: usize = 32 * 1024 * 1024;
fn allowed(path: &str, method: &str) -> bool {
    if !matches!(method, "GET" | "POST") || path.len() > 16384 || path.contains(['\\', '#']) {
        return false;
    }
    let route = path.split('?').next().unwrap_or("");
    if route.contains('%') {
        return false;
    }
    let parts = route.split('/').collect::<Vec<_>>();
    if parts
        .iter()
        .any(|p| p.is_empty() || *p == "." || *p == "..")
    {
        return false;
    }
    match parts.first().copied() {
        Some("bootstrap" | "capabilities" | "repositories" | "events") => method == "GET",
        Some("repos") => {
            (parts.len() == 1 && method == "GET") || parts.get(1).is_some_and(|id| valid_id(id))
        }
        Some("directory") => true,
        Some("operator") => {
            matches!(parts.as_slice(), ["operator", "access"] if method == "GET")
                || matches!(parts.as_slice(), ["operator", "agents"])
        }
        Some("agents") => matches!(
            parts.get(1).copied(),
            None | Some(
                "chat"
                    | "command"
                    | "outcome"
                    | "events"
                    | "stream"
                    | "targets"
                    | "workspace"
                    | "feedback"
            )
        ),
        _ => false,
    }
}
impl Manager {
    pub(super) async fn probe(
        &self,
        origin: &str,
        path: &str,
        token: Option<&BearerToken>,
    ) -> Result<serde_json::Value> {
        let mut request = self
            .0
            .client
            .get(format!("{origin}/v1/{path}"))
            .header("x-jj-store-protocol", "1")
            .timeout(std::time::Duration::from_secs(30));
        if let Some(token) = token {
            request = request.bearer_auth(token.expose().as_str());
        }
        let mut response = request.send().await.map_err(|_| Error::Unavailable)?;
        if response.status().is_redirection() {
            return Err(Error::Denied);
        }
        if !response.status().is_success() {
            return Err(
                if response.status().as_u16() == 401 || response.status().as_u16() == 403 {
                    Error::Denied
                } else {
                    Error::Unavailable
                },
            );
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| Error::Unavailable)? {
            if bytes.len() + chunk.len() > 1024 * 1024 {
                return Err(Error::Limit);
            }
            bytes.extend_from_slice(&chunk);
        }
        serde_json::from_slice(&bytes).map_err(|_| Error::Invalid)
    }
    pub(super) async fn transfer(&self, request: Forward) -> Result<Response> {
        if !allowed(&request.path, &request.method) || request.body.len() > MAX_BODY {
            return Err(Error::Invalid);
        }
        let (connection, session) = {
            let state = self.0.state.lock().await;
            let connection = state
                .saved
                .connections
                .get(&request.connection)
                .cloned()
                .ok_or(Error::Denied)?;
            let session = state
                .sessions
                .get(&request.handle)
                .cloned()
                .filter(|s| s.connection == request.connection && !*s.closed.borrow())
                .ok_or(Error::Denied)?;
            (connection, session)
        };
        let permit = self
            .0
            .slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| Error::Limit)?;
        let mut closed = session.closed.subscribe();
        let mut upstream = self
            .0
            .client
            .request(
                reqwest::Method::from_bytes(request.method.as_bytes())
                    .map_err(|_| Error::Invalid)?,
                format!("{}/v1/{}", connection.origin, request.path),
            )
            .header("x-jj-store-protocol", "1")
            .header("x-jj-store-target-installation", &connection.installation)
            .bearer_auth(session.token.expose().as_str())
            .body(request.body);
        for (name, value) in request.headers {
            if matches!(
                name.as_str(),
                "accept" | "content-type" | "range" | "last-event-id"
            ) {
                upstream = upstream.header(name, value);
            }
        }
        let response = tokio::select! {r=tokio::time::timeout(std::time::Duration::from_secs(35),upstream.send())=>r.map_err(|_|Error::Unavailable)?.map_err(|_|Error::Unavailable)?, _=closed.changed()=>return Err(Error::Denied)};
        if response.status().is_redirection() {
            return Err(Error::Denied);
        }
        if response
            .headers()
            .get("x-jj-store-installation")
            .is_none_or(|id| id.as_bytes() != connection.installation.as_bytes())
        {
            session.closed.send_replace(true);
            return Err(Error::Identity);
        }
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .filter(|(name, _)| {
                matches!(
                    name.as_str(),
                    "content-type"
                        | "content-range"
                        | "accept-ranges"
                        | "content-length"
                        | "x-jj-store-operation"
                        | "x-jj-store-repository"
                )
            })
            .filter_map(|(name, value)| {
                value
                    .to_str()
                    .ok()
                    .map(|v| (name.as_str().into(), v.into()))
            })
            .collect();
        let streaming = response
            .headers()
            .get("content-type")
            .is_some_and(|v| v.as_bytes().starts_with(b"text/event-stream"));
        let mut source = response.bytes_stream();
        let (sender, chunks) = tokio::sync::mpsc::channel(4);
        tokio::spawn(async move {
            let _permit = permit;
            let deadline = tokio::time::Instant::now()
                + std::time::Duration::from_secs(if streaming { 300 } else { 35 });
            let mut total = 0;
            loop {
                let next = tokio::select! {v=source.next()=>v, _=closed.changed()=>break, _=sender.closed()=>break, _=tokio::time::sleep_until(deadline)=>break};
                let Some(next) = next else { break };
                let bytes = match next {
                    Ok(v) => v,
                    Err(_) => {
                        let _ = sender.try_send(Err(Error::Unavailable));
                        break;
                    }
                };
                total += bytes.len();
                if !streaming && total > MAX_BODY {
                    let _ = sender.try_send(Err(Error::Limit));
                    break;
                }
                tokio::select! {_=closed.changed()=>break, _=tokio::time::sleep_until(deadline)=>break, result=sender.send(Ok(bytes.to_vec()))=>if result.is_err(){break}}
            }
        });
        Ok(Response {
            status,
            headers,
            chunks,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gateway_paths_are_fixed_public_apis() {
        assert!(allowed(
            "repos/0123456789abcdef0123456789abcdef/files?at_op=abc&path=a%20b",
            "GET"
        ));
        for path in [
            "local/agents",
            "../bootstrap",
            "repos//files",
            "https://other/v1/repos",
            "agents/secrets",
        ] {
            assert!(!allowed(path, "POST"), "{path}");
        }
        assert!(allowed(
            "repos/0123456789abcdef0123456789abcdef/files?at_op=abc",
            "GET"
        ));
        assert!(allowed("agents/chat", "POST"));
        assert!(allowed("operator/access", "GET"));
        assert!(!allowed("operator/access", "POST"));
        assert!(allowed("operator/agents", "POST"));
        assert!(!allowed("operator/unknown", "POST"));
        assert!(allowed("directory/identities", "POST"));
    }
}
