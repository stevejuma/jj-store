//! Capability-contained regular-file restoration, shared by native hosts.
use super::*;
use cap_std::{ambient_authority, fs::Dir};
use sha2::{Digest as _, Sha256};
use std::io::{Read as _, Write as _};

pub(super) fn checked(path: &str) -> Result<()> {
    if path.is_empty()
        || path.len() > 4096
        || path.contains('\\')
        || path
            .split('/')
            .any(|p| matches!(p, "" | "." | ".." | ".jj" | ".git" | ".lapis"))
    {
        return Err(Error::Binding);
    }
    Ok(())
}
pub(super) fn root(binding: &Binding) -> Result<Dir> {
    binding.verify_saved()?;
    Ok(Dir::open_ambient_dir(&binding.root, ambient_authority())?)
}
pub(super) fn read(root: &Dir, path: &str) -> Result<Option<Vec<u8>>> {
    checked(path)?;
    match root.symlink_metadata(path) {
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
    let file = root.open_with(path, &options)?;
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
    #[cfg(unix)]
    {
        use cap_std::fs::PermissionsExt as _;
        Ok(root.symlink_metadata(path)?.permissions().mode() & 0o111 != 0)
    }
    #[cfg(not(unix))]
    {
        let _ = (root, path);
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
    let parent_path = Path::new(path)
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let name = Path::new(path).file_name().ok_or(Error::Binding)?;
    if digest(read(root, path)?.as_deref()) != expected {
        return Err(Error::Jj(
            "The file changed since the restore preview; review it again".into(),
        ));
    }
    if let Some(bytes) = target {
        root.create_dir_all(parent_path)?;
        let parent = root.open_dir(parent_path)?;
        let mut file = cap_tempfile::TempFile::new(&parent)?;
        #[cfg(unix)]
        {
            use cap_std::fs::PermissionsExt as _;
            let mode = parent
                .metadata(name)
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
        file.replace(name)?;
        parent.into_std_file().sync_all()?;
    } else if expected != "missing" {
        root.remove_file(path)?;
        root.open_dir(parent_path)?.into_std_file().sync_all()?;
    }
    Ok(())
}
