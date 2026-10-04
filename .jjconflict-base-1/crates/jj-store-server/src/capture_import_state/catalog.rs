use super::*;
use std::collections::BTreeSet;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Catalog {
    version: u32,
    installation: String,
    control_generation: String,
    pub receipts: Vec<CaptureImportReceipt>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub readers: Vec<CaptureImportReaderBinding>,
}
impl Catalog {
    pub fn new(installation: [u8; 16], generation: [u8; 16]) -> Self {
        Self {
            version: 1,
            installation: hex::encode(installation),
            control_generation: hex::encode(generation),
            receipts: vec![],
            readers: vec![],
        }
    }
    pub fn decode(bytes: &[u8], installation: [u8; 16], generation: [u8; 16]) -> Result<Self> {
        if bytes.len() > MAX_BYTES {
            return Err(Error::Unavailable);
        }
        let catalog: Self = serde_json::from_slice(bytes).map_err(|_| Error::Unavailable)?;
        if catalog.version != 1
            || catalog.installation != hex::encode(installation)
            || catalog.control_generation != hex::encode(generation)
            || catalog.receipts.len() > MAX_RECEIPTS
            || catalog.readers.len() > catalog.receipts.len()
        {
            return Err(Error::Unavailable);
        }
        let mut requests = BTreeSet::new();
        for (index, receipt) in catalog.receipts.iter().enumerate() {
            if receipt.ordinal != index as u64 + 1
                || receipt.intent.generation != generation
                || !requests.insert(receipt.intent.request)
            {
                return Err(Error::Unavailable);
            }
            receipt.intent.validate().map_err(|_| Error::Unavailable)?;
            receipt.source.validate().map_err(|_| Error::Unavailable)?;
        }
        let mut ordinals = BTreeSet::new();
        for reader in &catalog.readers {
            let receipt = reader
                .ordinal
                .checked_sub(1)
                .and_then(|n| usize::try_from(n).ok())
                .and_then(|n| catalog.receipts.get(n))
                .ok_or(Error::Unavailable)?;
            if !ordinals.insert(reader.ordinal) {
                return Err(Error::Unavailable);
            }
            reader.validate(receipt).map_err(|_| Error::Unavailable)?;
        }
        Ok(catalog)
    }
    pub fn find(&self, intent: &CaptureImportIntent) -> Result<Option<&CaptureImportReceipt>> {
        let Some(receipt) = self
            .receipts
            .iter()
            .find(|r| r.intent.request == intent.request)
        else {
            return Ok(None);
        };
        if receipt.intent != *intent {
            return Err(Error::Conflict);
        }
        Ok(Some(receipt))
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let bytes = serde_json::to_vec(self).map_err(|_| Error::Invalid)?;
        if bytes.len() > MAX_BYTES {
            return Err(Error::Capacity);
        }
        Ok(bytes)
    }
}
