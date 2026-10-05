//! Short-lived physical binding validation; never object or principal authority.
use super::{Error, LocalFolder, Result};
use jj_store_model::RepositoryId;
use std::{
    collections::BTreeMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

const MAX_AGE: Duration = Duration::from_secs(1);
const CAPACITY: usize = 256;
struct Validated {
    folder: LocalFolder,
    generation: u64,
    revision: u64,
    completed: Instant,
}
#[derive(Default)]
pub struct FolderAdmission {
    revision: AtomicU64,
    value: Mutex<Option<Validated>>,
    validations: AtomicU64,
    hits: AtomicU64,
    validation_nanos: AtomicU64,
}
impl FolderAdmission {
    pub fn invalidate(&self) {
        self.revision.fetch_add(1, Ordering::AcqRel);
    }
    fn matches(&self, value: &Validated, folder: &LocalFolder, generation: u64) -> bool {
        value.revision == self.revision.load(Ordering::Acquire)
            && value.completed.elapsed() < MAX_AGE
            && value.folder == *folder
            && value.generation == generation
    }
    /// Never waits for physical IO on the async caller's thread.
    pub fn cached(&self, folder: &LocalFolder, generation: u64) -> bool {
        let Ok(value) = self.value.try_lock() else {
            return false;
        };
        let hit = value
            .as_ref()
            .is_some_and(|v| self.matches(v, folder, generation));
        if hit {
            self.hits.fetch_add(1, Ordering::Relaxed);
        }
        hit
    }
    /// Call on a blocking worker. Authoritative boundaries force a fresh check;
    /// concurrent callers may share a check completed after they began waiting.
    pub fn validate(
        &self,
        folder: &LocalFolder,
        generation: u64,
        force: bool,
        load: impl FnOnce() -> Result<(LocalFolder, u64)>,
    ) -> Result<()> {
        let requested = Instant::now();
        let mut value = self.value.lock().expect("folder validation");
        if value.as_ref().is_some_and(|v| {
            self.matches(v, folder, generation) && (!force || v.completed >= requested)
        }) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            return Ok(());
        }
        *value = None;
        let revision = self.revision.load(Ordering::Acquire);
        self.validations.fetch_add(1, Ordering::Relaxed);
        let start = Instant::now();
        let result = load();
        self.validation_nanos.fetch_add(
            start.elapsed().as_nanos().min(u64::MAX as u128) as u64,
            Ordering::Relaxed,
        );
        let (current, current_generation) = result?;
        if current != *folder || current_generation != generation {
            return Err(Error::IdentityChanged);
        }
        if revision != self.revision.load(Ordering::Acquire) {
            return Err(Error::Stale);
        }
        *value = Some(Validated {
            folder: current,
            generation,
            revision,
            completed: Instant::now(),
        });
        Ok(())
    }
    /// Process-local diagnostic counters, never part of the HTTP contract.
    pub fn stats(&self) -> (u64, u64, u64) {
        (
            self.validations.load(Ordering::Relaxed),
            self.hits.load(Ordering::Relaxed),
            self.validation_nanos.load(Ordering::Relaxed),
        )
    }
}
#[derive(Default)]
pub struct FolderAdmissions(Mutex<BTreeMap<RepositoryId, Arc<FolderAdmission>>>);
impl FolderAdmissions {
    pub fn get(&self, repository: RepositoryId) -> Option<Arc<FolderAdmission>> {
        let mut entries = self.0.lock().expect("folder admissions");
        if let Some(entry) = entries.get(&repository) {
            return Some(entry.clone());
        }
        if entries.len() >= CAPACITY {
            let idle = entries
                .iter()
                .find(|(_, e)| Arc::strong_count(e) == 1)
                .map(|(id, _)| *id)?;
            entries.remove(&idle);
        }
        let entry = Arc::new(FolderAdmission::default());
        entries.insert(repository, entry.clone());
        Some(entry)
    }
    pub fn invalidate(&self) {
        for entry in self.0.lock().expect("folder admissions").values() {
            entry.invalidate();
        }
    }
}
#[cfg(test)]
mod tests;
