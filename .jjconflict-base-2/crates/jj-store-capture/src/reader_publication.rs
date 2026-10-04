//! Body-free device evidence of exact native publication, separate from reader ACKs.
use crate::{
    CaptureResult,
    agent_capture::{AgentCaptureSpool, PendingCaptureV1},
};
use jj_agent::{IntegrationId, SourceInstanceId};
mod catalog;
mod files;
mod inspection;
mod projection;
mod ranges;
mod snapshot;
mod spool;
use catalog::Catalog;
pub use inspection::ReaderPublicationInspection;
use ranges::{Channel, Key, Range};
pub use snapshot::{
    SnapshotPublicationInspection, SnapshotPublicationProgress, SnapshotPublicationQuery,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReaderPublicationProgress {
    pub generation: u64,
    pub history_available: bool,
    pub captured_byte_offset: u64,
    pub committed_byte_offset: u64,
    pub outstanding_ranges: usize,
    pub receipts: u64,
}

impl AgentCaptureSpool {
    /// Reads private bounded metadata only; never reads or enrolls a transcript.
    pub fn reader_publication_progress(
        &self,
        integration: IntegrationId,
        source: SourceInstanceId,
        generation_digest: [u8; 32],
    ) -> CaptureResult<Option<ReaderPublicationProgress>> {
        if generation_digest == [0; 32] {
            return Err("invalid publication generation digest".into());
        }
        let _lock = self.mutation_lock()?;
        let Some(catalog) = files::load(self.state_root())? else {
            return Ok(None);
        };
        projection::physical_history(
            &catalog,
            &self.retained_progress()?,
            integration,
            source,
            generation_digest,
        )
    }
    pub(crate) fn register_reader_publication(
        &self,
        entry: &PendingCaptureV1,
        initialize: bool,
    ) -> CaptureResult<()> {
        let Some((key, range)) = ranges::of(entry)? else {
            return Ok(());
        };
        let mut catalog = match files::load(self.state_root())? {
            Some(catalog) => catalog,
            None if initialize => Catalog::default(),
            None => return Ok(()),
        };
        if !catalog.channels.iter().any(|c| c.key == key) {
            if !initialize {
                return Ok(());
            }
            let prior = self.retained_progress()?.into_iter().any(|p| {
                p.cursor.integration == key.integration
                    && p.cursor.source_instance_id == key.source
                    && p.cursor.generation == key.generation
            });
            catalog.add(Channel::new(key, range.clone(), !prior))?;
        }
        catalog.register(key, range)?;
        files::persist(self.state_root(), &catalog)
    }
    pub(crate) fn recover_reader_publication(&self) -> CaptureResult<()> {
        let Some(mut catalog) = files::load(self.state_root())? else {
            return Ok(());
        };
        let mut pending = self.pending_locked()?;
        pending.sort_by(crate::agent_publication::capture_order);
        for entry in pending {
            let Some((key, range)) = ranges::of(&entry)? else {
                continue;
            };
            if catalog.channels.iter().any(|c| c.key == key) {
                catalog.register(key, range)?;
            }
        }
        files::persist(self.state_root(), &catalog)
    }
}
