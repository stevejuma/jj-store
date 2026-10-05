use super::*;
use jj_store_jj::local_execution::{CheckoutWitnessV1, PublishedHead, VerifiedUnchangedCheckout};
const MAX_WITNESS_BYTES: usize = 8 * 1024;

/// Installation-private recovery input; never included in a wire receipt.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedCheckoutV1 {
    pub id: m::RecordDigest,
    pub witness: CheckoutWitnessV1,
}
impl SavedCheckoutV1 {
    pub(super) fn validate(
        &self,
        execution: &LocalExecutionV1,
        preview: &s::Preview,
    ) -> Result<()> {
        if execution.retention.is_none()
            || self.witness.encode().map_err(|_| Error::Invalid)?.len() > MAX_WITNESS_BYTES
            || self.witness.id().map_err(|_| Error::Invalid)? != self.id
            || self.witness.preparation() != &execution.key
            || self.witness.recovery() != execution.witness_id
            || self.witness.candidate() != (preview.plan.operation, preview.plan.view)
            || self.witness.physical() != preview.plan.request.selection.working_tree
            || preview.plan.checkout.before != preview.plan.checkout.after
            || !preview.plan.checkout.paths.is_empty()
            || !matches!(preview.plan.request.command, s::Command::Reword { .. })
        {
            return Err(Error::IntentMismatch);
        }
        Ok(())
    }
}
impl StackCommandJournal {
    pub fn acknowledge_local_publication(
        &self,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
        proof: &PublishedHead,
    ) -> Result<Selection> {
        let record = self
            .status(principal, apply.request_id, apply.intent_digest)?
            .ok_or(Error::NotFound)?;
        let execution = record.execution.as_ref().ok_or(Error::Invalid)?;
        if execution.retention.is_none()
            || proof.preparation() != &execution.key
            || proof.witness() != execution.witness_id
            || proof.operation != record.preview.plan.operation
            || proof.view != record.preview.plan.view
        {
            return Err(Error::IntentMismatch);
        }
        self.select_transition(
            principal,
            apply,
            &s::ReceiptState::PublicationUnknown,
            &s::ReceiptState::Published {
                operation: proof.operation,
                view: proof.view,
                checkout: s::CheckoutState::Required,
            },
            true,
        )
    }
    /// Saving exact input is durable before the caller may finish the pointer.
    pub fn save_local_checkout(
        &self,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
        witness: &CheckoutWitnessV1,
    ) -> Result<Selection> {
        let saved = SavedCheckoutV1 {
            id: witness.id().map_err(|_| Error::Invalid)?,
            witness: witness.clone(),
        };
        let gate = self.root.gate()?;
        let (mut catalog, stamp) = self.read(&gate)?;
        let record = catalog
            .lookup(principal, apply.request_id, apply.intent_digest)?
            .ok_or(Error::NotFound)?;
        apply
            .validate(&record.preview.plan.request, record.preview.prepared)
            .map_err(|_| Error::IntentMismatch)?;
        let execution = record.execution.as_ref().ok_or(Error::Invalid)?;
        saved.validate(execution, &record.preview)?;
        if let Some(original) = &execution.checkout {
            if *original != saved {
                return Err(Error::IntentMismatch);
            }
            self.root.sync_selected(&gate, stamp)?;
            return Ok(Selection::from_record(record, true));
        }
        if !matches!(
            record.receipt.state,
            s::ReceiptState::Published {
                checkout: s::CheckoutState::Required | s::CheckoutState::Blocked { .. },
                ..
            }
        ) {
            return Err(Error::InvalidTransition);
        }
        let index = catalog
            .records
            .iter()
            .position(|r| r.receipt.request_id == apply.request_id)
            .ok_or(Error::NotFound)?;
        catalog.records[index]
            .execution
            .as_mut()
            .ok_or(Error::Invalid)?
            .checkout = Some(saved);
        catalog.include_execution();
        let selected = Selection::from_record(&catalog.records[index], false);
        self.root.persist(&gate, &catalog.encode()?, Some(stamp))?;
        Ok(selected)
    }
    pub fn acknowledge_local_checkout(
        &self,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
        proof: &VerifiedUnchangedCheckout,
    ) -> Result<Selection> {
        let record = self
            .status(principal, apply.request_id, apply.intent_digest)?
            .ok_or(Error::NotFound)?;
        let execution = record.execution.as_ref().ok_or(Error::Invalid)?;
        let saved = execution.checkout.as_ref().ok_or(Error::Invalid)?;
        if proof.preparation() != &execution.key
            || proof.witness() != saved.id
            || proof.physical() != saved.witness.physical()
            || proof.operation != record.preview.plan.operation
            || proof.view != record.preview.plan.view
        {
            return Err(Error::IntentMismatch);
        }
        self.select_transition(
            principal,
            apply,
            &record.receipt.state,
            &s::ReceiptState::Published {
                operation: proof.operation,
                view: proof.view,
                checkout: s::CheckoutState::Verified {
                    fingerprint: proof.physical(),
                },
            },
            true,
        )
    }
}
