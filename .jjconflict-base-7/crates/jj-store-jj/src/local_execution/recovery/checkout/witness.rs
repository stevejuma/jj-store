use super::*;
use prost::Message as _;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
const MAX_BYTES: usize = 128 * 1024;
/// Installation-private exact pointer transition. Trusted composition durably
/// links its digest to the original request before calling completion.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckoutWitnessV1 {
    version: u32,
    key: StackPreparationKey,
    recovery: m::RecordDigest,
    operation: m::OperationId,
    view: m::ViewId,
    physical: m::RecordDigest,
    before: Vec<u8>,
    before_fingerprint: m::RecordDigest,
    after: Vec<u8>,
}
impl CheckoutWitnessV1 {
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.native()?;
        let bytes = serde_json::to_vec(self).map_err(|_| Error::Invalid)?;
        if bytes.len() > MAX_BYTES {
            return Err(Error::Limit);
        }
        Ok(bytes)
    }
    pub fn id(&self) -> Result<m::RecordDigest> {
        let mut hash = Sha256::new_with_prefix(b"jj-store/unchanged-checkout-witness/v1\0");
        hash.update(self.encode()?);
        Ok(m::RecordDigest::new(hash.finalize().into()))
    }
    pub fn decode(id: m::RecordDigest, bytes: &[u8]) -> Result<Self> {
        if bytes.len() > MAX_BYTES {
            return Err(Error::Limit);
        }
        let value: Self = serde_json::from_slice(bytes).map_err(|_| Error::Invalid)?;
        if value.encode()? != bytes || value.id()? != id {
            return Err(Error::Invalid);
        }
        Ok(value)
    }
    fn native(&self) -> Result<jj_lib::protos::local_working_copy::Checkout> {
        if self.version != 1
            || self.before.len() > 65536
            || self.after.len() > 65536
            || self.before_fingerprint == m::RecordDigest::new([0; 32])
            || self.physical == m::RecordDigest::new([0; 32])
            || self.recovery == m::RecordDigest::new([0; 32])
        {
            return Err(Error::Invalid);
        }
        let original = jj_lib::protos::local_working_copy::Checkout::decode(self.before.as_slice())
            .map_err(|_| Error::Invalid)?;
        if original.operation_id.len() != 64
            || original.workspace_name.len() > 256
            || original.workspace_name.chars().any(char::is_control)
            || original.encode_to_vec() != self.before
        {
            return Err(Error::Invalid);
        }
        let mut candidate = original.clone();
        candidate.operation_id = self.operation.as_bytes().to_vec();
        if candidate.encode_to_vec() != self.after || self.before == self.after {
            return Err(Error::Invalid);
        }
        Ok(original)
    }
    pub(super) fn capture(
        recovery: &RecoveredHistory,
        physical: m::RecordDigest,
        budget: &Budget,
    ) -> Result<Self> {
        let files =
            capture::CheckoutFiles::read(&recovery.root, recovery.access.metadata_path(), budget)?;
        let before = files.checkout.bytes;
        let mut proto = jj_lib::protos::local_working_copy::Checkout::decode(before.as_slice())
            .map_err(|_| Error::Invalid)?;
        proto.operation_id = recovery.archive.operation.as_bytes().to_vec();
        let value = Self {
            version: 1,
            key: recovery.key.clone(),
            recovery: recovery.witness.id()?,
            operation: recovery.archive.operation,
            view: recovery.archive.operations[&recovery.archive.operation].view_id,
            physical,
            before,
            before_fingerprint: files.checkout.fingerprint,
            after: proto.encode_to_vec(),
        };
        value.validate(recovery)?;
        value.encode()?;
        Ok(value)
    }
    pub(super) fn validate(&self, recovery: &RecoveredHistory) -> Result<()> {
        let native = self.native()?;
        let binding = recovery.witness.binding();
        recovery.witness.unchanged_tree(&recovery.archive)?;
        let workspace = if native.workspace_name.is_empty() {
            "default"
        } else {
            &native.workspace_name
        };
        if self.key != recovery.key
            || self.recovery != recovery.witness.id()?
            || self.operation != recovery.archive.operation
            || self.view != recovery.archive.operations[&self.operation].view_id
            || native.operation_id != binding.operation.as_bytes()
            || workspace != binding.workspace
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    pub(in crate::local_execution::recovery) fn normalize(
        &self,
        files: &mut capture::CheckoutFiles,
    ) -> Result<()> {
        if files.checkout.bytes == self.before {
            if files.checkout.fingerprint != self.before_fingerprint {
                return Err(Error::EpochChanged);
            }
        } else if files.checkout.bytes == self.after {
            // Normalize only the intended atomic replacement's file stamp. All
            // other metadata, policy and directory identities remain exact.
            files.checkout.fingerprint = self.before_fingerprint;
        } else {
            return Err(Error::EpochChanged);
        }
        Ok(())
    }
    pub(super) fn is_selected(&self, bytes: &[u8]) -> bool {
        bytes == self.after
    }
    pub(super) fn after(&self) -> &[u8] {
        &self.after
    }
    pub(super) fn physical(&self) -> m::RecordDigest {
        self.physical
    }
}
