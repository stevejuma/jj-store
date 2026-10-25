//! Capability-contained regular-file restoration, shared by native hosts.
use super::*;
use cap_fs_ext::DirExt as _;
use cap_std::{ambient_authority, fs::Dir};
use sha2::{Digest as _, Sha256};
use std::io::{Read as _, Write as _};

pub(super) fn checked(path: &str) -> Result<()> {
    if !backup_path(path).file() {
        return Err(Error::Binding);
    }
    Ok(())
}
pub(super) fn root(binding: &Binding) -> Result<Dir> {
    binding.verify_saved()?;
    let root = Dir::open_ambient_dir(&binding.root, ambient_authority())?;
    binding.verify_saved()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        let opened = root.try_clone()?.into_std_file().metadata()?;
        let selected = fs::symlink_metadata(&binding.root)?;
        if opened.dev() != selected.dev() || opened.ino() != selected.ino() {
            return Err(Error::Binding);
        }
    }
    Ok(root)
}
/// Open one parent at a time: rejecting only the leaf symlink is insufficient
/// when a directory alias can point into excluded runtime state.
fn parent(root: &Dir, path: &str, create: bool) -> std::io::Result<(Dir, std::ffi::OsString)> {
    let path = Path::new(path);
    let name = path
        .file_name()
        .ok_or_else(|| std::io::Error::other("Invalid restore path"))?
        .to_owned();
    let mut directory = root.try_clone()?;
    for component in path.parent().unwrap_or(Path::new("")).components() {
        let std::path::Component::Normal(part) = component else {
            return Err(std::io::Error::other("Invalid restore directory"));
        };
        if create {
            match directory.create_dir(part) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error),
            }
        }
        directory = directory.open_dir_nofollow(part)?;
    }
    Ok((directory, name))
}
pub(super) fn read(root: &Dir, path: &str) -> Result<Option<Vec<u8>>> {
    checked(path)?;
    let (directory, name) = match parent(root, path, false) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        result => result?,
    };
    match directory.symlink_metadata(&name) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Ok(info) if !info.is_file() || info.is_symlink() => return Err(Error::Binding),
        Ok(_) => {}
        Err(e) => return Err(e.into()),
    }
    let mut options = cap_std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use cap_std::fs::OpenOptionsExt as _;
        options.custom_flags(
            (rustix::fs::OFlags::NONBLOCK | rustix::fs::OFlags::NOFOLLOW).bits() as i32,
        );
    }
    let file = directory.open_with(&name, &options)?;
    if !file.metadata()?.is_file() {
        return Err(Error::Binding);
    }
    let mut data = Vec::new();
    file.take(64 * 1024 * 1024 + 1).read_to_end(&mut data)?;
    if data.len() > 64 * 1024 * 1024 {
        return Err(Error::Binding);
    }
    Ok(Some(data))
}
pub(super) fn digest(data: Option<&[u8]>) -> String {
    data.map(|bytes| format!("{:x}", Sha256::digest(bytes)))
        .unwrap_or_else(|| "missing".into())
}
pub(super) fn executable(root: &Dir, path: &str) -> Result<bool> {
    checked(path)?;
    let (directory, name) = parent(root, path, false)?;
    #[cfg(unix)]
    {
        use cap_std::fs::PermissionsExt as _;
        Ok(directory.symlink_metadata(name)?.permissions().mode() & 0o111 != 0)
    }
    #[cfg(not(unix))]
    {
        let _ = (directory, name);
        Ok(false)
    }
}
pub(super) fn apply(
    root: &Dir,
    path: &str,
    expected: &str,
    target: Option<&[u8]>,
    executable: bool,
) -> Result<()> {
    checked(path)?;
    if digest(read(root, path)?.as_deref()) != expected {
        return Err(Error::Jj(
            "The file changed since the restore preview; review it again".into(),
        ));
    }
    if let Some(bytes) = target {
        let (parent, name) = parent(root, path, true)?;
        let mut file = cap_tempfile::TempFile::new(&parent)?;
        #[cfg(unix)]
        {
            use cap_std::fs::PermissionsExt as _;
            let mode = parent
                .symlink_metadata(&name)
                .map(|m| m.permissions().mode())
                .unwrap_or(0o644);
            file.as_file()
                .set_permissions(cap_std::fs::Permissions::from_mode(if executable {
                    mode | 0o111
                } else {
                    mode & !0o111
                }))?;
        }
        #[cfg(not(unix))]
        let _ = executable;
        file.write_all(bytes)?;
        file.as_file().sync_all()?;
        if digest(read(root, path)?.as_deref()) != expected {
            return Err(Error::Binding);
        }
        file.replace(&name)?;
        // Linux directory capabilities may be search-only; reopen for fsync.
        parent.open(".")?.sync_all()?;
    } else if expected != "missing" {
        let (parent, name) = parent(root, path, false)?;
        parent.remove_file(name)?;
        parent.open(".")?.sync_all()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_file_restore_and_delete_synchronize_the_directory() {
        let temp = tempfile::tempdir().unwrap();
        let root = Dir::open_ambient_dir(temp.path(), ambient_authority()).unwrap();
        apply(&root, "Note.md", "missing", Some(b"restored"), false).unwrap();
        assert_eq!(
            read(&root, "Note.md").unwrap().as_deref(),
            Some(b"restored".as_slice())
        );
        apply(&root, "Note.md", &digest(Some(b"restored")), None, false).unwrap();
        assert!(read(&root, "Note.md").unwrap().is_none());
    }
}
