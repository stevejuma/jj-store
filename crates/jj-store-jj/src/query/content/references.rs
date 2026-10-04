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
    Workspaces,
    Changes,
    #[default]
    All,
}
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub operation: m::OperationId,
    pub query: String,
    pub exact: Option<String>,
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
    Workspace {
        name: String,
        targets: Vec<Target>,
    },
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
        || request.exact.as_ref().is_some_and(|name| name.len() > 1024)
        || request.query.len() > 1024
        || request.anchor.as_ref().is_some_and(|v| v.len() > 4096)
    {
        return Err(Error::Limit);
    }
    if request.exact.is_some()
        && (!request.query.is_empty() || request.anchor.is_some() || request.kind == Kind::All)
    {
        return Err(Error::Invalid);
    }
    let operation = context.operation(request.operation).await?;
    let view = context.view(operation.view_id).await?;
    if request.kind == Kind::Workspaces {
        return workspace_window(&context, &view, &request).await;
    }
    if request.kind == Kind::Bookmarks && request.query.is_empty() {
        return bookmark_window(&context, &view, &request).await;
    }
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
        if request
            .exact
            .as_ref()
            .is_some_and(|name| entry.key != format!("change:{name}"))
        {
            return false;
        }
        needle.is_empty()
            || entry.key.to_lowercase().contains(&needle)
            || match &entry.value {
                Reference::Bookmark { targets, .. }
                | Reference::Change { targets, .. }
                | Reference::Workspace { targets, .. } => targets
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
async fn bookmark_window(context: &Context, view: &dto::View, request: &Request) -> Result<Page> {
    use std::ops::Bound::{Excluded, Unbounded};
    let anchor = request
        .anchor
        .as_deref()
        .map(|value| value.strip_prefix("bookmark:").ok_or(Error::Invalid))
        .transpose()?;
    let mut selected: Vec<_> = if let Some(name) = &request.exact {
        view.local_bookmarks
            .get_key_value(name)
            .into_iter()
            .collect()
    } else {
        match request.direction {
            Direction::Forward => view
                .local_bookmarks
                .range::<str, _>((anchor.map_or(Unbounded, Excluded), Unbounded))
                .take(request.limit)
                .collect(),
            Direction::Backward => view
                .local_bookmarks
                .range::<str, _>((Unbounded, anchor.map_or(Unbounded, Excluded)))
                .rev()
                .take(request.limit)
                .collect(),
        }
    };
    if matches!(request.direction, Direction::Backward) {
        selected.reverse();
    }
    let previous = selected
        .first()
        .filter(|_| request.exact.is_none())
        .and_then(|(name, _)| {
            view.local_bookmarks
                .first_key_value()
                .filter(|(first, _)| first < name)
                .map(|_| format!("bookmark:{name}"))
        });
    let next = selected
        .last()
        .filter(|_| request.exact.is_none())
        .and_then(|(name, _)| {
            view.local_bookmarks
                .last_key_value()
                .filter(|(last, _)| last > name)
                .map(|_| format!("bookmark:{name}"))
        });
    let mut references = Vec::with_capacity(selected.len());
    let mut hydrated = BTreeMap::new();
    for (name, target) in selected {
        context.budget.work(1)?;
        let ids: BTreeSet<_> = target.0.iter().flatten().copied().collect();
        let mut targets = Vec::with_capacity(ids.len());
        for id in &ids {
            if !hydrated.contains_key(id) {
                let value = context
                    .targets(view, std::iter::once(*id))
                    .await?
                    .pop()
                    .ok_or(Error::Incomplete)?;
                hydrated.insert(*id, value);
            }
            targets.push(hydrated[id].clone());
        }
        let value = Reference::Bookmark {
            name: name.clone(),
            terms: target.0.clone(),
            conflicted: target.0.len() != 1,
            workspaces: workspace_names(view, &ids),
            targets,
        };
        context.budget.output(&value)?;
        references.push(value);
    }
    let page = Page {
        version: 1,
        operation: request.operation,
        references,
        previous,
        next,
        stats: context.budget.stats(),
    };
    context.budget.complete_output(&page)?;
    Ok(page)
}

async fn workspace_window(context: &Context, view: &dto::View, request: &Request) -> Result<Page> {
    use std::ops::Bound::{Excluded, Unbounded};
    let anchor = request
        .anchor
        .as_deref()
        .map(|value| value.strip_prefix("workspace:").ok_or(Error::Invalid))
        .transpose()?;
    let needle = request.query.to_lowercase();
    let matches = |name: &str| -> Result<bool> {
        context.budget.work(1)?;
        Ok(needle.is_empty() || name.to_lowercase().contains(&needle))
    };
    let select = |iter: &mut dyn Iterator<Item = (&String, &m::CommitId)>| -> Result<Vec<_>> {
        let mut selected = Vec::new();
        for (name, id) in iter {
            if matches(name)? {
                selected.push((name.clone(), *id));
                if selected.len() > request.limit {
                    break;
                }
            }
        }
        Ok(selected)
    };
    let mut selected = if let Some(name) = &request.exact {
        view.wc_commit_ids
            .get_key_value(name)
            .map(|(name, id)| (name.clone(), *id))
            .into_iter()
            .collect()
    } else {
        match request.direction {
            Direction::Forward => select(
                &mut view
                    .wc_commit_ids
                    .range::<str, _>((anchor.map_or(Unbounded, Excluded), Unbounded)),
            )?,
            Direction::Backward => select(
                &mut view
                    .wc_commit_ids
                    .range::<str, _>((Unbounded, anchor.map_or(Unbounded, Excluded)))
                    .rev(),
            )?,
        }
    };
    let more = selected.len() > request.limit;
    selected.truncate(request.limit);
    if matches!(request.direction, Direction::Backward) {
        selected.reverse();
    }
    let mut previous = None;
    let mut next = None;
    if request.exact.is_none() && !selected.is_empty() {
        let first = &selected[0].0;
        let last = &selected[selected.len() - 1].0;
        let other = match request.direction {
            Direction::Forward => {
                let mut found = false;
                for (name, _) in view.wc_commit_ids.range(..first.clone()).rev() {
                    if matches(name)? {
                        found = true;
                        break;
                    }
                }
                found
            }
            Direction::Backward => {
                let mut found = false;
                for (name, _) in view
                    .wc_commit_ids
                    .range((Excluded(last.clone()), Unbounded))
                {
                    if matches(name)? {
                        found = true;
                        break;
                    }
                }
                found
            }
        };
        let (has_previous, has_next) = match request.direction {
            Direction::Forward => (other, more),
            Direction::Backward => (more, other),
        };
        previous = has_previous.then(|| format!("workspace:{first}"));
        next = has_next.then(|| format!("workspace:{last}"));
    }
    let mut references = Vec::with_capacity(selected.len());
    let mut hydrated = BTreeMap::new();
    for (name, id) in &selected {
        context.budget.work(1)?;
        if !hydrated.contains_key(id) {
            let commit = context.commit_dto(*id).await?;
            let target = Target {
                commit: *id,
                change: commit.change_id,
                parents: commit.parents,
                description: commit.description,
                author: commit.author,
                committer: commit.committer,
                conflicted: !commit.conflict_labels.is_empty() || commit.root_tree.len() != 1,
                workspaces: vec![],
            };
            hydrated.insert(*id, target);
        }
        let mut target = hydrated[id].clone();
        target.workspaces.push(name.clone());
        let value = Reference::Workspace {
            name: name.clone(),
            targets: vec![target],
        };
        context.budget.output(&value)?;
        references.push(value);
    }
    let page = Page {
        version: 1,
        operation: request.operation,
        references,
        previous,
        next,
        stats: context.budget.stats(),
    };
    context.budget.complete_output(&page)?;
    Ok(page)
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
