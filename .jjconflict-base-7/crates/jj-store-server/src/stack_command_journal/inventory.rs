use super::*;
use crate::stack_checkout_plans::{SavedTreePlanV1, StackCheckoutPlans};
use jj_store_jj::local_execution::PhysicalInventoryV1;
fn original_plan(
    record: &Record,
    store: &StackCheckoutPlans,
    scope: &Scope,
) -> Result<SavedTreePlanV1> {
    let execution = record.execution.as_ref().ok_or(Error::Invalid)?;
    let reference = execution.tree_checkout.as_ref().ok_or(Error::NotFound)?;
    store.check_scope(scope)?;
    let saved = store
        .lookup(
            record.principal,
            record.receipt.request_id,
            record.receipt.intent_digest,
        )?
        .ok_or(Error::Unavailable)?;
    if saved.reference != *reference {
        return Err(Error::IntentMismatch);
    }
    super::tree_checkout::linkage(record, &saved.plan)?;
    Ok(saved)
}
fn validate_inventory(record: &Record, inventory: &PhysicalInventoryV1) -> Result<()> {
    let execution = record.execution.as_ref().ok_or(Error::Invalid)?;
    inventory
        .check_selection(
            &execution.witness.binding(),
            record.preview.plan.request.selection.working_tree,
        )
        .map_err(|_| Error::IntentMismatch)
}
impl StackCommandJournal {
    /// Journal authentication precedes catalog access. Synced body precedes the
    /// original reference under the existing journal-then-plan lock order.
    pub fn save_local_physical_inventory(
        &self,
        store: &StackCheckoutPlans,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
        inventory: &PhysicalInventoryV1,
    ) -> Result<Selection> {
        let gate = self.root.gate()?;
        let (mut catalog, stamp) = self.read(&gate)?;
        let record = catalog
            .lookup(principal, apply.request_id, apply.intent_digest)?
            .ok_or(Error::NotFound)?;
        apply
            .validate(&record.preview.plan.request, record.preview.prepared)
            .map_err(|_| Error::IntentMismatch)?;
        let execution = record.execution.as_ref().ok_or(Error::Invalid)?;
        if execution.physical_inventory.is_none()
            && record.receipt.state != s::ReceiptState::Prepared
        {
            return Err(Error::InvalidTransition);
        }
        validate_inventory(record, inventory)?;
        let saved = original_plan(record, store, &self.scope)?;
        if let Some(reference) = &execution.physical_inventory {
            let retained = saved.inventory.ok_or(Error::Unavailable)?;
            if retained.reference != *reference || retained.inventory != *inventory {
                return Err(Error::IntentMismatch);
            }
            self.root.sync_selected(&gate, stamp)?;
            return Ok(Selection::from_record(record, true));
        }
        let reference = store.select_inventory(principal, &saved.plan, inventory)?;
        let index = catalog
            .records
            .iter()
            .position(|r| r.receipt.request_id == apply.request_id)
            .ok_or(Error::NotFound)?;
        catalog.records[index]
            .execution
            .as_mut()
            .ok_or(Error::Invalid)?
            .physical_inventory = Some(reference);
        catalog.include_execution();
        let selected = Selection::from_record(&catalog.records[index], false);
        self.root
            .persist(&gate, &catalog.encode_for_admission()?, Some(stamp))?;
        Ok(selected)
    }
    /// Resolves only exact saved original input; no fresh scan or native writes.
    pub fn load_local_physical_inventory(
        &self,
        store: &StackCheckoutPlans,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
    ) -> Result<PhysicalInventoryV1> {
        let gate = self.root.gate()?;
        let (catalog, stamp) = self.read(&gate)?;
        let record = catalog
            .lookup(principal, apply.request_id, apply.intent_digest)?
            .ok_or(Error::NotFound)?;
        apply
            .validate(&record.preview.plan.request, record.preview.prepared)
            .map_err(|_| Error::IntentMismatch)?;
        let reference = record
            .execution
            .as_ref()
            .and_then(|e| e.physical_inventory.as_ref())
            .ok_or(Error::NotFound)?;
        let saved = original_plan(record, store, &self.scope)?
            .inventory
            .ok_or(Error::Unavailable)?;
        if saved.reference != *reference {
            return Err(Error::IntentMismatch);
        }
        validate_inventory(record, &saved.inventory)?;
        self.root.sync_selected(&gate, stamp)?;
        Ok(saved.inventory)
    }
}
