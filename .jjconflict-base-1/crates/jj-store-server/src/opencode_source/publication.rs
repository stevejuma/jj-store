use super::*;
use jj_store_capture::{
    CaptureResult,
    agent_publication::{AgentCapturePublisher, drain_capture_spool_entry},
};
use jj_store_model::{PublicationId, RequestId};
use jj_store_service::transport::{
    AgentDeltaRequest, NormalizerProfileV1, PublishOutcome, SanitizedObservationBatchV1,
};
use sha2::{Digest as _, Sha256};

/// Body-free exact-entry proof, independently owned by the publication lane.
/// Cloning retains the existing exclusive leases, never a message page or scanner.
#[derive(Clone)]
pub(crate) struct OpenCodePublicationProof {
    api: OpenCodeReadApi,
    session: OpenCodeSessionSnapshot,
    lease: Arc<LeasedOpenCodeSource>,
    binding: CaptureSpoolBinding,
    capacity: u64,
    directory: VerifiedCaptureWorkspace,
    batch: CaptureBatchId,
    fingerprint: [u8; 32],
}
impl OpenCodePublicationProof {
    pub(super) fn new(
        source: &OpenCodeCaptureSource,
        directory: VerifiedCaptureWorkspace,
        capture: &ReaderCapture,
    ) -> Result<Self> {
        Ok(Self {
            api: source.api.clone(),
            session: source.session.clone(),
            lease: source.lease.clone(),
            binding: source.spool_binding.clone(),
            capacity: source.spool_capacity,
            directory,
            batch: capture.batch.batch_id,
            fingerprint: fingerprint(&(
                capture.repository,
                &capture.profile,
                &capture.cursor,
                &capture.batch,
                Some(&capture.metadata),
                &capture.thread_binding,
            ))?,
        })
    }
    pub(crate) fn batch_id(&self) -> CaptureBatchId {
        self.batch
    }
    fn check(&self, spool: &jj_store_capture::agent_capture::AgentCaptureSpool) -> Result<()> {
        self.lease
            .revalidate()
            .map_err(|_| Error::Service("OpenCode source lease changed"))?;
        if spool.pending_capacity() != self.capacity
            || spool
                .binding()
                .map_err(|_| Error::Service("OpenCode spool unavailable"))?
                != self.binding
        {
            return Err(Error::Service("OpenCode spool binding changed"));
        }
        Ok(())
    }
    pub(crate) async fn publish(
        &self,
        entry: &PendingCaptureV1,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<CaptureSourceStep> {
        self.check(context.spool)?;
        if entry.cursor.integration != IntegrationId::OpenCode
            || entry.cursor.source_instance_id != self.lease.source_instance_id()
            || entry.batch.batch_id != self.batch
            || fingerprint(&(
                entry.repository,
                &entry.profile,
                &entry.cursor,
                &entry.batch,
                &entry.reader_metadata,
                &entry.thread_binding,
            ))? != self.fingerprint
        {
            return Err(Error::Service("OpenCode exact publication proof differs"));
        }
        let spool = context.spool.fork_handle();
        let check = || self.check(&spool);
        let publisher = ApiPublisher {
            api: &self.api,
            session: &self.session,
            inner: context
                .authority
                .publisher(context.host, context.admission, &self.directory)
                .with_guard(&check),
        };
        let report = drain_capture_spool_entry(context.spool, entry, &publisher)
            .await
            .map_err(|error| {
                Error::capture_spool(error, "OpenCode exact publication unavailable")
            })?;
        Ok(CaptureSourceStep::Published(report))
    }
}
fn fingerprint(value: &impl serde::Serialize) -> Result<[u8; 32]> {
    let bytes = serde_json::to_vec(value)
        .map_err(|_| Error::Service("OpenCode proof fingerprint unavailable"))?;
    Ok(Sha256::digest(bytes).into())
}
struct ApiPublisher<'a> {
    api: &'a OpenCodeReadApi,
    session: &'a OpenCodeSessionSnapshot,
    inner: crate::capture_authority::GuardedCapturePublisher<'a>,
}
impl ApiPublisher<'_> {
    async fn current(&self) -> CaptureResult<()> {
        if self
            .api
            .observe(self.session.native_session_key())
            .await?
            .session()
            != self.session
        {
            return Err("OpenCode publication binding changed".into());
        }
        Ok(())
    }
}
#[async_trait::async_trait]
impl AgentCapturePublisher for ApiPublisher<'_> {
    async fn prepare_agent_capture(
        &self,
        request: RequestId,
        publication: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> CaptureResult<AgentDeltaRequest> {
        self.current().await?;
        self.inner
            .prepare_agent_capture(request, publication, profile, batch, result)
            .await
    }
    async fn publish_agent_delta(
        &self,
        request: &AgentDeltaRequest,
    ) -> CaptureResult<PublishOutcome> {
        self.current().await?;
        self.inner.publish_agent_delta(request).await
    }
}
impl OpenCodeCaptureSource {
    /// Earliest intent per partition, with fair selection across unavailable repositories.
    pub async fn publish_step(
        &mut self,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<CaptureSourceStep> {
        self.current(context).await?;
        let entries: Vec<_> = context
            .spool
            .pending()
            .map_err(|_| Error::Service("OpenCode publication queue unavailable"))?
            .into_iter()
            .filter(|e| {
                e.cursor.integration == IntegrationId::OpenCode
                    && e.cursor.source_instance_id == self.source_instance_id()
            })
            .collect();
        let Some(entry) = self
            .delivery
            .select(&entries)
            .map_err(|_| Error::Service("OpenCode publication selection unavailable"))?
        else {
            self.proofs.clear();
            return Ok(CaptureSourceStep::Waiting);
        };
        let Some(proof) = self.proofs.get(&entry.batch.batch_id) else {
            return Ok(CaptureSourceStep::Withheld);
        };
        let result = proof.publish(entry, context).await?;
        if matches!(&result,CaptureSourceStep::Published(report) if report.committed > 0) {
            self.proofs.remove(&entry.batch.batch_id);
        }
        Ok(result)
    }
}
