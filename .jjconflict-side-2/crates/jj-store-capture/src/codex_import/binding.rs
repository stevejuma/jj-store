use super::*;
use crate::profile::CaptureProfile;
use sha2::{Digest as _, Sha256};

/// Opaque metadata only; no native admission, source identity, paths or raw keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodexImportBinding {
    pub profile: CaptureProfile,
    pub physical_digest: [u8; 32],
    pub header_digest: [u8; 32],
    pub session_digest: [u8; 32],
    pub prefix_digest: [u8; 32],
    pub complete_byte_offset: u64,
    pub complete_records: u64,
}
impl CodexImportBinding {
    /// Body-free format comparison only; grants no current source/prefix authority.
    pub fn matches_generation_digest(&self, generation_digest: [u8; 32]) -> bool {
        generation_digest != [0; 32]
            && self.header_digest == digest(b"header/v1", &[&generation_digest])
    }
    /// Body-free metadata matching only; this grants no current source or prefix authority.
    pub fn matches_qualified(&self, qualified: &QualifiedCodexSource) -> bool {
        self.same_source(&identity(qualified))
    }
    pub(super) fn validate(&self) -> CaptureResult<()> {
        if !matches!(
            self.profile,
            CaptureProfile::CodexCliJsonlV1 | CaptureProfile::CodexDesktopJsonlV1
        ) || [
            self.physical_digest,
            self.header_digest,
            self.session_digest,
            self.prefix_digest,
        ]
        .contains(&[0; 32])
            || self.complete_records == 0
            || self.complete_byte_offset < self.complete_records
        {
            return Err("invalid Codex import binding".into());
        }
        Ok(())
    }
    pub(super) fn same_source(&self, other: &Self) -> bool {
        self.profile == other.profile
            && self.physical_digest == other.physical_digest
            && self.header_digest == other.header_digest
            && self.session_digest == other.session_digest
    }
}
pub(super) fn identity(qualified: &QualifiedCodexSource) -> CodexImportBinding {
    let physical = qualified.candidate.identity();
    CodexImportBinding {
        profile: qualified.profile,
        physical_digest: digest(
            b"physical/v1",
            &[
                &physical.device.to_be_bytes(),
                &physical.inode.to_be_bytes(),
            ],
        ),
        header_digest: digest(b"header/v1", &[&qualified.header.witness]),
        session_digest: digest(
            b"session/v1",
            &[
                qualified.profile.product_version().as_bytes(),
                qualified
                    .native_session_key()
                    .to_ascii_lowercase()
                    .as_bytes(),
            ],
        ),
        prefix_digest: [0; 32],
        complete_byte_offset: 0,
        complete_records: 0,
    }
}
fn digest(domain: &[u8], parts: &[&[u8]]) -> [u8; 32] {
    let mut hash = Sha256::new_with_prefix(b"jj-store/codex-import/");
    hash.update(domain);
    for bytes in parts {
        hash.update((bytes.len() as u64).to_be_bytes());
        hash.update(bytes);
    }
    hash.finalize().into()
}
