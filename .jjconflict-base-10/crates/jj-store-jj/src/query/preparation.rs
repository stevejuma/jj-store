//! Trusted construction policy, independent of ordinary page admission.
use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_commits: usize,
    pub max_work: usize,
    pub max_read_bytes: u64,
    pub max_metadata_nodes: u64,
    pub timeout: Duration,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_commits: 4_000_000,
            max_work: 500_000_000,
            max_read_bytes: 16 * 1024 * 1024 * 1024,
            max_metadata_nodes: 1_000_000_000,
            timeout: Duration::from_secs(3600),
        }
    }
}
impl Limits {
    pub(super) fn validate(self) -> Result<()> {
        let ceiling = Self::default();
        if self.max_commits == 0
            || self.max_commits > ceiling.max_commits
            || self.max_work == 0
            || self.max_work > ceiling.max_work
            || self.max_read_bytes == 0
            || self.max_read_bytes > ceiling.max_read_bytes
            || self.max_metadata_nodes == 0
            || self.max_metadata_nodes > ceiling.max_metadata_nodes
            || self.timeout.is_zero()
            || self.timeout > ceiling.timeout
        {
            return Err(Error::Limit);
        }
        Ok(())
    }
    pub(super) fn budget(self, interactive: super::Limits) -> super::Limits {
        super::Limits {
            max_index_commits: self.max_commits.min(interactive.max_index_commits),
            max_work: self.max_work,
            max_object_bytes: self.max_read_bytes,
            metadata: admission::Limits {
                max_nodes: self.max_metadata_nodes,
                ..interactive.metadata
            },
            timeout: self.timeout,
            ..interactive
        }
    }
}
