//! Bounded descriptor-relative listing. Never follows links or reads ordinary file bodies.
use super::{Error, FileEntry, Result};
use jj_store_jj::local_browser::working_tree::{IgnoreFile, IgnoreRules};
use rustix::fs::{self, AtFlags, FileType, Mode, OFlags};
use sha2::{Digest as _, Sha256};
use std::{
    collections::BTreeSet,
    fs::File,
    io::Read as _,
    path::Path,
    time::{Duration, Instant},
};
const DIR: OFlags = OFlags::RDONLY
    .union(OFlags::CLOEXEC)
    .union(OFlags::NOFOLLOW)
    .union(OFlags::DIRECTORY);
struct Scan<'a> {
    tracked: &'a BTreeSet<String>,
    base: Vec<Vec<u8>>,
    start: Instant,
    visited: usize,
    entries: Vec<FileEntry>,
}
pub fn list(root: &Path, tracked: &BTreeSet<String>, base: Vec<Vec<u8>>) -> Result<Vec<FileEntry>> {
    let dir: File = fs::open(root, DIR, Mode::empty())
        .map_err(|_| Error::Unavailable)?
        .into();
    let mut scan = Scan {
        tracked,
        base,
        start: Instant::now(),
        visited: 0,
        entries: vec![],
    };
    scan.walk(&dir, "", vec![], 0)?;
    scan.entries.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(scan.entries)
}
impl Scan<'_> {
    fn walk(
        &mut self,
        dir: &File,
        prefix: &str,
        mut ignores: Vec<IgnoreFile>,
        depth: usize,
    ) -> Result<()> {
        if depth > 64 || self.start.elapsed() > Duration::from_secs(2) {
            return Err(Error::Limit);
        }
        let before = crate::working_copy::stamp(&dir.metadata().map_err(|_| Error::Unavailable)?);
        match fs::openat(
            dir,
            ".gitignore",
            OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NONBLOCK,
            Mode::empty(),
        ) {
            Ok(fd) => {
                let file = File::from(fd);
                if !file.metadata().map_err(|_| Error::Unavailable)?.is_file() {
                    return Err(Error::Unavailable);
                }
                let mut bytes = vec![];
                file.take(65537)
                    .read_to_end(&mut bytes)
                    .map_err(|_| Error::Unavailable)?;
                if bytes.len() > 65536 {
                    return Err(Error::Limit);
                }
                ignores.push(IgnoreFile {
                    path: format!("{prefix}.gitignore"),
                    bytes,
                });
            }
            Err(rustix::io::Errno::NOENT) => {}
            Err(_) => return Err(Error::Unavailable),
        }
        let rules =
            IgnoreRules::new(self.base.clone(), ignores.clone()).map_err(|_| Error::Unavailable)?;
        for entry in fs::Dir::read_from(dir).map_err(|_| Error::Unavailable)? {
            self.visited += 1;
            if self.visited > 8192
                || self.entries.len() >= 4096
                || self.start.elapsed() > Duration::from_secs(2)
            {
                return Err(Error::Limit);
            }
            let entry = entry.map_err(|_| Error::Unavailable)?;
            let name = entry.file_name().to_str().map_err(|_| Error::Invalid)?;
            if matches!(name, "." | "..")
                || name.eq_ignore_ascii_case(".git")
                || name.eq_ignore_ascii_case(".jj")
            {
                continue;
            }
            let path = format!("{prefix}{name}");
            if !jj_store_jj::agent_workspaces::inspection::safe_path(&path) {
                return Err(Error::Invalid);
            }
            let info =
                fs::statat(dir, name, AtFlags::SYMLINK_NOFOLLOW).map_err(|_| Error::Stale)?;
            let is_dir = FileType::from_raw_mode(info.st_mode) == FileType::Directory;
            let tracked = self.tracked.contains(&path)
                || (is_dir
                    && self
                        .tracked
                        .iter()
                        .any(|p| p.starts_with(&format!("{path}/"))));
            let check = if is_dir {
                format!("{path}/__entry__")
            } else {
                path.clone()
            };
            let ignored = rules.ignored(&check).map_err(|_| Error::Unavailable)?;
            if !tracked && ignored {
                continue;
            }
            if is_dir {
                let child: File = fs::openat(dir, name, DIR, Mode::empty())
                    .map_err(|_| Error::Stale)?
                    .into();
                self.walk(&child, &format!("{path}/"), ignores.clone(), depth + 1)?;
            } else {
                let kind = match FileType::from_raw_mode(info.st_mode) {
                    FileType::RegularFile => "file",
                    FileType::Symlink => "symlink",
                    _ => "unsupported",
                };
                let mut hash = Sha256::new_with_prefix(b"jj-store/workspace-list-entry/v1\0");
                hash.update(path.as_bytes());
                for n in [
                    i128::from(info.st_dev) as u64,
                    info.st_ino,
                    info.st_mode as u64,
                    info.st_size as u64,
                    info.st_mtime as u64,
                    i128::from(info.st_mtime_nsec) as u64,
                    info.st_ctime as u64,
                    i128::from(info.st_ctime_nsec) as u64,
                ] {
                    hash.update(n.to_be_bytes());
                }
                self.entries.push(FileEntry {
                    path,
                    kind: kind.into(),
                    bytes: info.st_size.max(0) as u64,
                    fingerprint: hex::encode(hash.finalize()),
                    status: None,
                    additions: None,
                    deletions: None,
                });
            }
        }
        if before != crate::working_copy::stamp(&dir.metadata().map_err(|_| Error::Unavailable)?) {
            return Err(Error::Stale);
        }
        Ok(())
    }
}
