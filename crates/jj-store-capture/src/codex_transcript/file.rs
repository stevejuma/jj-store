use super::{CaptureProfile, CaptureResult};
use crate::codex_discovery::PhysicalId;
pub(super) use lapis_ai_capture::source_file::uuid_label;
pub(crate) use lapis_ai_capture::source_file::{FileStamp as Stamp, unchanged};
use lapis_ai_capture::source_file::{inspect_codex, open_regular};
use sha2::{Digest as _, Sha256};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub(crate) struct Header {
    pub session: String,
    pub directory: PathBuf,
    pub witness: [u8; 32],
}

pub(crate) fn inspect_bound(
    profile: CaptureProfile,
    path: &Path,
    identity: Option<PhysicalId>,
) -> CaptureResult<(File, Header, Stamp)> {
    if profile.is_cursor() {
        let (file, observed) = open_regular(path, identity)?;
        let binding = crate::cursor_binding::read(profile, path)?;
        let mut witness = Sha256::new_with_prefix(b"jj-store/cursor-transcript-file/v1\0");
        witness.update(observed.device.to_be_bytes());
        witness.update(observed.inode.to_be_bytes());
        witness.update(binding.witness());
        if !unchanged(&file, path, &observed)? {
            return Err("Cursor transcript changed during inspection".into());
        }
        return Ok((
            file,
            Header {
                session: binding.session,
                directory: binding.directory,
                witness: witness.finalize().into(),
            },
            observed,
        ));
    }
    let (file, header, observed) = inspect_codex(profile.into(), path, identity)?;
    let mut witness = Sha256::new_with_prefix(b"jj-store/codex-transcript-file/v1\0");
    witness.update(observed.device.to_be_bytes());
    witness.update(observed.inode.to_be_bytes());
    witness.update(&header.bytes);
    Ok((
        file,
        Header {
            session: header.session,
            directory: header.directory,
            witness: witness.finalize().into(),
        },
        observed,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn replaced_physical_identity_is_rejected_before_header_parsing() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("session.jsonl");
        fs::write(&path, b"retained original file\n").unwrap();
        let identity = PhysicalId::from(&fs::metadata(&path).unwrap()).unwrap();
        fs::rename(&path, root.path().join("old-file")).unwrap();
        fs::write(&path, b"rejected private replacement\n").unwrap();
        let error = inspect_bound(CaptureProfile::CodexCliJsonlV1, &path, Some(identity))
            .err()
            .unwrap();
        assert_eq!(error.to_string(), "transcript source identity changed");
        let unbound = inspect_bound(CaptureProfile::CodexCliJsonlV1, &path, None)
            .err()
            .unwrap();
        assert_eq!(unbound.to_string(), "invalid transcript header");
    }

    #[test]
    fn codex_witness_retains_exact_header_bytes_and_native_file_identity() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("session.jsonl");
        // Unknown keys and whitespace must remain in the pre-existing witness.
        let bytes = br#"{ "type":"session_meta", "payload":{"id":"12345678-1234-1234-1234-123456789abc","cwd":"/private/repo","source":"cli","originator":"codex_cli_rs","cli_version":"0.156.1","extra":true} }
"#;
        fs::write(&path, bytes).unwrap();
        let identity = PhysicalId::from(&fs::metadata(&path).unwrap()).unwrap();
        let (_, header, _) =
            inspect_bound(CaptureProfile::CodexCliJsonlV1, &path, Some(identity)).unwrap();
        let mut canonical = b"jj-store/codex-transcript-file/v1\0".to_vec();
        canonical.extend(identity.device.to_be_bytes());
        canonical.extend(identity.inode.to_be_bytes());
        canonical.extend(bytes);
        assert_eq!(header.witness, <[u8; 32]>::from(Sha256::digest(&canonical)));
        let normalized =
            serde_json::to_vec(&serde_json::from_slice::<serde_json::Value>(bytes).unwrap())
                .unwrap();
        canonical.truncate(canonical.len() - bytes.len());
        canonical.extend(normalized);
        assert_ne!(header.witness, <[u8; 32]>::from(Sha256::digest(&canonical)));
    }
}
