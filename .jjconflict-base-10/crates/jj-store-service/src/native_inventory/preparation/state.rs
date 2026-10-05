use super::*;
use wal::inventory::SourceScope;

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_ids: usize,
    pub max_descriptors: usize,
    pub max_record_reads: usize,
    pub max_record_bytes: u64,
    pub max_bytes: usize,
    pub timeout: Duration,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_ids: 100_000,
            max_descriptors: 500_000_000,
            max_record_reads: 1_000_000_000,
            max_record_bytes: 16 * 1024 * 1024 * 1024,
            max_bytes: 64 * 1024 * 1024,
            timeout: Duration::from_secs(3600),
        }
    }
}
impl Limits {
    pub(crate) fn validate(self) -> q::Result<()> {
        let ceiling = Self::default();
        if self.max_ids == 0
            || self.max_ids > ceiling.max_ids
            || self.max_descriptors == 0
            || self.max_descriptors > ceiling.max_descriptors
            || self.max_record_reads == 0
            || self.max_record_reads > ceiling.max_record_reads
            || self.max_record_bytes == 0
            || self.max_record_bytes > ceiling.max_record_bytes
            || self.max_bytes < HEADER
            || self.max_bytes > ceiling.max_bytes
            || self.timeout.is_zero()
            || self.timeout > ceiling.timeout
        {
            return Err(Error::Limit);
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, schemars::JsonSchema)]
#[schemars(rename = "KindPreparationStats")]
pub struct Stats {
    pub windows: usize,
    pub scanned: usize,
    /// Successful descriptors plus conservative full-window charges after an
    /// unknown/failed/canceled read; retrying cannot hide that spent work.
    pub descriptor_work: usize,
    pub record_reads: usize,
    pub record_bytes: u64,
}
#[derive(Clone, Copy, Debug, Default, serde::Serialize, schemars::JsonSchema)]
#[schemars(rename = "KindPreparationProgress")]
pub struct Progress {
    pub complete: bool,
    pub matching_ids: usize,
    pub stats: Stats,
}
/// Owned immutable data and a cursor. No source, principal, credential,
/// publication capability or async task is retained here. Drop releases memory.
pub struct Preparation {
    pub(super) selected: ManifestSnapshot,
    pub(super) scope: SourceScope,
    pub(super) copies: bool,
    pub(super) limits: Limits,
    pub(super) expires: Instant,
    pub(super) after: Option<m::RecordDigest>,
    pub(super) ids: BTreeSet<m::ObjectKey>,
    pub(super) lease: Option<cache::Lease>,
    pub(super) ready: Option<Arc<cache::Data>>,
    pub(super) stats: Stats,
}
impl Preparation {
    pub(crate) fn deadline(&self) -> Instant {
        self.expires
    }
    pub fn progress(&self) -> Progress {
        Progress {
            complete: self.ready.is_some(),
            matching_ids: self
                .ready
                .as_ref()
                .map_or(self.ids.len(), |data| data.ids.len()),
            stats: self.stats,
        }
    }
    pub(super) fn reserve(
        &mut self,
        descriptors: usize,
        reads: usize,
        bytes: u64,
    ) -> q::Result<()> {
        let next = Stats {
            windows: self
                .stats
                .windows
                .checked_add(1)
                .ok_or(Error::IndexBudget)?,
            scanned: self.stats.scanned,
            descriptor_work: self
                .stats
                .descriptor_work
                .checked_add(descriptors)
                .ok_or(Error::IndexBudget)?,
            record_reads: self
                .stats
                .record_reads
                .checked_add(reads)
                .ok_or(Error::IndexBudget)?,
            record_bytes: self
                .stats
                .record_bytes
                .checked_add(bytes)
                .ok_or(Error::IndexBudget)?,
        };
        if next.descriptor_work > self.limits.max_descriptors
            || next.record_reads > self.limits.max_record_reads
            || next.record_bytes > self.limits.max_record_bytes
        {
            return Err(Error::IndexBudget);
        }
        self.stats = next;
        Ok(())
    }
}
