//! Exact-operation local history tasks and credential-free prepared generations.
use crate::local_browser::code as q;
use jj_store_jj::query::{Cancellation, cache};
use jj_store_model as m;
use serde::Serialize;
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{sync::Semaphore, time::Instant};

#[derive(Clone, Copy)]
pub struct Limits {
    pub retained: usize,
    pub completed_ttl: Duration,
    pub preparation: q::preparation::Limits,
    pub cache: cache::Policy,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            retained: 32,
            completed_ttl: Duration::from_secs(300),
            preparation: Default::default(),
            cache: Default::default(),
        }
    }
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalIndexState")]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum State {
    Running,
    Canceling,
    Ready { stats: q::Stats },
    Failed { error: q::Error },
    Canceled,
}
impl State {
    fn live(&self) -> bool {
        matches!(self, Self::Running | Self::Canceling)
    }
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
pub struct LocalIndexTask {
    pub version: u32,
    pub id: String,
    pub snapshot: q::Snapshot,
    #[serde(flatten)]
    pub state: State,
}
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("local index task expired or serving instance changed")]
    Reset,
    #[error(transparent)]
    Query(#[from] q::Error),
}
pub type Result<T> = std::result::Result<T, Error>;
struct Entry {
    task: LocalIndexTask,
    metadata: PathBuf,
    expires: Instant,
    used: Instant,
    cancellation: Cancellation,
    prepared: Option<q::preparation::Retained>,
}
impl Drop for Entry {
    fn drop(&mut self) {
        self.cancellation.cancel();
    }
}
impl Entry {
    fn matches(&self, access: &dyn q::Access, operation: m::OperationId) -> bool {
        self.task.snapshot.repository == access.repository()
            && self.task.snapshot.generation == access.generation()
            && self.task.snapshot.operation == operation
            && self.metadata == access.metadata_path()
    }
    fn cancel(&mut self) {
        if self.task.state.live() {
            self.task.state = State::Canceling;
            self.cancellation.cancel();
        }
    }
}
#[derive(Default)]
struct Registry(BTreeMap<String, Entry>);
impl Registry {
    fn prune(&mut self) {
        let now = Instant::now();
        self.0.retain(|_, entry| {
            if entry.prepared.as_ref().is_some_and(|p| !p.present()) {
                return false;
            }
            if now < entry.expires {
                return true;
            }
            entry.cancel();
            entry.task.state.live()
        });
    }
    fn existing(
        &self,
        access: &dyn q::Access,
        operation: m::OperationId,
    ) -> Option<LocalIndexTask> {
        self.0.values().find_map(|e| {
            if !e.matches(access, operation) || Instant::now() >= e.expires {
                return None;
            }
            if matches!(e.task.state, State::Running) {
                return Some(e.task.clone());
            }
            // Hold the observed generation until the Ready observation is
            // built, even if another allocator starts reclaiming now.
            let _prepared = e.prepared.as_ref()?.get()?;
            Some(e.task.clone())
        })
    }
    fn evict(&mut self, repository: Option<m::RepositoryId>, prepared_only: bool) -> bool {
        let id = self
            .0
            .iter()
            .filter(|(_, e)| {
                !e.task.state.live()
                    && (!prepared_only || e.prepared.is_some())
                    && repository.is_none_or(|r| e.task.snapshot.repository == r)
            })
            .min_by_key(|(_, e)| e.used)
            .map(|(id, _)| id.clone());
        id.is_some_and(|id| self.0.remove(&id).is_some())
    }
}

