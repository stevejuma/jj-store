//! Explicit, bounded search workers owned by the foreground server lifecycle.
use crate::{Error, Result, config, policy};
use jj_store_adapters::{self as a, CancellationToken};
use jj_store_http::SearchIndex;
use jj_store_model as m;
use jj_store_service::{administration::RepositoryHost, auth::BearerToken};
use jj_store_tantivy::{self as t, native};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    time::Duration,
};
use tokio::{
    task::JoinHandle,
    time::{Instant, timeout_at},
};
use zeroize::Zeroizing;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Repository {
    pub repository: m::RepositoryId,
    pub token_environment: String,
    #[serde(default)]
    pub source: native::Config,
    #[serde(default = "poll_millis")]
    pub poll_millis: u64,
    #[serde(default = "timeout_seconds")]
    pub timeout_seconds: u64,
    #[serde(default = "index_bytes")]
    pub index_bytes: u64,
    #[serde(default = "snapshot_every")]
    pub snapshot_every: u64,
}
fn poll_millis() -> u64 {
    1000
}
fn timeout_seconds() -> u64 {
    60
}
fn index_bytes() -> u64 {
    512 * 1024 * 1024
}
fn snapshot_every() -> u64 {
    32
}
pub(crate) fn validate(entries: &[Repository]) -> Result<()> {
    if entries.len() > 8 {
        return Err(Error::Config("search repository limit"));
    }
    let mut repositories = BTreeSet::new();
    let mut credentials = BTreeSet::new();
    for entry in entries {
        let suffix = entry
            .token_environment
            .strip_prefix("JJ_STORE_SEARCH_TOKEN_");
        if !repositories.insert(entry.repository)
            || !credentials.insert(&entry.token_environment)
            || entry.token_environment.len() > 128
            || suffix.is_none_or(|s| {
                s.is_empty()
                    || !s
                        .bytes()
                        .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
            })
            || !(10..=60000).contains(&entry.poll_millis)
            || !(1..=300).contains(&entry.timeout_seconds)
            || !(1..=2 * 1024 * 1024 * 1024).contains(&entry.index_bytes)
            || entry.snapshot_every > 1_000_000
            || entry.source.identity().is_err()
        {
            return Err(Error::Config(
                "search repository, credential name, source scopes or limits",
            ));
        }
    }
    Ok(())
}
pub(crate) fn environment(name: &str) -> Result<BearerToken> {
    let value = Zeroizing::new(
        std::env::var(name).map_err(|_| Error::Config("search credential unavailable"))?,
    );
    BearerToken::parse(&value).map_err(|_| Error::Config("invalid search credential"))
}
type Runner = a::Runner<t::TantivyProjection<native::NativeExtractor>>;
pub struct PreparedSearch {
    runners: Vec<Runner>,
}
impl PreparedSearch {
    /// Resolves only configured credential names; performs no authoritative I/O.
    pub fn new(
        host: &RepositoryHost,
        root: &Path,
        entries: &[Repository],
        server_limits: &config::Limits,
        mut credential: impl FnMut(&str) -> Result<BearerToken>,
    ) -> Result<(Self, BTreeMap<m::RepositoryId, SearchIndex>)> {
        validate(entries)?;
        let root = policy::outside_checkout(root)?;
        if !(1..=300).contains(&server_limits.request_seconds)
            || !(1..=1_000_000).contains(&server_limits.inventory_nodes)
        {
            return Err(Error::Config("search serving limits"));
        }
        let mut runners = Vec::new();
        let mut indexes = BTreeMap::new();
        for entry in entries {
            let token = credential(&entry.token_environment)?;
            let session = host
                .endpoint(entry.repository)
                .map_err(|_| Error::Config("search repository endpoint"))?
                .bind(token)
                .map_err(|_| Error::Config("search session limits"))?
                .into_session();
            let identity = entry
                .source
                .identity()
                .map_err(|_| Error::Config("search source configuration"))?;
            let index_root =
                policy::outside_checkout(&root.join("search").join(entry.repository.to_string()))?;
            let owner_path = policy::outside_checkout(
                &root
                    .join("adapters")
                    .join(entry.repository.to_string())
                    .join("tantivy"),
            )?;
            if !index_root.starts_with(&root) || !owner_path.starts_with(&root) {
                return Err(Error::Config("search state escapes the server directory"));
            }
            let (projection, search) = t::TantivyProjection::open(
                index_root,
                native::NativeExtractor::new(entry.source.clone())
                    .map_err(|_| Error::Config("search extractor"))?,
                t::Limits {
                    index_bytes: entry.index_bytes,
                    ..Default::default()
                },
            )
            .map_err(|_| Error::Config("search index ownership or local state"))?;
            let poll = Duration::from_millis(entry.poll_millis);
            let transfer_bytes = entry
                .index_bytes
                .saturating_mul(2)
                .min(2 * 1024 * 1024 * 1024);
            let runner = Runner::new(
                session.adapter_source(),
                identity,
                projection,
                &root,
                a::Limits {
                    poll,
                    max_backoff: poll.max(Duration::from_secs(30)),
                    timeout: Duration::from_secs(entry.timeout_seconds),
                    inventory_nodes: server_limits.inventory_nodes,
                    bytes: transfer_bytes,
                    object_bytes: entry.index_bytes,
                    snapshot_every: entry.snapshot_every,
                    ..Default::default()
                },
            )
            .map_err(|_| Error::Config("search runner ownership or limits"))?;
            let timeout = Duration::from_secs(server_limits.request_seconds);
            let limits = t::paging::Limits {
                timeout,
                max_wait: Duration::from_secs(5).min(timeout),
                read: jj_store_service::reads::ReadLimits {
                    max_objects: server_limits.inventory_nodes,
                    timeout,
                },
                query: t::SearchLimits {
                    timeout: Duration::from_secs(5).min(timeout),
                    ..Default::default()
                },
                ..Default::default()
            };
            indexes.insert(
                entry.repository,
                SearchIndex {
                    search,
                    health: runner.subscribe(),
                    limits,
                },
            );
            runners.push(runner);
        }
        Ok((Self { runners }, indexes))
    }
    pub fn start(self) -> RunningSearch {
        let cancel = CancellationToken::new();
        let tasks = self
            .runners
            .into_iter()
            .map(|mut runner| {
                let cancel = cancel.clone();
                tokio::spawn(async move {
                    let result = runner.run(cancel).await;
                    let drained = runner.wait_idle().await;
                    result.and(drained)
                })
            })
            .collect();
        RunningSearch { cancel, tasks }
    }
}
pub struct RunningSearch {
    cancel: CancellationToken,
    tasks: Vec<JoinHandle<a::Result<()>>>,
}
impl RunningSearch {
    pub async fn shutdown(mut self) -> Result<()> {
        self.cancel.cancel();
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut result = Ok(());
        for task in &mut self.tasks {
            match timeout_at(deadline, task).await {
                Ok(Ok(Ok(()))) => {}
                Ok(_) => result = Err(Error::Service("search runner failed during shutdown")),
                Err(_) => return Err(Error::Service("search shutdown exceeded five seconds")),
            }
        }
        self.tasks.clear();
        result
    }
}
impl Drop for RunningSearch {
    fn drop(&mut self) {
        self.cancel.cancel();
        for task in &self.tasks {
            task.abort();
        }
    }
}

