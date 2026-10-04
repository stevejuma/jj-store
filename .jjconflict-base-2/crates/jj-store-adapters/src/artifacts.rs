//! Optional derived artifacts. No key supplied by a projection reaches storage.
use crate::{source::Captured, *};
use jj_store_object::{Condition, Key, StagedUpload, VersionToken, WriteOutcome};
use sha2::{Digest as _, Sha256};
use std::collections::BTreeMap;
use tokio::io::{AsyncRead, AsyncReadExt as _, AsyncWrite, AsyncWriteExt as _};

const DESCRIPTOR_BYTES: u64 = 4 * 1024 * 1024;
const POINTER_BYTES: u64 = 16 * 1024;
const MAX_FILES: usize = 4096;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactBlob {
    pub bytes: u64,
    pub digest: m::RecordDigest,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactBundle {
    pub version: u32,
    pub checkpoint: Checkpoint,
    pub files: BTreeMap<String, ArtifactBlob>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pointer {
    version: u32,
    checkpoint: Checkpoint,
    descriptor: ArtifactBlob,
}
/// A source-authorized capability, retaining deadline, admission and ownership.
/// It is independent of the authoritative manifest and replay cursor.
#[derive(Clone)]
pub struct Artifacts {
    captured: Captured,
    bound: Position,
    identity: Identity,
    prefix: String,
}
impl Artifacts {
    pub(crate) fn new(captured: Captured, bound: Position, identity: Identity) -> Result<Self> {
        identity.validate()?;
        let digest = Sha256::digest(serde_json::to_vec(&identity).map_err(|_| Error::Invalid)?);
        let identity_digest = m::RecordDigest::new(digest.into());
        let prefix = format!(
            "repos/{}/adapters/v1/{}/snapshots/v1/{}/{identity_digest}",
            bound.repository, identity.name, bound.epoch
        );
        Ok(Self {
            captured,
            bound,
            identity,
            prefix,
        })
    }
    fn context(&self) -> &Context {
        &self.captured.store.context
    }
    fn key(&self, suffix: &str) -> Key {
        Key::new(format!("{}/{suffix}", self.prefix)).expect("internally derived snapshot key")
    }
    fn blob_key(&self, blob: &ArtifactBlob) -> Key {
        self.key(&format!("blobs/{}", blob.digest))
    }
    async fn authorize(&self) -> Result<()> {
        self.captured.store.authorize().await
    }
    async fn checkpoint(&self, checkpoint: &Checkpoint) -> Result<()> {
        checkpoint.validate()?;
        if checkpoint.identity != self.identity
            || checkpoint.source.repository != self.bound.repository
            || checkpoint.source.epoch != self.bound.epoch
            || checkpoint.source.position.0 > self.bound.position.0
        {
            return Err(Error::Invalid);
        }
        self.captured.validate_position(&checkpoint.source).await
    }
    fn blob(&self, blob: &ArtifactBlob) -> Result<()> {
        if blob.bytes > self.context().limits().object_bytes {
            return Err(Error::Limit);
        }
        Ok(())
    }
    async fn bundle(&self, bundle: &ArtifactBundle) -> Result<()> {
        if bundle.version != 1 || bundle.files.is_empty() || bundle.files.len() > MAX_FILES {
            return Err(Error::Invalid);
        }
        let mut total = 0u64;
        for (name, blob) in &bundle.files {
            if name.is_empty()
                || name == "."
                || name == ".."
                || name.len() > 256
                || !name
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
            {
                return Err(Error::Invalid);
            }
            self.blob(blob)?;
            total = total.checked_add(blob.bytes).ok_or(Error::Limit)?;
            if total > self.context().limits().bytes {
                return Err(Error::Limit);
            }
        }
        self.checkpoint(&bundle.checkpoint).await
    }
    async fn staged(&self, blob: &ArtifactBlob) -> Result<StagedUpload> {
        self.blob(blob)?;
        self.authorize().await?;
        self.context().request()?;
        let download = self
            .captured
            .store
            .inner
            .get(&self.blob_key(blob), None)
            .await?;
        if download.metadata.len != blob.bytes {
            return Err(Error::Corrupt);
        }
        self.context().bytes(blob.bytes)?;
        let staged = StagedUpload::from_reader(download.reader, blob.bytes).await?;
        if staged.len() != blob.bytes || *staged.sha256() != *blob.digest.as_bytes() {
            return Err(Error::Corrupt);
        }
        self.authorize().await?;
        Ok(staged)
    }
    async fn put_blob(&self, upload: &StagedUpload) -> Result<ArtifactBlob> {
        let blob = ArtifactBlob {
            bytes: upload.len(),
            digest: m::RecordDigest::new(*upload.sha256()),
        };
        self.blob(&blob)?;
        self.authorize().await?;
        self.context().request()?;
        self.context().bytes(blob.bytes)?;
        let outcome = self
            .captured
            .store
            .inner
            .put(&self.blob_key(&blob), Condition::Absent, upload)
            .await?;
        self.authorize().await?;
        if !matches!(outcome, WriteOutcome::Applied(_)) {
            // The key is content addressed; a pre-existing value or lost reply
            // is successful only if the full bytes can be verified by readback.
            self.staged(&blob).await?;
        }
        Ok(blob)
    }
    /// Streams to a private bounded upload spool; never buffers the file body.
    pub async fn upload(&self, reader: impl AsyncRead + Unpin, limit: u64) -> Result<ArtifactBlob> {
        self.context()
            .until(async {
                self.authorize().await?;
                if limit > self.context().limits().object_bytes {
                    return Err(Error::Limit);
                }
                let upload = StagedUpload::from_reader(reader, limit).await?;
                self.put_blob(&upload).await
            })
            .await
    }
    /// Copies verified bytes into caller-owned staging. The caller must sync and
    /// validate the complete restored index before activating any of its files.
    pub async fn download(
        &self,
        blob: &ArtifactBlob,
        mut writer: impl AsyncWrite + Unpin,
    ) -> Result<()> {
        self.context()
            .until(async {
                let staged = self.staged(blob).await?;
                tokio::io::copy(&mut staged.reader()?, &mut writer)
                    .await
                    .map_err(|_| Error::Unavailable)?;
                writer.flush().await.map_err(|_| Error::Unavailable)?;
                self.authorize().await
            })
            .await
    }
    async fn pointer(&self) -> Result<Option<(Pointer, VersionToken)>> {
        self.authorize().await?;
        self.context().request()?;
        let result = match self
            .captured
            .store
            .inner
            .get(&self.key("snapshot.json"), None)
            .await
        {
            Err(jj_store_object::Error::NotFound) => None,
            Err(error) => return Err(error.into()),
            Ok(download) => {
                self.context().bytes(download.metadata.len)?;
                let version = download.metadata.version.clone();
                let bytes = download.read_bounded(POINTER_BYTES).await?;
                let pointer: Pointer = canonical(&bytes)?;
                if pointer.version != 1 || pointer.descriptor.bytes > DESCRIPTOR_BYTES {
                    return Err(Error::Corrupt);
                }
                self.blob(&pointer.descriptor)?;
                self.checkpoint(&pointer.checkpoint).await?;
                Some((pointer, version))
            }
        };
        self.authorize().await?;
        Ok(result)
    }
    async fn load(&self, pointer: &Pointer) -> Result<ArtifactBundle> {
        let staged = self.staged(&pointer.descriptor).await?;
        let mut bytes = Vec::new();
        staged
            .reader()?
            .read_to_end(&mut bytes)
            .await
            .map_err(|_| Error::Unavailable)?;
        let bundle: ArtifactBundle = canonical(&bytes)?;
        self.bundle(&bundle).await?;
        if bundle.checkpoint != pointer.checkpoint {
            return Err(Error::Corrupt);
        }
        Ok(bundle)
    }
    /// A selected descriptor is not proof of surviving files. Restore downloads
    /// every file and verifies projection-owned schema/payload before use.
    pub async fn selected(&self) -> Result<Option<ArtifactBundle>> {
        self.context()
            .until(async {
                match self.pointer().await? {
                    Some((pointer, _)) => self.load(&pointer).await.map(Some),
                    None => Ok(None),
                }
            })
            .await
    }
    /// Verify immutable dependencies, write the descriptor, then CAS the pointer.
    /// An older publisher must never move a compatible snapshot backwards.
    pub async fn publish(&self, bundle: &ArtifactBundle) -> Result<ArtifactBundle> {
        self.context()
            .until(async {
                self.authorize().await?;
                self.bundle(bundle).await?;
                let old = self.pointer().await?;
                if let Some((pointer, _)) = &old
                    && pointer.checkpoint.source.position.0 > bundle.checkpoint.source.position.0
                {
                    return Err(Error::CursorConflict);
                }
                let bytes = serde_json::to_vec(bundle).map_err(|_| Error::Invalid)?;
                if bytes.len() as u64 > DESCRIPTOR_BYTES {
                    return Err(Error::Limit);
                }
                // A caller cannot fabricate a reference and select missing files.
                for blob in bundle.files.values() {
                    self.staged(blob).await?;
                }
                let upload = StagedUpload::from_bytes(&bytes, DESCRIPTOR_BYTES).await?;
                let descriptor = self.put_blob(&upload).await?;
                let pointer = Pointer {
                    version: 1,
                    checkpoint: bundle.checkpoint.clone(),
                    descriptor,
                };
                if old
                    .as_ref()
                    .is_some_and(|(selected, _)| selected == &pointer)
                {
                    return Ok(bundle.clone());
                }
                let bytes = serde_json::to_vec(&pointer).map_err(|_| Error::Invalid)?;
                let upload = StagedUpload::from_bytes(&bytes, POINTER_BYTES).await?;
                self.authorize().await?;
                self.context().request()?;
                self.context().bytes(upload.len())?;
                let condition = old.map_or(Condition::Absent, |(_, version)| {
                    Condition::Matches(version)
                });
                let outcome = self
                    .captured
                    .store
                    .inner
                    .put(&self.key("snapshot.json"), condition, &upload)
                    .await?;
                self.authorize().await?;
                // Checked readback also catches a lost acknowledgement. Only the
                // exact descriptor proves this attempt selected its complete bundle.
                if self
                    .pointer()
                    .await?
                    .as_ref()
                    .is_some_and(|(found, _)| *found == pointer)
                {
                    return Ok(bundle.clone());
                }
                match outcome {
                    WriteOutcome::PreconditionFailed => Err(Error::CursorConflict),
                    _ => Err(Error::Unavailable),
                }
            })
            .await
    }
}
fn canonical<T: serde::de::DeserializeOwned + Serialize>(bytes: &[u8]) -> Result<T> {
    let value: T = serde_json::from_slice(bytes).map_err(|_| Error::Corrupt)?;
    if serde_json::to_vec(&value).map_err(|_| Error::Corrupt)? != bytes {
        return Err(Error::Corrupt);
    }
    Ok(value)
}
