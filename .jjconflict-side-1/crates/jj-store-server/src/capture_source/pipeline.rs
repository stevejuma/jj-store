//! Independent bounded lanes; the supervisor owns scheduling and blocking I/O.
use super::*;

// Raw device paths/reader state intentionally have no Debug or serialization.
pub struct CodexCapturePipeline {
    pub(super) source: CodexCaptureSource,
}
impl CodexCaptureSource {
    /// Consume the conservative serial driver; queued intent remains durable.
    pub fn into_pipeline(mut self) -> CodexCapturePipeline {
        self.pending = None;
        self.recovery = None;
        CodexCapturePipeline { source: self }
    }
}
impl CodexCapturePipeline {
    pub(crate) fn compact_idle(&mut self) -> bool {
        if self.source.recovery.is_some()
            || self.source.pending.is_some()
            || !self.source.reader.compact_idle()
        {
            return false;
        }
        self.source.sanitizer = None;
        true
    }
    pub(crate) fn anchor(&self) -> &VerifiedCaptureWorkspace {
        &self.source.anchor
    }
    pub fn partition(&self) -> CaptureDrainPartition {
        self.source.partition()
    }
    /// Admit and sanitize one bounded batch regardless of prior publication state.
    pub async fn ingest_step(
        &mut self,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<CaptureSourceStep> {
        self.source.ingest_step(context, false).await
    }
    /// Recover this exact selected range; another entry cannot inherit its proof.
    pub async fn publish_step(
        &mut self,
        entry: &PendingCaptureV1,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<CaptureSourceStep> {
        self.source.check_spool(context.spool)?;
        if entry.batch.integration != self.source.profile.integration
            || entry.batch.source_instance_id != self.source.reader.source_instance_id()
        {
            return Err(Error::Service("capture publication source differs"));
        }
        self.source
            .recover_entry_step(entry.clone(), context, false)
            .await
    }
}
