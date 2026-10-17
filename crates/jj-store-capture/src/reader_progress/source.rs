//! Bounded physical selection and channel allocation floors; no new durable format.
use super::ReaderProgressV1;
#[cfg(feature = "application")]
use super::{CaptureResult, history};
#[cfg(feature = "application")]
use crate::agent_capture::{AgentCaptureSpool, PendingCaptureV1};
#[cfg(feature = "application")]
use jj_agent::{IntegrationId, SourceInstanceId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReaderSourceProgress {
    pub progress: Option<ReaderProgressV1>,
    pub generation_floor: u64,
    pub sequence_floor: u64,
}
#[cfg(feature = "application")]
impl AgentCaptureSpool {
    /// Select one exact physical/header witness and global floors in one locked read.
    pub fn source_reader_progress(
        &self,
        integration: IntegrationId,
        source: SourceInstanceId,
        generation_digest: [u8; 32],
    ) -> CaptureResult<ReaderSourceProgress> {
        if generation_digest == [0; 32] {
            return Err("invalid reader generation digest".into());
        }
        let _lock = self.mutation_lock()?;
        self.source_reader_progress_locked(integration, source, generation_digest)
    }
    fn source_reader_progress_locked(
        &self,
        integration: IntegrationId,
        source: SourceInstanceId,
        digest: [u8; 32],
    ) -> CaptureResult<ReaderSourceProgress> {
        let pending = self.pending_locked()?;
        let mut retained = crate::reader_progress_files::records(self.state_root())?;
        for entry in &pending {
            if let Some(progress) = ReaderProgressV1::from_pending(entry)? {
                retained.push(progress);
            }
        }
        let records: Vec<_> = retained
            .into_iter()
            .filter(|p| {
                p.cursor.integration == integration && p.cursor.source_instance_id == source
            })
            .collect();
        history::validate(&records)?;
        let progress = records
            .iter()
            .filter(|p| p.metadata.generation_digest == digest)
            .max_by_key(|p| (p.cursor.generation, p.last_sequence))
            .cloned();
        let pending: Vec<_> = pending
            .iter()
            .filter(|entry| {
                entry.cursor.integration == integration && entry.cursor.source_instance_id == source
            })
            .collect();
        Ok(ReaderSourceProgress {
            progress,
            generation_floor: records
                .iter()
                .map(|p| p.cursor.generation)
                .chain(pending.iter().map(|entry| entry.cursor.generation))
                .max()
                .unwrap_or(0),
            sequence_floor: records
                .iter()
                .map(|p| p.last_sequence)
                .chain(pending.iter().map(|entry| {
                    entry
                        .batch
                        .observations
                        .last()
                        .expect("validated batch")
                        .source_sequence
                }))
                .max()
                .unwrap_or(0),
        })
    }
    pub(crate) fn validate_monotonic_reader(&self, entry: &PendingCaptureV1) -> CaptureResult<()> {
        let metadata = entry
            .reader_metadata
            .as_ref()
            .ok_or("reader metadata missing")?;
        let bounds = self.source_reader_progress_locked(
            entry.cursor.integration,
            entry.cursor.source_instance_id,
            metadata.generation_digest,
        )?;
        if entry.cursor.generation < bounds.generation_floor
            || (entry.cursor.generation == bounds.generation_floor
                && bounds
                    .progress
                    .as_ref()
                    .is_none_or(|p| p.cursor.generation != entry.cursor.generation))
            || entry
                .batch
                .observations
                .first()
                .ok_or("reader observations missing")?
                .source_sequence
                <= bounds.sequence_floor
        {
            return Err("automatic reader allocation superseded".into());
        }
        Ok(())
    }
}
