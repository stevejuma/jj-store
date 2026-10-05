//! Imported readers cannot be converted to ordinary active pipelines.
use super::*;
use crate::capture_management::CaptureImportRenewal;

pub struct ImportedCodexCapturePipeline {
    pub(super) pipeline: CodexCapturePipeline,
    pub(super) renewal: CaptureImportRenewal,
    pub(super) sources: Arc<SourceRegistry>,
    pub(super) probe: Option<CodexImportProbe>,
}
impl ImportedCodexCapturePipeline {
    pub(super) fn new(
        pipeline: CodexCapturePipeline,
        renewal: CaptureImportRenewal,
        sources: Arc<SourceRegistry>,
    ) -> Self {
        Self {
            pipeline,
            renewal,
            sources,
            probe: None,
        }
    }
    pub fn receipt(&self) -> &CaptureImportReceipt {
        self.renewal.receipt()
    }
    pub fn partition(&self) -> CaptureDrainPartition {
        self.pipeline.partition()
    }
    pub(crate) fn anchor(&self) -> &VerifiedCaptureWorkspace {
        self.pipeline.anchor()
    }
    pub(crate) fn compact_idle(&mut self) -> bool {
        self.probe.as_ref().is_none_or(CodexImportProbe::is_compact) && self.pipeline.compact_idle()
    }
    /// Current consent and accepted prefix precede any common-reader body work.
    pub async fn ingest_step(
        &mut self,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<CaptureSourceStep> {
        let renewal = self.renewal.clone();
        let mut settlement = renewal
            .settle(context.admission)
            .await
            .map_err(policy_error)?;
        if let Some(step) = self.prepare(&mut settlement, context).await? {
            return Ok(step);
        }
        let probe = self.probe.as_ref().expect("prepared import prefix");
        self.pipeline
            .source
            .ingest_guarded(context, false, &|| current(&settlement, probe))
            .await
    }
    /// Exact queued-range recovery remains the common publication implementation.
    pub async fn publish_step(
        &mut self,
        entry: &PendingCaptureV1,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<CaptureSourceStep> {
        let renewal = self.renewal.clone();
        let mut settlement = renewal
            .settle(context.admission)
            .await
            .map_err(policy_error)?;
        if let Some(step) = self.prepare(&mut settlement, context).await? {
            return Ok(step);
        }
        if entry.batch.integration != self.pipeline.source.profile.integration
            || entry.batch.source_instance_id != self.pipeline.source.reader.source_instance_id()
        {
            return Err(Error::Service("capture publication source differs"));
        }
        let probe = self.probe.as_ref().expect("prepared import prefix");
        self.pipeline
            .source
            .recover_entry_guarded(entry.clone(), context, false, &|| {
                current(&settlement, probe)
            })
            .await
    }
    async fn prepare(
        &mut self,
        settlement: &mut CaptureImportSettlement,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<Option<CaptureSourceStep>> {
        self.pipeline.source.check_spool(context.spool)?;
        settlement.check().map_err(policy_error)?;
        let qualified = self.pipeline.source.reader.qualified();
        let (retained, _) = settlement.prepare_source().map_err(policy_error)?;
        if retained.fingerprint() != qualified.fingerprint() {
            return Ok(Some(CaptureSourceStep::Withheld));
        }
        let snapshot = match prefix(&mut self.probe, qualified, self.renewal.receipt())? {
            CodexImportPoll::Verifying => return Ok(Some(CaptureSourceStep::Verifying)),
            CodexImportPoll::ChangedDuringRead => {
                return Ok(Some(CaptureSourceStep::ChangedDuringRead));
            }
            CodexImportPoll::Ready(snapshot) => snapshot,
        };
        let Some(proof) = admit(&snapshot, qualified, context).await? else {
            return Ok(Some(CaptureSourceStep::Withheld));
        };
        context
            .authority
            .bind(context.host, context.admission, &proof)
            .await?;
        let probe = self.probe.as_ref().expect("prepared import prefix");
        current(settlement, probe)?;
        if self
            .sources
            .lookup_import_admitted(probe, snapshot.binding(), || {
                settlement.check().map_err(|e| e.into())
            })
            .map_err(|_| Error::Service("capture imported eligibility unavailable"))?
            != Some(self.pipeline.partition().source)
        {
            return Ok(Some(CaptureSourceStep::Withheld));
        }
        let previous = context
            .spool
            .source_reader_progress(
                self.pipeline.source.profile.integration,
                self.pipeline.source.reader.source_instance_id(),
                self.pipeline.source.reader.qualified().generation_digest(),
            )
            .map_err(|_| Error::Service("capture imported reader progress unavailable"))?;
        if !self
            .probe
            .as_mut()
            .expect("prepared import prefix")
            .verify_reader_progress(previous.progress.as_ref())
            .map_err(|_| Error::Service("capture imported reader prefix unavailable"))?
        {
            return Ok(Some(CaptureSourceStep::Verifying));
        }
        let probe = self.probe.as_ref().expect("prepared import prefix");
        settlement
            .bind_reader(&self.pipeline.source.reader)
            .map_err(policy_error)?;
        current(settlement, probe)?;
        self.pipeline.source.anchor = proof;
        Ok(None)
    }
    pub fn park(self) -> ParkedImportedCodexCapturePipeline {
        ParkedImportedCodexCapturePipeline::new(
            self.pipeline.park_automatic(),
            self.renewal,
            self.sources,
            self.probe,
        )
    }
}
pub(super) fn prefix(
    probe: &mut Option<CodexImportProbe>,
    qualified: &QualifiedCodexSource,
    receipt: &CaptureImportReceipt,
) -> Result<CodexImportPoll> {
    // A completed cached witness may outlive a rewrite or append. Reconstruct
    // accepted bytes at the new stable snapshot before granting any body work.
    // Incomplete bounded scans remain retained across worker/parking steps.
    if let Some(ready) = probe.as_ref()
        && ready.is_compact()
        && !stable(ready)?
    {
        *probe = None;
    }
    if probe.is_none() {
        *probe = Some(
            CodexImportProbe::reconcile(qualified.clone(), receipt.source.codex())
                .map_err(|_| Error::Service("capture accepted prefix unavailable"))?,
        );
    }
    probe
        .as_mut()
        .expect("retained import prefix")
        .poll()
        .map_err(|_| Error::Service("capture accepted prefix changed"))
}
pub(super) fn current(
    settlement: &CaptureImportSettlement,
    probe: &CodexImportProbe,
) -> Result<()> {
    settlement.check().map_err(policy_error)?;
    if !stable(probe)? {
        return Err(Error::Service("capture accepted snapshot changed"));
    }
    Ok(())
}
