//! Versioned authenticated application API; no direct storage or native JJ types.
#![forbid(unsafe_code)]
pub mod assets;
mod body;
pub mod contract;
mod dto;
mod error;
mod mutations;
mod routes;
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
use jj_store_service::{
    administration::RepositoryHost,
    auth::{BearerToken, Permission, SystemClock},
    commands, events, index_tasks,
    native::NativeSession,
    pagination::{self, GraphPager, SnapshotCursors, directory::DirectoryPager},
    reads::{self, query as q},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap, io::Write as _, path::PathBuf, str::FromStr, sync::Arc, time::Duration,
};
use tokio::{sync::Semaphore, time::Instant};
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct Config {
    pub origin: String,
    pub concurrency: usize,
    pub event_concurrency: usize,
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
            concurrency: 8,
            event_concurrency: 16,
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
    host: RepositoryHost,
    config: Config,
    slots: Arc<Semaphore>,
    event_slots: Arc<Semaphore>,
    queries: q::workers::Executors,
    graphs: GraphPager,
    directories: DirectoryPager,
    tasks: index_tasks::IndexTasks,
    commands: jj_store_service::mutations::CommandService,
    uploads: jj_store_service::uploads::Uploads,
}
#[derive(Clone)]
pub struct HttpApi(Arc<State>);
impl HttpApi {
    pub fn new(
        host: RepositoryHost,
        cache: PathBuf,
        config: Config,
    ) -> std::result::Result<Self, &'static str> {
        let url = url::Url::parse(&config.origin).map_err(|_| "HTTP origin")?;
        if !matches!(url.scheme(), "http" | "https")
            || url.origin().ascii_serialization() != config.origin
            || !(1..=64).contains(&config.concurrency)
            || !(1..=128).contains(&config.event_concurrency)
            || !(64..=64 * 1024 * 1024).contains(&config.max_json_bytes)
            || !(512..=32768).contains(&config.max_uri_bytes)
            || config.timeout.is_zero()
            || config.timeout > Duration::from_secs(300)
        {
            return Err("HTTP limits or origin");
        }
        config.events.validate().map_err(|_| "HTTP event limits")?;
        let queries = q::workers::Executors::new(
            cache,
            config.graph,
            config.content,
            q::cache::Policy::default(),
            config.workers,
        )
        .map_err(|_| "HTTP native query limits")?;
        let cursors = Arc::new(
            SnapshotCursors::new(
                pagination::Limits {
                    concurrency: config.concurrency,
                    read: config.read,
                    ..Default::default()
                },
                Arc::new(SystemClock),
            )
            .map_err(|_| "HTTP cursor limits")?,
        );
        let graphs = GraphPager::with_cursors(queries.graph().clone(), cursors.clone());
        let directories = DirectoryPager::with_cursors(queries.content().clone(), cursors);
        let tasks = index_tasks::IndexTasks::new(
            queries.graph().clone(),
            index_tasks::Limits {
                running: config.workers,
                read: config.read,
                ..Default::default()
            },
        )
        .map_err(|_| "HTTP index task limits")?;
        let commands = jj_store_service::mutations::CommandService::new(
            commands::Executor::with_workers(config.commands, queries.workers().clone())
                .map_err(|_| "HTTP native command limits")?,
        );
        let uploads = jj_store_service::uploads::Uploads::new(config.uploads)
            .map_err(|_| "HTTP upload limits")?;
        Ok(Self(Arc::new(State {
            host,
            slots: Arc::new(Semaphore::new(config.concurrency)),
            event_slots: Arc::new(Semaphore::new(config.event_concurrency)),
            config,
            queries,
            graphs,
            directories,
            tasks,
            commands,
            uploads,
        })))
    }
    pub fn router(&self) -> Router {
        Router::new().nest(
            "/v1",
            routes::router()
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
            if let Some(origin) = request.headers().get(header::ORIGIN)
                && (request.headers().get_all(header::ORIGIN).iter().count() != 1
                    || origin.as_bytes() != state.config.origin.as_bytes())
            {
                return Err(Error::denied());
            }
            if matches!(
                *request.method(),
                axum::http::Method::GET | axum::http::Method::HEAD
            ) || request.uri().path().ends_with("/cancel")
            {
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
            .is_some_and(|path| path.as_str() == "/v1/repos/{repo}/events");
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
    secure(response.map(|body| Body::new(body::Guard::new(body, permit, deadline))))
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
        let repository: m::RepositoryId = paths
            .get("repo")
            .ok_or_else(Error::invalid)?
            .parse()
            .map_err(|_| Error::invalid())?;
        let session = state.host.endpoint(repository)?.bind(token)?.into_session();
        session
            .principal(Permission::Read)
            .await
            .map_err(reads::Error::from)?;
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
