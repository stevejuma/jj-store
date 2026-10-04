use crate::{Error, Result, installation::Identity, policy, registry_catalog::Catalog};
use std::{
    fs::File,
    io::Write as _,
    path::{Path, PathBuf},
};

const MAX_BYTES: u64 = 4 * 1024 * 1024;

pub(crate) fn read(path: &Path) -> Result<Vec<u8>> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        if std::fs::symlink_metadata(path)?.permissions().mode() & 0o077 != 0 {
            return Err(Error::Config("repository registry permissions"));
        }
    }
    policy::read_regular(path, MAX_BYTES)
}

pub(crate) fn lock(path: &Path) -> Result<File> {
    let mut name = PathBuf::from(path).into_os_string();
    name.push(".lock");
    policy::exclusive_lock(Path::new(&name))
}

pub(crate) fn persist(path: &Path, catalog: &Catalog) -> Result<()> {
    catalog.validate(&Identity {
        version: 1,
        installation: catalog.installation.clone(),
    })?;
    let parent = path
        .parent()
        .ok_or(Error::Config("repository registry parent"))?;
    let bytes = serde_json::to_vec(catalog)
        .map_err(|_| Error::Config("repository registry serialization"))?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err(Error::Config("repository registry size"));
    }
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        temp.as_file()
            .set_permissions(std::fs::Permissions::from_mode(0o600))?;
    }
    temp.write_all(&bytes)?;
    temp.as_file().sync_all()?;
    temp.persist(path).map_err(|error| Error::Io(error.error))?;
    #[cfg(unix)]
    File::open(parent)?.sync_all()?;
    Ok(())
}
