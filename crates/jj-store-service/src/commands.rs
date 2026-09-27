//! Checked native preparation; durable public preview/apply/status follows here.
use crate::{auth::Permission, native::NativeSession};
pub use jj_store_jj::commands::{
    Command, ConflictResolution, Error, Executor, Limits, Prepared, Resolution, Result,
};
use jj_store_jj::{
    commands::MutationAccess,
    query::{self, ReadAccess},
};
use jj_store_model as m;
use jj_store_wal::{self as wal, publication::Snapshot};
use std::{collections::BTreeSet, sync::Arc};

struct Access {
    session: Arc<NativeSession>,
    expected: Snapshot,
    principal: m::PrincipalId,
    operation: m::OperationId,
    inventory: BTreeSet<m::ObjectKey>,
}
fn error(error: wal::Error) -> query::Error {
    match error {
        wal::Error::Denied => query::Error::Denied,
        wal::Error::Limit(_) => query::Error::Limit,
        _ => query::Error::Unavailable,
    }
}
#[async_trait::async_trait]
impl ReadAccess for Access {
    fn repository(&self) -> m::RepositoryId {
        self.session.repository
    }
    fn epoch(&self) -> m::EpochId {
        self.expected.epoch
    }
    fn operation(&self) -> m::OperationId {
        self.operation
    }
    fn inventory(&self) -> &BTreeSet<m::ObjectKey> {
        &self.inventory
    }
    async fn authorize(&self) -> query::Result<()> {
        self.authorize_write().await
    }
}
#[async_trait::async_trait]
impl MutationAccess for Access {
    fn principal(&self) -> m::PrincipalId {
        self.principal
    }
    fn heads(&self) -> &BTreeSet<m::OperationId> {
        &self.expected.heads
    }
    async fn authorize_write(&self) -> query::Result<()> {
        let principal = self
            .session
            .principal(Permission::Write)
            .await
            .map_err(error)?;
        if principal.id() != self.principal {
            return Err(query::Error::Denied);
        }
        let snapshot = self.session.snapshot().await.map_err(error)?;
        if snapshot.manifest().epoch != self.expected.epoch {
            return Err(query::Error::EpochChanged);
        }
        Ok(())
    }
}
impl NativeSession {
    pub async fn prepare_command(
        self: &Arc<Self>,
        executor: &Executor,
        expected: &Snapshot,
        command: Command,
    ) -> Result<Prepared> {
        tokio::time::timeout(self.limits.timeout, async {
            let principal = self.principal(Permission::Write).await.map_err(error)?;
            let before = self.snapshot().await.map_err(error)?;
            if before.manifest().snapshot() != *expected {
                return Err(Error::Stale);
            }
            let operation = match &command {
                Command::ReconcileOperations { base } => {
                    if !(2..=64).contains(&expected.heads.len()) || !expected.heads.contains(base) {
                        return Err(Error::Invalid);
                    }
                    *base
                }
                _ => {
                    if expected.heads.len() != 1 {
                        return Err(Error::MultipleHeads);
                    }
                    *expected.heads.first().unwrap()
                }
            };
            let inventory = self
                .coordinator
                .inventory(&before, self.limits.inventory_nodes)
                .await
                .map_err(error)?
                .into_iter()
                .map(|node| node.key)
                .collect();
            let access = Arc::new(Access {
                session: self.clone(),
                expected: expected.clone(),
                principal: principal.id(),
                operation,
                inventory,
            });
            access.authorize_write().await?;
            let prepared = executor
                .prepare(self.object_store.clone(), access.clone(), command)
                .await?;
            access.authorize_write().await?;
            if self.snapshot().await.map_err(error)?.manifest().snapshot() != *expected {
                return Err(Error::Stale);
            }
            Ok(prepared)
        })
        .await
        .map_err(|_| query::Error::Limit)?
    }
}
