//! In-process service adapter; the native service retains publication authority.
use crate::{CaptureResult, agent_publication::AgentCapturePublisher};
use jj_agent::{NormalizerProfileV1, SanitizedObservationBatchV1};
use jj_store_model::{PublicationId, RequestId};
use jj_store_service::transport::NativeRequest;
use jj_store_wal::publication::{AgentDeltaRequest, PublishOutcome};

#[async_trait::async_trait]
impl AgentCapturePublisher for NativeRequest {
    async fn prepare_agent_capture(
        &self,
        request: RequestId,
        publication: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> CaptureResult<AgentDeltaRequest> {
        Ok(
            NativeRequest::prepare_agent_capture(
                self,
                request,
                publication,
                profile,
                batch,
                result,
            )
            .await?,
        )
    }
    async fn prepare_bound_agent_capture(
        &self,
        request: RequestId,
        publication: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
        binding: &jj_collab::ThreadBindingV1,
    ) -> CaptureResult<AgentDeltaRequest> {
        Ok(NativeRequest::prepare_agent_capture_with_binding(
            self,
            request,
            publication,
            profile,
            batch,
            result,
            Some(binding),
        )
        .await?)
    }
    async fn publish_agent_delta(
        &self,
        request: &AgentDeltaRequest,
    ) -> CaptureResult<PublishOutcome> {
        Ok(NativeRequest::publish_agent_delta(self, request).await?)
    }
}
