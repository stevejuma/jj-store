//! Non-authoritative, content-checked staging bound to an exact conflict context.
use crate::{
    auth::Permission,
    commands::{Command, ConflictResolution, Resolution},
    native::NativeSession,
};
use jj_store_jj::objects::{self, NativeObjects, UploadStatus};
use jj_store_model as m;
use jj_store_object::{
    self as storage, ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload,
    WriteOutcome,
};
use jj_store_wal as wal;
pub use jj_store_wal::publication::Snapshot;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{collections::BTreeMap, sync::Arc};
use tokio::{io::AsyncRead, sync::Semaphore};
const RECORD_BYTES: u64 = 16 * 1024;
pub const MAX_FILE_BYTES: u64 = 256 * 1024 * 1024;
pub const MAX_BATCH_BYTES: u64 = 256 * 1024 * 1024;
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("upload is not authorized")]
    Denied,
    #[error("upload requires the exact current snapshot")]
    Stale,
    #[error("invalid upload request")]
    Invalid,
    #[error("upload is outside this conflict context")]
    InvalidUpload,
    #[error("upload resource limit exceeded")]
    Limit,
    #[error("upload workers are busy")]
    Busy,
    #[error("upload integrity check failed")]
    Corrupt,
    #[error("upload storage is unavailable; repeating identical upload is safe")]
    Unavailable,
}
pub type Result<T> = std::result::Result<T, Error>;
impl From<wal::Error> for Error {
    fn from(e: wal::Error) -> Self {
        match e {
            wal::Error::Denied => Self::Denied,
            wal::Error::Limit(_) => Self::Limit,
            wal::Error::Stale => Self::Stale,
            _ => Self::Unavailable,
        }
    }
}
impl From<storage::Error> for Error {
    fn from(e: storage::Error) -> Self {
        match e {
            storage::Error::Limit => Self::Limit,
            _ => Self::Unavailable,
        }
    }
}
impl From<objects::Error> for Error {
    fn from(e: objects::Error) -> Self {
        match e {
            objects::Error::Store(e) => e.into(),
            objects::Error::Integrity | objects::Error::Envelope(_) | objects::Error::Collision => {
                Self::Corrupt
            }
            _ => Self::Unavailable,
        }
    }
}
impl From<Error> for crate::commands::Error {
    fn from(e: Error) -> Self {
        use jj_store_jj::query::Error as Q;
        match e {
            Error::Stale => Self::Stale,
            Error::Invalid => Self::Invalid,
            Error::InvalidUpload => Self::InvalidUpload,
            Error::Denied => Q::Denied.into(),
            Error::Limit => Q::Limit.into(),
            Error::Busy => Q::Busy.into(),
            Error::Corrupt => Q::Corrupt.into(),
            Error::Unavailable => Q::Unavailable.into(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    pub expected: Snapshot,
    pub commit: m::CommitId,
    pub path: String,
    pub fingerprint: m::ConflictFingerprint,
}
impl Binding {
    fn validate(&self) -> Result<()> {
        if self.expected.heads.len() != 1 || self.commit == jj_store_jj::roots::ROOT_COMMIT {
            return Err(Error::Invalid);
        }
        Command::ResolveConflicts {
            commit: self.commit,
            paths: vec![ConflictResolution {
                path: self.path.clone(),
                fingerprint: self.fingerprint,
                resolution: Resolution::Delete,
            }],
        }
        .validate()
        .map_err(|_| Error::Invalid)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "ConflictUpload")]
pub struct Upload {
    pub version: u32,
    pub id: m::ConflictUploadId,
    pub file: m::FileId,
    pub bytes: u64,
    pub binding: Binding,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    version: u32,
    repository: m::RepositoryId,
    principal: m::PrincipalId,
    binding: Binding,
    file: m::FileId,
    bytes: u64,
}
impl Record {
    fn encode(&self) -> Result<Vec<u8>> {
        self.binding.validate()?;
        if self.version != 1 || self.bytes > MAX_FILE_BYTES {
            return Err(Error::Corrupt);
        }
        let bytes = serde_json::to_vec(self).map_err(|_| Error::Corrupt)?;
        if bytes.len() as u64 > RECORD_BYTES {
            return Err(Error::Limit);
        }
        Ok(bytes)
    }
    fn digest(bytes: &[u8]) -> m::ConflictUploadId {
        let mut hash = Sha256::new_with_prefix(b"jj-store/conflict-upload/v1\0");
        hash.update(bytes);
        m::ConflictUploadId::new(hash.finalize().into())
    }
    fn decode(bytes: &[u8], id: m::ConflictUploadId) -> Result<Self> {
        if bytes.len() as u64 > RECORD_BYTES || Self::digest(bytes) != id {
            return Err(Error::Corrupt);
        }
        let record: Self = serde_json::from_slice(bytes).map_err(|_| Error::Corrupt)?;
        if record.encode().map_err(|_| Error::Corrupt)? != bytes {
            return Err(Error::Corrupt);
        }
        Ok(record)
    }
}
fn key(
    repo: m::RepositoryId,
    epoch: m::EpochId,
    principal: m::PrincipalId,
    id: m::ConflictUploadId,
) -> Key {
    Key::new(format!("repos/{repo}/uploads/v1/{epoch}/{principal}/{id}"))
        .expect("typed upload scope")
}
async fn authorize(
    session: &NativeSession,
    expected: &Snapshot,
    principal: m::PrincipalId,
) -> Result<()> {
    if session.principal(Permission::Write).await?.id() != principal {
        return Err(Error::Denied);
    }
    if session.snapshot().await?.manifest().snapshot() != *expected {
        return Err(Error::Stale);
    }
    Ok(())
}
#[derive(Clone, Copy)]
pub struct Limits {
    pub max_file_bytes: u64,
    pub concurrency: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_file_bytes: 16 * 1024 * 1024,
            concurrency: 2,
        }
    }
}
#[derive(Clone)]
pub struct Uploads {
    limits: Limits,
    slots: Arc<Semaphore>,
}
impl Uploads {
    pub fn new(limits: Limits) -> Result<Self> {
        if !(1..=MAX_FILE_BYTES).contains(&limits.max_file_bytes)
            || !(1..=32).contains(&limits.concurrency)
        {
            return Err(Error::Limit);
        }
        Ok(Self {
            limits,
            slots: Arc::new(Semaphore::new(limits.concurrency)),
        })
    }
    pub async fn stage(
        &self,
        session: Arc<NativeSession>,
        binding: Binding,
        reader: impl AsyncRead + Unpin + Send,
        declared: Option<u64>,
    ) -> Result<Upload> {
        let _permit = self
            .slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| Error::Busy)?;
        tokio::time::timeout(session.limits.timeout, async {
            let principal = session.principal(Permission::Write).await?.id();
            binding.validate()?;
            authorize(&session, &binding.expected, principal).await?;
            if declared.is_some_and(|n| n > self.limits.max_file_bytes) {
                return Err(Error::Limit);
            }
            let snapshot = session.snapshot().await?;
            let members = session
                .coordinator
                .inventory(&snapshot, session.limits.inventory_nodes)
                .await?;
            if !members
                .iter()
                .any(|node| node.key == m::ObjectKey::Commit(binding.commit))
            {
                return Err(Error::Invalid);
            }
            let raw = StagedUpload::from_reader(reader, self.limits.max_file_bytes).await?;
            if declared.is_some_and(|n| n != raw.len()) {
                return Err(Error::Invalid);
            }
            authorize(&session, &binding.expected, principal).await?;
            let scoped = Arc::new(ScopedStore {
                session: session.clone(),
                expected: binding.expected.clone(),
                principal,
            });
            let objects =
                NativeObjects::new(scoped, session.repository, self.limits.max_file_bytes)?;
            let uploaded = objects.put_file(raw.reader()?).await;
            authorize(&session, &binding.expected, principal).await?;
            let (file, status) = uploaded?;
            if status == UploadStatus::Unknown {
                return Err(Error::Unavailable);
            }
            let record = Record {
                version: 1,
                repository: session.repository,
                principal,
                binding: binding.clone(),
                file,
                bytes: raw.len(),
            };
            let bytes = record.encode()?;
            let id = Record::digest(&bytes);
            let path = key(session.repository, binding.expected.epoch, principal, id);
            let framed = StagedUpload::from_bytes(&bytes, RECORD_BYTES).await?;
            authorize(&session, &binding.expected, principal).await?;
            let outcome = session
                .object_store
                .put(&path, Condition::Absent, &framed)
                .await?;
            authorize(&session, &binding.expected, principal).await?;
            if !matches!(outcome, WriteOutcome::Applied(_)) {
                let found = session
                    .object_store
                    .get(&path, None)
                    .await?
                    .read_bounded(RECORD_BYTES)
                    .await?;
                if found != bytes || Record::decode(&found, id)? != record {
                    return Err(Error::Corrupt);
                }
            }
            authorize(&session, &binding.expected, principal).await?;
            Ok(Upload {
                version: 1,
                id,
                file,
                bytes: raw.len(),
                binding,
            })
        })
        .await
        .map_err(|_| Error::Limit)?
    }
}
/// Only called by checked command preparation; returns scoped native membership,
/// never a caller-deserialized capability. No record is needed for receipt replay.
pub(crate) async fn resolve(
    session: &NativeSession,
    expected: &Snapshot,
    command: &Command,
) -> Result<BTreeMap<m::ConflictUploadId, m::FileId>> {
    let mut files = BTreeMap::new();
    let Command::ResolveConflicts { commit, paths } = command else {
        return Ok(files);
    };
    let principal = session.principal(Permission::Write).await?.id();
    let mut total = 0u64;
    for resolution in paths {
        let Resolution::UploadedFile { upload, .. } = resolution.resolution else {
            continue;
        };
        authorize(session, expected, principal).await?;
        let path = key(session.repository, expected.epoch, principal, upload);
        let found = match session.object_store.get(&path, None).await {
            Ok(found) => found.read_bounded(RECORD_BYTES).await?,
            Err(storage::Error::NotFound) => return Err(Error::InvalidUpload),
            Err(e) => return Err(e.into()),
        };
        authorize(session, expected, principal).await?;
        let record = Record::decode(&found, upload)?;
        if record.repository != session.repository
            || record.principal != principal
            || record.binding
                != (Binding {
                    expected: expected.clone(),
                    commit: *commit,
                    path: resolution.path.clone(),
                    fingerprint: resolution.fingerprint,
                })
        {
            return Err(Error::InvalidUpload);
        }
        total = total
            .checked_add(record.bytes)
            .filter(|n| *n <= MAX_BATCH_BYTES)
            .ok_or(Error::Limit)?;
        files.insert(upload, record.file);
    }
    Ok(files)
}
/// Rechecks current authorization before every native storage call, including
/// writes delayed by spooling/hashing and collision verification reads.
struct ScopedStore {
    session: Arc<NativeSession>,
    expected: Snapshot,
    principal: m::PrincipalId,
}
impl ScopedStore {
    async fn check(&self) -> storage::Result<()> {
        authorize(&self.session, &self.expected, self.principal)
            .await
            .map_err(|_| storage::Error::Invalid("upload scope"))
    }
}
#[async_trait::async_trait]
impl ObjectStore for ScopedStore {
    async fn head(&self, key: &Key) -> storage::Result<Metadata> {
        self.check().await?;
        let r = self.session.object_store.head(key).await;
        self.check().await?;
        r
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> storage::Result<Download> {
        self.check().await?;
        let r = self.session.object_store.get(key, range).await;
        self.check().await?;
        r
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> storage::Result<WriteOutcome> {
        self.check().await?;
        let r = self.session.object_store.put(key, condition, upload).await;
        self.check().await?;
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conflict_upload_record_v1_has_canonical_independent_digest_and_scope_binding() {
        let record = Record {
            version: 1,
            repository: m::RepositoryId::new([1; 16]),
            principal: m::PrincipalId::new([2; 16]),
            binding: Binding {
                expected: Snapshot {
                    epoch: m::EpochId::new([3; 16]),
                    position: m::WalPosition(7),
                    code_state_version: 0,
                    heads: std::collections::BTreeSet::from([m::OperationId::new([4; 64])]),
                },
                commit: m::CommitId::new([5; 64]),
                path: "src/file".into(),
                fingerprint: m::ConflictFingerprint::new([6; 64]),
            },
            file: m::FileId::new([7; 64]),
            bytes: 9,
        };
        let bytes = record.encode().unwrap();
        let id = Record::digest(&bytes);
        // Independent Python hashlib over canonical JSON with this field order.
        assert_eq!(
            id.to_string(),
            "243b34cd0b1b70e9d0009a408a8c442586705fd7f0247c91a1df274e706e5016"
        );
        assert_eq!(Record::decode(&bytes, id).unwrap(), record);
        for field in 0..9 {
            let mut changed = record.clone();
            match field {
                0 => changed.repository = m::RepositoryId::new([9; 16]),
                1 => changed.principal = m::PrincipalId::new([9; 16]),
                2 => changed.binding.expected.epoch = m::EpochId::new([9; 16]),
                3 => changed.binding.expected.position.0 += 1,
                4 => changed.binding.path = "another".into(),
                5 => changed.binding.fingerprint = m::ConflictFingerprint::new([9; 64]),
                6 => changed.file = m::FileId::new([9; 64]),
                7 => changed.bytes += 1,
                _ => changed.binding.commit = m::CommitId::new([9; 64]),
            }
            assert_ne!(Record::digest(&changed.encode().unwrap()), id);
            assert_eq!(
                Record::decode(&changed.encode().unwrap(), id),
                Err(Error::Corrupt)
            );
        }
        let mut extra = bytes;
        extra.push(b' ');
        assert_eq!(
            Record::decode(&extra, Record::digest(&extra)),
            Err(Error::Corrupt)
        );
    }
}
