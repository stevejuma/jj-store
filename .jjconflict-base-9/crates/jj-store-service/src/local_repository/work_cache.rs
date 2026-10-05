//! Data-only reuse remains behind fresh companion and Work-blob admission.
mod data;
mod memory;
use super::{LocalRepository, map_metadata, work::decode_work};
use crate::work::{Error, Result};
pub(super) use data::Data;
use jj_store_jj::local_metadata::{Domain, GitOid, Snapshot};
use jj_store_model::RepositoryId;
use memory::Budget;
use sha2::{Digest as _, Sha256};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, Instant},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    repository: Option<RepositoryId>,
    object: Option<GitOid>,
    bytes: [u8; 32],
}
struct Entry {
    data: Arc<Data>,
    touched: Instant,
}
struct Cache {
    entries: Mutex<BTreeMap<Key, Entry>>,
    build: Mutex<()>,
    budget: Arc<Budget>,
    roots: usize,
    retained: usize,
    ttl: Duration,
}
impl Default for Cache {
    fn default() -> Self {
        Self {
            entries: Mutex::new(BTreeMap::new()),
            build: Mutex::new(()),
            budget: Arc::new(Budget::new(512 * 1024 * 1024)),
            roots: 32,
            retained: 256 * 1024 * 1024,
            ttl: Duration::from_secs(300),
        }
    }
}
impl Cache {
    fn lookup(&self, key: Key, now: Instant) -> Option<Arc<Data>> {
        let mut entries = self.entries.lock().unwrap();
        let expired: Vec<_> = entries
            .iter()
            .filter(|(_, entry)| now.saturating_duration_since(entry.touched) >= self.ttl)
            .map(|(key, _)| *key)
            .collect();
        let removed: Vec<_> = expired
            .into_iter()
            .filter_map(|key| entries.remove(&key))
            .collect();
        let data = entries.get_mut(&key).map(|entry| {
            entry.touched = now;
            entry.data.clone()
        });
        drop(entries);
        drop(removed);
        data
    }
    fn evict(&self) {
        let removed = std::mem::take(&mut *self.entries.lock().unwrap());
        drop(removed);
    }
    fn load(&self, key: Key, bytes: &[u8]) -> Result<Arc<Data>> {
        if let Some(data) = self.lookup(key, Instant::now()) {
            return Ok(data);
        }
        // Serialize cold derivation without blocking warm map lookups.
        let _builder = self.build.lock().unwrap();
        if let Some(data) = self.lookup(key, Instant::now()) {
            return Ok(data);
        }
        let scratch = memory::decoding(bytes.len())?;
        if scratch > self.budget.limit() {
            return Err(Error::Limit);
        }
        let mut lease = match self.budget.reserve(scratch) {
            Ok(lease) => lease,
            Err(Error::Busy) => {
                // Eviction never releases a charge still owned by a reader.
                self.evict();
                self.budget.reserve(scratch)?
            }
            Err(error) => return Err(error),
        };
        let state = match key.repository.filter(|_| key.object.is_some()) {
            Some(repository) => decode_work(bytes, repository)?,
            None => Default::default(),
        };
        let (resident, retention) = memory::resident(&state)?;
        if let Err(error) = lease.resize(resident) {
            if error != Error::Busy {
                return Err(error);
            }
            self.evict();
            lease.resize(resident)?;
        }
        let data = Arc::new(Data::new(state, lease, retention, self.budget.clone())?);
        if retention <= self.retained && self.roots > 0 {
            let mut removed = Vec::new();
            let mut entries = self.entries.lock().unwrap();
            while entries.len() >= self.roots
                || entries
                    .values()
                    .map(|entry| entry.data.retention)
                    .sum::<usize>()
                    > self.retained - retention
            {
                let Some(oldest) = entries
                    .iter()
                    .min_by_key(|(_, entry)| entry.touched)
                    .map(|(key, _)| *key)
                else {
                    break;
                };
                removed.extend(entries.remove(&oldest));
            }
            entries.insert(
                key,
                Entry {
                    data: data.clone(),
                    touched: Instant::now(),
                },
            );
            drop(entries);
            drop(removed);
        }
        Ok(data)
    }
}
impl LocalRepository {
    pub(super) fn work_data(&self, snapshot: Option<&Snapshot>) -> Result<Arc<Data>> {
        self.check_identity(snapshot)?;
        // A cached root cannot conceal missing objects or tighter source limits.
        let bytes = snapshot
            .map(|snapshot| {
                self.metadata
                    .read_domain(snapshot, Domain::Work)
                    .map_err(map_metadata)
            })
            .transpose()?
            .flatten()
            .unwrap_or_default();
        let key = Key {
            repository: snapshot.map(Snapshot::repository),
            object: snapshot
                .and_then(|snapshot| snapshot.selection(Domain::Work))
                .map(|s| s.object),
            bytes: Sha256::digest(&bytes).into(),
        };
        static CACHE: OnceLock<Cache> = OnceLock::new();
        CACHE.get_or_init(Cache::default).load(key, &bytes)
    }
}

#[cfg(test)]
mod tests;
