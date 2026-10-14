//! Credential-free indexes keyed by audited immutable source roots, never query results.
use super::*;
use crate::auth::Clock;
use jj_store_model as m;
use jj_store_wal::records::RecordRef;
use std::{collections::BTreeMap, sync::Mutex};
use tokio::sync::OnceCell;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    source: u64,
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
pub(crate) struct Cache {
    clock: Arc<dyn Clock>,
    entries: Mutex<BTreeMap<Key, Entry>>,
    sources: crate::read_indexes::Sources,
    retained: Mutex<Vec<std::sync::Weak<index::Index>>>,
}
impl Cache {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self {
            clock,
            entries: Mutex::default(),
            sources: Default::default(),
            retained: Mutex::default(),
        }
    }
    pub(super) async fn get(
        &self,
        session: &NativeSession,
        epoch: m::EpochId,
        root: RecordRef,
    ) -> Result<Arc<index::Index>> {
        let now = self.clock.unix_seconds().map_err(|_| Error::Unavailable)?;
        let key = Key {
            source: self
                .sources
                .identify(&session.object_store)
                .map_err(|_| Error::Unavailable)?,
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
                let mut retained = self.retained.lock().map_err(|_| Error::Unavailable)?;
                retained.retain(|value| value.strong_count() != 0);
                let weak = Arc::downgrade(value);
                if !retained.iter().any(|prior| prior.ptr_eq(&weak)) {
                    retained.push(weak);
                }
                while retained
                    .iter()
                    .filter_map(std::sync::Weak::upgrade)
                    .map(|index| index.bytes)
                    .fold(0usize, usize::saturating_add)
                    > index::MAX_BYTES
                {
                    let victim = entries
                        .iter()
                        .filter(|(_, e)| e.cell.get().is_some())
                        .min_by_key(|(_, e)| e.used)
                        .map(|(key, _)| *key);
                    match victim {
                        Some(victim) => {
                            entries.remove(&victim);
                        }
                        None => return Err(Error::Limit),
                    }
                }
                Ok(value.clone())
            }
        }
    }
}
async fn load(session: &NativeSession, root: RecordRef) -> Result<Arc<index::Index>> {
    let epoch = session.snapshot().await?.manifest().epoch;
    let data = session
        .read_indexes
        .cache
        .get(session, epoch, root)
        .await
        .map_err(|e| match e {
            crate::collab_mutations::Error::Denied => Error::Denied,
            crate::collab_mutations::Error::Corrupt => Error::Corrupt,
            crate::collab_mutations::Error::Limit => Error::Limit,
            _ => Error::Unavailable,
        })?;
    use crate::collab_reads::materialization::Data;
    let mut value = match data.as_ref() {
        Data::Thread(data) => index::Index::threads(&data.state, &data.graph)?,
        Data::Work(data) => index::Index::work(&data.state)?,
        _ => return Err(Error::Corrupt),
    };
    value.accelerate(session, epoch, root).await;
    Ok(Arc::new(value))
}
