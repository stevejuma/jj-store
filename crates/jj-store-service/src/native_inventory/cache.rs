//! Complete exact kind data only; source/authorization stays request-owned.
mod memory;
use super::*;
use memory::{Budget, Lease};
use std::{
    collections::BTreeSet,
    sync::{Mutex, OnceLock},
};
use wal::{inventory::SourceScope, records::RecordRef};

// Covers a decoded128KiB manifest, fixed map/Arc overhead and all set nodes.
const HEADER: usize = 1024 * 1024;
fn allocation(ids: usize) -> Option<usize> {
    ids.checked_mul(192)?.checked_add(HEADER)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct Key {
    scope: SourceScope,
    epoch: m::EpochId,
    root: Option<RecordRef>,
    copies: bool,
}
impl Key {
    pub fn new(scope: SourceScope, selected: &ManifestSnapshot, copies: bool) -> Self {
        Self {
            scope,
            epoch: selected.manifest().epoch,
            root: selected.manifest().inventory,
            copies,
        }
    }
    fn namespace(self, other: Self) -> bool {
        self.scope == other.scope && self.epoch == other.epoch && self.copies == other.copies
    }
}
pub(super) struct Data {
    pub selected: ManifestSnapshot,
    pub ids: BTreeSet<m::ObjectKey>,
    _lease: Lease,
}
struct Entry {
    key: Key,
    data: Arc<Data>,
    created: tokio::time::Instant,
    touched: tokio::time::Instant,
}
pub(super) struct Cache {
    entries: Mutex<Vec<Entry>>,
    budget: Arc<Budget>,
    roots: usize,
    ttl: Duration,
    pub permits: tokio::sync::Semaphore,
}
impl Default for Cache {
    fn default() -> Self {
        Self {
            entries: Mutex::new(Vec::new()),
            budget: Budget::new(64 * 1024 * 1024),
            roots: 32,
            ttl: Duration::from_secs(300),
            permits: tokio::sync::Semaphore::new(8),
        }
    }
}
pub(super) fn shared() -> &'static Cache {
    static CACHE: OnceLock<Cache> = OnceLock::new();
    CACHE.get_or_init(Cache::default)
}
impl Cache {
    pub fn lookup(&self, key: Key, revision: u64) -> (Option<Arc<Data>>, Option<Arc<Data>>) {
        self.lookup_at(key, revision, tokio::time::Instant::now())
    }
    fn lookup_at(
        &self,
        key: Key,
        revision: u64,
        now: tokio::time::Instant,
    ) -> (Option<Arc<Data>>, Option<Arc<Data>>) {
        let mut entries = self.entries.lock().unwrap();
        entries.retain(|entry| now.saturating_duration_since(entry.created) < self.ttl);
        if let Some(entry) = entries.iter_mut().find(|entry| entry.key == key) {
            entry.touched = now;
            return (Some(entry.data.clone()), None);
        }
        let baseline = entries
            .iter_mut()
            .filter(|entry| {
                key.namespace(entry.key) && entry.data.selected.manifest().revision <= revision
            })
            .max_by_key(|entry| entry.data.selected.manifest().revision)
            .map(|entry| {
                entry.touched = now;
                entry.data.clone()
            });
        (None, baseline)
    }
    pub fn discard(&self, key: Key) {
        self.entries
            .lock()
            .unwrap()
            .retain(|entry| entry.key != key);
    }
    fn reserve(&self, bytes: usize) -> Option<Lease> {
        if bytes > self.budget.limit {
            return None;
        }
        self.budget.reserve(bytes).or_else(|| {
            self.entries.lock().unwrap().clear();
            // Active readers keep their allocation charged independently.
            self.budget.reserve(bytes)
        })
    }
    pub fn build(&self, base: Option<&Data>) -> Build {
        let count = base.map_or(0, |base| base.ids.len());
        let lease = allocation(count).and_then(|bytes| self.reserve(bytes));
        // A shared baseline is copied only after full allocation admission.
        let ids = if lease.is_some() {
            base.map_or_else(BTreeSet::new, |base| base.ids.clone())
        } else {
            BTreeSet::new()
        };
        let incremental = base.is_some() && lease.is_some();
        Build {
            ids,
            lease,
            incremental,
        }
    }
    pub fn finish(
        &self,
        key: Key,
        selected: ManifestSnapshot,
        build: Build,
    ) -> BTreeSet<m::ObjectKey> {
        let Some(lease) = build.lease else {
            return build.ids;
        };
        let data = Arc::new(Data {
            selected,
            ids: build.ids,
            _lease: lease,
        });
        let result = data.ids.clone();
        let now = tokio::time::Instant::now();
        let mut entries = self.entries.lock().unwrap();
        if self.roots != 0 && !entries.iter().any(|entry| entry.key == key) {
            while entries.len() >= self.roots {
                let oldest = entries
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, entry)| entry.touched)
                    .unwrap()
                    .0;
                entries.swap_remove(oldest);
            }
            entries.push(Entry {
                key,
                data,
                created: now,
                touched: now,
            });
        }
        result
    }
}

#[cfg(test)]
mod tests;
pub(super) struct Build {
    pub ids: BTreeSet<m::ObjectKey>,
    lease: Option<Lease>,
    pub incremental: bool,
}
impl Build {
    pub fn admitted_base(&self) -> bool {
        self.incremental && self.lease.is_some()
    }
    pub fn insert(&mut self, key: m::ObjectKey) {
        if self.lease.as_mut().is_some_and(|lease| {
            allocation(self.ids.len() + 1)
                .and_then(|bytes| lease.resize(bytes))
                .is_none()
        }) {
            self.lease = None;
        }
        self.ids.insert(key);
    }
}
