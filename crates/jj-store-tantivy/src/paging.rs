//! Authenticated search serving with credential-free generation/view pins.
mod cursors;
use crate::{
    Document, PinnedReader, ScopeSnapshot, Search, SearchLimits, SearchRequest, SearchView,
};
use jj_store_adapters as a;
use jj_store_model as m;
use jj_store_service::{
    auth::Clock,
    native::NativeSession,
    reads::{
        self, HeadSnapshot, OpenRead, ReadLimits, ReadSnapshot, query::content::ContentReader,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::{
    sync::{Semaphore, watch},
    time::{Instant, timeout_at},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid search request")]
    Invalid,
    #[error("invalid search cursor or changed query")]
    InvalidCursor,
    #[error("search cursor expired or its serving instance changed")]
    CursorReset,
    #[error("repository epoch changed")]
    EpochChanged,
    #[error("search index is not ready")]
    NotReady,
    #[error("search index has not reached the requested position")]
    IndexLag(Box<Status>),
    #[error("select an explicit operation")]
    SelectHead(Box<HeadSnapshot>),
    #[error("search serving resource limit exceeded")]
    Limit,
    #[error("search serving is busy")]
    Busy,
    #[error("search serving is unavailable")]
    Unavailable,
    #[error(transparent)]
    Adapter(#[from] a::Error),
    #[error(transparent)]
    Read(#[from] reads::Error),
    #[error(transparent)]
    Native(#[from] reads::query::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "SearchMode")]
pub enum Mode {
    #[default]
    VisibleCommits,
    Historical,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "SearchRequest")]
pub struct Request {
    pub q: String,
    #[serde(default)]
    pub mode: Mode,
    pub operation: Option<m::OperationId>,
    pub scope: Option<String>,
    #[serde(default = "default_page_size")]
    pub page_size: usize,
    pub minimum_position: Option<m::WalPosition>,
    #[serde(default)]
    pub wait_millis: u64,
}
fn default_page_size() -> usize {
    50
}
impl Default for Request {
    fn default() -> Self {
        Self {
            q: String::new(),
            mode: Mode::VisibleCommits,
            operation: None,
            scope: None,
            page_size: default_page_size(),
            minimum_position: None,
            wait_millis: 0,
        }
    }
}
#[derive(Clone, Copy)]
pub struct Limits {
    pub pins: usize,
    pub view_nodes: usize,
    pub concurrency: usize,
    pub ttl: Duration,
    pub timeout: Duration,
    pub max_wait: Duration,
    pub poll: Duration,
    pub read: ReadLimits,
    pub query: SearchLimits,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            pins: 64,
            view_nodes: 1_000_000,
            concurrency: 4,
            ttl: Duration::from_secs(300),
            timeout: Duration::from_secs(30),
            max_wait: Duration::from_secs(5),
            poll: Duration::from_millis(50),
            read: ReadLimits::default(),
            query: SearchLimits::default(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "SearchStatus")]
pub struct Status {
    pub version: u32,
    pub repository: m::RepositoryId,
    pub epoch: m::EpochId,
    pub repository_position: m::WalPosition,
    pub durable_position: Option<m::WalPosition>,
    pub indexed_position: Option<m::WalPosition>,
    pub index_generation: Option<m::RecordDigest>,
    pub is_stale: bool,
    pub degraded: bool,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "SearchPage")]
pub struct Page {
    pub version: u32,
    pub repository: m::RepositoryId,
    pub epoch: m::EpochId,
    pub repository_position: m::WalPosition,
    pub indexed_position: m::WalPosition,
    pub index_generation: m::RecordDigest,
    pub selected_view: Option<ReadSnapshot>,
    pub scope_snapshots: BTreeMap<String, ScopeSnapshot>,
    pub results: Vec<Document>,
    pub is_stale: bool,
    pub degraded: bool,
    pub next_cursor: Option<String>,
    pub truncated: bool,
}
#[derive(Clone)]
pub struct Backend {
    pub search: Search,
    pub health: watch::Receiver<a::Health>,
}
pub struct Pager {
    active: RwLock<Option<Backend>>,
    reader: ContentReader,
    limits: Limits,
    cursors: cursors::Cursors,
    slots: Semaphore,
}
impl Pager {
    pub fn new(
        search: Search,
        health: watch::Receiver<a::Health>,
        reader: ContentReader,
        limits: Limits,
        clock: Arc<dyn Clock>,
    ) -> Result<Self> {
        if !(1..=1024).contains(&limits.pins)
            || !(1..=2_000_000).contains(&limits.view_nodes)
            || !(1..=64).contains(&limits.concurrency)
            || limits.ttl < Duration::from_secs(1)
            || limits.ttl > Duration::from_secs(3600)
            || limits.timeout.is_zero()
            || limits.timeout > Duration::from_secs(300)
            || limits.max_wait > limits.timeout
            || limits.poll < Duration::from_millis(1)
            || limits.poll > Duration::from_secs(1)
            || limits.read.timeout.is_zero()
            || limits.read.timeout > limits.timeout
            || limits.read.max_objects == 0
            || limits.query.timeout.is_zero()
            || limits.query.timeout > limits.timeout
            || limits.query.timeout > Duration::from_secs(30)
            || limits.query.max_offset > 100_000
            || !(1..=1000).contains(&limits.query.max_page)
            || !(1..=8 * 1024 * 1024).contains(&limits.query.output_bytes)
        {
            return Err(Error::Limit);
        }
        Ok(Self {
            active: RwLock::new(Some(Backend { search, health })),
            reader,
            limits,
            cursors: cursors::Cursors::new(limits, clock)?,
            slots: Semaphore::new(limits.concurrency),
        })
    }
    pub fn replace(&self, backend: Option<Backend>) -> Result<()> {
        self.cursors.prune()?;
        *self.active.write().unwrap() = backend;
        Ok(())
    }
    pub fn configured(&self) -> bool {
        self.active.read().unwrap().is_some()
    }
    fn backend(&self) -> Result<Backend> {
        self.active.read().unwrap().clone().ok_or(Error::NotReady)
    }
    fn validate(&self, request: &Request) -> Result<()> {
        if request.q.trim().is_empty()
            || request.q.len() > 1024
            || request.page_size == 0
            || request.page_size > self.limits.query.max_page
            || Duration::from_millis(request.wait_millis) > self.limits.max_wait
            || (request.wait_millis != 0 && request.minimum_position.is_none())
            || (request.mode == Mode::Historical && request.operation.is_some())
            || request.scope.as_ref().is_some_and(|s| {
                s.is_empty()
                    || s.len() > 128
                    || !s
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"-_:.".contains(&b))
            })
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    async fn current(&self, session: &NativeSession) -> Result<HeadSnapshot> {
        let snapshot = session.snapshot().await.map_err(reads::Error::from)?;
        Ok(snapshot.manifest().into())
    }
    async fn status_inner(&self, session: &Arc<NativeSession>) -> Result<Status> {
        self.status_backend(session, &self.backend()?).await
    }
    async fn status_backend(
        &self,
        session: &Arc<NativeSession>,
        backend: &Backend,
    ) -> Result<Status> {
        let access = session.adapter_source();
        let state = backend.search.status(&access).await?;
        let health = backend.health.borrow().clone();
        let current = self.current(session).await?;
        for position in [
            state.durable.as_ref().map(|c| &c.source),
            state.visible.as_ref().map(|c| &c.source),
            health.repository_position.as_ref(),
            health.applied.as_ref().map(|c| &c.source),
        ]
        .into_iter()
        .flatten()
        {
            if position.repository != current.repo_id {
                return Err(a::Error::Denied.into());
            }
            if position.epoch != current.epoch {
                return Err(Error::EpochChanged);
            }
            if position.position.0 > current.published_seq.0 {
                return Err(Error::Unavailable);
            }
        }
        let indexed_position = state.visible.as_ref().map(|c| c.source.position);
        Ok(Status {
            version: 1,
            repository: current.repo_id,
            epoch: current.epoch,
            repository_position: current.published_seq,
            durable_position: state.durable.as_ref().map(|c| c.source.position),
            indexed_position,
            index_generation: state.visible.as_ref().map(|c| c.artifact),
            is_stale: indexed_position != Some(current.published_seq),
            degraded: state.degraded || health.degraded || backend.health.has_changed().is_err(),
        })
    }
    pub async fn status(&self, session: Arc<NativeSession>) -> Result<Status> {
        let _permit = self.slots.try_acquire().map_err(|_| Error::Busy)?;
        within(self.limits.timeout, async {
            self.cursors.prune()?;
            self.status_inner(&session).await
        })
        .await
    }
    async fn wait_reader(
        &self,
        session: &Arc<NativeSession>,
        request: &Request,
    ) -> Result<PinnedReader> {
        let deadline = Instant::now() + Duration::from_millis(request.wait_millis);
        loop {
            let backend = self.backend()?;
            let status = self.status_backend(session, &backend).await?;
            if status
                .indexed_position
                .is_some_and(|p| request.minimum_position.is_none_or(|min| p.0 >= min.0))
            {
                let pin = backend.search.pin(&session.adapter_source()).await?;
                // A concurrent restore/config switch may have replaced the generation.
                if request
                    .minimum_position
                    .is_none_or(|min| pin.checkpoint().source.position.0 >= min.0)
                {
                    return Ok(pin);
                }
            }
            if request.minimum_position.is_none() {
                return Err(Error::NotReady);
            }
            if Instant::now() >= deadline {
                return Err(Error::IndexLag(Box::new(status)));
            }
            tokio::time::sleep_until((Instant::now() + self.limits.poll).min(deadline)).await;
        }
    }
    pub async fn begin(&self, session: Arc<NativeSession>, request: Request) -> Result<Page> {
        let _permit = self.slots.try_acquire().map_err(|_| Error::Busy)?;
        within(self.limits.timeout, async {
            self.current(&session).await?;
            self.validate(&request)?;
            self.cursors.prune()?;
            let reader = self.wait_reader(&session, &request).await?;
            let view = match request.mode {
                Mode::Historical => SearchView::Historical,
                Mode::VisibleCommits => {
                    let read = match session
                        .open_read(request.operation, self.limits.read)
                        .await?
                    {
                        OpenRead::Pinned(read) => Arc::new(read),
                        OpenRead::SelectHead(heads) => {
                            return Err(Error::SelectHead(Box::new(heads)));
                        }
                    };
                    SearchView::VisibleCommits(read.search_visibility(&self.reader).await?)
                }
            };
            let page = self.execute(&session, &reader, &view, &request, 0).await?;
            let (after, truncated) = self.continuation(&page);
            let mut response = self.response(&session, page, truncated).await?;
            let retained = if let Some(after) = after {
                let (token, id) = self.cursors.retain(reader, view, request, after)?;
                response.next_cursor = Some(token);
                Some(id)
            } else {
                None
            };
            if let Err(error) = crate::bounded_json(&response, self.limits.query.output_bytes) {
                if let Some(id) = retained {
                    self.cursors.release(id)?;
                }
                return Err(error.into());
            }
            Ok(response)
        })
        .await
    }
    pub async fn resume(
        &self,
        session: Arc<NativeSession>,
        cursor: &str,
        request: Request,
    ) -> Result<Page> {
        let _permit = self.slots.try_acquire().map_err(|_| Error::Busy)?;
        within(self.limits.timeout, async {
            let current = self.current(&session).await?;
            self.validate(&request)?;
            let (claims, entry) = self.cursors.lookup(cursor, &current, &request)?;
            let page = self
                .execute(
                    &session,
                    &entry.reader,
                    &entry.view,
                    &request,
                    claims.offset,
                )
                .await?;
            let (after, truncated) = self.continuation(&page);
            let mut response = self.response(&session, page, truncated).await?;
            response.next_cursor = self.cursors.next(&claims, &entry, after)?;
            crate::bounded_json(&response, self.limits.query.output_bytes)?;
            Ok(response)
        })
        .await
    }
    async fn execute(
        &self,
        session: &Arc<NativeSession>,
        reader: &PinnedReader,
        view: &SearchView,
        request: &Request,
        offset: usize,
    ) -> Result<crate::Page> {
        Ok(reader
            .query(
                &session.adapter_source(),
                SearchRequest {
                    text: &request.q,
                    offset,
                    limit: request.page_size,
                    view: view.clone(),
                    scope: request.scope.as_deref(),
                },
                self.limits.query,
            )
            .await?)
    }
    fn continuation(&self, page: &crate::Page) -> (Option<usize>, bool) {
        match page.next_offset {
            Some(next) if next > self.limits.query.max_offset => (None, true),
            next => (next, false),
        }
    }
    async fn response(
        &self,
        session: &Arc<NativeSession>,
        page: crate::Page,
        truncated: bool,
    ) -> Result<Page> {
        // Continuations authorize against native state, even after disable/replacement.
        let current = self.current(session).await?;
        let degraded = self
            .active
            .read()
            .unwrap()
            .as_ref()
            .is_some_and(|backend| backend.health.borrow().degraded);
        if current.repo_id != page.checkpoint.source.repository {
            return Err(a::Error::Denied.into());
        }
        if current.epoch != page.checkpoint.source.epoch {
            return Err(Error::EpochChanged);
        }
        if page.checkpoint.source.position.0 > current.published_seq.0 {
            return Err(Error::Unavailable);
        }
        Ok(Page {
            version: 1,
            repository: current.repo_id,
            epoch: current.epoch,
            repository_position: current.published_seq,
            indexed_position: page.checkpoint.source.position,
            index_generation: page.checkpoint.artifact,
            selected_view: page.selected_view,
            scope_snapshots: page.scopes,
            results: page.hits,
            is_stale: page.checkpoint.source.position != current.published_seq,
            degraded,
            next_cursor: None,
            truncated,
        })
    }
}
async fn within<T>(
    duration: Duration,
    future: impl std::future::Future<Output = Result<T>>,
) -> Result<T> {
    let deadline = Instant::now() + duration;
    let result = timeout_at(deadline, future)
        .await
        .map_err(|_| Error::Limit)?;
    if Instant::now() >= deadline {
        return Err(Error::Limit);
    }
    result
}
