//! Optional attribution never blocks names on graph or changed-path preparation.
use super::*;
use q::content::{ContentReader, paths as native};
impl Indexes {
    fn path_start_allowed(&self, key: &Key) -> q::Result<()> {
        if let Some(entry) = self.registry.lock().unwrap().entries.get(key) {
            match &entry.task.state {
                State::Failed(error) => return Err(error.clone()),
                State::Canceled | State::Canceling => return Err(q::Error::Canceled),
                _ => {}
            }
        }
        Ok(())
    }
    fn maintain_paths(self: &Arc<Self>, source: Source, previous: Arc<Ready>) -> q::Result<()> {
        let key = source.key();
        let mut registry = self.registry.lock().unwrap();
        if registry.pending.len() >= 64 {
            return Err(q::Error::Busy);
        }
        let entry = registry
            .entries
            .get_mut(&key)
            .ok_or(q::Error::IndexRebuildRequired)?;
        if entry.task.state.live() || entry.path_failure.is_some() {
            return Ok(());
        }
        entry.task.state = State::Running;
        entry.expires = Instant::now() + std::time::Duration::from_secs(30);
        entry.cancel = Default::default();
        registry.pending.push_back(Job {
            source,
            previous: Some(previous),
        });
        if !registry.active {
            registry.active = true;
            jobs::spawn(Arc::downgrade(self));
        }
        Ok(())
    }
    pub async fn native_paths(
        self: &Arc<Self>,
        read: Arc<CommittedRead>,
        reader: &ContentReader,
        mut request: native::Request,
    ) -> q::Result<native::Page> {
        if !request.include_last_change {
            return read.paths(reader, request).await;
        }
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
                let page = read
                    .paths(
                        &reader
                            .clone()
                            .with_code_index(ready.native().ok_or(q::Error::Corrupt)?),
                        request,
                    )
                    .await?;
                if page.entries.iter().any(|e| {
                    matches!(
                        e.last_change,
                        native::LastChange::Unavailable {
                            reason: native::LastChangeUnavailable::Pending
                        }
                    )
                }) {
                    let source = Source::Native(read.code_source().await?);
                    // Optional preparation pressure must not discard this usable batch.
                    if let Err(error) = self.maintain_paths(source, ready)
                        && !matches!(error, q::Error::Busy)
                    {
                        return Err(error);
                    }
                }
                Ok(page)
            }
            Err(q::Error::IndexRebuildRequired) => {
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
        let snapshot = local::Snapshot {
            version: 1,
            profile: local::PROFILE,
            repository: access.repository(),
            generation: access.generation(),
            operation,
        };
        let key = Key {
            binding: Binding::Local {
                repository: snapshot.repository,
                generation: snapshot.generation,
            },
            operation,
        };
        match self.ready(&key) {
            Ok(ready) => {
                let response = self
                    .local
                    .read_code_index(
                        access.clone(),
                        ready.local().ok_or(q::Error::Corrupt)?,
                        local::Request::Paths(request),
                    )
                    .await?;
                if let local::Response::Paths(page) = &response
                    && page.entries.iter().any(|e| {
                        matches!(
                            e.last_change,
                            local::paths::LastChange::Unavailable {
                                reason: local::paths::LastChangeUnavailable::Pending
                            }
                        )
                    })
                {
                    if let Err(error) =
                        self.maintain_paths(Source::Local { access, snapshot }, ready)
                        && !matches!(error, q::Error::Busy)
                    {
                        return Err(error);
                    }
                }
                Ok(response)
            }
            Err(q::Error::IndexRebuildRequired) => {
                self.path_start_allowed(&key)?;
                self.start_local(access.clone(), operation).await?;
                request.include_last_change = false;
                let mut response = self
                    .local
                    .read(access, operation, local::Request::Paths(request))
                    .await?;
                if let local::Response::Paths(page) = &mut response {
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
