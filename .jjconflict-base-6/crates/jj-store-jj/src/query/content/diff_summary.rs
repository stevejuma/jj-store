//! Cursor-ready changed-path summaries; selected content remains a separate read.
use super::native::{map_backend, path, terms};
use super::*;
use futures::StreamExt as _;
use jj_lib::matchers::PrefixMatcher;
use serde::Deserialize;
use std::collections::VecDeque;

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub from: m::CommitId,
    pub to: m::CommitId,
    pub path: String,
    pub limit: usize,
    pub anchor: Option<String>,
    pub direction: Direction,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
pub struct Page {
    pub version: u32,
    pub from: m::CommitId,
    pub to: m::CommitId,
    pub path: String,
    pub changes: Vec<diff::Change>,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub stats: Stats,
}
pub(super) async fn read(context: Context, request: Request) -> Result<Page> {
    if request.limit == 0
        || request.limit > context.limits.max_page
        || request.path.len() > 4096
        || request.anchor.as_ref().is_some_and(|v| v.len() > 4096)
    {
        return Err(Error::Limit);
    }
    let root = path(&request.path)?;
    let store = context.native_store()?;
    let from = context.commit(&store, request.from).await?.tree();
    let to = context.commit(&store, request.to).await?.tree();
    let matcher = PrefixMatcher::new([root.as_ref()]);
    let mut stream = from.diff_stream(&to, &matcher);
    let mut selected = VecDeque::new();
    let mut has_previous = false;
    let mut has_next = false;
    while let Some(entry) = stream.next().await {
        context.budget.authorize(context.access.as_ref()).await?;
        context.budget.work(1)?;
        let path = entry.path.as_internal_file_string().to_owned();
        let include = match (request.direction, request.anchor.as_deref()) {
            (_, None) => true,
            (Direction::Forward, Some(anchor)) => path.as_str() > anchor,
            (Direction::Backward, Some(anchor)) => path.as_str() < anchor,
        };
        if !include {
            if request.direction == Direction::Forward {
                has_previous = true;
            } else {
                has_next = true;
            }
            continue;
        }
        selected.push_back((entry.path, entry.values.map_err(map_backend)?));
        if request.direction == Direction::Forward && selected.len() > request.limit {
            selected.pop_back();
            has_next = true;
            break;
        }
        if request.direction == Direction::Backward && selected.len() > request.limit {
            selected.pop_front();
            has_previous = true;
        }
    }
    let previous = (has_previous && !selected.is_empty()).then(|| {
        selected
            .front()
            .unwrap()
            .0
            .as_internal_file_string()
            .to_owned()
    });
    let next = (has_next && !selected.is_empty()).then(|| {
        selected
            .back()
            .unwrap()
            .0
            .as_internal_file_string()
            .to_owned()
    });
    let mut copies = super::diff_copies::Copies::default();
    let mut changes = vec![];
    for (entry_path, values) in selected {
        let name = entry_path.as_internal_file_string().to_owned();
        let mut sources = vec![];
        if let Some(Some(b::TreeValue::File { copy_id, .. })) = values.after.as_resolved() {
            let same = matches!(values.before.as_resolved(), Some(Some(b::TreeValue::File { copy_id: old, .. })) if old == copy_id);
            if !same {
                sources = copies
                    .sources(&context, &from, &to, copy_id, &entry_path)
                    .await?;
            }
        }
        if sources.is_empty() && !values.before.is_absent() {
            sources.push(diff::Source {
                kind: diff::SourceKind::Normal,
                path: name.clone(),
                terms: terms(&values.before)?,
            });
        }
        let change = diff::Change {
            path: name,
            before: terms(&values.before)?,
            after: terms(&values.after)?,
            sources,
            content: diff::Content::Unavailable {
                reason: diff::Unavailable::NotRequested,
            },
        };
        context.budget.output(&change)?;
        changes.push(change);
    }
    let result = Page {
        version: 1,
        from: request.from,
        to: request.to,
        path: request.path,
        changes,
        previous,
        next,
        stats: context.budget.stats(),
    };
    context.budget.complete_output(&result)?;
    Ok(result)
}
