//! Immutable copy relations and checked roots; credentials never enter slots.
use super::*;
use std::sync::{Mutex, OnceLock};
use wal::{inventory::SourceScope, records::RecordRef};

pub(super) const HEADER: usize = 1024 * 1024;
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct Key {
    scope: SourceScope,
    epoch: m::EpochId,
    root: Option<RecordRef>,
}
impl Key {
    pub fn new(scope: SourceScope, selected: &ManifestSnapshot) -> Self {
        Self {
            scope,
            epoch: selected.manifest().epoch,
            root: selected.manifest().inventory,
        }
    }
    fn namespace(self, other: Self) -> bool {
        self.scope == other.scope && self.epoch == other.epoch
    }
}
pub(super) struct Graph {
    pub index: index::Index,
    pub _lease: super::super::cache::Lease,
}
pub(super) struct Data {
    pub selected: ManifestSnapshot,
    pub graph: Arc<Graph>,
    pub _lease: super::super::cache::Lease,
}
struct Entry {
    key: Key,
    gate: Arc<tokio::sync::Mutex<()>>,
    data: Option<Arc<Data>>,
    created: tokio::time::Instant,
    touched: tokio::time::Instant,
}
pub(super) struct Cache {
    entries: Mutex<Vec<Entry>>,
    budget: Arc<super::super::cache::Budget>,
    shared_ids: bool,
    roots: usize,
    ttl: Duration,
}
pub(super) fn shared() -> &'static Cache {
    static CACHE: OnceLock<Cache> = OnceLock::new();
    CACHE.get_or_init(|| Cache {
        entries: Mutex::new(Vec::new()),
        budget: super::super::cache::shared().budget.clone(),
        shared_ids: true,
        roots: 32,
        ttl: Duration::from_secs(300),
    })
}
type CacheEntry = (Option<Arc<Data>>, Arc<tokio::sync::Mutex<()>>);
impl Cache {
    pub fn entry(&self, key: Key) -> query::Result<CacheEntry> {
        let now = tokio::time::Instant::now();
        let mut entries = self.entries.lock().unwrap();
        entries.retain(|entry| {
            now.saturating_duration_since(entry.created) < self.ttl
                || Arc::strong_count(&entry.gate) > 1
        });
        if let Some(entry) = entries.iter_mut().find(|entry| entry.key == key) {
            entry.touched = now;
            let data = (now.saturating_duration_since(entry.created) < self.ttl)
                .then(|| entry.data.clone())
                .flatten();
            return Ok((data, entry.gate.clone()));
        }
        while entries.len() >= self.roots {
            let oldest = entries
                .iter()
                .enumerate()
                .filter(|(_, entry)| Arc::strong_count(&entry.gate) == 1)
                .min_by_key(|(_, entry)| entry.touched)
                .map(|(i, _)| i)
                .ok_or(query::Error::IndexBudget)?;
            entries.swap_remove(oldest);
        }
        let gate = Arc::new(tokio::sync::Mutex::new(()));
        entries.push(Entry {
            key,
            gate: gate.clone(),
            data: None,
            created: now,
            touched: now,
        });
        Ok((None, gate))
    }
    pub fn ready(&self, key: Key) -> Option<Arc<Data>> {
        let now = tokio::time::Instant::now();
        self.entries
            .lock()
            .unwrap()
            .iter()
            .find(|entry| {
                entry.key == key && now.saturating_duration_since(entry.created) < self.ttl
            })
            .and_then(|entry| entry.data.clone())
    }
    pub fn base(&self, key: Key, revision: u64) -> Option<Arc<Data>> {
        let now = tokio::time::Instant::now();
        self.entries
            .lock()
            .unwrap()
            .iter()
            .filter(|entry| {
                key.namespace(entry.key) && now.saturating_duration_since(entry.created) < self.ttl
            })
            .filter_map(|entry| entry.data.as_ref())
            .filter(|data| data.selected.manifest().revision <= revision)
            .max_by_key(|data| data.selected.manifest().revision)
            .cloned()
    }
    fn clear_idle(&self) {
        self.entries
            .lock()
            .unwrap()
            .retain(|entry| Arc::strong_count(&entry.gate) > 1);
        if self.shared_ids {
            super::super::cache::shared().clear();
        }
    }
    pub fn reserve(&self, bytes: usize) -> query::Result<super::super::cache::Lease> {
        self.budget
            .reserve(bytes)
            .or_else(|| {
                self.clear_idle();
                self.budget.reserve(bytes)
            })
            .ok_or(query::Error::IndexBudget)
    }
    pub fn grow(&self, lease: &mut super::super::cache::Lease, bytes: usize) -> query::Result<()> {
        lease
            .resize(bytes)
            .or_else(|| {
                self.clear_idle();
                lease.resize(bytes)
            })
            .ok_or(query::Error::IndexBudget)
    }
    pub fn finish(&self, key: Key, gate: &Arc<tokio::sync::Mutex<()>>, data: Arc<Data>) {
        let mut entries = self.entries.lock().unwrap();
        if let Some(entry) = entries
            .iter_mut()
            .find(|entry| entry.key == key && Arc::ptr_eq(&entry.gate, gate))
        {
            entry.data = Some(data);
            entry.created = tokio::time::Instant::now();
            entry.touched = entry.created;
        }
    }
}

#[cfg(test)]
mod tests;
