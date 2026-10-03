use super::*;
use crate::{agent_capture::PendingCaptureStateV1, agent_capture_files::sync_dir};
use jj_agent::CaptureBatchId;
use jj_store_wal::publication::Receipt;
use std::fs;
impl AgentCaptureSpool {
    /// Only the exact native drain may turn a confirmed receipt into coverage.
    pub(crate) fn complete_committed(
        &mut self,
        batch: CaptureBatchId,
        receipt: &Receipt,
    ) -> CaptureResult<()> {
        let _lock = self.mutation_lock()?;
        let path = self.path(batch);
        let entry = self.read(&path)?;
        let PendingCaptureStateV1::Prepared { request } = &entry.state else {
            return Err("unprepared publication cannot be confirmed".into());
        };
        crate::agent_publication::validate_receipt(request, receipt)?;
        if let Some((key, range)) = ranges::of(&entry)?
            && let Some(mut catalog) = files::load(self.state_root())?
            && catalog.channels.iter().any(|c| c.key == key)
        {
            catalog.confirm(key, &range, ranges::Evidence::of(receipt)?)?;
            // On any failure retain the exact Prepared intent and native receipt retry.
            files::persist(self.state_root(), &catalog)?;
        }
        self.record_reader_progress(&entry)?;
        fs::remove_file(path)?;
        sync_dir(&self.state_root().join("pending"))
    }
}
