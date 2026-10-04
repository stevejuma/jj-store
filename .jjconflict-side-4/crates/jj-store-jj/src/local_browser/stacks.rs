//! Typed bounded stack range/history. No checkout, snapshot or publication.
use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "LocalStackHistoryRequest")]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub head: m::GitSha1Id,
    /// None means bounded target history. Some excludes all base ancestors.
    pub base: Option<m::GitSha1Id>,
    pub limit: usize,
    pub after: Option<m::GitSha1Id>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalStackHistoryPage")]
pub struct Page {
    pub version: u32,
    pub operation: m::OperationId,
    pub head: m::GitSha1Id,
    pub base: Option<m::GitSha1Id>,
    pub commits: Vec<CommitNode>,
    pub next: Option<m::GitSha1Id>,
    pub stats: Stats,
}
impl Request {
    pub(super) fn graph(&self) -> (GraphRequest, GraphWindow) {
        let revset = match self.base {
            Some(base) => format!("{base}..{}", self.head),
            None => format!("ancestors({})", self.head),
        };
        (
            GraphRequest::Log {
                revset,
                workspace: None,
                user_email: String::new(),
                at_time_millis: 0,
            },
            GraphWindow {
                limit: self.limit,
                after: self.after.map(ObjectKey::Commit),
            },
        )
    }
    pub(super) fn validate(&self, budget: &Budget) -> Result<()> {
        let (graph, window) = self.graph();
        graphs::validate(&graph, &window, 0, budget)
    }
}
pub(super) async fn read(
    context: Context,
    operation: m::OperationId,
    request: Request,
) -> Result<Page> {
    request.validate(&context.budget)?;
    let (graph, window) = request.graph();
    let (nodes, next) = graphs::run(
        context.repo.as_deref().ok_or(Error::Unavailable)?,
        graph,
        window,
        &context.access,
        &context.budget,
    )
    .await?;
    let Nodes::Log(commits) = nodes else {
        return Err(Error::Unavailable);
    };
    let next = match next {
        Some(ObjectKey::Commit(commit)) => Some(commit),
        None => None,
        _ => return Err(Error::Unavailable),
    };
    Ok(Page {
        version: 1,
        operation,
        head: request.head,
        base: request.base,
        commits,
        next,
        stats: context.budget.stats(),
    })
}
