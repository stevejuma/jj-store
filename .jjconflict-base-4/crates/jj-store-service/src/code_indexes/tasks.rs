//! Compatibility reports over the shared owner; control always reauthorizes.
use super::*;
use crate::{
    auth::Permission, index_tasks as native, local_index_tasks as local_tasks,
    native::NativeSession,
};
use std::time::Duration;

impl Task {
    pub fn native(self) -> native::Result<native::Task> {
        let Snapshot::Native(snapshot) = self.snapshot else {
            return Err(native::Error::Denied);
        };
        Ok(native::Task {
            version: 1,
            id: self.id,
            snapshot,
            state: match self.state {
                State::Running => native::State::Running,
                State::Canceling => native::State::Canceling,
                State::Canceled => native::State::Canceled,
                State::Failed(error) => native::State::Failed { error },
                State::Ready(stats) => native::State::Ready {
                    index: q::IndexReport {
                        version: 1,
                        operation: self.key.operation,
                        stats,
                        cache: q::cache::Report {
                            state: q::cache::State::Reused,
                            evicted: 0,
                            retained_entries: 1,
                            retained_bytes: 0,
                        },
                    },
                },
            },
        })
    }
    pub fn local(self) -> local_tasks::Result<local_tasks::LocalIndexTask> {
        let Snapshot::Local(snapshot) = self.snapshot else {
            return Err(local::Error::Denied.into());
        };
        Ok(local_tasks::LocalIndexTask {
            version: 1,
            id: self.id,
            snapshot,
            state: match self.state {
                State::Running => local_tasks::State::Running,
                State::Canceling => local_tasks::State::Canceling,
                State::Canceled => local_tasks::State::Canceled,
                State::Failed(error) => local_tasks::State::Failed { error },
                State::Ready(s) => local_tasks::State::Ready {
                    stats: local::Stats {
                        metadata_nodes: s.metadata_nodes,
                        predicate_file_bytes: s.predicate_file_bytes,
                        predicate_lines: s.predicate_lines,
                        object_reads: s.object_reads,
                        object_bytes: s.object_bytes,
                        work: s.work,
                    },
                },
            },
        })
    }
}
impl Indexes {
    fn task_key(&self, repository: m::RepositoryId, id: &str) -> q::Result<Key> {
        if id.len() != 32 || !id.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(q::Error::NotFound);
        }
        self.registry
            .lock()
            .unwrap()
            .entries
            .iter()
            .find(|(key, e)| key.binding.repository() == repository && e.task.id == id)
            .map(|(key, _)| key.clone())
            .ok_or(q::Error::NotFound)
    }
    pub async fn inspect_local(
        &self,
        access: Arc<dyn local::Access>,
        id: &str,
        cancel: bool,
    ) -> local_tasks::Result<local_tasks::LocalIndexTask> {
        Self::authorize_local(access.as_ref()).await?;
        let key = self
            .task_key(access.repository(), id)
            .map_err(|e| match e {
                q::Error::NotFound => local_tasks::Error::Reset,
                e => e.into(),
            })?;
        if key.binding
            != (Binding::Local {
                repository: access.repository(),
                generation: access.generation(),
            })
        {
            return Err(q::Error::EpochChanged.into());
        }
        self.inspect(&key, id, cancel)
            .map_err(|e| match e {
                q::Error::NotFound => local_tasks::Error::Reset,
                e => e.into(),
            })?
            .local()
    }
    /// Used only by LocalTrusted HTTP serving; authenticated task ownership keeps
    /// its existing principal-scoped facade until automatic maintenance is qualified.
    pub async fn inspect_native(
        &self,
        session: Arc<NativeSession>,
        id: &str,
        cancel: bool,
    ) -> native::Result<native::Task> {
        let epoch = tokio::time::timeout(Duration::from_secs(10), async {
            session
                .principal(Permission::Read)
                .await
                .map_err(native::map_wal)?;
            Ok::<_, native::Error>(
                session
                    .snapshot()
                    .await
                    .map_err(native::map_wal)?
                    .manifest()
                    .epoch,
            )
        })
        .await
        .map_err(|_| native::Error::Limit)??;
        let key = self.task_key(session.repository, id).map_err(|e| match e {
            q::Error::NotFound => native::Error::Reset,
            e => e.into(),
        })?;
        if key.binding
            != (Binding::Native {
                repository: session.repository,
                epoch,
            })
        {
            return Err(native::Error::EpochChanged);
        }
        self.inspect(&key, id, cancel)
            .map_err(|e| match e {
                q::Error::NotFound => native::Error::Reset,
                e => e.into(),
            })?
            .native()
    }
}
