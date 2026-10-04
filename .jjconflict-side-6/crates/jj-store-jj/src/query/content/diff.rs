//! Copy-aware native file changes and bounded exact line hunks.
use super::native::{Text, map_backend};
use super::*;
use futures::StreamExt as _;
use jj_lib::{diff::DiffHunkKind, matchers::PrefixMatcher};
use serde::Deserialize;

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "FileDiffRequest")]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub from: m::CommitId,
    pub to: m::CommitId,
    pub path: String,
    pub content: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "FileDiffSourceKind")]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    Normal,
    Copy,
    Rename,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "FileDiffSource")]
pub struct Source {
    pub kind: SourceKind,
    pub path: String,
    pub terms: Vec<Option<dto::TreeValue>>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "FileDiffUnavailable")]
#[serde(rename_all = "snake_case")]
pub enum Unavailable {
    NotRequested,
    Conflict,
    Structural,
    MultipleSources,
    Binary,
    TooLarge,
    Complexity,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "FileDiffHunkKind")]
#[serde(rename_all = "snake_case")]
pub enum HunkKind {
    Matching,
    Different,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "FileDiffHunk")]
pub struct Hunk {
    pub kind: HunkKind,
    pub before_start: usize,
    pub after_start: usize,
    pub before: String,
    pub after: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "FileDiffContent")]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Content {
    Text { hunks: Vec<Hunk> },
    Unavailable { reason: Unavailable },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "FileDiffChange")]
pub struct Change {
    pub path: String,
    /// Native values at the target; source annotations never replace these.
    pub before: Vec<Option<dto::TreeValue>>,
    pub after: Vec<Option<dto::TreeValue>>,
    pub sources: Vec<Source>,
    pub content: Content,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "FileDiffResponse")]
pub struct Response {
    pub version: u32,
    pub from: m::CommitId,
    pub to: m::CommitId,
    pub path: String,
    pub changes: Vec<Change>,
    pub stats: Stats,
}
pub(super) async fn read(context: Context, request: Request) -> Result<Response> {
    let path = native::path(&request.path)?;
    let store = context.native_store()?;
    let from = context.commit(&store, request.from).await?.tree();
    let to = context.commit(&store, request.to).await?.tree();
    let matcher = PrefixMatcher::new([path.as_ref()]);
    let mut stream = from.diff_stream(&to, &matcher);
    let mut copies = super::diff_copies::Copies::default();
    let mut changes = vec![];
    while let Some(entry) = stream.next().await {
        context.budget.authorize(context.access.as_ref()).await?;
        if changes.len() >= context.limits.max_changes {
            return Err(Error::Limit);
        }
        let values = entry.values.map_err(map_backend)?;
        let path = entry.path.as_internal_file_string().to_owned();
        let mut sources = vec![];
        if let Some(Some(b::TreeValue::File { copy_id, .. })) = values.after.as_resolved() {
            let same = matches!(values.before.as_resolved(), Some(Some(b::TreeValue::File { copy_id: old, .. })) if old == copy_id);
            if !same {
                sources = copies
                    .sources(&context, &from, &to, copy_id, &entry.path)
                    .await?;
            }
        }
        if sources.is_empty() && !values.before.is_absent() {
            sources.push(Source {
                kind: SourceKind::Normal,
                path: path.clone(),
                terms: native::terms(&values.before)?,
            });
        }
        let before = native::terms(&values.before)?;
        let after = native::terms(&values.after)?;
        context.budget.output(&before)?;
        context.budget.output(&after)?;
        context.budget.output(&sources)?;
        let content = if request.content {
            context.diff_content(&after, &sources).await?
        } else {
            unavailable(Unavailable::NotRequested)
        };
        changes.push(Change {
            path,
            before,
            after,
            sources,
            content,
        });
    }
    let result = Response {
        version: 1,
        from: request.from,
        to: request.to,
        path: request.path,
        changes,
        stats: context.budget.stats(),
    };
    context.budget.complete_output(&result)?;
    Ok(result)
}
fn unavailable(reason: Unavailable) -> Content {
    Content::Unavailable { reason }
}
impl Context {
    async fn diff_content(
        &self,
        after: &[Option<dto::TreeValue>],
        sources: &[Source],
    ) -> Result<Content> {
        let [target] = after else {
            return Ok(unavailable(Unavailable::Conflict));
        };
        let before = match sources {
            [] => &None,
            [source] => match source.terms.as_slice() {
                [value] => value,
                _ => return Ok(unavailable(Unavailable::Conflict)),
            },
            _ => return Ok(unavailable(Unavailable::MultipleSources)),
        };
        let mut inputs = vec![];
        for value in [before, target] {
            let text = match value {
                None => String::new(),
                Some(dto::TreeValue::File { id, .. }) => match self.text(*id).await? {
                    Text::Utf8(text) => text,
                    Text::Binary => return Ok(unavailable(Unavailable::Binary)),
                    Text::TooLarge => return Ok(unavailable(Unavailable::TooLarge)),
                },
                _ => return Ok(unavailable(Unavailable::Structural)),
            };
            inputs.push(text);
        }
        if inputs.iter().map(String::len).sum::<usize>() > self.limits.max_text_bytes {
            return Ok(unavailable(Unavailable::TooLarge));
        }
        let [before, after] = inputs.as_slice() else {
            unreachable!()
        };
        let Some(spans) =
            crate::line_diff::bounded(before, after, self.limits.max_diff_cells, |work| {
                self.budget.work(work)
            })?
        else {
            return Ok(unavailable(Unavailable::Complexity));
        };
        self.budget.authorize(self.access.as_ref()).await?;
        let mut hunks = vec![];
        for span in spans {
            self.budget.work(1)?;
            let hunk = Hunk {
                kind: match span.kind {
                    DiffHunkKind::Matching => HunkKind::Matching,
                    DiffHunkKind::Different => HunkKind::Different,
                },
                before_start: span.before.start,
                after_start: span.after.start,
                before: before[span.before].to_owned(),
                after: after[span.after].to_owned(),
            };
            self.budget.output(&hunk)?;
            hunks.push(hunk);
        }
        Ok(Content::Text { hunks })
    }
}
