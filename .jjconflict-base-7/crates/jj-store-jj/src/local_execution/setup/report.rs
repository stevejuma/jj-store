use super::*;
pub(super) fn build(
    source_history: OperationArchiveV1,
    history: OperationArchiveV1,
    target: SetupBookmark,
    merge: m::GitSha1Id,
    wip: m::GitSha1Id,
    expected: &CurrentState,
) -> Result<PreparedSetup> {
    let source = source_history
        .capsule()
        .map_err(super::super::reword::archive_error)?;
    let capsule = history
        .capsule()
        .map_err(super::super::reword::archive_error)?;
    if source_history
        .operations
        .iter()
        .any(|(id, data)| history.operations.get(id) != Some(data))
        || source_history
            .views
            .iter()
            .any(|(id, data)| history.views.get(id) != Some(data))
        || source_history
            .commits
            .iter()
            .any(|(id, data)| history.commits.get(id) != Some(data))
        || capsule.operation.parents != [source.operation_id]
        || capsule.view.wc_commit_ids.get(&expected.checkout.workspace) != Some(&wip)
        || capsule.commits.get(&wip).map(|c| &c.root_tree) != Some(&expected.checkout.trees)
        || [expected.baseline.commit, target.head].contains(&merge)
        || [expected.baseline.commit, target.head, merge].contains(&wip)
    {
        return Err(Error::Corrupt);
    }
    verify(
        source,
        capsule,
        history,
        target,
        merge,
        wip,
        &RecoveryBinding {
            repository: expected.baseline.repository,
            generation: expected.baseline.generation,
            physical_workspace: expected.physical_workspace,
            operation: expected.baseline.operation,
            view: expected.baseline.view,
            commit: expected.baseline.commit,
            workspace: expected.checkout.workspace.clone(),
        },
    )
}
fn verify(
    source: GitEvidenceCapsuleV1,
    capsule: GitEvidenceCapsuleV1,
    history: OperationArchiveV1,
    target: SetupBookmark,
    merge: m::GitSha1Id,
    wip: m::GitSha1Id,
    binding: &RecoveryBinding,
) -> Result<PreparedSetup> {
    let old = source
        .commits
        .get(&binding.commit)
        .ok_or(Error::Incomplete)?;
    let base = source.commits.get(&target.head).ok_or(Error::Incomplete)?;
    let merged = capsule.commits.get(&merge).ok_or(Error::Incomplete)?;
    let unfinished = capsule.commits.get(&wip).ok_or(Error::Incomplete)?;
    if old.root_tree.len() != 1
        || base.root_tree.len() != 1
        || source.commits.contains_key(&merge)
        || source.commits.contains_key(&wip)
        || merged.parents != [target.head]
        || merged.root_tree != base.root_tree
        || unfinished.parents != [merge]
        || unfinished.root_tree != old.root_tree
        || source
            .commits
            .values()
            .any(|c| c.change_id == merged.change_id || c.change_id == unfinished.change_id)
        || old.change_id == unfinished.change_id
        || merged.change_id == unfinished.change_id
        || !merged.predecessors.is_empty()
        || !unfinished.predecessors.is_empty()
        || capsule.view.local_bookmarks != source.view.local_bookmarks
        || capsule.view.local_tags != source.view.local_tags
        || capsule.view.remote_views != source.view.remote_views
        || capsule.view.git_refs != source.view.git_refs
        || capsule.view.git_heads != source.view.git_heads
        || source.view.wc_commit_ids.iter().any(|(name, id)| {
            name != &binding.workspace && capsule.view.wc_commit_ids.get(name) != Some(id)
        })
        || capsule.view.wc_commit_ids.len() != source.view.wc_commit_ids.len()
        || !super::super::reword::visible(&source)?
            .is_subset(&super::super::reword::visible(&capsule)?)
    {
        return Err(Error::Corrupt);
    }
    Ok(PreparedSetup {
        source,
        capsule,
        history,
        target,
        merge,
        wip,
    })
}

pub(in crate::local_execution) fn recover(
    history: OperationArchiveV1,
    target: SetupBookmark,
    binding: &RecoveryBinding,
) -> Result<PreparedSetup> {
    let source = history
        .capsule_at(binding.operation)
        .map_err(super::super::reword::archive_error)?;
    let capsule = history
        .capsule()
        .map_err(super::super::reword::archive_error)?;
    super::target(&source, &target)?;
    if source.repository != binding.repository
        || source.view_id != binding.view
        || source.view.wc_commit_ids.get(&binding.workspace) != Some(&binding.commit)
        || capsule.operation.parents != [source.operation_id]
    {
        return Err(Error::Corrupt);
    }
    let wip = *capsule
        .view
        .wc_commit_ids
        .get(&binding.workspace)
        .ok_or(Error::Incomplete)?;
    let commit = capsule.commits.get(&wip).ok_or(Error::Incomplete)?;
    let [merge] = commit.parents.as_slice() else {
        return Err(Error::Corrupt);
    };
    if source.commits.contains_key(&wip) || source.commits.contains_key(merge) {
        return Err(Error::Corrupt);
    }
    verify(
        source,
        capsule.clone(),
        history,
        target,
        *merge,
        wip,
        binding,
    )
}
