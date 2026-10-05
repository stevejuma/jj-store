use super::*;
use std::collections::BTreeSet;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Catalog {
    version: u32,
    scope: journal::Scope,
    pub plans: Vec<SavedTreePlanV1>,
}
impl Catalog {
    pub fn new(scope: journal::Scope) -> Self {
        Self {
            version: 1,
            scope,
            plans: vec![],
        }
    }
    fn validate(&self) -> Result<()> {
        self.scope.validate()?;
        let version = if self.plans.iter().any(|p| p.inventory.is_some()) {
            2
        } else {
            1
        };
        if self.version != version || self.plans.len() > MAX_RECORDS {
            return Err(Error::Invalid);
        }
        let mut requests = BTreeSet::new();
        for saved in &self.plans {
            saved.validate(&self.scope)?;
            if !requests.insert(saved.plan.preparation().request) {
                return Err(Error::Invalid);
            }
        }
        Ok(())
    }
    pub fn include_inventory(&mut self) {
        self.version = 2;
    }
    pub fn decode(bytes: &[u8], scope: &journal::Scope) -> Result<Self> {
        if bytes.len() > MAX_BYTES {
            return Err(Error::Unavailable);
        }
        let catalog: Self = serde_json::from_slice(bytes).map_err(|_| Error::Unavailable)?;
        if catalog.scope != *scope {
            return Err(Error::ScopeMismatch);
        }
        catalog.validate().map_err(|_| Error::Unavailable)?;
        // Plan storage admits exactly the original canonical representation.
        if serde_json::to_vec(&catalog).map_err(|_| Error::Unavailable)? != bytes {
            return Err(Error::Unavailable);
        }
        Ok(catalog)
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(|_| Error::Invalid)?;
        if bytes.len() > MAX_BYTES {
            return Err(Error::Capacity);
        }
        Ok(bytes)
    }
    pub fn lookup(
        &self,
        principal: m::PrincipalId,
        request: m::RequestId,
        intent: m::IntentDigest,
    ) -> Result<Option<&SavedTreePlanV1>> {
        if principal.as_bytes() == &[0; 16] || request.as_bytes() == &[0; 16] {
            return Err(Error::Invalid);
        }
        let saved = self
            .plans
            .iter()
            .find(|s| s.plan.preparation().request == request);
        if saved.is_some_and(|s| {
            let key = s.plan.preparation();
            key.principal != principal || key.intent != intent
        }) {
            return Err(Error::IntentMismatch);
        }
        Ok(saved)
    }
}
