//! Weak global LRU registration; cache ownership is separate from live readers.
use super::*;

pub(super) trait Entry: Send + Sync {
    fn used(&self) -> u64;
    fn present(&self) -> bool;
    fn evict_idle(&self);
}
pub(crate) struct Retained<T> {
    value: Mutex<Option<Arc<T>>>,
    used: AtomicU64,
    pool: Arc<Pool>,
}
impl<T: Send + Sync + 'static> Retained<T> {
    pub(crate) fn new(value: Arc<T>) -> Arc<Self> {
        Self::in_pool(value, Pool::global().clone())
    }
    fn in_pool(value: Arc<T>, pool: Arc<Pool>) -> Arc<Self> {
        let retained = Arc::new(Self {
            value: Mutex::new(Some(value)),
            used: AtomicU64::new(pool.clock.fetch_add(1, Ordering::AcqRel)),
            pool: pool.clone(),
        });
        let entry: Arc<dyn Entry> = retained.clone();
        let mut entries = pool.retained.lock().expect("resident registry");
        entries.retain(|entry| entry.upgrade().is_some_and(|entry| entry.present()));
        entries.push(Arc::downgrade(&entry));
        retained
    }
    pub(crate) fn get(&self) -> Option<Arc<T>> {
        self.get_if(|_| true)
    }
    pub(crate) fn get_if(&self, matches: impl FnOnce(&T) -> bool) -> Option<Arc<T>> {
        let value = self
            .value
            .lock()
            .expect("resident slot")
            .as_ref()
            .filter(|value| matches(value))
            .cloned();
        if value.is_some() {
            self.used.store(
                self.pool.clock.fetch_add(1, Ordering::AcqRel),
                Ordering::Release,
            );
        }
        value
    }
    pub(crate) fn present(&self) -> bool {
        self.value.lock().expect("resident slot").is_some()
    }
    pub(crate) fn evict_idle_if(&self, matches: impl FnOnce(&T) -> bool) -> bool {
        let value = {
            let Ok(mut value) = self.value.try_lock() else {
                return false;
            };
            if value
                .as_ref()
                .is_some_and(|value| Arc::strong_count(value) == 1 && matches(value))
            {
                value.take()
            } else {
                None
            }
        };
        let reclaimed = value.is_some();
        // File and allocation cleanup can acquire other locks.
        drop(value);
        reclaimed
    }
}
impl<T: Send + Sync + 'static> Entry for Retained<T> {
    fn used(&self) -> u64 {
        self.used.load(Ordering::Acquire)
    }
    fn present(&self) -> bool {
        self.present()
    }
    fn evict_idle(&self) {
        self.evict_idle_if(|_| true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selective_idle_reclamation_preserves_active_and_unmatched_values() {
        let pool = Arc::new(Pool::new(100));
        let one = Retained::in_pool(Arc::new(pool.reserve(40).unwrap()), pool.clone());
        let two = Retained::in_pool(Arc::new(pool.reserve(40).unwrap()), pool.clone());
        let active = one.get().unwrap();
        assert!(!one.evict_idle_if(|_| true));
        assert!(!two.evict_idle_if(|_| false));
        assert!(two.evict_idle_if(|_| true));
        assert_eq!(pool.used.load(Ordering::Acquire), 40);
        drop(active);
        assert!(one.evict_idle_if(|_| true));
        assert_eq!(pool.used.load(Ordering::Acquire), 0);
    }
    #[test]
    fn shared_pool_reclaims_idle_native_and_local_generations_before_growth() {
        let pool = Arc::new(Pool::new(100));
        let native = Retained::in_pool(Arc::new(pool.reserve(40).unwrap()), pool.clone());
        let local = Retained::in_pool(Arc::new(pool.reserve(40).unwrap()), pool.clone());
        let reader = native.get().unwrap(); // Make local the older idle generation.
        let next = pool
            .reserve(50)
            .expect("idle retained allocation prevented new work");
        assert!(native.present());
        assert!(!local.present());
        assert_eq!(pool.used.load(Ordering::Acquire), 90);
        assert!(matches!(pool.reserve(51), Err(Error::Limit)));
        drop(reader);
        let replacement = pool.reserve(40).unwrap();
        assert!(!native.present());
        assert_eq!(pool.used.load(Ordering::Acquire), 90);
        drop(next);
        drop(replacement);
        assert_eq!(pool.used.load(Ordering::Acquire), 0);
    }

    #[test]
    fn shared_pool_eviction_follows_access_order_and_preserves_oversized_denials() {
        let pool = Arc::new(Pool::new(100));
        let older = Retained::in_pool(Arc::new(pool.reserve(40).unwrap()), pool.clone());
        let newer = Retained::in_pool(Arc::new(pool.reserve(40).unwrap()), pool.clone());
        drop(older.get());
        assert!(matches!(pool.reserve(101), Err(Error::Limit)));
        assert!(older.present() && newer.present());
        let next = pool.reserve(50).unwrap();
        assert!(older.present());
        assert!(!newer.present());
        drop(older);
        drop(newer);
        drop(next);
        assert_eq!(pool.used.load(Ordering::Acquire), 0);
    }

    #[test]
    fn shared_pool_admission_does_not_wait_on_busy_cache_slots() {
        let pool = Arc::new(Pool::new(100));
        let retained = Retained::in_pool(Arc::new(pool.reserve(80).unwrap()), pool.clone());
        let locked = retained.value.lock().unwrap();
        assert!(matches!(pool.reserve(30), Err(Error::Limit)));
        assert_eq!(pool.used.load(Ordering::Acquire), 80);
        drop(locked);
        let next = pool.reserve(30).unwrap();
        assert!(!retained.present());
        drop(next);
        assert_eq!(pool.used.load(Ordering::Acquire), 0);
    }
}
