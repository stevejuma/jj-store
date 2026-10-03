use super::{CaptureProfile, CaptureResult, MAX_RECORD_BYTES};
use crate::codex_discovery::PhysicalId;
use sha2::{Digest as _, Sha256};
use std::{
    fs::{self, File, Metadata},
    io::{BufRead as _, BufReader, Read as _},
    path::{Path, PathBuf},
    time::SystemTime,
};
#[derive(Clone)]
pub(crate) struct Header {
    pub session: String,
    pub directory: PathBuf,
    pub witness: [u8; 32],
}
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Stamp {
    pub len: u64,
    modified: SystemTime,
    device: u64,
    inode: u64,
}
fn stamp(metadata: &Metadata) -> CaptureResult<Stamp> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        Ok(Stamp {
            len: metadata.len(),
            modified: metadata.modified()?,
            device: metadata.dev(),
            inode: metadata.ino(),
        })
    }
    #[cfg(not(unix))]
    {
        let _ = metadata;
        Err("physical transcript identity is unavailable on this platform".into())
    }
}
pub(crate) fn inspect_bound(
    profile: CaptureProfile,
    path: &Path,
    identity: Option<PhysicalId>,
) -> CaptureResult<(File, Header, Stamp)> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err("transcript must be a regular file".into());
    }
    let file = File::open(path)?;
    let observed = stamp(&file.metadata()?)?;
    if observed != stamp(&metadata)? {
        return Err("transcript changed during open".into());
    }
    if identity.is_some_and(|id| id.device != observed.device || id.inode != observed.inode) {
        return Err("transcript source identity changed".into());
    }
    let mut bytes = Vec::new();
    BufReader::new((&file).take(MAX_RECORD_BYTES as u64 + 1)).read_until(b'\n', &mut bytes)?;
    if bytes.len() > MAX_RECORD_BYTES || bytes.last() != Some(&b'\n') {
        return Err("complete bounded transcript header required".into());
    }
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|_| "invalid transcript header")?;
    let payload = &value["payload"];
    let surface_matches = match profile {
        CaptureProfile::CodexCliJsonlV1 => {
            matches!(payload["source"].as_str(), Some("cli" | "exec"))
                && matches!(
                    payload["originator"].as_str(),
                    Some("codex_cli_rs" | "codex_exec" | "Codex Desktop")
                )
        }
        CaptureProfile::CodexDesktopJsonlV1 => {
            payload["source"].as_str() == Some("vscode")
                && payload["originator"].as_str() == Some("Codex Desktop")
        }
        _ => return Err("profile does not select Codex JSONL".into()),
    };
    if value["type"].as_str() != Some("session_meta")
        || payload["cli_version"].as_str() != Some(profile.product_version())
        || !surface_matches
    {
        return Err("unsupported transcript product surface or version".into());
    }
    let session = payload["id"]
        .as_str()
        .ok_or("transcript session identity missing")?;
    if !uuid_label(session) {
        return Err("invalid transcript session identity".into());
    }
    let directory = PathBuf::from(
        payload["cwd"]
            .as_str()
            .ok_or("transcript directory missing")?,
    );
    if !directory.is_absolute() {
        return Err("transcript directory must be absolute".into());
    }
    let mut witness = Sha256::new_with_prefix(b"jj-store/codex-transcript-file/v1\0");
    witness.update(observed.device.to_be_bytes());
    witness.update(observed.inode.to_be_bytes());
    witness.update(&bytes);
    let header = Header {
        session: session.into(),
        directory,
        witness: witness.finalize().into(),
    };
    if !unchanged(&file, path, &observed)? {
        return Err("transcript changed during inspection".into());
    }
    Ok((file, header, observed))
}
pub(crate) fn unchanged(file: &File, path: &Path, observed: &Stamp) -> CaptureResult<bool> {
    let path_metadata = fs::symlink_metadata(path)?;
    Ok(!path_metadata.file_type().is_symlink()
        && path_metadata.is_file()
        && stamp(&file.metadata()?)? == *observed
        && stamp(&path_metadata)? == *observed)
}
pub(super) fn uuid_label(value: &str) -> bool {
    value.len() == 36
        && value.bytes().enumerate().all(|(i, b)| {
            if [8, 13, 18, 23].contains(&i) {
                b == b'-'
            } else {
                b.is_ascii_hexdigit()
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
