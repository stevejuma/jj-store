//! Source operation authority is established by parent metadata, never cache presence.
use super::*;
use crate::query::memory::Lease;

pub(in crate::local_browser) async fn validate(
    access: Arc<dyn Access>,
    outer: m::OperationId,
    source: m::OperationId,
    budget: Arc<Budget>,
    limits: Limits,
) -> Result<()> {
    // Bound both the frontier/visited IDs and the guarded operation store's
    // parent grants before growing them. Metadata bodies have their own budget.
    let bytes = limits
        .max_history_visits
        .checked_mul(512)
        .ok_or(Error::Limit)?;
    let _memory = Lease::reserve(bytes as u64)?;
    let context = Context::open(access, outer, budget.clone(), limits, true, vec![], true).await?;
    let selected = operation_store::selected(&context, outer).await?;
    let store = selected.op_store();
    let mut pending = BTreeSet::from([outer]);
    let mut visited = BTreeSet::new();
    while let Some(id) = if pending.remove(&source) {
        Some(source)
    } else {
        pending.pop_first()
    } {
        budget.work(1)?;
        if !visited.insert(id) {
            continue;
        }
        if visited.len() > limits.max_history_visits {
            return Err(Error::Limit);
        }
        let value = store
            .read_operation(&o::OperationId::from_bytes(id.as_bytes()))
            .await
            .map_err(crate::query::language::operations::store_error)?;
        if id == source {
            return Ok(());
        }
        for parent in value.parents {
            budget.work(1)?;
            let parent = operation_id(&parent)?;
            if !visited.contains(&parent) {
                if pending.len() >= limits.max_history_visits {
                    return Err(Error::Limit);
                }
                pending.insert(parent);
            }
        }
    }
    Err(Error::NotCommitted)
}

impl Executor {
    /// Prove the retained outer/source relation without views or Git history.
    pub async fn validate_reference_operation(
        &self,
        access: Arc<dyn Access>,
        outer: m::OperationId,
        source: m::OperationId,
    ) -> Result<()> {
        let limits = self.limits;
        self.worker(access, move |access, budget| {
            Box::pin(validate(access, outer, source, budget, limits))
        })
        .await
    }
}
