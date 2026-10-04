//! Ordered identities are selected before any target body is hydrated.
use super::*;
use jj_lib::{
    object_id::ObjectId as _,
    ref_name::{RefName, RefNameBuf},
};
use std::ops::Bound::{Excluded, Unbounded};

enum Selection<'a> {
    Bookmark(&'a RefNameBuf, &'a o::RefTarget),
    Change(&'a String, m::ChangeId, &'a [m::CommitId]),
}
impl Selection<'_> {
    fn key(&self) -> String {
        match self {
            Self::Bookmark(name, _) => format!("bookmark:{}", name.as_str()),
            Self::Change(key, ..) => (*key).clone(),
        }
    }
}
fn bookmarks<'a>(
    view: &'a o::View,
    request: &Request,
) -> Box<dyn DoubleEndedIterator<Item = Selection<'a>> + 'a> {
    if request.kind != Kind::All {
        return Box::new(std::iter::empty());
    }
    let anchor = request.anchor.as_deref();
    let (low, high) = if let Some(name) = anchor.and_then(|a| a.strip_prefix("bookmark:")) {
        match request.direction {
            Direction::Forward => (Excluded(RefName::new(name)), Unbounded),
            Direction::Backward => (Unbounded, Excluded(RefName::new(name))),
        }
    } else {
        if anchor.is_some_and(|a| match request.direction {
            Direction::Forward => a > "bookmark:",
            Direction::Backward => a < "bookmark:",
        }) {
            return Box::new(std::iter::empty());
        }
        (Unbounded, Unbounded)
    };
    Box::new(
        view.local_bookmarks
            .range::<RefName, _>((low, high))
            .map(|(name, target)| Selection::Bookmark(name, target)),
    )
}

pub(in crate::query) async fn read(
    context: Context,
    request: Request,
    prepared: &crate::query::projection::Reader,
    view: &o::View,
) -> Result<Page> {
    validate(&request, &context.limits)?;
    if request.operation != context.access.operation() || !request.query.is_empty() {
        return Err(Error::Invalid);
    }
    let mut memory = vec![crate::query::memory::Lease::reserve(
        4096 + request.limit as u64 * 512,
    )?];
    // Retained maps are immutable; charge logarithmic seeks before selecting IDs.
    context.budget.work(
        2 + view.local_bookmarks.len().max(1).ilog2() as usize
            + prepared.reference_changes().len().max(1).ilog2() as usize,
    )?;
    let changes = prepared.reference_changes();
    let mut selected = if let Some(exact) = &request.exact {
        changes
            .get_key_value(&format!("change:{exact}"))
            .map(|(key, targets)| Selection::Change(key, targets.change, &targets.commits))
            .into_iter()
            .collect::<Vec<_>>()
    } else {
        let bounds = match request.direction {
            Direction::Forward => (
                request.anchor.as_deref().map_or(Unbounded, Excluded),
                Unbounded,
            ),
            Direction::Backward => (
                Unbounded,
                request.anchor.as_deref().map_or(Unbounded, Excluded),
            ),
        };
        let rows = bookmarks(view, &request).chain(
            changes
                .range::<str, _>(bounds)
                .map(|(key, targets)| Selection::Change(key, targets.change, &targets.commits)),
        );
        match request.direction {
            Direction::Forward => rows.take(request.limit).collect(),
            Direction::Backward => rows.rev().take(request.limit).collect(),
        }
    };
    if request.direction == Direction::Backward {
        selected.reverse();
    }
    let first = if request.kind == Kind::All {
        view.local_bookmarks
            .first_key_value()
            .map(|(name, _)| format!("bookmark:{}", name.as_str()))
    } else {
        None
    }
    .or_else(|| changes.first_key_value().map(|(key, _)| key.clone()));
    let last = changes
        .last_key_value()
        .map(|(key, _)| key.clone())
        .or_else(|| {
            (request.kind == Kind::All)
                .then(|| {
                    view.local_bookmarks
                        .last_key_value()
                        .map(|(name, _)| format!("bookmark:{}", name.as_str()))
                })
                .flatten()
        });
    let previous = selected
        .first()
        .filter(|_| request.exact.is_none())
        .map(Selection::key)
        .filter(|key| first.as_ref().is_some_and(|first| first < key));
    let next = selected
        .last()
        .filter(|_| request.exact.is_none())
        .map(Selection::key)
        .filter(|key| last.as_ref().is_some_and(|last| last > key));
    let mut references = Vec::with_capacity(selected.len());
    let mut hydrated = BTreeMap::new();
    for row in selected {
        context.budget.work(1)?;
        if let Selection::Change(_, _, ids) = &row {
            // A divergent group may be large even for a one-row page. Admit
            // distinct targets before copying their identities or bodies.
            if ids.len() > context.limits.max_history_visits {
                return Err(Error::Limit);
            }
            context.budget.work(ids.len())?;
        }
        let count = match &row {
            Selection::Bookmark(_, target) => target.as_merge().iter().count(),
            Selection::Change(_, _, ids) => ids.len(),
        };
        memory.push(crate::query::memory::Lease::reserve(
            512 + count as u64 * 256,
        )?);
        let ids: Vec<_> = match &row {
            Selection::Bookmark(_, target) => target
                .as_merge()
                .iter()
                .flatten()
                .map(|id| m::CommitId::from_slice(id.as_bytes()).map_err(|_| Error::Corrupt))
                .collect::<Result<BTreeSet<_>>>()?
                .into_iter()
                .collect(),
            Selection::Change(_, _, ids) => ids.to_vec(),
        };
        let mut targets = Vec::with_capacity(ids.len());
        for id in ids {
            if !hydrated.contains_key(&id) {
                let commit = context.commit_dto(id).await?;
                let target = Target {
                    commit: id,
                    change: commit.change_id,
                    parents: commit.parents,
                    description: commit.description,
                    author: commit.author,
                    committer: commit.committer,
                    conflicted: !commit.conflict_labels.is_empty() || commit.root_tree.len() != 1,
                    workspaces: prepared.workspace_names(id)?,
                };
                memory.push(target_memory(&target)?);
                hydrated.insert(id, target);
            }
            memory.push(target_memory(&hydrated[&id])?);
            targets.push(hydrated[&id].clone());
        }
        let value = match row {
            Selection::Bookmark(name, target) => Reference::Bookmark {
                name: name.as_str().to_owned(),
                terms: dto::RefTarget::try_from(target)
                    .map_err(|_| Error::Corrupt)?
                    .0,
                conflicted: target.has_conflict(),
                workspaces: targets
                    .iter()
                    .flat_map(|target| target.workspaces.iter().cloned())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect(),
                targets,
            },
            Selection::Change(_, change, ids) => Reference::Change {
                change,
                divergent: ids.len() > 1,
                targets,
            },
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

fn target_memory(target: &Target) -> Result<crate::query::memory::Lease> {
    // Admit cached bodies and presentation copies, including parent IDs, strings,
    // reverse names, map/vector nodes and the lease list itself, before copying.
    let mut bytes = 1024 + target.parents.len() as u64 * 256;
    for text in [
        target.description.as_str(),
        target.author.name.as_str(),
        target.author.email.as_str(),
        target.committer.name.as_str(),
        target.committer.email.as_str(),
    ]
    .into_iter()
    .chain(target.workspaces.iter().map(String::as_str))
    {
        bytes = bytes
            .checked_add(128 + text.len() as u64 * 4)
            .ok_or(Error::Limit)?;
    }
    crate::query::memory::Lease::reserve(bytes)
}
