//! Persistent code namespaces charged alongside ordinary native query caches.
use super::*;
use reservation::{OVERHEAD, RECORD, Record};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "profile", rename_all = "snake_case", deny_unknown_fields)]
pub enum Binding {
    Native {
        repository: m::RepositoryId,
        epoch: m::EpochId,
    },
    Local {
        repository: m::RepositoryId,
        generation: u64,
    },
}
impl Binding {
    pub fn repository(&self) -> m::RepositoryId {
        match self {
            Self::Native { repository, .. } | Self::Local { repository, .. } => *repository,
        }
    }
}
/// Locked writer reservation. Ready data lives beyond this handle; memory
/// readers do not keep the writer lock or prevent bounded disk eviction.
pub struct Namespace {
    path: PathBuf,
    policy: Policy,
    binding: Binding,
    _lock: fs::File,
    complete: bool,
}
impl Namespace {
    pub fn acquire(root: &Path, binding: Binding, policy: Policy) -> Result<Self> {
        policy.validate()?;
        if !root.is_absolute()
            || policy.max_entry_bytes <= OVERHEAD
            || policy.max_entry_bytes > 2 * 1024 * 1024 * 1024
        {
            return Err(Error::Limit);
        }
        let budget = Budget::new(Limits::default())?;
        let mut cache = Cache::open(root, policy, &budget)?;
        let existing = cache
            .reservations
            .iter()
            .position(|record| record.managed.as_ref() == Some(&binding));
        let (path, lock) = if let Some(index) = existing {
            let record = cache.reservations.remove(index);
            let lock = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(record.path.join("lock"))
                .map_err(|_| Error::Unavailable)?;
            lock.try_lock().map_err(|_| Error::Busy)?;
            (record.path, lock)
        } else {
            let directory = tempfile::Builder::new()
                .prefix(".reserved-code-")
                .tempdir_in(&cache.root)
                .map_err(|_| Error::Unavailable)?;
            let lock =
                fs::File::create(directory.path().join("lock")).map_err(|_| Error::Unavailable)?;
            lock.try_lock().map_err(|_| Error::Busy)?;
            (directory.keep(), lock)
        };
        let mut namespace = Self {
            path,
            policy,
            binding,
            _lock: lock,
            complete: existing.is_some(),
        };
        while let Err(error) = cache.reserve(namespace.binding.repository(), policy.max_entry_bytes)
        {
            if !matches!(error, Error::Limit) || !evict(&mut cache)? {
                return Err(error);
            }
        }
        // Content-addressed JJ segments can be checked without a second
        // repository graph projection. Damage only clears the private copy.
        if existing.is_some() {
            let (files, _) = inventory(&namespace.path, policy, &budget, true)?;
            let damaged = files.iter().any(|(name, file)| {
                name.strip_prefix("index/segments/")
                    .is_some_and(|id| id != file.digest)
            });
            if damaged {
                fs::remove_dir_all(namespace.path.join("index")).map_err(|_| Error::Unavailable)?;
            }
        }
        namespace.record(policy.max_entry_bytes)?;
        fs::create_dir_all(namespace.path.join("index")).map_err(|_| Error::Unavailable)?;
        // Preserve an existing index on cancellation/failure. It is always
        // revalidated with fresh source authority before any later adoption.
        namespace.complete = true;
        Ok(namespace)
    }
    pub fn binding(&self) -> &Binding {
        &self.binding
    }
    pub fn path(&self) -> PathBuf {
        self.path.join("index")
    }
    pub fn bytes(&self) -> u64 {
        self.policy.max_entry_bytes - OVERHEAD
    }
    pub fn finish(&self) -> Result<()> {
        let budget = Budget::new(Limits::default())?;
        let (_, bytes) = inventory(&self.path, self.policy, &budget, false)?;
        self.record(bytes.max(OVERHEAD + 1))
    }
    fn record(&self, bytes: u64) -> Result<()> {
        let record = Record {
            version: 2,
            managed: Some(self.binding.clone()),
            repository: self.binding.repository(),
            bytes,
            path: self.path.clone(),
        };
        let data = serde_json::to_vec(&record).map_err(|_| Error::Unavailable)?;
        if data.len() as u64 > OVERHEAD {
            return Err(Error::Limit);
        }
        let mut file =
            tempfile::NamedTempFile::new_in(&self.path).map_err(|_| Error::Unavailable)?;
        file.write_all(&data).map_err(|_| Error::Unavailable)?;
        file.as_file().sync_all().map_err(|_| Error::Unavailable)?;
        file.persist(self.path.join(RECORD))
            .map_err(|_| Error::Unavailable)?;
        Ok(())
    }
    /// Cache presence carries activation; use bounded metadata discovery at restart.
    pub fn retained(root: &Path, policy: Policy) -> Result<Vec<Binding>> {
        let budget = Budget::new(Limits::default())?;
        let cache = Cache::open(root, policy, &budget)?;
        Ok(cache
            .reservations
            .into_iter()
            .filter_map(|record| record.managed)
            .collect())
    }
}
impl Drop for Namespace {
    fn drop(&mut self) {
        if !self.complete {
            let _ = fs::remove_dir_all(&self.path);
            return;
        }
        // Charge partial private files on error too. The next authorized build
        // may repair them, but they are never treated as repository authority.
        let _ = self.finish();
    }
}
fn evict(cache: &mut Cache) -> Result<bool> {
    // Only inactive managed namespaces are eligible. Existing standard slots
    // were already considered by Cache::reserve under the same namespace lock.
    let mut candidates: Vec<_> = cache
        .reservations
        .iter()
        .enumerate()
        .filter(|(_, r)| r.managed.is_some())
        .collect();
    candidates.sort_by_key(|(_, record)| {
        fs::metadata(record.path.join(RECORD))
            .and_then(|m| m.modified())
            .ok()
    });
    for (index, record) in candidates {
        let lock = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(record.path.join("lock"))
            .map_err(|_| Error::Unavailable)?;
        if lock.try_lock().is_err() {
            continue;
        }
        fs::remove_dir_all(&record.path).map_err(|_| Error::Unavailable)?;
        cache.reservations.remove(index);
        cache.evicted += 1;
        return Ok(true);
    }
    Ok(false)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn persistent_namespaces_share_quota_resume_and_reuse_one_repository_graph() {
        let root = tempfile::tempdir().unwrap();
        let policy = Policy {
            max_entries: 2,
            max_entry_bytes: 16 * 1024,
            max_repository_bytes: 32 * 1024,
            max_total_bytes: 32 * 1024,
            ..Default::default()
        };
        let binding = Binding::Local {
            repository: m::RepositoryId::new([21; 16]),
            generation: 3,
        };
        let namespace = Namespace::acquire(root.path(), binding.clone(), policy).unwrap();
        let path = namespace.path();
        fs::write(path.join("kept"), b"index bytes").unwrap();
        assert!(matches!(
            Namespace::acquire(root.path(), binding.clone(), policy),
            Err(Error::Busy)
        ));
        let other =
            Reservation::acquire(root.path(), m::RepositoryId::new([22; 16]), policy).unwrap();
        assert!(matches!(
            Reservation::acquire(root.path(), m::RepositoryId::new([23; 16]), policy),
            Err(Error::Limit)
        ));
        drop((namespace, other));
        assert_eq!(
            Namespace::retained(root.path(), policy).unwrap(),
            vec![binding.clone()]
        );
        let resumed = Namespace::acquire(root.path(), binding, policy).unwrap();
        assert_eq!(resumed.path(), path);
        assert_eq!(fs::read(path.join("kept")).unwrap(), b"index bytes");
    }
}
