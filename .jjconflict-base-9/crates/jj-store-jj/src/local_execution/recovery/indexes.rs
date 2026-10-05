//! Explicit archive-only restoration of missing original/candidate commit indexes.
mod archive;
mod destination;
mod materialize;
mod verify;
use super::records::synced;
use super::*;
use crate::local_browser::working_tree::current::physical;
const MAX_SEGMENT: usize = 4 * 1024 * 1024;
const MAX_BYTES: usize = 16 * 1024 * 1024;
pub(super) struct Validation {
    destination: destination::Destination,
    snapshot: destination::Snapshot,
}
impl Validation {
    pub(super) fn check(&self, budget: &Budget) -> Result<()> {
        if self.destination.capture(budget)? != self.snapshot {
            return Err(Error::EpochChanged);
        }
        Ok(())
    }
}
pub(super) async fn validate(recovery: &RecoveredHistory, budget: &Budget) -> Result<Validation> {
    let destination = destination::Destination::open(recovery.access.metadata_path())?;
    let snapshot = destination.capture(budget)?;
    if snapshot.kind.is_none()
        || !materialize::missing(recovery, &snapshot, budget)
            .await?
            .is_empty()
    {
        return Err(Error::Incomplete);
    }
    let validation = Validation {
        destination,
        snapshot,
    };
    validation.check(budget)?;
    Ok(validation)
}
pub(super) async fn restore(recovery: &RecoveredHistory, budget: &Budget) -> Result<()> {
    recovery.check(budget).await?;
    let mut destination = destination::Destination::open(recovery.access.metadata_path())?;
    let mut saved = destination.capture(budget)?;
    let output = materialize::missing(recovery, &saved, budget).await?;
    recovery.check(budget).await?;
    if destination.capture(budget)? != saved {
        return Err(Error::EpochChanged);
    }
    destination.ensure()?;
    let index = destination.index.as_ref().unwrap();
    if saved.kind.is_none() {
        synced::select(index, "type", b"default", budget)?;
        saved.kind = physical::read(index, "type", 64, false, budget)?;
    }
    // Immutable segments are durable before any operation association is selected.
    for group in ["segments", "op_links"] {
        for ((directory, name), bytes) in output.iter().filter(|((dir, _), _)| dir == group) {
            recovery.check(budget).await?;
            if destination.capture(budget)? != saved {
                return Err(Error::EpochChanged);
            }
            let dir = destination.directory(directory)?;
            let key = (directory.clone(), name.clone());
            if !saved.files.contains_key(&key) {
                synced::select(dir, name, bytes, budget)?;
            }
            let actual =
                physical::read(dir, name, MAX_SEGMENT, false, budget)?.ok_or(Error::Unavailable)?;
            if actual.bytes != *bytes {
                return Err(Error::EpochChanged);
            }
            synced::acknowledge(dir, name, budget)?;
            saved.files.insert(key, actual);
        }
        destination
            .directory(group)?
            .sync_all()
            .map_err(|_| Error::Unavailable)?;
    }
    synced::acknowledge(index, "type", budget)?;
    for (directory, name) in saved.files.keys() {
        recovery.check(budget).await?;
        destination.check()?;
        synced::acknowledge(destination.directory(directory)?, name, budget)?;
    }
    index.sync_all().map_err(|_| Error::Unavailable)?;
    recovery.check(budget).await?;
    if destination.capture(budget)? != saved {
        return Err(Error::EpochChanged);
    }
    Ok(())
}
