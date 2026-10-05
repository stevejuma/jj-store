//! Current device admission is distinct from native publication and durable intent.
use super::CaptureAuthority;
use crate::capture_admission::CaptureAdmission;
use jj_store_capture::{CaptureResult, agent_publication::AgentCapturePublisher};
use jj_store_model::{PublicationId, RequestId};
use jj_store_service::{
    administration::RepositoryHost,
    collab_model::ThreadBindingV1,
    local_management::VerifiedCaptureWorkspace,
    transport::{
        AgentDeltaRequest, NormalizerProfileV1, PublishOutcome, SanitizedObservationBatchV1,
    },
};

/// Device-only proof; intentionally has no Debug or persistence encoding.
pub struct GuardedCapturePublisher<'a> {
    authority: &'a CaptureAuthority,
    host: &'a RepositoryHost,
    admission: &'a CaptureAdmission,
    proof: &'a VerifiedCaptureWorkspace,
    check: Option<&'a (dyn Fn() -> crate::Result<()> + Sync)>,
}
impl<'a> GuardedCapturePublisher<'a> {
    pub(super) fn new(
        authority: &'a CaptureAuthority,
        host: &'a RepositoryHost,
        admission: &'a CaptureAdmission,
        proof: &'a VerifiedCaptureWorkspace,
    ) -> Self {
        Self {
            authority,
            host,
            admission,
            proof,
            check: None,
        }
    }
    pub(crate) fn with_guard(mut self, check: &'a (dyn Fn() -> crate::Result<()> + Sync)) -> Self {
        self.check = Some(check);
        self
    }
    fn current(&self) -> CaptureResult<()> {
        if let Some(check) = self.check {
            check()?;
        }
        Ok(())
    }
}
#[async_trait::async_trait]
impl AgentCapturePublisher for GuardedCapturePublisher<'_> {
    async fn prepare_agent_capture(
        &self,
        request: RequestId,
        publication: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> CaptureResult<AgentDeltaRequest> {
        self.current()?;
        let native = self
            .authority
            .bind(self.host, self.admission, self.proof)
            .await?;
        self.current()?;
        let prepared = AgentCapturePublisher::prepare_agent_capture(
            &native,
            request,
            publication,
            profile,
            batch,
            result,
        )
        .await?;
        self.admission.revalidate(self.proof).await?;
        self.current()?;
        Ok(prepared)
    }
    async fn prepare_bound_agent_capture(
        &self,
        request: RequestId,
        publication: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
        binding: &ThreadBindingV1,
    ) -> CaptureResult<AgentDeltaRequest> {
        self.current()?;
        let native = self
            .authority
            .bind(self.host, self.admission, self.proof)
            .await?;
        self.current()?;
        let prepared = AgentCapturePublisher::prepare_bound_agent_capture(
            &native,
            request,
            publication,
            profile,
            batch,
            result,
            binding,
        )
        .await?;
        self.admission.revalidate(self.proof).await?;
        self.current()?;
        Ok(prepared)
    }
    async fn publish_agent_delta(
        &self,
        request: &AgentDeltaRequest,
    ) -> CaptureResult<PublishOutcome> {
        self.current()?;
        if request.principal != self.authority.principal() {
            return Err("capture publication actor mismatch".into());
        }
        let native = self
            .authority
            .bind(self.host, self.admission, self.proof)
            .await?;
        self.current()?;
        // Never convert a committed receipt into a device-admission error afterward.
        AgentCapturePublisher::publish_agent_delta(&native, request).await
    }
}
