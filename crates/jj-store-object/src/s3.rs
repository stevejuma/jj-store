//! Path-style S3 transport for the qualified RustFS contract. Conditional PUT
//! has no automatic retry: a lost acknowledgement remains an unknown outcome.
use crate::*;
use base64::Engine as _;
use chrono::{DateTime, Utc};
use futures::TryStreamExt as _;
use hmac::{Hmac, Mac as _};
use reqwest::{Client, Method, Response, StatusCode, Url};
use sha2::{Digest as _, Sha256};
use std::{
    collections::BTreeMap,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::io::{AsyncRead, ReadBuf};
use tokio_util::io::{ReaderStream, StreamReader};

pub struct Credentials {
    pub access_key: String,
    pub secret_key: String,
    pub session_token: Option<String>,
}
impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Credentials([REDACTED])")
    }
}
#[derive(Debug)]
pub struct Config {
    pub endpoint: String,
    pub bucket: String,
    pub region: String,
    pub credentials: Credentials,
    pub allow_http_loopback: bool,
    pub max_object_bytes: u64,
    pub timeout: Duration,
}
pub struct S3Store {
    client: Client,
    endpoint: Url,
    config: Config,
    runtime: std::sync::RwLock<(u64, Duration)>,
}
impl S3Store {
    pub fn new(config: Config) -> Result<Self> {
        let endpoint =
            Url::parse(&config.endpoint).map_err(|_| Error::Invalid("storage endpoint"))?;
        let host = endpoint.host_str().unwrap_or("").trim_matches(['[', ']']);
        let loopback = host
            .parse::<std::net::IpAddr>()
            .is_ok_and(|ip| ip.is_loopback());
        if endpoint.scheme() != "https"
            && !(endpoint.scheme() == "http" && config.allow_http_loopback && loopback)
        {
            return Err(Error::Invalid(
                "storage endpoint requires HTTPS or explicit loopback HTTP",
            ));
        }
        if !endpoint.username().is_empty()
            || endpoint.password().is_some()
            || endpoint.query().is_some()
            || endpoint.fragment().is_some()
            || endpoint.path() != "/"
        {
            return Err(Error::Invalid(
                "endpoint must contain only scheme and authority",
            ));
        }
        if !(3..=63).contains(&config.bucket.len())
            || config
                .bucket
                .bytes()
                .any(|b| !b.is_ascii_lowercase() && !b.is_ascii_digit() && !b".-".contains(&b))
            || config.bucket.contains("..")
        {
            return Err(Error::Invalid("bucket"));
        }
        if config.region.is_empty()
            || config.region.len() > 128
            || config
                .region
                .bytes()
                .any(|b| !b.is_ascii_alphanumeric() && b != b'-')
        {
            return Err(Error::Invalid("region"));
        }
        if config.credentials.access_key.is_empty() || config.credentials.secret_key.is_empty() {
            return Err(Error::Invalid("credentials are required"));
        }
        if config.max_object_bytes > staging::MAX_SINGLE_PUT || config.timeout.is_zero() {
            return Err(Error::Invalid("storage byte limit or timeout"));
        }
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(config.timeout)
            .no_proxy()
            .build()
            .map_err(|_| Error::Invalid("HTTP client"))?;
        Ok(Self {
            client,
            endpoint,
            runtime: std::sync::RwLock::new((config.max_object_bytes, config.timeout)),
            config,
        })
    }
    pub fn reload_limits(&self, max_object_bytes: u64, timeout: Duration) -> Result<()> {
        if max_object_bytes > staging::MAX_SINGLE_PUT || timeout.is_zero() {
            return Err(Error::Invalid("storage client limits"));
        }
        *self
            .runtime
            .write()
            .map_err(|_| Error::Invalid("storage runtime lock"))? = (max_object_bytes, timeout);
        Ok(())
    }
    fn runtime_limits(&self) -> (u64, Duration) {
        *self.runtime.read().expect("storage runtime poisoned")
    }
    fn request(
        &self,
        method: Method,
        key: &Key,
        payload: [u8; 32],
        extra: BTreeMap<String, String>,
    ) -> Result<reqwest::RequestBuilder> {
        let mut url = self.endpoint.clone();
        url.set_path(&format!("/{}/{}", self.config.bucket, key.as_str()));
        Ok(self.signed_request(method, url, payload, extra))
    }
    fn signed_request(
        &self,
        method: Method,
        url: Url,
        payload: [u8; 32],
        extra: BTreeMap<String, String>,
    ) -> reqwest::RequestBuilder {
        let headers = sign(
            &method,
            &url,
            &self.config.region,
            &self.config.credentials,
            payload,
            extra,
            Utc::now(),
        );
        let mut request = self
            .client
            .request(method, url)
            .timeout(self.runtime_limits().1);
        for (name, value) in headers {
            request = request.header(name, value);
        }
        request
    }
    fn metadata(&self, response: &Response, max_object_bytes: u64) -> Result<Metadata> {
        let len = response
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .ok_or(Error::Response("missing content length"))?;
        if len > max_object_bytes {
            return Err(Error::Limit);
        }
        Ok(Metadata {
            len,
            version: version(response)?,
        })
    }
    /// Create the configured bucket if it is absent.
    ///
    /// This is an explicit provisioning operation for a managed storage
    /// runtime. Ordinary object reads and writes never list or provision.
    pub async fn ensure_bucket(&self) -> Result<()> {
        let mut url = self.endpoint.clone();
        url.set_path(&format!("/{}", self.config.bucket));
        let response = self
            .signed_request(Method::PUT, url, Sha256::digest([]).into(), BTreeMap::new())
            .body(Vec::new())
            .send()
            .await
            .map_err(|_| Error::Transport)?;
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            status => Err(Error::Rejected(status.as_u16())),
        }
    }

    /// Verify that a previously provisioned bucket remains present.
    pub async fn bucket_exists(&self) -> Result<bool> {
        let mut url = self.endpoint.clone();
        url.set_path(&format!("/{}", self.config.bucket));
        let response = self
            .signed_request(
                Method::HEAD,
                url,
                Sha256::digest([]).into(),
                BTreeMap::new(),
            )
            .send()
            .await
            .map_err(|_| Error::Transport)?;
        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            status => Err(Error::Rejected(status.as_u16())),
        }
    }
}
fn version(response: &Response) -> Result<VersionToken> {
    VersionToken::new(
        response
            .headers()
            .get("etag")
            .and_then(|v| v.to_str().ok())
            .ok_or(Error::Response("missing version token"))?,
    )
}
fn read_status(response: &Response, expected: StatusCode) -> Result<()> {
    match response.status() {
        status if status == expected => Ok(()),
        StatusCode::NOT_FOUND => Err(Error::NotFound),
        StatusCode::RANGE_NOT_SATISFIABLE => Err(Error::Range),
        status => Err(Error::Rejected(status.as_u16())),
    }
}
#[async_trait]
impl ObjectStore for S3Store {
    async fn head(&self, key: &Key) -> Result<Metadata> {
        let max_object_bytes = self.runtime_limits().0;
        let response = self
            .request(
                Method::HEAD,
                key,
                Sha256::digest([]).into(),
                BTreeMap::new(),
            )?
            .send()
            .await
            .map_err(|_| Error::Transport)?;
        read_status(&response, StatusCode::OK)?;
        self.metadata(&response, max_object_bytes)
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> Result<Download> {
        let max_object_bytes = self.runtime_limits().0;
        let mut headers = BTreeMap::new();
        if let Some(r) = range {
            if r.len() > max_object_bytes {
                return Err(Error::Limit);
            }
            headers.insert("range".into(), format!("bytes={}-{}", r.start, r.end - 1));
        }
        let response = self
            .request(Method::GET, key, Sha256::digest([]).into(), headers)?
            .send()
            .await
            .map_err(|_| Error::Transport)?;
        read_status(
            &response,
            if range.is_some() {
                StatusCode::PARTIAL_CONTENT
            } else {
                StatusCode::OK
            },
        )?;
        let metadata = self.metadata(&response, max_object_bytes)?;
        if let Some(r) = range {
            let content = response
                .headers()
                .get("content-range")
                .and_then(|v| v.to_str().ok())
                .ok_or(Error::Response("missing content range"))?;
            let expected = format!("bytes {}-{}/", r.start, r.end - 1);
            let total = content
                .strip_prefix(&expected)
                .and_then(|n| n.parse::<u64>().ok())
                .ok_or(Error::Response("incorrect content range"))?;
            if total < r.end || metadata.len != r.len() {
                return Err(Error::Response("range length mismatch"));
            }
        }
        let stream = response
            .bytes_stream()
            .map_err(|_| std::io::Error::other("storage response stream failed"));
        let reader = ExactReader {
            inner: Box::pin(StreamReader::new(stream)),
            remaining: metadata.len,
        };
        Ok(Download {
            metadata,
            reader: Box::pin(reader),
        })
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> Result<WriteOutcome> {
        if upload.len() > self.runtime_limits().0 {
            return Err(Error::Limit);
        }
        let mut headers = BTreeMap::from([
            ("content-length".into(), upload.len().to_string()),
            (
                "x-amz-checksum-sha256".into(),
                base64::engine::general_purpose::STANDARD.encode(upload.sha256()),
            ),
        ]);
        match condition {
            Condition::Absent => {
                headers.insert("if-none-match".into(), "*".into());
            }
            Condition::Matches(token) => {
                headers.insert("if-match".into(), token.0);
            }
        }
        let body =
            reqwest::Body::wrap_stream(ReaderStream::with_capacity(upload.reader()?, 64 * 1024));
        let request = self
            .request(Method::PUT, key, *upload.sha256(), headers)?
            .body(body);
        let response = match request.send().await {
            Ok(r) => r,
            Err(_) => return Ok(WriteOutcome::Unknown),
        };
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => {
                Ok(match version(&response) {
                    Ok(v) => WriteOutcome::Applied(v),
                    Err(_) => WriteOutcome::Unknown,
                })
            }
            StatusCode::PRECONDITION_FAILED => Ok(WriteOutcome::PreconditionFailed),
            StatusCode::BAD_REQUEST
            | StatusCode::UNAUTHORIZED
            | StatusCode::FORBIDDEN
            | StatusCode::NOT_FOUND => Err(Error::Rejected(response.status().as_u16())),
            _ => Ok(WriteOutcome::Unknown),
        }
    }
}
fn sign(
    method: &Method,
    url: &Url,
    region: &str,
    credentials: &Credentials,
    payload: [u8; 32],
    mut headers: BTreeMap<String, String>,
    now: DateTime<Utc>,
) -> BTreeMap<String, String> {
    let date = now.format("%Y%m%d").to_string();
    let stamp = now.format("%Y%m%dT%H%M%SZ").to_string();
    let host = match url.port() {
        Some(port) => format!("{}:{port}", url.host_str().unwrap()),
        None => url.host_str().unwrap().to_owned(),
    };
    headers.insert("host".into(), host);
    headers.insert("x-amz-date".into(), stamp.clone());
    headers.insert("x-amz-content-sha256".into(), hex::encode(payload));
    if let Some(token) = &credentials.session_token {
        headers.insert("x-amz-security-token".into(), token.clone());
    }
    for value in headers.values_mut() {
        *value = value.split_whitespace().collect::<Vec<_>>().join(" ");
    }
    let names = headers.keys().cloned().collect::<Vec<_>>().join(";");
    let canonical_headers = headers
        .iter()
        .map(|(k, v)| format!("{k}:{v}\n"))
        .collect::<String>();
    let canonical = format!(
        "{}\n{}\n\n{canonical_headers}\n{names}\n{}",
        method.as_str(),
        url.path(),
        hex::encode(payload)
    );
    let scope = format!("{date}/{region}/s3/aws4_request");
    let to_sign = format!(
        "AWS4-HMAC-SHA256\n{stamp}\n{scope}\n{}",
        hex::encode(Sha256::digest(canonical))
    );
    fn hmac(key: &[u8], value: &str) -> Vec<u8> {
        let mut h = Hmac::<Sha256>::new_from_slice(key).expect("HMAC accepts any key length");
        h.update(value.as_bytes());
        h.finalize().into_bytes().to_vec()
    }
    let mut key = format!("AWS4{}", credentials.secret_key).into_bytes();
    for part in [&date, region, "s3", "aws4_request"] {
        key = hmac(&key, part);
    }
    let signature = hex::encode(hmac(&key, &to_sign));
    headers.insert(
        "authorization".into(),
        format!(
            "AWS4-HMAC-SHA256 Credential={}/{scope}, SignedHeaders={names}, Signature={signature}",
            credentials.access_key
        ),
    );
    headers
}
/// Checks truncation and overrun even when a caller consumes the stream directly.
struct ExactReader {
    inner: Reader,
    remaining: u64,
}
impl AsyncRead for ExactReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        out: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        if out.remaining() == 0 {
            return Poll::Ready(Ok(()));
        }
        if self.remaining == 0 {
            let mut byte = [0];
            let mut extra = ReadBuf::new(&mut byte);
            return match self.inner.as_mut().poll_read(cx, &mut extra) {
                Poll::Ready(Ok(())) if !extra.filled().is_empty() => Poll::Ready(Err(
                    std::io::Error::other("response exceeded content length"),
                )),
                other => other,
            };
        }
        let len = out
            .remaining()
            .min(usize::try_from(self.remaining).unwrap_or(usize::MAX));
        let mut slice = ReadBuf::new(out.initialize_unfilled_to(len));
        match self.inner.as_mut().poll_read(cx, &mut slice) {
            Poll::Ready(Ok(())) => {
                let n = slice.filled().len();
                if n == 0 {
                    return Poll::Ready(Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "truncated storage response",
                    )));
                }
                self.remaining -= n as u64;
                out.advance(n);
                Poll::Ready(Ok(()))
            }
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sigv4_matches_independent_python_vector() {
        let credentials = Credentials {
            access_key: "jj-store-disposable-test".into(),
            secret_key: "jj-store-disposable-test-secret-only".into(),
            session_token: None,
        };
        let signed = sign(
            &Method::PUT,
            &Url::parse("http://localhost:9000/bucket/object").unwrap(),
            "us-east-1",
            &credentials,
            Sha256::digest(b"value").into(),
            BTreeMap::from([("if-match".into(), "\"version\"".into())]),
            "2026-09-26T00:00:00Z".parse().unwrap(),
        );
        assert_eq!(
            signed["authorization"],
            include_str!("../tests/sigv4.txt").trim()
        );
        assert!(!format!("{credentials:?}").contains(&credentials.secret_key));
    }
    #[tokio::test]
    async fn streaming_reader_rejects_short_and_excess_bodies() {
        for (data, len) in [(b"short".as_slice(), 6), (b"extra".as_slice(), 4)] {
            let mut reader = ExactReader {
                inner: Box::pin(std::io::Cursor::new(data.to_vec())),
                remaining: len,
            };
            let mut out = Vec::new();
            assert!(reader.read_to_end(&mut out).await.is_err());
        }
    }
}
