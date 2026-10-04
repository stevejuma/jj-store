//! Explicit history data flow. The caller independently owns bounded work and storage.
use super::*;
use crate::{
    capture_import_state::{CaptureImportReceipt, CaptureImportSourceBinding},
    capture_management::{CaptureImportDispatch, CaptureImportPolicy, CaptureImportSettlement},
};
use jj_store_capture::{
    codex_import::{CodexImportPoll, CodexImportProbe, CodexImportSnapshot},
    source_registry::{QualifiedCodexSource, SourceRegistry},
};
use std::sync::Arc;
mod parking;
mod pipeline;
pub use parking::ParkedImportedCodexCapturePipeline;
pub use pipeline::ImportedCodexCapturePipeline;

pub enum CaptureImportStep {
    Verifying,
    ChangedDuringRead,
    Withheld,
    Ready(Box<ImportedCodexCapturePipeline>),
}
pub struct CodexCaptureImport {
    settlement: Option<CaptureImportSettlement>,
    dispatch: CaptureImportDispatch,
    qualified: QualifiedCodexSource,
    probe: Option<CodexImportProbe>,
    consumed: bool,
    accepted: Option<jj_store_capture::codex_import::CodexImportBinding>,
    sources: Arc<SourceRegistry>,
    privacy: CapturePrivacyRoots,
    binding: CaptureSpoolBinding,
}
impl CodexCaptureImport {
    /// Run on independently retained bounded work, after reserving a reader frame.
    /// Current policy precedes all source-body inspection; cached repository is no proof.
    pub fn prepare(
        policy: CaptureImportPolicy,
        sources: Arc<SourceRegistry>,
        privacy: CapturePrivacyRoots,
        spool: &AgentCaptureSpool,
    ) -> Result<Self> {
        let settlement = policy.into_settlement().map_err(policy_error)?;
        let (qualified, probe) = settlement.prepare_source().map_err(policy_error)?;
        let accepted = settlement.receipt().map(|r| r.source.codex());
        let dispatch = settlement.dispatch(qualified.clone());
        Ok(Self {
            settlement: Some(settlement),
            dispatch,
            qualified,
            probe: Some(probe),
            consumed: false,
            accepted,
            sources,
            privacy,
            binding: spool
                .binding()
                .map_err(|_| Error::Service("capture import spool unavailable"))?,
        })
    }
    pub fn receipt(&self) -> Option<&CaptureImportReceipt> {
        self.settlement.as_ref().and_then(|s| s.receipt())
    }
    /// Called only after an owned step settles or its future is dropped.
    /// Prefix state stays in its reserved frame; current gates are renewed next step.
    pub(crate) fn release_policy(&mut self) {
        self.settlement = None;
    }
    pub(crate) fn compact(&self) -> bool {
        self.probe.as_ref().is_none_or(CodexImportProbe::is_compact)
    }
    /// One bounded prefix step. Native Write and durable intent precede enrollment.
    /// The supervisor must retain this driver, its frame and storage across awaits.
    pub async fn step(
        &mut self,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<CaptureImportStep> {
        if self.consumed {
            return Err(Error::Service("capture import consumed"));
        }
        if self.settlement.is_none() {
            self.settlement = Some(
                self.dispatch
                    .begin(context.admission)
                    .await
                    .map_err(policy_error)?
                    .into_settlement()
                    .map_err(policy_error)?,
            );
        }
        let settlement = self
            .settlement
            .as_mut()
            .ok_or(Error::Service("capture import consumed"))?;
        settlement.check().map_err(policy_error)?;
        check_spool(&self.binding, context.spool)?;
        if let Some(receipt) = settlement.receipt()
            && self.accepted != Some(receipt.source.codex())
        {
            self.probe = Some(
                CodexImportProbe::reconcile(self.qualified.clone(), receipt.source.codex())
                    .map_err(|_| Error::Service("capture accepted prefix unavailable"))?,
            );
            self.accepted = Some(receipt.source.codex());
        }
        let snapshot = match self
            .probe
            .as_mut()
            .expect("retained import probe")
            .poll()
            .map_err(|_| Error::Service("capture import prefix unavailable"))?
        {
            CodexImportPoll::Verifying => return Ok(CaptureImportStep::Verifying),
            CodexImportPoll::ChangedDuringRead => return Ok(CaptureImportStep::ChangedDuringRead),
            CodexImportPoll::Ready(snapshot) => snapshot,
        };
        settlement.check().map_err(policy_error)?;
        let Some(proof) = admit(&snapshot, &self.qualified, context).await? else {
            return Ok(CaptureImportStep::Withheld);
        };
        context
            .authority
            .bind(context.host, context.admission, &proof)
            .await?;
        settlement.check().map_err(policy_error)?;
        let probe = self.probe.as_ref().expect("retained import probe");
        if !stable(probe)? {
            return Ok(CaptureImportStep::ChangedDuringRead);
        }
        check_spool(&self.binding, context.spool)?;
        let selection = settlement
            .select(&CaptureImportSourceBinding::from_codex(snapshot.binding()))
            .map_err(policy_error)?;
        let accepted = selection.receipt.source.codex();
        self.probe
            .as_mut()
            .expect("retained import probe")
            .retain_accepted(accepted)
            .map_err(|_| Error::Service("capture accepted snapshot changed"))?;
        self.accepted = Some(accepted);
        let probe = self.probe.as_ref().expect("retained import probe");
        // This synchronous selection runs within the caller's owned blocking frame.
        let registered = self
            .sources
            .acquire_import(probe, accepted, || settlement.check().map_err(|e| e.into()))
            .map_err(|_| Error::Service("capture import enrollment unavailable"))?;
        context
            .authority
            .bind(context.host, context.admission, &proof)
            .await?;
        settlement.check().map_err(policy_error)?;
        if !stable(probe)? {
            return Ok(CaptureImportStep::ChangedDuringRead);
        }
        let source = CodexCaptureSource::open(
            registered,
            super::acquire::normalizer(self.qualified.profile())?,
            proof,
            self.privacy.clone(),
            context.spool,
        )?;
        if !self
            .probe
            .as_mut()
            .expect("retained import probe")
            .verify_reader_progress(source.reader_progress.as_ref())
            .map_err(|_| Error::Service("capture imported reader prefix unavailable"))?
        {
            return Ok(CaptureImportStep::Verifying);
        }
        let probe = self.probe.as_ref().expect("retained import probe");
        check_spool(&self.binding, context.spool)?;
        settlement.check().map_err(policy_error)?;
        if !stable(probe)? {
            return Ok(CaptureImportStep::ChangedDuringRead);
        }
        settlement
            .bind_reader(&source.reader)
            .map_err(policy_error)?;
        check_spool(&self.binding, context.spool)?;
        settlement.check().map_err(policy_error)?;
        if !stable(probe)? {
            return Ok(CaptureImportStep::ChangedDuringRead);
        }
        let renewal = settlement.renewal().map_err(policy_error)?;
        let mut pipeline = ImportedCodexCapturePipeline::new(
            source.into_pipeline(),
            renewal,
            self.sources.clone(),
        );
        pipeline.probe = self.probe.take();
        self.consumed = true;
        self.settlement = None;
        Ok(CaptureImportStep::Ready(Box::new(pipeline)))
    }
}
fn policy_error(_: jj_store_service::capture_management::Error) -> Error {
    Error::Service("capture import policy unavailable; retain original request")
}
fn check_spool(binding: &CaptureSpoolBinding, spool: &AgentCaptureSpool) -> Result<()> {
    if spool
        .binding()
        .map_err(|_| Error::Service("capture import spool unavailable"))?
        != *binding
    {
        return Err(Error::Service("capture import spool changed"));
    }
    Ok(())
}
fn stable(probe: &CodexImportProbe) -> Result<bool> {
    probe
        .revalidate()
        .map_err(|_| Error::Service("capture import prefix unavailable"))
}
async fn admit(
    snapshot: &CodexImportSnapshot,
    qualified: &QualifiedCodexSource,
    context: &CaptureStepContext<'_>,
) -> Result<Option<VerifiedCaptureWorkspace>> {
    for path in [
        snapshot.directory(),
        snapshot.prior_directory(),
        Some(qualified.directory()),
    ]
    .into_iter()
    .flatten()
    {
        let Some(path) = path.to_str() else { continue };
        match context.admission.inspect(path).await {
            Ok(proof) => return Ok(Some(proof)),
            Err(
                crate::capture_admission::Error::Unbound | crate::capture_admission::Error::Limit,
            ) => {}
            Err(_) => return Ok(None),
        }
    }
    Ok(None)
}
