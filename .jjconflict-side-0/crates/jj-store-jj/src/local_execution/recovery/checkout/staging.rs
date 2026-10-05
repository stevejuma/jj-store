use super::*;
use rustix::fs::{self, Mode, OFlags};
use std::{fs::File, io::Write as _};
fn identity(directory: &File, name: &str, file: &File) -> Result<()> {
    let held = fs::fstat(file).map_err(|_| Error::Unavailable)?;
    let selected = fs::statat(directory, name, fs::AtFlags::SYMLINK_NOFOLLOW)
        .map_err(|_| Error::EpochChanged)?;
    if held.st_nlink != 1
        || selected.st_nlink != 1
        || (held.st_dev, held.st_ino) != (selected.st_dev, selected.st_ino)
    {
        return Err(Error::EpochChanged);
    }
    Ok(())
}
/// One deterministic staging name per original witness bounds interrupted retry.
/// Partial writes can only append the remaining verified exact byte prefix.
pub(super) fn prepare(
    directory: &File,
    checkout: &CheckoutWitnessV1,
    budget: &Budget,
) -> Result<(String, File)> {
    let name = format!(".lapis-stack-checkout-{}", checkout.id()?);
    let before = physical::read(directory, &name, 65536, true, budget)?;
    let mut file: File = if let Some(saved) = &before {
        if !checkout.after().starts_with(&saved.bytes) {
            return Err(Error::Corrupt);
        }
        fs::openat(
            directory,
            &name,
            OFlags::WRONLY | OFlags::APPEND | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NONBLOCK,
            Mode::empty(),
        )
    } else {
        fs::openat(
            directory,
            &name,
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::RUSR | Mode::WUSR,
        )
    }
    .map_err(|_| Error::Unavailable)?
    .into();
    identity(directory, &name, &file)?;
    let actual =
        physical::read(directory, &name, 65536, false, budget)?.ok_or(Error::Incomplete)?;
    if before.as_ref().is_some_and(|saved| *saved != actual)
        || (before.is_none() && !actual.bytes.is_empty())
    {
        return Err(Error::EpochChanged);
    }
    file.write_all(&checkout.after()[actual.bytes.len()..])
        .map_err(|_| Error::Unavailable)?;
    file.sync_all().map_err(|_| Error::Unavailable)?;
    directory.sync_all().map_err(|_| Error::Unavailable)?;
    budget.check()?;
    identity(directory, &name, &file)?;
    if physical::read(directory, &name, 65536, false, budget)?
        .ok_or(Error::Incomplete)?
        .bytes
        != checkout.after()
    {
        return Err(Error::EpochChanged);
    }
    Ok((name, file))
}
pub(super) fn select(
    directory: &File,
    name: &str,
    file: &File,
    bytes: &[u8],
    budget: &Budget,
) -> Result<()> {
    // Foreground coordination excludes legitimate metadata writers; atomic
    // replacement is not claimed as a filesystem CAS against arbitrary actors.
    budget.check()?;
    identity(directory, name, file)?;
    if physical::read(directory, name, 65536, false, budget)?
        .ok_or(Error::Incomplete)?
        .bytes
        != bytes
    {
        return Err(Error::EpochChanged);
    }
    fs::renameat(directory, name, directory, "checkout").map_err(|_| Error::Unavailable)?;
    directory.sync_all().map_err(|_| Error::Unavailable)
}
