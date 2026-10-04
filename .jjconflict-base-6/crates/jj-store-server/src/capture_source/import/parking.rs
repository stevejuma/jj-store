//! Retain original consent and raw channel across failed native admission awaits.
use super::*;
use crate::capture_management::CaptureImportRenewal;
use jj_store_capture::source_registry::validate_parked_progress;

pub struct ParkedImportedCodexCapturePipeline {
    parked: ParkedCodexCapturePipeline,
    renewal: CaptureImportRenewal,
    sources: Arc<SourceRegistry>,
    probe: Option<CodexImportProbe>,
}
impl ParkedImportedCodexCapturePipeline {
    pub(super) fn new(
        parked: ParkedCodexCapturePipeline,
        renewal: CaptureImportRenewal,
        sources: Arc<SourceRegistry>,
        probe: Option<CodexImportProbe>,
    ) -> Self {
        Self {
            parked,
            renewal,
            sources,
            probe,
        }
    }
    pub fn receipt(&self) -> &CaptureImportReceipt {
        self.renewal.receipt()
    }
    pub(crate) fn anchor(&self) -> Option<&VerifiedCaptureWorkspace> {
        self.parked.anchor()
    }
    pub(crate) fn compact_idle(&mut self) -> bool {
        self.probe.as_ref().is_none_or(CodexImportProbe::is_compact) && self.parked.compact_idle()
    }
    /// Failure retains the frame. Only renewed prefix/consent and native Write transfer it.
    pub async fn resume_current(
        &mut self,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<CaptureImportStep> {
        let renewal = self.renewal.clone();
        let settlement = renewal
            .settle(context.admission)
            .await
            .map_err(policy_error)?;
        let source = self
            .parked
            .source
            .as_mut()
            .ok_or(Error::Service("capture imported frame consumed"))?;
        check_spool(&source.spool_binding, context.spool)?;
        let (retained, _) = settlement.prepare_source().map_err(policy_error)?;
        if retained.fingerprint() != source.reader.qualified().fingerprint() {
            return Ok(CaptureImportStep::Withheld);
        }
        let snapshot = match pipeline::prefix(
            &mut self.probe,
            source.reader.qualified(),
            self.renewal.receipt(),
        )? {
            CodexImportPoll::Verifying => return Ok(CaptureImportStep::Verifying),
            CodexImportPoll::ChangedDuringRead => return Ok(CaptureImportStep::ChangedDuringRead),
            CodexImportPoll::Ready(snapshot) => snapshot,
        };
        let Some(proof) = admit(&snapshot, source.reader.qualified(), context).await? else {
            return Ok(CaptureImportStep::Withheld);
        };
        context
            .authority
            .bind(context.host, context.admission, &proof)
            .await?;
        let probe = self.probe.as_ref().expect("prepared import prefix");
        pipeline::current(&settlement, probe)?;
        let expected = source.reader.source_instance_id();
        if self
            .sources
            .lookup_import_admitted(probe, snapshot.binding(), || {
                settlement.check().map_err(|e| e.into())
            })
            .map_err(|_| Error::Service("capture imported eligibility unavailable"))?
            != Some(expected)
        {
            return Ok(CaptureImportStep::Withheld);
        }
        let bounds = context
            .spool
            .source_reader_progress(
                source.profile.integration,
                expected,
                source.reader.qualified().generation_digest(),
            )
            .map_err(|e| Error::capture_spool(e, "capture imported progress unavailable"))?;
        validate_parked_progress(self.parked.progress.as_ref(), bounds.progress.as_ref())
            .map_err(|_| Error::Service("capture imported progress regressed or missing"))?;
        let advanced = bounds.progress != self.parked.progress;
        // Preserve the automatic channel in this frame throughout subsequent awaits.
        let reserved = if source.reader.has_channel_reservation() {
            Some(
                source
                    .reader
                    .acquire_reserved_import(&self.sources, probe, snapshot.binding(), || {
                        settlement.check().map_err(|e| e.into())
                    })
                    .map_err(|_| Error::Service("capture imported lease unavailable"))?
                    .ok_or(Error::Service("capture imported lease changed"))?,
            )
        } else {
            None
        };
        let registered = if reserved.is_none() {
            Some(
                self.sources
                    .acquire_import(probe, snapshot.binding(), || {
                        settlement.check().map_err(|e| e.into())
                    })
                    .map_err(|_| Error::Service("capture imported lease unavailable"))?,
            )
        } else {
            None
        };
        if registered
            .as_ref()
            .is_some_and(|s| s.source_instance_id() != expected)
        {
            return Err(Error::Service("capture imported channel changed"));
        }
        context
            .authority
            .bind(context.host, context.admission, &proof)
            .await?;
        pipeline::current(&settlement, probe)?;
        check_spool(&source.spool_binding, context.spool)?;
        if context
            .spool
            .source_reader_progress(
                source.profile.integration,
                expected,
                source.reader.qualified().generation_digest(),
            )
            .map_err(|e| Error::capture_spool(e, "capture imported progress unavailable"))?
            != bounds
        {
            return Err(Error::Service("capture imported progress changed"));
        }
        let reader = if let Some(reserved) = reserved {
            source
                .reader
                .resume_reserved_at(reserved, &bounds, advanced)
                .map_err(|_| Error::Service("capture imported reserved reader unavailable"))?
                .ok_or(Error::Service("capture imported reader changed"))?
        } else {
            let registered = registered.expect("fresh imported acquisition");
            if advanced {
                registered
                    .open_reader_at(&bounds)
                    .map_err(|_| Error::Service("capture imported reader unavailable"))?
                    .ok_or(Error::Service("capture imported reader changed"))?
            } else {
                source
                    .reader
                    .resume_at(registered, &bounds)
                    .map_err(|_| Error::Service("capture imported reader unavailable"))?
            }
        };
        let mut source = self
            .parked
            .source
            .take()
            .expect("retained imported frame")
            .map_reader(|_| reader);
        source.anchor = proof;
        if advanced {
            source.needs_resume = false;
            source.reader_progress = bounds.progress;
            source.pending = None;
            source.recovery = None;
        }
        let mut pipeline = ImportedCodexCapturePipeline::new(
            CodexCapturePipeline { source },
            self.renewal.clone(),
            self.sources.clone(),
        );
        pipeline.probe = self.probe.take();
        Ok(CaptureImportStep::Ready(Box::new(pipeline)))
    }
}
