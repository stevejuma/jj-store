use jj_cli::command_error::{CommandError, user_error};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::time::Duration;

const MAX_RESPONSE: usize = 64 * 1024;
pub(super) struct Client {
    http: reqwest::Client,
    base: url::Url,
}
pub(super) fn failure(message: &'static str, command: bool) -> CommandError {
    user_error(message).hinted(if command {
        "Retain and retry the same command with its original request identity, generation and revision."
    } else {
        "Use 'jj agent capture --help' and the explicit local server URL."
    })
}
impl Client {
    pub fn new(value: &str) -> Result<Self, CommandError> {
        let invalid = || {
            failure(
                "capture requires a credential-free HTTP URL on a literal loopback address",
                false,
            )
        };
        if value.len() > 4096 {
            return Err(invalid());
        }
        let base = url::Url::parse(value).map_err(|_| invalid())?;
        let loopback = match base.host() {
            Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
            Some(url::Host::Ipv6(ip)) => ip.is_loopback(),
            _ => false,
        };
        if base.scheme() != "http"
            || !loopback
            || !base.username().is_empty()
            || base.password().is_some()
            || base.path() != "/"
            || base.query().is_some()
            || base.fragment().is_some()
        {
            return Err(invalid());
        }
        let http = reqwest::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(2))
            .build()
            .map_err(|_| failure("capture HTTP client unavailable", false))?;
        Ok(Self { http, base })
    }
    pub async fn request<T: DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        body: Option<Value>,
    ) -> Result<T, CommandError> {
        let command = body.is_some();
        let unknown = || {
            failure(
                if command {
                    "capture command outcome unknown"
                } else {
                    "capture status unavailable"
                },
                command,
            )
        };
        let url = self.base.join(path).map_err(|_| unknown())?;
        let method = reqwest::Method::from_bytes(method.as_bytes()).map_err(|_| unknown())?;
        let mut request = self
            .http
            .request(method, url)
            .header("x-jj-store-protocol", "1")
            .header("accept", "application/json");
        if let Some(body) = body {
            request = request.json(&body);
        }
        let mut response = request.send().await.map_err(|_| unknown())?;
        let status = response.status();
        if response
            .content_length()
            .is_some_and(|length| length > MAX_RESPONSE as u64)
            || response
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .is_none_or(|v| v.split(';').next().map(str::trim) != Some("application/json"))
        {
            return Err(unknown());
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| unknown())? {
            if chunk.len() > MAX_RESPONSE - bytes.len() {
                return Err(unknown());
            }
            bytes.extend_from_slice(&chunk);
        }
        if status != reqwest::StatusCode::OK {
            let value: Value = serde_json::from_slice(&bytes).map_err(|_| unknown())?;
            let message = match value["code"].as_str() {
                Some("capture_management_denied" | "access_denied") if status.as_u16() == 403 => {
                    "capture management denied"
                }
                Some("capture_disabled") if status.as_u16() == 409 => "capture is disabled",
                Some("capture_paused") if status.as_u16() == 409 => "capture is paused",
                Some("capture_policy_changed") if status.as_u16() == 409 => {
                    "capture policy changed; refresh status before issuing a new command"
                }
                Some("capture_source_selection_changed") if status.as_u16() == 409 => {
                    "capture source selection changed; list sources again"
                }
                Some("capture_control_intent_conflict") if status.as_u16() == 409 => {
                    "capture control intent conflicts"
                }
                Some("capture_control_busy") if status.as_u16() == 409 => "capture management busy",
                Some("invalid_capture_control" | "invalid_request") if status.as_u16() == 400 => {
                    "invalid capture control"
                }
                Some("capture_control_capacity" | "resource_limit") if status.as_u16() == 413 => {
                    "capture control capacity exhausted"
                }
                _ => return Err(unknown()),
            };
            return Err(failure(message, false));
        }
        serde_json::from_slice(&bytes).map_err(|_| unknown())
    }
}
