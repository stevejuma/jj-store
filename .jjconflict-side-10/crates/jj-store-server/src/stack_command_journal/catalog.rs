use super::*;
use std::collections::BTreeSet;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Catalog {
    version: u32,
    scope: Scope,
    pub records: Vec<Record>,
}
impl Catalog {
    pub fn new(scope: Scope) -> Self {
        Self {
            version: 1,
            scope,
            records: vec![],
        }
    }
    pub fn decode(bytes: &[u8], scope: &Scope) -> Result<Self> {
        let catalog: Self = serde_json::from_slice(bytes).map_err(|_| Error::Unavailable)?;
        if catalog.scope != *scope {
            return Err(Error::ScopeMismatch);
        }
        catalog.validate().map_err(|_| Error::Unavailable)?;
        Ok(catalog)
    }
    fn validate(&self) -> Result<()> {
        self.scope.validate()?;
        if self.version != 1 || self.records.len() > MAX_RECORDS {
            return Err(Error::Invalid);
        }
        let mut requests = BTreeSet::new();
        let mut unresolved = 0;
        for record in &self.records {
            record.validate(&self.scope)?;
            if !requests.insert(record.receipt.request_id) {
                return Err(Error::Invalid);
            }
            if super::transition::unresolved(&record.receipt.state) {
                unresolved += 1;
            }
        }
        if unresolved > 1 {
            return Err(Error::Invalid);
        }
        Ok(())
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
        digest: m::IntentDigest,
    ) -> Result<Option<&Record>> {
        if principal.as_bytes() == &[0; 16] || request.as_bytes() == &[0; 16] {
            return Err(Error::Invalid);
        }
        let record = self
            .records
            .iter()
            .find(|r| r.receipt.request_id == request);
        if record.is_some_and(|r| r.principal != principal || r.receipt.intent_digest != digest) {
            return Err(Error::IntentMismatch);
        }
        Ok(record)
    }
    pub fn encode_for_admission(&self) -> Result<Vec<u8>> {
        let bytes = self.encode()?;
        // Reasons admit at most 4096 UTF-8 bytes without control characters,
        // which JSON can at most double. IDs and tags add less than 1 KiB.
        // Reserve 16 KiB per record for every later receipt state so capacity
        // pressure cannot prevent publication/checkout recovery or retirement.
        if bytes.len() + self.records.len() * 16 * 1024 > MAX_BYTES {
            return Err(Error::Capacity);
        }
        Ok(bytes)
    }
}
