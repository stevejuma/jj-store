use super::*;
use std::collections::BTreeMap;
pub(super) fn build(
    source: GitEvidenceCapsuleV1,
    capsule: GitEvidenceCapsuleV1,
    history: OperationArchiveV1,
    expected: &CurrentState,
    request: &Reword,
    affected: &BTreeSet<m::GitSha1Id>,
) -> Result<PreparedCode> {
    if capsule.operation.parents != [source.operation_id]
        || capsule.commits.len() > MAX_COMMITS * 2
        || capsule.view.local_bookmarks.len() > MAX_BOOKMARKS
    {
        return Err(Error::Corrupt);
    }
    let mut rewrites: BTreeMap<m::ChangeId, Rewrite> = BTreeMap::new();
    let mut rewritten = BTreeSet::new();
    for (after, before) in capsule
        .operation
        .commit_predecessors
        .as_ref()
        .ok_or(Error::Corrupt)?
    {
        if before.is_empty() {
            return Err(Error::Corrupt);
        }
        let new = capsule.commits.get(after).ok_or(Error::Incomplete)?;
        for before in before {
            rewritten.insert(*before);
            let old = source.commits.get(before).ok_or(Error::Incomplete)?;
            if old.change_id != new.change_id
                || old.root_tree != new.root_tree
                || old.conflict_labels != new.conflict_labels
                || old.author != new.author
                || new.description
                    != if *before == request.commit {
                        &request.message
                    } else {
                        &old.description
                    }
                    .as_str()
            {
                return Err(Error::Corrupt);
            }
            let rewrite = rewrites.entry(new.change_id).or_insert_with(|| Rewrite {
                change: new.change_id,
                before: vec![],
                after: vec![],
            });
            rewrite.before.push(*before);
            rewrite.after.push(*after);
        }
    }
    if rewrites.is_empty() || rewrites.len() > MAX_COMMITS || &rewritten != affected {
        return Err(Error::Corrupt);
    }
    for rewrite in rewrites.values_mut() {
        rewrite.before.sort();
        rewrite.before.dedup();
        rewrite.after.sort();
        rewrite.after.dedup();
    }
    let names: BTreeSet<_> = source
        .view
        .local_bookmarks
        .keys()
        .chain(capsule.view.local_bookmarks.keys())
        .collect();
    let absent = GitRefTargetV1(vec![None]);
    let mut bookmarks = vec![];
    for name in names {
        let before = source.view.local_bookmarks.get(name).unwrap_or(&absent);
        let after = capsule.view.local_bookmarks.get(name).unwrap_or(&absent);
        if before != after {
            bookmarks.push(BookmarkEffect {
                name: name.clone(),
                before: before.clone(),
                after: after.clone(),
            });
        }
    }
    let checkout_before = source
        .commits
        .get(&expected.baseline.commit)
        .ok_or(Error::Incomplete)?
        .root_tree
        .clone();
    let checkout_after = capsule
        .commits
        .get(
            capsule
                .view
                .wc_commit_ids
                .get(&expected.checkout.workspace)
                .ok_or(Error::Corrupt)?,
        )
        .ok_or(Error::Incomplete)?
        .root_tree
        .clone();
    if checkout_before != expected.checkout.trees || checkout_before != checkout_after {
        return Err(Error::Corrupt);
    }
    Ok(PreparedCode {
        source,
        capsule,
        history,
        rewrites: rewrites.into_values().collect(),
        bookmarks,
        checkout_before,
        checkout_after,
    })
}
