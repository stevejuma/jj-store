//! Request-scoped metadata reads over an immutable retained operation closure.
use super::*;
use jj_lib::{
    object_id::{HexPrefix, ObjectId as _, PrefixResolution},
    op_store::{OpStore, OpStoreError, OpStoreResult, OperationId, ViewId},
    operation::Operation,
};
use std::{fmt, time::SystemTime};

struct Guard {
    source: Arc<dyn OpStore>,
    lookup: Arc<prepared::operations::Lookup>,
    access: Arc<dyn ReadAccess>,
    budget: Arc<Budget>,
}
impl fmt::Debug for Guard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PreparedNativeOperationGuard")
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
    fn root_operation_id(&self) -> &OperationId {
        self.source.root_operation_id()
    }
    async fn read_operation(&self, id: &OperationId) -> OpStoreResult<jj_lib::op_store::Operation> {
        self.budget
            .authorize(self.access.as_ref())
            .await
            .map_err(error)?;
        let owned = m::OperationId::from_slice(id.as_bytes()).map_err(|_| error(Error::Corrupt))?;
        if self
            .lookup
            .fingerprint(owned, |amount| self.budget.work(amount))
            .map_err(error)?
            .is_none()
        {
            return Err(error(Error::NotCommitted));
        }
        // The source independently proves current capability membership and the
        // complete native envelope/identity. The lookup is never authority.
        self.source.read_operation(id).await
    }
    async fn resolve_operation_id_prefix(
        &self,
        prefix: &HexPrefix,
    ) -> OpStoreResult<PrefixResolution<OperationId>> {
        self.budget
            .authorize(self.access.as_ref())
            .await
            .map_err(error)?;
        self.lookup
            .resolve(prefix, |amount| self.budget.work(amount))
            .map_err(error)
    }
    async fn read_view(&self, _: &ViewId) -> OpStoreResult<jj_lib::op_store::View> {
        Err(error(Error::Denied))
    }
    async fn write_view(&self, _: &jj_lib::op_store::View) -> OpStoreResult<ViewId> {
        Err(error(Error::Denied))
    }
    async fn write_operation(&self, _: &jj_lib::op_store::Operation) -> OpStoreResult<OperationId> {
        Err(error(Error::Denied))
    }
    async fn gc(&self, _: &[OperationId], _: SystemTime) -> OpStoreResult<()> {
        Err(error(Error::Denied))
    }
}

pub(super) async fn start(
    head: &Operation,
    filter: &language::operations::Filter,
    lookup: Arc<prepared::operations::Lookup>,
    access: Arc<dyn ReadAccess>,
    budget: Arc<Budget>,
) -> Result<Operation> {
    budget.authorize(access.as_ref()).await?;
    let source: Arc<dyn OpStore> = Arc::new(Guard {
        source: head.op_store().clone(),
        lookup: lookup.clone(),
        access: access.clone(),
        budget: budget.clone(),
    });
    let mut selected_filter = filter.clone();
    let selected = if let Some(selector) = filter.from.as_deref() {
        let symbol = selector.trim_end_matches(['-', '+']);
        let selected = if symbol == "@" {
            // Operation::clone shares its immutable body. Replacing its store
            // would copy potentially large predecessor maps needlessly.
            head.clone()
        } else {
            let prefix = HexPrefix::try_from_hex(symbol).ok_or(Error::Invalid)?;
            let id = match source
                .resolve_operation_id_prefix(&prefix)
                .await
                .map_err(language::operations::store_error)?
            {
                PrefixResolution::SingleMatch(id) => id,
                _ => {
                    return Err(language::invalid(
                        "unresolved_symbol",
                        "Operation selector is missing or ambiguous in this history.",
                    ));
                }
            };
            let data = source
                .read_operation(&id)
                .await
                .map_err(language::operations::store_error)?;
            Operation::new(source.clone(), id, data)
        };
        // Interpret only the suffix relative to the admitted base. The full
        // external query/snapshot remains unchanged for filtering and cursors.
        selected_filter.from = Some(format!("@{}", &selector[symbol.len()..]));
        selected
    } else {
        head.clone()
    };
    selected_filter
        .start_at_indexed(
            &selected,
            async |parent, prior| {
                budget.authorize(access.as_ref()).await?;
                let parent_id = m::OperationId::from_slice(parent.id().as_bytes())
                    .map_err(|_| Error::Corrupt)?;
                let id = match lookup.child(parent_id, |amount| budget.work(amount))? {
                    PrefixResolution::SingleMatch(id) => id,
                    _ => {
                        return Err(language::invalid(
                            "unresolved_symbol",
                            "Operation selector is missing or ambiguous in this history.",
                        ));
                    }
                };
                if let Some(prior) = prior.filter(|prior| prior.id() == &id) {
                    budget.work(prior.parent_ids().len())?;
                    if !prior.parent_ids().contains(parent.id()) {
                        return Err(Error::Corrupt);
                    }
                    return Ok(prior.clone());
                }
                let data = source
                    .read_operation(&id)
                    .await
                    .map_err(language::operations::store_error)?;
                budget.work(data.parents.len())?;
                if !data.parents.contains(parent.id()) {
                    return Err(Error::Corrupt);
                }
                Ok(Operation::new(source.clone(), id, data))
            },
            |amount| budget.work(amount),
        )
        .await
}
