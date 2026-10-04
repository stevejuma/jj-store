//! Reuse verified immutable in-memory indexes, never unchecked on-disk bytes.
use super::*;
use jj_lib::default_index::DefaultReadonlyIndex;
use std::sync::Mutex;

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
    pub index: DefaultReadonlyIndex,
    path: PathBuf,
    repository: m::RepositoryId,
    epoch: m::EpochId,
    pub operation: m::OperationId,
    pub(super) verification: Verification,
    weight: u64,
    policy: Policy,
}
impl Saved {
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
pub(in super::super) struct Residents(Mutex<Vec<Arc<Saved>>>);
impl Residents {
    pub(super) fn get(
        &self,
        access: &dyn ReadAccess,
        path: &Path,
        files: &BTreeMap<String, File>,
        policy: Policy,
        budget: &Budget,
    ) -> Result<Option<Arc<Saved>>> {
        let saved = {
            let mut entries = self.0.lock().unwrap();
            let Some(index) = entries.iter().position(|e| {
                e.path == path
                    && e.repository == access.repository()
                    && e.epoch == access.epoch()
                    && e.operation == access.operation()
            }) else {
                return Ok(None);
            };
            let saved = entries.remove(index);
            entries.push(saved.clone());
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
        index: DefaultReadonlyIndex,
        path: PathBuf,
        access: &dyn ReadAccess,
        files: BTreeMap<String, File>,
        bytes: u64,
        policy: Policy,
        budget: &Budget,
    ) -> Result<()> {
        // Account Vec capacity slack, per-file/index metadata, names and hashes.
        let weight = bytes
            .saturating_mul(2)
            .saturating_add(files.len() as u64 * 1024)
            .saturating_add(files.keys().map(|name| name.len() as u64 * 2).sum::<u64>());
        let ceiling = policy.max_total_bytes.min(256 * 1024 * 1024);
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
        let mut entries = self.0.lock().unwrap();
        entries.retain(|entry| entry.path != path);
        // The namespace lease covers evaluation; no other job on this executor
        // can retain an evicted resident while entering the same namespace.
        while entries.len() >= policy.max_entries
            || entries.iter().map(|e| e.weight).sum::<u64>() + weight > ceiling
        {
            entries.remove(0);
        }
        entries.push(Arc::new(Saved {
            index,
            path,
            repository: access.repository(),
            epoch: access.epoch(),
            operation: access.operation(),
            verification,
            weight,
            policy,
        }));
        Ok(())
    }
}
