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
    let _memory = crate::query::memory::Lease::reserve(16384 + request.limit as u64 * 16384)?;
    let changes = prepared.reference_changes();
    let filtered = if request.query.is_empty() {
        None
    } else {
        Some(filter_window(context, view, request)?)
    };
    let mut selected = if let Some((keys, _, _)) = &filtered {
        keys.iter()
            .map(|key| {
                if let Some(name) = key.strip_prefix("bookmark:") {
                    let (name, target) = view
                        .local_bookmarks
                        .get_key_value(name)
                        .ok_or(Error::Corrupt)?;
                    Ok(Selection::Bookmark(name, target))
                } else {
                    let (key, targets) = changes.get_key_value(key).ok_or(Error::Corrupt)?;
                    Ok(Selection::Change(key, targets.change, &targets.commits))
                }
            })
            .collect::<Result<Vec<_>>>()?
    } else if let Some(exact) = &request.exact {
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
    if filtered.is_none() && request.direction == Direction::Backward {
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
    let (previous, next) =
        filtered.map_or((previous, next), |(_, previous, next)| (previous, next));
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
            if let std::collections::btree_map::Entry::Vacant(entry) = hydrated.entry(id) {
                let target = context
                    .targets(view, std::iter::once(id))
                    .await?
                    .pop()
                    .ok_or(Error::Incomplete)?;
                entry.insert(target);
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

type Window = (Vec<String>, Option<String>, Option<String>);
fn filter_window(context: &Context, view: &dto::View, request: &Request) -> Result<Window> {
    let prepared = context.access.prepared.as_ref().ok_or(Error::Unavailable)?;
    let filter = prepared.reference_filter();
    let needle = request.query.to_lowercase();
    let matches = |key: &str| -> Result<bool> {
        let bookmark = key.strip_prefix("bookmark:");
        if (bookmark.is_some() && request.kind == Kind::Changes)
            || (bookmark.is_none() && request.kind == Kind::Bookmarks)
        {
            return Ok(false);
        }
        let (bookmark_target, change_target) = if let Some(name) = bookmark {
            let Some(target) = view.local_bookmarks.get(name) else {
                return Ok(false);
            };
            (Some(target), None)
        } else {
            let Some(targets) = prepared.reference_changes().get(key) else {
                return Ok(false);
            };
            (None, Some(targets))
        };
        context.budget.work(1 + key.len().div_ceil(64))?;
        if key.to_lowercase().contains(&needle) {
            return Ok(true);
        }
        for id in bookmark_target
            .into_iter()
            .flat_map(|target| target.0.iter().flatten())
            .chain(
                change_target
                    .into_iter()
                    .flat_map(|target| target.commits.iter()),
            )
        {
            if filter.contains(
                id.as_bytes(),
                &needle,
                |amount| context.budget.work(amount),
                |bytes| context.budget.read_bytes(bytes),
            )? {
                return Ok(true);
            }
        }
        Ok(false)
    };
    let search = |anchor: Option<&str>, backward, limit| {
        filter.search(
            &needle,
            anchor,
            backward,
            limit,
            matches,
            |amount| context.budget.work(amount),
            |bytes| context.budget.read_bytes(bytes),
        )
    };
    let backward = request.direction == Direction::Backward;
    let mut keys = search(request.anchor.as_deref(), backward, request.limit + 1)?;
    let more = keys.len() > request.limit;
    keys.truncate(request.limit);
    if backward {
        keys.reverse();
    }
    let previous = keys.first().filter(|_| backward && more).cloned();
    let next = keys.last().filter(|_| !backward && more).cloned();
    let previous = if !backward {
        if let Some(first) = keys.first() {
            (!search(Some(first), true, 1)?.is_empty()).then(|| first.clone())
        } else {
            None
        }
    } else {
        previous
    };
    let next = if backward {
        if let Some(last) = keys.last() {
            (!search(Some(last), false, 1)?.is_empty()).then(|| last.clone())
        } else {
            None
        }
    } else {
        next
    };
    Ok((keys, previous, next))
}
