//! Explicit, bounded search workers owned by the foreground server lifecycle.
pub use crate::search_config::{Repository, SearchAccess};
pub(crate) use crate::search_config::{environment, validate};
pub use crate::search_worker::{Control, RunningSearch};
use crate::{Error, Result, config, policy};
use jj_store_adapters::{self as a};
use jj_store_http::SearchIndex;
use jj_store_model as m;
use jj_store_service::{
    administration::RepositoryHost,
    auth::{BearerToken, Permission},
};
use jj_store_tantivy::{self as t, native};
use serde::Serialize;
use std::sync::Arc;
use std::{collections::BTreeMap, path::Path, time::Duration};
pub(crate) type Runner = a::Runner<t::TantivyProjection<native::NativeExtractor>>;
pub struct PreparedSearch {
    pub(crate) runners: Vec<(m::RepositoryId, Runner)>,
    control: Arc<Control>,
}
impl PreparedSearch {
    /// Resolves only configured credential names; performs no authoritative I/O.
    pub fn new(
        host: &RepositoryHost,
        root: &Path,
        entries: &[Repository],
        server_limits: &config::Limits,
        access: SearchAccess,
        mut credential: impl FnMut(&str) -> Result<BearerToken>,
    ) -> Result<(Self, BTreeMap<m::RepositoryId, SearchIndex>)> {
        Self::build(
            host,
            root,
            entries,
            server_limits,
            access,
            &mut credential,
            false,
        )
    }
    pub(crate) fn build(
        host: &RepositoryHost,
        root: &Path,
        entries: &[Repository],
        server_limits: &config::Limits,
        access: SearchAccess,
        mut credential: impl FnMut(&str) -> Result<BearerToken>,
        temporary: bool,
    ) -> Result<(Self, BTreeMap<m::RepositoryId, SearchIndex>)> {
        validate(entries, matches!(access, SearchAccess::Authenticated))?;
        let root = policy::outside_checkout(root)?;
        if !(1..=300).contains(&server_limits.request_seconds)
            || !(1..=1_000_000).contains(&server_limits.inventory_nodes)
        {
            return Err(Error::Config("search serving limits"));
        }
        let mut runners = Vec::new();
        let mut indexes = BTreeMap::new();
        for entry in entries.iter().filter(|entry| entry.enabled) {
            let endpoint = host
                .endpoint(entry.repository)
                .map_err(|_| Error::Config("search repository endpoint"))?;
            let request = match access {
                SearchAccess::Authenticated => endpoint.bind(credential(
                    entry
                        .token_environment
                        .as_deref()
                        .ok_or(Error::Config("search credential name"))?,
                )?),
                SearchAccess::LocalTrusted(principal) => {
                    endpoint.bind_local_trusted(principal, Permission::Read)
                }
            }
            .map_err(|_| Error::Config("search session limits"))?;
            let session = request.into_session();
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
            let temp = if temporary {
                let staging = root.join("search-replacements");
                std::fs::create_dir_all(&staging)?;
                Some(
                    tempfile::Builder::new()
                        .prefix("search-")
                        .tempdir_in(staging)?,
                )
            } else {
                None
            };
            let runner_root = temp
                .as_ref()
                .map_or(root.clone(), |dir| dir.path().to_owned());
            let extractor = native::NativeExtractor::new(entry.source.clone())
                .map_err(|_| Error::Config("search extractor"))?;
            let projection_limits = t::Limits {
                index_bytes: entry.index_bytes,
                ..Default::default()
            };
            let (projection, search) = match temp {
                Some(temp) => {
                    t::TantivyProjection::open_temporary(temp, extractor, projection_limits)
                }
                None => t::TantivyProjection::open(index_root, extractor, projection_limits),
            }
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
                &runner_root,
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
            runners.push((entry.repository, runner));
        }
        Ok((
            Self {
                runners,
                control: Arc::new(Control::default()),
            },
            indexes,
        ))
    }
    pub fn control(&self) -> Arc<Control> {
        self.control.clone()
    }
    pub async fn start(self) -> RunningSearch {
        for (repo, runner) in self.runners {
            self.control.start(repo, runner).await;
        }
        RunningSearch(self.control)
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
    diagnose_with_token(config, repository, Some(token)).await
}

pub async fn diagnose_local(config: &config::Config, repository: m::RepositoryId) -> Diagnosis {
    diagnose_with_token(config, repository, None).await
}

async fn diagnose_with_token(
    config: &config::Config,
    repository: m::RepositoryId,
    token: Option<&BearerToken>,
) -> Diagnosis {
    if config.validate().is_err() {
        return Diagnosis::state(true, "invalid_configuration");
    }
    if !config
        .search
        .iter()
        .any(|entry| entry.repository == repository && entry.enabled)
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
        let url = format!("{}v1/repos/{repository}/search/status", config.public_url);
        let mut request = client.get(url).header("x-jj-store-protocol", "1");
        if let Some(token) = token {
            let mut header = reqwest::header::HeaderValue::from_str(&format!(
                "Bearer {}",
                token.expose().as_str()
            ))
            .map_err(|_| "unavailable")?;
            header.set_sensitive(true);
            request = request.header(reqwest::header::AUTHORIZATION, header);
        }
        let mut response = request.send().await.map_err(|_| "unavailable")?;
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
