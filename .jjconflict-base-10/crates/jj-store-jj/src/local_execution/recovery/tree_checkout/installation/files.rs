use super::*;
use rustix::fs::{AtFlags, FileType, Mode, OFlags};
use std::{fs::File, io::Write as _};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::local_execution::recovery::tree_checkout) struct Snapshot {
    pub stamp: [u64; 9],
    pub content: m::RecordDigest,
}
fn stamp(stat: &fs::Stat) -> [u64; 9] {
    [
        stat.st_dev as u64,
        stat.st_ino,
        stat.st_mode as u64,
        stat.st_nlink as u64,
        stat.st_size as u64,
        stat.st_mtime as u64,
        stat.st_mtime_nsec as u64,
        stat.st_ctime as u64,
        stat.st_ctime_nsec as u64,
    ]
}
pub(in crate::local_execution::recovery::tree_checkout) fn directory_stamp(
    dir: &File,
) -> Result<[u64; 8]> {
    let s = fs::fstat(dir).map_err(|_| Error::Unavailable)?;
    Ok([
        s.st_dev as u64,
        s.st_ino,
        s.st_size as u64,
        s.st_mode as u64,
        s.st_mtime as u64,
        s.st_mtime_nsec as u64,
        s.st_ctime as u64,
        s.st_ctime_nsec as u64,
    ])
}
pub(in crate::local_execution::recovery::tree_checkout) fn entries(
    dir: &File,
    maximum: usize,
) -> Result<Vec<String>> {
    let mut entries = Vec::new();
    let read = fs::Dir::read_from(dir).map_err(|_| Error::Unavailable)?;
    for entry in read {
        let entry = entry.map_err(|_| Error::Unavailable)?;
        let name = entry.file_name().to_str().map_err(|_| Error::Invalid)?;
        if matches!(name, "." | "..") {
            continue;
        }
        if entries.len() >= maximum {
            return Err(Error::Limit);
        }
        entries.push(name.into());
    }
    entries.sort();
    Ok(entries)
}
pub(in crate::local_execution::recovery::tree_checkout) fn parent(
    root: &File,
    path: &str,
) -> Result<Option<(File, String)>> {
    safe_path(path)?;
    let parts: Vec<_> = path.split('/').collect();
    let mut dir = root.try_clone().map_err(|_| Error::Unavailable)?;
    for name in &parts[..parts.len() - 1] {
        match fs::statat(&dir, *name, AtFlags::SYMLINK_NOFOLLOW) {
            Err(rustix::io::Errno::NOENT) => return Ok(None),
            Ok(stat) if FileType::from_raw_mode(stat.st_mode) == FileType::Directory => {
                let next = physical::child(&dir, name)?;
                let held = fs::fstat(&next).map_err(|_| Error::Unavailable)?;
                if (stat.st_dev, stat.st_ino) != (held.st_dev, held.st_ino) {
                    return Err(Error::EpochChanged);
                }
                dir = next;
            }
            _ => return Err(Error::EpochChanged),
        }
    }
    Ok(Some((dir, parts[parts.len() - 1].into())))
}
/// Reopened ancestors must still be exact original or prepared directories.
pub(in crate::local_execution::recovery::tree_checkout) fn checked_parent(
    root: &File,
    path: &str,
    plan: &TreeCheckoutPlanV1,
    state: &Progress,
) -> Result<Option<(File, String)>> {
    safe_path(path)?;
    let mut prefix = String::new();
    let parts: Vec<_> = path.split('/').collect();
    let mut dir = root.try_clone().map_err(|_| Error::Unavailable)?;
    for (depth, part) in std::iter::once("")
        .chain(parts[..parts.len() - 1].iter().copied())
        .enumerate()
    {
        if depth > 0 {
            if !prefix.is_empty() {
                prefix.push('/');
            }
            prefix.push_str(part);
            match records::destination::optional(&dir, part)? {
                Some(child) => dir = child,
                None => return Ok(None),
            }
        }
        let i = plan
            .directories
            .iter()
            .position(|d| d.path == prefix)
            .ok_or(Error::Corrupt)?;
        let stamp = directory_stamp(&dir)?;
        if let Some(expected) = plan.directories[i].identity {
            let mut hash = Sha256::new_with_prefix(b"jj-store/tree-checkout-directory/v1\0");
            hash.update(stamp[0].to_be_bytes());
            hash.update(stamp[1].to_be_bytes());
            if m::RecordDigest::new(hash.finalize().into()) != expected {
                return Err(Error::EpochChanged);
            }
        } else if i >= state.dirs_done
            || state.directories[i]
                .is_none_or(|expected| expected[..2] != stamp[..2] || expected[3] != stamp[3])
        {
            return Err(Error::EpochChanged);
        }
        if stamp[0] != state.private[0] {
            return Err(Error::Invalid);
        }
    }
    Ok(Some((dir, parts[parts.len() - 1].into())))
}
pub(in crate::local_execution::recovery::tree_checkout) fn read(
    dir: &File,
    name: &str,
    budget: &Budget,
) -> Result<Option<(Snapshot, physical::Bytes)>> {
    let before = match fs::statat(dir, name, AtFlags::SYMLINK_NOFOLLOW) {
        Err(rustix::io::Errno::NOENT) => return Ok(None),
        value => value.map_err(|_| Error::EpochChanged)?,
    };
    let data =
        physical::read(dir, name, tree::MAX_FILE_BYTES, false, budget)?.ok_or(Error::Incomplete)?;
    let after =
        fs::statat(dir, name, AtFlags::SYMLINK_NOFOLLOW).map_err(|_| Error::EpochChanged)?;
    if stamp(&before) != stamp(&after) {
        return Err(Error::EpochChanged);
    }
    Ok(Some((
        Snapshot {
            stamp: stamp(&before),
            content: tree::content_digest(&data.bytes),
        },
        data,
    )))
}
impl Snapshot {
    pub fn fingerprint(&self) -> m::RecordDigest {
        let mut hash = Sha256::new_with_prefix(b"jj-store/physical-entry/v1\0");
        for word in self.stamp {
            hash.update(word.to_be_bytes());
        }
        hash.update(self.content.as_bytes());
        m::RecordDigest::new(hash.finalize().into())
    }
    pub fn matches(&self, actual: &Self, moved: bool) -> bool {
        self.content == actual.content
            && if moved {
                self.stamp[..7] == actual.stamp[..7]
            } else {
                self.stamp == actual.stamp
            }
    }
    pub fn value(&self, value: &FileValue) -> Result<()> {
        if self.stamp[1] == 0
            || self.stamp[2] & 0o170000 != 0o100000
            || self.stamp[3] != 1
            || self.stamp[2] & 0o7000 != 0
            || self.stamp[4] != value.size
            || self.content != value.content
            || (self.stamp[2] & 0o111 != 0) != value.executable
        {
            return Err(Error::Corrupt);
        }
        Ok(())
    }
}
pub(in crate::local_execution::recovery::tree_checkout) fn sync_file(
    dir: &File,
    name: &str,
    expected: &Snapshot,
) -> Result<File> {
    let file: File = fs::openat(
        dir,
        name,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NONBLOCK,
        Mode::empty(),
    )
    .map_err(|_| Error::EpochChanged)?
    .into();
    if stamp(&fs::fstat(&file).map_err(|_| Error::Unavailable)?) != expected.stamp
        || stamp(
            &fs::statat(dir, name, AtFlags::SYMLINK_NOFOLLOW).map_err(|_| Error::EpochChanged)?,
        ) != expected.stamp
    {
        return Err(Error::EpochChanged);
    }
    file.sync_all().map_err(|_| Error::Unavailable)?;
    Ok(file)
}
pub(in crate::local_execution::recovery::tree_checkout) fn prepare_file(
    dir: &File,
    name: &str,
    bytes: &[u8],
    executable: bool,
    budget: &Budget,
) -> Result<Snapshot> {
    prepare_mode(
        dir,
        name,
        bytes,
        if executable { 0o755 } else { 0o644 },
        budget,
    )
}
pub(in crate::local_execution::recovery::tree_checkout) fn prepare_mode(
    dir: &File,
    name: &str,
    bytes: &[u8],
    mode: u32,
    budget: &Budget,
) -> Result<Snapshot> {
    let saved = read(dir, name, budget)?;
    if saved
        .as_ref()
        .is_some_and(|(_, data)| !bytes.starts_with(&data.bytes))
    {
        return Err(Error::Corrupt);
    }
    // A completed staged inode must not be chmod'd or rewritten on retry:
    // even chmod to the same mode changes ctime and invalidates a partially
    // written initial progress frame that already names this exact identity.
    if let Some((snapshot, data)) = &saved
        && data.bytes == bytes
        && snapshot.stamp[2] & 0o7777 == u64::from(mode)
    {
        sync_file(dir, name, snapshot)?;
        return Ok(snapshot.clone());
    }
    let flags = OFlags::WRONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::NONBLOCK;
    let mut file: File = fs::openat(
        dir,
        name,
        if saved.is_some() {
            flags | OFlags::APPEND
        } else {
            flags | OFlags::CREATE | OFlags::EXCL
        },
        Mode::RUSR | Mode::WUSR,
    )
    .map_err(|_| Error::Unavailable)?
    .into();
    let actual = read(dir, name, budget)?.ok_or(Error::Incomplete)?;
    if saved.as_ref().is_some_and(|old| old != &actual)
        || (saved.is_none() && !actual.1.bytes.is_empty())
    {
        return Err(Error::EpochChanged);
    }
    let held = stamp(&fs::fstat(&file).map_err(|_| Error::Unavailable)?);
    if held != actual.0.stamp {
        return Err(Error::EpochChanged);
    }
    file.write_all(&bytes[actual.1.bytes.len()..])
        .map_err(|_| Error::Unavailable)?;
    fs::fchmod(&file, Mode::from_raw_mode(mode as _)).map_err(|_| Error::Unavailable)?;
    file.sync_all().map_err(|_| Error::Unavailable)?;
    dir.sync_all().map_err(|_| Error::Unavailable)?;
    let (snapshot, data) = read(dir, name, budget)?.ok_or(Error::Incomplete)?;
    if data.bytes != bytes || snapshot.stamp[..2] != held[..2] {
        return Err(Error::EpochChanged);
    }
    Ok(snapshot)
}
