//! Bounded owner-only reader cursor journal. Contains no source bodies or paths.
use crate::{
    CaptureResult,
    agent_capture_files::{recover_pending, restrict_directory, sync_dir},
    reader_progress::ReaderProgressV1,
};
use jj_agent::{IntegrationId, SourceInstanceId};
use jj_store_model::RepositoryId;
use std::{
    fs::{self, File},
    io::{Read as _, Write as _},
    path::{Path, PathBuf},
};
pub(crate) const MAX_PROGRESS: usize = 4096;
const MAX_BYTES: u64 = 8 * 1024;
fn directory(root: &Path) -> PathBuf {
    root.join("reader-progress")
}
fn filename(
    repository: RepositoryId,
    integration: IntegrationId,
    source: SourceInstanceId,
    generation: u64,
) -> String {
    format!(
        "{repository}-{}-{source}-{generation}.json",
        integration as u8
    )
}
fn check_directory(root: &Path) -> CaptureResult<bool> {
    match fs::symlink_metadata(directory(root)) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
        Ok(metadata) => {
            if !metadata.is_dir() || metadata.file_type().is_symlink() {
                return Err("reader progress must be a private directory".into());
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt as _;
                if metadata.permissions().mode() & 0o077 != 0 {
                    return Err("reader progress directory permissions".into());
                }
            }
            Ok(true)
        }
    }
}
fn read(path: &Path) -> CaptureResult<ReaderProgressV1> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > MAX_BYTES {
        return Err("invalid reader progress file".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err("reader progress file permissions".into());
        }
    }
    let mut bytes = Vec::new();
    File::open(path)?
        .take(MAX_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err("reader progress byte limit exceeded".into());
    }
    let progress: ReaderProgressV1 =
        serde_json::from_slice(&bytes).map_err(|_| "invalid reader progress record")?;
    progress.validate()?;
    if path.file_name().and_then(|name| name.to_str())
        != Some(
            filename(
                progress.repository,
                progress.cursor.integration,
                progress.cursor.source_instance_id,
                progress.cursor.generation,
            )
            .as_str(),
        )
    {
        return Err("reader progress filename mismatch".into());
    }
    Ok(progress)
}
pub(crate) fn records(root: &Path) -> CaptureResult<Vec<ReaderProgressV1>> {
    if !check_directory(root)? {
        return Ok(Vec::new());
    }
    let mut records = Vec::new();
    for entry in fs::read_dir(directory(root))? {
        if records.len() >= MAX_PROGRESS {
            return Err("reader progress capacity exhausted".into());
        }
        records.push(read(&entry?.path())?);
    }
    Ok(records)
}
pub(crate) fn recover(root: &Path) -> CaptureResult<()> {
    recover_temporary(root)?;
    records(root)?;
    Ok(())
}
pub(crate) fn recover_temporary(root: &Path) -> CaptureResult<()> {
    if check_directory(root)? {
        recover_pending(&directory(root))?;
    }
    Ok(())
}
pub(crate) fn load(
    root: &Path,
    repository: RepositoryId,
    integration: IntegrationId,
    source: SourceInstanceId,
    generation: u64,
) -> CaptureResult<Option<ReaderProgressV1>> {
    if !check_directory(root)? {
        return Ok(None);
    }
    let path = directory(root).join(filename(repository, integration, source, generation));
    match fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.into()),
        Ok(_) => read(&path).map(Some),
    }
}
pub(crate) fn persist(root: &Path, progress: &ReaderProgressV1) -> CaptureResult<()> {
    let directory = directory(root);
    if !check_directory(root)? {
        fs::create_dir(&directory)?;
        restrict_directory(&directory)?;
        sync_dir(root)?;
    }
    let bytes = serde_json::to_vec(progress)?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err("reader progress byte limit exceeded".into());
    }
    let path = directory.join(filename(
        progress.repository,
        progress.cursor.integration,
        progress.cursor.source_instance_id,
        progress.cursor.generation,
    ));
    let mut temporary = tempfile::NamedTempFile::new_in(&directory)?;
    temporary.write_all(&bytes)?;
    temporary.as_file().sync_all()?;
    temporary.persist(&path).map_err(|error| error.error)?;
    sync_dir(&directory)
}
