//! Conservative migration of live merge heads left by the original composer.
use super::*;
use std::collections::BTreeSet;

pub(super) async fn candidates(repo: &ReadonlyRepo, saved: &SavedWorkspace) -> Result<Vec<Commit>> {
    // Reference edges in actual Stacks operations prove ownership. Neither a
    // matching message nor mere reachability from the workspace suffices.
    let mut operation = repo.operation().clone();
    let mut proven = BTreeSet::new();
    for _ in 0..256 {
        let mut parents = operation.parents().await.map_err(jj)?;
        if parents.len() != 1 {
            break;
        }
        let parent = parents.remove(0);
        if [
            "Stacks: Apply ",
            "Stacks: Unapply ",
            "Stacks: UnapplyBranches ",
            "Stacks: SetTarget ",
            "Stacks: Set target branch to ",
            "Stacks: Update ",
            "Stacks: Reconcile",
        ]
        .iter()
        .any(|prefix| operation.metadata().description.starts_with(prefix))
        {
            let before = parent.view().await.map_err(jj)?;
            let after = operation.view().await.map_err(jj)?;
            let old = before
                .get_local_bookmark(saved.merge_ref.as_ref())
                .as_normal();
            let new = after
                .get_local_bookmark(saved.merge_ref.as_ref())
                .as_normal();
            if let (Some(old), Some(new)) = (old, new)
                && old != new
            {
                let old_commit = repo.store().get_commit(old).map_err(jj)?;
                let new_commit = repo.store().get_commit(new).map_err(jj)?;
                if old_commit.change_id() != new_commit.change_id() {
                    proven.insert(old.clone());
                }
            }
        }
        operation = parent;
    }
    let mut references = repo.view().clone();
    references.store_view_mut().head_ids.clear();
    let protected: BTreeSet<_> = references.all_referenced_commit_ids().cloned().collect();
    let mut candidates = vec![];
    for id in proven {
        if !repo.view().heads().contains(&id) || protected.contains(&id) {
            continue;
        }
        // Heads can be referenced ancestors; protect descendants even if the
        // stored head set has not yet been normalized by JJ.
        let mut descendant = false;
        for other in repo.view().heads().iter().chain(&protected) {
            if other != &id && repo.index().is_ancestor(&id, other).await.map_err(jj)? {
                descendant = true;
                break;
            }
        }
        if !descendant {
            candidates.push(repo.store().get_commit(&id).map_err(jj)?);
        }
    }
    Ok(candidates)
}
