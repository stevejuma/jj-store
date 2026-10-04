//! Bounded current context grants a new ingress anchor, never old body authority.
use super::*;
use jj_store_capture::codex_resume::{CodexResumePoll, CodexResumeProbe};

pub enum CaptureParkedResumeStep {
    Verifying,
    ChangedDuringRead,
    Withheld,
    Ready(Box<CodexCapturePipeline>),
}
impl ParkedCodexCapturePipeline {
    /// One current-context step; failure retains the exact parked reading frame.
    pub async fn resume_current(
        &mut self,
        sources: Arc<SourceRegistry>,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<CaptureParkedResumeStep> {
        let source = self
            .source
            .as_ref()
            .ok_or(Error::Service("capture parked frame consumed"))?;
        check_spool(source, context.spool)?;
        let qualified = source.reader.qualified().clone();
        if sources
            .lookup_admitted(&qualified)
            .map_err(|_| Error::Service("capture parked eligibility unavailable"))?
            != Some(source.reader.source_instance_id())
        {
            return Ok(CaptureParkedResumeStep::Withheld);
        }
        let progress = context
            .spool
            .source_reader_progress(
                source.profile.integration,
                source.reader.source_instance_id(),
                source.reader.qualified().generation_digest(),
            )
            .map_err(|error| Error::capture_spool(error, "capture parked progress unavailable"))?
            .progress;
        validate_parked_progress(self.progress.as_ref(), progress.as_ref())
            .map_err(|_| Error::Service("capture parked progress regressed or missing"))?;
        // Unchanged admitted ingress takes the ordinary metadata-only path. No
        // body reconstruction is needed unless current authority is unavailable.
        if self.current.is_none()
            && context
                .authority
                .bind(context.host, context.admission, &source.anchor)
                .await
                .is_ok()
        {
            let proof = source.anchor.clone();
            return self
                .transfer(sources, proof, context, false)
                .await
                .map(outcome);
        }
        if self.current.is_none() {
            self.current = Some(Box::new(
                CodexResumeProbe::new(qualified.clone())
                    .map_err(|_| Error::Service("capture parked context unavailable"))?,
            ));
        }
        let probe = self.current.as_mut().expect("retained current preparation");
        let directory = match probe
            .poll()
            .map_err(|_| Error::Service("capture parked context unavailable"))?
        {
            CodexResumePoll::Verifying => return Ok(CaptureParkedResumeStep::Verifying),
            CodexResumePoll::ChangedDuringRead => {
                return Ok(CaptureParkedResumeStep::ChangedDuringRead);
            }
            CodexResumePoll::Ready(directory) => directory,
        };
        for path in [
            directory.directory(),
            directory.prior_directory(),
            Some(qualified.directory()),
        ]
        .into_iter()
        .flatten()
        {
            let Some(path) = path.to_str() else { continue };
            let proof = match context.admission.inspect(path).await {
                Ok(proof) => proof,
                Err(
                    crate::capture_admission::Error::Unbound
                    | crate::capture_admission::Error::Limit,
                ) => continue,
                Err(_) => return Ok(CaptureParkedResumeStep::Withheld),
            };
            if !valid_snapshot(self.current.as_deref(), true)? {
                return Ok(CaptureParkedResumeStep::ChangedDuringRead);
            }
            return self
                .transfer(sources, proof, context, true)
                .await
                .map(outcome);
        }
        Ok(CaptureParkedResumeStep::Withheld)
    }
}
fn outcome(pipeline: Option<CodexCapturePipeline>) -> CaptureParkedResumeStep {
    pipeline.map_or(CaptureParkedResumeStep::Withheld, |pipeline| {
        CaptureParkedResumeStep::Ready(Box::new(pipeline))
    })
}
pub(super) fn valid_snapshot(probe: Option<&CodexResumeProbe>, required: bool) -> Result<bool> {
    if !required {
        return Ok(true);
    }
    probe
        .ok_or(Error::Service("capture parked preparation unavailable"))?
        .revalidate()
        .map_err(|_| Error::Service("capture parked context unavailable"))
}
