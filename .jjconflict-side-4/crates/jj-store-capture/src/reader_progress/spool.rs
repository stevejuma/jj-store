use super::{CaptureResult, ReaderCapture, ReaderProgressV1, history};
use crate::agent_capture::{AgentCaptureSpool, DurableCaptureAck, PendingCaptureV1};
use jj_agent::{IntegrationId, SourceInstanceId};
use jj_store_model::RepositoryId;

impl AgentCaptureSpool {
    pub fn enqueue_from_reader(
        &mut self,
        input: ReaderCapture,
    ) -> CaptureResult<DurableCaptureAck> {
        self.enqueue_reader(input, false)
    }
    /// Automatic source-channel allocation is monotonic across physical generations.
    /// Exact already-durable retries precede this check and retain their identities.
    pub fn enqueue_from_automatic_reader(
        &mut self,
        input: ReaderCapture,
    ) -> CaptureResult<DurableCaptureAck> {
        self.enqueue_reader(input, true)
    }
    fn enqueue_reader(
        &mut self,
        input: ReaderCapture,
        monotonic: bool,
    ) -> CaptureResult<DurableCaptureAck> {
        self.enqueue_candidate_checked(
            PendingCaptureV1 {
                schema_version: 1,
                repository: input.repository,
                profile: input.profile,
                cursor: input.cursor,
                batch: input.batch,
                state: crate::agent_capture::PendingCaptureStateV1::Captured,
                thread_binding: input.thread_binding,
                reader_metadata: Some(input.metadata),
            },
            monotonic,
        )
    }
    pub fn reader_progress(
        &self,
        repository: RepositoryId,
        integration: IntegrationId,
        source: SourceInstanceId,
        generation: u64,
    ) -> CaptureResult<Option<ReaderProgressV1>> {
        if generation == 0 {
            return Err("reader generation must be positive".into());
        }
        let _lock = self.mutation_lock()?;
        crate::reader_progress_files::load(
            self.state_root(),
            repository,
            integration,
            source,
            generation,
        )
    }
    /// Reconstruct the latest sanitized durable physical cursor across repository partitions.
    pub fn latest_reader_progress(
        &self,
        integration: IntegrationId,
        source: SourceInstanceId,
    ) -> CaptureResult<Option<ReaderProgressV1>> {
        let _lock = self.mutation_lock()?;
        history::latest(&self.retained_progress()?, integration, source, None)
    }
    pub(crate) fn retained_progress(&self) -> CaptureResult<Vec<ReaderProgressV1>> {
        let mut records = crate::reader_progress_files::records(self.state_root())?;
        for pending in self.pending_locked()? {
            if let Some(progress) = ReaderProgressV1::from_pending(&pending)? {
                records.push(progress);
            }
        }
        Ok(records)
    }
    pub(crate) fn validate_reader_admission(&self, entry: &PendingCaptureV1) -> CaptureResult<()> {
        let Some(progress) = ReaderProgressV1::from_pending(entry)? else {
            return Ok(());
        };
        let records = self.retained_progress()?;
        if let Some(old) = history::latest(
            &records,
            progress.cursor.integration,
            progress.cursor.source_instance_id,
            Some(progress.cursor.generation),
        )? {
            progress.validate_forward(&old)?;
            if progress.last_sequence == old.last_sequence {
                return Err("reader offset already acknowledged".into());
            }
        }
        let existing = crate::reader_progress_files::records(self.state_root())?;
        if !existing.iter().any(|old| same_key(old, &progress))
            && existing.len() >= crate::reader_progress_files::MAX_PROGRESS
        {
            return Err("reader progress capacity exhausted".into());
        }
        Ok(())
    }
    pub(crate) fn record_reader_progress(&self, entry: &PendingCaptureV1) -> CaptureResult<()> {
        let Some(progress) = ReaderProgressV1::from_pending(entry)? else {
            return Ok(());
        };
        let records = crate::reader_progress_files::records(self.state_root())?;
        history::validate_candidate(&records, &progress)?;
        let existing = crate::reader_progress_files::load(
            self.state_root(),
            progress.repository,
            progress.cursor.integration,
            progress.cursor.source_instance_id,
            progress.cursor.generation,
        )?;
        if let Some(old) = &existing {
            progress.validate_binding(old)?;
            if progress.last_sequence < old.last_sequence {
                return Ok(());
            }
            progress.validate_forward(old)?;
            if progress == *old {
                return Ok(());
            }
        } else if records.len() >= crate::reader_progress_files::MAX_PROGRESS {
            return Err("reader progress capacity exhausted".into());
        }
        crate::reader_progress_files::persist(self.state_root(), &progress)
    }
    pub(crate) fn recover_reader_progress(&self) -> CaptureResult<()> {
        crate::reader_progress_files::recover(self.state_root())?;
        let mut known = crate::reader_progress_files::records(self.state_root())?;
        history::validate(&known)?;
        let mut pending = self.pending_locked()?;
        pending.sort_by(crate::agent_publication::capture_order);
        for entry in pending {
            let Some(progress) = ReaderProgressV1::from_pending(&entry)? else {
                continue;
            };
            history::validate_candidate(&known, &progress)?;
            if let Some(old) = known.iter_mut().find(|old| same_key(old, &progress)) {
                progress.validate_binding(old)?;
                if progress.last_sequence < old.last_sequence {
                    continue;
                }
                progress.validate_forward(old)?;
                if progress != *old {
                    crate::reader_progress_files::persist(self.state_root(), &progress)?;
                    *old = progress;
                }
            } else {
                if known.len() >= crate::reader_progress_files::MAX_PROGRESS {
                    return Err("reader progress capacity exhausted".into());
                }
                crate::reader_progress_files::persist(self.state_root(), &progress)?;
                known.push(progress);
            }
        }
        Ok(())
    }
}
fn same_key(left: &ReaderProgressV1, right: &ReaderProgressV1) -> bool {
    left.repository == right.repository && history::same_generation(left, right)
}
