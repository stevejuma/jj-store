//! Bounded selected-view head facts, without graph/index or ancestry work.
use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "NativeCodeSummaryLatest")]
pub struct Latest {
    pub id: m::CommitId,
    pub description: String,
    pub author: dto::Signature,
    pub committer: dto::Signature,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "NativeCodeSummary")]
pub struct Summary {
    pub version: u32,
    pub commits: usize,
    pub commits_partial: bool,
    pub latest: Option<Latest>,
    pub stats: Stats,
}
pub(super) async fn read(context: Context, limit: usize) -> Result<Summary> {
    if !(1..=256).contains(&limit) || limit > context.limits.max_page {
        return Err(Error::Limit);
    }
    let operation = context.operation(context.access.operation()).await?;
    let view = context.view(operation.view_id).await?;
    let mut heads = view
        .head_ids
        .iter()
        .copied()
        .filter(|id| *id != roots::ROOT_COMMIT);
    let mut summary = Summary {
        version: 1,
        commits: 0,
        commits_partial: false,
        latest: None,
        stats: context.budget.stats(),
    };
    for id in heads.by_ref().take(limit) {
        context.budget.work(1)?;
        let commit = context.commit_dto(id).await?;
        context.budget.work(commit.parents.len())?;
        summary.commits += 1;
        summary.commits_partial |= commit.parents.iter().any(|id| *id != roots::ROOT_COMMIT);
        if summary.latest.as_ref().is_none_or(|latest| {
            commit.committer.timestamp.millis > latest.committer.timestamp.millis
        }) {
            summary.latest = Some(Latest {
                id,
                description: commit.description,
                author: commit.author,
                committer: commit.committer,
            });
        }
    }
    summary.commits_partial |= heads.next().is_some();
    summary.stats = context.budget.stats();
    context.budget.complete_output(&summary)?;
    Ok(summary)
}
