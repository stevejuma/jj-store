use jj_store_jj::operation_stores::AuthorityResult;
use std::{
    ffi::OsStr,
    fs::{self, File, OpenOptions},
    path::Path,
};

// Older binaries hold capture.lock exclusively for their entire spool handle.
// New handles share that legacy gate, then use independent mutation/drain locks.
// This prevents an older writer bypassing the new short-lock protocol.
pub(crate) fn compatibility_lock(path: &Path) -> AuthorityResult<File> {
    let path = jj_store_server::policy::outside_checkout(path)?;
    if fs::symlink_metadata(&path).is_ok_and(|metadata| !metadata.is_file()) {
        return Err("capture coordination lock must be a regular file".into());
    }
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let file = options.open(path)?;
    file.try_lock_shared()
        .map_err(|_| "capture spool is held by an incompatible writer")?;
    Ok(file)
}

pub(crate) fn recover_pending(path: &Path) -> AuthorityResult<()> {
    let mut removed = false;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let name = entry.file_name();
        if entry
            .path()
            .extension()
            .is_some_and(|value| value == "json")
        {
            continue;
        }
        if file_type.is_file() && !file_type.is_symlink() && temporary_name(&name) {
            fs::remove_file(entry.path())?;
            removed = true;
        } else {
            return Err("unexpected capture spool entry".into());
        }
    }
    if removed {
        sync_dir(path)?;
    }
    Ok(())
}

fn temporary_name(name: &OsStr) -> bool {
    name.to_str().is_some_and(|value| value.starts_with(".tmp"))
}

pub(crate) fn sync_dir(path: &Path) -> AuthorityResult<()> {
    #[cfg(unix)]
    File::open(path)?.sync_all()?;
    Ok(())
}

pub(crate) fn restrict_directory(path: &Path) -> AuthorityResult<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    }
    Ok(())
}
