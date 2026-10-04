use super::*;
use sha2::{Digest as _, Sha256};

pub(super) struct Scan {
    pub offset: u64,
    pub context: ContextReconstruction,
    pub prior: Option<PathBuf>,
    selected_offset: Option<u64>,
    hash: Sha256,
    complete_hash: Sha256,
    complete_byte_offset: u64,
    complete_records: u64,
}
impl Scan {
    pub fn new(header: &Path, expected: Option<CodexImportBinding>) -> Self {
        let context = ContextReconstruction::new(header);
        let prior = context.history.context.directory().map(Path::to_owned);
        let hash = Sha256::new_with_prefix(b"jj-store/codex-import/prefix/v1\0");
        Self {
            offset: 0,
            context,
            prior,
            selected_offset: expected.map(|e| e.complete_byte_offset),
            complete_hash: hash.clone(),
            hash,
            complete_byte_offset: 0,
            complete_records: 0,
        }
    }
    pub fn feed(&mut self, bytes: &[u8]) {
        let prefix_len = self.selected_offset.map_or(bytes.len(), |end| {
            end.saturating_sub(self.offset).min(bytes.len() as u64) as usize
        });
        let mut consumed = 0;
        for part in bytes[..prefix_len].split_inclusive(|b| *b == b'\n') {
            self.hash.update(part);
            consumed += part.len();
            if part.last() == Some(&b'\n') {
                self.complete_hash = self.hash.clone();
                self.complete_byte_offset = self.offset + consumed as u64;
                self.complete_records += 1;
            }
        }
        self.context.feed_observed(bytes, |context| {
            if let Some(path) = context.directory() {
                self.prior = Some(path.to_owned());
            }
        });
        self.offset += bytes.len() as u64;
    }
    pub fn binding(&self, mut identity: CodexImportBinding) -> CodexImportBinding {
        identity.prefix_digest = self.complete_hash.clone().finalize().into();
        identity.complete_byte_offset = self.complete_byte_offset;
        identity.complete_records = self.complete_records;
        identity
    }
}
