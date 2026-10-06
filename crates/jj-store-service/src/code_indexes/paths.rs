//! Quick metadata is final; optional history reuses the core index in bounded batches.
use super::*;
use q::content::{ContentReader, paths as native};
impl Indexes {
    pub(super) fn path_start_allowed(&self, key: &Key) -> q::Result<()> {
        if let Some(entry) = self.registry.lock().unwrap().entries.get(key) {
            match &entry.task.state {
                State::Failed(error) => return Err(error.clone()),
                State::Canceled | State::Canceling => return Err(q::Error::Canceled),
                _ => {}
            }
        }
        Ok(())
    }

    pub async fn native_paths(
        self: &Arc<Self>,
        read: Arc<CommittedRead>,
        reader: &ContentReader,
        mut request: native::Request,
    ) -> q::Result<native::Page> {
        let snapshot = read.snapshot().await.map_err(crate::reads::query_error)?;
        let key = Key {
            binding: Binding::Native {
                repository: snapshot.authority.repo_id,
                epoch: snapshot.authority.epoch,
            },
            operation: snapshot.operation_id,
        };
        match self.ready(&key) {
            Ok(ready) => {
                read.paths(
                    &reader
                        .clone()
                        .with_code_index(ready.native().ok_or(q::Error::Corrupt)?),
                    request,
                )
                .await
            }
            Err(q::Error::IndexRebuildRequired) => {
                if request.include_last_change
                    && request.last_change_mode == native::LastChangeMode::History
                {
                    self.path_start_allowed(&key)?;
                    self.start_native(read.clone()).await?;
                    request.include_last_change = false;
                    let mut page = read.paths(reader, request).await?;
                    for entry in &mut page.entries {
                        entry.last_change = native::LastChange::Unavailable {
                            reason: native::LastChangeUnavailable::Pending,
                        };
                    }
                    Ok(page)
                } else {
                    read.paths(reader, request).await
                }
            }
            Err(error) => Err(error),
        }
    }
    pub(super) async fn local_paths(
        self: &Arc<Self>,
        access: Arc<dyn local::Access>,
        operation: m::OperationId,
        mut request: local::paths::Request,
    ) -> q::Result<local::Response> {
        Self::authorize_local(access.as_ref()).await?;
        let key = Key {
            binding: Binding::Local {
                repository: access.repository(),
                generation: access.generation(),
            },
            operation,
        };
        match self.ready(&key) {
            Ok(ready) => {
                self.local
                    .read_code_index(
                        access,
                        ready.local().ok_or(q::Error::Corrupt)?,
                        local::Request::Paths(request),
                    )
                    .await
            }
            Err(q::Error::IndexRebuildRequired) => {
                let history = request.include_last_change
                    && request.last_change_mode == native::LastChangeMode::History;
                if history {
                    self.path_start_allowed(&key)?;
                    self.start_local(access.clone(), operation).await?;
                    request.include_last_change = false;
                }
                let mut response = self
                    .local
                    .read(access, operation, local::Request::Paths(request))
                    .await?;
                if history && let local::Response::Paths(page) = &mut response {
                    for entry in &mut page.entries {
                        entry.last_change = local::paths::LastChange::Unavailable {
                            reason: local::paths::LastChangeUnavailable::Pending,
                        };
                    }
                }
                Ok(response)
            }
            Err(error) => Err(error),
        }
    }
}
