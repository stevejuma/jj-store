use super::*;
use sha2::{Digest as _, Sha256};
use std::{
    fs,
    io::{Read as _, Write as _},
    path::Path,
};
fn safe(path: &str) -> bool {
    jj_store_jj::vault_workspace::backup_path(path).file()
}
fn valid(value: &HistoryFiles, commit: &str, after: Option<&str>) -> bool {
    value.commit == commit
        && value.after.as_deref() == after
        && value.files.len() <= 8
        && value.files.iter().all(|f| {
            safe(&f.path)
                && after.is_none_or(|a| f.path.as_str() > a)
                && f.text.as_ref().is_none_or(|t| t.len() <= 1024 * 1024)
        })
        && value.files.windows(2).all(|p| p[0].path < p[1].path)
        && value
            .next
            .as_ref()
            .is_none_or(|n| value.files.last().is_some_and(|f| &f.path == n))
}
fn read(path: &Path) -> Option<HistoryFiles> {
    let meta = fs::symlink_metadata(path).ok()?;
    if !meta.is_file() || meta.is_symlink() || meta.len() > 16 * 1024 * 1024 {
        return None;
    }
    let mut data = Vec::new();
    fs::File::open(path)
        .ok()?
        .take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut data)
        .ok()?;
    if data.len() > 16 * 1024 * 1024 {
        return None;
    }
    serde_json::from_slice(&data).ok()
}
impl Index {
    pub(super) fn files(
        &self,
        source: &mut impl Source,
        commit: &str,
        after: Option<&str>,
    ) -> Result<HistoryFiles, String> {
        let key = format!(
            "{:x}",
            Sha256::digest(
                serde_json::to_vec(&(1, &self.namespace, commit, after))
                    .map_err(|e| e.to_string())?
            )
        );
        let directory = fs::create_dir_all(&self.directory)
            .ok()
            .and_then(|()| fs::canonicalize(&self.directory).ok())
            .filter(|p| p == &self.directory);
        let path = self.directory.join(format!("{key}.json"));
        if directory.is_some()
            && let Some(value) = read(&path).filter(|v| valid(v, commit, after))
        {
            return Ok(value);
        }
        let value = source.files(commit, after)?;
        if !valid(&value, commit, after) {
            return Err("Invalid immutable history projection".into());
        }
        if directory.is_some() {
            // Cache writes are best effort. Loss cannot fail a source read.
            let _ = (|| -> std::io::Result<()> {
                let bytes = serde_json::to_vec(&value)?;
                if bytes.len() > 16 * 1024 * 1024 || !room(&self.directory, bytes.len() as u64) {
                    return Ok(());
                }
                let mut staged = tempfile::NamedTempFile::new_in(&self.directory)?;
                staged.write_all(&bytes)?;
                staged.persist(&path).map_err(std::io::Error::other)?;
                Ok(())
            })();
        }
        Ok(value)
    }
}

// A full cache simply falls back to native objects. Never prune authoritative
// state, and never turn a cache quota into a failed history read.
fn room(directory: &Path, incoming: u64) -> bool {
    let Ok(entries) = fs::read_dir(directory) else {
        return false;
    };
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(100);
    let mut bytes = incoming;
    for (count, entry) in entries.enumerate() {
        if count >= 4096 || std::time::Instant::now() > deadline {
            return false;
        }
        let Ok(entry) = entry else {
            return false;
        };
        let Ok(meta) = fs::symlink_metadata(entry.path()) else {
            return false;
        };
        bytes = bytes.saturating_add(meta.len());
        if bytes > 256 * 1024 * 1024 {
            return false;
        }
    }
    true
}
