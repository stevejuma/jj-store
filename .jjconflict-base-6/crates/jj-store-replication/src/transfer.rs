use crate::{Error, Result};
use jj_store_model::{EnvelopeDigest, ObjectKey, RepositoryId, envelope};
use jj_store_object::{Condition, Key, ObjectStore, StagedUpload, WriteOutcome};
use jj_store_wal::{
    inventory::Representation, publication::OriginClosureV1, records::MAX_RECORD_BYTES,
};
use sha2::{Digest as _, Sha256};
use std::{collections::BTreeSet, sync::Arc};
use tokio::io::AsyncReadExt as _;

#[derive(Clone, Copy, Debug)]
pub struct TransferLimits {
    pub max_objects: usize,
    pub max_bytes: u64,
}

impl Default for TransferLimits {
    fn default() -> Self {
        Self {
            max_objects: 100_000,
            max_bytes: 1024 * 1024 * 1024,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransferOutcome {
    pub native_objects: usize,
    pub control_records: usize,
    pub transferred_bytes: u64,
}

/// Inventory digests cover the native envelope prefix and payload, not the
/// SHA-256 of the stored header plus payload.
pub(crate) async fn verify_native_upload(
    key: ObjectKey,
    upload: &StagedUpload,
    expected_digest: EnvelopeDigest,
    expected_bytes: u64,
) -> Result<()> {
    if upload.len() != expected_bytes || expected_bytes < envelope::HEADER_LEN as u64 {
        return Err(Error::Integrity("native envelope length"));
    }
    let mut reader = upload.reader()?;
    let mut prefix = [0u8; envelope::HEADER_LEN];
    reader
        .read_exact(&mut prefix)
        .await
        .map_err(|_| Error::Integrity("native envelope header"))?;
    let header =
        envelope::Header::decode(&prefix, key, expected_bytes - envelope::HEADER_LEN as u64)
            .map_err(|_| Error::Integrity("native envelope header"))?;
    if header.payload_len + envelope::HEADER_LEN as u64 != expected_bytes
        || header.digest != expected_digest
    {
        return Err(Error::Integrity("native envelope digest"));
    }
    let mut hash = envelope::Header::hasher(key, header.payload_len);
    let mut remaining = header.payload_len;
    let mut buffer = [0u8; 64 * 1024];
    while remaining > 0 {
        let length = remaining.min(buffer.len() as u64) as usize;
        let read = reader
            .read(&mut buffer[..length])
            .await
            .map_err(|_| Error::Integrity("native envelope payload"))?;
        if read == 0 {
            return Err(Error::Integrity("native envelope payload"));
        }
        hash.update(&buffer[..read]);
        remaining -= read as u64;
    }
    if hash.finalize().as_slice() != expected_digest.as_bytes() {
        return Err(Error::Integrity("native envelope digest"));
    }
    let mut extra = [0u8; 1];
    if reader
        .read(&mut extra)
        .await
        .map_err(|_| Error::Integrity("native envelope length"))?
        != 0
    {
        return Err(Error::Integrity("native envelope length"));
    }
    Ok(())
}

/// Copy only immutable origin closure objects. No manifest, WAL candidate, or
/// receipt is accepted here; the receiver publishes them through apply_origins.
pub async fn transfer_closures(
    source: Arc<dyn ObjectStore>,
    destination: Arc<dyn ObjectStore>,
    repository: RepositoryId,
    closures: &[OriginClosureV1],
    limits: TransferLimits,
) -> Result<TransferOutcome> {
    if limits.max_objects == 0 || limits.max_bytes == 0 {
        return Err(Error::Limit("transfer limits"));
    }
    let mut native = BTreeSet::new();
    let mut control = BTreeSet::new();
    for closure in closures {
        native.extend(closure.native.iter().map(|node| node.key));
        control.extend(closure.control.iter().copied());
    }
    if native.len().saturating_add(control.len()) > limits.max_objects {
        return Err(Error::Limit("transfer objects"));
    }
    let mut transferred_bytes = 0u64;
    let mut native_objects = 0usize;
    for key in &native {
        let node = closures
            .iter()
            .flat_map(|closure| &closure.native)
            .find(|node| node.key == *key)
            .ok_or(Error::Integrity("native descriptor missing"))?;
        let Representation::Stored {
            envelope_digest,
            bytes,
        } = node.representation
        else {
            continue;
        };
        native_objects += 1;
        let storage_key = Key::new(key.storage_path(repository))?;
        let data = source
            .get(&storage_key, None)
            .await?
            .read_bounded(bytes)
            .await?;
        let upload = StagedUpload::from_bytes(&data, bytes).await?;
        verify_native_upload(*key, &upload, envelope_digest, bytes).await?;
        put_exact(&destination, &storage_key, &data, bytes).await?;
        transferred_bytes = add_bytes(transferred_bytes, data.len() as u64, limits)?;
    }
    for reference in &control {
        let key = reference.key(repository)?;
        let data = source
            .get(&key, None)
            .await?
            .read_bounded(MAX_RECORD_BYTES)
            .await?;
        if jj_store_model::RecordDigest::new(Sha256::digest(&data).into()) != reference.digest {
            return Err(Error::Integrity("control record digest"));
        }
        put_exact(&destination, &key, &data, MAX_RECORD_BYTES).await?;
        transferred_bytes = add_bytes(transferred_bytes, data.len() as u64, limits)?;
    }
    Ok(TransferOutcome {
        native_objects,
        control_records: control.len(),
        transferred_bytes,
    })
}

fn add_bytes(current: u64, added: u64, limits: TransferLimits) -> Result<u64> {
    let total = current
        .checked_add(added)
        .ok_or(Error::Limit("transfer bytes"))?;
    if total > limits.max_bytes {
        return Err(Error::Limit("transfer bytes"));
    }
    Ok(total)
}

async fn put_exact(
    store: &Arc<dyn ObjectStore>,
    key: &Key,
    bytes: &[u8],
    limit: u64,
) -> Result<()> {
    let upload = StagedUpload::from_bytes(bytes, limit).await?;
    match store.put(key, Condition::Absent, &upload).await? {
        WriteOutcome::Applied(_) => Ok(()),
        WriteOutcome::PreconditionFailed | WriteOutcome::Unknown => {
            let existing = store.get(key, None).await?.read_bounded(limit).await?;
            if existing == bytes {
                Ok(())
            } else {
                Err(Error::Integrity("immutable transfer collision"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jj_store_model::OperationId;

    #[tokio::test]
    async fn native_transfer_checks_the_envelope_digest_and_payload() {
        let key = ObjectKey::Operation(OperationId::new([7; 64]));
        let bytes = envelope::encode(key, b"native operation", 1024).unwrap();
        let digest = envelope::Header::decode(&bytes[..envelope::HEADER_LEN], key, 1024)
            .unwrap()
            .digest;
        assert_ne!(digest, EnvelopeDigest::new(Sha256::digest(&bytes).into()));
        let upload = StagedUpload::from_bytes(&bytes, bytes.len() as u64)
            .await
            .unwrap();
        verify_native_upload(key, &upload, digest, bytes.len() as u64)
            .await
            .unwrap();

        let mut corrupt = bytes.clone();
        *corrupt.last_mut().unwrap() ^= 1;
        let upload = StagedUpload::from_bytes(&corrupt, corrupt.len() as u64)
            .await
            .unwrap();
        assert!(
            verify_native_upload(key, &upload, digest, corrupt.len() as u64)
                .await
                .is_err()
        );
        assert!(
            verify_native_upload(
                ObjectKey::Operation(OperationId::new([8; 64])),
                &upload,
                digest,
                corrupt.len() as u64,
            )
            .await
            .is_err()
        );
    }
}
