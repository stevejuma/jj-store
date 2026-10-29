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
            request = authorize(request, url, token);
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

fn authorize(request: reqwest::RequestBuilder, url: &str, token: &str) -> reqwest::RequestBuilder {
    if url::Url::parse(url).is_ok_and(|url| url.path().starts_with("/api/v1/")) {
        request.header(reqwest::header::AUTHORIZATION, format!("token {token}"))
    } else {
        request.bearer_auth(token)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn provider_requests_use_their_own_token_header_without_network_or_redirects() {
        let client = reqwest::Client::new();
        for (url, expected) in [
            ("https://forge.example/api/v1/user", "token synthetic"),
            ("https://api.github.com/user", "Bearer synthetic"),
            ("https://enterprise.example/api/v3/user", "Bearer synthetic"),
        ] {
            let request = authorize(client.get(url), url, "synthetic")
                .build()
                .unwrap();
            assert_eq!(
                request
                    .headers()
                    .get(reqwest::header::AUTHORIZATION)
                    .unwrap(),
                expected
            );
            assert!(request.url().username().is_empty());
        }
    }
}
