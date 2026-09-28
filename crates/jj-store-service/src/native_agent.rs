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
        let selected = self.snapshot().await?;
        let root = selected
            .manifest()
            .agent_root
            .ok_or(wal::Error::Corrupt("agent domain root"))?;
        let pinned = self.coordinator.agent_store(root).await?;
        let delta = self
            .coordinator
            .stage_agent_capture(&pinned, profile, batch, activated_at_millis)
            .await?;
        let current = self.principal(Permission::Write).await?;
        if current.id() != principal.id() {
            return Err(wal::Error::Denied);
        }
        AgentDeltaRequest::new(principal.id(), request_id, publication_id, delta, result)
    }

    pub async fn publish_agent_delta(
        &self,
        request: &AgentDeltaRequest,
    ) -> wal::Result<PublishOutcome> {
        let principal = self.principal(Permission::Write).await?;
        if principal.id() != request.principal {
            return Err(wal::Error::Denied);
        }
        let outcome = self.coordinator.publish_agent_delta(request).await?;
        if matches!(outcome, PublishOutcome::Committed(_)) {
            self.events.published(self.repository);
        }
        Ok(outcome)
    }
}
