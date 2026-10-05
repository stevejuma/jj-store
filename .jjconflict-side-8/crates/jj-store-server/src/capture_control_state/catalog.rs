use super::*;
use std::collections::BTreeSet;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Catalog {
    version: u32,
    installation: String,
    generation: String,
    pub revision: u64,
    pub paused: bool,
    pub receipts: Vec<CaptureControlReceipt>,
}
impl Catalog {
    pub fn new(installation: [u8; 16], generation: [u8; 16]) -> Self {
        Self {
            version: 1,
            installation: hex::encode(installation),
            generation: hex::encode(generation),
            revision: 0,
            paused: false,
            receipts: vec![],
        }
    }
    pub fn decode(bytes: &[u8], installation: [u8; 16]) -> Result<Self> {
        let catalog: Self = serde_json::from_slice(bytes).map_err(|_| Error::Unavailable)?;
        if catalog.version != 1
            || catalog.installation != hex::encode(installation)
            || catalog.receipts.len() > MAX_RECEIPTS
            || catalog.revision != catalog.receipts.len() as u64
            || catalog.paused != catalog.receipts.last().is_some_and(|r| r.paused)
        {
            return Err(Error::Unavailable);
        }
        catalog.snapshot()?;
        let mut requests = BTreeSet::new();
        for (index, r) in catalog.receipts.iter().enumerate() {
            if r.expected_revision != index as u64
                || r.revision != index as u64 + 1
                || r.principal.as_bytes() == &[0; 16]
                || r.request.as_bytes() == &[0; 16]
                || !requests.insert(r.request)
            {
                return Err(Error::Unavailable);
            }
        }
        Ok(catalog)
    }
    pub fn snapshot(&self) -> Result<CaptureControlSnapshot> {
        let generation: [u8; 16] = hex::decode(&self.generation)
            .map_err(|_| Error::Unavailable)?
            .try_into()
            .map_err(|_| Error::Unavailable)?;
        if generation == [0; 16] || self.generation != hex::encode(generation) {
            return Err(Error::Unavailable);
        }
        Ok(CaptureControlSnapshot {
            generation,
            revision: self.revision,
            paused: self.paused,
        })
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let bytes = serde_json::to_vec(self).map_err(|_| Error::Invalid)?;
        if bytes.len() > MAX_BYTES {
            return Err(Error::Capacity);
        }
        Ok(bytes)
    }
}
