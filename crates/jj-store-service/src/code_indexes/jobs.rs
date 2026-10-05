//! A single coalesced queue; no repository watcher or head poller lives here.
use super::*;
use std::{sync::Weak, time::Duration};

pub(super) fn spawn(weak: Weak<Indexes>) {
    tokio::spawn(async move {
        loop {
            let Some(manager) = weak.upgrade() else {
                return;
            };
            let next = {
                let mut registry = manager.registry.lock().unwrap();
                match registry.pending.pop_front() {
                    Some(job) => {
                        let key = job.source.key();
                        registry
                            .entries
                            .get(&key)
                            .map(|e| (job, key, e.cancel.clone(), e.expires))
                    }
                    None => {
                        registry.active = false;
                        return;
                    }
                }
            };
            let Some((job, key, cancel, expires)) = next else {
                continue;
            };
            let source = job.source;
            let previous = job.previous;
            let graph = manager.graph.clone();
            let local = manager.local.clone();
            let root = manager.root.clone();
            let policy = manager.policy;
            drop(manager); // Server teardown must cancel work, not retain the owner.
            let mut delay = Duration::from_millis(100);
            let result = loop {
                if cancel.is_cancelled() {
                    break Err(q::Error::Canceled);
                }
                if Instant::now() >= expires {
                    break Err(q::Error::Limit);
                }
                let authorization = tokio::time::timeout_at(
                    expires.min(Instant::now() + Duration::from_secs(10)),
                    async {
                        match &source {
                            Source::Native(s) => s.access.authorize().await,
                            Source::Local { access, .. } => access.authorize().await,
                        }
                    },
                );
                let authorization = tokio::select! {
                    result = authorization => result.unwrap_or(Err(q::Error::Limit)),
                    _ = cancel.cancelled() => Err(q::Error::Canceled),
                };
                if let Err(error) = authorization {
                    break Err(error);
                }
                if cancel.is_cancelled() {
                    break Err(q::Error::Canceled);
                }
                let ns_root = root.clone();
                let binding = key.binding.clone();
                let namespace = tokio::task::spawn_blocking(move || {
                    Namespace::acquire(&ns_root, binding, policy)
                })
                .await;
                let result = match namespace {
                    Ok(Ok(namespace)) => {
                        if cancel.is_cancelled() {
                            drop(namespace);
                            break Err(q::Error::Canceled);
                        }
                        let remaining = expires.saturating_duration_since(Instant::now());
                        if remaining.is_zero() {
                            break Err(q::Error::Limit);
                        }
                        let timeout = remaining.min(if previous.is_some() {
                            Duration::from_secs(30)
                        } else {
                            remaining
                        });
                        match (&source, &previous) {
                            (Source::Native(s), Some(previous)) => graph
                                .clone()
                                .with_preparation_limits(q::preparation::Limits {
                                    timeout,
                                    max_read_bytes: 64 * 1024 * 1024,
                                    max_work: 1_000_000,
                                    max_metadata_nodes: 4096,
                                    ..Default::default()
                                })
                                .expect("bounded path preparation")
                                .prepare_managed_path_index(
                                    s.store.clone(),
                                    s.access.clone(),
                                    namespace,
                                    cancel.clone(),
                                    previous
                                        .native()
                                        .ok_or(q::Error::Corrupt)
                                        .expect("native source"),
                                )
                                .await
                                .map(|p| Ready::Native(Arc::new(p))),
                            (Source::Local { access, .. }, Some(previous)) => local
                                .prepare_managed_path_index(
                                    access.clone(),
                                    namespace,
                                    local::preparation::Limits {
                                        timeout,
                                        max_read_bytes: 64 * 1024 * 1024,
                                        max_work: 1_000_000,
                                        max_metadata_records: 4096,
                                        ..Default::default()
                                    },
                                    cancel.clone(),
                                    previous
                                        .local()
                                        .ok_or(q::Error::Corrupt)
                                        .expect("local source"),
                                )
                                .await
                                .map(|p| Ready::Local(Arc::new(p))),
                            (Source::Native(s), None) => graph
                                .clone()
                                .with_preparation_limits(q::preparation::Limits {
                                    timeout,
                                    ..Default::default()
                                })
                                .expect("bounded remaining preparation time")
                                .prepare_managed_code_index(
                                    s.store.clone(),
                                    s.access.clone(),
                                    namespace,
                                    cancel.clone(),
                                )
                                .await
                                .map(|p| Ready::Native(Arc::new(p))),
                            (Source::Local { access, snapshot }, None) => local
                                .prepare_managed_code_index(
                                    access.clone(),
                                    snapshot.operation,
                                    namespace,
                                    local::preparation::Limits {
                                        timeout,
                                        ..Default::default()
                                    },
                                    cancel.clone(),
                                )
                                .await
                                .map(|p| Ready::Local(Arc::new(p))),
                        }
                    }
                    Ok(Err(e)) => Err(e),
                    Err(_) => Err(q::Error::Unavailable),
                };
                if !matches!(result, Err(q::Error::Busy)) {
                    break result;
                }
                tokio::select! { _ = cancel.cancelled() => {}, _ = tokio::time::sleep(delay) => {} }
                delay = (delay * 2).min(Duration::from_secs(5));
            };
            let Some(manager) = weak.upgrade() else {
                return;
            };
            let task_id = {
                let mut registry = manager.registry.lock().unwrap();
                let Some(entry) = registry.entries.get_mut(&key) else {
                    continue;
                };
                entry.task.state = match result {
                    Ok(ready) if !cancel.is_cancelled() => {
                        let stats = ready.stats();
                        entry.ready = Some(q::code_index::Retained::new(Arc::new(ready)));
                        State::Ready(stats)
                    }
                    _ if cancel.is_cancelled() => State::Canceled,
                    Err(error) if previous.is_some() => {
                        entry.path_failure = Some(error);
                        entry
                            .ready
                            .as_ref()
                            .and_then(|r| r.get())
                            .map(|r| State::Ready(r.stats()))
                            .unwrap_or(State::Failed(q::Error::IndexRebuildRequired))
                    }
                    Err(error) => State::Failed(error),
                    _ => State::Canceled,
                };
                entry.expires = Instant::now() + Duration::from_secs(300);
                entry.task.id.clone()
            };
            manager.changes.notify(
                Some(key.binding.repository()),
                Domain::IndexTasks,
                Some(task_id),
            );
        }
    });
}