pub struct IndexTasks {
    changes: crate::freshness::Signals,
    executor: q::Executor,
    root: PathBuf,
    limits: Limits,
    slots: Arc<Semaphore>,
    registry: Arc<Mutex<Registry>>,
}
impl Drop for IndexTasks {
    fn drop(&mut self) {
        let mut registry = self.registry.lock().expect("local index registry");
        for entry in registry.0.values_mut() {
            entry.cancel();
        }
        registry.0.clear();
    }
}
impl IndexTasks {
    pub fn freshness(&self) -> tokio::sync::broadcast::Receiver<crate::freshness::Change> {
        self.changes.subscribe()
    }
    pub fn new(executor: q::Executor, root: PathBuf, limits: Limits) -> Result<Self> {
        if !root.is_absolute()
            || !(1..=64).contains(&limits.retained)
            || limits.completed_ttl.is_zero()
            || limits.completed_ttl > Duration::from_secs(3600)
            || limits.preparation.timeout.is_zero()
            || limits.preparation.timeout > Duration::from_secs(3600)
        {
            return Err(q::Error::Limit.into());
        }
        Ok(Self {
            changes: Default::default(),
            executor,
            root,
            limits,
            slots: Arc::new(Semaphore::new(1)),
            registry: Arc::default(),
        })
    }
    async fn authorize(access: &dyn q::Access) -> Result<()> {
        tokio::time::timeout(Duration::from_secs(10), access.authorize())
            .await
            .map_err(|_| q::Error::Limit)??;
        Ok(())
    }
    pub async fn start(
        &self,
        access: Arc<dyn q::Access>,
        operation: m::OperationId,
    ) -> Result<LocalIndexTask> {
        Self::authorize(access.as_ref()).await?;
        {
            let mut registry = self.registry.lock().expect("local index registry");
            registry.prune();
            if let Some(task) = registry.existing(access.as_ref(), operation) {
                return Ok(task);
            }
        }
        // Bounds snapshot capture and the complete task, independently of the
        // executor's single shared native/local preparation worker.
        let slot = self
            .slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| q::Error::Busy)?;
        self.executor
            .read(
                access.clone(),
                operation,
                q::Request::Operation { operation },
            )
            .await?;
        Self::authorize(access.as_ref()).await?;
        let mut random = [0; 16];
        getrandom::fill(&mut random).map_err(|_| q::Error::Unavailable)?;
        let id = hex::encode(random);
        let task = LocalIndexTask {
            version: 1,
            id: id.clone(),
            snapshot: q::Snapshot {
                version: 1,
                profile: q::PROFILE,
                repository: access.repository(),
                generation: access.generation(),
                operation,
            },
            state: State::Running,
        };
        let cancellation = Cancellation::default();
        let expires = Instant::now() + self.limits.preparation.timeout;
        {
            let mut registry = self.registry.lock().expect("local index registry");
            registry.prune();
            if registry.0.len() >= self.limits.retained && !registry.evict(None, false) {
                return Err(q::Error::Busy.into());
            }
            if registry.0.contains_key(&id) {
                return Err(q::Error::Unavailable.into());
            }
            registry.0.insert(
                id.clone(),
                Entry {
                    task: task.clone(),
                    metadata: access.metadata_path().into(),
                    expires,
                    used: Instant::now(),
                    cancellation: cancellation.clone(),
                    prepared: None,
                },
            );
        }
        let registry = self.registry.clone();
        let changes = self.changes.clone();
        let repository = task.snapshot.repository;
        let worker_registry = registry.clone();
        let executor = self.executor.clone();
        let limits = self.limits;
        let root = self.root.clone();
        tokio::spawn(async move {
            let signal = cancellation.clone();
            let mut worker = tokio::spawn(async move {
                let _slot = slot;
                let repository = access.repository();
                let reservation = loop {
                    if signal.is_cancelled() {
                        return Err(q::Error::Canceled);
                    }
                    let path = root.clone();
                    let outcome = tokio::task::spawn_blocking(move || {
                        cache::Reservation::acquire(&path, repository, limits.cache)
                    })
                    .await
                    .map_err(|_| q::Error::Unavailable)?;
                    match outcome {
                        Ok(reservation) => break Arc::new(reservation),
                        Err(q::Error::Limit) => {
                            let mut registry =
                                worker_registry.lock().expect("local index registry");
                            if !registry.evict(Some(repository), true)
                                && !registry.evict(None, true)
                            {
                                return Err(q::Error::Limit);
                            }
                        }
                        Err(error) => return Err(error),
                    }
                };
                executor
                    .prepare_reserved(access, operation, reservation, limits.preparation, signal)
                    .await
            });
            let result = tokio::select! {
                result = &mut worker => result,
                _ = tokio::time::sleep_until(expires) => { cancellation.cancel(); worker.await }
            }
            .unwrap_or(Err(q::Error::Unavailable));
            let completed_expires = Instant::now() + limits.completed_ttl;
            let retired = Cancellation::default();
            {
                let mut registry = registry.lock().expect("local index registry");
                if registry
                    .0
                    .get(&id)
                    .is_none_or(|entry| Instant::now() >= entry.expires)
                {
                    registry.0.remove(&id);
                    changes.notify(
                        Some(repository),
                        crate::freshness::Domain::IndexTasks,
                        Some(id.clone()),
                    );
                    return;
                }
                if let Some(entry) = registry.0.get_mut(&id) {
                    entry.task.state = match result {
                        _ if cancellation.is_cancelled() => State::Canceled,
                        Ok(prepared) => {
                            let stats = prepared.stats.clone();
                            entry.prepared = Some(Arc::new(prepared).retain());
                            State::Ready { stats }
                        }
                        Err(q::Error::Canceled) => State::Canceled,
                        Err(error) => State::Failed { error },
                    };
                    entry.cancellation = retired.clone();
                    entry.expires = completed_expires;
                    entry.used = Instant::now();
                    changes.notify(
                        Some(repository),
                        crate::freshness::Domain::IndexTasks,
                        Some(id.clone()),
                    );
                }
            }
            // Expiry frees resident and disk capacity without requiring polling.
            tokio::select! {
                _ = retired.cancelled() => {},
                _ = tokio::time::sleep_until(completed_expires) => {
                    registry.lock().expect("local index registry").prune();
                    changes.notify(Some(repository), crate::freshness::Domain::IndexTasks, Some(id.clone()));
                }
            }
        });
        Ok(task)
    }
    pub async fn status(&self, access: Arc<dyn q::Access>, id: &str) -> Result<LocalIndexTask> {
        self.inspect(access, id, false).await
    }
    pub async fn cancel(&self, access: Arc<dyn q::Access>, id: &str) -> Result<LocalIndexTask> {
        self.inspect(access, id, true).await
    }
    async fn inspect(
        &self,
        access: Arc<dyn q::Access>,
        id: &str,
        cancel: bool,
    ) -> Result<LocalIndexTask> {
        Self::authorize(access.as_ref()).await?;
        if id.len() != 32 || !id.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(Error::Reset);
        }
        let mut registry = self.registry.lock().expect("local index registry");
        registry.prune();
        let entry = registry.0.get_mut(id).ok_or(Error::Reset)?;
        if entry.task.snapshot.repository != access.repository() {
            return Err(q::Error::Denied.into());
        }
        if entry.task.snapshot.generation != access.generation()
            || entry.metadata != access.metadata_path()
        {
            return Err(q::Error::EpochChanged.into());
        }
        if Instant::now() >= entry.expires {
            return Err(Error::Reset);
        }
        let _prepared = if matches!(entry.task.state, State::Ready { .. }) {
            Some(
                entry
                    .prepared
                    .as_ref()
                    .and_then(q::preparation::Retained::get)
                    .ok_or(Error::Reset)?,
            )
        } else {
            None
        };
        if cancel {
            entry.cancel();
        }
        Ok(entry.task.clone())
    }
    pub async fn graph(
        &self,
        access: Arc<dyn q::Access>,
        operation: m::OperationId,
        request: q::GraphRequest,
        page: q::GraphWindow,
    ) -> Result<q::GraphPage> {
        let response = self
            .read(access, operation, q::Request::Graph(request, page))
            .await?;
        let q::Response::Graph(page) = response else {
            return Err(q::Error::Invalid.into());
        };
        Ok(page)
    }
    pub async fn read(
        &self,
        access: Arc<dyn q::Access>,
        operation: m::OperationId,
        request: q::Request,
    ) -> Result<q::Response> {
        if !request.can_use_prepared_index() {
            return Err(q::Error::Invalid.into());
        }
        Self::authorize(access.as_ref()).await?;
        let source = match &request {
            q::Request::References(r) => r.operation,
            _ => operation,
        };
        let prepared = {
            let mut registry = self.registry.lock().expect("local index registry");
            registry.prune();
            registry
                .0
                .values_mut()
                .find(|entry| entry.matches(access.as_ref(), source) && entry.prepared.is_some())
                .and_then(|entry| {
                    entry.used = Instant::now();
                    entry
                        .prepared
                        .as_ref()
                        .and_then(q::preparation::Retained::get)
                })
        };
        let Some(prepared) = prepared else {
            if source != operation {
                self.executor
                    .validate_reference_operation(access, operation, source)
                    .await?;
            }
            return Err(q::Error::IndexRebuildRequired.into());
        };
        Ok(self
            .executor
            .read_prepared_at(access, operation, prepared, request)
            .await?)
    }
}
