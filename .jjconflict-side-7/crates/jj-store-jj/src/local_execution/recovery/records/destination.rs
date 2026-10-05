use super::*;
use rustix::fs::{self, AtFlags, FileType, Mode};
pub(super) struct Destination {
    path: PathBuf,
    root: File,
    pub store: Option<File>,
    pub views: Option<File>,
    pub operations: Option<File>,
}
impl Destination {
    pub fn open(path: &Path) -> Result<Self> {
        let root = physical::directory(path)?;
        let store = optional(&root, "op_store")?;
        let views = store
            .as_ref()
            .map(|dir| optional(dir, "views"))
            .transpose()?
            .flatten();
        let operations = store
            .as_ref()
            .map(|dir| optional(dir, "operations"))
            .transpose()?
            .flatten();
        Ok(Self {
            path: path.to_path_buf(),
            root,
            store,
            views,
            operations,
        })
    }
    pub fn check(&self) -> Result<()> {
        same(&self.root, &physical::directory(&self.path)?)?;
        match (&self.store, optional(&self.root, "op_store")?) {
            (Some(held), Some(current)) => {
                same(held, &current)?;
                for (name, saved) in [("views", &self.views), ("operations", &self.operations)] {
                    match (saved, optional(held, name)?) {
                        (Some(saved), Some(current)) => same(saved, &current)?,
                        (None, None) => {}
                        _ => return Err(Error::EpochChanged),
                    }
                }
            }
            (None, None) => {}
            _ => return Err(Error::EpochChanged),
        }
        Ok(())
    }
    pub fn ensure(&mut self) -> Result<()> {
        self.check()?;
        if self.store.is_none() {
            self.store = Some(create(&self.root, "op_store")?);
        }
        let store = self.store.as_ref().unwrap();
        if self.views.is_none() {
            self.views = Some(create(store, "views")?);
        }
        if self.operations.is_none() {
            self.operations = Some(create(store, "operations")?);
        }
        self.check()
    }
}
pub(in crate::local_execution::recovery) fn optional(
    parent: &File,
    name: &str,
) -> Result<Option<File>> {
    match fs::statat(parent, name, AtFlags::SYMLINK_NOFOLLOW) {
        Err(rustix::io::Errno::NOENT) => Ok(None),
        Ok(info) if FileType::from_raw_mode(info.st_mode) == FileType::Directory => {
            physical::child(parent, name).map(Some)
        }
        _ => Err(Error::Invalid),
    }
}
pub(in crate::local_execution::recovery) fn create(parent: &File, name: &str) -> Result<File> {
    fs::mkdirat(parent, name, Mode::RWXU).map_err(|_| Error::EpochChanged)?;
    parent.sync_all().map_err(|_| Error::Unavailable)?;
    physical::child(parent, name)
}
pub(in crate::local_execution::recovery) fn same(a: &File, b: &File) -> Result<()> {
    let a = fs::fstat(a).map_err(|_| Error::Unavailable)?;
    let b = fs::fstat(b).map_err(|_| Error::Unavailable)?;
    if (a.st_dev, a.st_ino) != (b.st_dev, b.st_ino) {
        return Err(Error::EpochChanged);
    }
    Ok(())
}
