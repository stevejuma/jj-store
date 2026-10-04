use super::*;
use tokio::sync::OwnedSemaphorePermit;

#[allow(clippy::too_many_arguments)]
pub(super) fn spawn(
    manager: &KindTasks,
    session: Arc<NativeSession>,
    mut preparation: prep::Preparation,
    slot: OwnedSemaphorePermit,
    task: Task,
    owner: m::PrincipalId,
    expires: Instant,
    cancellation: Cancellation,
) {
    let registry = manager.registry.clone();
    let changes = manager.changes.clone();
    let ttl = manager.limits.ttl;
    let window_timeout = manager.limits.read.timeout;
    let id = task.id;
    let repository = task.snapshot.authority.repo_id;
    tokio::spawn(async move {
        let signal = cancellation.clone();
        let progress_registry = registry.clone();
        let progress_id = id.clone();
        // Supervision converts panics to terminal failure and keeps cancellation
        // lease ownership with the actual job, never just its status record.
        let mut worker = tokio::spawn(async move {
            let _slot = slot;
            loop {
                let deadline = expires.min(Instant::now() + window_timeout);
                let outcome = tokio::select! {
                    biased;
                    _ = signal.cancelled() => Err(q::Error::Canceled),
                    result = session.advance_kind_preparation(&mut preparation, deadline) => result,
                };
                let progress = preparation.progress();
                if let Some(entry) = progress_registry
                    .lock()
                    .expect("kind task registry")
                    .0
                    .get_mut(&progress_id)
                {
                    entry.task.progress = progress;
                }
                match outcome {
                    Ok(progress) if progress.complete => return Ok(()),
                    Ok(_) => tokio::task::yield_now().await,
                    Err(error) => return Err(error),
                }
            }
        });
        let outcome = tokio::select! {
            result = &mut worker => result,
            _ = tokio::time::sleep_until(expires) => { cancellation.cancel(); worker.await }
        }
        .unwrap_or(Err(q::Error::Unavailable));
        let completed_expires = Instant::now() + ttl;
        let retired = Cancellation::default();
        let retained = {
            let mut registry = registry.lock().expect("kind task registry");
            if let Some(entry) = registry.0.get_mut(&id) {
                if Instant::now() >= entry.expires {
                    registry.0.remove(&id);
                } else {
                    entry.task.state = match outcome {
                        _ if matches!(entry.task.state, State::Canceling) => State::Canceled,
                        Ok(()) => State::Ready,
                        Err(q::Error::Canceled) => State::Canceled,
                        Err(error) => State::Failed { error },
                    };
                    entry.expires = completed_expires;
                    entry.cancellation = retired.clone();
                }
            }
            registry.prune();
            registry.0.contains_key(&id)
        };
        // Progress is inspected from bounded metadata; do not emit one event for
        // each descriptor window in a large root. Only settlement/expiry hints.
        changes.notify_to(
            Some(repository),
            crate::freshness::Domain::IndexTasks,
            Some(id.clone()),
            Some(owner),
        );
        if !retained {
            return;
        }
        tokio::select! { _ = retired.cancelled() => return, _ = tokio::time::sleep_until(completed_expires) => {} }
        registry.lock().expect("kind task registry").prune();
        changes.notify_to(
            Some(repository),
            crate::freshness::Domain::IndexTasks,
            Some(id),
            Some(owner),
        );
    });
}
