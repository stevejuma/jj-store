//! A readonly disk reader pins the complete quota-accounted generation.
use super::*;

pub(super) const PIN: &str = "reader-pin";
pub(in crate::query) struct Pin(fs::File);
impl Pin {
    pub(in crate::query) fn acquire(path: &Path) -> Result<Self> {
        let path = path.join(PIN);
        match fs::symlink_metadata(&path) {
            Ok(_) => regular(&path)?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => return Err(Error::Unavailable),
        }
        let file = fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .map_err(|_| Error::Unavailable)?;
        regular(&path)?;
        file.try_lock_shared().map_err(|_| Error::Busy)?;
        Ok(Self(file))
    }
}
impl Drop for Pin {
    fn drop(&mut self) {
        let _ = self.0.unlock();
    }
}
fn regular(path: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(path).map_err(|_| Error::Unavailable)?;
    if !metadata.is_file() || metadata.len() != 0 {
        return Err(Error::Unavailable);
    }
    Ok(())
}
pub(super) fn exclusive(path: &Path) -> Result<Option<fs::File>> {
    let path = path.join(PIN);
    match fs::symlink_metadata(&path) {
        Ok(_) => regular(&path)?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err(Error::Unavailable),
    }
    let file = fs::File::options()
        .read(true)
        .write(true)
        .open(path)
        .map_err(|_| Error::Unavailable)?;
    file.try_lock().map_err(|_| Error::Busy)?;
    Ok(Some(file))
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    #[test]
    fn pinned_slots_stay_charged_and_eviction_skips_to_a_removable_generation() {
        let root = tempfile::tempdir().unwrap();
        let repository = m::RepositoryId::new([1; 16]);
        let policy = Policy {
            max_entries: 2,
            max_entry_bytes: 32 * 1024,
            max_repository_bytes: 64 * 1024,
            max_total_bytes: 64 * 1024,
            ..Default::default()
        };
        let budget = Budget::new(Limits::default()).unwrap();
        let cache = Cache::open(root.path(), policy, &budget).unwrap();
        for id in 0..2 {
            let path = cache.path(id);
            fs::create_dir(&path).unwrap();
            fs::write(path.join("data"), [1; 16 * 1024]).unwrap();
            let receipt = Receipt {
                version: 1,
                repository,
                epoch: m::EpochId::new([1; 16]),
                operation: m::OperationId::new([id as u8; 64]),
                used: id as u64,
                files: inventory(&path, policy, &budget, true).unwrap().0,
            };
            fs::write(path.join(RECEIPT), serde_json::to_vec(&receipt).unwrap()).unwrap();
        }
        let first_path = cache.path(0);
        let second_path = cache.path(1);
        drop(cache);
        let first = Pin::acquire(&first_path).unwrap();
        let second = Pin::acquire(&second_path).unwrap();
        let mut cache = Cache::open(root.path(), policy, &budget).unwrap();
        let bytes = cache.bytes();
        assert!(matches!(cache.reserve(repository, 1024), Err(Error::Limit)));
        assert_eq!(cache.bytes(), bytes);
        assert_eq!(cache.slots.len(), 2);
        drop(second);
        cache.reserve(repository, 1024).unwrap();
        assert_eq!(cache.slots.len(), 1);
        assert!(first_path.exists());
        assert!(!second_path.exists());
        assert_eq!(cache.bytes(), cache.slots[0].bytes);
        assert_eq!(cache.evicted, 1);
        drop(first);
        cache.evict().unwrap();
        assert_eq!(cache.bytes(), 0);
    }
    #[test]
    fn readonly_generation_pins_preserve_files_and_quota_until_last_reader_exits() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("generation");
        fs::create_dir(&path).unwrap();
        fs::write(path.join("data"), [1; 4096]).unwrap();
        let one = Arc::new(Pin::acquire(&path).unwrap());
        let reader = one.clone();
        let two = Pin::acquire(&path).unwrap();
        let budget = Budget::new(Limits::default()).unwrap();
        let (files, bytes) = inventory(&path, Policy::default(), &budget, true).unwrap();
        assert_eq!(bytes, 4096);
        assert_eq!(files.len(), 1);
        assert!(matches!(remove(&path), Err(Error::Busy)));
        drop(one);
        drop(two);
        assert!(matches!(remove(&path), Err(Error::Busy)));
        assert!(path.join("data").exists());
        drop(reader);
        remove(&path).unwrap();
        assert!(!path.exists());
    }
}
