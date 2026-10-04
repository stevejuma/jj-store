//! Cross-profile staging and retained-handle admission in the native namespace.
use super::*;

const RECORD: &str = "reservation.json";
const OVERHEAD: u64 = 4096;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Record {
    version: u32,
    pub repository: m::RepositoryId,
    pub bytes: u64,
    // Namespace-local bookkeeping only; persisted version1 bytes stay unchanged.
    #[serde(skip)]
    pub path: PathBuf,
}

/// A caller-owned disposable generation. Keep this alive until all its files
/// and readers are dropped. `bytes()` is the writable allowance after metadata.
/// Acquisition does bounded filesystem work and belongs on a blocking worker.
/// The namespace lock is released before this handle is returned. An independent
/// file lock keeps its full reservation charged across other processes/queries.
pub struct Reservation {
    // Remove files while the reservation is still locked (field drop order).
    directory: tempfile::TempDir,
    _lock: fs::File,
    bytes: u64,
}
impl Reservation {
    pub fn acquire(root: &Path, repository: m::RepositoryId, policy: Policy) -> Result<Self> {
        let budget = Budget::new(Limits::default())?;
        Self::acquire_for_query(root, repository, policy, &budget)
    }
    pub(in crate::query) fn acquire_for_query(
        root: &Path,
        repository: m::RepositoryId,
        policy: Policy,
        budget: &Budget,
    ) -> Result<Self> {
        policy.validate()?;
        if !root.is_absolute()
            || policy.max_entry_bytes <= OVERHEAD
            || policy.max_entry_bytes > 2 * 1024 * 1024 * 1024
        {
            return Err(Error::Limit);
        }
        let mut cache = Cache::open(root, policy, budget)?;
        Self::within(&mut cache, repository)
    }
    pub(super) fn within(cache: &mut Cache, repository: m::RepositoryId) -> Result<Self> {
        let policy = cache.policy;
        if policy.max_entry_bytes <= OVERHEAD || policy.max_entry_bytes > 2 * 1024 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        cache.reserve(repository, policy.max_entry_bytes)?;
        let directory = tempfile::Builder::new()
            .prefix(".reserved-")
            .tempdir_in(&cache.root)
            .map_err(|_| Error::Unavailable)?;
        let lock =
            fs::File::create(directory.path().join("lock")).map_err(|_| Error::Unavailable)?;
        lock.try_lock().map_err(|_| Error::Busy)?;
        let record = Record {
            version: 1,
            repository,
            bytes: policy.max_entry_bytes,
            path: directory.path().to_owned(),
        };
        let data = serde_json::to_vec(&record).map_err(|_| Error::Unavailable)?;
        if data.len() as u64 > OVERHEAD {
            return Err(Error::Limit);
        }
        fs::write(directory.path().join(RECORD), data).map_err(|_| Error::Unavailable)?;
        cache.reservations.push(record);
        Ok(Self {
            directory,
            _lock: lock,
            bytes: policy.max_entry_bytes - OVERHEAD,
        })
    }
    pub fn path(&self) -> &Path {
        self.directory.path()
    }
    pub fn bytes(&self) -> u64 {
        self.bytes
    }
}

