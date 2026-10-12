//! State owners send settlement hints; observers never poll storage overviews.
use super::*;
use tokio::sync::broadcast;
pub(super) fn start(state: &Arc<State>) -> tokio::task::JoinHandle<()> {
    let mut receivers = vec![
        state.tasks.freshness(),
        state.code_indexes.freshness(),
        state.kind_tasks.freshness(),
        state.local_tasks.freshness(),
        state.search.freshness(),
    ];
    macro_rules! add {
        ($manager:ident) => {
            if let Some(receiver) = state.config.$manager.as_ref().and_then(|m| m.freshness()) {
                receivers.push(receiver);
            }
        };
    }
    add!(local_storage);
    add!(local_sync);
    add!(local_search);
    add!(local_capture);
    add!(local_settings);
    add!(local_workspaces);
    add!(local_repositories);
    add!(local_agents);
    add!(remote_agents);
    add!(directory);
    let weak = Arc::downgrade(state);
    tokio::spawn(async move {
        loop {
            let futures = receivers
                .iter_mut()
                .enumerate()
                .map(|(index, receiver)| Box::pin(async move { (index, receiver.recv().await) }))
                .collect::<Vec<_>>();
            if futures.is_empty() {
                return;
            }
            let ((index, result), _, _) = futures::future::select_all(futures).await;
            let Some(state) = weak.upgrade() else {
                return;
            };
            match result {
                Ok(change) => state.freshness.changed(change),
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    // Missing settlement notices require a complete consumer reset, not invented continuity.
                    state.freshness.changed(Change {
                        repository: None,
                        domain: Domain::Repository,
                        witness: format!("lost:{}", state.freshness.snapshot().0),
                        identity: None,
                        audience: None,
                    });
                }
                Err(broadcast::error::RecvError::Closed) => {
                    receivers.remove(index);
                }
            }
        }
    })
}
