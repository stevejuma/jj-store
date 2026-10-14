use super::*;
use rustix::fs::{self, AtFlags, FileType, Mode, OFlags};
use std::{fs::File, io::Read as _, os::unix::fs::MetadataExt as _};
const DIRECTORY: OFlags = OFlags::RDONLY
    .union(OFlags::CLOEXEC)
    .union(OFlags::NOFOLLOW)
    .union(OFlags::DIRECTORY);
fn unavailable<T>(_: T) -> Error {
    Error::Unavailable
}
fn stale<T>(_: T) -> Error {
    Error::Stale
}
fn stat_bytes(info: &fs::Stat) -> Vec<u8> {
    [
        i128::from(info.st_dev) as u64,
        info.st_ino,
        info.st_mode as u64,
        i128::from(info.st_nlink) as u64,
        info.st_size as u64,
        info.st_mtime as u64,
        i128::from(info.st_mtime_nsec) as u64,
        info.st_ctime as u64,
        i128::from(info.st_ctime_nsec) as u64,
    ]
    .into_iter()
    .flat_map(u64::to_be_bytes)
    .collect()
}
fn stat(dir: &File, name: &str) -> Result<fs::Stat> {
    fs::statat(dir, name, AtFlags::SYMLINK_NOFOLLOW).map_err(stale)
}
fn frame(hash: &mut Sha256, bytes: &[u8]) {
    hash.update((bytes.len() as u64).to_be_bytes());
    hash.update(bytes);
}
pub(super) fn read<F: FnMut(Phase) -> Result<()>>(
    root: &Path,
    repository: RepositoryId,
    generation: u64,
    budget: &mut Budget<'_, F>,
) -> Result<WorkingTree> {
    budget.check()?;
    let dir: File = fs::open(root, DIRECTORY, Mode::empty())
        .map_err(unavailable)?
        .into();
    let jj: File = fs::openat(&dir, ".jj", DIRECTORY, Mode::empty())
        .map_err(unavailable)?
        .into();
    let info = dir.metadata().map_err(unavailable)?;
    let jinfo = jj.metadata().map_err(unavailable)?;
    let mut identity = Sha256::new_with_prefix(b"jj-store/physical-workspace/v1\0");
    identity.update(root.as_os_str().as_encoded_bytes());
    for n in [info.dev(), info.ino(), jinfo.dev(), jinfo.ino()] {
        identity.update(n.to_be_bytes());
    }
    let workspace = PhysicalWorkspaceId(RecordDigest::new(identity.finalize().into()));
    let mut entries = BTreeMap::new();
    let mut directories = BTreeMap::new();
    walk(&dir, "", 0, budget, &mut entries, &mut directories)?;
    let mut hash = Sha256::new_with_prefix(b"jj-store/physical-tree/v1\0");
    hash.update(repository.as_bytes());
    hash.update(generation.to_be_bytes());
    hash.update(workspace.0.as_bytes());
    for (path, stamp) in &directories {
        hash.update(b"D");
        frame(&mut hash, path.as_bytes());
        frame(&mut hash, stamp);
    }
    let entries: Vec<_> = entries.into_values().collect();
    for entry in &entries {
        hash.update(b"F");
        frame(&mut hash, entry.path.as_bytes());
        hash.update(entry.fingerprint.as_bytes());
    }
    budget.check()?;
    Ok(WorkingTree {
        version: 1,
        repository,
        generation,
        workspace,
        fingerprint: RecordDigest::new(hash.finalize().into()),
        entries,
        directories: directories
            .into_iter()
            .map(|(path, stamp)| {
                let (words, tail) = stamp.as_chunks::<8>();
                if !tail.is_empty() {
                    return Err(Error::Invalid);
                }
                let stamp = words
                    .iter()
                    .map(|value| u64::from_be_bytes(*value))
                    .collect::<Vec<_>>()
                    .try_into()
                    .map_err(|_| Error::Invalid)?;
                Ok(jj_store_jj::local_execution::PhysicalDirectoryV1 { path, stamp })
            })
            .collect::<Result<_>>()?,
    })
}
fn walk<F: FnMut(Phase) -> Result<()>>(
    dir: &File,
    prefix: &str,
    depth: usize,
    budget: &mut Budget<'_, F>,
    entries: &mut BTreeMap<String, Entry>,
    directories: &mut BTreeMap<String, Vec<u8>>,
) -> Result<()> {
    budget.check()?;
    if depth > budget.limits.max_depth {
        return Err(Error::Limit);
    }
    let before = crate::working_copy::stamp(&dir.metadata().map_err(unavailable)?);
    let mut names = fs::Dir::read_from(dir).map_err(unavailable)?;
    for entry in &mut names {
        let entry = entry.map_err(unavailable)?;
        let bytes = entry.file_name().to_bytes();
        if matches!(bytes, b"." | b"..") {
            continue;
        }
        budget.entry()?;
        let name = std::str::from_utf8(bytes).map_err(|_| Error::Invalid)?;
        if name.eq_ignore_ascii_case(".jj") || name.eq_ignore_ascii_case(".git") {
            continue;
        }
        if name.contains(['/', '\\']) || name.chars().any(char::is_control) {
            return Err(Error::Invalid);
        }
        let path = if prefix.is_empty() {
            name.into()
        } else {
            format!("{prefix}/{name}")
        };
        if path.len() > 4096 {
            return Err(Error::Limit);
        }
        let info = stat(dir, name)?;
        match FileType::from_raw_mode(info.st_mode) {
            FileType::Directory => {
                let child: File = fs::openat(dir, name, DIRECTORY, Mode::empty())
                    .map_err(stale)?
                    .into();
                let actual = child.metadata().map_err(stale)?;
                if actual.dev() != i128::from(info.st_dev) as u64 || actual.ino() != info.st_ino {
                    return Err(Error::Stale);
                }
                walk(&child, &path, depth + 1, budget, entries, directories)?;
                if stat_bytes(&info) != stat_bytes(&stat(dir, name)?) {
                    return Err(Error::Stale);
                }
            }
            FileType::RegularFile | FileType::Symlink => {
                let row = read_entry(dir, name, &path, &info, budget)?;
                if entries.insert(path, row).is_some() {
                    return Err(Error::Stale);
                }
            }
            _ => return Err(Error::Unsupported),
        }
    }
    let after = crate::working_copy::stamp(&dir.metadata().map_err(stale)?);
    if before != after {
        return Err(Error::Stale);
    }
    directories.insert(prefix.into(), after);
    Ok(())
}
fn read_entry<F: FnMut(Phase) -> Result<()>>(
    dir: &File,
    name: &str,
    path: &str,
    info: &fs::Stat,
    budget: &mut Budget<'_, F>,
) -> Result<Entry> {
    if info.st_size < 0 || info.st_size as u64 > budget.limits.max_file_bytes as u64 {
        return Err(Error::Limit);
    }
    let (kind, bytes) = if FileType::from_raw_mode(info.st_mode) == FileType::Symlink {
        let mut buffer = [0; 4097];
        let length = fs::readlinkat_raw(dir, name, &mut buffer[..]).map_err(stale)?;
        if length == buffer.len() {
            return Err(Error::Limit);
        }
        budget.bytes(length)?;
        (Kind::Symlink, buffer[..length].to_vec())
    } else {
        if info.st_nlink != 1 || info.st_mode as u32 & 0o7000 != 0 {
            return Err(Error::Unsupported);
        }
        let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::NONBLOCK;
        let mut file: File = fs::openat(dir, name, flags, Mode::empty())
            .map_err(stale)?
            .into();
        let actual = fs::fstat(&file).map_err(stale)?;
        if stat_bytes(info) != stat_bytes(&actual) {
            return Err(Error::Stale);
        }
        let mut bytes = Vec::new();
        let mut buffer = [0; 65536];
        loop {
            budget.check()?;
            let length = file.read(&mut buffer).map_err(unavailable)?;
            budget.bytes(length)?;
            if bytes.len() + length > budget.limits.max_file_bytes {
                return Err(Error::Limit);
            }
            bytes.extend_from_slice(&buffer[..length]);
            if length == 0 {
                break;
            }
        }
        if stat_bytes(info) != stat_bytes(&fs::fstat(&file).map_err(stale)?) {
            return Err(Error::Stale);
        }
        if bytes.len() as u64 != info.st_size as u64 {
            return Err(Error::Stale);
        }
        (
            Kind::File {
                executable: info.st_mode as u32 & 0o111 != 0,
            },
            bytes,
        )
    };
    if stat_bytes(info) != stat_bytes(&stat(dir, name)?) {
        return Err(Error::Stale);
    }
    let content = digest(b"jj-store/physical-content/v1\0", &bytes);
    let mut hash = Sha256::new_with_prefix(b"jj-store/physical-entry/v1\0");
    hash.update(stat_bytes(info));
    hash.update(content.as_bytes());
    Ok(Entry {
        path: path.into(),
        kind,
        bytes: bytes.len() as u64,
        content,
        fingerprint: RecordDigest::new(hash.finalize().into()),
        ignore_bytes: (name == ".gitignore" && matches!(kind, Kind::File { .. })).then_some(bytes),
    })
}
