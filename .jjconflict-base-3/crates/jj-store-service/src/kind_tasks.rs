//! Authenticated process-local preparation of exact-root native kind projections.
mod state;
mod worker;
pub use crate::index_tasks::{Error, Result};
use crate::{
    auth::Permission,
    native::NativeSession,
    native_inventory::preparation as prep,
    reads::{OpenRead, ReadLimits, ReadSnapshot},
};
use jj_store_jj::query::{self as q, Cancellation};
use jj_store_model as m;
use state::{Entry, Registry};
pub use state::{Kind, Limits, State, Task};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::Instant;

/// Create once per serving instance. Only live jobs retain sessions/credentials;
/// completed reports contain bounded public data and no projection IDs.
pub struct KindTasks {
    changes: crate::freshness::Signals,
    workers: q::workers::Workers,
    limits: Limits,
    registry: Arc<Mutex<Registry>>,
}
impl Drop for KindTasks {
    fn drop(&mut self) {
        for entry in self
            .registry
            .lock()
            .expect("kind task registry")
            .0
            .values_mut()
        {
            entry.cancel();
            entry.cancellation.cancel();
        }
    }
}
impl KindTasks {
    pub fn new(workers: q::workers::Workers, limits: Limits) -> Result<Self> {
        limits.validate()?;
        Ok(Self {
            changes: Default::default(),
            workers,
            limits,
            registry: Default::default(),
        })
    }
    pub fn freshness(&self) -> tokio::sync::broadcast::Receiver<crate::freshness::Change> {
        self.changes.subscribe()
    }
    async fn authorize(&self, session: &NativeSession) -> Result<(m::PrincipalId, m::EpochId)> {
        let deadline = Instant::now()
            .checked_add(self.limits.read.timeout)
            .ok_or(Error::Limit)?;
        let result = tokio::time::timeout_at(deadline, async {
            let principal = session
                .principal(Permission::Read)
                .await
                .map_err(crate::index_tasks::map_wal)?;
            let snapshot = session
                .snapshot()
                .await
                .map_err(crate::index_tasks::map_wal)?;
            Ok((principal.id(), snapshot.manifest().epoch))
        })
        .await
        .map_err(|_| Error::Limit)?;
        if Instant::now() >= deadline {
            return Err(Error::Limit);
        }
        result
    }
    pub async fn start(
        &self,
        session: Arc<NativeSession>,
        operation: m::OperationId,
        kind: Kind,
    ) -> Result<Task> {
        let (owner, epoch) = self.authorize(&session).await?;
        {
            let mut registry = self.registry.lock().expect("kind task registry");
            registry.prune();
            if let Some(task) = registry.existing(session.repository, epoch, operation, kind, owner)
            {
                return Ok(task);
            }
            if registry.0.len() >= self.limits.retained {
                return Err(Error::Busy);
            }
        }
        // The same reserved lease bounds graph/local/source preparation, including
        // capture. No second pool or queued job can multiply background work.
        let slot = self
            .workers
            .acquire_preparation()
            .map_err(|_| Error::Busy)?;
        let OpenRead::Pinned(read) = session.open_read(Some(operation), self.limits.read).await?
        else {
            return Err(Error::Unavailable);
        };
        let snapshot = read.snapshot().await?.clone();
        if snapshot.authority.epoch != epoch {
            return Err(Error::EpochChanged);
        }
        let preparation = read
            .begin_kind_preparation(kind == Kind::Copies, self.limits.preparation)
            .await?;
        drop(read);
        let mut random = [0; 16];
        getrandom::fill(&mut random).map_err(|_| Error::Unavailable)?;
        let id = hex::encode(random);
        let cancellation = Cancellation::default();
        let expires = preparation.deadline();
        let task = Task {
            version: 1,
            id: id.clone(),
            snapshot,
            kind,
            progress: preparation.progress(),
            state: State::Running,
        };
        {
            let mut registry = self.registry.lock().expect("kind task registry");
            registry.prune();
            if registry.0.len() >= self.limits.retained {
                return Err(Error::Busy);
            }
            if registry.0.contains_key(&id) {
                return Err(Error::Unavailable);
            }
            registry.0.insert(
                id.clone(),
                Entry {
                    task: task.clone(),
                    owner,
                    expires,
                    cancellation: cancellation.clone(),
                },
            );
        }
        worker::spawn(
            self,
            session,
            preparation,
            slot,
            task.clone(),
            owner,
            expires,
            cancellation,
        );
        Ok(task)
    }
    pub async fn status(&self, session: Arc<NativeSession>, id: &str) -> Result<Task> {
        self.inspect(session, id, false).await
    }
    pub async fn cancel(&self, session: Arc<NativeSession>, id: &str) -> Result<Task> {
        self.inspect(session, id, true).await
    }
    async fn inspect(&self, session: Arc<NativeSession>, id: &str, cancel: bool) -> Result<Task> {
        let (principal, epoch) = self.authorize(&session).await?;
        if id.len() != 32 || !id.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(Error::Reset);
        }
        let mut registry = self.registry.lock().expect("kind task registry");
        registry.prune();
        let entry = registry.0.get_mut(id).ok_or(Error::Reset)?;
        if entry.task.snapshot.authority.repo_id != session.repository {
            return Err(Error::Denied);
        }
        if entry.task.snapshot.authority.epoch != epoch {
            return Err(Error::EpochChanged);
        }
        if Instant::now() >= entry.expires {
            return Err(Error::Reset);
        }
        if cancel {
            if entry.owner != principal {
                return Err(Error::Denied);
            }
            entry.cancel();
        }
        Ok(entry.task.clone())
    }
}
