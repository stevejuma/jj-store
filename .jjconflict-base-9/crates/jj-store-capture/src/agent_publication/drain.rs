//! Shared exact-intent publication and completion under the caller's drain lease.
use super::*;

pub(super) async fn selected<P: AgentCapturePublisher>(
    spool: &mut AgentCaptureSpool,
    pending: Vec<PendingCaptureV1>,
    publisher: &P,
) -> AuthorityResult<CaptureDrainReport> {
    let mut committed = 0;
    for entry in pending {
        let request = match entry.state {
            PendingCaptureStateV1::Captured => {
                let request_id = request_id(entry.repository, entry.batch.batch_id);
                let publication_id = publication_id(entry.repository, entry.batch.batch_id);
                let result = capture_result(entry.batch.batch_id);
                let request = if let Some(binding) = &entry.thread_binding {
                    publisher
                        .prepare_bound_agent_capture(
                            request_id,
                            publication_id,
                            &entry.profile,
                            &entry.batch,
                            result.clone(),
                            binding,
                        )
                        .await?
                } else {
                    publisher
                        .prepare_agent_capture(
                            request_id,
                            publication_id,
                            &entry.profile,
                            &entry.batch,
                            result.clone(),
                        )
                        .await?
                };
                validate_prepared(
                    &request,
                    request_id,
                    publication_id,
                    &result,
                    entry.thread_binding.as_ref(),
                )?;
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
                spool.complete_committed(entry.batch.batch_id, &receipt)?;
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
