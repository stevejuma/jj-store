//! Reuse verified immutable in-memory indexes, never unchecked on-disk bytes.
mod additional;
use super::*;
pub(in crate::query) use additional::Data;
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
    pub(super) index_bytes: u64,
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
    pub fn data(&self) -> Data {
        Data {
            index: self.index.clone(),
            memory: self.memory.clone(),
        }
    }
    pub fn reader(&self, budget: Arc<Budget>) -> Result<Box<dyn jj_lib::index::ReadonlyIndex>> {
        self.data().reader(budget)
    }
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
}
#[derive(Default)]
pub(in super::super) struct Residents {
    entries: Mutex<Vec<Arc<super::super::memory::Retained<Saved>>>>,
    retained: Arc<AtomicU64>,
    additional: additional::AdditionalResidents,
}
impl Residents {
    pub(in crate::query) fn reclaim_filtered(&self) -> bool {
        let entries = self.entries.lock().unwrap().clone();
        let mut reclaimed = false;
        for entry in entries {
            reclaimed |= entry.evict_idle_if(|saved| saved.index.has_reference_filter());
        }
        reclaimed | self.additional.reclaim_filtered()
    }
    pub(in crate::query) fn get_additional(
        &self,
        access: &dyn ReadAccess,
        operation: m::OperationId,
    ) -> Option<Data> {
        self.additional.get(access, operation)
    }
    pub(in crate::query) fn remember_additional(
        &self,
        access: &dyn ReadAccess,
        operation: m::OperationId,
        data: Data,
        policy: Policy,
    ) {
        self.additional.remember(access, operation, data, policy);
    }
    pub(in super::super) fn get_memory(&self, access: &dyn ReadAccess) -> Option<Arc<Saved>> {
        self.get_memory_at(access, access.operation())
    }
    pub(in super::super) fn get_memory_at(
        &self,
        access: &dyn ReadAccess,
        operation: m::OperationId,
    ) -> Option<Arc<Saved>> {
        let mut entries = self.entries.lock().unwrap();
        entries.retain(|entry| entry.present());
        let (index, saved) = entries.iter().enumerate().find_map(|(index, slot)| {
            slot.get_if(|e| {
                e.repository == access.repository()
                    && e.epoch == access.epoch()
                    && e.operation == operation
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
            index_bytes,
            report,
        } = input;
        // Account Vec capacity slack, per-file/index metadata, names and hashes.
        let weight = resident_weight(index_bytes, index.weight(), &files);
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
            index_bytes,
            retained: self.retained.clone(),
            report: Mutex::new(report),
            memory: budget.index_memory.clone(),
        })));
        Ok(())
    }
}

fn resident_weight(index_bytes: u64, projection: u64, files: &BTreeMap<String, File>) -> u64 {
    // Framed JJ allocations and the bounded database cache have their own
    // leases. The complete filter file belongs to disk quota, not resident RAM.
    index_bytes
        .saturating_add(projection)
        .saturating_add(files.len() as u64 * 1024)
        .saturating_add(files.keys().map(|name| name.len() as u64 * 2).sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_resident_admission_counts_index_allocations_and_bounded_filter_cache() {
        let commits = 2_000_000u64;
        let index_bytes = commits * 104 * 2 + 1024 * 1024;
        let projection = commits * 192 + 1024 * 1024 + 16 * 1024 * 1024 + 65536;
        let mut files = BTreeMap::from([
            (
                "index/segments/native".into(),
                File {
                    bytes: commits * 104,
                    digest: "a".repeat(128),
                },
            ),
            (
                "index/local-reference-filter".into(),
                File {
                    bytes: 1024 * 1024 * 1024,
                    digest: "b".repeat(128),
                },
            ),
        ]);
        let weight = resident_weight(index_bytes, projection, &files);
        assert!(
            weight + 128 * 1024 * 1024 < super::super::super::memory::LIMIT,
            "disk-only filter bytes prevented a bounded resident: {weight}"
        );
        files.get_mut("index/local-reference-filter").unwrap().bytes += 512 * 1024 * 1024;
        assert_eq!(
            resident_weight(index_bytes, projection, &files),
            weight,
            "disk growth is covered by slot quota, not additional resident RAM"
        );
    }
}

pub(super) struct Insertion {
    pub index: projection::Prepared,
    pub path: PathBuf,
    pub files: BTreeMap<String, File>,
    pub index_bytes: u64,
    pub report: Report,
}
