//! Bounded process-local tasks for exact-operation native index preparation.
use crate::{
    auth::Permission,
    native::NativeSession,
    reads::{self, OpenRead, ReadLimits, ReadSnapshot},
};
use jj_store_jj::query::{self as q, Cancellation, GraphExecutor, IndexReport};
use jj_store_model as m;
use serde::Serialize;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{sync::Semaphore, time::Instant};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("index task is not authorized")]
    Denied,
    #[error("index task expired or its serving instance changed; request preparation again")]
    Reset,
    #[error("repository epoch changed; select a new snapshot")]
    EpochChanged,
    #[error("index task capacity is busy")]
    Busy,
    #[error("index task resource limit exceeded")]
    Limit,
    #[error("index task service is unavailable")]
    Unavailable,
    #[error(transparent)]
    Read(#[from] reads::Error),
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Clone, Copy)]
pub struct Limits {
    pub running: usize,
    pub retained: usize,
    pub ttl: Duration,
    pub preparation_timeout: Duration,
    pub read: ReadLimits,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            running: 2,
            retained: 32,
            ttl: Duration::from_secs(300),
            preparation_timeout: Duration::from_secs(3600),
            read: ReadLimits::default(),
        }
    }
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "IndexState")]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum State {
    Running,
    Canceling,
    Ready { index: IndexReport },
    Failed { error: q::Error },
    Canceled,
}
impl State {
    fn live(&self) -> bool {
        matches!(self, Self::Running | Self::Canceling)
    }
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "IndexTask")]
pub struct Task {
    pub version: u32,
    pub id: String,
    pub snapshot: ReadSnapshot,
    #[serde(flatten)]
    pub state: State,
}
struct Entry {
    task: Task,
    owner: m::PrincipalId,
    expires: Instant,
    cancellation: Cancellation,
}
impl Entry {
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
            if now < entry.expires {
                return true;
            }
            // A live job continues to own its inventory, worker and task slot
            // until actual exit, even when status already reports expiration.
            entry.cancel();
            entry.task.state.live()
        });
    }
    fn existing(
        &self,
        repository: m::RepositoryId,
        epoch: m::EpochId,
        operation: m::OperationId,
        owner: m::PrincipalId,
    ) -> Option<Task> {
        self.0
            .values()
            .find(|e| {
                e.owner == owner
                    && e.task.snapshot.authority.repo_id == repository
                    && e.task.snapshot.authority.epoch == epoch
                    && e.task.snapshot.operation_id == operation
                    && matches!(e.task.state, State::Running)
                    && Instant::now() < e.expires
            })
            .map(|e| e.task.clone())
    }
}
/// Share one instance across repositories. Completed entries retain only owned
/// snapshot/status data; credentials and complete inventories live in running jobs.
pub struct IndexTasks {
    executor: GraphExecutor,
    limits: Limits,
    slots: Arc<Semaphore>,
    registry: Arc<Mutex<Registry>>,
}
impl Drop for IndexTasks {
    fn drop(&mut self) {
        for entry in self
            .registry
            .lock()
            .expect("index task registry")
            .0
            .values_mut()
        {
            entry.cancel();
        }
    }
}
impl IndexTasks {
    pub fn new(executor: GraphExecutor, limits: Limits) -> Result<Self> {
        if !(1..=64).contains(&limits.running)
            || !(1..=1024).contains(&limits.retained)
            || limits.ttl.is_zero()
            || limits.ttl > Duration::from_secs(3600)
            || limits.preparation_timeout.is_zero()
            || limits.preparation_timeout > Duration::from_secs(3600)
            || limits.read.max_objects == 0
            || limits.read.max_objects > 2_000_000
            || limits.read.timeout.is_zero()
            || limits.read.timeout > Duration::from_secs(300)
        {
            return Err(Error::Limit);
        }
        Ok(Self {
            executor,
            limits,
            slots: Arc::new(Semaphore::new(limits.running)),
            registry: Default::default(),
        })
    }
    async fn authorize(&self, session: &NativeSession) -> Result<(m::PrincipalId, m::EpochId)> {
        let deadline = Instant::now()
            .checked_add(self.limits.read.timeout)
            .ok_or(Error::Limit)?;
        let result = tokio::time::timeout_at(deadline, async {
            let principal = session.principal(Permission::Read).await.map_err(map_wal)?;
            let snapshot = session.snapshot().await.map_err(map_wal)?;
            Ok((principal.id(), snapshot.manifest().epoch))
        })
        .await
        .map_err(|_| Error::Limit)?;
        if Instant::now() >= deadline {
            return Err(Error::Limit);
        }
        result
    }
    pub async fn start(
        &self,
        session: Arc<NativeSession>,
        operation: m::OperationId,
    ) -> Result<Task> {
        let (owner, epoch) = self.authorize(&session).await?;
        {
            let mut registry = self.registry.lock().expect("index task registry");
            registry.prune();
            if let Some(task) = registry.existing(session.repository, epoch, operation, owner) {
                return Ok(task);
            }
            if registry.0.len() >= self.limits.retained {
                return Err(Error::Busy);
            }
        }
        // Admission covers snapshot capture too. A duplicate running task is
        // returned above without allocating another inventory or queued job.
        let slot = self
            .slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| Error::Busy)?;
        let OpenRead::Pinned(read) = session.open_read(Some(operation), self.limits.read).await?
        else {
            return Err(Error::Unavailable);
        };
        let read = Arc::new(read);
        let snapshot = read.snapshot().await?.clone();
        if snapshot.authority.epoch != epoch {
            return Err(Error::EpochChanged);
        }
        let mut random = [0; 16];
        getrandom::fill(&mut random).map_err(|_| Error::Unavailable)?;
        let id = hex::encode(random);
        let cancellation = Cancellation::default();
        let expires = Instant::now()
            .checked_add(self.limits.preparation_timeout)
            .ok_or(Error::Limit)?;
        let task = Task {
            version: 1,
            id: id.clone(),
            snapshot,
            state: State::Running,
        };
        {
            let mut registry = self.registry.lock().expect("index task registry");
            registry.prune();
            // Another admitted snapshot capture may have installed this job.
            if let Some(task) = registry.existing(session.repository, epoch, operation, owner) {
                return Ok(task);
            }
            if registry.0.len() >= self.limits.retained {
                return Err(Error::Busy);
            }
            if registry.0.contains_key(&id) {
                return Err(Error::Unavailable);
            }
            registry.0.insert(
                id.clone(),
                Entry {
                    task: task.clone(),
                    owner,
                    expires,
                    cancellation: cancellation.clone(),
                },
            );
        }
        let registry = self.registry.clone();
        let executor = self.executor.clone();
        let completed_ttl = self.limits.ttl;
        tokio::spawn(async move {
            let signal = cancellation.clone();
            // Supervise even a panic so the registry cannot retain a phantom
            // running job. This supervisor retains no credential or inventory.
            let mut worker = tokio::spawn(async move {
                let _slot = slot;
                read.prepare_index(&executor, signal).await
            });
            let outcome = tokio::select! {
                result = &mut worker => result,
                _ = tokio::time::sleep_until(expires) => {
                    cancellation.cancel();
                    worker.await
                }
            }
            .unwrap_or(Err(q::Error::Unavailable));
            let mut registry = registry.lock().expect("index task registry");
            if let Some(entry) = registry.0.get_mut(&id) {
                if Instant::now() >= entry.expires {
                    registry.0.remove(&id);
                    return;
                }
                entry.task.state = match outcome {
                    _ if matches!(entry.task.state, State::Canceling) => State::Canceled,
                    Ok(index) => State::Ready { index },
                    Err(q::Error::Canceled) => State::Canceled,
                    Err(error) => State::Failed { error },
                };
                entry.expires = Instant::now() + completed_ttl;
            }
            registry.prune();
        });
        Ok(task)
    }
    pub async fn status(&self, session: Arc<NativeSession>, id: &str) -> Result<Task> {
        self.inspect(session, id, false).await
    }
    pub async fn cancel(&self, session: Arc<NativeSession>, id: &str) -> Result<Task> {
        self.inspect(session, id, true).await
    }
    async fn inspect(&self, session: Arc<NativeSession>, id: &str, cancel: bool) -> Result<Task> {
        let (principal, epoch) = self.authorize(&session).await?;
        if id.len() != 32 || !id.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(Error::Reset);
        }
        let mut registry = self.registry.lock().expect("index task registry");
        registry.prune();
        let entry = registry.0.get_mut(id).ok_or(Error::Reset)?;
        if entry.task.snapshot.authority.repo_id != session.repository {
            return Err(Error::Denied);
        }
        if entry.task.snapshot.authority.epoch != epoch {
            return Err(Error::EpochChanged);
        }
        if Instant::now() >= entry.expires {
            return Err(Error::Reset);
        }
        if cancel {
            if entry.owner != principal {
                return Err(Error::Denied);
            }
            entry.cancel();
        }
        Ok(entry.task.clone())
    }
}
fn map_wal(error: jj_store_wal::Error) -> Error {
    match error {
        jj_store_wal::Error::Denied => Error::Denied,
        jj_store_wal::Error::Limit(_) => Error::Limit,
        _ => Error::Unavailable,
    }
}
