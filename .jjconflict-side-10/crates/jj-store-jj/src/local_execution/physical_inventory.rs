//! Original physical input only; never installation progress or a write proof.
use super::*;
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::BTreeSet;
mod validation;
pub const MAX_INVENTORY_BYTES: usize = 8 * 1024 * 1024;
const MAX_RECORDS: usize = 4097;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalDirectoryV1 {
    pub path: String,
    /// Exact existing scanner frame: dev, ino, len, mode, mtime seconds/nanos,
    /// ctime seconds/nanos. These are observations, never OS write instructions.
    pub stamp: [u64; 8],
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalFileV1 {
    pub path: String,
    /// Existing content/mode/identity/timestamp fingerprint, including symlinks.
    pub fingerprint: m::RecordDigest,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalInventoryV1 {
    version: u32,
    repository: m::RepositoryId,
    generation: u64,
    workspace: m::RecordDigest,
    fingerprint: m::RecordDigest,
    directories: Vec<PhysicalDirectoryV1>,
    files: Vec<PhysicalFileV1>,
}
impl PhysicalInventoryV1 {
    /// Trusted scanner supplies complete rows; data construction grants no proof.
    pub fn new(
        repository: m::RepositoryId,
        generation: u64,
        workspace: m::RecordDigest,
        directories: Vec<PhysicalDirectoryV1>,
        files: Vec<PhysicalFileV1>,
    ) -> Result<Self> {
        let mut value = Self {
            version: 1,
            repository,
            generation,
            workspace,
            fingerprint: m::RecordDigest::new([0; 32]),
            directories,
            files,
        };
        value.structure()?;
        value.fingerprint = value.calculate();
        value.encode()?;
        Ok(value)
    }
    pub fn fingerprint(&self) -> m::RecordDigest {
        self.fingerprint
    }
    pub fn directories(&self) -> impl Iterator<Item = &PhysicalDirectoryV1> {
        self.directories.iter()
    }
    pub fn files(&self) -> impl Iterator<Item = &PhysicalFileV1> {
        self.files.iter()
    }
    pub fn check_selection(
        &self,
        binding: &RecoveryBinding,
        expected: m::RecordDigest,
    ) -> Result<()> {
        self.encode()?;
        if self.repository != binding.repository
            || self.generation != binding.generation
            || self.workspace != binding.physical_workspace
            || self.fingerprint != expected
        {
            return Err(Error::EpochChanged);
        }
        Ok(())
    }
    /// Stable original scanner hash; canonical inventory IDs use a separate domain.
    fn calculate(&self) -> m::RecordDigest {
        fn frame(hash: &mut Sha256, bytes: &[u8]) {
            hash.update((bytes.len() as u64).to_be_bytes());
            hash.update(bytes);
        }
        let mut hash = Sha256::new_with_prefix(b"jj-store/physical-tree/v1\0");
        hash.update(self.repository.as_bytes());
        hash.update(self.generation.to_be_bytes());
        hash.update(self.workspace.as_bytes());
        for directory in &self.directories {
            hash.update(b"D");
            frame(&mut hash, directory.path.as_bytes());
            frame(
                &mut hash,
                &directory
                    .stamp
                    .into_iter()
                    .flat_map(u64::to_be_bytes)
                    .collect::<Vec<_>>(),
            );
        }
        for file in &self.files {
            hash.update(b"F");
            frame(&mut hash, file.path.as_bytes());
            hash.update(file.fingerprint.as_bytes());
        }
        m::RecordDigest::new(hash.finalize().into())
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.structure()?;
        if self.fingerprint == m::RecordDigest::new([0; 32]) || self.calculate() != self.fingerprint
        {
            return Err(Error::Invalid);
        }
        let bytes = serde_json::to_vec(self).map_err(|_| Error::Invalid)?;
        if bytes.len() > MAX_INVENTORY_BYTES {
            return Err(Error::Limit);
        }
        Ok(bytes)
    }
    pub fn id(&self) -> Result<m::RecordDigest> {
        let mut hash = Sha256::new_with_prefix(b"jj-store/physical-inventory/v1\0");
        hash.update(self.encode()?);
        Ok(m::RecordDigest::new(hash.finalize().into()))
    }
    pub fn decode(expected: m::RecordDigest, bytes: &[u8]) -> Result<Self> {
        if bytes.len() > MAX_INVENTORY_BYTES {
            return Err(Error::Limit);
        }
        let value: Self = serde_json::from_slice(bytes).map_err(|_| Error::Invalid)?;
        if value.encode()? != bytes || value.id()? != expected {
            return Err(Error::Invalid);
        }
        Ok(value)
    }
}
