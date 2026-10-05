//! Versioned authenticated application API; no direct storage or native JJ types.
#![forbid(unsafe_code)]
mod application_events;
pub mod assets;
mod body;
mod capture_management;
mod collab;
mod collaboration;
mod content_search;
pub mod contract;
mod dto;
mod error;
mod local_browser;
mod local_management;
mod mutations;
mod repositories;
mod review_mutations;
mod routes;
mod search;
mod search_management;
mod server_settings;
mod storage_management;
mod sync_management;
mod work;
pub use search::{SearchIndex, SearchRegistry};
mod uploads;
use axum::{
    Router,
    body::Body,
    extract::{FromRequestParts, Path, State as ExtractState},
    http::{HeaderMap, StatusCode, header, request::Parts},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
pub use error::Error;
use jj_store_model as m;
use jj_store_service::registry::RepositoryRegistry;
use jj_store_service::{
    administration::RepositoryHost,
    auth::{BearerToken, Permission, SystemClock},
    collaboration::{AgentSessionPager, ReviewThreadPager},
    commands, events, index_tasks,
    native::NativeSession,
    pagination::{
        self, GraphPager, SnapshotCursors, browsing::BrowserPager, directory::DirectoryPager,
        operation_browsing::OperationBrowserPager,
    },
    reads::{self, query as q},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap, io::Write as _, path::PathBuf, str::FromStr, sync::Arc, time::Duration,
};
use tokio::{sync::Semaphore, time::Instant};
type Result<T> = std::result::Result<T, Error>;

const PROTOCOL_HEADER: &str = "x-jj-store-protocol";

#[derive(Clone)]
pub enum Access {
    Authenticated,
    LocalTrusted {
        principal: m::PrincipalId,
        permission: Permission,
    },
}

#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
pub struct BootstrapRepository {
    pub id: m::RepositoryId,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
pub struct BackupStatus {
    pub target: String,
    pub driver: String,
    pub transfer: BackupTransfer,
    pub last_verified_restore_point: Option<VerifiedRestorePoint>,
}

#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
pub struct BackupTransfer {
    pub state: String,
    pub completed_objects: u64,
    pub total_objects: Option<u64>,
    pub completed_bytes: u64,
    pub total_bytes: Option<u64>,
}

#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
pub struct VerifiedRestorePoint {
    pub repository: m::RepositoryId,
    pub epoch: m::EpochId,
    pub position: u64,
    pub operation: m::OperationId,
    pub verified_at_millis: i64,
}

#[derive(Clone)]
pub struct Config {
    pub origin: String,
    pub access: Access,
    pub registry: Option<Arc<dyn RepositoryRegistry>>,
    pub local_repositories: Option<Arc<dyn jj_store_service::local_browser::LocalRepositories>>,
    pub local_search: Option<Arc<dyn jj_store_service::search_management::SearchManager>>,
    pub local_storage: Option<Arc<dyn jj_store_service::storage_management::StorageManager>>,
    pub local_sync: Option<Arc<dyn jj_store_service::sync_management::SyncManager>>,
    pub local_settings: Option<Arc<dyn jj_store_service::server_settings::ServerSettingsManager>>,
    pub local_capture: Option<Arc<dyn jj_store_service::capture_management::CaptureManager>>,
    pub local_workspaces: Option<Arc<dyn jj_store_service::local_management::WorkspaceManager>>,
    pub local_folder_picker: Option<Arc<dyn jj_store_service::local_management::FolderPicker>>,
    pub storage_ready: bool,
    pub native_authority: bool,
    pub backups: Vec<BackupStatus>,
    pub concurrency: usize,
    pub event_concurrency: usize,
    pub event_lifetime: Duration,
    pub events: events::Limits,
    pub workers: usize,
    pub timeout: Duration,
    pub max_json_bytes: usize,
    pub max_uri_bytes: usize,
    pub read: reads::ReadLimits,
    pub graph: q::Limits,
    pub content: q::content::Limits,
    pub commands: commands::Limits,
    pub uploads: jj_store_service::uploads::Limits,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            origin: "http://127.0.0.1:7443".into(),
            access: Access::Authenticated,
            registry: None,
            local_workspaces: None,
            local_repositories: None,
            local_storage: None,
            local_sync: None,
            local_capture: None,
            local_settings: None,
            local_search: None,
            local_folder_picker: None,
            storage_ready: true,
            native_authority: true,
            backups: Vec::new(),
            concurrency: 8,
            event_concurrency: 16,
            event_lifetime: Duration::from_secs(300),
            events: Default::default(),
            workers: 2,
            timeout: Duration::from_secs(30),
            max_json_bytes: 16 * 1024 * 1024,
            max_uri_bytes: 16 * 1024,
            read: Default::default(),
            graph: Default::default(),
            content: Default::default(),
            commands: Default::default(),
            uploads: Default::default(),
        }
    }
}
struct State {
    freshness: jj_store_service::freshness::Hub,
    event_monitors: application_events::Monitors,
    event_generation: String,
    host: Option<RepositoryHost>,
    local_reader: jj_store_service::local_browser::code::Executor,
    working_tree: jj_store_service::working_tree::Observer,
    local_tasks: jj_store_service::local_index_tasks::IndexTasks,
    code_indexes: Arc<jj_store_service::code_indexes::Indexes>,
    local_jobs: Arc<Semaphore>,
    search: SearchRegistry,
    config: Config,
    slots: Arc<Semaphore>,
    event_slots: Arc<Semaphore>,
    queries: q::workers::Executors,
    graphs: GraphPager,
    directories: DirectoryPager,
    browser: BrowserPager,
    operation_browser: OperationBrowserPager,
    agent_sessions: AgentSessionPager,
    review_threads: ReviewThreadPager,
    collab: jj_store_service::collab_pages::CollabPager,
    work: jj_store_service::work::WorkPager,
    content: jj_store_service::content_search::ContentPager,
    review_commands: jj_store_service::review_mutations::ReviewCommandService,
    tasks: index_tasks::IndexTasks,
    kind_tasks: jj_store_service::kind_tasks::KindTasks,
    commands: jj_store_service::mutations::CommandService,
    uploads: jj_store_service::uploads::Uploads,
    repository_cursor_key: [u8; 32],
}
#[derive(Clone)]
pub struct HttpApi(Arc<State>);
impl HttpApi {
    pub fn new(
        host: RepositoryHost,
        cache: PathBuf,
        config: Config,
    ) -> std::result::Result<Self, &'static str> {
        Self::with_search(host, cache, config, BTreeMap::new())
    }
    /// Compose explicitly configured search projections without starting adapter writers.
    pub fn with_search(
        host: impl Into<Option<RepositoryHost>>,
        cache: PathBuf,
        config: Config,
        indexes: BTreeMap<m::RepositoryId, SearchIndex>,
    ) -> std::result::Result<Self, &'static str> {
        let host = host.into();
        let mut config = config;
        config.native_authority = host.is_some();
        let url = url::Url::parse(&config.origin).map_err(|_| "HTTP origin")?;
        if !matches!(url.scheme(), "http" | "https")
            || url.origin().ascii_serialization() != config.origin
            || config.backups.len() > 16
            || !(1..=64).contains(&config.concurrency)
            || !(1..=128).contains(&config.event_concurrency)
            || config.event_lifetime.is_zero()
            || config.event_lifetime > Duration::from_secs(300)
            || !(64..=64 * 1024 * 1024).contains(&config.max_json_bytes)
            || !(512..=32768).contains(&config.max_uri_bytes)
            || config.timeout.is_zero()
            || config.timeout > Duration::from_secs(300)
        {
            return Err("HTTP limits or origin");
        }
        config.events.validate().map_err(|_| "HTTP event limits")?;
        let queries = q::workers::Executors::new(
            cache.clone(),
            config.graph,
            config.content,
            q::cache::Policy::default(),
            config.workers,
        )
        .map_err(|_| "HTTP native query limits")?;
        if indexes.len() > 128 {
            return Err("HTTP search repository limit");
        }
        let mut search = BTreeMap::new();
        for (repo, mut index) in indexes {
            index.limits.query.output_bytes =
                index.limits.query.output_bytes.min(config.max_json_bytes);
            let pager = jj_store_tantivy::paging::Pager::new(
                index.search,
                index.health,
                queries.content().clone(),
                index.limits,
                Arc::new(SystemClock),
            )
            .map_err(|_| "HTTP search limits")?;
            search.insert(repo, Arc::new(pager));
        }
        let search = SearchRegistry::new(search, queries.content().clone(), config.max_json_bytes);
        let paging_limits = pagination::Limits {
            concurrency: config.concurrency,
            read: config.read,
            ..Default::default()
        };
        let cursors = Arc::new(
            SnapshotCursors::new(paging_limits, Arc::new(SystemClock))
                .map_err(|_| "HTTP cursor limits")?,
        );
        let graphs = GraphPager::with_cursors(queries.graph().clone(), cursors.clone());
        let directories = DirectoryPager::with_cursors(queries.content().clone(), cursors.clone());
        let browser = BrowserPager::with_cursors(queries.content().clone(), cursors.clone());
        let operation_browser =
            OperationBrowserPager::with_cursors(queries.content().clone(), cursors);
        let agent_sessions = AgentSessionPager::new(paging_limits, Arc::new(SystemClock))
            .map_err(|_| "HTTP collaboration cursor limits")?;
        let review_threads = ReviewThreadPager::new(paging_limits, Arc::new(SystemClock))
            .map_err(|_| "HTTP review cursor limits")?;
        let collab = jj_store_service::collab_pages::CollabPager::new(
            jj_store_service::collab_pages::Limits {
                concurrency: config.concurrency,
                timeout: config.timeout,
                ..Default::default()
            },
            Arc::new(SystemClock),
            Arc::new(jj_store_service::collab_reads::RepositoryMemberReactionActors),
        )
        .map_err(|_| "HTTP Thread/Reaction cursor limits")?;
        let review_commands = jj_store_service::review_mutations::ReviewCommandService;
        let tasks = index_tasks::IndexTasks::new(
            queries.graph().clone(),
            index_tasks::Limits {
                running: config.workers,
                read: config.read,
                ..Default::default()
            },
        )
        .map_err(|_| "HTTP index task limits")?;
        let kind_tasks = jj_store_service::kind_tasks::KindTasks::new(
            queries.workers().clone(),
            jj_store_service::kind_tasks::Limits {
                read: config.read,
                ..Default::default()
            },
        )
        .map_err(|_| "HTTP kind preparation task limits")?;
        let commands = jj_store_service::mutations::CommandService::new(
            commands::Executor::with_workers(config.commands, queries.workers().clone())
                .map_err(|_| "HTTP native command limits")?,
        );
        let uploads = jj_store_service::uploads::Uploads::new(config.uploads)
            .map_err(|_| "HTTP upload limits")?;
        let mut repository_cursor_key = [0; 32];
        getrandom::fill(&mut repository_cursor_key).map_err(|_| "HTTP cursor entropy")?;
        let local_reader = jj_store_service::local_browser::code::Executor::with_workers(
            queries.workers().clone(),
            config.content,
        );
        let local_tasks = jj_store_service::local_index_tasks::IndexTasks::new(
            local_reader.clone(),
            cache.clone(),
            Default::default(),
        )
        .map_err(|_| "HTTP local index task limits")?;
        let code_indexes = jj_store_service::code_indexes::Indexes::new(
            queries.graph().clone(),
            local_reader.clone(),
            cache,
            Default::default(),
        );
        let browser = if matches!(config.access, Access::LocalTrusted { .. }) {
            browser.with_code_indexes(code_indexes.clone())
        } else {
            browser
        };
        let graphs = if matches!(config.access, Access::LocalTrusted { .. }) {
            graphs.with_code_indexes(code_indexes.clone())
        } else {
            graphs
        };
        let working_tree = jj_store_service::working_tree::Observer::new(local_reader.clone(), 1)
            .map_err(|_| "HTTP physical observation limits")?;
        let api = Self(Arc::new(State {
            freshness: jj_store_service::freshness::Hub::new(1024),
            event_monitors: Default::default(),
            event_generation: hex::encode(&repository_cursor_key[..16]),
            host,
            local_reader,
            working_tree,
            local_tasks,
            code_indexes,
            search,
            slots: Arc::new(Semaphore::new(config.concurrency)),
            local_jobs: Arc::new(Semaphore::new(config.workers)),
            event_slots: Arc::new(Semaphore::new(config.event_concurrency)),
            config,
            queries,
            graphs,
            directories,
            browser,
            operation_browser,
            agent_sessions,
            review_threads,
            collab,
            work: jj_store_service::work::WorkPager::new(Arc::new(SystemClock)),
            content: jj_store_service::content_search::ContentPager::new(Arc::new(SystemClock)),
            review_commands,
            tasks,
            kind_tasks,
            commands,
            uploads,
            repository_cursor_key,
        }));
        application_events::resume_code_indexes(&api.0);
        Ok(api)
    }
    pub fn search_registry(&self) -> SearchRegistry {
        self.0.search.clone()
    }
    pub fn router(&self) -> Router {
        Router::new().nest(
            "/v1",
            routes::router()
                .merge(application_events::router())
                .merge(local_browser::router())
                .merge(capture_management::router())
                .merge(server_settings::router())
                .with_state(self.0.clone())
                .layer(middleware::from_fn_with_state(self.0.clone(), admit)),
        )
    }
}
fn secure(mut response: Response) -> Response {
    let headers = response.headers_mut();
    headers.insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
    headers.insert(header::X_CONTENT_TYPE_OPTIONS, "nosniff".parse().unwrap());
    headers.insert(header::REFERRER_POLICY, "no-referrer".parse().unwrap());
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        "default-src 'none'; frame-ancestors 'none'; base-uri 'none'"
            .parse()
            .unwrap(),
    );
    response
}
async fn admit(
    ExtractState(state): ExtractState<Arc<State>>,
    mut request: axum::extract::Request,
    next: Next,
) -> Response {
    for (name, value) in request.headers_mut().iter_mut() {
        if name == header::AUTHORIZATION {
            value.set_sensitive(true);
        }
    }
    let checked = {
        let check = || -> Result<()> {
            if request.uri().to_string().len() > state.config.max_uri_bytes
                || request.headers().len() > 64
                || request
                    .headers()
                    .iter()
                    .map(|(k, v)| k.as_str().len() + v.as_bytes().len())
                    .sum::<usize>()
                    > 16 * 1024
            {
                return Err(Error::limit());
            }
            let url = url::Url::parse(&state.config.origin).map_err(|_| Error::denied())?;
            let host = match url.host().ok_or_else(Error::denied)? {
                url::Host::Domain(host) => host.to_owned(),
                url::Host::Ipv4(host) => host.to_string(),
                url::Host::Ipv6(host) => format!("[{host}]"),
            };
            let expected_authority = match url.port() {
                Some(port) => format!("{host}:{port}"),
                None => host,
            };
            let host = request.headers().get_all(header::HOST);
            if host.iter().count() != 1
                || host
                    .iter()
                    .next()
                    .is_none_or(|value| value.as_bytes() != expected_authority.as_bytes())
            {
                return Err(Error::denied());
            }
            let protocol = request.headers().get_all(PROTOCOL_HEADER);
            if protocol.iter().count() != 1
                || protocol.iter().next().is_none_or(|value| value != "1")
            {
                return Err(Error::denied());
            }
            if let Some(origin) = request.headers().get(header::ORIGIN)
                && (request.headers().get_all(header::ORIGIN).iter().count() != 1
                    || origin == "null"
                    || origin.as_bytes() != state.config.origin.as_bytes())
            {
                return Err(Error::denied());
            }
            for name in [
                "x-http-method-override",
                "x-method-override",
                "x-http-method",
            ] {
                if request.headers().contains_key(name) {
                    return Err(Error::denied());
                }
            }
            let fetch_names = ["sec-fetch-site", "sec-fetch-mode", "sec-fetch-dest"];
            let fetch_present = fetch_names
                .iter()
                .any(|name| request.headers().contains_key(*name));
            if fetch_present
                && (request.headers().get_all("sec-fetch-site").iter().count() != 1
                    || request
                        .headers()
                        .get("sec-fetch-site")
                        .is_none_or(|v| v != "same-origin")
                    || request.headers().get_all("sec-fetch-mode").iter().count() != 1
                    || request
                        .headers()
                        .get("sec-fetch-mode")
                        .is_none_or(|v| v != "cors" && v != "same-origin")
                    || request.headers().get_all("sec-fetch-dest").iter().count() != 1
                    || request
                        .headers()
                        .get("sec-fetch-dest")
                        .is_none_or(|v| v != "empty"))
            {
                return Err(Error::denied());
            }
            if matches!(state.config.access, Access::LocalTrusted { .. })
                && request.headers().contains_key(header::AUTHORIZATION)
            {
                return Err(Error::denied());
            }
            let path = request.uri().path();
            if (path.starts_with("/local/") || path.starts_with("/v1/local/"))
                && !matches!(
                    state.config.access,
                    Access::LocalTrusted {
                        permission: Permission::Admin,
                        ..
                    }
                )
            {
                return Err(Error::denied());
            }
            let bodyless = matches!(
                *request.method(),
                axum::http::Method::GET | axum::http::Method::HEAD
            ) || path.ends_with("/cancel");
            let media = request.headers().get_all(header::CONTENT_TYPE);
            if bodyless {
                if media.iter().count() != 0 {
                    return Err(Error::invalid());
                }
            } else if request.method() == axum::http::Method::POST {
                let expected = if path.ends_with("/conflict-uploads")
                    || request
                        .extensions()
                        .get::<axum::extract::MatchedPath>()
                        .is_some_and(|p| {
                            p.as_str() == "/v1/repos/{repo}/reaction-emojis/{emoji_id}"
                        }) {
                    "application/octet-stream"
                } else {
                    "application/json"
                };
                if media.iter().count() != 1
                    || media.iter().next().is_none_or(|value| value != expected)
                    || request.headers().contains_key(header::CONTENT_ENCODING)
                {
                    return Err(Error::new(
                        StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        "unsupported_media_type",
                    ));
                }
            }
            if bodyless {
                use http_body::Body as _;
                if request.body().size_hint().lower() > 0
                    || request.headers().contains_key(header::TRANSFER_ENCODING)
                    || request
                        .headers()
                        .get(header::CONTENT_LENGTH)
                        .is_some_and(|v| v != "0")
                {
                    return Err(Error::invalid());
                }
            }
            Ok(())
        };
        check()
    };
    if let Err(error) = checked {
        return secure(error.into_response());
    }
    let event_route = request.method() == axum::http::Method::GET
        && request
            .extensions()
            .get::<axum::extract::MatchedPath>()
            .is_some_and(|path| {
                matches!(
                    path.as_str(),
                    "/v1/events" | "/v1/repos/{repo}/events" | "/v1/local/repos/{repo}/events"
                )
            });
    let slots = if event_route {
        &state.event_slots
    } else {
        &state.slots
    };
    let permit = match slots.clone().try_acquire_owned() {
        Ok(p) => p,
        Err(_) => return secure(Error::busy().into_response()),
    };
    let deadline = Instant::now() + state.config.timeout;
    if matches!(
        *request.method(),
        axum::http::Method::GET | axum::http::Method::HEAD
    ) || request.uri().path().ends_with("/cancel")
    {
        // HTTP/2 DATA can arrive without Content-Length or a lower size hint.
        // Verify EOF under admission before a bodyless route executes.
        let body = std::mem::replace(request.body_mut(), Body::empty());
        match tokio::time::timeout_at(deadline, axum::body::to_bytes(body, 0)).await {
            Ok(Ok(_)) => {}
            Ok(Err(_)) => return secure(Error::invalid().into_response()),
            Err(_) => return secure(Error::timeout().into_response()),
        }
    }
    let pending = mutations::Pending::default();
    let head = request.method() == axum::http::Method::HEAD;
    request.extensions_mut().insert(pending.clone());
    let response = match tokio::time::timeout_at(deadline, next.run(request)).await {
        Ok(response) if Instant::now() < deadline => response,
        _ => {
            let mut response = pending
                .response()
                .unwrap_or_else(|| Error::timeout().into_response());
            if head {
                *response.body_mut() = Body::empty();
            }
            return secure(response);
        }
    };
    let delivery_deadline = if event_route && response.status().is_success() {
        Instant::now() + state.config.event_lifetime
    } else {
        deadline
    };
    secure(response.map(|body| Body::new(body::Guard::new(body, permit, delivery_deadline))))
}
struct Context {
    session: Arc<NativeSession>,
    paths: BTreeMap<String, String>,
    headers: HeaderMap,
}
impl Context {
    fn value(&self, key: &str) -> Result<&str> {
        self.paths
            .get(key)
            .map(String::as_str)
            .ok_or_else(Error::invalid)
    }
    fn id<T: FromStr>(&self, key: &str) -> Result<T> {
        self.value(key)?.parse().map_err(|_| Error::invalid())
    }
}
impl FromRequestParts<Arc<State>> for Context {
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &Arc<State>) -> Result<Self> {
        let Path(paths) = Path::<BTreeMap<String, String>>::from_request_parts(parts, state)
            .await
            .map_err(|_| Error::invalid())?;
        let repository: m::RepositoryId = paths
            .get("repo")
            .ok_or_else(Error::invalid)?
            .parse()
            .map_err(|_| Error::invalid())?;
        let endpoint = state
            .host
            .as_ref()
            .ok_or_else(|| Error::new(StatusCode::NOT_IMPLEMENTED, "native_authority_unavailable"))?
            .endpoint(repository)?;
        let session = match &state.config.access {
            Access::Authenticated => {
                if parts.headers.get_all(header::AUTHORIZATION).iter().count() != 1 {
                    return Err(Error::new(StatusCode::UNAUTHORIZED, "bearer_required"));
                }
                let text = parts
                    .headers
                    .get(header::AUTHORIZATION)
                    .and_then(|h| h.to_str().ok())
                    .and_then(|h| h.strip_prefix("Bearer "))
                    .ok_or_else(|| Error::new(StatusCode::UNAUTHORIZED, "bearer_required"))?;
                let token = BearerToken::parse(text)
                    .map_err(|_| Error::new(StatusCode::UNAUTHORIZED, "invalid_credential"))?;
                endpoint.bind(token)?.into_session()
            }
            Access::LocalTrusted {
                principal,
                permission,
                ..
            } => endpoint
                .bind_local_trusted(*principal, *permission)?
                .into_session(),
        };
        session
            .principal(Permission::Read)
            .await
            .map_err(reads::Error::from)?;
        state
            .event_monitors
            .activate(state, Some(repository), false)
            .await;
        // Retain only range/request metadata; no bearer header enters response state.
        let mut headers = HeaderMap::new();
        for name in [
            header::RANGE,
            axum::http::HeaderName::from_static("last-event-id"),
        ] {
            for value in parts.headers.get_all(&name) {
                headers.append(name.clone(), value.clone());
            }
        }
        Ok(Self {
            session,
            paths,
            headers,
        })
    }
}
async fn pinned(
    context: &Context,
    state: &State,
    operation: Option<m::OperationId>,
) -> Result<Arc<reads::CommittedRead>> {
    match context
        .session
        .open_read(operation, state.config.read)
        .await?
    {
        reads::OpenRead::Pinned(read) => Ok(Arc::new(read)),
        reads::OpenRead::SelectHead(heads) => Err(Error::heads(heads)),
    }
}
fn encode(value: &impl Serialize, limit: usize) -> Result<Vec<u8>> {
    struct Buffer {
        bytes: Vec<u8>,
        limit: usize,
    }
    impl std::io::Write for Buffer {
        fn write(&mut self, value: &[u8]) -> std::io::Result<usize> {
            if value.len() > self.limit.saturating_sub(self.bytes.len()) {
                return Err(std::io::Error::other("JSON limit"));
            }
            self.bytes.extend_from_slice(value);
            Ok(value.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut buffer = Buffer {
        bytes: Vec::new(),
        limit,
    };
    serde_json::to_writer(&mut buffer, value).map_err(|_| Error::limit())?;
    buffer.flush().map_err(|_| Error::unavailable())?;
    Ok(buffer.bytes)
}
impl State {
    fn json(&self, value: &impl Serialize) -> Result<Response> {
        Ok((
            [(header::CONTENT_TYPE, "application/json")],
            encode(value, self.config.max_json_bytes)?,
        )
            .into_response())
    }
}
#[derive(Serialize, schemars::JsonSchema)]
#[schemars(rename = "Envelope_{T}")]
struct Envelope<T> {
    snapshot: reads::ReadSnapshot,
    data: T,
    next_cursor: Option<String>,
}
async fn envelope(
    state: &State,
    read: &reads::CommittedRead,
    value: impl Serialize,
) -> Result<Response> {
    state.json(&Envelope {
        snapshot: read.snapshot().await?.clone(),
        data: value,
        next_cursor: None,
    })
}
