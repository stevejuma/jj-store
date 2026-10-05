//! Allocation charges remain owned after cache eviction.
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

pub(super) struct Budget {
    pub limit: usize,
    used: AtomicUsize,
}
impl Budget {
    pub fn new(limit: usize) -> Arc<Self> {
        Arc::new(Self {
            limit,
            used: AtomicUsize::new(0),
        })
    }
    fn add(&self, bytes: usize) -> Option<()> {
        self.used
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                used.checked_add(bytes).filter(|next| *next <= self.limit)
            })
            .ok()?;
        Some(())
    }
    pub fn reserve(self: &Arc<Self>, bytes: usize) -> Option<Lease> {
        self.add(bytes)?;
        Some(Lease {
            budget: self.clone(),
            bytes,
        })
    }
    #[cfg(test)]
    pub fn used(&self) -> usize {
        self.used.load(Ordering::Acquire)
    }
}
pub(super) struct Lease {
    budget: Arc<Budget>,
    bytes: usize,
}
impl Lease {
    pub fn resize(&mut self, bytes: usize) -> Option<()> {
        if bytes > self.bytes {
            self.budget.add(bytes - self.bytes)?;
        } else {
            self.budget
                .used
                .fetch_sub(self.bytes - bytes, Ordering::AcqRel);
        }
        self.bytes = bytes;
        Some(())
    }
}
impl Drop for Lease {
    fn drop(&mut self) {
        self.budget.used.fetch_sub(self.bytes, Ordering::AcqRel);
    }
}
