//! Operation-visible changes grouped without inventing rewrite relationships.
use super::*;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub operation: m::OperationId,
    pub parent: m::OperationId,
    pub limit: usize,
    pub anchor: Option<String>,
    pub direction: Direction,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Comparison {
    Exact {
        from: m::CommitId,
        to: m::CommitId,
    },
    Select {
        before: Vec<m::CommitId>,
        after: Vec<m::CommitId>,
    },
    Unavailable,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
pub struct Group {
    pub change: m::ChangeId,
    pub before: Vec<m::CommitId>,
    pub after: Vec<m::CommitId>,
    pub comparison: Comparison,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
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
    before: BTreeSet<m::CommitId>,
    after: BTreeSet<m::CommitId>,
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
        .difference(request.operation, operation, request.parent)
        .await?;
    let mut groups: BTreeMap<m::ChangeId, Pending> = BTreeMap::new();
    for id in &difference.visible_commits.removed {
        let commit = context.commit_dto(*id).await?;
        groups
            .entry(commit.change_id)
            .or_default()
            .before
            .insert(*id);
    }
    for id in &difference.visible_commits.added {
        let commit = context.commit_dto(*id).await?;
        let group = groups.entry(commit.change_id).or_default();
        group.after.insert(*id);
        if let Some(predecessors) = difference
            .commit_predecessors
            .as_ref()
            .and_then(|map| map.get(id))
        {
            group.before.extend(predecessors.iter().copied());
        } else if group.before.is_empty() {
            group.before.extend(commit.parents);
        }
    }
    let mut entries = vec![];
    for (change, pending) in groups {
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
        entries.push((
            change.to_string(),
            Group {
                change,
                before,
                after,
                comparison,
            },
        ));
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
    let groups = entries
        .into_iter()
        .skip(start)
        .take(end - start)
        .map(|(_, group)| group)
        .collect();
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
