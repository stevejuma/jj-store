//! Reuse verified immutable in-memory indexes, never unchecked on-disk bytes.
use super::*;
use std::sync::{
    Mutex,
    atomic::{AtomicU64, Ordering},
};

#[derive(Clone, PartialEq, Eq)]
struct Stamp {
    #[cfg(unix)]
    fields: [u64; 7],
}
impl Stamp {
    fn read(path: &Path) -> Result<Self> {
        let metadata = fs::symlink_metadata(path).map_err(|_| Error::Unavailable)?;
        if !metadata.is_file() {
            return Err(Error::Unavailable);
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt as _;
            // Coarse timestamp filesystems use full verification on every read.
            if metadata.mtime_nsec() == 0 || metadata.ctime_nsec() == 0 {
                return Err(Error::Unavailable);
            }
            Ok(Self {
                fields: [
                    metadata.dev(),
                    metadata.ino(),
                    metadata.len(),
                    metadata.mtime() as u64,
                    metadata.mtime_nsec() as u64,
                    metadata.ctime() as u64,
                    metadata.ctime_nsec() as u64,
                ],
            })
        }
        #[cfg(not(unix))]
        Err(Error::Unavailable)
    }
}
pub(super) struct Verification {
    pub(super) files: BTreeMap<String, File>,
    stamps: BTreeMap<String, Stamp>,
}
impl Verification {
    pub fn capture(path: &Path, files: BTreeMap<String, File>, budget: &Budget) -> Result<Self> {
        let mut stamps = BTreeMap::new();
        for name in files.keys() {
            budget.work(1)?;
            stamps.insert(name.clone(), Stamp::read(&path.join(name))?);
        }
        Ok(Self { files, stamps })
    }
    pub fn matches(&self, path: &Path, policy: Policy, budget: &Budget) -> Result<bool> {
        let (files, _) = inventory(path, policy, budget, false)?;
        if files.len() != self.files.len() {
            return Ok(false);
        }
        for (name, file) in files {
            budget.work(1)?;
            if self
                .files
                .get(&name)
                .is_none_or(|saved| saved.bytes != file.bytes)
                || self.stamps.get(&name) != Some(&Stamp::read(&path.join(name))?)
            {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
pub(in super::super) struct Saved {
    pub index: projection::Prepared,
    path: PathBuf,
    repository: m::RepositoryId,
    epoch: m::EpochId,
    pub operation: m::OperationId,
    pub(super) verification: Verification,
    weight: u64,
    policy: Policy,
    retained: Arc<AtomicU64>,
    report: Mutex<Report>,
    pub(super) memory: Arc<super::super::memory::Indexes>,
}
impl Drop for Saved {
    fn drop(&mut self) {
        self.retained.fetch_sub(self.weight, Ordering::AcqRel);
    }
}
impl Saved {
    pub fn report(&self) -> Report {
        Report {
            state: State::Reused,
            evicted: 0,
            ..self.report.lock().unwrap().clone()
        }
    }
    pub(super) fn update_report(&self, report: Report) {
        *self.report.lock().unwrap() = report;
    }
    /// Additional native operation loaders read disk, so they must reverify it.
    pub fn verify_disk(&self, budget: &Budget) -> Result<()> {
        let (files, _) = inventory(&self.path, self.policy, budget, true)?;
        if files != self.verification.files {
            return Err(Error::Unavailable);
        }
        Ok(())
    }
}
#[derive(Default)]
pub(in super::super) struct Residents {
    entries: Mutex<Vec<Arc<super::super::memory::Retained<Saved>>>>,
    retained: Arc<AtomicU64>,
}
impl Residents {
    pub(in super::super) fn get_memory(&self, access: &dyn ReadAccess) -> Option<Arc<Saved>> {
        let mut entries = self.entries.lock().unwrap();
        entries.retain(|entry| entry.present());
        let (index, saved) = entries.iter().enumerate().find_map(|(index, slot)| {
            slot.get_if(|e| {
                e.repository == access.repository()
                    && e.epoch == access.epoch()
                    && e.operation == access.operation()
            })
            .map(|saved| (index, saved))
        })?;
        let slot = entries.remove(index);
        entries.push(slot);
        Some(saved)
    }
    pub(super) fn get(
        &self,
        access: &dyn ReadAccess,
        path: &Path,
        files: &BTreeMap<String, File>,
        policy: Policy,
        budget: &Budget,
    ) -> Result<Option<Arc<Saved>>> {
        let saved = {
            let mut entries = self.entries.lock().unwrap();
            entries.retain(|entry| entry.present());
            let Some((index, saved)) = entries.iter().enumerate().find_map(|(index, slot)| {
                slot.get_if(|e| {
                    e.path == path
                        && e.repository == access.repository()
                        && e.epoch == access.epoch()
                        && e.operation == access.operation()
                })
                .map(|saved| (index, saved))
            }) else {
                return Ok(None);
            };
            let slot = entries.remove(index);
            entries.push(slot);
            saved
        };
        if saved.verification.files == *files && saved.verification.matches(path, policy, budget)? {
            Ok(Some(saved))
        } else {
            Ok(None)
        }
    }
    pub(super) fn remember(
        &self,
        input: Insertion,
        access: &dyn ReadAccess,
        policy: Policy,
        budget: &Budget,
    ) -> Result<()> {
        let Insertion {
            index,
            path,
            files,
            bytes,
            report,
        } = input;
        // Account Vec capacity slack, per-file/index metadata, names and hashes.
        let weight = bytes
            .saturating_mul(2)
            .saturating_add(index.weight())
            .saturating_add(files.len() as u64 * 1024)
            .saturating_add(files.keys().map(|name| name.len() as u64 * 2).sum::<u64>());
        let ceiling = policy.max_total_bytes.min(super::super::memory::LIMIT);
        if weight > ceiling {
            return Ok(());
        }
        let verification = match Verification::capture(&path, files, budget) {
            Ok(value) => value,
            Err(_) => {
                budget.check()?;
                return Ok(());
            }
        };
        let mut entries = self.entries.lock().unwrap();
        entries.retain(|entry| entry.present() && entry.get_if(|e| e.path == path).is_none());
        // Eviction does not free a resident still held by an executing query.
        while !entries.is_empty()
            && (entries.len() >= policy.max_entries
                || self.retained.load(Ordering::Acquire) + weight > ceiling)
        {
            entries.remove(0);
        }
        if self.retained.load(Ordering::Acquire) + weight > ceiling {
            return Ok(());
        }
        self.retained.fetch_add(weight, Ordering::AcqRel);
        entries.push(super::super::memory::Retained::new(Arc::new(Saved {
            index,
            path,
            repository: access.repository(),
            epoch: access.epoch(),
            operation: access.operation(),
            verification,
            weight,
            policy,
            retained: self.retained.clone(),
            report: Mutex::new(report),
            memory: budget.index_memory.clone(),
        })));
        Ok(())
    }
}

pub(super) struct Insertion {
    pub index: projection::Prepared,
    pub path: PathBuf,
    pub files: BTreeMap<String, File>,
    pub bytes: u64,
    pub report: Report,
}
