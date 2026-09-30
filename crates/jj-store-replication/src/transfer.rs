use crate::{Error, Result};
use jj_store_model::{EnvelopeDigest, RepositoryId};
use jj_store_object::{Condition, Key, ObjectStore, StagedUpload, WriteOutcome};
use jj_store_wal::{
    inventory::Representation, publication::OriginClosureV1, records::MAX_RECORD_BYTES,
};
use sha2::{Digest as _, Sha256};
use std::{collections::BTreeSet, sync::Arc};

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
        if EnvelopeDigest::new(Sha256::digest(&data).into()) != envelope_digest {
            return Err(Error::Integrity("native object digest"));
        }
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
