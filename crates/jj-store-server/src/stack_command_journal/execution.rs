use super::*;
use jj_store_jj::local_evidence::OperationArchiveV1;
use jj_store_jj::{
    local_execution::RecoveryWitnessV1,
    local_metadata::{GitOid, StackPreparationKey, StackRetention},
};

/// Installation-private linkage, never included in a client preview or receipt.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalExecutionV1 {
    pub version: u32,
    pub witness_id: m::RecordDigest,
    pub witness: RecoveryWitnessV1,
    pub key: StackPreparationKey,
    pub retention: Option<GitOid>,
    pub archive: m::RecordDigest,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history: Option<OperationArchiveV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout: Option<super::checkout::SavedCheckoutV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree_checkout: Option<crate::stack_checkout_plans::TreeCheckoutReferenceV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_inventory: Option<crate::stack_checkout_plans::PhysicalInventoryReferenceV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub combination: Option<crate::stack_combinations::CombinationBindingV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion: Option<super::combination::NativeCompletionV1>,
}
pub fn preparation_key(principal: m::PrincipalId, preview: &s::Preview) -> StackPreparationKey {
    let request = &preview.plan.request;
    StackPreparationKey {
        repository: request.selection.repository,
        physical_workspace: request.selection.physical_workspace.0,
        principal,
        request: request.request_id,
        intent: request.intent_digest,
        prepared: preview.prepared.0,
    }
}
impl LocalExecutionV1 {
    pub fn pending(
        principal: m::PrincipalId,
        preview: &s::Preview,
        witness: RecoveryWitnessV1,
        history: OperationArchiveV1,
    ) -> Result<Self> {
        let capsule = history.capsule().map_err(|_| Error::Invalid)?;
        if capsule.repository != preview.plan.request.selection.repository
            || capsule.operation_id != preview.plan.operation
            || capsule.view_id != preview.plan.view
        {
            return Err(Error::IntentMismatch);
        }
        let value = Self {
            version: 1,
            witness_id: witness.id().map_err(|_| Error::Invalid)?,
            witness,
            key: preparation_key(principal, preview),
            retention: None,
            archive: history.id().map_err(|_| Error::Invalid)?,
            history: Some(history),
            checkout: None,
            tree_checkout: None,
            physical_inventory: None,
            combination: None,
            completion: None,
        };
        value.validate(principal, preview)?;
        Ok(value)
    }
    /// Caller supplies an acknowledged provider retention, never inbound evidence.
    pub fn from_retention(
        principal: m::PrincipalId,
        preview: &s::Preview,
        witness: RecoveryWitnessV1,
        retained: &StackRetention,
    ) -> Result<Self> {
        let history = retained.history.as_ref().ok_or(Error::Invalid)?;
        if retained.key != preparation_key(principal, preview)
            || history.capsule().map_err(|_| Error::Invalid)? != retained.capsule
            || retained.capsule.operation_id != preview.plan.operation
            || retained.capsule.view_id != preview.plan.view
        {
            return Err(Error::IntentMismatch);
        }
        let value = Self {
            version: 1,
            witness_id: witness.id().map_err(|_| Error::Invalid)?,
            witness,
            key: retained.key.clone(),
            retention: Some(retained.commit),
            archive: history.id().map_err(|_| Error::Invalid)?,
            history: None,
            checkout: None,
            tree_checkout: None,
            physical_inventory: None,
            combination: None,
            completion: None,
        };
        value.validate(principal, preview)?;
        Ok(value)
    }
    pub(super) fn validate(&self, principal: m::PrincipalId, preview: &s::Preview) -> Result<()> {
        if let Some(binding) = &self.combination {
            binding.validate().map_err(|_| Error::Invalid)?;
            if self.retention.is_none() || binding.selection() != &preview.plan.combination {
                return Err(Error::IntentMismatch);
            }
        }
        if let Some(completion) = &self.completion {
            completion.validate(self)?;
        }
        if let Some(inventory) = &self.physical_inventory {
            inventory.validate()?;
            if !self
                .tree_checkout
                .as_ref()
                .is_some_and(|p| p.id == inventory.plan)
            {
                return Err(Error::Invalid);
            }
        }
        if let Some(reference) = &self.tree_checkout {
            reference.validate()?;
            if self.checkout.is_some()
                || self.retention.is_none()
                || preview.plan.checkout.before == preview.plan.checkout.after
            {
                return Err(Error::Invalid);
            }
        }
        if let Some(checkout) = &self.checkout {
            checkout.validate(self, preview)?;
        }
        match (&self.history, self.retention) {
            (Some(history), None) if history.id().map_err(|_| Error::Invalid)? == self.archive => {
                let capsule = history.capsule().map_err(|_| Error::Invalid)?;
                if capsule.operation_id != preview.plan.operation
                    || capsule.view_id != preview.plan.view
                    || capsule.repository != self.key.repository
                {
                    return Err(Error::IntentMismatch);
                }
            }
            (None, Some(_)) => {}
            _ => return Err(Error::Invalid),
        }
        let selection = &preview.plan.request.selection;
        let binding = self.witness.binding();
        let s::CodeSelection::GitSha1Jj045 {
            operation,
            view,
            working_copy,
        } = &selection.code
        else {
            return Err(Error::Invalid);
        };
        if self.version != 1
            || self.witness.id().map_err(|_| Error::Invalid)? != self.witness_id
            || self.key != preparation_key(principal, preview)
            || self.archive == m::RecordDigest::new([0; 32])
            || binding.repository != selection.repository
            || binding.generation != selection.binding_generation
            || binding.physical_workspace != selection.physical_workspace.0
            || binding.workspace != selection.jj_workspace
            || binding.operation != *operation
            || binding.view != *view
            || binding.commit != *working_copy
        {
            return Err(Error::IntentMismatch);
        }
        Ok(())
    }
}
impl StackCommandJournal {
    /// Only provider-acknowledged exact original retention may complete pending
    /// evidence; legacy or historical receipt state never gets reset.
    pub fn acknowledge_local_retention(
        &self,
        principal: m::PrincipalId,
        preview: &s::Preview,
        retained: &StackRetention,
    ) -> Result<Selection> {
        let gate = self.root.gate()?;
        let (mut catalog, stamp) = self.read(&gate)?;
        let request = &preview.plan.request;
        let record = catalog
            .lookup(principal, request.request_id, request.intent_digest)?
            .ok_or(Error::NotFound)?;
        if record.preview != *preview {
            return Err(Error::IntentMismatch);
        }
        let pending = record.execution.as_ref().ok_or(Error::Invalid)?;
        let mut completed = LocalExecutionV1::from_retention(
            principal,
            preview,
            pending.witness.clone(),
            retained,
        )?;
        if completed.archive != pending.archive {
            return Err(Error::IntentMismatch);
        }
        if pending.retention.is_some() {
            completed.checkout = pending.checkout.clone();
            completed.tree_checkout = pending.tree_checkout.clone();
            completed.physical_inventory = pending.physical_inventory.clone();
            completed.combination = pending.combination.clone();
            completed.completion = pending.completion.clone();
            if *pending != completed {
                return Err(Error::IntentMismatch);
            }
            self.root.sync_selected(&gate, stamp)?;
            return Ok(Selection::from_record(record, true));
        }
        if record.receipt.state != s::ReceiptState::Prepared {
            return Err(Error::InvalidTransition);
        }
        let index = catalog
            .records
            .iter()
            .position(|r| r.receipt.request_id == request.request_id)
            .ok_or(Error::NotFound)?;
        catalog.records[index].execution = Some(completed);
        let selected = Selection::from_record(&catalog.records[index], false);
        self.root.persist(&gate, &catalog.encode()?, Some(stamp))?;
        Ok(selected)
    }
}
