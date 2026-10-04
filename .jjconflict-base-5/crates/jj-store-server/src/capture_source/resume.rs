//! One-use staged resume; no current authority or source lease is retained across steps.
use super::*;
use jj_store_capture::{
    codex_activity::ActiveCodexSource,
    codex_resume::{CodexResumePoll, CodexResumeProbe},
    source_registry::{QualifiedCodexSource, SourceInstanceId, SourceRegistry},
};
use std::sync::Arc;
mod eligibility;
use eligibility::Eligibility;

// Device source/paths and raw context intentionally have no Debug/serialization.
pub enum CaptureResumeStep {
    Verifying,
    ChangedDuringRead,
    Withheld,
    Ready(Box<CodexCaptureSource>),
}
pub struct CodexCaptureResume {
    sources: Arc<SourceRegistry>,
    qualified: QualifiedCodexSource,
    eligibility: Eligibility,
    profile: NormalizerProfileV1,
    probe: CodexResumeProbe,
    privacy: CapturePrivacyRoots,
    spool_binding: CaptureSpoolBinding,
    finished: bool,
}
impl CodexCaptureResume {
    /// Existing native eligibility is required before any context-body inspection.
    /// This read-only preparation allocates no source identity, alias or reader lock.
    pub fn prepare(
        sources: Arc<SourceRegistry>,
        qualified: QualifiedCodexSource,
        privacy: CapturePrivacyRoots,
        spool: &AgentCaptureSpool,
    ) -> Result<Option<Self>> {
        let Some(source) = sources
            .lookup_admitted(&qualified)
            .map_err(|_| Error::Service("capture resume eligibility unavailable"))?
        else {
            return Ok(None);
        };
        Self::new(
            sources,
            qualified,
            privacy,
            spool,
            Eligibility::Admitted(source),
        )
        .map(Some)
    }
    /// A current proof of a genuinely new observed start replaces retained admission
    /// for this preparation only. Current directory and Write still precede enrollment.
    pub fn prepare_active(
        sources: Arc<SourceRegistry>,
        active: ActiveCodexSource,
        privacy: CapturePrivacyRoots,
        spool: &AgentCaptureSpool,
    ) -> Result<Self> {
        active
            .revalidate()
            .map_err(|_| Error::Service("capture active snapshot changed"))?;
        let qualified = active.qualified().clone();
        let this = Self::new(
            sources,
            qualified,
            privacy,
            spool,
            Eligibility::Active(Box::new(active)),
        )?;
        this.eligible()?;
        Ok(this)
    }
    fn new(
        sources: Arc<SourceRegistry>,
        qualified: QualifiedCodexSource,
        privacy: CapturePrivacyRoots,
        spool: &AgentCaptureSpool,
        eligibility: Eligibility,
    ) -> Result<Self> {
        let profile = super::acquire::normalizer(qualified.profile())?;
        let spool_binding = spool
            .binding()
            .map_err(|_| Error::Service("capture spool binding unavailable"))?;
        let probe = CodexResumeProbe::new(qualified.clone())
            .map_err(|_| Error::Service("capture resume source unavailable"))?;
        Ok(Self {
            sources,
            qualified,
            eligibility,
            profile,
            probe,
            privacy,
            spool_binding,
            finished: false,
        })
    }
    /// One bounded context step followed by independent current native Write admission.
    /// Scheduling and ownership of synchronous work remain the supervisor's responsibility.
    pub async fn step(
        &mut self,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<CaptureResumeStep> {
        if self.finished {
            return Err(Error::Service(
                "capture resume preparation already completed",
            ));
        }
        self.check_spool(context.spool)?;
        if !self.eligible()? {
            return Ok(CaptureResumeStep::Withheld);
        }
        let directory = match self
            .probe
            .poll()
            .map_err(|_| Error::Service("capture resume source unavailable"))?
        {
            CodexResumePoll::Verifying => return Ok(CaptureResumeStep::Verifying),
            CodexResumePoll::ChangedDuringRead => return Ok(CaptureResumeStep::ChangedDuringRead),
            CodexResumePoll::Ready(directory) => directory,
        };
        let candidates = [
            directory.directory(),
            directory.prior_directory(),
            Some(self.qualified.directory()),
        ];
        let mut admitted = None;
        for path in candidates.into_iter().flatten() {
            let Some(path) = path.to_str() else {
                continue;
            };
            match context.admission.inspect(path).await {
                Ok(proof) => {
                    admitted = Some(proof);
                    break;
                }
                Err(
                    crate::capture_admission::Error::Unbound
                    | crate::capture_admission::Error::Limit,
                ) => {}
                Err(_) => return Ok(CaptureResumeStep::Withheld),
            }
        }
        let Some(proof) = admitted else {
            return Ok(CaptureResumeStep::Withheld);
        };
        context
            .authority
            .bind(context.host, context.admission, &proof)
            .await?;
        self.check_spool(context.spool)?;
        if !self.eligible()?
            || !self
                .probe
                .revalidate()
                .map_err(|_| Error::Service("capture resume source unavailable"))?
        {
            return Ok(CaptureResumeStep::Withheld);
        }
        let sources = self.sources.clone();
        let qualified = self.qualified.clone();
        let eligibility = self.eligibility.clone();
        let registered = context
            .admission
            .source_io(move || eligibility.acquire(&sources, qualified))
            .await
            .map_err(|_| Error::Service("capture resume acquisition unavailable"))?
            .map_err(|_| Error::Service("capture resume source unavailable"))?;
        let Some(registered) = registered else {
            return Ok(CaptureResumeStep::Withheld);
        };
        if !self.eligibility.matches(registered.source_instance_id())
            || self
                .sources
                .lookup_admitted(&self.qualified)
                .map_err(|_| Error::Service("capture resume eligibility unavailable"))?
                != Some(registered.source_instance_id())
            || !self.eligible()?
            || !self
                .probe
                .revalidate()
                .map_err(|_| Error::Service("capture resume source unavailable"))?
        {
            return Ok(CaptureResumeStep::Withheld);
        }
        context
            .admission
            .revalidate(&proof)
            .await
            .map_err(|_| Error::Service("capture resume admission changed"))?;
        self.check_spool(context.spool)?;
        let source = CodexCaptureSource::open(
            registered,
            self.profile.clone(),
            proof,
            self.privacy.clone(),
            context.spool,
        )?;
        self.check_spool(context.spool)?;
        if !self.eligible()?
            || self
                .sources
                .lookup_admitted(&self.qualified)
                .map_err(|_| Error::Service("capture resume eligibility unavailable"))?
                != Some(source.partition().source)
            || !self
                .probe
                .revalidate()
                .map_err(|_| Error::Service("capture resume source unavailable"))?
        {
            return Ok(CaptureResumeStep::Withheld);
        }
        self.finished = true;
        Ok(CaptureResumeStep::Ready(Box::new(source)))
    }
    fn eligible(&self) -> Result<bool> {
        self.eligibility.check(&self.sources, &self.qualified)
    }
    fn check_spool(&self, spool: &AgentCaptureSpool) -> Result<()> {
        if spool
            .binding()
            .map_err(|_| Error::Service("capture spool binding unavailable"))?
            != self.spool_binding
        {
            return Err(Error::Service("capture spool binding changed"));
        }
        Ok(())
    }
}
