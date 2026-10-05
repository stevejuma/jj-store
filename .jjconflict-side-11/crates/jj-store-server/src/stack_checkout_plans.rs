//! Durable private inputs, never publication or physical-completion authority.
use crate::{capture_journal_files as files, stack_command_journal as journal};
use jj_store_jj::local_execution::TreeCheckoutPlanV1;
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use std::path::Path;
mod catalog;
mod inventory;
use catalog::Catalog;
pub use inventory::{PhysicalInventoryReferenceV1, SavedPhysicalInventoryV1};
pub use journal::{Error, Result};
pub const MAX_RECORDS: usize = 128;
pub const MAX_BYTES: usize = 64 * 1024 * 1024;
pub const MAX_PLAN_BYTES: usize = 32 * 1024 * 1024;
const LAYOUT: files::Layout = files::Layout {
    lock: "checkout-plans.lock",
    temporary: ".stack-checkout-plan-",
    max_bytes: MAX_BYTES,
};
/// A reference carries no path, body, permission or execution proof.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TreeCheckoutReferenceV1 {
    pub id: m::RecordDigest,
    pub encoded_bytes: u32,
}
impl TreeCheckoutReferenceV1 {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.id == m::RecordDigest::new([0; 32])
            || self.encoded_bytes == 0
            || self.encoded_bytes as usize > MAX_PLAN_BYTES
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    pub(crate) fn from_plan(plan: &TreeCheckoutPlanV1) -> Result<Self> {
        Ok(Self {
            id: plan.id().map_err(|_| Error::Invalid)?,
            encoded_bytes: plan
                .encode()
                .map_err(|_| Error::Invalid)?
                .len()
                .try_into()
                .map_err(|_| Error::Capacity)?,
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedTreePlanV1 {
    pub reference: TreeCheckoutReferenceV1,
    pub plan: TreeCheckoutPlanV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory: Option<SavedPhysicalInventoryV1>,
}
impl SavedTreePlanV1 {
    fn validate(&self, scope: &journal::Scope) -> Result<()> {
        if let Some(inventory) = &self.inventory {
            inventory.validate(&self.plan)?;
        }
        self.reference.validate()?;
        let key = self.plan.preparation();
        if scope.profile != journal::Profile::GitSha1Jj045
            || key.repository != scope.repository
            || key.physical_workspace != scope.physical_workspace.0
            || self.plan.workspace().map_err(|_| Error::Invalid)? != scope.jj_workspace
            || key.principal.as_bytes() == &[0; 16]
            || key.request.as_bytes() == &[0; 16]
            || key.intent.as_bytes() == &[0; 32]
            || key.prepared.as_bytes() == &[0; 32]
            || self.reference != TreeCheckoutReferenceV1::from_plan(&self.plan)?
        {
            return Err(Error::IntentMismatch);
        }
        Ok(())
    }
}
#[derive(Clone)]
pub struct StackCheckoutPlans {
    root: files::Root,
    scope: journal::Scope,
}
impl StackCheckoutPlans {
    pub(crate) fn check_scope(&self, scope: &journal::Scope) -> Result<()> {
        if self.scope != *scope {
            Err(Error::ScopeMismatch)
        } else {
            Ok(())
        }
    }
    pub fn create(path: &Path, scope: journal::Scope) -> Result<Self> {
        scope.validate()?;
        let root = files::Root::create(path, LAYOUT)?;
        let gate = root.gate()?;
        root.persist(&gate, &Catalog::new(scope.clone()).encode()?, None)?;
        drop(gate);
        Ok(Self { root, scope })
    }
    /// Read-only open; a missing or invalid store is never initialized.
    pub fn open(path: &Path, scope: journal::Scope) -> Result<Self> {
        scope.validate()?;
        let store = Self {
            root: files::Root::open(path, LAYOUT)?,
            scope,
        };
        let gate = store.root.gate()?;
        store.read(&gate)?;
        Ok(store)
    }
    fn read(&self, gate: &files::Gate) -> Result<(Catalog, files::Stamp)> {
        let (bytes, stamp) = self.root.read(gate)?;
        Ok((Catalog::decode(&bytes, &self.scope)?, stamp))
    }
    /// Trusted original input only. Duplicate request selection syncs the
    /// retained bytes; an orphan plan never permits execution on its own.
    pub fn select(
        &self,
        principal: m::PrincipalId,
        plan: &TreeCheckoutPlanV1,
    ) -> Result<TreeCheckoutReferenceV1> {
        let saved = SavedTreePlanV1 {
            reference: TreeCheckoutReferenceV1::from_plan(plan)?,
            plan: plan.clone(),
            inventory: None,
        };
        saved.validate(&self.scope)?;
        let key = plan.preparation();
        if principal != key.principal {
            return Err(Error::IntentMismatch);
        }
        let gate = self.root.gate()?;
        let (mut catalog, stamp) = self.read(&gate)?;
        if let Some(original) = catalog.lookup(principal, key.request, key.intent)? {
            if original.reference != saved.reference || original.plan != saved.plan {
                return Err(Error::IntentMismatch);
            }
            self.root.sync_selected(&gate, stamp)?;
            return Ok(original.reference.clone());
        }
        if catalog.plans.len() == MAX_RECORDS {
            return Err(Error::Capacity);
        }
        catalog.plans.push(saved.clone());
        self.root.persist(&gate, &catalog.encode()?, Some(stamp))?;
        Ok(saved.reference)
    }
    pub fn lookup(
        &self,
        principal: m::PrincipalId,
        request: m::RequestId,
        intent: m::IntentDigest,
    ) -> Result<Option<SavedTreePlanV1>> {
        let gate = self.root.gate()?;
        let (catalog, stamp) = self.read(&gate)?;
        let original = catalog.lookup(principal, request, intent)?.cloned();
        if original.is_some() {
            self.root.sync_selected(&gate, stamp)?;
        }
        Ok(original)
    }
}
#[cfg(all(test, unix))]
mod tests;
