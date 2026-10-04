//! Retained frame. Admission and cursor checks precede every lease handoff.
use super::*;
use jj_store_capture::{
    reader_progress::ReaderProgressV1,
    source_registry::{ParkedCodexTranscriptReader, SourceRegistry, validate_parked_progress},
};
use std::sync::Arc;
mod acquire;
mod current;
pub use current::CaptureParkedResumeStep;

// Raw reader/context and device paths intentionally have no Debug or encoding.
pub struct ParkedCodexCapturePipeline {
    pub(super) source: Option<CodexCaptureSource<ParkedCodexTranscriptReader>>,
    pub(super) progress: Option<ReaderProgressV1>,
    current: Option<Box<jj_store_capture::codex_resume::CodexResumeProbe>>,
}
impl CodexCapturePipeline {
    /// Caller must bound retained frames separately from concurrently held leases.
    pub fn park(self) -> ParkedCodexCapturePipeline {
        self.park_with(|reader| reader.park())
    }
    /// Retain a raw allocation's channel within the supervisor's bounded frame.
    pub(crate) fn park_automatic(self) -> ParkedCodexCapturePipeline {
        self.park_with(|reader| reader.park_automatic())
    }
    fn park_with(
        self,
        park: impl FnOnce(
            jj_store_capture::source_registry::LeasedCodexTranscriptReader,
        ) -> ParkedCodexTranscriptReader,
    ) -> ParkedCodexCapturePipeline {
        let progress = self.source.reader_progress.clone();
        ParkedCodexCapturePipeline {
            source: Some(self.source.map_reader(park)),
            progress,
            current: None,
        }
    }
}
impl ParkedCodexCapturePipeline {
    pub(crate) async fn idle_unchanged(
        &self,
        sources: &SourceRegistry,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<bool> {
        let source = self
            .source
            .as_ref()
            .ok_or(Error::Service("capture parked frame consumed"))?;
        check_spool(source, context.spool)?;
        if self.current.is_some() {
            return Ok(false);
        }
        if sources
            .lookup_admitted(source.reader.qualified())
            .map_err(|_| Error::Service("capture parked eligibility unavailable"))?
            != Some(source.reader.source_instance_id())
        {
            return Ok(false);
        }
        context
            .authority
            .bind(context.host, context.admission, &source.anchor)
            .await?;
        let bounds = context
            .spool
            .source_reader_progress(
                source.profile.integration,
                source.reader.source_instance_id(),
                source.reader.qualified().generation_digest(),
            )
            .map_err(|error| Error::capture_spool(error, "capture parked progress unavailable"))?;
        let current = bounds.progress;
        validate_parked_progress(self.progress.as_ref(), current.as_ref())
            .map_err(|_| Error::Service("capture parked progress regressed or missing"))?;
        check_spool(source, context.spool)?;
        Ok(current == self.progress
            && source
                .reader
                .idle_unchanged()
                .map_err(|_| Error::Service("capture parked source unavailable"))?)
    }
    pub(crate) fn compact_idle(&mut self) -> bool {
        let Some(source) = self.source.as_mut() else {
            return false;
        };
        if source.recovery.is_some()
            || source.pending.is_some()
            || self.current.as_ref().is_some_and(|p| !p.is_compact())
            || !source.reader.compact_idle()
        {
            return false;
        }
        source.sanitizer = None;
        true
    }
    pub(crate) fn anchor(&self) -> Option<&VerifiedCaptureWorkspace> {
        self.source.as_ref().map(|source| &source.anchor)
    }
    /// Failure retains the exact frame; success consumes it once. A newly advanced
    /// durable cursor replaces stale in-memory reading only after fresh acquisition.
    pub async fn resume(
        &mut self,
        sources: Arc<SourceRegistry>,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<Option<CodexCapturePipeline>> {
        let proof = self
            .source
            .as_ref()
            .ok_or(Error::Service("capture parked frame consumed"))?
            .anchor
            .clone();
        self.transfer(sources, proof, context, false).await
    }
    async fn transfer(
        &mut self,
        sources: Arc<SourceRegistry>,
        proof: VerifiedCaptureWorkspace,
        context: &mut CaptureStepContext<'_>,
        current_snapshot: bool,
    ) -> Result<Option<CodexCapturePipeline>> {
        let source = self
            .source
            .as_mut()
            .ok_or(Error::Service("capture parked frame consumed"))?;
        check_spool(source, context.spool)?;
        let qualified = source.reader.qualified().clone();
        let expected = source.reader.source_instance_id();
        if sources
            .lookup_admitted(&qualified)
            .map_err(|_| Error::Service("capture parked eligibility unavailable"))?
            != Some(expected)
        {
            return Ok(None);
        }
        context
            .authority
            .bind(context.host, context.admission, &proof)
            .await?;
        let bounds = context
            .spool
            .source_reader_progress(
                source.profile.integration,
                expected,
                qualified.generation_digest(),
            )
            .map_err(|error| Error::capture_spool(error, "capture parked progress unavailable"))?;
        let current = bounds.progress.clone();
        validate_parked_progress(self.progress.as_ref(), current.as_ref())
            .map_err(|_| Error::Service("capture parked progress regressed or missing"))?;
        if !current::valid_snapshot(self.current.as_deref(), current_snapshot)? {
            return Ok(None);
        }
        let sources_for_check = sources.clone();
        let registered = acquire::acquire(&source.reader, sources, context).await?;
        let Some(registered) = registered else {
            return Ok(None);
        };
        context
            .admission
            .revalidate(&proof)
            .await
            .map_err(|_| Error::Service("capture parked admission changed"))?;
        check_spool(source, context.spool)?;
        if context
            .spool
            .source_reader_progress(
                source.profile.integration,
                expected,
                source.reader.qualified().generation_digest(),
            )
            .map_err(|error| Error::capture_spool(error, "capture parked progress unavailable"))?
            != bounds
        {
            return Err(Error::Service("capture parked progress changed"));
        }
        if !current::valid_snapshot(self.current.as_deref(), current_snapshot)? {
            return Ok(None);
        }
        if sources_for_check
            .lookup_admitted(source.reader.qualified())
            .map_err(|_| Error::Service("capture parked eligibility unavailable"))?
            != Some(expected)
        {
            return Ok(None);
        }
        let advanced = current != self.progress;
        let reader = registered.open(&mut source.reader, &bounds, advanced)?;
        let mut source = self
            .source
            .take()
            .expect("retained frame")
            .map_reader(|_| reader);
        source.anchor = proof;
        if advanced {
            source.needs_resume = false;
            source.reader_progress = current;
            source.pending = None;
            source.recovery = None;
        }
        self.current = None;
        Ok(Some(CodexCapturePipeline { source }))
    }
}
fn check_spool<R>(source: &CodexCaptureSource<R>, spool: &AgentCaptureSpool) -> Result<()> {
    if spool
        .binding()
        .map_err(|_| Error::Service("capture spool binding unavailable"))?
        != source.spool_binding
    {
        return Err(Error::Service("capture spool binding changed"));
    }
    Ok(())
}
impl<R> CodexCaptureSource<R> {
    pub(super) fn map_reader<T>(self, replace: impl FnOnce(R) -> T) -> CodexCaptureSource<T> {
        CodexCaptureSource {
            spool_binding: self.spool_binding,
            reader: replace(self.reader),
            reader_progress: self.reader_progress,
            profile: self.profile,
            anchor: self.anchor,
            pending: self.pending,
            recovery: self.recovery,
            needs_resume: self.needs_resume,
            privacy: self.privacy,
            sanitizer: self.sanitizer,
        }
    }
}
