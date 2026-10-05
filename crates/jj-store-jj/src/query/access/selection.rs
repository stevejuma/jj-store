//! A source selector retains the complete outer read capability.
use super::*;

pub(in crate::query) async fn select_operation(
    outer: Arc<dyn ReadAccess>,
    operation: m::OperationId,
    budget: &Budget,
) -> Result<Arc<dyn ReadAccess>> {
    if operation == outer.operation() {
        return Ok(outer);
    }
    budget.work(1)?;
    if !budget
        .until(outer.contains(m::ObjectKey::Operation(operation)))
        .await??
    {
        return Err(Error::NotCommitted);
    }
    Ok(Arc::new(SelectedOperation { outer, operation }))
}

struct SelectedOperation {
    outer: Arc<dyn ReadAccess>,
    operation: m::OperationId,
}
#[async_trait::async_trait]
impl ReadAccess for SelectedOperation {
    fn repository(&self) -> m::RepositoryId {
        self.outer.repository()
    }
    fn epoch(&self) -> m::EpochId {
        self.outer.epoch()
    }
    fn operation(&self) -> m::OperationId {
        self.operation
    }
    async fn contains(&self, key: m::ObjectKey) -> Result<bool> {
        self.outer.contains(key).await
    }
    async fn inventory(&self) -> Result<Arc<BTreeSet<m::ObjectKey>>> {
        self.outer.inventory().await
    }
    async fn inventory_kind(
        &self,
        kind: InventoryKind,
        limit: usize,
        deadline: Instant,
    ) -> Result<BTreeSet<m::ObjectKey>> {
        self.outer.inventory_kind(kind, limit, deadline).await
    }
    async fn authorize(&self) -> Result<()> {
        self.outer.authorize().await
    }
}
