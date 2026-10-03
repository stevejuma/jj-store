//! One bounded private catalog, atomically replaced under the spool mutation lock.
use super::*;
use crate::agent_capture_files::{recover_pending, restrict_directory, sync_dir};
use std::{
    fs::{self, File},
    io::{Read as _, Write as _},
    path::{Path, PathBuf},
};
const MAX_BYTES: u64 = 2 * 1024 * 1024;
fn directory(root: &Path) -> PathBuf {
    root.join("reader-publication")
}
fn check_directory(root: &Path) -> CaptureResult<bool> {
    match fs::symlink_metadata(directory(root)) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e.into()),
        Ok(metadata) => {
            if !metadata.is_dir() || metadata.file_type().is_symlink() {
                return Err("invalid publication directory".into());
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt as _;
                if metadata.permissions().mode() & 0o077 != 0 {
                    return Err("publication directory permissions".into());
                }
            }
            Ok(true)
        }
    }
}
pub(super) fn load(root: &Path) -> CaptureResult<Option<Catalog>> {
    read(root, true)
}
pub(super) fn inspect(root: &Path) -> CaptureResult<Option<Catalog>> {
    read(root, false)
}
fn read(root: &Path, recover: bool) -> CaptureResult<Option<Catalog>> {
    if !check_directory(root)? {
        return Ok(None);
    }
    let directory = directory(root);
    if recover {
        recover_pending(&directory)?;
    }
    if fs::read_dir(&directory)?
        .any(|entry| entry.is_err() || entry.is_ok_and(|e| e.file_name() != "catalog.json"))
    {
        return Err("unexpected publication catalog entry".into());
    }
    let path = directory.join("catalog.json");
    let metadata = fs::symlink_metadata(&path)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > MAX_BYTES {
        return Err("invalid publication catalog file".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err("publication catalog permissions".into());
        }
    }
    let mut bytes = Vec::new();
    File::open(path)?
        .take(MAX_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err("publication catalog byte limit exceeded".into());
    }
    let catalog: Catalog =
        serde_json::from_slice(&bytes).map_err(|_| "invalid publication catalog")?;
    catalog.validate()?;
    Ok(Some(catalog))
}
pub(super) fn persist(root: &Path, catalog: &Catalog) -> CaptureResult<()> {
    catalog.validate()?;
    let bytes = serde_json::to_vec(catalog)?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err("publication catalog byte limit exceeded".into());
    }
    let directory = directory(root);
    if !check_directory(root)? {
        fs::create_dir(&directory)?;
        restrict_directory(&directory)?;
        sync_dir(root)?;
    }
    let mut temporary = tempfile::NamedTempFile::new_in(&directory)?;
    temporary.write_all(&bytes)?;
    temporary.as_file().sync_all()?;
    temporary
        .persist(directory.join("catalog.json"))
        .map_err(|e| e.error)?;
    sync_dir(&directory)
}
