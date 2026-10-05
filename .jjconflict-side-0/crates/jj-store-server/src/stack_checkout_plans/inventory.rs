use super::*;
use jj_store_jj::local_execution::{MAX_INVENTORY_BYTES, PhysicalInventoryV1};
/// Original input reference, never provider installation or completion evidence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalInventoryReferenceV1 {
    pub id: m::RecordDigest,
    pub encoded_bytes: u32,
    pub plan: m::RecordDigest,
}
impl PhysicalInventoryReferenceV1 {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.id == m::RecordDigest::new([0; 32])
            || self.plan == m::RecordDigest::new([0; 32])
            || self.encoded_bytes == 0
            || self.encoded_bytes as usize > MAX_INVENTORY_BYTES
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    pub(super) fn from_inventory(
        plan: &TreeCheckoutPlanV1,
        inventory: &PhysicalInventoryV1,
    ) -> Result<Self> {
        Ok(Self {
            id: inventory.id().map_err(|_| Error::Invalid)?,
            encoded_bytes: inventory
                .encode()
                .map_err(|_| Error::Invalid)?
                .len()
                .try_into()
                .map_err(|_| Error::Capacity)?,
            plan: plan.id().map_err(|_| Error::Invalid)?,
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedPhysicalInventoryV1 {
    pub reference: PhysicalInventoryReferenceV1,
    pub inventory: PhysicalInventoryV1,
}
impl SavedPhysicalInventoryV1 {
    pub(super) fn validate(&self, plan: &TreeCheckoutPlanV1) -> Result<()> {
        self.reference.validate()?;
        if self.inventory.fingerprint() != plan.physical()
            || self.reference
                != PhysicalInventoryReferenceV1::from_inventory(plan, &self.inventory)?
        {
            return Err(Error::IntentMismatch);
        }
        Ok(())
    }
}
impl StackCheckoutPlans {
    /// Attaches only immutable original input to an already selected exact plan.
    /// Caller must authenticate and validate the original recovery binding first.
    pub fn select_inventory(
        &self,
        principal: m::PrincipalId,
        plan: &TreeCheckoutPlanV1,
        inventory: &PhysicalInventoryV1,
    ) -> Result<PhysicalInventoryReferenceV1> {
        let saved = SavedPhysicalInventoryV1 {
            reference: PhysicalInventoryReferenceV1::from_inventory(plan, inventory)?,
            inventory: inventory.clone(),
        };
        saved.validate(plan)?;
        if principal != plan.preparation().principal {
            return Err(Error::IntentMismatch);
        }
        let gate = self.root.gate()?;
        let (mut catalog, stamp) = self.read(&gate)?;
        let key = plan.preparation();
        let original = catalog
            .lookup(principal, key.request, key.intent)?
            .ok_or(Error::NotFound)?;
        if original.plan != *plan {
            return Err(Error::IntentMismatch);
        }
        if let Some(retained) = &original.inventory {
            if *retained != saved {
                return Err(Error::IntentMismatch);
            }
            self.root.sync_selected(&gate, stamp)?;
            return Ok(retained.reference.clone());
        }
        let index = catalog
            .plans
            .iter()
            .position(|p| p.plan.preparation().request == key.request)
            .ok_or(Error::NotFound)?;
        catalog.plans[index].inventory = Some(saved.clone());
        catalog.include_inventory();
        self.root.persist(&gate, &catalog.encode()?, Some(stamp))?;
        Ok(saved.reference)
    }
}
