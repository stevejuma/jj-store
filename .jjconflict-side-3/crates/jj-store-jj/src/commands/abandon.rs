//! Native abandonment, following jj-cli 0.45.1 commands/abandon.rs.
//! Upstream algorithm: Copyright 2020 The Jujutsu Authors, Apache-2.0.
use super::*;
use crate::roots;
use jj_lib::{repo::ReadonlyRepo, revset::RevsetExpression, rewrite::RewriteRefsOptions};
use std::collections::{BTreeSet, HashMap};

pub(super) struct ResultReport {
    pub repo: Arc<ReadonlyRepo>,
    pub abandoned: Vec<m::CommitId>,
    pub skipped_hidden: Vec<m::CommitId>,
}
fn unavailable(_: impl std::fmt::Debug) -> Error {
    query::Error::Unavailable.into()
}
pub(super) async fn stage(
    repo: &Arc<ReadonlyRepo>,
    access: &dyn ReadAccess,
    commits: Vec<m::CommitId>,
    retain_bookmarks: bool,
    restore_descendants: bool,
    principal: m::PrincipalId,
    budget: &Budget,
) -> Result<ResultReport> {
    let mut targets = BTreeSet::new();
    let mut abandoned = vec![];
    let mut skipped_hidden = vec![];
    for id in commits {
        budget.work(1)?;
        if id == roots::ROOT_COMMIT {
            return Err(Error::Root);
        }
        match super::execute::commit(repo, access, id, true, budget).await {
            Ok(commit) => {
                targets.insert(commit.id().clone());
                abandoned.push(id);
            }
            Err(Error::Hidden) => skipped_hidden.push(id),
            Err(error) => return Err(error),
        }
    }
    if targets.is_empty() {
        return Err(Error::NoChanges);
    }
    let mut tx = repo.start_transaction();
    tx.set_attribute("jj-store.principal".into(), principal.to_string());
    tx.repo_mut()
        .transform_descendants_with_options(
            targets.iter().cloned().collect(),
            &RevsetExpression::none(),
            &HashMap::new(),
            &RewriteRefsOptions {
                delete_abandoned_bookmarks: !retain_bookmarks,
            },
            async |rewriter| {
                if targets.contains(rewriter.old_commit().id()) {
                    rewriter.abandon();
                } else if restore_descendants {
                    rewriter.reparent().write().await?;
                } else {
                    rewriter.rebase().await?.write().await?;
                }
                Ok(())
            },
        )
        .await
        .map_err(unavailable)?;
    tx.repo_mut()
        .rebase_descendants()
        .await
        .map_err(unavailable)?;
    budget.check()?;
    let repo = tx
        .write("jj-store: abandon changes")
        .await
        .map_err(unavailable)?
        .leave_unpublished();
    Ok(ResultReport {
        repo,
        abandoned,
        skipped_hidden,
    })
}
