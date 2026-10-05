//! Operation-visible changes grouped without inventing rewrite relationships.
use super::*;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalOperationChangesRequest")]
pub struct Request {
    pub operation: m::OperationId,
    pub parent: m::OperationId,
    pub limit: usize,
    pub anchor: Option<String>,
    pub direction: Direction,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
#[schemars(rename = "LocalOperationChangesComparison")]
pub enum Comparison {
    Exact {
        from: m::GitSha1Id,
        to: m::GitSha1Id,
    },
    Select {
        before: Vec<m::GitSha1Id>,
        after: Vec<m::GitSha1Id>,
    },
    Unavailable,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalOperationChangesGroup")]
pub struct Group {
    pub change: m::ChangeId,
    pub before: Vec<m::GitSha1Id>,
    pub after: Vec<m::GitSha1Id>,
    pub comparison: Comparison,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalOperationChangesPage")]
pub struct Page {
    pub version: u32,
    pub operation: m::OperationId,
    pub parent: m::OperationId,
    pub groups: Vec<Group>,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub stats: Stats,
}
#[derive(Default)]
struct Pending {
    before: BTreeSet<m::GitSha1Id>,
    after: BTreeSet<m::GitSha1Id>,
}
pub(super) async fn read(context: Context, request: Request) -> Result<Page> {
    if request.limit == 0
        || request.limit > context.limits.max_page
        || request.anchor.as_ref().is_some_and(|v| v.len() > 64)
    {
        return Err(Error::Limit);
    }
    let operation = context.operation(request.operation).await?;
    if !operation.parents.contains(&request.parent) {
        return Err(Error::Invalid);
    }
    let difference = context
        .visibility_delta(request.operation, &operation, request.parent)
        .await?;
    let mut groups: BTreeMap<m::ChangeId, Pending> = BTreeMap::new();
    // Admit grouping nodes and target IDs before growth. Complete differences
    // are still visit-bounded; only the returned window hydrates added commits.
    let _memory = crate::query::memory::Lease::reserve(
        (difference.removed.len() as u64 + difference.added.len() as u64)
            .checked_mul(512)
            .ok_or(Error::Limit)?
            .checked_add(8192)
            .ok_or(Error::Limit)?,
    )?;
    for id in &difference.removed {
        let change = change_identity(&context, *id).await?;
        groups.entry(change).or_default().before.insert(*id);
    }
    for id in &difference.added {
        let change = change_identity(&context, *id).await?;
        let group = groups.entry(change).or_default();
        group.after.insert(*id);
    }
    let mut entries = vec![];
    for (change, pending) in groups {
        context.budget.work(1)?;
        entries.push((change.to_string(), change, pending));
    }
    let boundary = request.anchor.as_deref();
    let (start, end) = match request.direction {
        Direction::Forward => {
            let start = boundary.map_or(0, |anchor| {
                entries.partition_point(|e| e.0.as_str() <= anchor)
            });
            (start, (start + request.limit).min(entries.len()))
        }
        Direction::Backward => {
            let end = boundary.map_or(entries.len(), |anchor| {
                entries.partition_point(|e| e.0.as_str() < anchor)
            });
            (end.saturating_sub(request.limit), end)
        }
    };
    let previous = (start > 0 && start < end).then(|| entries[start].0.clone());
    let next = (end < entries.len() && start < end).then(|| entries[end - 1].0.clone());
    let mut groups = vec![];
    let mut association_memory = vec![];
    for (_, change, mut pending) in entries.into_iter().skip(start).take(end - start) {
        for id in &pending.after {
            if let Some(predecessors) = operation
                .commit_predecessors
                .as_ref()
                .and_then(|map| map.get(id))
            {
                context.budget.work(predecessors.len())?;
                association_memory.push(crate::query::memory::Lease::reserve(
                    (predecessors.len() as u64)
                        .checked_mul(256)
                        .ok_or(Error::Limit)?,
                )?);
                pending.before.extend(predecessors.iter().copied());
            } else if pending.before.is_empty() {
                let commit = context.commit_dto(*id).await?;
                if commit.change_id != change {
                    return Err(Error::Corrupt);
                }
                context.budget.work(commit.parents.len())?;
                association_memory.push(crate::query::memory::Lease::reserve(
                    (commit.parents.len() as u64)
                        .checked_mul(256)
                        .ok_or(Error::Limit)?,
                )?);
                pending.before.extend(commit.parents);
            }
        }
        let before: Vec<_> = pending.before.into_iter().collect();
        let after: Vec<_> = pending.after.into_iter().collect();
        let comparison = match (before.as_slice(), after.as_slice()) {
            ([from], [to]) => Comparison::Exact {
                from: *from,
                to: *to,
            },
            ([], _) | (_, []) => Comparison::Unavailable,
            _ => Comparison::Select {
                before: before.clone(),
                after: after.clone(),
            },
        };
        let group = Group {
            change,
            before,
            after,
            comparison,
        };
        context.budget.output(&group)?;
        groups.push(group);
    }
    let result = Page {
        version: 1,
        operation: request.operation,
        parent: request.parent,
        groups,
        previous,
        next,
        stats: context.budget.stats(),
    };
    context.budget.complete_output(&result)?;
    Ok(result)
}
async fn change_identity(context: &Context, id: m::GitSha1Id) -> Result<m::ChangeId> {
    if let Some(prepared) = &context.access.prepared {
        prepared.change_identity(id, &context.budget)
    } else {
        Ok(context.commit_dto(id).await?.change_id)
    }
}
