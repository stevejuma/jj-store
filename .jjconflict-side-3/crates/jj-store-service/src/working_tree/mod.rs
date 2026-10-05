//! Private physical observations. Neither a JJ snapshot nor command authority.
mod walk;
use crate::stack_workspace::PhysicalWorkspaceId;
use jj_store_model::{RecordDigest, RepositoryId};
use sha2::{Digest as _, Sha256};
use std::{
    collections::BTreeMap,
    path::Path,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("invalid physical workspace entry")]
    Invalid,
    #[error("physical workspace scan limit")]
    Limit,
    #[error("physical workspace changed during scan")]
    Stale,
    #[error("unsupported physical workspace entry")]
    Unsupported,
    #[error("physical workspace unavailable")]
    Unavailable,
    #[error("physical workspace scan cancelled")]
    Cancelled,
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Read,
    Verify,
}
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    /// Cumulative visited entries across both passes, including metadata names.
    pub max_entries: usize,
    /// Cumulative content bytes read across both passes.
    pub max_bytes: usize,
    pub max_file_bytes: usize,
    pub max_depth: usize,
    pub timeout: Duration,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_entries: 8192,
            max_bytes: 16 * 1024 * 1024,
            max_file_bytes: 2 * 1024 * 1024,
            max_depth: 64,
            timeout: Duration::from_secs(2),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    File { executable: bool },
    Symlink,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub path: String,
    pub kind: Kind,
    pub bytes: u64,
    /// Raw bytes (or link target), independent of timestamps and physical identity.
    pub content: RecordDigest,
    /// Exact physical identity, mode, timestamps and bytes.
    pub fingerprint: RecordDigest,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkingTree {
    pub version: u32,
    pub repository: RepositoryId,
    pub generation: u64,
    pub workspace: PhysicalWorkspaceId,
    pub fingerprint: RecordDigest,
    pub entries: Vec<Entry>,
}
struct Budget<'a, F> {
    limits: Limits,
    start: Instant,
    entries: usize,
    bytes: usize,
    guard: &'a mut F,
    phase: Phase,
}
impl<F: FnMut(Phase) -> Result<()>> Budget<'_, F> {
    fn check(&mut self) -> Result<()> {
        (self.guard)(self.phase)?;
        if self.start.elapsed() >= self.limits.timeout {
            return Err(Error::Limit);
        }
        Ok(())
    }
    fn entry(&mut self) -> Result<()> {
        self.check()?;
        self.entries += 1;
        if self.entries > self.limits.max_entries {
            return Err(Error::Limit);
        }
        Ok(())
    }
    fn bytes(&mut self, bytes: usize) -> Result<()> {
        self.check()?;
        self.bytes = self.bytes.checked_add(bytes).ok_or(Error::Limit)?;
        if self.bytes > self.limits.max_bytes {
            return Err(Error::Limit);
        }
        Ok(())
    }
}
fn digest(domain: &[u8], data: &[u8]) -> RecordDigest {
    let mut hash = Sha256::new_with_prefix(domain);
    hash.update(data);
    RecordDigest::new(hash.finalize().into())
}
/// Caller supplies registration/authorization/cancellation checks throughout.
/// Ignored files remain obstacles; this inventory is not an Unstaged assignment.
pub fn scan(
    root: &Path,
    repository: RepositoryId,
    generation: u64,
    limits: Limits,
    mut guard: impl FnMut(Phase) -> Result<()>,
) -> Result<WorkingTree> {
    let cap = Limits::default();
    if !root.is_absolute() {
        return Err(Error::Invalid);
    }
    if limits.max_entries == 0
        || limits.max_entries > cap.max_entries
        || limits.max_bytes == 0
        || limits.max_bytes > cap.max_bytes
        || limits.max_file_bytes == 0
        || limits.max_file_bytes > cap.max_file_bytes
        || limits.max_depth == 0
        || limits.max_depth > cap.max_depth
        || limits.timeout.is_zero()
        || limits.timeout > cap.timeout
    {
        return Err(Error::Limit);
    }
    let mut budget = Budget {
        limits,
        start: Instant::now(),
        entries: 0,
        bytes: 0,
        guard: &mut guard,
        phase: Phase::Read,
    };
    let first = walk::read(root, repository, generation, &mut budget)?;
    budget.phase = Phase::Verify;
    let second = walk::read(root, repository, generation, &mut budget)?;
    budget.check()?;
    if first != second {
        return Err(Error::Stale);
    }
    Ok(second)
}
