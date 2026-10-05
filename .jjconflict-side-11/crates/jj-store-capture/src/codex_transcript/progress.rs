use super::*;
use crate::reader_progress::ReaderSourceProgress;
impl CodexTranscriptReader {
    pub(crate) fn refresh_source_progress(
        &mut self,
        bounds: &ReaderSourceProgress,
    ) -> CaptureResult<()> {
        if let Some(progress) = &bounds.progress {
            progress.validate()?;
            if progress.metadata.generation_digest != self.header.witness
                || progress.cursor.integration != self.profile.integration()
                || progress.cursor.source_instance_id != self.source
                || progress.cursor.generation > bounds.generation_floor
                || progress.last_sequence > bounds.sequence_floor
            {
                return Err("transcript source progress mismatch".into());
            }
        }
        if let Some(pending) = &self.pending {
            // An allocated raw identity is immutable even when another file advances.
            if pending.batch.records[0].sequence <= bounds.sequence_floor
                || pending.batch.cursor.generation < bounds.generation_floor
                || (pending.batch.cursor.generation == bounds.generation_floor
                    && bounds
                        .progress
                        .as_ref()
                        .is_none_or(|p| p.cursor.generation != pending.batch.cursor.generation))
            {
                return Err("transcript raw allocation superseded".into());
            }
            return Ok(());
        }
        self.sequence = self.sequence.max(bounds.sequence_floor);
        self.generation_floor = self.generation_floor.max(bounds.generation_floor);
        if self.generation <= bounds.generation_floor
            && bounds
                .progress
                .as_ref()
                .is_none_or(|p| p.cursor.generation != self.generation)
        {
            self.continue_generation()?;
        }
        Ok(())
    }
    // A delayed old-file append retains its verified prefix and context.
    pub(super) fn continue_generation(&mut self) -> CaptureResult<()> {
        self.generation = self
            .generation
            .max(self.generation_floor)
            .checked_add(1)
            .ok_or("transcript generation limit")?;
        Ok(())
    }
}
