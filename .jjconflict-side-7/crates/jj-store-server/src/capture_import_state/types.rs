use super::hex_codec::{hex16, hex32};
use super::*;
use jj_store_capture::profile::CaptureProfile;

/// Owned server input. Actor is already authenticated; never deserialize from HTTP.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureImportIntent {
    pub version: u32,
    #[serde(with = "hex16")]
    pub generation: [u8; 16],
    pub principal: PrincipalId,
    pub request: RequestId,
    pub expected_revision: u64,
    pub selector: String,
}
impl CaptureImportIntent {
    pub(crate) fn validate(&self) -> Result<()> {
        let (runtime, ordinal) = self.selector.split_once(':').ok_or(Error::Invalid)?;
        if self.version != 1
            || self.generation == [0; 16]
            || self.principal.as_bytes() == &[0; 16]
            || self.request.as_bytes() == &[0; 16]
            || runtime.len() != 32
            || ordinal.len() != 16
            || runtime
                .bytes()
                .chain(ordinal.bytes())
                .any(|b| !b.is_ascii_digit() && !(b'a'..=b'f').contains(&b))
            || runtime == "0".repeat(32)
            || u64::from_str_radix(ordinal, 16).map_err(|_| Error::Invalid)? == 0
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
}
/// Opaque immutable metadata. Digests are not current source/directory proofs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureImportSourceBinding {
    pub profile: CaptureProfile,
    pub product_version: String,
    #[serde(with = "hex32")]
    pub physical_digest: [u8; 32],
    #[serde(with = "hex32")]
    pub header_digest: [u8; 32],
    #[serde(with = "hex32")]
    pub session_digest: [u8; 32],
    #[serde(with = "hex32")]
    pub prefix_digest: [u8; 32],
    pub complete_byte_offset: u64,
    pub complete_records: u64,
}
impl CaptureImportSourceBinding {
    pub(crate) fn from_codex(b: jj_store_capture::codex_import::CodexImportBinding) -> Self {
        Self {
            profile: b.profile,
            product_version: b.profile.product_version().into(),
            physical_digest: b.physical_digest,
            header_digest: b.header_digest,
            session_digest: b.session_digest,
            prefix_digest: b.prefix_digest,
            complete_byte_offset: b.complete_byte_offset,
            complete_records: b.complete_records,
        }
    }
    pub(crate) fn codex(&self) -> jj_store_capture::codex_import::CodexImportBinding {
        jj_store_capture::codex_import::CodexImportBinding {
            profile: self.profile,
            physical_digest: self.physical_digest,
            header_digest: self.header_digest,
            session_digest: self.session_digest,
            prefix_digest: self.prefix_digest,
            complete_byte_offset: self.complete_byte_offset,
            complete_records: self.complete_records,
        }
    }
    pub(super) fn validate(&self) -> Result<()> {
        let position_valid = match self.profile {
            CaptureProfile::CodexCliJsonlV1 | CaptureProfile::CodexDesktopJsonlV1 => {
                self.complete_records != 0 && self.complete_byte_offset >= self.complete_records
            }
            CaptureProfile::OpencodeReadOnlyApiV1 => {
                self.complete_byte_offset == 0
                    && self.complete_records <= 131072
                    && (self.complete_records != 0
                        || self.prefix_digest
                            == jj_store_capture::opencode_checkpoint::initial_message_prefix(
                                self.header_digest,
                            ))
            }
            _ => false,
        };
        if self.product_version != self.profile.product_version()
            || [
                self.physical_digest,
                self.header_digest,
                self.session_digest,
                self.prefix_digest,
            ]
            .contains(&[0; 32])
            || !position_valid
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureImportReceipt {
    /// Journal selection order, independent of pause policy revisions.
    pub ordinal: u64,
    pub intent: CaptureImportIntent,
    pub source: CaptureImportSourceBinding,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaptureImportSelection {
    pub receipt: CaptureImportReceipt,
    pub replayed: bool,
}
