//! Process-wide admission for immutable resident index allocations.
use super::{Error, Result};
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicU64, Ordering},
};
use std::{
    collections::BTreeMap,
    fs,
    io::Read as _,
    path::{Path, PathBuf},
    sync::Mutex,
};
pub(crate) mod framing;
mod retained;
pub(crate) use retained::Retained;

pub(crate) const LIMIT: u64 = 1024 * 1024 * 1024;
struct Pool {
    used: AtomicU64,
    limit: u64,
    clock: AtomicU64,
    retained: Mutex<Vec<std::sync::Weak<dyn retained::Entry>>>,
}
impl Pool {
    fn new(limit: u64) -> Self {
        Self {
            used: AtomicU64::new(0),
            limit,
            clock: AtomicU64::new(0),
            retained: Mutex::default(),
        }
    }
    fn global() -> &'static Arc<Self> {
        static POOL: OnceLock<Arc<Pool>> = OnceLock::new();
        POOL.get_or_init(|| Arc::new(Self::new(LIMIT)))
    }
    fn reserve(self: &Arc<Self>, bytes: u64) -> Result<Lease> {
        if bytes > self.limit {
            return Err(Error::Limit);
        }
        let admit = || {
            self.used
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |old| {
                    old.checked_add(bytes).filter(|n| *n <= self.limit)
                })
                .is_ok()
        };
        if !admit() {
            let mut entries: Vec<_> = self
                .retained
                .lock()
                .expect("resident registry")
                .iter()
                .filter_map(std::sync::Weak::upgrade)
                .collect();
            entries.sort_by_key(|entry| entry.used());
            let mut admitted = false;
            for entry in entries {
                entry.evict_idle();
                if admit() {
                    admitted = true;
                    break;
                }
            }
            if !admitted {
                return Err(Error::Limit);
            }
        }
        Ok(Lease {
            pool: self.clone(),
            bytes,
        })
    }
}
pub(crate) struct Lease {
    pool: Arc<Pool>,
    bytes: u64,
}
impl Lease {
    pub(crate) fn reserve(bytes: u64) -> Result<Self> {
        Pool::global().reserve(bytes)
    }
}
impl Drop for Lease {
    fn drop(&mut self) {
        self.pool.used.fetch_sub(self.bytes, Ordering::AcqRel);
    }
}

#[derive(Default)]
struct Held {
    directories: BTreeMap<PathBuf, (u64, Vec<Lease>)>,
    parents: Vec<Arc<Indexes>>,
}
/// A query and its retained immutable index share ownership of allocation leases.
#[derive(Default)]
pub(crate) struct Indexes(Mutex<Held>);
impl Indexes {
    pub(crate) fn include(&self, prior: Arc<Self>) {
        self.0.lock().expect("index memory").parents.push(prior);
    }
    pub(crate) fn reserve(
        &self,
        path: &Path,
        commit_len: usize,
        change_len: usize,
        max_commits: usize,
        mut check: impl FnMut() -> Result<()>,
    ) -> Result<()> {
        let bytes = framing::weight(path, commit_len, change_len, max_commits, &mut check)?;
        let mut held = self.0.lock().expect("index memory");
        let (reserved, leases) = held.directories.entry(path.to_owned()).or_default();
        if bytes > *reserved {
            leases.push(Lease::reserve(bytes - *reserved)?);
            *reserved = bytes;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_resident_admission_lives_until_the_last_reader_exits() {
        let pool = Arc::new(Pool::new(100));
        let native = Arc::new(pool.reserve(60).unwrap());
        let reader = native.clone();
        let local = pool.reserve(40).unwrap();
        assert!(matches!(pool.reserve(1), Err(Error::Limit)));
        drop(native);
        assert!(pool.reserve(60).is_err());
        drop(reader);
        let replacement = pool.reserve(60).unwrap();
        assert!(pool.reserve(u64::MAX).is_err());
        drop(local);
        drop(replacement);
        assert_eq!(pool.used.load(Ordering::Acquire), 0);
    }
}
