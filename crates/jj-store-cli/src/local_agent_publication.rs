//! Ordered recovery of the exact local capture request, without native RPC.
use crate::{
    agent_capture::{AgentCaptureSpool, PendingCaptureStateV1},
    agent_publication::{CaptureDrainReport, capture_order, request_id},
};
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_model::{PrincipalId, RepositoryId};
use jj_store_service::local_repository::{
    LocalAgentCommand, LocalAgentOutcome, LocalAgentRequest, LocalRepository,
};

#[async_trait::async_trait]
pub trait LocalAgentCapturePublisher: Send + Sync {
    fn binding(&self) -> AuthorityResult<(RepositoryId, PrincipalId)>;
    async fn apply(&self, request: &LocalAgentRequest) -> AuthorityResult<LocalAgentOutcome>;
}
#[async_trait::async_trait]
impl LocalAgentCapturePublisher for LocalRepository {
    fn binding(&self) -> AuthorityResult<(RepositoryId, PrincipalId)> {
        Ok((
            self.repository()?
                .ok_or("local metadata is uninitialized")?,
            self.ensure_actor()?,
        ))
    }
    async fn apply(&self, request: &LocalAgentRequest) -> AuthorityResult<LocalAgentOutcome> {
        Ok(self.apply_agent_command(request).await?)
    }
}
pub async fn drain_local_capture_spool<P: LocalAgentCapturePublisher>(
    spool: &mut AgentCaptureSpool,
    publisher: &P,
) -> AuthorityResult<CaptureDrainReport> {
    let (repository, actor) = publisher.binding()?;
    let mut pending = spool.pending()?;
    if pending.iter().any(|entry| {
        entry.repository != repository
            || matches!(entry.state, PendingCaptureStateV1::Prepared { .. })
    }) {
        return Err("capture spool profile or repository mismatch".into());
    }
    pending.sort_by(capture_order);
    let mut committed = 0;
    for entry in pending {
        let request = match entry.state {
            PendingCaptureStateV1::Captured => {
                let request = LocalAgentRequest {
                    version: 1,
                    request_id: request_id(repository, entry.batch.batch_id),
                    command: LocalAgentCommand::Capture {
                        profile: entry.profile.clone(),
                        batch: Box::new(entry.batch.clone()),
                    },
                };
                spool.prepare_local(entry.batch.batch_id, request.clone())?;
                request
            }
            PendingCaptureStateV1::LocalPrepared { request } => request,
            PendingCaptureStateV1::Prepared { .. } => {
                return Err("native prepared capture in local spool".into());
            }
        };
        if request.request_id != request_id(repository, entry.batch.batch_id) {
            return Err("local capture request identity mismatch".into());
        }
        match publisher.apply(&request).await? {
            LocalAgentOutcome::Committed { receipt } => {
                if receipt.repository != repository
                    || receipt.actor != actor
                    || receipt.request_id != request.request_id
                    || receipt.intent != request.intent_for(actor)?
                    || receipt.batch != Some(entry.batch.batch_id)
                {
                    return Err("local capture receipt mismatch".into());
                }
                spool.complete_local(entry.batch.batch_id, request.request_id)?;
                committed += 1;
            }
            LocalAgentOutcome::OutcomeUnknown { request_id }
                if request_id == request.request_id =>
            {
                return Ok(CaptureDrainReport {
                    committed,
                    remaining: spool.pending()?.len(),
                    outcome_unknown: true,
                });
            }
            _ => return Err("local capture outcome identity mismatch".into()),
        }
    }
    Ok(CaptureDrainReport {
        committed,
        remaining: spool.pending()?.len(),
        outcome_unknown: false,
    })
}
