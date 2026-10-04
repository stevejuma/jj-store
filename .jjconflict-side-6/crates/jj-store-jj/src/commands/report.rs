use super::*;
use jj_lib::{backend as b, repo::Repo as _};
use std::collections::BTreeSet;

pub(super) async fn resolution(
    repo: &jj_lib::repo::ReadonlyRepo,
    access: &dyn ReadAccess,
    before: m::CommitId,
    mut selected_paths: Vec<String>,
    rewrites: &[Rewrite],
    created: &[m::CommitId],
    budget: &Budget,
) -> Result<ConflictResolutionReport> {
    let mut targets = rewrites.iter().filter(|r| r.predecessors.contains(&before));
    let after = targets.next().ok_or(query::Error::Corrupt)?.commit;
    if targets.next().is_some() {
        return Err(query::Error::Corrupt.into());
    }
    selected_paths.sort();
    let target = Change { before, after };
    budget.output(&target)?;
    budget.output(&selected_paths)?;
    let mut remaining = BTreeMap::new();
    let ids: BTreeSet<_> = rewrites
        .iter()
        .map(|r| r.commit)
        .chain(created.iter().copied())
        .collect();
    for id in ids {
        budget.authorize(access).await?;
        budget.output(&id)?;
        let commit = repo
            .store()
            .get_commit_async(&b::CommitId::from_bytes(id.as_bytes()))
            .await
            .map_err(|_| query::Error::Unavailable)?;
        let mut paths = vec![];
        for (path, value) in commit.tree().conflicts() {
            budget.authorize(access).await?;
            let value = value.map_err(|_| query::Error::Unavailable)?;
            budget.work(value.iter().len())?;
            let path = path.as_internal_file_string();
            // Charge before retaining each path; errors never yield a partial
            // success report or publish the staged operation.
            budget.output(&path)?;
            paths.push(path.to_owned());
        }
        paths.sort();
        remaining.insert(id, paths);
    }
    Ok(ConflictResolutionReport {
        target,
        selected_paths,
        remaining,
    })
}
