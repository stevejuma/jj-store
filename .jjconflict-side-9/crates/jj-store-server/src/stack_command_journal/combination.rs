use super::*;
use crate::stack_combinations::{CombinationStore, Snapshot};
use jj_store_jj::{
    local_execution::{VerifiedTreeCheckout, VerifiedUnchangedCheckout},
    local_metadata::StackRetention,
};

/// Provider-verified native completion retained without public acknowledgement.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeCompletionV1 {
    version: u32,
    physical: m::RecordDigest,
    proof: CompletionKind,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum CompletionKind {
    Unchanged {
        witness: m::RecordDigest,
    },
    Tree {
        plan: m::RecordDigest,
        inventory: m::RecordDigest,
        frame: m::RecordDigest,
    },
}
impl NativeCompletionV1 {
    pub fn physical(&self) -> m::RecordDigest {
        self.physical
    }
    pub(super) fn validate(&self, execution: &LocalExecutionV1) -> Result<()> {
        if self.version != 1
            || self.physical.as_bytes() == &[0; 32]
            || execution.retention.is_none()
            || execution.combination.is_none()
        {
            return Err(Error::Invalid);
        }
        match &self.proof {
            CompletionKind::Unchanged { witness }
                if execution.checkout.as_ref().is_some_and(|saved| {
                    saved.id == *witness && saved.witness.physical() == self.physical
                }) => {}
            CompletionKind::Tree {
                plan,
                inventory,
                frame,
            } if frame.as_bytes() != &[0; 32]
                && execution
                    .tree_checkout
                    .as_ref()
                    .is_some_and(|saved| saved.id == *plan)
                && execution
                    .physical_inventory
                    .as_ref()
                    .is_some_and(|saved| saved.id == *inventory && saved.plan == *plan) => {}
            _ => return Err(Error::IntentMismatch),
        }
        Ok(())
    }
    fn tree(record: &Record, proof: &VerifiedTreeCheckout) -> Result<Self> {
        let execution = record.execution.as_ref().ok_or(Error::Invalid)?;
        check_proof(record, proof.preparation(), proof.operation, proof.view)?;
        let value = Self {
            version: 1,
            physical: proof.physical(),
            proof: CompletionKind::Tree {
                plan: proof.plan(),
                inventory: proof.inventory(),
                frame: proof.completion(),
            },
        };
        value.validate(execution)?;
        Ok(value)
    }
    fn unchanged(record: &Record, proof: &VerifiedUnchangedCheckout) -> Result<Self> {
        let execution = record.execution.as_ref().ok_or(Error::Invalid)?;
        check_proof(record, proof.preparation(), proof.operation, proof.view)?;
        let value = Self {
            version: 1,
            physical: proof.physical(),
            proof: CompletionKind::Unchanged {
                witness: proof.witness(),
            },
        };
        value.validate(execution)?;
        Ok(value)
    }
}
fn check_proof(
    record: &Record,
    key: &jj_store_jj::local_metadata::StackPreparationKey,
    operation: m::OperationId,
    view: m::ViewId,
) -> Result<()> {
    let execution = record.execution.as_ref().ok_or(Error::Invalid)?;
    if key != &execution.key
        || operation != record.preview.plan.operation
        || view != record.preview.plan.view
    {
        return Err(Error::IntentMismatch);
    }
    Ok(())
}
fn pending(record: &Record) -> Result<()> {
    if !matches!(
        record.receipt.state,
        s::ReceiptState::Published {
            checkout: s::CheckoutState::Required | s::CheckoutState::Blocked { .. },
            ..
        }
    ) {
        return Err(Error::InvalidTransition);
    }
    Ok(())
}
fn original<'a>(
    catalog: &'a Catalog,
    principal: m::PrincipalId,
    apply: &s::ApplyRequest,
) -> Result<&'a Record> {
    let record = catalog
        .lookup(principal, apply.request_id, apply.intent_digest)?
        .ok_or(Error::NotFound)?;
    apply
        .validate(&record.preview.plan.request, record.preview.prepared)
        .map_err(|_| Error::IntentMismatch)?;
    Ok(record)
}
fn combination_error(error: crate::stack_combinations::Error) -> Error {
    use crate::stack_combinations::Error as C;
    match error {
        C::Invalid => Error::Invalid,
        C::Forbidden => Error::ScopeMismatch,
        C::IntentMismatch => Error::IntentMismatch,
        C::Stale => Error::StaleState,
        C::Pending => Error::Pending,
        C::Capacity => Error::Capacity,
        C::Unavailable => Error::Unavailable,
        C::OutcomeUnknown => Error::OutcomeUnknown,
    }
}
impl StackCommandJournal {
    /// Provider-derived after-state only. Journal then combination coordination;
    /// staged state sync precedes the exact binding, both before publication.
    pub fn save_local_combination(
        &self,
        store: &CombinationStore,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
        expected_revision: u64,
        after: &Snapshot,
        retained: &StackRetention,
    ) -> Result<Selection> {
        let gate = self.root.gate()?;
        let (mut catalog, stamp) = self.read(&gate)?;
        let record = original(&catalog, principal, apply)?;
        let execution = record.execution.as_ref().ok_or(Error::Invalid)?;
        if execution.key != retained.key
            || execution.retention != Some(retained.commit)
            || retained.history.as_ref().and_then(|h| h.id().ok()) != Some(execution.archive)
        {
            return Err(Error::IntentMismatch);
        }
        store
            .check_scope(&self.scope, principal)
            .map_err(combination_error)?;
        if let Some(saved) = &execution.combination {
            let binding = store
                .binding_proposal(record, expected_revision, after, retained)
                .map_err(combination_error)?;
            if binding != *saved {
                return Err(Error::IntentMismatch);
            }
            self.root.sync_selected(&gate, stamp)?;
            return Ok(Selection::from_record(record, true));
        }
        if record.receipt.state != s::ReceiptState::Prepared {
            return Err(Error::InvalidTransition);
        }
        // Reserve the full reference before mutating either private catalog.
        let candidate = crate::stack_combinations::CombinationBindingV1::proposal(
            &self.scope,
            principal,
            &record.preview,
            expected_revision,
            after,
            retained,
        )
        .map_err(combination_error)?;
        let index = catalog
            .records
            .iter()
            .position(|r| r.receipt.request_id == apply.request_id)
            .ok_or(Error::NotFound)?;
        let mut admission = catalog.records[index].clone();
        admission
            .execution
            .as_mut()
            .ok_or(Error::Invalid)?
            .combination = Some(candidate.clone());
        let mut reserved = Catalog::new(self.scope.clone());
        reserved.records = catalog.records.clone();
        reserved.records[index] = admission;
        reserved.include_execution();
        reserved.encode_for_admission()?;
        let staged = store
            .stage(
                principal,
                expected_revision,
                &record.preview,
                after,
                retained,
            )
            .map_err(combination_error)?;
        if staged.phase != crate::stack_combinations::Phase::Pending {
            return Err(Error::InvalidTransition);
        }
        let binding = store
            .binding_proposal(record, expected_revision, after, retained)
            .map_err(combination_error)?;
        if binding != candidate {
            return Err(Error::IntentMismatch);
        }
        catalog.records[index]
            .execution
            .as_mut()
            .ok_or(Error::Invalid)?
            .combination = Some(binding);
        catalog.include_execution();
        let selection = Selection::from_record(&catalog.records[index], false);
        self.root
            .persist(&gate, &catalog.encode_for_admission()?, Some(stamp))?;
        Ok(selection)
    }
    /// Validate and sync the original staged binding before provider execution.
    pub fn check_local_combination(
        &self,
        store: &CombinationStore,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
    ) -> Result<()> {
        let gate = self.root.gate()?;
        let (catalog, stamp) = self.read(&gate)?;
        let record = original(&catalog, principal, apply)?;
        let saved = record
            .execution
            .as_ref()
            .and_then(|e| e.combination.as_ref())
            .ok_or(Error::NotFound)?;
        store
            .check_scope(&self.scope, principal)
            .map_err(combination_error)?;
        if store.binding(record).map_err(combination_error)? != *saved {
            return Err(Error::IntentMismatch);
        }
        self.root.sync_selected(&gate, stamp)?;
        Ok(())
    }
    pub fn save_local_tree_completion(
        &self,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
        proof: &VerifiedTreeCheckout,
    ) -> Result<Selection> {
        self.completion(
            principal,
            apply,
            |r| NativeCompletionV1::tree(r, proof),
            None,
        )
    }
    pub fn save_local_unchanged_completion(
        &self,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
        proof: &VerifiedUnchangedCheckout,
    ) -> Result<Selection> {
        self.completion(
            principal,
            apply,
            |r| NativeCompletionV1::unchanged(r, proof),
            None,
        )
    }
    /// Fresh provider verification is mandatory on retry. Saved evidence alone
    /// cannot select the combination; synced combination precedes success.
    pub fn finish_local_tree_combination(
        &self,
        store: &CombinationStore,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
        proof: &VerifiedTreeCheckout,
    ) -> Result<Selection> {
        self.completion(
            principal,
            apply,
            |r| NativeCompletionV1::tree(r, proof),
            Some(store),
        )
    }
    pub fn finish_local_unchanged_combination(
        &self,
        store: &CombinationStore,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
        proof: &VerifiedUnchangedCheckout,
    ) -> Result<Selection> {
        self.completion(
            principal,
            apply,
            |r| NativeCompletionV1::unchanged(r, proof),
            Some(store),
        )
    }
    fn completion(
        &self,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
        proof: impl FnOnce(&Record) -> Result<NativeCompletionV1>,
        store: Option<&CombinationStore>,
    ) -> Result<Selection> {
        let gate = self.root.gate()?;
        let (mut catalog, stamp) = self.read(&gate)?;
        let record = original(&catalog, principal, apply)?;
        let completion = proof(record)?;
        let saved = record
            .execution
            .as_ref()
            .and_then(|e| e.completion.as_ref());
        if saved.is_some_and(|old| *old != completion) {
            return Err(Error::IntentMismatch);
        }
        if record.receipt.completed() {
            if saved.is_none() {
                return Err(Error::InvalidTransition);
            }
            // Historical acknowledgement never needs current combination state.
            self.root.sync_selected(&gate, stamp)?;
            return Ok(Selection::from_record(record, true));
        }
        pending(record)?;
        if let Some(store) = store {
            if saved.is_none() {
                return Err(Error::InvalidTransition);
            }
            store
                .check_scope(&self.scope, principal)
                .map_err(combination_error)?;
            store.select_completed(record).map_err(combination_error)?;
        } else if saved.is_some() {
            self.root.sync_selected(&gate, stamp)?;
            return Ok(Selection::from_record(record, true));
        }
        let index = catalog
            .records
            .iter()
            .position(|r| r.receipt.request_id == apply.request_id)
            .ok_or(Error::NotFound)?;
        let selected = &mut catalog.records[index];
        selected
            .execution
            .as_mut()
            .ok_or(Error::Invalid)?
            .completion = Some(completion.clone());
        if store.is_some() {
            selected.receipt.state = s::ReceiptState::Published {
                operation: selected.preview.plan.operation,
                view: selected.preview.plan.view,
                checkout: s::CheckoutState::Verified {
                    fingerprint: completion.physical,
                },
            };
        }
        let result = Selection::from_record(selected, false);
        catalog.include_execution();
        self.root.persist(&gate, &catalog.encode()?, Some(stamp))?;
        Ok(result)
    }
}

#[cfg(all(test, unix))]
mod tests;
