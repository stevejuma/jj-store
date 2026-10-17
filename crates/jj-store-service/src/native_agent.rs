//! Server-owned AgentStore capture preparation and receipt publication.
use crate::{auth::Permission, native::NativeSession};
use jj_store_model::{PublicationId, RequestId};
use jj_store_wal::{
    self as wal,
    publication::{
        AgentDeltaRequest, NormalizerProfileV1, PublishOutcome, SanitizedObservationBatchV1,
    },
};

impl NativeSession {
    pub async fn prepare_agent_capture(
        &self,
        request_id: RequestId,
        publication_id: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> wal::Result<AgentDeltaRequest> {
        self.prepare_agent_capture_with_binding(
            request_id,
            publication_id,
            profile,
            batch,
            result,
            None,
        )
        .await
    }

    pub async fn prepare_agent_capture_with_binding(
        &self,
        request_id: RequestId,
        publication_id: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
        binding: Option<&jj_collab::ThreadBindingV1>,
    ) -> wal::Result<AgentDeltaRequest> {
        if let Some(binding) = binding {
            binding
                .validate()
                .map_err(|_| wal::Error::Corrupt("capture Thread binding"))?;
        }
        profile
            .validate()
            .map_err(|_| wal::Error::Corrupt("agent normalizer profile"))?;
        batch
            .validate()
            .map_err(|_| wal::Error::Corrupt("sanitized agent capture"))?;
        let principal = self.principal(Permission::Write).await?;
        let activated_at_millis =
            i64::try_from(self.clock.unix_seconds().map_err(|_| wal::Error::Denied)?)
                .ok()
                .and_then(|value| value.checked_mul(1_000))
                .ok_or(wal::Error::Limit("agent capture activation time"))?;
        let stores = self
            .collaboration_stores_at(Some(activated_at_millis))
            .await?;
        let adapted = jj_store_core::StoreRequest::new(
            request_id,
            jj_store_core::AgentCommand::Capture {
                profile: profile.clone(),
                batch: Box::new(batch.clone()),
                binding: binding.cloned(),
            },
        )
        .with_publication_id(publication_id);
        let prepared = stores.prepare_agent(&adapted).await?;
        let mut native = jj_store_native::to_agent_delta(principal.id(), &prepared)?;
        native.result = result;
        native.validate()?;
        Ok(native)
    }

    pub async fn publish_agent_delta(
        &self,
        request: &AgentDeltaRequest,
    ) -> wal::Result<PublishOutcome> {
        let principal = self.principal(Permission::Write).await?;
        if principal.id() != request.principal {
            return Err(wal::Error::Denied);
        }
        let stores = self.collaboration_stores().await?;
        let adapted = jj_store_native::from_agent_delta(request)?;
        let value = stores.execute_agent(&adapted).await?;
        let outcome = crate::native::publication_outcome(value, request.publication_id)?;
        if matches!(outcome, PublishOutcome::Committed(_)) {
            self.events.published(self.repository);
        }
        Ok(outcome)
    }
}
