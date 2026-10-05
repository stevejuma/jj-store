use super::*;
use rustix::fs::{self, AtFlags, FileType, Mode, OFlags};
use std::{fs::File, io::Read as _, os::unix::fs::MetadataExt as _, path::Component};
pub(super) const DIR: OFlags = OFlags::RDONLY
    .union(OFlags::CLOEXEC)
    .union(OFlags::NOFOLLOW)
    .union(OFlags::DIRECTORY);
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Bytes {
    pub bytes: Vec<u8>,
    pub fingerprint: m::RecordDigest,
}
pub(crate) fn directory(path: &Path) -> Result<File> {
    if !path.is_absolute() {
        return Err(Error::Invalid);
    }
    let mut dir: File = fs::open("/", DIR, Mode::empty())
        .map_err(|_| Error::Unavailable)?
        .into();
    for part in path.components() {
        match part {
            Component::RootDir => {}
            Component::Normal(part) => {
                dir = fs::openat(&dir, part, DIR, Mode::empty())
                    .map_err(|_| Error::Unavailable)?
                    .into()
            }
            _ => return Err(Error::Invalid),
        }
    }
    Ok(dir)
}
pub(crate) fn child(dir: &File, name: &str) -> Result<File> {
    fs::openat(dir, name, DIR, Mode::empty())
        .map(File::from)
        .map_err(|_| Error::Unavailable)
}
fn stamp(info: &fs::Stat) -> Vec<u8> {
    [
        info.st_dev as u64,
        info.st_ino,
        info.st_mode as u64,
        info.st_nlink as u64,
        info.st_size as u64,
        info.st_mtime as u64,
        info.st_mtime_nsec as u64,
        info.st_ctime as u64,
        info.st_ctime_nsec as u64,
    ]
    .into_iter()
    .flat_map(u64::to_be_bytes)
    .collect()
}
pub(crate) fn read(
    dir: &File,
    name: &str,
    max: usize,
    optional: bool,
    budget: &Budget,
) -> Result<Option<Bytes>> {
    budget.work(1)?;
    let before = match fs::statat(dir, name, AtFlags::SYMLINK_NOFOLLOW) {
        Err(e) if e == rustix::io::Errno::NOENT && optional => return Ok(None),
        value => value.map_err(|_| Error::Unavailable)?,
    };
    if FileType::from_raw_mode(before.st_mode) != FileType::RegularFile || before.st_nlink != 1 {
        return Err(Error::Invalid);
    }
    if before.st_size < 0 || before.st_size as u64 > max as u64 {
        return Err(Error::Limit);
    }
    let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::NONBLOCK;
    let mut file: File = fs::openat(dir, name, flags, Mode::empty())
        .map_err(|_| Error::Unavailable)?
        .into();
    if stamp(&before) != stamp(&fs::fstat(&file).map_err(|_| Error::Unavailable)?) {
        return Err(Error::EpochChanged);
    }
    let mut bytes = Vec::new();
    let mut buffer = [0; 65536];
    loop {
        budget.check()?;
        let n = file.read(&mut buffer).map_err(|_| Error::Unavailable)?;
        budget.read_bytes(n)?;
        if bytes.len() + n > max {
            return Err(Error::Limit);
        }
        bytes.extend_from_slice(&buffer[..n]);
        if n == 0 {
            break;
        }
    }
    if bytes.len() as u64 != before.st_size as u64
        || stamp(&before) != stamp(&fs::fstat(&file).map_err(|_| Error::Unavailable)?)
        || stamp(&before)
            != stamp(
                &fs::statat(dir, name, AtFlags::SYMLINK_NOFOLLOW)
                    .map_err(|_| Error::EpochChanged)?,
            )
    {
        return Err(Error::EpochChanged);
    }
    let mut hash = Sha256::new_with_prefix(b"jj-store/current-state-file/v1\0");
    hash.update(stamp(&before));
    hash.update(&bytes);
    Ok(Some(Bytes {
        bytes,
        fingerprint: m::RecordDigest::new(hash.finalize().into()),
    }))
}
pub(super) fn absolute(path: &Path, budget: &Budget) -> Result<Option<Bytes>> {
    let parent = path.parent().ok_or(Error::Invalid)?;
    match std::fs::symlink_metadata(parent) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err(Error::Unavailable),
        _ => {}
    }
    let dir = directory(parent)?;
    let name = path
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or(Error::Invalid)?;
    read(&dir, name, MAX_FILE_BYTES, true, budget)
}
pub(super) fn workspace(root: &Path, dir: &File, jj: &File) -> Result<m::RecordDigest> {
    let info = dir.metadata().map_err(|_| Error::Unavailable)?;
    let jinfo = jj.metadata().map_err(|_| Error::Unavailable)?;
    let mut hash = Sha256::new_with_prefix(b"jj-store/physical-workspace/v1\0");
    hash.update(root.as_os_str().as_encoded_bytes());
    for n in [info.dev(), info.ino(), jinfo.dev(), jinfo.ino()] {
        hash.update(n.to_be_bytes());
    }
    Ok(m::RecordDigest::new(hash.finalize().into()))
}
