//! Bounded provider HTTP. Error messages never contain response bodies or tokens.
use crate::*;
use futures::StreamExt;
use reqwest::Method;
use serde_json::Value;
use zeroize::Zeroizing;

#[async_trait::async_trait(?Send)]
pub trait Transport: Send + Sync {
    async fn request(&self, method: &str, path: &str, body: Option<Value>) -> Result<Value>;
}
pub struct Client {
    transport: Option<std::sync::Arc<dyn Transport>>,
    pub connection: Connection,
    http: reqwest::Client,
    token: Zeroizing<String>,
}
impl Client {
    pub fn new(connection: Connection, token: String) -> Result<Self> {
        connection.validate()?;
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::none())
            .user_agent("jj-store-forge/1")
            .build()
            .map_err(|_| Error::State)?;
        Ok(Self {
            connection,
            http,
            token: Zeroizing::new(token),
            transport: None,
        })
    }
    pub fn with_transport(
        connection: Connection,
        transport: std::sync::Arc<dyn Transport>,
    ) -> Result<Self> {
        let mut client = Self::new(connection, String::new())?;
        client.transport = Some(transport);
        Ok(client)
    }
    pub async fn git_username(&self) -> Result<String> {
        match self.connection.provider {
            Provider::Github => Ok("x-access-token".into()),
            Provider::Gitlab => Ok("oauth2".into()),
            Provider::Forgejo => {
                let v = self.request(Method::GET, "user", None).await?;
                let user = text(&v, "login");
                if user.is_empty() || user.contains(':') {
                    return Err(Error::Denied);
                }
                Ok(user)
            }
        }
    }
    pub async fn capabilities_checked(&self) -> Result<Capabilities> {
        let mut caps = self.capabilities();
        if self.connection.provider == Provider::Forgejo {
            let v = self.request(Method::GET, "version", None).await?;
            let major = text(&v, "version")
                .trim_start_matches('v')
                .split('.')
                .next()
                .and_then(|n| n.parse::<u32>().ok())
                .unwrap_or_default();
            caps.logs = major >= 16;
            caps.cancel = major >= 16;
        }
        Ok(caps)
    }
    pub fn capabilities(&self) -> Capabilities {
        Capabilities {
            edit: true,
            draft: true,
            merge: true,
            auto_merge: false,
            checks: true,
            logs: true,
            rerun: self.connection.provider != Provider::Forgejo,
            rerun_all: self.connection.provider == Provider::Github,
            cancel: true,
            merge_methods: if self.connection.provider == Provider::Gitlab {
                vec!["merge".into(), "squash".into()]
            } else {
                vec!["merge".into(), "squash".into(), "rebase".into()]
            },
            native_stacks: self.connection.provider == Provider::Github
                && self.connection.native_stacks
                && !self.connection.fork(),
        }
    }
    pub(crate) fn project(&self) -> String {
        if self.connection.provider == Provider::Gitlab {
            format!("projects/{}", encode(&self.connection.repository))
        } else {
            format!("repos/{}", self.connection.repository)
        }
    }
    pub(crate) async fn request(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<Value> {
        if let Some(port) = &self.transport {
            return port.request(method.as_str(), path, body).await;
        }
        let mutating = method != Method::GET;
        let bytes = self.bytes(method, path, body).await?;
        if bytes.is_empty() {
            Ok(Value::Null)
        } else {
            serde_json::from_slice(&bytes).map_err(|_| {
                if mutating {
                    Error::Unknown
                } else {
                    Error::Invalid
                }
            })
        }
    }
    pub(crate) async fn bytes(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<Vec<u8>> {
        if path.starts_with('/') || path.contains("..") || path.contains("://") {
            return Err(Error::Invalid);
        }
        let base = self.connection.api_url.trim_end_matches('/');
        let url = if self.connection.provider == Provider::Github
            && path == "graphql"
            && base.ends_with("/api/v3")
        {
            format!("{}/graphql", base.trim_end_matches("/v3"))
        } else {
            format!("{base}/{path}")
        };
        let mut r = self
            .http
            .request(method.clone(), url)
            .header("Accept", "application/json");
        if !self.token.is_empty() {
            r = match self.connection.provider {
                Provider::Gitlab => r.header("PRIVATE-TOKEN", self.token.as_str()),
                Provider::Forgejo => {
                    r.header("Authorization", format!("token {}", self.token.as_str()))
                }
                Provider::Github => r
                    .bearer_auth(self.token.as_str())
                    .header("X-GitHub-Api-Version", "2026-03-10"),
            };
        }
        if let Some(v) = body {
            r = r.json(&v);
        }
        let mut response = r.send().await.map_err(|_| Error::Unknown)?;
        if response.status().is_redirection() && method == Method::GET && path.ends_with("/logs") {
            let location = response
                .headers()
                .get(reqwest::header::LOCATION)
                .and_then(|v| v.to_str().ok())
                .ok_or(Error::Invalid)?;
            let destination = url::Url::parse(location).map_err(|_| Error::Invalid)?;
            if destination.scheme() != "https"
                || !destination.username().is_empty()
                || destination.password().is_some()
            {
                return Err(Error::Invalid);
            }
            response = self
                .http
                .get(destination)
                .send()
                .await
                .map_err(|_| Error::Unknown)?;
        }
        let code = response.status().as_u16();
        if code == 403
            && (response
                .headers()
                .get("x-ratelimit-remaining")
                .is_some_and(|v| v == "0")
                || response.headers().contains_key("retry-after"))
        {
            return Err(Error::RateLimited);
        }
        if code == 401 || code == 403 {
            return Err(Error::Denied);
        }
        if code == 429 {
            return Err(Error::RateLimited);
        }
        if !(200..300).contains(&code) {
            return Err(if code >= 500 {
                Error::Unknown
            } else {
                Error::Rejected(code)
            });
        }
        let mut out = vec![];
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|_| Error::Unknown)?;
            if out.len() + chunk.len() > 2 * 1024 * 1024 {
                return Err(if method == Method::GET {
                    Error::Limit
                } else {
                    Error::Unknown
                });
            }
            out.extend_from_slice(&chunk);
        }
        Ok(out)
    }
    pub(crate) async fn pages(&self, path: &str) -> Result<Vec<Value>> {
        let mut values = vec![];
        for page in 1..=10 {
            let separator = if path.contains('?') { '&' } else { '?' };
            let v = self
                .request(
                    Method::GET,
                    &format!("{path}{separator}per_page=100&limit=100&page={page}"),
                    None,
                )
                .await?;
            let list = v
                .as_array()
                .or_else(|| v.get("workflow_runs").and_then(Value::as_array))
                .or_else(|| v.get("check_runs").and_then(Value::as_array))
                .or_else(|| v.get("jobs").and_then(Value::as_array))
                .ok_or(Error::Invalid)?;
            values.extend(list.iter().cloned());
            if list.len() < 100 {
                return Ok(values);
            }
        }
        Err(Error::Limit)
    }
}
pub(crate) fn encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
pub(crate) fn text(v: &Value, key: &str) -> String {
    v.get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}
pub(crate) fn number(v: &Value, key: &str) -> u64 {
    v.get(key).and_then(Value::as_u64).unwrap_or_default()
}
