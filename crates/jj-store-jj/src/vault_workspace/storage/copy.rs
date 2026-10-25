use super::*;
use cap_fs_ext::DirExt as _;
use cap_std::{
    ambient_authority,
    fs::{Dir, OpenOptions},
};
use std::io::{Read as _, Write as _};
fn options(write: bool) -> OpenOptions {
    let mut options = OpenOptions::new();
    options.read(!write).write(write);
    if write {
        options.create_new(true);
    }
    #[cfg(unix)]
    {
        use cap_std::fs::OpenOptionsExt as _;
        options.custom_flags(
            (rustix::fs::OFlags::NOFOLLOW | rustix::fs::OFlags::NONBLOCK).bits() as i32,
        );
    }
    options
}
fn copy(
    source: &Dir,
    target: &Dir,
    count: &mut StorageProgress,
    progress: &mut impl FnMut(StorageProgress) -> Result<()>,
) -> Result<()> {
    let mut entries = source.entries()?.collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let name = entry.file_name();
        let metadata = source.symlink_metadata(&name)?;
        if metadata.is_symlink() || (!metadata.is_dir() && !metadata.is_file()) {
            return Err(Error::Binding);
        }
        if metadata.is_dir() {
            match target.create_dir(&name) {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(e) => return Err(e.into()),
            }
            copy(
                &source.open_dir_nofollow(&name)?,
                &target.open_dir_nofollow(&name)?,
                count,
                progress,
            )?;
        } else {
            count.files += 1;
            if count.files > 1_000_000
                || metadata.len() > 8 * 1024 * 1024 * 1024
                || count.bytes.saturating_add(metadata.len()) > 64 * 1024 * 1024 * 1024
            {
                return Err(Error::Jj(
                    "History storage exceeds its migration work limit".into(),
                ));
            }
            let mut input = source.open_with(&name, &options(false))?;
            let before = input.metadata()?;
            if !before.is_file() {
                return Err(Error::Binding);
            }
            // A previous interrupted copy can have installed a read-only Git
            // object. Replace only this regular file in our owned staging dir;
            // truncating it would fail when its original permissions are 0444.
            match target.symlink_metadata(&name) {
                Ok(meta) if meta.is_file() && !meta.is_symlink() => target.remove_file(&name)?,
                Ok(_) => return Err(Error::Binding),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => return Err(e.into()),
            }
            let mut output = target.open_with(&name, &options(true))?;
            let mut bytes = 0;
            let mut buffer = [0u8; 65536];
            loop {
                let n = input.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                output.write_all(&buffer[..n])?;
                bytes += n as u64;
                if bytes > metadata.len() {
                    return Err(Error::Binding);
                }
            }
            if bytes != metadata.len() || input.metadata()?.modified()? != before.modified()? {
                return Err(Error::Busy);
            }
            output.set_permissions(before.permissions())?;
            output.sync_all()?;
            count.bytes += bytes;
            progress(count.clone())?;
        }
    }
    // Linux directory capabilities may be search-only; reopen for fsync.
    target.open(".")?.sync_all()?;
    Ok(())
}
pub(super) fn all(
    source: &Path,
    target: &Path,
    progress: &mut impl FnMut(StorageProgress) -> Result<()>,
) -> Result<()> {
    let source = Dir::open_ambient_dir(source, ambient_authority())?;
    let target = Dir::open_ambient_dir(target, ambient_authority())?;
    let mut count = StorageProgress {
        phase: "copying".into(),
        files: 0,
        bytes: 0,
    };
    copy(&source, &target, &mut count, progress)
}
