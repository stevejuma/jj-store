//! Owned identity-only copy lookup. No upstream types cross the transport facade.
use super::*;
use jj_store_jj::query::{self as q, copy_index};
use tokio::time::Instant;

#[derive(Clone, Copy, Debug)]
pub struct CopyFamilyLimits {
    pub max_nodes: usize,
    pub max_work: usize,
    pub max_bytes: usize,
}
impl Default for CopyFamilyLimits {
    fn default() -> Self {
        Self {
            max_nodes: 100_000,
            max_work: 1_000_000,
            max_bytes: 64 * 1024 * 1024,
        }
    }
}
#[derive(Debug)]
pub struct CopyFamily {
    pub ids: BTreeSet<m::CopyId>,
    pub work: usize,
}
#[derive(Debug, thiserror::Error)]
pub enum CopyFamilyError {
    #[error("copy family access denied")]
    Denied,
    #[error("copy family resource limit")]
    Limit,
    #[error("copy seed is not selected")]
    NotCommitted,
    #[error("copy family selection changed")]
    EpochChanged,
    #[error("copy family data is corrupt")]
    Corrupt,
    #[error("copy family unavailable")]
    Unavailable,
}
impl From<q::Error> for CopyFamilyError {
    fn from(error: q::Error) -> Self {
        match error {
            q::Error::Denied => Self::Denied,
            q::Error::NotCommitted => Self::NotCommitted,
            q::Error::EpochChanged => Self::EpochChanged,
            q::Error::Corrupt | q::Error::Incomplete => Self::Corrupt,
            q::Error::Limit
            | q::Error::Busy
            | q::Error::IndexBudget
            | q::Error::IndexRebuildRequired
            | q::Error::Canceled => Self::Limit,
            _ => Self::Unavailable,
        }
    }
}
impl NativeRequest {
    /// Trusted endpoint ceilings for seed admission before transport conversion.
    pub fn copy_family_limits(&self) -> CopyFamilyLimits {
        let ceiling = CopyFamilyLimits::default();
        CopyFamilyLimits {
            max_nodes: ceiling.max_nodes.min(self.endpoint.limits.inventory_nodes),
            ..ceiling
        }
    }
    /// Ordinary remote backend selection. Explicitly pinned browser providers
    /// continue to call their original selected-root capability independently.
    pub async fn copy_family(
        &self,
        seeds: &BTreeSet<m::CopyId>,
        required: &BTreeSet<m::CopyId>,
        limits: CopyFamilyLimits,
        deadline: Instant,
    ) -> std::result::Result<CopyFamily, CopyFamilyError> {
        let ceiling = self.copy_family_limits();
        let limits = copy_index::LookupLimits {
            max_nodes: limits.max_nodes.min(ceiling.max_nodes),
            max_work: limits.max_work.min(ceiling.max_work),
            max_bytes: limits.max_bytes.min(ceiling.max_bytes),
        };
        if limits.max_nodes == 0
            || limits.max_work == 0
            || limits.max_bytes == 0
            || seeds.len() > limits.max_nodes
            || !required.is_subset(seeds)
            || seeds
                .len()
                .checked_mul(512)
                .is_none_or(|bytes| bytes > limits.max_bytes)
            || Instant::now() >= deadline
        {
            return Err(CopyFamilyError::Limit);
        }
        let deadline = deadline.min(
            Instant::now()
                .checked_add(self.endpoint.limits.timeout)
                .ok_or(CopyFamilyError::Limit)?,
        );
        let family = tokio::time::timeout_at(deadline, async {
            let selected = self
                .session
                .snapshot()
                .await
                .map_err(|error| CopyFamilyError::from(crate::reads::kind_error(error)))?;
            self.session
                .copy_family_at(&selected, seeds, required, limits, deadline)
                .await
                .map_err(CopyFamilyError::from)
        })
        .await
        .map_err(|_| CopyFamilyError::Limit)??;
        if Instant::now() >= deadline {
            return Err(CopyFamilyError::Limit);
        }
        Ok(CopyFamily {
            ids: family.ids,
            work: family.work,
        })
    }
}
