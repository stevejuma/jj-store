//! Process-owned configuration. Publication leases fence policy replacement.
use crate::native::SessionLimits;
use jj_store_jj::policy::Protection;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Configuration {
    pub protection: Protection,
    pub limits: SessionLimits,
    pub max_blob: u64,
    pub concurrent: usize,
}
pub struct RuntimePolicy {
    values: RwLock<Configuration>,
    active: [std::sync::atomic::AtomicUsize; 2],
    pub publication: Arc<tokio::sync::RwLock<()>>,
}
impl RuntimePolicy {
    pub fn new(values: Configuration) -> Arc<Self> {
        Arc::new(Self {
            active: Default::default(),
            values: RwLock::new(values),
            publication: Arc::new(Default::default()),
        })
    }
    pub fn permit(self: &Arc<Self>, scope: usize) -> Option<Permit> {
        let limit = self.current().concurrent;
        self.active[scope]
            .try_update(
                std::sync::atomic::Ordering::AcqRel,
                std::sync::atomic::Ordering::Acquire,
                |n| (n < limit).then_some(n + 1),
            )
            .ok()?;
        Some(Permit(self.clone(), scope))
    }
    pub fn current(&self) -> Configuration {
        self.values.read().expect("runtime policy poisoned").clone()
    }
    /// The caller owns the exclusive publication lease.
    pub fn replace(&self, values: Configuration) {
        *self.values.write().expect("runtime policy poisoned") = values;
    }
}

pub struct Permit(Arc<RuntimePolicy>, usize);
impl Drop for Permit {
    fn drop(&mut self) {
        self.0.active[self.1].fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    }
}