pub(super) fn inspect(path: &Path) -> Result<Option<Record>> {
    if !fs::symlink_metadata(path)
        .map_err(|_| Error::Unavailable)?
        .is_dir()
    {
        return Err(Error::Unavailable);
    }
    let lock_path = path.join("lock");
    match fs::symlink_metadata(&lock_path) {
        Ok(info) if info.is_file() => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            // Creation holds the namespace lock. A directory without its lock
            // can only be an interrupted creation while we hold that lock.
            remove(path)?;
            return Ok(None);
        }
        _ => return Err(Error::Unavailable),
    }
    let lock = fs::File::options()
        .read(true)
        .write(true)
        .open(lock_path)
        .map_err(|_| Error::Unavailable)?;
    match lock.try_lock() {
        Ok(()) => {
            // A lost process cannot retain a quota reservation. Its derived
            // generation is disposable and is never adopted without a proof.
            remove(path)?;
            Ok(None)
        }
        Err(std::fs::TryLockError::WouldBlock) => {
            let record = path.join(RECORD);
            let info = fs::symlink_metadata(&record).map_err(|_| Error::Unavailable)?;
            if !info.is_file() || info.len() > OVERHEAD {
                return Err(Error::Unavailable);
            }
            let mut bytes = vec![];
            fs::File::open(record)
                .map_err(|_| Error::Unavailable)?
                .take(OVERHEAD + 1)
                .read_to_end(&mut bytes)
                .map_err(|_| Error::Unavailable)?;
            if bytes.len() as u64 > OVERHEAD {
                return Err(Error::Unavailable);
            }
            let mut record: Record =
                serde_json::from_slice(&bytes).map_err(|_| Error::Unavailable)?;
            if record.version != 1
                || record.bytes <= OVERHEAD
                || record.bytes > 2 * 1024 * 1024 * 1024
            {
                return Err(Error::Unavailable);
            }
            record.path = path.to_owned();
            Ok(Some(record))
        }
        Err(std::fs::TryLockError::Error(_)) => Err(Error::Unavailable),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn policy() -> Policy {
        Policy {
            max_entry_bytes: 16 * 1024,
            max_repository_bytes: 32 * 1024,
            max_total_bytes: 48 * 1024,
            ..Default::default()
        }
    }
    #[test]
    fn reservations_share_native_repository_total_and_lifetime_admission() {
        let root = tempfile::tempdir().unwrap();
        let a = m::RepositoryId::new([1; 16]);
        let b = m::RepositoryId::new([2; 16]);
        let one = Arc::new(Reservation::acquire(root.path(), a, policy()).unwrap());
        let active_reader = one.clone();
        let two = Reservation::acquire(root.path(), a, policy()).unwrap();
        assert!(matches!(
            Reservation::acquire(root.path(), a, policy()),
            Err(Error::Limit)
        ));
        let three = Reservation::acquire(root.path(), b, policy()).unwrap();
        assert!(matches!(
            Reservation::acquire(root.path(), b, policy()),
            Err(Error::Limit)
        ));
        let budget = Budget::new(Limits::default()).unwrap();
        let mut native = Cache::open(root.path(), policy(), &budget).unwrap();
        assert_eq!(native.bytes(), 48 * 1024);
        assert_eq!(native.repository_bytes(a), 32 * 1024);
        assert!(matches!(native.reserve(a, 1), Err(Error::Limit)));
        drop(native);
        let path = one.path().to_owned();
        drop(one);
        assert!(path.exists());
        assert!(matches!(
            Reservation::acquire(root.path(), a, policy()),
            Err(Error::Limit)
        ));
        drop(active_reader);
        assert!(!path.exists());
        let replacement = Reservation::acquire(root.path(), a, policy()).unwrap();
        assert_eq!(replacement.bytes(), 12 * 1024);
        drop((two, three, replacement));
        assert_eq!(
            Cache::open(root.path(), policy(), &budget).unwrap().bytes(),
            0
        );
    }
    #[test]
    fn stale_reserved_generations_are_discarded_before_reuse() {
        let root = tempfile::tempdir().unwrap();
        let repository = m::RepositoryId::new([1; 16]);
        let reservation = Reservation::acquire(root.path(), repository, policy()).unwrap();
        fs::write(reservation.path().join("staged-data"), b"partial").unwrap();
        let Reservation {
            directory,
            _lock: lock,
            bytes: _,
        } = reservation;
        let path = directory.keep();
        drop(lock); // Model process exit without destructors.
        let next = Reservation::acquire(root.path(), repository, policy()).unwrap();
        assert!(!path.exists());
        assert!(next.path().exists());
    }
}
