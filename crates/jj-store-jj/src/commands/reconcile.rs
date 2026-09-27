//! Explicit native operation merge. No head store mutation is available here.
use super::*;
use jj_lib::{object_id::ObjectId as _, op_store as o, repo::ReadonlyRepo};
use std::collections::BTreeSet;

fn unavailable(_: impl std::fmt::Debug) -> Error {
    query::Error::Unavailable.into()
}

pub(super) async fn merge(
    base_repo: &Arc<ReadonlyRepo>,
    heads: &BTreeSet<m::OperationId>,
    base: m::OperationId,
    principal: m::PrincipalId,
    budget: &Budget,
) -> Result<Arc<ReadonlyRepo>> {
    if !(2..=64).contains(&heads.len())
        || !heads.contains(&base)
        || base_repo.op_id().as_bytes() != base.as_bytes()
    {
        return Err(Error::Invalid);
    }
    let loader = base_repo.loader();
    let mut pending: Vec<_> = heads.iter().copied().collect();
    let mut visited = BTreeSet::new();
    while let Some(id) = pending.pop() {
        budget.work(1)?;
        if !visited.insert(id) {
            continue;
        }
        let op = loader
            .load_operation(&o::OperationId::from_bytes(id.as_bytes()))
            .await
            .map_err(unavailable)?;
        for parent in op.parent_ids() {
            budget.work(1)?;
            pending.push(m::OperationId::from_slice(parent.as_bytes()).map_err(unavailable)?);
        }
    }
    // Reserve conservative pairwise ancestor work before upstream's cached
    // recursive merge. This also bounds its inputs when no further I/O occurs.
    let reserved = visited
        .len()
        .checked_mul(visited.len())
        .and_then(|n| n.checked_mul(heads.len()))
        .ok_or(query::Error::Limit)?;
    budget.work(reserved)?;
    let mut operations = Vec::with_capacity(heads.len());
    for id in std::iter::once(&base).chain(heads.iter().filter(|id| **id != base)) {
        budget.check()?;
        operations.push(
            loader
                .load_operation(&o::OperationId::from_bytes(id.as_bytes()))
                .await
                .map_err(unavailable)?,
        );
    }
    let (merged, _) = loader
        .merge_operations(
            operations,
            None,
            Some("jj-store: reconcile operations"),
            [("jj-store.principal".into(), principal.to_string())],
        )
        .await
        .map_err(unavailable)?;
    budget.check()?;
    Ok(merged)
}
