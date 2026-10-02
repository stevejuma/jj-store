//! Process-local validated data only. Authorization and snapshots stay request-bound.
use super::*;
use crate::{auth::Clock, collab_pages::Limits};
use std::sync::Mutex;
use tokio::sync::OnceCell;
use wal::records::RecordRef;

#[derive(Clone)]
pub enum ReadScope {
    All,
    Threads,
    Emojis,
    Reactions(ReactionTargetV1),
}
impl ReadScope {
    pub(super) fn domains(&self) -> [bool; 5] {
        match self {
            Self::All => [true; 5],
            Self::Threads => [true, false, false, false, false],
            Self::Emojis => [false, true, false, false, false],
            Self::Reactions(target) => match target {
                ReactionTargetV1::Thread { .. } | ReactionTargetV1::HumanMessage { .. } => {
                    [true, true, false, false, false]
                }
                ReactionTargetV1::ReviewThread { .. } | ReactionTargetV1::ReviewComment { .. } => {
                    [false, true, false, true, false]
                }
                ReactionTargetV1::WorkComment { .. } => [false, true, false, false, true],
                _ => [false, true, true, false, false],
            },
        }
    }
}
pub(super) struct ThreadData {
    pub state: ThreadStateV1,
    pub graph: CollabGraphV1,
    pub directory: BTreeMap<ThreadId, ThreadDirectoryMetadata>,
    pub bytes: usize,
}
impl ThreadData {
    pub fn empty() -> Self {
        Self {
            state: ThreadStateV1::empty(),
            graph: CollabGraphV1::default(),
            directory: BTreeMap::new(),
            bytes: 0,
        }
    }
}
pub(super) struct ReactionData {
    pub state: ReactionStateV1,
    pub graph: CollabGraphV1,
    pub targets: BTreeMap<CollabCatalogKey, Vec<CollabObjectRef>>,
    pub bytes: usize,
}
impl ReactionData {
    pub fn empty() -> Self {
        Self {
            state: ReactionStateV1::empty(),
            graph: CollabGraphV1::default(),
            targets: BTreeMap::new(),
            bytes: 0,
        }
    }
}
pub(super) enum Data {
    Thread(Arc<ThreadData>),
    Reaction(Arc<ReactionData>),
    Agent(Arc<wal::publication::AgentStoreSelection>),
    Review(Arc<wal::publication::ReviewStoreSelection>),
    Work(Arc<WorkData>),
}
pub(super) struct WorkData {
    pub state: wal::publication::WorkStateV1,
    pub objects: usize,
    pub bytes: usize,
}
impl Data {
    fn size(&self) -> (usize, usize) {
        match self {
            Self::Thread(d) => (d.graph.objects.len(), d.bytes),
            Self::Reaction(d) => (d.graph.objects.len(), d.bytes),
            Self::Agent(d) => (d.object_count(), 0),
            Self::Review(d) => (d.object_count(), 0),
            Self::Work(d) => (d.objects, d.bytes),
        }
    }
}
fn encoded_bytes(graph: &CollabGraphV1) -> Result<usize> {
    graph.objects.values().try_fold(0usize, |n, o| {
        n.checked_add(encode_object(o).map_err(model)?.len())
            .ok_or(Error::Limit)
    })
}
pub(super) async fn load(session: &NativeSession, root: RecordRef) -> Result<Arc<Data>> {
    use wal::records::RecordKind;
    let data = match root.kind {
        RecordKind::ThreadRoot => {
            let store = session.coordinator.native_thread_store(root).await?;
            let directory = crate::thread_directory::project(&store.state, &store.graph)?;
            let bytes = encoded_bytes(&store.graph)?;
            Data::Thread(Arc::new(ThreadData {
                state: store.state,
                graph: store.graph,
                directory,
                bytes,
            }))
        }
        RecordKind::ReactionRoot => {
            let store = session.coordinator.native_reaction_store(root).await?;
            let state = ReactionStateV1::read(session.repository, store.root, &store.graph)
                .map_err(model)?;
            let mut targets: BTreeMap<CollabCatalogKey, Vec<CollabObjectRef>> = BTreeMap::new();
            for reference in state.slots.values() {
                let CollabObjectV1::ReactionSlot(slot) =
                    store.graph.get(*reference).map_err(model)?
                else {
                    return Err(Error::Corrupt);
                };
                targets
                    .entry(slot.target.exact_key().map_err(model)?)
                    .or_default()
                    .push(*reference);
            }
            let bytes = encoded_bytes(&store.graph)?;
            Data::Reaction(Arc::new(ReactionData {
                state,
                graph: store.graph,
                targets,
                bytes,
            }))
        }
        RecordKind::AgentRoot => Data::Agent(Arc::new(
            session.coordinator.agent_store(root).await?.selection(),
        )),
        RecordKind::ReviewRoot => Data::Review(Arc::new(
            session.coordinator.review_store(root).await?.selection(),
        )),
        RecordKind::WorkRoot => {
            let store = session.coordinator.work_store(root).await?;
            let state = store.state();
            let bytes = serde_json::to_vec(&(&state.projects, &state.tasks, &state.comments))
                .map_err(|_| Error::Corrupt)?
                .len();
            Data::Work(Arc::new(WorkData {
                state: state.clone(),
                objects: store.object_count(),
                bytes,
            }))
        }
        _ => return Err(Error::Corrupt),
    };
    if data.size().0 > session.limits.inventory_nodes {
        return Err(Error::Limit);
    }
    Ok(Arc::new(data))
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    repository: m::RepositoryId,
    epoch: m::EpochId,
    root: RecordRef,
}
struct Entry {
    issued: u64,
    expires: u64,
    used: u64,
    cell: Arc<OnceCell<Arc<Data>>>,
}
/// The cache never owns a session, guard, actor policy or viewer-filtered result.
pub(crate) struct Cache {
    limits: Limits,
    clock: Arc<dyn Clock>,
    entries: Mutex<BTreeMap<Key, Entry>>,
}
impl Cache {
    pub fn new(limits: Limits, clock: Arc<dyn Clock>) -> Self {
        Self {
            limits,
            clock,
            entries: Mutex::default(),
        }
    }
    pub(super) async fn get(
        &self,
        session: &NativeSession,
        epoch: m::EpochId,
        root: RecordRef,
    ) -> Result<Arc<Data>> {
        let now = self.clock.unix_seconds().map_err(|_| Error::Unavailable)?;
        let key = Key {
            repository: session.repository,
            epoch,
            root,
        };
        let cell = {
            let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
            entries.retain(|_, e| e.issued <= now && now < e.expires);
            if let Some(entry) = entries.get_mut(&key) {
                entry.used = now;
                entry.cell.clone()
            } else {
                if entries.len() >= self.limits.snapshots {
                    if let Some(victim) = entries
                        .iter()
                        .filter(|(_, e)| e.cell.get().is_some())
                        .min_by_key(|(_, e)| e.used)
                        .map(|(k, _)| *k)
                    {
                        entries.remove(&victim);
                    }
                }
                let cell = Arc::new(OnceCell::new());
                // A query may need more roots than the retention capacity. If
                // every slot is initializing, validate this root request-locally
                // under the pager's existing concurrency and per-pin budgets.
                if entries.len() < self.limits.snapshots {
                    entries.insert(
                        key,
                        Entry {
                            issued: now,
                            expires: now
                                .checked_add(self.limits.ttl.as_secs())
                                .ok_or(Error::Limit)?,
                            used: now,
                            cell: cell.clone(),
                        },
                    );
                }
                cell
            }
        };
        // Cancellation leaves an empty bounded cell; a subsequent caller retries
        // initialization. Errors are never published as validated data.
        let value = cell.get_or_try_init(|| load(session, root)).await;
        let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
        match value {
            Err(error) => {
                if entries
                    .get(&key)
                    .is_some_and(|e| Arc::ptr_eq(&e.cell, &cell))
                {
                    entries.remove(&key);
                }
                Err(error.clone())
            }
            Ok(value) => {
                let (objects, bytes) = value.size();
                if objects > self.limits.objects || bytes > self.limits.encoded_collab_bytes {
                    if entries
                        .get(&key)
                        .is_some_and(|e| Arc::ptr_eq(&e.cell, &cell))
                    {
                        entries.remove(&key);
                    }
                    return Err(Error::Limit);
                }
                loop {
                    let (objects, bytes) = entries
                        .values()
                        .filter_map(|e| e.cell.get())
                        .map(|d| d.size())
                        .fold((0usize, 0usize), |(a, b), (c, d)| {
                            (a.saturating_add(c), b.saturating_add(d))
                        });
                    if objects <= self.limits.objects && bytes <= self.limits.encoded_collab_bytes {
                        break;
                    }
                    let victim = entries
                        .iter()
                        .filter(|(_, e)| e.cell.get().is_some())
                        .min_by_key(|(_, e)| e.used)
                        .map(|(k, _)| *k)
                        .ok_or(Error::Limit)?;
                    entries.remove(&victim);
                }
                Ok(value.clone())
            }
        }
    }
}
