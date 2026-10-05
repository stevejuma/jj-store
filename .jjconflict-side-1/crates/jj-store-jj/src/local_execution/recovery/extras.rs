//! Explicit missing-extra repair; exact immutable metadata, never implicit import.
mod destination;
mod materialize;
use super::records::synced;
use super::*;
use crate::local_browser::working_tree::current::physical;
const MAX_SEGMENT: usize = 4 * 1024 * 1024;
const MAX_EXTRA_BYTES: usize = 16 * 1024 * 1024;
pub(super) async fn restore(recovery: &RecoveredHistory, budget: &Budget) -> Result<()> {
    recovery.check(budget).await?;
    let mut destination = destination::Destination::open(recovery.access.metadata_path())?;
    // Existing extra-table locks use the same pinned nonblocking flock protocol.
    // Missing directories are created only after complete private preflight.
    let mut lock = destination
        .extra
        .as_ref()
        .map(|dir| dir.try_clone().map_err(|_| Error::Unavailable))
        .transpose()?
        .map(|dir| guard::Guard::acquire_in(dir, "lock"))
        .transpose()?;
    let mut snapshot = destination.capture(budget)?;
    let output = materialize::missing(recovery, &snapshot, budget)?;
    recovery.check(budget).await?;
    if destination.capture(budget)? != snapshot {
        return Err(Error::EpochChanged);
    }
    destination.ensure()?;
    let extra = destination.extra.as_ref().ok_or(Error::Unavailable)?;
    let heads = destination.heads.as_ref().ok_or(Error::Unavailable)?;
    if lock.is_none() {
        lock = Some(guard::Guard::acquire_in(
            extra.try_clone().map_err(|_| Error::Unavailable)?,
            "lock",
        )?);
    }
    let lock = lock.as_ref().unwrap();
    lock.check_directory(extra)?;
    if destination.capture(budget)? != snapshot {
        return Err(Error::EpochChanged);
    }
    for (name, bytes) in &output.segments {
        recovery.check(budget).await?;
        destination.check()?;
        lock.check_directory(extra)?;
        if !snapshot.segments.contains_key(name) {
            synced::select(extra, name, bytes, budget)?;
        }
        let actual =
            physical::read(extra, name, MAX_SEGMENT, false, budget)?.ok_or(Error::Unavailable)?;
        if actual.bytes != *bytes {
            return Err(Error::EpochChanged);
        }
        synced::acknowledge(extra, name, budget)?;
        snapshot.segments.insert(name.clone(), actual);
    }
    for name in &output.heads {
        recovery.check(budget).await?;
        destination.check()?;
        lock.check_directory(extra)?;
        if destination.capture(budget)? != snapshot {
            return Err(Error::EpochChanged);
        }
        if !snapshot.heads.contains_key(name) {
            synced::select(heads, name, &[], budget)?;
        }
        let actual = physical::read(heads, name, 0, false, budget)?.ok_or(Error::Unavailable)?;
        synced::acknowledge(heads, name, budget)?;
        snapshot.heads.insert(name.clone(), actual);
    }
    recovery.check(budget).await?;
    destination.check()?;
    lock.check_directory(extra)?;
    if destination.capture(budget)? != snapshot {
        return Err(Error::EpochChanged);
    }
    // Also sync prior selected segments/heads before acknowledging exact retry.
    for name in snapshot.segments.keys() {
        synced::acknowledge(extra, name, budget)?;
    }
    for name in snapshot.heads.keys() {
        synced::acknowledge(heads, name, budget)?;
    }
    extra.sync_all().map_err(|_| Error::Unavailable)?;
    heads.sync_all().map_err(|_| Error::Unavailable)?;
    recovery.check(budget).await?;
    destination.check()?;
    lock.check_directory(extra)?;
    if destination.capture(budget)? != snapshot {
        return Err(Error::EpochChanged);
    }
    Ok(())
}
