//! Ordered identities are selected before any target body is hydrated.
use super::*;
use std::ops::Bound::{Excluded, Unbounded};

enum Selection<'a> {
    Bookmark(&'a String, &'a dto::RefTarget),
    Change(&'a String, m::ChangeId, &'a [m::GitSha1Id]),
}
impl Selection<'_> {
    fn key(&self) -> String {
        match self {
            Self::Bookmark(name, _) => format!("bookmark:{name}"),
            Self::Change(key, ..) => (*key).clone(),
        }
    }
}
fn bookmarks<'a>(
    view: &'a dto::View,
    request: &Request,
) -> Box<dyn DoubleEndedIterator<Item = Selection<'a>> + 'a> {
    if request.kind != Kind::All {
        return Box::new(std::iter::empty());
    }
    let anchor = request.anchor.as_deref();
    let (low, high) = if let Some(name) = anchor.and_then(|a| a.strip_prefix("bookmark:")) {
        match request.direction {
            Direction::Forward => (Excluded(name), Unbounded),
            Direction::Backward => (Unbounded, Excluded(name)),
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
            .range::<str, _>((low, high))
            .map(|(name, target)| Selection::Bookmark(name, target)),
    )
}

pub(super) async fn read(context: &Context, view: &dto::View, request: &Request) -> Result<Page> {
    let prepared = context.access.prepared.as_ref().ok_or(Error::Unavailable)?;
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
        let rows = bookmarks(view, request).chain(
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
            .map(|(name, _)| format!("bookmark:{name}"))
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
                        .map(|(name, _)| format!("bookmark:{name}"))
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
        let ids: Vec<_> = match &row {
            Selection::Bookmark(_, target) => target
                .0
                .iter()
                .flatten()
                .copied()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
            Selection::Change(_, _, ids) => ids.to_vec(),
        };
        let mut targets = Vec::with_capacity(ids.len());
        for id in ids {
            if !hydrated.contains_key(&id) {
                let target = context
                    .targets(view, std::iter::once(id))
                    .await?
                    .pop()
                    .ok_or(Error::Incomplete)?;
                hydrated.insert(id, target);
            }
            targets.push(hydrated[&id].clone());
        }
        let value = match row {
            Selection::Bookmark(name, target) => Reference::Bookmark {
                name: name.clone(),
                terms: target.0.clone(),
                conflicted: target.0.len() != 1,
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
