use crate::{Error, Result};
use futures::StreamExt;
use serde_json::Value;

#[async_trait::async_trait(?Send)]
pub trait Transport: Send + Sync {
    async fn request(&self, url: &str, token: Option<&str>, body: Option<Value>) -> Result<Value>;
}
pub struct Http;
#[async_trait::async_trait(?Send)]
impl Transport for Http {
    async fn request(&self, url: &str, token: Option<&str>, body: Option<Value>) -> Result<Value> {
        let http = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("jj-store-forge/1")
            .build()
            .map_err(|_| Error::State)?;
        let mut request = if let Some(body) = body {
            http.post(url).json(&body)
        } else {
            http.get(url)
        }
        .header("Accept", "application/json");
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        let response = request.send().await.map_err(|_| Error::Unknown)?;
        let status = response.status().as_u16();
        if status == 401 || status == 403 {
            return Err(Error::Denied);
        }
        if status == 429 {
            return Err(Error::RateLimited);
        }
        if !(200..300).contains(&status) {
            return Err(Error::Rejected(status));
        }
        let mut stream = response.bytes_stream();
        let mut bytes = vec![];
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|_| Error::Unknown)?;
            if bytes.len() + chunk.len() > 256 * 1024 {
                return Err(Error::Limit);
            }
            bytes.extend_from_slice(&chunk);
        }
        serde_json::from_slice(&bytes).map_err(|_| Error::Invalid)
    }
}
