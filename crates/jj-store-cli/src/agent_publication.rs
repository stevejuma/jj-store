//! Ordered recovery of durable sanitized captures through native AgentStore RPC.
use crate::agent_capture::{AgentCaptureSpool, PendingCaptureStateV1, PendingCaptureV1};
use jj_agent::{CaptureBatchId, NormalizerProfileV1, SanitizedObservationBatchV1};
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_model::{PublicationId, RepositoryId, RequestId};
use jj_store_rpc::client::NativeClient;
use jj_store_wal::publication::{AgentDeltaRequest, PublishOutcome, Receipt};
use sha2::{Digest as _, Sha256};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CaptureDrainReport {
    pub committed: usize,
    pub remaining: usize,
    pub outcome_unknown: bool,
}

#[async_trait::async_trait]
pub trait AgentCapturePublisher: Send + Sync {
    async fn prepare_agent_capture(
        &self,
        request_id: RequestId,
        publication_id: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> AuthorityResult<AgentDeltaRequest>;

    async fn publish_agent_delta(
        &self,
        request: &AgentDeltaRequest,
    ) -> AuthorityResult<PublishOutcome>;
}

#[async_trait::async_trait]
impl AgentCapturePublisher for NativeClient {
    async fn prepare_agent_capture(
        &self,
        request_id: RequestId,
        publication_id: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> AuthorityResult<AgentDeltaRequest> {
        Ok(self
            .prepare_agent_capture(request_id, publication_id, profile, batch, result)
            .await?)
    }

    async fn publish_agent_delta(
        &self,
        request: &AgentDeltaRequest,
    ) -> AuthorityResult<PublishOutcome> {
        Ok(self.publish_agent_delta(request).await?)
    }
}

pub async fn drain_capture_spool<P: AgentCapturePublisher>(
    spool: &mut AgentCaptureSpool,
    repository: RepositoryId,
    publisher: &P,
) -> AuthorityResult<CaptureDrainReport> {
    let mut pending = spool.pending()?;
    if pending.iter().any(|entry| {
        entry.repository != repository
            || matches!(entry.state, PendingCaptureStateV1::LocalPrepared { .. })
    }) {
        return Err("capture spool belongs to another repository".into());
    }
    pending.sort_by(capture_order);
    let mut committed = 0;
    for entry in pending {
        let request = match entry.state {
            PendingCaptureStateV1::Captured => {
                let request_id = request_id(repository, entry.batch.batch_id);
                let publication_id = publication_id(repository, entry.batch.batch_id);
                let result = capture_result(entry.batch.batch_id);
                let request = publisher
                    .prepare_agent_capture(
                        request_id,
                        publication_id,
                        &entry.profile,
                        &entry.batch,
                        result.clone(),
                    )
                    .await?;
                validate_prepared(&request, request_id, publication_id, &result)?;
                spool.prepare(entry.batch.batch_id, request.clone())?;
                request
            }
            PendingCaptureStateV1::Prepared { request } => request,
            PendingCaptureStateV1::LocalPrepared { .. } => {
                return Err("local capture cannot publish through native RPC".into());
            }
        };
        match publisher.publish_agent_delta(&request).await? {
            PublishOutcome::Committed(receipt) => {
                validate_receipt(&request, &receipt)?;
                spool.complete(
                    entry.batch.batch_id,
                    request.request_id,
                    request.publication_id,
                )?;
                committed += 1;
            }
            PublishOutcome::OutcomeUnknown {
                request_id,
                publication_id,
            } if request_id == request.request_id && publication_id == request.publication_id => {
                return Ok(CaptureDrainReport {
                    committed,
                    remaining: spool.pending()?.len(),
                    outcome_unknown: true,
                });
            }
            PublishOutcome::OutcomeUnknown { .. } => {
                return Err("capture publication identity mismatch".into());
            }
        }
    }
    Ok(CaptureDrainReport {
        committed,
        remaining: spool.pending()?.len(),
        outcome_unknown: false,
    })
}

pub(crate) fn capture_order(
    left: &PendingCaptureV1,
    right: &PendingCaptureV1,
) -> std::cmp::Ordering {
    left.batch
        .integration
        .cmp(&right.batch.integration)
        .then_with(|| {
            left.batch
                .source_instance_id
                .cmp(&right.batch.source_instance_id)
        })
        .then_with(|| left.cursor.generation.cmp(&right.cursor.generation))
        .then_with(|| first_sequence(left).cmp(&first_sequence(right)))
        .then_with(|| left.batch.batch_id.cmp(&right.batch.batch_id))
}

fn first_sequence(entry: &PendingCaptureV1) -> u64 {
    entry
        .batch
        .observations
        .first()
        .expect("validated capture batch")
        .source_sequence
}

pub(crate) fn request_id(repository: RepositoryId, batch: CaptureBatchId) -> RequestId {
    RequestId::new(derived_identity(
        b"jj-store/agent-capture-request/v1\0",
        repository,
        batch,
    ))
}

fn publication_id(repository: RepositoryId, batch: CaptureBatchId) -> PublicationId {
    PublicationId::new(derived_identity(
        b"jj-store/agent-capture-publication/v1\0",
        repository,
        batch,
    ))
}

fn derived_identity(domain: &[u8], repository: RepositoryId, batch: CaptureBatchId) -> [u8; 16] {
    let mut hash = Sha256::new_with_prefix(domain);
    hash.update(repository.as_bytes());
    hash.update(batch.as_bytes());
    hash.finalize()[..16]
        .try_into()
        .expect("fixed SHA-256 prefix")
}

fn capture_result(batch: CaptureBatchId) -> Vec<u8> {
    let mut result = b"jj-store-agent-capture-v1\0".to_vec();
    result.extend_from_slice(batch.as_bytes());
    result
}

fn validate_prepared(
    request: &AgentDeltaRequest,
    request_id: RequestId,
    publication_id: PublicationId,
    result: &[u8],
) -> AuthorityResult<()> {
    AgentDeltaRequest::new(
        request.principal,
        request.request_id,
        request.publication_id,
        request.delta.clone(),
        request.result.clone(),
    )?;
    if request.request_id != request_id
        || request.publication_id != publication_id
        || request.result != result
    {
        return Err("capture preparation identity mismatch".into());
    }
    Ok(())
}

fn validate_receipt(request: &AgentDeltaRequest, receipt: &Receipt) -> AuthorityResult<()> {
    if receipt.principal != request.principal
        || receipt.request_id != request.request_id
        || receipt.publication_id != request.publication_id
        || receipt.result != request.result
    {
        return Err("capture committed receipt mismatch".into());
    }
    Ok(())
}
