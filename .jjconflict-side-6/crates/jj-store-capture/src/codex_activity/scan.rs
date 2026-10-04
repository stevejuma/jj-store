use crate::{
    CaptureResult,
    codex_transcript::{
        MAX_BATCH_RECORDS, MAX_RECORD_BYTES, MAX_SCAN_BYTES, context::DirectoryHistory,
    },
};
use sha2::{Digest as _, Sha256};
use std::{
    fs::File,
    io::{Read as _, Seek as _},
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub(super) struct Checkpoint {
    pub(super) offset: u64,
    hash: Sha256,
    history: DirectoryHistory,
    pub(super) active: Option<PathBuf>,
}
impl Checkpoint {
    pub(super) fn digest(&self) -> [u8; 32] {
        self.hash.clone().finalize().into()
    }
}
pub(super) struct Scan {
    pub(super) complete: Checkpoint,
    offset: u64,
    hash: Sha256,
    line: Vec<u8>,
    oversized: bool,
}
impl Scan {
    pub(super) fn release_eof_bytes(&mut self) {
        // A newline cannot arrive without a metadata change. On that change the
        // existing discard_partial path restores the last complete checkpoint.
        self.line = Vec::new();
    }
    pub(super) fn reached(&self, offset: u64) -> bool {
        self.offset >= offset
    }
    pub(super) fn new(initial: &Path) -> Self {
        Self {
            complete: Checkpoint {
                offset: 0,
                hash: Sha256::new(),
                history: DirectoryHistory::new(initial),
                active: None,
            },
            offset: 0,
            hash: Sha256::new(),
            line: Vec::new(),
            oversized: false,
        }
    }
    pub(super) fn discard_partial(&mut self) {
        self.offset = self.complete.offset;
        self.hash = self.complete.hash.clone();
        self.line.clear();
        self.oversized = false;
    }
    pub(super) fn read(&mut self, file: &mut File, cutoff: u64) -> CaptureResult<bool> {
        file.seek(std::io::SeekFrom::Start(self.offset))?;
        let mut budget = MAX_SCAN_BYTES;
        let mut records = 0;
        let mut buffer = [0; 64 * 1024];
        while budget > 0 && records < MAX_BATCH_RECORDS {
            let limit = budget.min(buffer.len());
            let amount = file.read(&mut buffer[..limit])?;
            if amount == 0 {
                return Ok(true);
            }
            budget -= amount;
            let mut consumed = 0;
            while consumed < amount {
                let rest = &buffer[consumed..amount];
                let length = rest
                    .iter()
                    .position(|b| *b == b'\n')
                    .map_or(rest.len(), |i| i + 1);
                let part = &rest[..length];
                self.offset = self
                    .offset
                    .checked_add(length as u64)
                    .ok_or("Codex activity offset limit")?;
                self.hash.update(part);
                if !self.oversized {
                    if self.line.len() + part.len() > MAX_RECORD_BYTES {
                        self.line.clear();
                        self.oversized = true;
                    } else {
                        self.line.extend_from_slice(part);
                    }
                }
                consumed += length;
                if part.last() == Some(&b'\n') {
                    let bytes = (!self.oversized).then_some(self.line.as_slice());
                    self.complete.history.observe(bytes);
                    if self.complete.offset >= cutoff
                        && let Some(directory) = self.complete.history.context.directory()
                        && bytes
                            .and_then(|v| serde_json::from_slice::<serde_json::Value>(v).ok())
                            .is_some_and(|v| {
                                v["type"] == "event_msg" && v["payload"]["type"] == "task_started"
                            })
                    {
                        self.complete.active = Some(directory.to_owned());
                    }
                    self.complete.offset = self.offset;
                    self.complete.hash = self.hash.clone();
                    self.line.clear();
                    self.oversized = false;
                    records += 1;
                    if records >= MAX_BATCH_RECORDS {
                        return Ok(false);
                    }
                }
            }
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eof_compaction_releases_allocation_without_rewinding_the_complete_prefix() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let bytes = [
            b"{\"type\":\"session_meta\"}\n".as_slice(),
            &vec![b'x'; 900_000],
        ]
        .concat();
        std::fs::write(file.path(), &bytes).unwrap();
        let mut scan = Scan::new(Path::new("/workspace/a"));
        let mut reader = File::open(file.path()).unwrap();
        assert!(scan.read(&mut reader, bytes.len() as u64).unwrap());
        assert!(scan.line.capacity() >= 900_000);
        let offset = scan.complete.offset;
        let prefix = scan.complete.digest();
        scan.release_eof_bytes();
        assert_eq!(scan.line.capacity(), 0);
        assert!(scan.reached(bytes.len() as u64));
        assert_eq!(scan.complete.offset, offset);
        assert_eq!(scan.complete.digest(), prefix);
        scan.discard_partial();
        assert!(!scan.reached(bytes.len() as u64));
        assert!(scan.complete.active.is_none());
    }
}