#[derive(Serialize)]
pub struct Diagnosis {
    pub configured: bool,
    pub state: &'static str,
    pub status: Option<t::paging::Status>,
}
impl Diagnosis {
    fn state(configured: bool, state: &'static str) -> Self {
        Self {
            configured,
            state,
            status: None,
        }
    }
}
/// Inspect the serving process without opening its local index or writer locks.
pub async fn diagnose(
    config: &config::Config,
    repository: m::RepositoryId,
    token: &BearerToken,
) -> Diagnosis {
    if config.validate().is_err() {
        return Diagnosis::state(true, "invalid_configuration");
    }
    if !config
        .search
        .iter()
        .any(|entry| entry.repository == repository)
    {
        return Diagnosis::state(false, "not_configured");
    }
    let result = async {
        let timeout = Duration::from_secs(config.limits.request_seconds.min(5));
        let mut builder = reqwest::Client::builder()
            .timeout(timeout)
            .redirect(reqwest::redirect::Policy::none());
        if let Some(tls) = &config.tls {
            let pem =
                policy::read_regular(&tls.certificate, 1024 * 1024).map_err(|_| "unavailable")?;
            builder = builder.add_root_certificate(
                reqwest::Certificate::from_pem(&pem).map_err(|_| "unavailable")?,
            );
        }
        let client = builder.build().map_err(|_| "unavailable")?;
        let mut header =
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token.expose().as_str()))
                .map_err(|_| "unavailable")?;
        header.set_sensitive(true);
        let url = format!("{}v1/repos/{repository}/search/status", config.public_url);
        let mut response = client
            .get(url)
            .header(reqwest::header::AUTHORIZATION, header)
            .send()
            .await
            .map_err(|_| "unavailable")?;
        if matches!(response.status().as_u16(), 401 | 403) {
            return Err("access_denied");
        }
        if response.status().as_u16() == 501 {
            return Err("not_configured_on_server");
        }
        if !response.status().is_success() || response.content_length().is_some_and(|n| n > 16384) {
            return Err("unavailable");
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| "unavailable")? {
            if bytes.len() + chunk.len() > 16384 {
                return Err("unavailable");
            }
            bytes.extend_from_slice(&chunk);
        }
        let status: t::paging::Status =
            serde_json::from_slice(&bytes).map_err(|_| "unavailable")?;
        if status.version != 1
            || status.repository != repository
            || status.indexed_position.is_some() != status.index_generation.is_some()
            || status.is_stale != (status.indexed_position != Some(status.repository_position))
            || status
                .indexed_position
                .is_some_and(|p| p.0 > status.repository_position.0)
            || status
                .durable_position
                .is_some_and(|p| p.0 > status.repository_position.0)
        {
            return Err("unavailable");
        }
        Ok(status)
    }
    .await;
    match result {
        Ok(status) => Diagnosis {
            configured: true,
            state: if status.degraded {
                "degraded"
            } else if status.indexed_position.is_none() {
                "not_ready"
            } else if status.is_stale {
                "stale"
            } else {
                "ready"
            },
            status: Some(status),
        },
        Err(state) => Diagnosis::state(true, state),
    }
}
