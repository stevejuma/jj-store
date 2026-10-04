//! Conservative allocation allowances, distinct from source/blob admission.
use crate::work::{Error, Result};
use jj_store_wal::publication::WorkStateV1;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

pub(super) struct Budget {
    limit: usize,
    used: AtomicUsize,
}
impl Budget {
    pub fn new(limit: usize) -> Self {
        Self {
            limit,
            used: AtomicUsize::new(0),
        }
    }
    pub fn limit(&self) -> usize {
        self.limit
    }
    pub fn reserve(self: &Arc<Self>, bytes: usize) -> Result<Lease> {
        self.add(bytes)?;
        Ok(Lease {
            budget: self.clone(),
            bytes,
        })
    }
    fn add(&self, bytes: usize) -> Result<()> {
        if bytes > self.limit {
            return Err(Error::Limit);
        }
        self.used
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                used.checked_add(bytes).filter(|next| *next <= self.limit)
            })
            .map_err(|_| Error::Busy)?;
        Ok(())
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
    pub fn resize(&mut self, bytes: usize) -> Result<()> {
        if bytes > self.budget.limit() {
            return Err(Error::Limit);
        }
        if bytes > self.bytes {
            self.budget.add(bytes - self.bytes)?;
        } else {
            self.budget
                .used
                .fetch_sub(self.bytes - bytes, Ordering::AcqRel);
        }
        self.bytes = bytes;
        Ok(())
    }
}
impl Drop for Lease {
    fn drop(&mut self) {
        self.budget.used.fetch_sub(self.bytes, Ordering::AcqRel);
    }
}
fn sum(mut values: impl Iterator<Item = usize>) -> Result<usize> {
    values.try_fold(0usize, |total, value| {
        total.checked_add(value).ok_or(Error::Limit)
    })
}
fn scaled(value: usize, factor: usize) -> Result<usize> {
    value.checked_mul(factor).ok_or(Error::Limit)
}
pub(super) fn decoding(encoded: usize) -> Result<usize> {
    // JSON has at least 35 bytes per 16-byte map key. Six times its encoded
    // size covers tree nodes, decoded string growth and canonical serialization
    // scratch (including that Vec's geometric capacity). Fixed overhead covers
    // map roots and bounded validation sets. Source buffers are admitted upstream.
    scaled(encoded, 6)?
        .checked_add(1024 * 1024)
        .ok_or(Error::Limit)
}
pub(super) fn query_capacity(state: &WorkStateV1) -> Result<usize> {
    // Vec growth up to twice the 16-byte key count, query text and Arc/map slots.
    scaled(state.projects.len().max(state.tasks.len()), 32)?
        .checked_add(4096)
        .ok_or(Error::Limit)
}
pub(super) fn resident(state: &WorkStateV1) -> Result<(usize, usize)> {
    let entities = state.projects.len() + state.tasks.len() + state.comments.len();
    let memberships = sum(state.projects.values().map(|p| p.repositories.len()))?;
    let text = sum(state
        .projects
        .values()
        .map(|p| p.title.capacity() + p.body.capacity())
        .chain(
            state
                .tasks
                .values()
                .map(|t| t.title.capacity() + t.body.capacity()),
        )
        .chain(
            state
                .comments
                .values()
                .map(|c| c.body.as_ref().map_or(0, String::capacity)),
        ))?;
    let searchable = sum(state
        .projects
        .values()
        .map(|p| p.title.len() + p.body.len())
        .chain(state.tasks.values().map(|t| t.title.len() + t.body.len())))?;
    // 1 KiB per source record and 128 bytes per membership bound BTree nodes;
    // 2 KiB per entity bounds projection vectors/maps/counts/activity. Unicode
    // lowercase may expand; six bytes per source byte covers its grown capacity.
    let resident = sum([
        text,
        scaled(memberships, 128)?,
        scaled(entities, 3072)?,
        scaled(searchable, 6)?,
        4096,
    ]
    .into_iter())?;
    // Root retention includes all eight potential query slots. Each key set has
    // its own live lease, which survives eviction until its last Arc exits.
    let retention = resident
        .checked_add(scaled(query_capacity(state)?, 8)?)
        .ok_or(Error::Limit)?;
    Ok((resident, retention))
}
