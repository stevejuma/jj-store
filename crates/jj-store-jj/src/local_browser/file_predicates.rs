//! File predicates may use complete upstream changed-path metadata only.
use super::*;
use futures::TryStreamExt as _;
use jj_lib::{repo::Repo as _, revset::ResolvedExpression as Expr};

pub(super) async fn require_coverage(
    repo: &ReadonlyRepo,
    pin: &Arc<Pin>,
    budget: &Budget,
) -> Result<()> {
    let mut heads = Vec::new();
    let mut memory = Vec::new();
    for id in repo
        .index()
        .all_heads_for_gc()
        .map_err(|_| Error::Unavailable)?
    {
        budget.work(1)?;
        if heads.len().is_multiple_of(256) {
            memory.push(crate::query::memory::Lease::reserve(256 * 128)?);
        }
        heads.push(id);
    }
    let expression = Expr::Ancestors {
        heads: Box::new(Expr::Commits(heads)),
        generation: 0..u64::MAX,
        parents_range: 0..u32::MAX,
    };
    let set = repo
        .index()
        .evaluate_revset(&expression, repo.store())
        .map_err(|_| Error::Unavailable)?;
    let mut stream = set.stream();
    while let Some(id) = stream.try_next().await.map_err(|_| Error::Unavailable)? {
        budget.authorize(pin).await?;
        let paths = repo
            .index()
            .changed_paths_in_commit(&id)
            .await
            .map_err(|_| Error::Unavailable)?
            .ok_or_else(|| {
                crate::query::language::invalid(
                    "path_index_required",
                    "Prepare the pinned changed-path index before reading file history.",
                )
            })?;
        for path in paths {
            // Admit the path scan and its later native predicate evaluation.
            budget.work(2)?;
            budget.read_bytes(path.as_internal_file_string().len())?;
            native::path(path.as_internal_file_string())?;
        }
    }
    Ok(())
}
