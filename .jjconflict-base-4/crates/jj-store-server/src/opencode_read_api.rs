//! Approved loopback acquisition; never starts a harness or sends mutation requests.
use crate::{Error, Result};
use jj_store_capture::opencode_snapshot::{
    MAX_CURSOR_BYTES, MAX_PAGE_MESSAGES, MAX_RESPONSE_BYTES, OpenCodeActivity,
    OpenCodeMessageSnapshotPage, OpenCodeSessionSnapshot, PRODUCT_VERSION, native_id_valid,
};
use reqwest::{Client, Url, header};
use serde_json::Value;
use std::{collections::BTreeSet, time::Duration};
mod discovery;
pub use discovery::{OpenCodeDirectoryInventory, OpenCodeProjectInventory};

#[derive(Clone)]
pub struct OpenCodeReadApi {
    endpoint: Url,
    client: Client,
}
pub struct OpenCodeObservedSession {
    session: OpenCodeSessionSnapshot,
    activity: OpenCodeActivity,
}
pub struct OpenCodeApiDiscovery {
    pub candidates: Vec<OpenCodeSessionCandidate>,
    pub observed: usize,
    pub unsupported: usize,
}
pub struct OpenCodeSessionCandidate {
    native_session: String,
}
impl OpenCodeSessionCandidate {
    pub fn native_session_key(&self) -> &str {
        &self.native_session
    }
}
pub struct OpenCodeMessageAcquisition {
    api: OpenCodeReadApi,
    session: OpenCodeSessionSnapshot,
    before: Option<String>,
    visited: BTreeSet<String>,
    finished: bool,
    pages: usize,
}
impl OpenCodeObservedSession {
    pub fn session(&self) -> &OpenCodeSessionSnapshot {
        &self.session
    }
    pub fn activity(&self) -> OpenCodeActivity {
        self.activity
    }
}
impl OpenCodeReadApi {
    pub(crate) fn endpoint_digest(&self) -> [u8; 32] {
        use sha2::{Digest as _, Sha256};
        Sha256::digest(self.endpoint.as_str().as_bytes()).into()
    }
    pub fn source_binding(
        &self,
        session: &OpenCodeSessionSnapshot,
    ) -> Result<jj_store_capture::source_registry::OpenCodeSourceBinding> {
        jj_store_capture::source_registry::OpenCodeSourceBinding::new(
            self.endpoint_digest(),
            session,
        )
        .map_err(|_| Error::Service("OpenCode source binding unavailable"))
    }
    pub fn new(endpoint: &str) -> Result<Self> {
        let url =
            Url::parse(endpoint).map_err(|_| Error::Config("OpenCode endpoint unavailable"))?;
        if !matches!(url.scheme(), "http" | "https")
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || url.path() != "/"
            || !url.host_str().is_some_and(|host| {
                host.trim_matches(['[', ']'])
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback())
            })
        {
            return Err(Error::Config(
                "OpenCode requires a credential-free loopback endpoint",
            ));
        }
        let client = Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .referer(false)
            .connect_timeout(Duration::from_secs(2))
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|_| Error::Service("OpenCode read client unavailable"))?;
        Ok(Self {
            endpoint: url,
            client,
        })
    }
    async fn get(
        &self,
        segments: &[&str],
        query: &[(&str, &str)],
    ) -> Result<(Vec<u8>, Option<String>)> {
        let mut url = self.endpoint.clone();
        url.path_segments_mut()
            .map_err(|_| Error::Config("OpenCode endpoint unavailable"))?
            .extend(segments);
        if !query.is_empty() {
            url.query_pairs_mut().extend_pairs(query.iter().copied());
        }
        let mut response = self
            .client
            .get(url)
            .header(header::ACCEPT, "application/json")
            .send()
            .await
            .map_err(|_| Error::Service("OpenCode read unavailable"))?;
        if !response.status().is_success()
            || response
                .content_length()
                .is_some_and(|n| n > MAX_RESPONSE_BYTES as u64)
            || response
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.split(';').next())
                != Some("application/json")
        {
            return Err(Error::Service("OpenCode response unavailable"));
        }
        let next = response
            .headers()
            .get("x-next-cursor")
            .map(|v| {
                v.to_str()
                    .ok()
                    .filter(|s| cursor_valid(s))
                    .map(str::to_owned)
                    .ok_or(Error::Service("OpenCode cursor unavailable"))
            })
            .transpose()?;
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| Error::Service("OpenCode response unavailable"))?
        {
            if chunk.len() > MAX_RESPONSE_BYTES - bytes.len() {
                return Err(Error::Service("OpenCode response exceeds bound"));
            }
            bytes.extend_from_slice(&chunk);
        }
        Ok((bytes, next))
    }
    async fn health(&self) -> Result<()> {
        let (bytes, _) = self.get(&["global", "health"], &[]).await?;
        let value: Value = serde_json::from_slice(&bytes)
            .map_err(|_| Error::Service("OpenCode health unavailable"))?;
        if value["healthy"].as_bool() != Some(true)
            || value["version"].as_str() != Some(PRODUCT_VERSION)
        {
            return Err(Error::Service("OpenCode product version unavailable"));
        }
        Ok(())
    }
    async fn info(&self, id: &str, directory: Option<&str>) -> Result<OpenCodeSessionSnapshot> {
        if !native_id_valid(id, "ses") {
            return Err(Error::Service("OpenCode session identity unavailable"));
        }
        let query = directory
            .map(|value| vec![("directory", value)])
            .unwrap_or_default();
        let (bytes, _) = self.get(&["session", id], &query).await?;
        OpenCodeSessionSnapshot::from_api_info(&bytes, id)
            .map_err(|_| Error::Service("OpenCode session binding unavailable"))
    }
    pub async fn observe(&self, id: &str) -> Result<OpenCodeObservedSession> {
        if !native_id_valid(id, "ses") {
            return Err(Error::Service("OpenCode session identity unavailable"));
        }
        self.health().await?;
        let session = self.info(id, None).await?;
        let directory = session
            .directory()
            .to_str()
            .ok_or(Error::Service("OpenCode directory unavailable"))?;
        let (bytes, _) = self
            .get(&["session", "status"], &[("directory", directory)])
            .await?;
        let activity = OpenCodeActivity::from_status_map(&bytes, id)
            .map_err(|_| Error::Service("OpenCode activity unavailable"))?;
        if self.info(id, Some(directory)).await? != session {
            return Err(Error::Service("OpenCode session binding changed"));
        }
        Ok(OpenCodeObservedSession { session, activity })
    }
    /// This pages acquisition only. It grants no reader acknowledgement or repository authority.
    pub fn messages(&self, session: &OpenCodeSessionSnapshot) -> OpenCodeMessageAcquisition {
        OpenCodeMessageAcquisition {
            api: self.clone(),
            session: session.clone(),
            before: None,
            visited: BTreeSet::new(),
            finished: false,
            pages: 0,
        }
    }
    async fn page(
        &self,
        session: &OpenCodeSessionSnapshot,
        before: Option<&str>,
    ) -> Result<(OpenCodeMessageSnapshotPage, Option<String>)> {
        self.health().await?;
        let directory = session
            .directory()
            .to_str()
            .ok_or(Error::Service("OpenCode directory unavailable"))?;
        let id = session.native_session_key();
        if self.info(id, Some(directory)).await? != *session {
            return Err(Error::Service("OpenCode session binding changed"));
        }
        let limit = MAX_PAGE_MESSAGES.to_string();
        let mut query = vec![("directory", directory), ("limit", limit.as_str())];
        if let Some(cursor) = before {
            query.push(("before", cursor));
        }
        let (bytes, next) = self.get(&["session", id, "message"], &query).await?;
        let page = session
            .messages(&bytes)
            .map_err(|_| Error::Service("OpenCode message snapshot unavailable"))?;
        if self.info(id, Some(directory)).await? != *session {
            return Err(Error::Service("OpenCode session binding changed"));
        }
        Ok((page, next))
    }
}
impl OpenCodeMessageAcquisition {
    pub(crate) fn finished(&self) -> bool {
        self.finished
    }
    pub async fn next_page(&mut self) -> Result<Option<OpenCodeMessageSnapshotPage>> {
        if self.finished {
            return Ok(None);
        }
        if self.pages >= 4096 {
            return Err(Error::Service("OpenCode page budget exhausted"));
        }
        let (page, next) = self.api.page(&self.session, self.before.as_deref()).await?;
        if next
            .as_ref()
            .is_some_and(|n| self.visited.contains(n) || self.before.as_ref() == Some(n))
        {
            return Err(Error::Service("OpenCode repeated message cursor"));
        }
        if let Some(cursor) = &self.before {
            self.visited.insert(cursor.clone());
        }
        self.finished = next.is_none();
        self.before = next;
        self.pages += 1;
        Ok(Some(page))
    }
}
fn cursor_valid(value: &str) -> bool {
    !value.is_empty() && value.len() <= MAX_CURSOR_BYTES && !value.chars().any(char::is_control)
}
