//! Immutable, typed, repository-bound control records.
use crate::{Error, Result};
use jj_store_model::{AttemptId, RecordDigest, RepositoryId};
use jj_store_object::{Condition, Key, ObjectStore, StagedUpload, WriteOutcome};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest as _, Sha256};
use std::sync::Arc;

pub const MAX_RECORD_BYTES: u64 = 16 * 1024 * 1024;
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordKind {
    Catalog,
    Inventory,
    Receipt,
    Wal,
    Checkpoint,
    Settings,
    AgentRoot,
    AgentObject,
    ReviewRoot,
    ReviewObject,
    ReplicationRoot,
    Origin,
    OriginReceipt,
}
impl RecordKind {
    fn path(self) -> &'static str {
        match self {
            Self::Catalog => "catalogs",
            Self::Inventory => "inventories",
            Self::Receipt => "receipts",
            Self::Wal => "wal",
            Self::Checkpoint => "checkpoints",
            Self::Settings => "settings",
            Self::AgentRoot => "agent-roots",
            Self::AgentObject => "agent-objects",
            Self::ReviewRoot => "review-roots",
            Self::ReviewObject => "review-objects",
            Self::ReplicationRoot => "replication-roots",
            Self::Origin => "origins",
            Self::OriginReceipt => "origin-receipts",
        }
    }

    /// Immutable data records needed to validate an imported origin. Authority
    /// records, WAL candidates, receipts and replication state never cross this
    /// transfer boundary.
    pub fn replication_transferable(self) -> bool {
        matches!(
            self,
            Self::Catalog
                | Self::Inventory
                | Self::AgentRoot
                | Self::AgentObject
                | Self::ReviewRoot
                | Self::ReviewObject
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordRef {
    pub kind: RecordKind,
    pub digest: RecordDigest,
    pub attempt: Option<AttemptId>,
}
impl RecordRef {
    pub fn key(self, repository: RepositoryId) -> Result<Key> {
        if (self.kind == RecordKind::Wal) != self.attempt.is_some() {
            return Err(Error::Corrupt("WAL attempt namespace"));
        }
        let path = if let Some(attempt) = self.attempt {
            format!("{}/{attempt}", self.kind.path())
        } else {
            self.kind.path().to_owned()
        };
        Ok(Key::new(format!(
            "repos/{repository}/{path}/{}.json",
            self.digest
        ))?)
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope<T> {
    format: u32,
    repository: RepositoryId,
    kind: RecordKind,
    payload: T,
}
#[derive(Clone)]
pub struct Records {
    pub(crate) store: Arc<dyn ObjectStore>,
    pub(crate) repository: RepositoryId,
}
impl Records {
    pub fn new(store: Arc<dyn ObjectStore>, repository: RepositoryId) -> Self {
        Self { store, repository }
    }
    pub async fn put<T: Serialize>(
        &self,
        kind: RecordKind,
        attempt: Option<AttemptId>,
        payload: &T,
    ) -> Result<RecordRef> {
        let bytes = encode(
            &Envelope {
                format: 1,
                repository: self.repository,
                kind,
                payload,
            },
            MAX_RECORD_BYTES,
        )?;
        let reference = RecordRef {
            kind,
            digest: digest(&bytes),
            attempt,
        };
        let key = reference.key(self.repository)?;
        let upload = StagedUpload::from_bytes(&bytes, MAX_RECORD_BYTES).await?;
        match self.store.put(&key, Condition::Absent, &upload).await? {
            WriteOutcome::Applied(_) => {}
            outcome => match self.read_bytes(reference).await {
                Ok(existing) if existing == bytes => {}
                Ok(_) => return Err(Error::Corrupt("immutable record collision")),
                Err(Error::Storage(
                    jj_store_object::Error::NotFound | jj_store_object::Error::Transport,
                )) if outcome == WriteOutcome::Unknown => return Err(Error::OutcomeUnknown),
                Err(err) => return Err(err),
            },
        }
        Ok(reference)
    }
    pub(crate) async fn read_bytes(&self, reference: RecordRef) -> Result<Vec<u8>> {
        let bytes = self
            .store
            .get(&reference.key(self.repository)?, None)
            .await?
            .read_bounded(MAX_RECORD_BYTES)
            .await?;
        if digest(&bytes) != reference.digest {
            return Err(Error::Corrupt("control record checksum"));
        }
        Ok(bytes)
    }
    pub async fn get<T: Serialize + DeserializeOwned>(
        &self,
        reference: RecordRef,
        expected: RecordKind,
    ) -> Result<T> {
        if reference.kind != expected {
            return Err(Error::Corrupt("record kind"));
        }
        let bytes = self.read_bytes(reference).await?;
        let envelope: Envelope<T> = decode(&bytes)?;
        if envelope.format != 1
            || envelope.repository != self.repository
            || envelope.kind != expected
        {
            return Err(Error::Corrupt("record envelope"));
        }
        Ok(envelope.payload)
    }

    pub(crate) async fn put_replication_bytes(
        &self,
        reference: RecordRef,
        bytes: &[u8],
    ) -> Result<WriteOutcome> {
        if reference.attempt.is_some()
            || !reference.kind.replication_transferable()
            || bytes.len() as u64 > MAX_RECORD_BYTES
            || digest(bytes) != reference.digest
        {
            return Err(Error::Corrupt("replication control record"));
        }
        // This is a staging boundary. Typed reads during apply enforce exact
        // canonical encoding for the concrete payload. Decoding the payload as
        // `Value` here would reorder nested object fields and falsely reject a
        // canonical typed record before it can be verified by its owner.
        let envelope: Envelope<serde_json::Value> = serde_json::from_slice(bytes)?;
        if envelope.format != 1
            || envelope.repository != self.repository
            || envelope.kind != reference.kind
        {
            return Err(Error::Corrupt("replication control record envelope"));
        }
        let upload = StagedUpload::from_bytes(bytes, MAX_RECORD_BYTES).await?;
        let key = reference.key(self.repository)?;
        let outcome = self.store.put(&key, Condition::Absent, &upload).await?;
        match outcome {
            WriteOutcome::Applied(_) => Ok(outcome),
            WriteOutcome::PreconditionFailed => match self.read_bytes(reference).await {
                Ok(existing) if existing == bytes => Ok(outcome),
                Ok(_) => Err(Error::Corrupt("immutable record collision")),
                Err(error) => Err(error),
            },
            WriteOutcome::Unknown => match self.read_bytes(reference).await {
                Ok(existing) if existing == bytes => Ok(WriteOutcome::PreconditionFailed),
                Ok(_) => Err(Error::Corrupt("immutable record collision")),
                Err(Error::Storage(jj_store_object::Error::NotFound)) => Ok(WriteOutcome::Unknown),
                Err(error) => Err(error),
            },
        }
    }
}
pub(crate) fn digest(bytes: &[u8]) -> RecordDigest {
    RecordDigest::new(Sha256::digest(bytes).into())
}
pub(crate) fn encode<T: Serialize>(value: &T, limit: u64) -> Result<Vec<u8>> {
    // Write through a limit instead of allocating an unbounded encoded payload.
    struct Limited {
        bytes: Vec<u8>,
        limit: u64,
    }
    impl std::io::Write for Limited {
        fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            if (self.bytes.len() as u64).saturating_add(data.len() as u64) > self.limit {
                return Err(std::io::Error::other("control record size limit"));
            }
            self.bytes.extend_from_slice(data);
            Ok(data.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut writer = Limited {
        bytes: Vec::new(),
        limit,
    };
    serde_json::to_writer(&mut writer, value)?;
    Ok(writer.bytes)
}
pub(crate) fn decode<T: Serialize + DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    let value: T = serde_json::from_slice(bytes)?;
    if encode(&value, MAX_RECORD_BYTES)? != bytes {
        return Err(Error::Corrupt("noncanonical control record"));
    }
    Ok(value)
}
pub fn new_attempt() -> Result<AttemptId> {
    let mut id = [0; 16];
    getrandom::fill(&mut id).map_err(|_| Error::Entropy)?;
    Ok(AttemptId::new(id))
}
