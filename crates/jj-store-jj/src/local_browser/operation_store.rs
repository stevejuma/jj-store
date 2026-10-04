//! Read-only parent-edge capability for operation metadata, independent of commits.
use super::*;
use jj_lib::{
    object_id::{HexPrefix, PrefixResolution},
    op_store::{OpStore, OpStoreError, OpStoreResult},
};
use std::{fmt, sync::Mutex, time::SystemTime};

struct Guard {
    source: Arc<dyn OpStore>,
    access: Arc<Pin>,
    budget: Arc<Budget>,
    allowed: Mutex<BTreeSet<o::OperationId>>,
    visited: Mutex<BTreeSet<o::OperationId>>,
    max_operations: usize,
    view: Option<o::ViewId>,
    evolution: bool,
}
impl fmt::Debug for Guard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OperationReadGuard")
    }
}
fn error(value: Error) -> OpStoreError {
    OpStoreError::Other(Box::new(value))
}
pub(super) fn fingerprint(operation: &dto::Operation, budget: &Budget) -> Result<[u8; 32]> {
    use sha2::{Digest as _, Sha256};
    struct Hash<'a> {
        digest: Sha256,
        budget: &'a Budget,
        failed: Option<Error>,
    }
    impl std::io::Write for Hash<'_> {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if let Err(error) = self.budget.work(1) {
                self.failed = Some(error.clone());
                return Err(std::io::Error::other(error));
            }
            self.digest.update(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut hash = Hash {
        digest: Sha256::new(),
        budget,
        failed: None,
    };
    hash.digest.update(b"jj-store/local-operation/v1\0");
    serde_json::to_writer(&mut hash, operation)
        .map_err(|_| hash.failed.take().unwrap_or(Error::Corrupt))?;
    Ok(hash.digest.finalize().into())
}
#[async_trait::async_trait]
impl OpStore for Guard {
    fn name(&self) -> &str {
        self.source.name()
    }
    fn root_operation_id(&self) -> &o::OperationId {
        self.source.root_operation_id()
    }
    async fn read_operation(&self, id: &o::OperationId) -> OpStoreResult<o::Operation> {
        self.budget.authorize(&self.access).await.map_err(error)?;
        let prepared = self.access.prepared.as_ref().filter(|_| !self.evolution);
        let wire_id = operation_id(id).map_err(error)?;
        let expected = prepared
            .map(|p| {
                p.operation_fingerprint(wire_id, &self.budget)
                    .map_err(error)
            })
            .transpose()?
            .flatten();
        if !self.allowed.lock().unwrap().contains(id) && expected.is_none() {
            return Err(error(Error::NotCommitted));
        }
        if prepared.is_some() {
            let mut visited = self.visited.lock().unwrap();
            if !visited.contains(id) && visited.len() >= self.max_operations {
                return Err(error(Error::Limit));
            }
            visited.insert(id.clone());
        }
        bounded_op_file(
            self.access.binding.metadata_path(),
            "operations",
            &id.hex(),
            &self.budget,
        )
        .map_err(error)?;
        let value = self.source.read_operation(id).await?;
        let metadata = dto::Operation::try_from(&value).map_err(|_| error(Error::Corrupt))?;
        self.budget.metadata(&metadata).map_err(error)?;
        if let Some(expected) = expected {
            if fingerprint(&metadata, &self.budget).map_err(error)? != expected {
                return Err(error(Error::Corrupt));
            }
        }
        if self
            .access
            .operations
            .get(&operation_id(id).map_err(error)?)
            .is_some_and(|selected| selected != &metadata)
        {
            return Err(error(Error::Corrupt));
        }
        if self.evolution {
            // JJ's predecessor walker uses splice/duplicate/topological work
            // inside one synchronous operation visit. Admit the complete batch
            // before it can expand its queues, including shared predecessors.
            let mut edges = 0usize;
            for prior in value
                .commit_predecessors
                .iter()
                .flat_map(|map| map.values())
            {
                self.budget.work(1).map_err(error)?;
                if prior.len() > 256 {
                    return Err(error(Error::Limit));
                }
                edges = edges
                    .checked_add(1 + prior.len())
                    .ok_or_else(|| error(Error::Limit))?;
            }
            self.budget
                .work(
                    edges
                        .checked_mul(edges)
                        .ok_or_else(|| error(Error::Limit))?,
                )
                .map_err(error)?;
        }
        if prepared.is_some() {
            return Ok(value);
        }
        let mut allowed = self.allowed.lock().unwrap();
        for parent in &value.parents {
            self.budget.work(1).map_err(error)?;
            if !allowed.contains(parent) && allowed.len() >= self.max_operations {
                return Err(error(Error::Limit));
            }
            allowed.insert(parent.clone());
        }
        Ok(value)
    }
    async fn read_view(&self, id: &o::ViewId) -> OpStoreResult<o::View> {
        self.budget.authorize(&self.access).await.map_err(error)?;
        if self.view.as_ref() != Some(id) {
            return Err(error(Error::Denied));
        }
        bounded_op_file(
            self.access.binding.metadata_path(),
            "views",
            &id.hex(),
            &self.budget,
        )
        .map_err(error)?;
        let view = self.source.read_view(id).await?;
        let metadata = dto::View::try_from(&view).map_err(|_| error(Error::Corrupt))?;
        self.budget.metadata(&metadata).map_err(error)?;
        let id = m::ViewId::from_slice(id.as_bytes()).map_err(|_| error(Error::Corrupt))?;
        if self
            .access
            .views
            .get(&id)
            .is_some_and(|selected| selected != &metadata)
        {
            return Err(error(Error::Corrupt));
        }
        Ok(view)
    }
    async fn write_view(&self, _: &o::View) -> OpStoreResult<o::ViewId> {
        Err(error(Error::Denied))
    }
    async fn write_operation(&self, _: &o::Operation) -> OpStoreResult<o::OperationId> {
        Err(error(Error::Denied))
    }
    async fn resolve_operation_id_prefix(
        &self,
        prefix: &HexPrefix,
    ) -> OpStoreResult<PrefixResolution<o::OperationId>> {
        self.budget.authorize(&self.access).await.map_err(error)?;
        self.access
            .prepared
            .as_ref()
            .ok_or_else(|| error(Error::Invalid))?
            .resolve_operation(prefix, &self.budget)
            .map_err(error)
    }
    async fn gc(&self, _: &[o::OperationId], _: SystemTime) -> OpStoreResult<()> {
        Err(error(Error::Denied))
    }
}

pub(super) async fn selected(
    context: &Context,
    operation: m::OperationId,
) -> Result<jj_lib::operation::Operation> {
    selected_for(context, operation, false).await
}
pub(super) async fn selected_for(
    context: &Context,
    operation: m::OperationId,
    evolution: bool,
) -> Result<jj_lib::operation::Operation> {
    context.member(ObjectKey::Operation(operation)).await?;
    let id = o::OperationId::from_bytes(operation.as_bytes());
    let source = loader(context.access.binding.metadata_path())?
        .op_store()
        .clone();
    let guard: Arc<dyn OpStore> = Arc::new(Guard {
        source,
        access: context.access.clone(),
        budget: context.budget.clone(),
        allowed: Mutex::new(BTreeSet::from([id.clone()])),
        visited: Mutex::default(),
        max_operations: context.limits.max_history_visits,
        evolution,
        view: context.access.prepared.as_ref().map(|_| {
            o::ViewId::from_bytes(context.access.operations[&operation].view_id.as_bytes())
        }),
    });
    let data = guard
        .read_operation(&id)
        .await
        .map_err(crate::query::language::operations::store_error)?;
    Ok(jj_lib::operation::Operation::new(guard, id, data))
}
