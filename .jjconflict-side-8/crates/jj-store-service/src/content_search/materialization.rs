//! Credential-free indexes keyed by audited immutable source roots, never query results.
use super::*;
use crate::auth::Clock;
use jj_store_model as m;
use jj_store_wal::records::{RecordKind, RecordRef};
use std::{collections::BTreeMap, sync::Mutex};
use tokio::sync::OnceCell;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    repository: m::RepositoryId,
    epoch: m::EpochId,
    root: RecordRef,
}
struct Entry {
    used: u64,
    issued: u64,
    expires: u64,
    cell: Arc<OnceCell<Arc<index::Index>>>,
}
pub(super) struct Cache {
    clock: Arc<dyn Clock>,
    entries: Mutex<BTreeMap<Key, Entry>>,
}
impl Cache {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self {
            clock,
            entries: Mutex::default(),
        }
    }
    pub async fn get(
        &self,
        session: &NativeSession,
        epoch: m::EpochId,
        root: RecordRef,
    ) -> Result<Arc<index::Index>> {
        let now = self.clock.unix_seconds().map_err(|_| Error::Unavailable)?;
        let key = Key {
            repository: session.repository,
            epoch,
            root,
        };
        let cell = {
            let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
            entries.retain(|_, entry| entry.issued <= now && now < entry.expires);
            if let Some(entry) = entries.get_mut(&key) {
                entry.used = now;
                entry.cell.clone()
            } else {
                if entries.len() >= 64
                    && let Some(victim) = entries
                        .iter()
                        .filter(|(_, e)| e.cell.get().is_some())
                        .min_by_key(|(_, e)| e.used)
                        .map(|(key, _)| *key)
                {
                    entries.remove(&victim);
                }
                let cell = Arc::new(OnceCell::new());
                if entries.len() < 64 {
                    entries.insert(
                        key,
                        Entry {
                            used: now,
                            issued: now,
                            expires: now.checked_add(300).ok_or(Error::Limit)?,
                            cell: cell.clone(),
                        },
                    );
                }
                cell
            }
        };
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
                while entries
                    .values()
                    .filter_map(|e| e.cell.get())
                    .map(|index| index.bytes)
                    .fold(0usize, usize::saturating_add)
                    > index::MAX_BYTES
                {
                    let victim = entries
                        .iter()
                        .filter(|(_, e)| e.cell.get().is_some())
                        .min_by_key(|(_, e)| e.used)
                        .map(|(key, _)| *key)
                        .ok_or(Error::Limit)?;
                    entries.remove(&victim);
                }
                // Cached data never retains the first reader's inventory budget.
                if value.objects > session.limits.inventory_nodes {
                    return Err(Error::Limit);
                }
                Ok(value.clone())
            }
        }
    }
}
async fn load(session: &NativeSession, root: RecordRef) -> Result<Arc<index::Index>> {
    let value = match root.kind {
        RecordKind::ThreadRoot => {
            let store = session.coordinator.native_thread_store(root).await?;
            if store.graph.objects.len() > session.limits.inventory_nodes {
                return Err(Error::Limit);
            }
            index::Index::threads(&store.state, &store.graph)?
        }
        RecordKind::WorkRoot => {
            let store = session.coordinator.work_store(root).await?;
            let state = store.state();
            if state.projects.len() + state.tasks.len() + state.comments.len()
                > session.limits.inventory_nodes
            {
                return Err(Error::Limit);
            }
            index::Index::work(state)?
        }
        _ => return Err(Error::Corrupt),
    };
    Ok(Arc::new(value))
}
