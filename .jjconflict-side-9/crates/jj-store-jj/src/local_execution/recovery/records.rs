pub(super) mod destination;
pub(super) mod synced;
use super::*;
use crate::local_browser::working_tree::current::physical;
use jj_lib::{
    op_store::{self as o, OpStore as _},
    simple_op_store::SimpleOpStore,
};
use std::{fs::File, path::Path};
const MAX_RECORD: usize = 4 * 1024 * 1024;
const MAX_RAW: usize = crate::local_evidence::MAX_ARCHIVE_BYTES;
pub(super) async fn restore(recovery: &RecoveredHistory, budget: &Budget) -> Result<()> {
    let archive = &recovery.archive;
    let materialized = budget
        .until(archive.materialize())
        .await?
        .map_err(evidence_error)?;
    let materialized_path = materialized
        .path()
        .canonicalize()
        .map_err(|_| Error::Unavailable)?;
    let mut destination = destination::Destination::open(recovery.access.metadata_path())?;
    destination.check()?;
    let observation = tempfile::tempdir().map_err(|_| Error::Unavailable)?;
    let decoder = SimpleOpStore::init(
        observation.path(),
        o::RootOperationData {
            root_commit_id: jj_lib::backend::CommitId::from_bytes(&[0; 20]),
        },
    )
    .map_err(|_| Error::Unavailable)?;
    let mut records = Vec::new();
    let mut raw = 0usize;
    for (id, data) in &archive.views {
        if id.as_bytes() == &[0; 64] {
            continue;
        }
        let name = id.to_string();
        let bytes = private_bytes(&materialized_path, "views", &name, budget, &mut raw)?;
        if let Some(existing) = existing(destination.views.as_ref(), &name, budget, &mut raw)? {
            std::fs::write(observation.path().join("views").join(&name), &existing)
                .map_err(|_| Error::Unavailable)?;
            let decoded = decoder
                .read_view(&o::ViewId::from_bytes(id.as_bytes()))
                .await
                .map_err(|_| Error::Corrupt)?;
            if decoded != data.clone().try_into().map_err(evidence_error)? {
                return Err(Error::Corrupt);
            }
            records.push(("views", name, existing, false));
        } else {
            records.push(("views", name, bytes, true));
        }
    }
    for (id, data) in &archive.operations {
        if id.as_bytes() == &[0; 64] {
            continue;
        }
        let name = id.to_string();
        let bytes = private_bytes(&materialized_path, "operations", &name, budget, &mut raw)?;
        if let Some(existing) = existing(destination.operations.as_ref(), &name, budget, &mut raw)?
        {
            std::fs::write(observation.path().join("operations").join(&name), &existing)
                .map_err(|_| Error::Unavailable)?;
            let decoded = decoder
                .read_operation(&o::OperationId::from_bytes(id.as_bytes()))
                .await
                .map_err(|_| Error::Corrupt)?;
            if decoded != data.clone().try_into().map_err(evidence_error)? {
                return Err(Error::Corrupt);
            }
            records.push(("operations", name, existing, false));
        } else {
            records.push(("operations", name, bytes, true));
        }
    }
    if let Some(store) = &destination.store
        && let Some(kind) = physical::read(store, "type", 64, true, budget)?
        && kind.bytes != SimpleOpStore::name().as_bytes()
    {
        return Err(Error::Invalid);
    }
    // No source directories or records are created before complete conflict preflight.
    recovery.check(budget).await?;
    destination.ensure()?;
    destination.check()?;
    let store = destination.store.as_ref().ok_or(Error::Unavailable)?;
    if physical::read(store, "type", 64, true, budget)?.is_none() {
        synced::select(store, "type", SimpleOpStore::name().as_bytes(), budget)?;
    }
    for (group, name, bytes, missing) in &records {
        if !missing {
            continue;
        }
        budget.until(recovery.access.authorize()).await??;
        recovery.guard.check(&recovery.root)?;
        destination.check()?;
        let dir = match *group {
            "views" => &destination.views,
            _ => &destination.operations,
        };
        synced::select(dir.as_ref().ok_or(Error::Unavailable)?, name, bytes, budget)?;
    }
    // Sync both prior and newly created entries before acknowledging exact replay.
    for (group, name, bytes, _) in records {
        let dir = match group {
            "views" => destination.views.as_ref().unwrap(),
            _ => destination.operations.as_ref().unwrap(),
        };
        if physical::read(dir, &name, MAX_RECORD, false, budget)?
            .ok_or(Error::Unavailable)?
            .bytes
            != bytes
        {
            return Err(Error::EpochChanged);
        }
        synced::acknowledge(dir, &name, budget)?;
    }
    destination
        .views
        .as_ref()
        .unwrap()
        .sync_all()
        .map_err(|_| Error::Unavailable)?;
    destination
        .operations
        .as_ref()
        .unwrap()
        .sync_all()
        .map_err(|_| Error::Unavailable)?;
    if physical::read(store, "type", 64, false, budget)?
        .ok_or(Error::Unavailable)?
        .bytes
        != SimpleOpStore::name().as_bytes()
    {
        return Err(Error::EpochChanged);
    }
    synced::acknowledge(store, "type", budget)?;
    store.sync_all().map_err(|_| Error::Unavailable)?;
    destination.check()?;
    Ok(())
}
fn charge(raw: &mut usize, n: usize) -> Result<()> {
    *raw = raw.checked_add(n).ok_or(Error::Limit)?;
    if *raw > MAX_RAW {
        return Err(Error::Limit);
    }
    Ok(())
}
fn private_bytes(
    root: &Path,
    group: &str,
    name: &str,
    budget: &Budget,
    raw: &mut usize,
) -> Result<Vec<u8>> {
    let dir = physical::child(&physical::directory(root)?, group)?;
    let bytes = physical::read(&dir, name, MAX_RECORD, false, budget)?
        .ok_or(Error::Unavailable)?
        .bytes;
    charge(raw, bytes.len())?;
    Ok(bytes)
}
fn existing(
    dir: Option<&File>,
    name: &str,
    budget: &Budget,
    raw: &mut usize,
) -> Result<Option<Vec<u8>>> {
    let Some(dir) = dir else {
        return Ok(None);
    };
    let bytes = physical::read(dir, name, MAX_RECORD, true, budget)?.map(|data| data.bytes);
    if let Some(bytes) = &bytes {
        charge(raw, bytes.len())?;
    }
    Ok(bytes)
}
