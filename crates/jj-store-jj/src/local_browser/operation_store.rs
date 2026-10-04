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
    max_operations: usize,
    view: Option<o::ViewId>,
}
impl fmt::Debug for Guard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OperationReadGuard")
    }
}
fn error(value: Error) -> OpStoreError {
    OpStoreError::Other(Box::new(value))
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
        if !self.allowed.lock().unwrap().contains(id) {
            return Err(error(Error::NotCommitted));
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
        self.budget
            .metadata(&dto::View::try_from(&view).map_err(|_| error(Error::Corrupt))?)
            .map_err(error)?;
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
        _: &HexPrefix,
    ) -> OpStoreResult<PrefixResolution<o::OperationId>> {
        // Prefix requests use the existing complete-admission path, not this capability.
        Err(error(Error::Invalid))
    }
    async fn gc(&self, _: &[o::OperationId], _: SystemTime) -> OpStoreResult<()> {
        Err(error(Error::Denied))
    }
}

pub(super) async fn selected(
    context: &Context,
    operation: m::OperationId,
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
        max_operations: context.limits.max_history_visits,
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
