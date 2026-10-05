use super::*;
use crate::local_evidence::GitOperationV1;
use jj_lib::{
    content_hash::blake2b_hash, op_store as o, op_store::OpStore as _,
    simple_op_store::SimpleOpStore,
};
use std::collections::{BTreeMap, BTreeSet};
const MAX_RECORD: usize = 4 * 1024 * 1024;
const MAX_OPERATIONS: usize = 4096;

/// Exact bounded operation ancestry only. No latest repository, index, view
/// traversal, Git extras import or CLI reconciliation participates here.
pub(in crate::local_execution::recovery) async fn selected(
    recovery: &RecoveredHistory,
    heads: &[m::OperationId],
    budget: &Budget,
) -> Result<bool> {
    let source = recovery.witness.binding().operation;
    let candidate = recovery.archive.operation;
    if candidate == source || recovery.archive.operations[&candidate].parents != [source] {
        return Err(Error::Invalid);
    }
    let destination = records::destination::Destination::open(recovery.access.metadata_path())?;
    let private = tempfile::tempdir().map_err(|_| Error::Unavailable)?;
    let decoder = SimpleOpStore::init(
        private.path(),
        o::RootOperationData {
            root_commit_id: jj_lib::backend::CommitId::from_bytes(&[0; 20]),
        },
    )
    .map_err(|_| Error::Unavailable)?;
    let mut pending: BTreeSet<_> = heads.iter().copied().collect();
    let mut parents = BTreeMap::new();
    let mut results: BTreeMap<_, (bool, bool)> = BTreeMap::new();
    let mut raw = 0usize;
    while let Some(id) = pending.pop_first() {
        budget.work(1)?;
        if parents.contains_key(&id) || results.contains_key(&id) {
            continue;
        }
        if parents.len() + results.len() >= MAX_OPERATIONS {
            return Err(Error::Limit);
        }
        if recovery.witness.original_heads().contains(&id) || id == candidate {
            results.insert(id, (true, id == candidate));
            continue;
        }
        if id.as_bytes() == &[0; 64] {
            results.insert(id, (false, false));
            continue;
        }
        let native = if let Some(op) = recovery.archive.operations.get(&id) {
            o::Operation::try_from(op.clone()).map_err(evidence_error)?
        } else {
            destination.check()?;
            let directory = destination.operations.as_ref().ok_or(Error::Incomplete)?;
            let bytes = physical::read(directory, &id.to_string(), MAX_RECORD, false, budget)?
                .ok_or(Error::Incomplete)?;
            raw = raw.checked_add(bytes.bytes.len()).ok_or(Error::Limit)?;
            if raw > crate::local_evidence::MAX_ARCHIVE_BYTES {
                return Err(Error::Limit);
            }
            std::fs::write(
                private.path().join("operations").join(id.to_string()),
                bytes.bytes,
            )
            .map_err(|_| Error::Unavailable)?;
            budget
                .until(decoder.read_operation(&o::OperationId::from_bytes(id.as_bytes())))
                .await?
                .map_err(|_| Error::Corrupt)?
        };
        if m::OperationId::new(blake2b_hash(&native).into()) != id {
            return Err(Error::Corrupt);
        }
        let op = GitOperationV1::try_from(&native).map_err(evidence_error)?;
        if op.parents.is_empty() || op.parents.len() > 64 {
            return Err(Error::Invalid);
        }
        pending.extend(op.parents.iter().copied());
        parents.insert(id, op.parents);
    }
    destination.check()?;
    // Bottom-up reachability also detects incomplete/cyclic input, with linear
    // bounded edge work rather than recursive or unbounded traversal.
    let mut children: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    let mut outstanding = BTreeMap::new();
    for (id, edges) in &parents {
        let unique: BTreeSet<_> = edges.iter().copied().collect();
        outstanding.insert(*id, unique.len());
        for parent in unique {
            children.entry(parent).or_default().insert(*id);
        }
    }
    let mut ready: BTreeSet<_> = results.keys().copied().collect();
    while let Some(id) = ready.pop_first() {
        for child in children.get(&id).into_iter().flatten() {
            budget.work(1)?;
            let remaining = outstanding.get_mut(child).ok_or(Error::Incomplete)?;
            *remaining -= 1;
            if *remaining == 0 {
                let mut found = (false, false);
                for parent in &parents[child] {
                    let value = results.get(parent).ok_or(Error::Incomplete)?;
                    found.0 |= value.0;
                    found.1 |= value.1;
                }
                results.insert(*child, found);
                ready.insert(*child);
            }
        }
    }
    let mut published = false;
    for id in heads {
        let (descendant, includes_candidate) = results.get(id).ok_or(Error::Incomplete)?;
        if !descendant {
            return Err(Error::EpochChanged);
        }
        published |= includes_candidate;
    }
    Ok(published)
}
