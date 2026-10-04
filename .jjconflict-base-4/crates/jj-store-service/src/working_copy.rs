//! Device-local physical files. Save never enters a JJ or companion transaction.
use jj_store_model::RepositoryId;
use rustix::fs::{self as fs, AtFlags, Mode, OFlags};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{
    fs::File,
    io::{Read as _, Write as _},
    os::unix::fs::MetadataExt as _,
    path::Path,
    sync::Mutex,
};

pub const MAX_BYTES: usize = 256 * 1024;
static SAVES: Mutex<()> = Mutex::new(());
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("invalid working-file selection")]
    Invalid,
    #[error("working file changed; keep the draft and reload the physical file")]
    Stale,
    #[error("only existing regular UTF-8 files are supported")]
    Unsupported,
    #[error("working file exceeds the text limit")]
    Limit,
    #[error("working file is unavailable")]
    Unavailable,
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkingFileSelection {
    pub version: u32,
    pub repository: RepositoryId,
    pub generation: u64,
    pub workspace: String,
    pub path: String,
    pub fingerprint: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkingFile {
    pub selection: WorkingFileSelection,
    pub contents: String,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReadWorkingFile {
    pub path: String,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SaveWorkingFile {
    pub selection: WorkingFileSelection,
    pub contents: String,
}

struct OpenPath {
    parent: File,
    temporary_directory: File,
    name: String,
    workspace: String,
}
fn open(root: &Path, path: &str) -> Result<OpenPath> {
    if !root.is_absolute()
        || path.is_empty()
        || path.len() > 4096
        || path.contains(['\\', '\0'])
        || path.chars().any(char::is_control)
        || path.split('/').any(|s| {
            s.is_empty()
                || matches!(s, "." | "..")
                || s.eq_ignore_ascii_case(".jj")
                || s.eq_ignore_ascii_case(".git")
        })
    {
        return Err(Error::Invalid);
    }
    let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::DIRECTORY;
    let mut parent: File = fs::open(root, flags, Mode::empty())
        .map_err(|_| Error::Unavailable)?
        .into();
    let jj: File = fs::openat(&parent, ".jj", flags, Mode::empty())
        .map_err(|_| Error::Unavailable)?
        .into();
    let info = parent.metadata().map_err(|_| Error::Unavailable)?;
    let jinfo = jj.metadata().map_err(|_| Error::Unavailable)?;
    let mut hash = Sha256::new_with_prefix(b"jj-store/physical-workspace/v1\0");
    hash.update(root.as_os_str().as_encoded_bytes());
    for n in [info.dev(), info.ino(), jinfo.dev(), jinfo.ino()] {
        hash.update(n.to_be_bytes());
    }
    let workspace = hex::encode(hash.finalize());
    let mut parts = path.split('/').peekable();
    while let Some(part) = parts.next() {
        if parts.peek().is_none() {
            return Ok(OpenPath {
                parent,
                temporary_directory: jj,
                name: part.into(),
                workspace,
            });
        }
        parent = fs::openat(&parent, part, flags, Mode::empty())
            .map_err(|_| Error::Unsupported)?
            .into();
    }
    Err(Error::Invalid)
}
fn read_open(
    open: &OpenPath,
    repository: RepositoryId,
    generation: u64,
    path: &str,
) -> Result<WorkingFile> {
    let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::NONBLOCK;
    let mut file: File = fs::openat(&open.parent, open.name.as_str(), flags, Mode::empty())
        .map_err(|_| Error::Unsupported)?
        .into();
    let info = file.metadata().map_err(|_| Error::Unavailable)?;
    if !info.is_file() || info.nlink() != 1 {
        return Err(Error::Unsupported);
    }
    if info.len() > MAX_BYTES as u64 {
        return Err(Error::Limit);
    }
    let mut bytes = Vec::new();
    (&mut file)
        .take(MAX_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Error::Unavailable)?;
    if bytes.len() > MAX_BYTES {
        return Err(Error::Limit);
    }
    let after = file.metadata().map_err(|_| Error::Unavailable)?;
    if info.len() != bytes.len() as u64 || stamp(&info) != stamp(&after) {
        return Err(Error::Stale);
    }
    if bytes.contains(&0) {
        return Err(Error::Unsupported);
    }
    let contents = String::from_utf8(bytes).map_err(|_| Error::Unsupported)?;
    let mut hash = Sha256::new_with_prefix(b"jj-store/working-file/v1\0");
    hash.update(stamp(&info));
    hash.update(contents.as_bytes());
    Ok(WorkingFile {
        selection: WorkingFileSelection {
            version: 1,
            repository,
            generation,
            workspace: open.workspace.clone(),
            path: path.into(),
            fingerprint: hex::encode(hash.finalize()),
        },
        contents,
    })
}
fn stamp(info: &std::fs::Metadata) -> Vec<u8> {
    [
        info.dev(),
        info.ino(),
        info.len(),
        info.mode() as u64,
        info.mtime() as u64,
        info.mtime_nsec() as u64,
        info.ctime() as u64,
        info.ctime_nsec() as u64,
    ]
    .into_iter()
    .flat_map(u64::to_be_bytes)
    .collect()
}
pub fn read(
    root: &Path,
    repository: RepositoryId,
    generation: u64,
    path: &str,
) -> Result<WorkingFile> {
    read_open(&open(root, path)?, repository, generation, path)
}
pub fn save(
    root: &Path,
    repository: RepositoryId,
    generation: u64,
    expected: &WorkingFileSelection,
    contents: &str,
) -> Result<WorkingFile> {
    if contents.len() > MAX_BYTES {
        return Err(Error::Limit);
    }
    if contents.contains('\0') {
        return Err(Error::Unsupported);
    }
    if expected.version != 1
        || expected.repository != repository
        || expected.generation != generation
    {
        return Err(Error::Stale);
    }
    let _lock = SAVES.lock().map_err(|_| Error::Unavailable)?;
    let open = open(root, &expected.path)?;
    let before = read_open(&open, repository, generation, &expected.path)?;
    if &before.selection != expected {
        return Err(Error::Stale);
    }
    // Private temporaries cannot be captured by an external JJ snapshot.
    if open
        .parent
        .metadata()
        .map_err(|_| Error::Unavailable)?
        .dev()
        != open
            .temporary_directory
            .metadata()
            .map_err(|_| Error::Unavailable)?
            .dev()
    {
        return Err(Error::Unsupported);
    }
    let mut random = [0; 16];
    getrandom::fill(&mut random).map_err(|_| Error::Unavailable)?;
    let temporary = format!(".jj-store-save-{}", hex::encode(random));
    let flags = OFlags::WRONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::CREATE | OFlags::EXCL;
    let mut file: File = fs::openat(
        &open.temporary_directory,
        temporary.as_str(),
        flags,
        Mode::RUSR | Mode::WUSR,
    )
    .map_err(|_| Error::Unavailable)?
    .into();
    let result = (|| {
        // Capture the admitted existing mode; special bits are rejected below.
        let info = fs::statat(&open.parent, open.name.as_str(), AtFlags::SYMLINK_NOFOLLOW)
            .map_err(|_| Error::Stale)?;
        let mode = Mode::from_bits_truncate(info.st_mode);
        if mode.intersects(Mode::SUID | Mode::SGID | Mode::SVTX) {
            return Err(Error::Unsupported);
        }
        file.write_all(contents.as_bytes())
            .map_err(|_| Error::Unavailable)?;
        fs::fchmod(&file, mode).map_err(|_| Error::Unavailable)?;
        file.sync_all().map_err(|_| Error::Unavailable)?;
        // Re-open both the path and physical workspace immediately before rename.
        if read(root, repository, generation, &expected.path)?.selection != *expected {
            return Err(Error::Stale);
        }
        fs::renameat(
            &open.temporary_directory,
            temporary.as_str(),
            &open.parent,
            open.name.as_str(),
        )
        .map_err(|_| Error::Unavailable)?;
        open.parent.sync_all().map_err(|_| Error::Unavailable)?;
        open.temporary_directory
            .sync_all()
            .map_err(|_| Error::Unavailable)?;
        read(root, repository, generation, &expected.path)
    })();
    let _ = fs::unlinkat(
        &open.temporary_directory,
        temporary.as_str(),
        AtFlags::empty(),
    );
    result
}
