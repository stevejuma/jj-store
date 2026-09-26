//! Checked, immutable upload spools. The file path is never exposed to callers.
use crate::{Error, Reader, Result};
use sha2::{Digest as _, Sha256};
use tokio::io::{AsyncRead, AsyncReadExt as _, AsyncWriteExt as _};

pub const MAX_SINGLE_PUT: u64 = 5 * 1024 * 1024 * 1024;
pub struct StagedUpload {
    file: tempfile::NamedTempFile,
    len: u64,
    digest: [u8; 32],
}
impl std::fmt::Debug for StagedUpload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StagedUpload")
            .field("len", &self.len)
            .finish_non_exhaustive()
    }
}
impl StagedUpload {
    pub async fn from_reader(mut reader: impl AsyncRead + Unpin, limit: u64) -> Result<Self> {
        if limit > MAX_SINGLE_PUT {
            return Err(Error::Invalid("single PUT limit exceeds 5 GiB"));
        }
        let file = tempfile::NamedTempFile::new()?;
        let mut writer = tokio::fs::File::from_std(file.reopen()?);
        let mut buffer = vec![0; 64 * 1024];
        let mut len = 0u64;
        let mut hash = Sha256::new();
        loop {
            let n = reader.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            len = len.checked_add(n as u64).ok_or(Error::Limit)?;
            if len > limit {
                return Err(Error::Limit);
            }
            hash.update(&buffer[..n]);
            writer.write_all(&buffer[..n]).await?;
        }
        writer.flush().await?;
        Ok(Self {
            file,
            len,
            digest: hash.finalize().into(),
        })
    }
    pub async fn from_bytes(bytes: &[u8], limit: u64) -> Result<Self> {
        Self::from_reader(bytes, limit).await
    }
    pub fn len(&self) -> u64 {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn sha256(&self) -> &[u8; 32] {
        &self.digest
    }
    pub fn reader(&self) -> Result<Reader> {
        Ok(Box::pin(tokio::fs::File::from_std(self.file.reopen()?)))
    }
}
