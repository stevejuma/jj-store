//! Postings select candidates; exact pinned visibility and text decide matches.
use super::*;
use jj_lib::{object_id::ObjectId as _, ref_name::RefName};

type Window = (Vec<String>, Option<String>, Option<String>);
pub(super) fn window(
    context: &Context,
    view: &o::View,
    request: &Request,
    prepared: &crate::query::projection::Reader,
) -> Result<Window> {
    let filter = prepared.reference_filter()?;
    let needle = request.query.to_lowercase();
    let matches = |key: &str| -> Result<bool> {
        let bookmark = key.strip_prefix("bookmark:");
        if (bookmark.is_some() && request.kind == Kind::Changes)
            || (bookmark.is_none() && request.kind == Kind::Bookmarks)
        {
            return Ok(false);
        }
        let (bookmark_target, change_target) = if let Some(name) = bookmark {
            let Some(target) = view.local_bookmarks.get(RefName::new(name)) else {
                return Ok(false);
            };
            (Some(target), None)
        } else {
            let Some(group) = prepared.reference_changes().get(key) else {
                return Ok(false);
            };
            (None, Some(group))
        };
        context.budget.work(
            2 + key.len().div_ceil(64) + prepared.reference_changes().len().max(1).ilog2() as usize,
        )?;
        if key.to_lowercase().contains(&needle) {
            return Ok(true);
        }
        // Include removed terms for bookmarks, but only current visible targets
        // for change groups. Indexed predecessor text cannot expose hidden rows.
        for id in bookmark_target
            .into_iter()
            .flat_map(|target| target.iter().flatten().map(|id| id.as_bytes()))
            .chain(change_target.into_iter().flat_map(|group| {
                group
                    .targets
                    .iter()
                    .map(|row| row.commit.as_bytes().as_slice())
            }))
        {
            if filter.contains(
                id,
                &needle,
                |n| context.budget.work(n),
                |n| context.budget.read_bytes(n),
            )? {
                return Ok(true);
            }
        }
        Ok(false)
    };
    let search = |anchor: Option<&str>, backward, limit| {
        filter.search(
            crate::query::prepared::filters::Search {
                needle: &needle,
                anchor,
                backward,
                limit,
            },
            matches,
            |n| context.budget.work(n),
            |n| context.budget.read_bytes(n),
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
