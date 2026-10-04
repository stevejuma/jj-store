use super::*;

/// Resolve only the exact sanitized intent whose device proof the caller holds.
/// A Captured intent may have become Prepared concurrently, but cannot change bodies.
pub async fn drain_capture_spool_entry<P: AgentCapturePublisher>(
    spool: &mut AgentCaptureSpool,
    expected: &PendingCaptureV1,
    publisher: &P,
) -> AuthorityResult<CaptureDrainReport> {
    expected.validate()?;
    if matches!(expected.state, PendingCaptureStateV1::LocalPrepared { .. }) {
        return Err("local capture cannot publish through native RPC".into());
    }
    let _lease = spool.drain_lease()?;
    let queue = spool.pending()?;
    let Some(entry) = queue
        .iter()
        .find(|entry| entry.batch.batch_id == expected.batch.batch_id)
    else {
        return Ok(CaptureDrainReport {
            committed: 0,
            remaining: queue.len(),
            outcome_unknown: false,
        });
    };
    if !crate::agent_capture::same_capture(entry, expected)
        || matches!(entry.state, PendingCaptureStateV1::LocalPrepared { .. })
        || (!matches!(expected.state, PendingCaptureStateV1::Captured)
            && entry.state != expected.state)
    {
        return Err("capture recovery differs from durable intent".into());
    }
    let partition = CaptureDrainPartition::of(entry);
    if queue.iter().any(|other| {
        CaptureDrainPartition::of(other) == partition
            && (matches!(other.state, PendingCaptureStateV1::LocalPrepared { .. })
                || capture_order(other, entry).is_lt())
    }) {
        return Err("capture exact entry is not the native partition head".into());
    }
    drain::selected(spool, vec![entry.clone()], publisher).await
}
