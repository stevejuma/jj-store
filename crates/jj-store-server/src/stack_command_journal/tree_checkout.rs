use super::*;
use crate::stack_checkout_plans::{StackCheckoutPlans, TreeCheckoutReferenceV1};
use jj_store_jj::local_execution::TreeCheckoutPlanV1;
fn linkage(record: &Record, plan: &TreeCheckoutPlanV1) -> Result<()> {
    let execution = record.execution.as_ref().ok_or(Error::Invalid)?;
    let preview = &record.preview;
    let (before, after) = plan.trees();
    if execution.checkout.is_some()
        || plan.preparation() != &execution.key
        || plan.recovery() != execution.witness_id
        || Some(plan.retained().0) != execution.retention
        || plan.retained().1 != execution.archive
        || plan.candidate() != (preview.plan.operation, preview.plan.view)
        || plan.physical() != preview.plan.request.selection.working_tree
        || plan.commits().0 != execution.witness.binding().commit
        || plan.workspace().map_err(|_| Error::Invalid)?
            != preview.plan.request.selection.jj_workspace
        || preview.plan.checkout.before != s::TreeIdentity::GitSha1(before).into()
        || preview.plan.checkout.after != s::TreeIdentity::GitSha1(after).into()
        || !plan
            .paths()
            .eq(preview.plan.checkout.paths.iter().map(String::as_str))
    {
        return Err(Error::IntentMismatch);
    }
    plan.encode().map_err(|_| Error::Invalid)?;
    Ok(())
}
impl StackCommandJournal {
    /// Original request is checked before plan-store access. Lock order is
    /// journal then plan catalog. Synced plan selection precedes its reference.
    pub fn save_local_tree_checkout(
        &self,
        store: &StackCheckoutPlans,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
        plan: &TreeCheckoutPlanV1,
    ) -> Result<Selection> {
        let gate = self.root.gate()?;
        let (mut catalog, stamp) = self.read(&gate)?;
        let record = catalog
            .lookup(principal, apply.request_id, apply.intent_digest)?
            .ok_or(Error::NotFound)?;
        apply
            .validate(&record.preview.plan.request, record.preview.prepared)
            .map_err(|_| Error::IntentMismatch)?;
        linkage(record, plan)?;
        let execution = record.execution.as_ref().ok_or(Error::Invalid)?;
        // Exact historical replay may sync an existing reference. It cannot
        // acquire a new one after Prepared, even when the body is orphaned.
        if execution.tree_checkout.is_none() && record.receipt.state != s::ReceiptState::Prepared {
            return Err(Error::InvalidTransition);
        }
        store.check_scope(&self.scope)?;
        let reference = TreeCheckoutReferenceV1::from_plan(plan)?;
        if let Some(original) = &execution.tree_checkout {
            if *original != reference {
                return Err(Error::IntentMismatch);
            }
            let saved = store
                .lookup(principal, apply.request_id, apply.intent_digest)?
                .ok_or(Error::Unavailable)?;
            if saved.reference != reference || saved.plan != *plan {
                return Err(Error::IntentMismatch);
            }
            self.root.sync_selected(&gate, stamp)?;
            return Ok(Selection::from_record(record, true));
        }
        store.select(principal, plan)?;
        let index = catalog
            .records
            .iter()
            .position(|r| r.receipt.request_id == apply.request_id)
            .ok_or(Error::NotFound)?;
        catalog.records[index]
            .execution
            .as_mut()
            .ok_or(Error::Invalid)?
            .tree_checkout = Some(reference);
        catalog.include_execution();
        let selected = Selection::from_record(&catalog.records[index], false);
        self.root
            .persist(&gate, &catalog.encode_for_admission()?, Some(stamp))?;
        Ok(selected)
    }
    /// Resolves original immutable input only. Caller must still revalidate it
    /// with the provider; this method performs no live read or code execution.
    pub fn load_local_tree_checkout(
        &self,
        store: &StackCheckoutPlans,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
    ) -> Result<TreeCheckoutPlanV1> {
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
            .and_then(|e| e.tree_checkout.as_ref())
            .ok_or(Error::NotFound)?;
        store.check_scope(&self.scope)?;
        let saved = store
            .lookup(principal, apply.request_id, apply.intent_digest)?
            .ok_or(Error::Unavailable)?;
        if saved.reference != *reference {
            return Err(Error::IntentMismatch);
        }
        linkage(record, &saved.plan)?;
        self.root.sync_selected(&gate, stamp)?;
        Ok(saved.plan)
    }
}
