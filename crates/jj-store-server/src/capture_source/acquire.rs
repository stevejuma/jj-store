use super::*;
use jj_store_capture::{
    codex_activity::ActiveCodexSource,
    profile::CaptureProfile,
    source_registry::{QualifiedCodexSource, RegisteredCodexSource, SourceRegistry},
};
use std::sync::Arc;
impl CodexCaptureSource {
    pub async fn acquire_active(
        sources: Arc<SourceRegistry>,
        active: ActiveCodexSource,
        privacy: CapturePrivacyRoots,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<Self> {
        let profile = normalizer(active.profile())?;
        let proof = context
            .admission
            .inspect_activity(&active)
            .await
            .map_err(|_| Error::Service("capture active admission unavailable"))?;
        // Native authority precedes source allocation; catalog membership alone cannot enroll.
        context
            .authority
            .bind(context.host, context.admission, &proof)
            .await?;
        let (registered, current) = context
            .admission
            .acquire_active_source(sources, active)
            .await
            .map_err(|_| Error::Service("capture active acquisition unavailable"))?;
        if current != proof {
            return Err(Error::Service("capture active admission changed"));
        }
        Self::open(registered, profile, proof, privacy, context.spool)
    }
    pub async fn acquire_admitted(
        sources: Arc<SourceRegistry>,
        qualified: QualifiedCodexSource,
        privacy: CapturePrivacyRoots,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<Option<Self>> {
        let profile = normalizer(qualified.profile())?;
        if !sources
            .previously_admitted(&qualified)
            .map_err(|_| Error::Service("capture admission history unavailable"))?
        {
            return Ok(None);
        }
        let directory = qualified
            .directory()
            .to_str()
            .ok_or(Error::Service("capture directory unavailable"))?;
        let proof = context
            .admission
            .inspect(directory)
            .await
            .map_err(|_| Error::Service("capture admitted directory unavailable"))?;
        context
            .authority
            .bind(context.host, context.admission, &proof)
            .await?;
        let registered = context
            .admission
            .source_io(move || sources.acquire_admitted(qualified))
            .await
            .map_err(|_| Error::Service("capture admitted acquisition unavailable"))?
            .map_err(|_| Error::Service("capture admitted source unavailable"))?;
        context
            .admission
            .revalidate(&proof)
            .await
            .map_err(|_| Error::Service("capture admitted directory changed"))?;
        registered
            .map(|registered| Self::open(registered, profile, proof, privacy, context.spool))
            .transpose()
    }
    pub(super) fn open(
        registered: RegisteredCodexSource,
        profile: NormalizerProfileV1,
        anchor: VerifiedCaptureWorkspace,
        privacy: CapturePrivacyRoots,
        spool: &mut AgentCaptureSpool,
    ) -> Result<Self> {
        let source = registered.source_instance_id();
        let spool_binding = spool
            .binding()
            .map_err(|_| Error::Service("capture spool binding unavailable"))?;
        let bounds = spool
            .source_reader_progress(
                profile.integration,
                source,
                registered.qualified().generation_digest(),
            )
            .map_err(|error| Error::capture_spool(error, "capture source progress unavailable"))?;
        let reader = registered
            .open_reader_at(&bounds)
            .map_err(|_| Error::Service("capture reader unavailable"))?
            .ok_or(Error::Service("capture reader changed"))?;
        if spool
            .binding()
            .map_err(|_| Error::Service("capture spool binding unavailable"))?
            != spool_binding
        {
            return Err(Error::Service("capture spool binding changed"));
        }
        Ok(Self {
            spool_binding,
            reader,
            reader_progress: bounds.progress,
            profile,
            anchor,
            pending: None,
            recovery: None,
            needs_resume: false,
            privacy,
            sanitizer: None,
        })
    }
}
pub(super) fn normalizer(profile: CaptureProfile) -> Result<NormalizerProfileV1> {
    match profile {
        CaptureProfile::CodexCliJsonlV1 => Ok(NormalizerProfileV1::codex_0_156_1()),
        CaptureProfile::CodexDesktopJsonlV1 => Ok(NormalizerProfileV1::codex_desktop_0_159_2()),
        _ => Err(Error::Config("unsupported admitted Codex profile")),
    }
}
