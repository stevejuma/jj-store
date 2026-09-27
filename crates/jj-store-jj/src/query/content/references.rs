//! Bounded bookmark and visible-change lookup at one native operation.
use super::*;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Bookmarks,
    Changes,
    #[default]
    All,
}
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub operation: m::OperationId,
    pub query: String,
    pub kind: Kind,
    pub limit: usize,
    pub anchor: Option<String>,
    pub direction: Direction,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
pub struct Target {
    pub commit: m::CommitId,
    pub change: m::ChangeId,
    pub parents: Vec<m::CommitId>,
    pub description: String,
    pub author: dto::Signature,
    pub committer: dto::Signature,
    pub conflicted: bool,
    pub workspaces: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Reference {
    Bookmark {
        name: String,
        terms: Vec<Option<m::CommitId>>,
        conflicted: bool,
        workspaces: Vec<String>,
        targets: Vec<Target>,
    },
    Change {
        change: m::ChangeId,
        divergent: bool,
        targets: Vec<Target>,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
pub struct Page {
    pub version: u32,
    pub operation: m::OperationId,
    pub references: Vec<Reference>,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub stats: Stats,
}
struct Entry {
    key: String,
    value: Reference,
}
pub(super) async fn read(context: Context, request: Request) -> Result<Page> {
    if request.limit == 0
        || request.limit > context.limits.max_page
        || request.query.len() > 1024
        || request.anchor.as_ref().is_some_and(|v| v.len() > 4096)
    {
        return Err(Error::Limit);
    }
    let operation = context.operation(request.operation).await?;
    let view = context.view(operation.view_id).await?;
    let mut entries = vec![];
    if request.kind != Kind::Changes {
        for (name, target) in &view.local_bookmarks {
            context.budget.work(1)?;
            let ids: BTreeSet<_> = target.0.iter().flatten().copied().collect();
            let targets = context.targets(&view, ids.iter().copied()).await?;
            let workspaces = workspace_names(&view, &ids);
            entries.push(Entry {
                key: format!("bookmark:{name}"),
                value: Reference::Bookmark {
                    name: name.clone(),
                    terms: target.0.clone(),
                    conflicted: target.0.len() != 1,
                    workspaces,
                    targets,
                },
            });
        }
    }
    if request.kind != Kind::Bookmarks {
        let ids = context
            .ancestors(view.head_ids.iter().copied().collect(), &mut 0)
            .await?;
        let mut changes: BTreeMap<m::ChangeId, BTreeSet<m::CommitId>> = BTreeMap::new();
        for id in ids {
            let commit = context.commit_dto(id).await?;
            changes.entry(commit.change_id).or_default().insert(id);
        }
        for (change, ids) in changes {
            let targets = context.targets(&view, ids.iter().copied()).await?;
            entries.push(Entry {
                key: format!("change:{change}"),
                value: Reference::Change {
                    change,
                    divergent: ids.len() > 1,
                    targets,
                },
            });
        }
    }
    let needle = request.query.to_lowercase();
    entries.retain(|entry| {
        needle.is_empty()
            || entry.key.to_lowercase().contains(&needle)
            || match &entry.value {
                Reference::Bookmark { targets, .. } | Reference::Change { targets, .. } => targets
                    .iter()
                    .any(|target| target.description.to_lowercase().contains(&needle)),
            }
    });
    entries.sort_by(|a, b| a.key.cmp(&b.key));
    let (values, previous, next) = window(entries, &request)?;
    let result = Page {
        version: 1,
        operation: request.operation,
        references: values,
        previous,
        next,
        stats: context.budget.stats(),
    };
    context.budget.complete_output(&result)?;
    Ok(result)
}
fn window(
    entries: Vec<Entry>,
    request: &Request,
) -> Result<(Vec<Reference>, Option<String>, Option<String>)> {
    let boundary = request.anchor.as_deref();
    let (start, end) = match request.direction {
        Direction::Forward => {
            let start = boundary.map_or(0, |anchor| {
                entries.partition_point(|e| e.key.as_str() <= anchor)
            });
            (start, (start + request.limit).min(entries.len()))
        }
        Direction::Backward => {
            let end = boundary.map_or(entries.len(), |anchor| {
                entries.partition_point(|e| e.key.as_str() < anchor)
            });
            (end.saturating_sub(request.limit), end)
        }
    };
    let previous = (start > 0 && start < end).then(|| entries[start].key.clone());
    let next = (end < entries.len() && start < end).then(|| entries[end - 1].key.clone());
    Ok((
        entries
            .into_iter()
            .skip(start)
            .take(end - start)
            .map(|entry| entry.value)
            .collect(),
        previous,
        next,
    ))
}
fn workspace_names(view: &dto::View, ids: &BTreeSet<m::CommitId>) -> Vec<String> {
    view.wc_commit_ids
        .iter()
        .filter(|(_, id)| ids.contains(id))
        .map(|(name, _)| name.clone())
        .collect()
}
impl Context {
    pub(super) async fn commit_dto(&self, id: m::CommitId) -> Result<dto::Commit> {
        self.member(m::ObjectKey::Commit(id)).await?;
        let native = if id == roots::ROOT_COMMIT {
            roots::commit()
        } else {
            self.objects
                .get::<b::Commit>(m::ObjectKey::Commit(id))
                .await
                .map_err(map_object)?
        };
        dto::Commit::try_from(&native).map_err(|_| Error::Corrupt)
    }
    async fn targets(
        &self,
        view: &dto::View,
        ids: impl Iterator<Item = m::CommitId>,
    ) -> Result<Vec<Target>> {
        let mut targets = vec![];
        for id in ids {
            let commit = self.commit_dto(id).await?;
            let workspaces = view
                .wc_commit_ids
                .iter()
                .filter(|(_, target)| **target == id)
                .map(|(name, _)| name.clone())
                .collect();
            targets.push(Target {
                commit: id,
                change: commit.change_id,
                parents: commit.parents,
                description: commit.description,
                author: commit.author,
                committer: commit.committer,
                conflicted: !commit.conflict_labels.is_empty() || commit.root_tree.len() != 1,
                workspaces,
            });
        }
        Ok(targets)
    }
}
