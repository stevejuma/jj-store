//! Worker state already has a watch channel. Suppress alternating idle scheduler lanes.
use super::*;
use jj_store_service::freshness::Domain;
pub(super) fn phase(entry: &Entry, phase: control::Phase) {
    let mut state = entry.state.lock().expect("replication state");
    if state.phase == phase {
        return;
    }
    state.phase = phase;
    drop(state);
    entry
        .changes
        .notify(Some(entry.repository), Domain::Sync, None);
}
pub(super) async fn run(
    entry: &Entry,
    worker: ReplicationWorker<LocalEndpoint, RemoteEndpoint>,
    shutdown: watch::Receiver<bool>,
) {
    let mut updates = worker.handle().subscribe();
    let mut previous = stable(updates.borrow().clone());
    let running = worker.run(shutdown);
    tokio::pin!(running);
    loop {
        tokio::select! {
            _ = &mut running => return,
            result = updates.changed() => {
                if result.is_err() { let _ = running.await; return; }
                let observed = stable(updates.borrow().clone());
                if previous != observed { previous = observed; entry.changes.notify(Some(entry.repository), Domain::Sync, None); }
            }
        }
    }
}
fn stable(mut status: jj_store_replication::WorkerStatus) -> jj_store_replication::WorkerStatus {
    status.active = None;
    status.next_direction = jj_store_replication::Direction::Pull;
    status
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn idle_replication_scheduler_does_not_change_the_semantic_witness() {
        let first = jj_store_replication::WorkerStatus::default();
        let mut active = first.clone();
        active.active = Some(jj_store_replication::Direction::Push);
        active.next_direction = jj_store_replication::Direction::Push;
        assert_eq!(stable(first.clone()), stable(active.clone()));
        active.checkpoint_revision = 1;
        assert_ne!(stable(first), stable(active));
    }
}
