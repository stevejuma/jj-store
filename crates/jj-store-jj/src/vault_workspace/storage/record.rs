use super::*;
use std::io::{Read as _, Write as _};
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Current {
    pub version: u32,
    pub vault_id: String,
    pub root_identity: String,
    pub revision: u64,
    pub location: StorageLocation,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Move {
    pub version: u32,
    pub vault_id: String,
    pub root_identity: String,
    pub request_id: String,
    pub from: StorageLocation,
    pub to: StorageLocation,
    pub revision: u64,
    pub source: Binding,
    pub destination: Binding,
    pub archive: PathBuf,
    pub stage: PathBuf,
    pub source_identity: String,
    pub stage_identity: String,
    pub phase: String,
}
pub(super) fn read<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let meta = fs::symlink_metadata(path)?;
    if !meta.is_file() || meta.is_symlink() || meta.len() > 512 * 1024 {
        return Err(Error::Binding);
    }
    let mut data = Vec::new();
    File::open(path)?
        .take(512 * 1024 + 1)
        .read_to_end(&mut data)?;
    if data.len() > 512 * 1024 {
        return Err(Error::Binding);
    }
    serde_json::from_slice(&data).map_err(jj)
}
pub(super) fn save<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let parent = path.parent().ok_or(Error::Binding)?;
    safe_dir(parent)?;
    let bytes = serde_json::to_vec(value).map_err(jj)?;
    if bytes.len() > 512 * 1024 {
        return Err(Error::Binding);
    }
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    temp.write_all(&bytes)?;
    temp.as_file().sync_all()?;
    temp.persist(path).map_err(|e| Error::Io(e.error))?;
    File::open(parent)?.sync_all()?;
    Ok(())
}
pub(super) fn safe_dir(path: &Path) -> Result<()> {
    if !path.is_absolute() {
        return Err(Error::Binding);
    }
    let mut current = PathBuf::new();
    for part in path.components() {
        current.push(part.as_os_str());
        match fs::symlink_metadata(&current) {
            Ok(m) if !m.is_dir() || m.is_symlink() => return Err(Error::Binding),
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let mut builder = fs::DirBuilder::new();
                #[cfg(unix)]
                {
                    use std::os::unix::fs::DirBuilderExt as _;
                    builder.mode(0o700);
                }
                builder.create(&current)?;
            }
            Err(e) => return Err(e.into()),
        }
    }
    if fs::canonicalize(path)? != path {
        return Err(Error::Binding);
    }
    Ok(())
}
pub(super) fn same(path: &Path, identity: &str) -> bool {
    root_identity(path).is_ok_and(|id| id == identity)
}
pub(super) fn current(storage: &Storage) -> Result<Current> {
    storage.base.validate()?;
    let path = storage.control.join("current.json");
    let value = match read(&path) {
        Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => Current {
            version: 1,
            vault_id: storage.base.vault_id.clone(),
            root_identity: root_identity(&storage.base.root)?,
            revision: 0,
            location: StorageLocation::External,
        },
        other => other?,
    };
    if value.version != 1
        || value.vault_id != storage.base.vault_id
        || value.root_identity != root_identity(&storage.base.root)?
    {
        return Err(Error::Binding);
    }
    Ok(value)
}
pub(super) fn clear_active(storage: &Storage, id: &str) -> Result<()> {
    let active = storage.control.join("active.json");
    match read::<String>(&active) {
        Ok(selected) if selected == id => {
            fs::remove_file(&active)?;
            File::open(&storage.control)?.sync_all()?;
        }
        Ok(_) => {}
        Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e),
    }
    Ok(())
}
