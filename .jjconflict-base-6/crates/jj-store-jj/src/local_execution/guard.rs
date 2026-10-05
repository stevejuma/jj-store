//! Descriptor-relative equivalent of JJ 0.45.1's Unix file-lock protocol.
//! Nonblocking flock avoids keeping a worker waiting indefinitely on a CLI.
use crate::local_browser::{
    Error, Result,
    working_tree::current::physical::{child, directory},
};
use rustix::fs::{self, AtFlags, FileType, FlockOperation, Mode, OFlags};
use std::{fs::File, path::Path};
const NAME: &str = "working_copy.lock";
pub(super) struct Guard {
    directory: File,
    file: File,
    name: &'static str,
}
fn identity(info: &fs::Stat) -> (u64, u64) {
    (info.st_dev as u64, info.st_ino)
}
fn current(root: &Path) -> Result<File> {
    child(&child(&directory(root)?, ".jj")?, "working_copy")
}
impl Guard {
    pub(super) fn acquire(root: &Path) -> Result<Self> {
        let guard = Self::acquire_in(current(root)?, NAME)?;
        guard.check(root)?;
        Ok(guard)
    }
    pub(super) fn acquire_in(directory: File, name: &'static str) -> Result<Self> {
        // A previous holder may unlink just before releasing flock. Retrying
        // that identity race is bounded and never waits for the holder.
        for _ in 0..4 {
            let file: File = fs::openat(
                &directory,
                name,
                OFlags::RDWR
                    | OFlags::CREATE
                    | OFlags::CLOEXEC
                    | OFlags::NOFOLLOW
                    | OFlags::NONBLOCK,
                Mode::RUSR | Mode::WUSR,
            )
            .map_err(|_| Error::Unavailable)?
            .into();
            let info = fs::fstat(&file).map_err(|_| Error::Unavailable)?;
            if FileType::from_raw_mode(info.st_mode) != FileType::RegularFile || info.st_nlink != 1
            {
                return Err(Error::Invalid);
            }
            match fs::flock(&file, FlockOperation::NonBlockingLockExclusive) {
                Err(rustix::io::Errno::WOULDBLOCK) => return Err(Error::Busy),
                Err(_) => return Err(Error::Unavailable),
                Ok(()) => {}
            }
            if fs::fstat(&file).map_err(|_| Error::Unavailable)?.st_nlink == 0 {
                continue;
            }
            let guard = Self {
                directory,
                file,
                name,
            };
            guard.check_file()?;
            return Ok(guard);
        }
        Err(Error::Busy)
    }
    pub(super) fn check(&self, root: &Path) -> Result<()> {
        self.check_directory(&current(root)?)
    }
    pub(super) fn check_directory(&self, current: &File) -> Result<()> {
        if identity(&fs::fstat(&self.directory).map_err(|_| Error::Unavailable)?)
            != identity(&fs::fstat(current).map_err(|_| Error::Unavailable)?)
        {
            return Err(Error::EpochChanged);
        }
        self.check_file()
    }
    fn check_file(&self) -> Result<()> {
        let held = fs::fstat(&self.file).map_err(|_| Error::Unavailable)?;
        let selected = fs::statat(&self.directory, self.name, AtFlags::SYMLINK_NOFOLLOW)
            .map_err(|_| Error::EpochChanged)?;
        if identity(&held) != identity(&selected)
            || held.st_nlink != 1
            || selected.st_nlink != 1
            || FileType::from_raw_mode(selected.st_mode) != FileType::RegularFile
        {
            return Err(Error::EpochChanged);
        }
        Ok(())
    }
}
impl Drop for Guard {
    fn drop(&mut self) {
        // Leave the empty lock file, as after upstream process loss. Removing
        // it conditionally would race with replacement between stat and unlink.
        // JJ can reuse it normally; flock ownership, not existence, is the lock.
        let _ = fs::flock(&self.file, FlockOperation::Unlock);
    }
}
