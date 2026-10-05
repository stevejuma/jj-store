//! Shared private device-file mechanics; selects bytes, never capture authority.
use std::path::Path;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Error {
    Unavailable,
    Capacity,
    OutcomeUnknown,
}
pub(crate) type Result<T> = std::result::Result<T, Error>;
#[derive(Clone, Copy)]
pub(crate) struct Layout {
    pub lock: &'static str,
    pub temporary: &'static str,
    pub max_bytes: usize,
}
mod metadata;
mod publication;
pub(crate) use metadata::Stamp;
use metadata::{id, stamp};
use std::{
    fs::{self, File, Metadata, OpenOptions},
    io::Read as _,
    path::PathBuf,
};
const STATE: &str = "state.json";
#[derive(Clone)]
pub(crate) struct Root {
    layout: Layout,
    logical: PathBuf,
    physical: PathBuf,
    id: [u64; 2],
    lock_id: [u64; 2],
    owner: u32,
}
pub(crate) struct Gate {
    file: File,
    id: [u64; 2],
}
fn unavailable<T>(result: std::io::Result<T>) -> Result<T> {
    result.map_err(|_| Error::Unavailable)
}
impl Root {
    pub fn at(&self, path: &Path) -> bool {
        self.logical == path
    }
    pub fn create(path: &Path, layout: Layout) -> Result<Self> {
        let physical = crate::policy::outside_checkout(path).map_err(|_| Error::Unavailable)?;
        let mut builder = fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt as _;
            builder.mode(0o700);
        }
        unavailable(builder.create(&physical))?;
        let mut lock = OpenOptions::new();
        lock.read(true).write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            lock.mode(0o600);
        }
        unavailable(lock.open(physical.join(layout.lock)))?;
        let root = Self::open(path, layout)?;
        unavailable(
            File::open(root.physical.parent().ok_or(Error::Unavailable)?)
                .and_then(|f| f.sync_all()),
        )?;
        Ok(root)
    }
    pub fn open(path: &Path, layout: Layout) -> Result<Self> {
        let physical = crate::policy::outside_checkout(path).map_err(|_| Error::Unavailable)?;
        let logical = if path.is_absolute() {
            path.to_owned()
        } else {
            unavailable(std::env::current_dir())?.join(path)
        };
        let metadata = unavailable(fs::symlink_metadata(&physical))?;
        #[cfg(unix)]
        let owner = {
            use std::os::unix::fs::MetadataExt as _;
            unavailable(unavailable(tempfile::tempfile())?.metadata())?.uid()
        };
        #[cfg(not(unix))]
        let owner = 0;
        let lock_metadata = unavailable(fs::symlink_metadata(physical.join(layout.lock)))?;
        let root = Self {
            layout,
            logical,
            physical,
            id: id(&metadata)?,
            lock_id: id(&lock_metadata)?,
            owner,
        };
        root.validate()?;
        Ok(root)
    }
    fn private(&self, metadata: &Metadata, directory: bool) -> Result<()> {
        if metadata.file_type().is_symlink()
            || (if directory {
                !metadata.is_dir()
            } else {
                !metadata.is_file()
            })
        {
            return Err(Error::Unavailable);
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt as _;
            if metadata.uid() != self.owner || metadata.mode() & 0o077 != 0 {
                return Err(Error::Unavailable);
            }
        }
        Ok(())
    }
    fn validate(&self) -> Result<()> {
        let logical = unavailable(fs::symlink_metadata(&self.logical))?;
        let physical = unavailable(fs::symlink_metadata(&self.physical))?;
        self.private(&logical, true)?;
        self.private(&physical, true)?;
        if unavailable(fs::canonicalize(&self.logical))? != self.physical
            || id(&physical)? != self.id
        {
            return Err(Error::Unavailable);
        }
        Ok(())
    }
    pub fn gate(&self) -> Result<Gate> {
        self.validate()?;
        let path = self.physical.join(self.layout.lock);
        let metadata = unavailable(fs::symlink_metadata(&path))?;
        self.private(&metadata, false)?;
        if id(&metadata)? != self.lock_id || metadata.len() != 0 {
            return Err(Error::Unavailable);
        }
        let mut options = OpenOptions::new();
        options.read(true).write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        let file = unavailable(options.open(&path))?;
        self.private(&unavailable(file.metadata())?, false)?;
        if id(&unavailable(file.metadata())?)? != self.lock_id {
            return Err(Error::Unavailable);
        }
        file.try_lock().map_err(|_| Error::Unavailable)?;
        let gate = Gate {
            id: id(&unavailable(file.metadata())?)?,
            file,
        };
        self.check_gate(&gate)?;
        Ok(gate)
    }
    fn check_gate(&self, gate: &Gate) -> Result<()> {
        self.validate()?;
        let file = unavailable(gate.file.metadata())?;
        let path = unavailable(fs::symlink_metadata(self.physical.join(self.layout.lock)))?;
        self.private(&file, false)?;
        self.private(&path, false)?;
        if gate.id != self.lock_id
            || id(&file)? != gate.id
            || id(&path)? != gate.id
            || file.len() != 0
            || path.len() != 0
        {
            return Err(Error::Unavailable);
        }
        Ok(())
    }
    fn inventory(&self) -> Result<Vec<PathBuf>> {
        let mut staged = vec![];
        for entry in unavailable(fs::read_dir(&self.physical))? {
            let entry = unavailable(entry)?;
            let metadata = unavailable(fs::symlink_metadata(entry.path()))?;
            self.private(&metadata, false)?;
            if metadata.len() > self.layout.max_bytes as u64 {
                return Err(Error::Unavailable);
            }
            let name = entry.file_name();
            let name = name.to_str().ok_or(Error::Unavailable)?;
            if name == STATE || name == self.layout.lock {
                continue;
            }
            if !name.starts_with(self.layout.temporary) || name.len() <= self.layout.temporary.len()
            {
                return Err(Error::Unavailable);
            }
            staged.push(entry.path());
            if staged.len() > 1 {
                return Err(Error::Unavailable);
            }
        }
        Ok(staged)
    }
    fn check_selected(&self, gate: &Gate, expected: Option<Stamp>) -> Result<()> {
        self.check_gate(gate)?;
        match fs::symlink_metadata(self.physical.join(STATE)) {
            Ok(metadata) => {
                self.private(&metadata, false)?;
                if Some(stamp(&metadata)?) != expected {
                    return Err(Error::Unavailable);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound && expected.is_none() => {}
            Err(_) => return Err(Error::Unavailable),
        }
        Ok(())
    }
    pub fn read(&self, gate: &Gate) -> Result<(Vec<u8>, Stamp)> {
        self.check_gate(gate)?;
        self.inventory()?;
        let path = self.physical.join(STATE);
        let before = unavailable(fs::symlink_metadata(&path))?;
        self.private(&before, false)?;
        if before.len() > self.layout.max_bytes as u64 {
            return Err(Error::Unavailable);
        }
        let selected = stamp(&before)?;
        let file = unavailable(File::open(&path))?;
        self.private(&unavailable(file.metadata())?, false)?;
        if stamp(&unavailable(file.metadata())?)? != selected {
            return Err(Error::Unavailable);
        }
        let mut bytes = vec![];
        unavailable(
            (&file)
                .take(self.layout.max_bytes as u64 + 1)
                .read_to_end(&mut bytes),
        )?;
        if bytes.len() > self.layout.max_bytes || stamp(&unavailable(file.metadata())?)? != selected
        {
            return Err(Error::Unavailable);
        }
        self.check_selected(gate, Some(selected))?;
        Ok((bytes, selected))
    }
}
