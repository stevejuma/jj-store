use crate::policy;
use jj_store_model::RepositoryId;
use jj_store_service::storage_management::*;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write as _,
    path::{Path, PathBuf},
};
const MAX_BYTES: u64 = 4 * 1024 * 1024;
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Entry {
    pub report: StorageOperation,
    pub capture: StorageCapture,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Journal {
    version: u32,
    repository: RepositoryId,
    pub entries: Vec<Entry>,
}
fn path(root: &Path, repo: RepositoryId) -> PathBuf {
    root.join(format!("storage-{repo}.json"))
}
pub(super) fn read(root: &Path, repo: RepositoryId) -> Result<Journal, Error> {
    let path = path(root, repo);
    if !path.try_exists().map_err(|_| Error::Unavailable)? {
        return Ok(Journal {
            version: 1,
            repository: repo,
            entries: Vec::new(),
        });
    }
    let bytes = policy::read_regular(&path, MAX_BYTES).map_err(|_| Error::Unavailable)?;
    let journal: Journal = serde_json::from_slice(&bytes).map_err(|_| Error::Unavailable)?;
    if journal.version != 1
        || journal.repository != repo
        || journal.entries.len() > 128
        || journal.entries.iter().any(|e| {
            e.report.target.repository != repo
                || e.report.target != e.capture.position
                || e.capture.validate(repo).is_err()
        })
        || journal
            .entries
            .iter()
            .map(|e| e.report.request_id)
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != journal.entries.len()
    {
        return Err(Error::Unavailable);
    }
    Ok(journal)
}
pub(super) fn save(root: &Path, repo: RepositoryId, journal: &Journal) -> Result<(), Error> {
    let bytes = serde_json::to_vec(journal).map_err(|_| Error::Unavailable)?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err(Error::Limit);
    }
    let path = path(root, repo);
    policy::outside_checkout(&path).map_err(|_| Error::Unavailable)?;
    if let Ok(meta) = fs::symlink_metadata(&path)
        && (!meta.is_file() || meta.file_type().is_symlink())
    {
        return Err(Error::Unavailable);
    }
    let mut tmp = tempfile::NamedTempFile::new_in(root).map_err(|_| Error::Unavailable)?;
    tmp.write_all(&bytes).map_err(|_| Error::Unavailable)?;
    tmp.as_file().sync_all().map_err(|_| Error::Unavailable)?;
    tmp.persist(path).map_err(|_| Error::Unknown)?;
    #[cfg(unix)]
    fs::File::open(root)
        .and_then(|f| f.sync_all())
        .map_err(|_| Error::Unknown)?;
    Ok(())
}
