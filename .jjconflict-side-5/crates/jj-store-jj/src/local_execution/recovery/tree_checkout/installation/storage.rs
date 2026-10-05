use super::*;
use rustix::fs::{AtFlags, Mode, OFlags};
use std::{fs::File, io::Write as _};
pub(in crate::local_execution::recovery::tree_checkout) struct Destination {
    path: PathBuf,
    working_copy: File,
    name: String,
    pub directory: File,
    selected: Option<physical::Bytes>,
}
impl Destination {
    pub fn open(
        recovery: &RecoveredHistory,
        plan: &TreeCheckoutPlanV1,
        budget: &Budget,
    ) -> Result<Self> {
        budget.check()?;
        let path = recovery.root.join(".jj/working_copy");
        let working_copy = physical::directory(&path)?;
        let name = format!(".lapis-stack-tree-{}", plan.id()?);
        let directory = match records::destination::optional(&working_copy, &name)? {
            Some(dir) => dir,
            None => records::destination::create(&working_copy, &name)?,
        };
        if directory_stamp(&directory)?[3] & 0o777 != 0o700 {
            return Err(Error::Invalid);
        }
        let selected =
            physical::read(&directory, "progress", progress::MAX_PROGRESS, true, budget)?;
        let value = Self {
            path,
            working_copy,
            name,
            directory,
            selected,
        };
        value.check(recovery)?;
        Ok(value)
    }
    /// Reopening completed installation must never recreate lost evidence.
    pub fn existing(
        recovery: &RecoveredHistory,
        plan: &TreeCheckoutPlanV1,
        budget: &Budget,
    ) -> Result<Self> {
        let path = recovery.root.join(".jj/working_copy");
        let working_copy = physical::directory(&path)?;
        let name = format!(".lapis-stack-tree-{}", plan.id()?);
        let directory = physical::child(&working_copy, &name)?;
        let selected = physical::read(
            &directory,
            "progress",
            progress::MAX_PROGRESS,
            false,
            budget,
        )?;
        let value = Self {
            path,
            working_copy,
            name,
            directory,
            selected,
        };
        value.check(recovery)?;
        Ok(value)
    }
    pub fn check(&self, recovery: &RecoveredHistory) -> Result<()> {
        if directory_stamp(&self.directory)?[3] & 0o7777 != 0o700 {
            return Err(Error::EpochChanged);
        }
        records::destination::same(&self.working_copy, &physical::directory(&self.path)?)?;
        records::destination::same(
            &self.directory,
            &physical::child(&self.working_copy, &self.name)?,
        )?;
        recovery.guard.check(&recovery.root)
    }
    pub fn load(&self, budget: &Budget) -> Result<Option<Progress>> {
        budget.check()?;
        self.selected
            .as_ref()
            .map(|data| {
                let state: Progress =
                    serde_json::from_slice(&data.bytes).map_err(|_| Error::Corrupt)?;
                if state.encode()? != data.bytes {
                    return Err(Error::Corrupt);
                }
                Ok(state)
            })
            .transpose()
    }
    pub fn verify_next(&self, expected: Option<&Progress>, budget: &Budget) -> Result<()> {
        if let Some(data) = physical::read(
            &self.directory,
            "next",
            progress::MAX_PROGRESS,
            true,
            budget,
        )? {
            let expected = expected.ok_or(Error::Corrupt)?.encode()?;
            if !expected.starts_with(&data.bytes) {
                return Err(Error::Corrupt);
            }
            let info = fs::statat(&self.directory, "next", AtFlags::SYMLINK_NOFOLLOW)
                .map_err(|_| Error::EpochChanged)?;
            if info.st_mode & 0o7777 != 0o600 {
                return Err(Error::EpochChanged);
            }
        }
        Ok(())
    }
    pub fn save(&mut self, state: &Progress, budget: &Budget) -> Result<()> {
        let bytes = state.encode()?;
        let next = physical::read(
            &self.directory,
            "next",
            progress::MAX_PROGRESS,
            true,
            budget,
        )?;
        if next
            .as_ref()
            .is_some_and(|saved| !bytes.starts_with(&saved.bytes))
        {
            return Err(Error::Corrupt);
        }
        let flags = OFlags::WRONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::NONBLOCK;
        let mut file: File = fs::openat(
            &self.directory,
            "next",
            if next.is_some() {
                flags | OFlags::APPEND
            } else {
                flags | OFlags::CREATE | OFlags::EXCL
            },
            Mode::RUSR | Mode::WUSR,
        )
        .map_err(|_| Error::Unavailable)?
        .into();
        let actual = physical::read(
            &self.directory,
            "next",
            progress::MAX_PROGRESS,
            false,
            budget,
        )?
        .ok_or(Error::Incomplete)?;
        if next.as_ref().is_some_and(|old| old != &actual)
            || (next.is_none() && !actual.bytes.is_empty())
        {
            return Err(Error::EpochChanged);
        }
        identity(&self.directory, "next", &file)?;
        file.write_all(&bytes[actual.bytes.len()..])
            .map_err(|_| Error::Unavailable)?;
        file.sync_all().map_err(|_| Error::Unavailable)?;
        if physical::read(
            &self.directory,
            "next",
            progress::MAX_PROGRESS,
            false,
            budget,
        )?
        .ok_or(Error::Incomplete)?
        .bytes
            != bytes
            || physical::read(
                &self.directory,
                "progress",
                progress::MAX_PROGRESS,
                true,
                budget,
            )? != self.selected
        {
            return Err(Error::EpochChanged);
        }
        identity(&self.directory, "next", &file)?;
        // Legitimate progress writers hold the native foreground lease. This
        // private metadata replacement is not an arbitrary-actor filesystem CAS.
        if self.selected.is_some() {
            fs::renameat(&self.directory, "next", &self.directory, "progress")
                .map_err(|_| Error::Unavailable)?;
        } else {
            fs::renameat_with(
                &self.directory,
                "next",
                &self.directory,
                "progress",
                RenameFlags::NOREPLACE,
            )
            .map_err(|_| Error::EpochChanged)?;
        }
        self.directory.sync_all().map_err(|_| Error::Unavailable)?;
        self.selected = physical::read(
            &self.directory,
            "progress",
            progress::MAX_PROGRESS,
            false,
            budget,
        )?;
        if self
            .selected
            .as_ref()
            .is_none_or(|data| data.bytes != bytes)
        {
            return Err(Error::EpochChanged);
        }
        Ok(())
    }
    pub fn acknowledge(&self, budget: &Budget) -> Result<()> {
        if physical::read(
            &self.directory,
            "progress",
            progress::MAX_PROGRESS,
            true,
            budget,
        )? != self.selected
        {
            return Err(Error::EpochChanged);
        }
        let file: File = fs::openat(
            &self.directory,
            "progress",
            OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW,
            Mode::empty(),
        )
        .map_err(|_| Error::Unavailable)?
        .into();
        identity(&self.directory, "progress", &file)?;
        file.sync_all().map_err(|_| Error::Unavailable)?;
        self.directory.sync_all().map_err(|_| Error::Unavailable)?;
        if physical::read(
            &self.directory,
            "progress",
            progress::MAX_PROGRESS,
            true,
            budget,
        )? != self.selected
        {
            return Err(Error::EpochChanged);
        }
        Ok(())
    }
}
fn identity(dir: &File, name: &str, file: &File) -> Result<()> {
    let held = fs::fstat(file).map_err(|_| Error::Unavailable)?;
    let actual =
        fs::statat(dir, name, AtFlags::SYMLINK_NOFOLLOW).map_err(|_| Error::EpochChanged)?;
    if held.st_nlink != 1
        || held.st_mode & 0o777 != 0o600
        || (held.st_dev, held.st_ino) != (actual.st_dev, actual.st_ino)
    {
        return Err(Error::EpochChanged);
    }
    Ok(())
}
