//! Parent-edge operation capability for JJ's standard incremental builder.
use super::*;
use jj_lib::{
    object_id::{HexPrefix, PrefixResolution},
    op_store::{OpStore, OpStoreError, OpStoreResult},
    operation::Operation,
};
use std::{sync::Mutex, time::SystemTime};
pub(super) struct Source {
    inner: Arc<dyn OpStore>,
    pin: Arc<Pin>,
    guard: Arc<scoped::Guard>,
    budget: Arc<Budget>,
    allowed: Mutex<BTreeSet<o::OperationId>>,
    views: Mutex<BTreeSet<o::ViewId>>,
    maximum: u64,
}
impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CodeIndexOperationSource")
    }
}
fn error(value: Error) -> OpStoreError {
    OpStoreError::Other(Box::new(value))
}
impl Source {
    pub(super) fn new(
        inner: Arc<dyn OpStore>,
        pin: Arc<Pin>,
        guard: Arc<scoped::Guard>,
        budget: Arc<Budget>,
        maximum: u64,
    ) -> Arc<Self> {
        Arc::new(Self {
            inner,
            pin,
            guard,
            budget,
            allowed: Mutex::default(),
            views: Mutex::default(),
            maximum,
        })
    }
    pub(super) async fn selected(self: &Arc<Self>, id: m::OperationId) -> Result<Operation> {
        let id = o::OperationId::from_bytes(id.as_bytes());
        self.allowed.lock().unwrap().insert(id.clone());
        let data = self
            .read_operation(&id)
            .await
            .map_err(crate::query::language::operations::store_error)?;
        Ok(Operation::new(self.clone(), id, data))
    }
}
#[async_trait::async_trait]
impl OpStore for Source {
    fn name(&self) -> &str {
        self.inner.name()
    }
    fn root_operation_id(&self) -> &o::OperationId {
        self.inner.root_operation_id()
    }
    async fn read_operation(&self, id: &o::OperationId) -> OpStoreResult<o::Operation> {
        self.budget.authorize(&self.pin).await.map_err(error)?;
        if !self.allowed.lock().unwrap().contains(id) {
            return Err(error(Error::NotCommitted));
        }
        bounded_op_file(
            self.pin.binding.metadata_path(),
            "operations",
            &id.hex(),
            &self.budget,
        )
        .map_err(error)?;
        let value = self.inner.read_operation(id).await?;
        let dto = dto::Operation::try_from(&value).map_err(|_| error(Error::Corrupt))?;
        self.budget.metadata(&dto).map_err(error)?;
        if value.parents.len() > 256 {
            return Err(error(Error::Limit));
        }
        if self
            .pin
            .operations
            .get(&operation_id(id).map_err(error)?)
            .is_some_and(|old| old != &dto)
        {
            return Err(error(Error::Corrupt));
        }
        let mut allowed = self.allowed.lock().unwrap();
        for parent in &value.parents {
            self.budget.work(1).map_err(error)?;
            if !allowed.contains(parent) && allowed.len() as u64 >= self.maximum {
                return Err(error(Error::Limit));
            }
            allowed.insert(parent.clone());
        }
        self.views.lock().unwrap().insert(value.view_id.clone());
        self.guard
            .grant(
                Operation::new(self.inner.clone(), id.clone(), value.clone())
                    .all_referenced_commit_ids()
                    .map(|id| git_id(id).map(ObjectKey::Commit))
                    .collect::<Result<Vec<_>>>()
                    .map_err(error)?,
            )
            .map_err(error)?;
        Ok(value)
    }
    async fn read_view(&self, id: &o::ViewId) -> OpStoreResult<o::View> {
        self.budget.authorize(&self.pin).await.map_err(error)?;
        if !self.views.lock().unwrap().contains(id) {
            return Err(error(Error::NotCommitted));
        }
        bounded_op_file(
            self.pin.binding.metadata_path(),
            "views",
            &id.hex(),
            &self.budget,
        )
        .map_err(error)?;
        let value = self.inner.read_view(id).await?;
        let dto = dto::View::try_from(&value).map_err(|_| error(Error::Corrupt))?;
        self.budget.metadata(&dto).map_err(error)?;
        let key = m::ViewId::from_slice(id.as_bytes()).map_err(|_| error(Error::Corrupt))?;
        if self.pin.views.get(&key).is_some_and(|old| old != &dto) {
            return Err(error(Error::Corrupt));
        }
        let view = jj_lib::view::View::new(value.clone(), false);
        self.guard
            .grant(
                view.all_referenced_commit_ids()
                    .map(|id| git_id(id).map(ObjectKey::Commit))
                    .collect::<Result<Vec<_>>>()
                    .map_err(error)?,
            )
            .map_err(error)?;
        Ok(value)
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
        Err(error(Error::Denied))
    }
    async fn gc(&self, _: &[o::OperationId], _: SystemTime) -> OpStoreResult<()> {
        Err(error(Error::Denied))
    }
}
