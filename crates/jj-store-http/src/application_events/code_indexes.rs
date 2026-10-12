//! Core-index maintenance is driven by the existing repository monitor.
use super::*;

pub(crate) fn resume(state: &Arc<State>) -> Option<tokio::task::JoinHandle<()>> {
    if !matches!(state.config.access, Access::LocalTrusted { .. }) {
        return None;
    }
    let Ok(runtime) = tokio::runtime::Handle::try_current() else {
        return None;
    };
    let weak = Arc::downgrade(state);
    Some(runtime.spawn(async move {
        let Some(state) = weak.upgrade() else {
            return;
        };
        state.event_monitors.activate(&state, None, false).await;
        if let Ok(bindings) = state.code_indexes.retained().await {
            for binding in bindings {
                let local = matches!(binding, q::cache::managed::Binding::Local { .. });
                state
                    .event_monitors
                    .activate(&state, Some(binding.repository()), local)
                    .await;
            }
        }
    }))
}
pub(super) async fn maintain(
    state: &State,
    repository: Option<m::RepositoryId>,
    local: bool,
    local_heads: Option<jj_store_service::local_browser::code::Heads>,
) -> Result<()> {
    let Some(repository) = repository else {
        return Ok(());
    };
    let result = async {
        if local {
            let access = crate::local_browser::binding(state, repository).await?;
            let heads = match local_heads {
                Some(heads) => heads,
                None => state.local_reader.heads(access.clone()).await?,
            };
            if heads.generation != access.generation() {
                return Ok(());
            }
            let divergent = heads.heads.len() > 1;
            for operation in heads.heads {
                let key = jj_store_service::code_indexes::Key {
                    binding: q::cache::managed::Binding::Local {
                        repository,
                        generation: heads.generation,
                    },
                    operation,
                };
                if state.code_indexes.needs(&key) {
                    // Preserve divergent head selections. Bookmarks share the same namespace.
                    if divergent {
                        state
                            .code_indexes
                            .start_local(access.clone(), operation)
                            .await?;
                    } else {
                        state
                            .code_indexes
                            .maintain_local(access.clone(), operation)
                            .await?;
                    }
                }
            }
        } else {
            let Access::LocalTrusted {
                principal,
                permission,
            } = state.config.access
            else {
                return Ok(());
            };
            let session = state
                .host
                .as_ref()
                .ok_or_else(Error::unavailable)?
                .endpoint(repository)?
                .bind_local_trusted(principal, permission)?
                .into_session();
            let selected = session.snapshot().await.map_err(reads::Error::from)?;
            let divergent = selected.manifest().heads.len() > 1;
            for &operation in &selected.manifest().heads {
                let key = jj_store_service::code_indexes::Key {
                    binding: q::cache::managed::Binding::Native {
                        repository,
                        epoch: selected.manifest().epoch,
                    },
                    operation,
                };
                if state.code_indexes.needs(&key) {
                    if let reads::OpenRead::Pinned(read) = session
                        .open_read(Some(operation), state.live_config().read)
                        .await?
                    {
                        if divergent {
                            state.code_indexes.start_native(Arc::new(read)).await?;
                        } else {
                            state.code_indexes.maintain_native(Arc::new(read)).await?;
                        }
                    }
                }
            }
        }
        Ok::<(), Error>(())
    };
    tokio::time::timeout(Duration::from_secs(10), result)
        .await
        .map_err(|_| Error::timeout())?
}
