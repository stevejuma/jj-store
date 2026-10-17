//! Trusted read budgets shared by native and Git readers.
#[cfg(feature = "native")]
use super::{Error, Result};
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct MetadataLimits {
    pub max_nodes: u64,
    pub max_merge_terms: usize,
    pub max_parent_edges: usize,
    pub max_tree_entries: usize,
}
impl Default for MetadataLimits {
    fn default() -> Self {
        Self {
            max_nodes: 4_000_000,
            max_merge_terms: 65,
            max_parent_edges: 256,
            max_tree_entries: 50_000,
        }
    }
}
impl MetadataLimits {
    #[cfg(feature = "native")]
    pub(crate) fn validate(self) -> Result<()> {
        if !(1..=16_000_000).contains(&self.max_nodes)
            || !(1..=257).contains(&self.max_merge_terms)
            || !(1..=1024).contains(&self.max_parent_edges)
            || !(1..=100_000).contains(&self.max_tree_entries)
        {
            return Err(Error::Limit);
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug)]
pub struct PredicateLimits {
    pub max_file_bytes: u64,
    pub max_total_file_bytes: u64,
    pub max_lines: u64,
    pub max_merge_terms: usize,
}
impl Default for PredicateLimits {
    fn default() -> Self {
        Self {
            max_file_bytes: 256 * 1024,
            max_total_file_bytes: 4 * 1024 * 1024,
            max_lines: 1000,
            max_merge_terms: 65,
        }
    }
}
impl PredicateLimits {
    #[cfg(feature = "native")]
    pub(crate) fn validate(self) -> Result<()> {
        if !(1..=1024 * 1024).contains(&self.max_file_bytes)
            || !(1..=16 * 1024 * 1024).contains(&self.max_total_file_bytes)
            || !(1..=4096).contains(&self.max_lines)
            || !(1..=257).contains(&self.max_merge_terms)
        {
            return Err(Error::Limit);
        }
        Ok(())
    }
}
#[derive(Clone, Copy)]
pub struct GraphLimits {
    pub metadata: MetadataLimits,
    pub predicates: PredicateLimits,
    pub max_page: usize,
    pub max_index_commits: usize,
    pub max_inline_rebuild_commits: usize,
    pub max_work: usize,
    pub max_object_bytes: u64,
    pub max_output_bytes: usize,
    pub max_revset_bytes: usize,
    pub timeout: Duration,
}
impl Default for GraphLimits {
    fn default() -> Self {
        Self {
            metadata: MetadataLimits::default(),
            predicates: PredicateLimits::default(),
            max_page: 500,
            max_index_commits: 4_000_000,
            max_inline_rebuild_commits: 1_000,
            max_work: 1_000_000,
            max_object_bytes: 256 * 1024 * 1024,
            max_output_bytes: 8 * 1024 * 1024,
            max_revset_bytes: 4096,
            timeout: Duration::from_secs(10),
        }
    }
}
#[derive(Clone, Copy)]
pub struct ContentLimits {
    pub metadata: MetadataLimits,
    pub max_blob_bytes: u64,
    pub max_read_bytes: u64,
    pub max_output_bytes: usize,
    pub max_work: usize,
    pub max_history_visits: usize,
    pub max_changes: usize,
    pub max_text_bytes: usize,
    pub max_diff_cells: usize,
    pub max_page: usize,
    pub timeout: Duration,
}
impl Default for ContentLimits {
    fn default() -> Self {
        Self {
            metadata: MetadataLimits::default(),
            max_blob_bytes: 256 * 1024 * 1024,
            max_read_bytes: 512 * 1024 * 1024,
            max_output_bytes: 16 * 1024 * 1024,
            max_work: 1_000_000,
            max_history_visits: 50_000,
            max_changes: 1_000,
            max_text_bytes: 256 * 1024,
            max_diff_cells: 250_000,
            max_page: 500,
            timeout: Duration::from_secs(30),
        }
    }
}
