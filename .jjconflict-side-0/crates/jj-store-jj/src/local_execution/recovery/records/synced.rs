use super::*;
use rustix::fs::{self, AtFlags, Mode, OFlags, RenameFlags};
use std::{
    io::Write as _,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
/// Synced no-replace rename. An interrupted staging write cannot select a partial record.
pub(in crate::local_execution::recovery) fn select(
    dir: &File,
    name: &str,
    bytes: &[u8],
    budget: &Budget,
) -> Result<()> {
    budget.work(1)?;
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| Error::Unavailable)?
        .as_nanos();
    let staging = format!(
        ".lapis-stack-recovery-{}-{nonce}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    );
    let flags = OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC;
    let mut file: File = fs::openat(dir, &staging, flags, Mode::RUSR | Mode::WUSR)
        .map_err(|_| Error::Unavailable)?
        .into();
    file.write_all(bytes).map_err(|_| Error::Unavailable)?;
    file.sync_all().map_err(|_| Error::Unavailable)?;
    budget.check()?;
    if physical::read(dir, &staging, MAX_RECORD, false, budget)?
        .ok_or(Error::Unavailable)?
        .bytes
        != bytes
    {
        return Err(Error::EpochChanged);
    }
    let held = fs::fstat(&file).map_err(|_| Error::Unavailable)?;
    let selected =
        fs::statat(dir, &staging, AtFlags::SYMLINK_NOFOLLOW).map_err(|_| Error::EpochChanged)?;
    if (held.st_dev, held.st_ino) != (selected.st_dev, selected.st_ino) {
        return Err(Error::EpochChanged);
    }
    // No platform fallback may silently overwrite an existing object.
    fs::renameat_with(dir, &staging, dir, name, RenameFlags::NOREPLACE)
        .map_err(|_| Error::EpochChanged)?;
    dir.sync_all().map_err(|_| Error::Unavailable)?;
    acknowledge(dir, name, budget)
}
pub(in crate::local_execution::recovery) fn acknowledge(
    dir: &File,
    name: &str,
    budget: &Budget,
) -> Result<()> {
    let bytes = physical::read(dir, name, MAX_RECORD, false, budget)?.ok_or(Error::Unavailable)?;
    let file: File = fs::openat(
        dir,
        name,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .map_err(|_| Error::Unavailable)?
    .into();
    file.sync_all().map_err(|_| Error::Unavailable)?;
    if physical::read(dir, name, MAX_RECORD, false, budget)?.ok_or(Error::Unavailable)? != bytes {
        return Err(Error::EpochChanged);
    }
    let held = fs::fstat(&file).map_err(|_| Error::Unavailable)?;
    let actual =
        fs::statat(dir, name, AtFlags::SYMLINK_NOFOLLOW).map_err(|_| Error::EpochChanged)?;
    if (held.st_dev, held.st_ino) != (actual.st_dev, actual.st_ino) {
        return Err(Error::EpochChanged);
    }
    Ok(())
}
