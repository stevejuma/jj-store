//! One bounded native/local core-index owner. Existing events supply fresh sources.
mod jobs;
mod tasks;
use crate::{
    freshness::{self, Domain},
    reads::{CommittedRead, ReadSnapshot},
};
use jj_store_jj::{local_browser as local, query as q};
use jj_store_model as m;
use q::cache::managed::{Binding, Namespace};
use std::{
    collections::{BTreeMap, VecDeque},
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::time::Instant;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key {
    pub binding: Binding,
    pub operation: m::OperationId,
}
#[derive(Clone)]
pub(crate) struct NativeSource {
    pub snapshot: ReadSnapshot,
    pub access: Arc<dyn q::ReadAccess>,
    pub store: Arc<dyn jj_store_object::ObjectStore>,
}
#[derive(Clone)]
enum Source {
    Native(NativeSource),
    Local {
        access: Arc<dyn local::Access>,
        snapshot: local::Snapshot,
    },
}
impl Source {
    fn key(&self) -> Key {
        match self {
            Self::Native(s) => Key {
                binding: Binding::Native {
                    repository: s.access.repository(),
                    epoch: s.access.epoch(),
                },
                operation: s.access.operation(),
            },
            Self::Local { snapshot, .. } => Key {
                binding: Binding::Local {
                    repository: snapshot.repository,
                    generation: snapshot.generation,
                },
                operation: snapshot.operation,
            },
        }
    }
}
#[derive(Clone)]
pub enum Snapshot {
    Native(ReadSnapshot),
    Local(local::Snapshot),
}
#[derive(Clone)]
pub enum State {
    Running,
    Canceling,
    Ready(q::Stats),
    Failed(q::Error),
    Canceled,
}
impl State {
    fn live(&self) -> bool {
        matches!(self, Self::Running | Self::Canceling)
    }
}
#[derive(Clone)]
pub struct Task {
    pub id: String,
    pub key: Key,
    pub snapshot: Snapshot,
    pub state: State,
}
enum Ready {
    Native(Arc<q::code_index::Prepared>),
    Local(Arc<local::code_index::Prepared>),
}
impl Ready {
    fn native(&self) -> Option<Arc<q::code_index::Prepared>> {
        match self {
            Self::Native(p) => Some(p.clone()),
            _ => None,
        }
    }
    fn local(&self) -> Option<Arc<local::code_index::Prepared>> {
        match self {
            Self::Local(p) => Some(p.clone()),
            _ => None,
        }
    }
    fn stats(&self) -> q::Stats {
        match self {
            Self::Native(p) => p.stats.clone(),
            Self::Local(p) => q::Stats {
                metadata_nodes: p.stats.metadata_nodes,
                predicate_file_bytes: p.stats.predicate_file_bytes,
                predicate_lines: p.stats.predicate_lines,
                object_reads: p.stats.object_reads,
                object_bytes: p.stats.object_bytes,
                work: p.stats.work,
            },
        }
    }
}
struct Entry {
    task: Task,
    cancel: q::Cancellation,
    used: Instant,
    ready: Option<q::code_index::Retained<Ready>>,
    foreground: bool,
    expires: Instant,
}
#[derive(Default)]
struct Registry {
    entries: BTreeMap<Key, Entry>,
    pending: VecDeque<Source>,
    active: bool,
}
pub struct Indexes {
    graph: q::GraphExecutor,
    local: local::Executor,
    root: PathBuf,
    policy: q::cache::Policy,
    changes: freshness::Signals,
    registry: Mutex<Registry>,
}
impl Drop for Indexes {
    fn drop(&mut self) {
        for entry in self.registry.get_mut().unwrap().entries.values() {
            entry.cancel.cancel();
        }
    }
}
impl Indexes {
    pub fn new(
        graph: q::GraphExecutor,
        local: local::Executor,
        root: PathBuf,
        policy: q::cache::Policy,
    ) -> Arc<Self> {
        Arc::new(Self {
            graph,
            local,
            root,
            policy,
            changes: Default::default(),
            registry: Default::default(),
        })
    }
    pub fn freshness(&self) -> tokio::sync::broadcast::Receiver<freshness::Change> {
        self.changes.subscribe()
    }
    pub async fn retained(&self) -> q::Result<Vec<Binding>> {
        let root = self.root.clone();
        let policy = self.policy;
        tokio::task::spawn_blocking(move || Namespace::retained(&root, policy))
            .await
            .map_err(|_| q::Error::Unavailable)?
    }
    pub async fn start_native(self: &Arc<Self>, read: Arc<CommittedRead>) -> q::Result<Task> {
        self.enqueue(Source::Native(read.code_source().await?), true)
    }
    pub async fn start_local(
        self: &Arc<Self>,
        access: Arc<dyn local::Access>,
        operation: m::OperationId,
    ) -> q::Result<Task> {
        Self::authorize_local(access.as_ref()).await?;
        self.local
            .read(
                access.clone(),
                operation,
                local::Request::Operation { operation },
            )
            .await?;
        Self::authorize_local(access.as_ref()).await?;
        let snapshot = local::Snapshot {
            version: 1,
            profile: local::PROFILE,
            repository: access.repository(),
            generation: access.generation(),
            operation,
        };
        self.enqueue(Source::Local { access, snapshot }, true)
    }
    pub async fn maintain_native(self: &Arc<Self>, read: Arc<CommittedRead>) -> q::Result<Task> {
        self.enqueue(Source::Native(read.code_source().await?), false)
    }
    pub async fn maintain_local(
        self: &Arc<Self>,
        access: Arc<dyn local::Access>,
        operation: m::OperationId,
    ) -> q::Result<Task> {
        Self::authorize_local(access.as_ref()).await?;
        let snapshot = local::Snapshot {
            version: 1,
            profile: local::PROFILE,
            repository: access.repository(),
            generation: access.generation(),
            operation,
        };
        self.enqueue(Source::Local { access, snapshot }, false)
    }
    async fn authorize_local(access: &dyn local::Access) -> q::Result<()> {
        tokio::time::timeout(std::time::Duration::from_secs(10), access.authorize())
            .await
            .map_err(|_| q::Error::Limit)?
    }
    pub fn invalidate(&self, repository: m::RepositoryId) {
        let mut registry = self.registry.lock().unwrap();
        for (key, entry) in &mut registry.entries {
            if key.binding.repository() == repository {
                entry.cancel.cancel();
                entry.ready = None;
                entry.task.state = if entry.task.state.live() {
                    State::Canceling
                } else {
                    State::Canceled
                };
            }
        }
    }
    fn enqueue(self: &Arc<Self>, source: Source, foreground: bool) -> q::Result<Task> {
        let key = source.key();
        let mut registry = self.registry.lock().unwrap();
        let obsolete: Vec<_> = registry
            .pending
            .iter()
            .map(Source::key)
            .filter(|old| {
                old.binding.repository() == key.binding.repository()
                    && (old.binding != key.binding
                        || (!foreground
                            && old.operation != key.operation
                            && registry.entries.get(old).is_some_and(|e| !e.foreground)))
            })
            .collect();
        registry
            .pending
            .retain(|source| !obsolete.contains(&source.key()));
        for old in obsolete {
            if let Some(entry) = registry.entries.get_mut(&old) {
                entry.cancel.cancel();
                entry.task.state = State::Canceled;
            }
        }
        for (old, entry) in &mut registry.entries {
            if old.binding.repository() == key.binding.repository() && old.binding != key.binding {
                entry.cancel.cancel();
                entry.ready = None;
                entry.task.state = if entry.task.state.live() {
                    State::Canceling
                } else {
                    State::Canceled
                };
            }
        }
        if let Some(entry) = registry.entries.get_mut(&key)
            && (entry.task.state.live() || entry.ready.as_ref().is_some_and(|r| r.present()))
        {
            entry.foreground |= foreground;
            entry.used = Instant::now();
            if !entry.task.state.live() {
                entry.expires = Instant::now() + std::time::Duration::from_secs(300);
            }
            return Ok(entry.task.clone());
        }
        if registry.pending.len() >= 64 {
            return Err(q::Error::Busy);
        }
        while registry.entries.len() >= 64 {
            let oldest = registry
                .entries
                .iter()
                .filter(|(_, e)| !e.task.state.live())
                .min_by_key(|(_, e)| e.used)
                .map(|(k, _)| k.clone())
                .ok_or(q::Error::Busy)?;
            registry.entries.remove(&oldest);
        }
        let mut random = [0; 16];
        getrandom::fill(&mut random).map_err(|_| q::Error::Unavailable)?;
        let snapshot = match &source {
            Source::Native(s) => Snapshot::Native(s.snapshot.clone()),
            Source::Local { snapshot, .. } => Snapshot::Local(snapshot.clone()),
        };
        let task = Task {
            id: hex::encode(random),
            key: key.clone(),
            snapshot,
            state: State::Running,
        };
        registry.entries.insert(
            key,
            Entry {
                task: task.clone(),
                cancel: Default::default(),
                used: Instant::now(),
                ready: None,
                foreground,
                expires: Instant::now() + std::time::Duration::from_secs(3600),
            },
        );
        registry.pending.push_back(source);
        if !registry.active {
            registry.active = true;
            jobs::spawn(Arc::downgrade(self));
        }
        Ok(task)
    }
    pub fn needs(&self, key: &Key) -> bool {
        self.registry
            .lock()
            .unwrap()
            .entries
            .get(key)
            .is_none_or(|e| {
                matches!(e.task.state, State::Failed(_) | State::Canceled)
                    || (matches!(e.task.state, State::Ready(_))
                        && !e.ready.as_ref().is_some_and(|r| r.present()))
            })
    }
    pub fn inspect(&self, key: &Key, id: &str, cancel: bool) -> q::Result<Task> {
        let mut registry = self.registry.lock().unwrap();
        let entry = registry
            .entries
            .get_mut(key)
            .filter(|e| e.task.id == id)
            .ok_or(q::Error::NotFound)?;
        if Instant::now() >= entry.expires
            || (matches!(entry.task.state, State::Ready(_))
                && !entry.ready.as_ref().is_some_and(|r| r.present()))
        {
            return Err(q::Error::NotFound);
        }
        entry.used = Instant::now();
        if cancel && entry.task.state.live() {
            entry.cancel.cancel();
            entry.task.state = State::Canceling;
        }
        Ok(entry.task.clone())
    }
    fn ready(&self, key: &Key) -> q::Result<Arc<Ready>> {
        let mut registry = self.registry.lock().unwrap();
        let entry = registry
            .entries
            .get_mut(key)
            .ok_or(q::Error::IndexRebuildRequired)?;
        entry.used = Instant::now();
        entry
            .ready
            .as_ref()
            .and_then(|r| r.get())
            .ok_or(q::Error::IndexRebuildRequired)
    }
    pub async fn native_page(
        self: &Arc<Self>,
        read: Arc<CommittedRead>,
        request: q::Request,
        page: q::Page,
    ) -> q::Result<q::GraphPage> {
        let snapshot = read.snapshot().await.map_err(crate::reads::query_error)?;
        let key = Key {
            binding: Binding::Native {
                repository: snapshot.authority.repo_id,
                epoch: snapshot.authority.epoch,
            },
            operation: snapshot.operation_id,
        };
        match self.ready(&key) {
            Ok(ready) => {
                let Ready::Native(_) = ready.as_ref() else {
                    return Err(q::Error::Corrupt);
                };
                // Keep the shared ready handle alive throughout the source read.
                read.graph_code_index(
                    &self.graph,
                    ready.native().ok_or(q::Error::Corrupt)?,
                    request,
                    page,
                )
                .await
            }
            Err(q::Error::IndexRebuildRequired) => {
                self.start_native(read).await?;
                Err(q::Error::IndexRebuildRequired)
            }
            Err(e) => Err(e),
        }
    }
    pub async fn local_read(
        self: &Arc<Self>,
        access: Arc<dyn local::Access>,
        operation: m::OperationId,
        request: local::Request,
    ) -> q::Result<local::Response> {
        Self::authorize_local(access.as_ref()).await?;
        let key = Key {
            binding: Binding::Local {
                repository: access.repository(),
                generation: access.generation(),
            },
            operation,
        };
        match self.ready(&key) {
            Ok(ready) => {
                self.local
                    .read_code_index(access, ready.local().ok_or(q::Error::Corrupt)?, request)
                    .await
            }
            Err(q::Error::IndexRebuildRequired) => {
                if matches!(
                    request,
                    local::Request::Graph(local::GraphRequest::Log { .. }, _)
                        | local::Request::Commit { .. }
                ) {
                    self.start_local(access, operation).await?;
                }
                Err(q::Error::IndexRebuildRequired)
            }
            Err(e) => Err(e),
        }
    }
}
