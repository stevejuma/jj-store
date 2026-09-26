//! Native objects over the owned transport, with integrity and collision checks.
use crate::codec::{self, MAX_METADATA_BYTES, NativeCodec};
use blake2::{Blake2b512, Digest as _};
use jj_store_model::{
    self as m,
    envelope::{self, HEADER_LEN, Header},
};
use jj_store_object::{Condition, Key, ObjectStore, StagedUpload, WriteOutcome};
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncReadExt as _};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Store(#[from] jj_store_object::Error),
    #[error(transparent)]
    Codec(#[from] codec::CodecError),
    #[error(transparent)]
    Envelope(#[from] envelope::EnvelopeError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("immutable native key contains different content")]
    Collision,
    #[error("native blob integrity or length mismatch")]
    Integrity,
}
type Result<T> = std::result::Result<T, Error>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UploadStatus {
    Stored,
    AlreadyPresent,
    Unknown,
}
pub struct NativeObjects {
    store: Arc<dyn ObjectStore>,
    repository: m::RepositoryId,
    max_blob: u64,
    inventory_scope: jj_store_wal::inventory::SourceScope,
}
impl NativeObjects {
    pub fn new(
        store: Arc<dyn ObjectStore>,
        repository: m::RepositoryId,
        max_blob: u64,
    ) -> Result<Self> {
        if max_blob > jj_store_object::staging::MAX_SINGLE_PUT - HEADER_LEN as u64 {
            return Err(jj_store_object::Error::Limit.into());
        }
        Ok(Self {
            store,
            repository,
            max_blob,
            inventory_scope: jj_store_wal::inventory::SourceScope::new(repository),
        })
    }
    fn path(&self, key: m::ObjectKey) -> Key {
        Key::new(key.storage_path(self.repository)).expect("owned IDs generate canonical keys")
    }
    pub async fn get<T: NativeCodec>(&self, key: m::ObjectKey) -> Result<T> {
        Ok(self
            .get_verified(key, MAX_METADATA_BYTES + HEADER_LEN as u64)
            .await?
            .0)
    }
    async fn get_verified<T: NativeCodec>(
        &self,
        key: m::ObjectKey,
        max_bytes: u64,
    ) -> Result<(T, Header, u64)> {
        let bytes = self
            .store
            .get(&self.path(key), None)
            .await?
            .read_bounded(max_bytes.min(MAX_METADATA_BYTES + HEADER_LEN as u64))
            .await?;
        let value = codec::decode(key, &bytes, MAX_METADATA_BYTES)?;
        let header = Header::decode(&bytes[..HEADER_LEN], key, MAX_METADATA_BYTES)?;
        Ok((value, header, bytes.len() as u64))
    }
    pub async fn put<T: NativeCodec>(&self, value: &T) -> Result<(m::ObjectKey, UploadStatus)> {
        let (key, bytes) = codec::encode(value, MAX_METADATA_BYTES)?;
        let upload =
            StagedUpload::from_bytes(&bytes, MAX_METADATA_BYTES + HEADER_LEN as u64).await?;
        let outcome = self
            .store
            .put(&self.path(key), Condition::Absent, &upload)
            .await?;
        let status = match outcome {
            WriteOutcome::Applied(_) => UploadStatus::Stored,
            WriteOutcome::PreconditionFailed | WriteOutcome::Unknown => {
                match self.get::<T>(key).await {
                    Ok(existing) if existing == *value => UploadStatus::AlreadyPresent,
                    Ok(_) => return Err(Error::Collision),
                    Err(Error::Store(
                        jj_store_object::Error::NotFound | jj_store_object::Error::Transport,
                    )) if outcome == WriteOutcome::Unknown => UploadStatus::Unknown,
                    Err(err) => return Err(err),
                }
            }
        };
        Ok((key, status))
    }
    /// Private temporary files permit identity calculation before conditional
    /// upload without retaining large blobs in memory. At most two full spools
    /// coexist while framing; callers must configure an appropriate disk budget.
    pub async fn put_file(
        &self,
        reader: impl AsyncRead + Unpin,
    ) -> Result<(m::FileId, UploadStatus)> {
        let raw = StagedUpload::from_reader(reader, self.max_blob).await?;
        let mut hash = Blake2b512::new();
        let mut input = raw.reader()?;
        let mut buffer = vec![0; 64 * 1024];
        loop {
            let n = input.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            hash.update(&buffer[..n]);
        }
        let id = m::FileId::new(hash.finalize().into());
        let key = m::ObjectKey::File(id);
        let digest = hash_envelope(key, &raw).await?;
        let header = Header {
            key,
            payload_len: raw.len(),
            digest,
        };
        let reader = std::io::Cursor::new(header.encode()).chain(raw.reader()?);
        let framed = StagedUpload::from_reader(reader, self.max_blob + HEADER_LEN as u64).await?;
        let outcome = self
            .store
            .put(&self.path(key), Condition::Absent, &framed)
            .await?;
        let status = match outcome {
            WriteOutcome::Applied(_) => UploadStatus::Stored,
            WriteOutcome::PreconditionFailed | WriteOutcome::Unknown => {
                match self.get_file(id).await {
                    Ok(existing) => {
                        if !equal_streams(&raw, &existing).await? {
                            return Err(Error::Collision);
                        }
                        UploadStatus::AlreadyPresent
                    }
                    Err(Error::Store(
                        jj_store_object::Error::NotFound | jj_store_object::Error::Transport,
                    )) if outcome == WriteOutcome::Unknown => UploadStatus::Unknown,
                    Err(err) => return Err(err),
                }
            }
        };
        Ok((id, status))
    }
    /// Returns a private verified spool. Unverified remote bytes are never
    /// exposed as successful native content, including late stream failures.
    pub async fn get_file(&self, id: m::FileId) -> Result<StagedUpload> {
        Ok(self
            .get_file_verified(id, self.max_blob + HEADER_LEN as u64)
            .await?
            .0)
    }
    async fn get_file_verified(
        &self,
        id: m::FileId,
        max_bytes: u64,
    ) -> Result<(StagedUpload, Header)> {
        let key = m::ObjectKey::File(id);
        let mut download = self.store.get(&self.path(key), None).await?;
        if download.metadata.len > max_bytes.min(self.max_blob + HEADER_LEN as u64) {
            return Err(jj_store_object::Error::Limit.into());
        }
        let mut bytes = [0; HEADER_LEN];
        download.reader.read_exact(&mut bytes).await?;
        let header = Header::decode(&bytes, key, self.max_blob)?;
        if header.payload_len.checked_add(HEADER_LEN as u64) != Some(download.metadata.len) {
            return Err(Error::Integrity);
        }
        let raw = StagedUpload::from_reader(download.reader, header.payload_len).await?;
        if raw.len() != header.payload_len {
            return Err(Error::Integrity);
        }
        let mut native = Blake2b512::new();
        let mut envelope = Header::hasher(key, raw.len());
        let mut input = raw.reader()?;
        let mut buffer = vec![0; 64 * 1024];
        loop {
            let n = input.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            native.update(&buffer[..n]);
            envelope.update(&buffer[..n]);
        }
        if native.finalize().as_slice() != id.as_bytes()
            || envelope.finalize().as_slice() != header.digest.as_bytes()
        {
            return Err(Error::Integrity);
        }
        Ok((raw, header))
    }
}
async fn hash_envelope(key: m::ObjectKey, raw: &StagedUpload) -> Result<m::EnvelopeDigest> {
    let mut hash = Header::hasher(key, raw.len());
    let mut input = raw.reader()?;
    let mut buffer = vec![0; 64 * 1024];
    loop {
        let n = input.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        hash.update(&buffer[..n]);
    }
    Ok(m::EnvelopeDigest::new(hash.finalize().into()))
}
async fn equal_streams(a: &StagedUpload, b: &StagedUpload) -> Result<bool> {
    if a.len() != b.len() || a.sha256() != b.sha256() {
        return Ok(false);
    }
    let mut a = a.reader()?;
    let mut b = b.reader()?;
    let mut left = vec![0; 64 * 1024];
    let mut right = vec![0; 64 * 1024];
    loop {
        let n = a.read(&mut left).await?;
        if n == 0 {
            return Ok(true);
        }
        b.read_exact(&mut right[..n]).await?;
        if left[..n] != right[..n] {
            return Ok(false);
        }
    }
}

#[async_trait::async_trait]
impl jj_store_wal::inventory::DependencySource for NativeObjects {
    fn scope(&self) -> jj_store_wal::inventory::SourceScope {
        self.inventory_scope
    }
    async fn load_verified(
        &self,
        key: m::ObjectKey,
        max_bytes: u64,
    ) -> jj_store_wal::inventory::Result<jj_store_wal::inventory::VerifiedNode> {
        use crate::{dependencies::NativeDependencies as _, roots};
        use jj_lib::{backend as b, op_store as o};
        use jj_store_wal::inventory::{InventoryError, Representation, VerifiedNode};
        let source = |e: Error| InventoryError::Source(e.to_string());
        let virtual_deps = match key {
            m::ObjectKey::Commit(id) if id == roots::ROOT_COMMIT => {
                Some(roots::commit().dependencies())
            }
            m::ObjectKey::View(id) if id == roots::ROOT_VIEW => Some(roots::view().dependencies()),
            m::ObjectKey::Operation(id) if id == roots::ROOT_OPERATION => {
                Some(roots::operation().dependencies())
            }
            m::ObjectKey::Tree(_) if key == b::Tree::default().key() => {
                Some(b::Tree::default().dependencies())
            }
            _ => None,
        };
        if let Some(dependencies) = virtual_deps {
            return Ok(VerifiedNode {
                key,
                representation: Representation::Virtual,
                dependencies: dependencies.map_err(|e| source(e.into()))?,
            });
        }
        macro_rules! metadata {
            ($type:ty) => {{
                let (value, header, bytes) = self
                    .get_verified::<$type>(key, max_bytes)
                    .await
                    .map_err(source)?;
                VerifiedNode {
                    key,
                    representation: Representation::Stored {
                        envelope_digest: header.digest,
                        bytes,
                    },
                    dependencies: value.dependencies().map_err(|e| source(e.into()))?,
                }
            }};
        }
        Ok(match key {
            m::ObjectKey::Commit(_) => metadata!(b::Commit),
            m::ObjectKey::Tree(_) => metadata!(b::Tree),
            m::ObjectKey::Copy(_) => metadata!(b::CopyHistory),
            m::ObjectKey::View(_) => metadata!(o::View),
            m::ObjectKey::Operation(_) => metadata!(o::Operation),
            m::ObjectKey::Symlink(_) => metadata!(codec::NativeSymlink),
            m::ObjectKey::File(id) => {
                let (raw, header) = self
                    .get_file_verified(id, max_bytes)
                    .await
                    .map_err(source)?;
                VerifiedNode {
                    key,
                    representation: Representation::Stored {
                        envelope_digest: header.digest,
                        bytes: raw.len() + HEADER_LEN as u64,
                    },
                    dependencies: Default::default(),
                }
            }
        })
    }
}
