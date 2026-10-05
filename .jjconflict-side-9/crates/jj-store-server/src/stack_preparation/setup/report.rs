use super::*;
use crate::stack_combinations::{Combination, RetainedCode, Snapshot};
pub(super) fn check(request: &s::PrepareRequest, context: &SetupContext) -> Result<()> {
    let selected = &request.selection;
    let current = &context.current;
    if selected.combination.is_some()
        || selected.repository != current.baseline.repository
        || selected.binding_generation != current.baseline.generation
        || selected.physical_workspace.0 != current.physical_workspace
        || selected.jj_workspace != current.checkout.workspace
        || selected.working_tree != context.working_tree
        || selected.code
            != (s::CodeSelection::GitSha1Jj045 {
                operation: current.baseline.operation,
                view: current.baseline.view,
                working_copy: current.baseline.commit,
            })
        || context.access.repository() != selected.repository
        || context.access.generation() != selected.binding_generation
    {
        return Err(Error::Stale);
    }
    Ok(())
}
pub(super) fn preview(request: s::PrepareRequest, candidate: &PreparedSetup) -> Result<s::Preview> {
    let mut id = [0; 16];
    getrandom::fill(&mut id).map_err(|_| Error::Invalid)?;
    let old = &candidate.source.commits[&match request.selection.code {
        s::CodeSelection::GitSha1Jj045 { working_copy, .. } => working_copy,
        _ => return Err(Error::Unsupported),
    }];
    let tree = s::TreeSelection {
        terms: old
            .root_tree
            .iter()
            .map(|id| s::TreeIdentity::GitSha1(*id))
            .collect(),
    };
    let plan = s::PreparedPlan {
        version: 1,
        request,
        operation: candidate.capsule.operation_id,
        view: candidate.capsule.view_id,
        checkout: s::CheckoutEffect {
            before: tree.clone(),
            after: tree,
            paths: vec![],
        },
        combination: s::CombinationSelection {
            id: s::CombinationId(m::RequestId::new(id)),
            generation: 1,
        },
        rewrites: vec![],
        bookmarks: vec![],
        conflicts: vec![],
    };
    Ok(s::Preview {
        prepared: plan.identity().map_err(|_| Error::Invalid)?,
        plan,
    })
}
pub(super) fn snapshot(preview: &s::Preview, retained: &StackRetention) -> Result<Snapshot> {
    let request = &preview.plan.request;
    let s::Command::Setup { target, name } = &request.command else {
        return Err(Error::Unsupported);
    };
    let s::CommitIdentity::GitSha1(head) = target.head else {
        return Err(Error::Unsupported);
    };
    let history = retained.history.as_ref().ok_or(Error::Invalid)?;
    let capsule = history.capsule().map_err(|_| Error::Invalid)?;
    let wip = *capsule
        .view
        .wc_commit_ids
        .get(&request.selection.jj_workspace)
        .ok_or(Error::Invalid)?;
    let commit = capsule.commits.get(&wip).ok_or(Error::Invalid)?;
    let [merge] = commit.parents.as_slice() else {
        return Err(Error::Invalid);
    };
    let merged = capsule.commits.get(merge).ok_or(Error::Invalid)?;
    let s::CodeSelection::GitSha1Jj045 {
        operation,
        view,
        working_copy,
    } = request.selection.code
    else {
        return Err(Error::Unsupported);
    };
    let source = history.operations.get(&operation).ok_or(Error::Invalid)?;
    let source_view = history.views.get(&view).ok_or(Error::Invalid)?;
    let old = history.commits.get(&working_copy).ok_or(Error::Invalid)?;
    let base = history.commits.get(&head).ok_or(Error::Invalid)?;
    let selected = if let Some(remote) = &target.remote {
        source_view
            .remote_views
            .get(remote)
            .and_then(|v| v.bookmarks.get(&target.name))
            .map(|r| &r.target)
    } else {
        source_view.local_bookmarks.get(&target.name)
    };
    if retained.capsule != capsule
        || retained.key != j::preparation_key(retained.key.principal, preview)
        || capsule.operation_id != preview.plan.operation
        || capsule.view_id != preview.plan.view
        || capsule.operation.parents != [operation]
        || source.view_id != view
        || source_view
            .wc_commit_ids
            .get(&request.selection.jj_workspace)
            != Some(&working_copy)
        || selected
            != Some(&jj_store_jj::local_evidence::GitRefTargetV1(vec![Some(
                head,
            )]))
        || merged.parents != [head]
        || merged.root_tree != base.root_tree
        || preview.plan.checkout.before.terms
            != old
                .root_tree
                .iter()
                .map(|id| s::TreeIdentity::GitSha1(*id))
                .collect::<Vec<_>>()
        || capsule.view.local_bookmarks != source_view.local_bookmarks
        || capsule.view.local_tags != source_view.local_tags
        || capsule.view.remote_views != source_view.remote_views
        || capsule.view.git_refs != source_view.git_refs
        || capsule.view.git_heads != source_view.git_heads
        || source_view.wc_commit_ids.iter().any(|(name, id)| {
            name != &request.selection.jj_workspace
                && capsule.view.wc_commit_ids.get(name) != Some(id)
        })
        || capsule.view.wc_commit_ids.len() != source_view.wc_commit_ids.len()
        || commit.root_tree != old.root_tree
        || old.root_tree.len() != 1
        || base.root_tree.len() != 1
        || !merged.predecessors.is_empty()
        || !commit.predecessors.is_empty()
        || merged.change_id == commit.change_id
        || commit.change_id == old.change_id
        || preview.plan.combination.generation != 1
        || request.selection.combination.is_some()
        || preview.plan.checkout.before != preview.plan.checkout.after
        || !preview.plan.checkout.paths.is_empty()
        || !preview.plan.rewrites.is_empty()
        || !preview.plan.bookmarks.is_empty()
        || !preview.plan.conflicts.is_empty()
    {
        return Err(Error::Invalid);
    }
    Ok(Snapshot {
        version: 1,
        revision: 1,
        active: Some(preview.plan.combination.id),
        combinations: vec![Combination {
            selection: preview.plan.combination.clone(),
            name: name.clone(),
            target: target.clone(),
            immutable_heads: vec![head],
            merge: *merge,
            code: s::CodeSelection::GitSha1Jj045 {
                operation: capsule.operation_id,
                view: capsule.view_id,
                working_copy: wip,
            },
            stacks: vec![],
            retention: RetainedCode {
                key: retained.key.clone(),
                commit: retained.commit,
                archive: history.id().map_err(|_| Error::Invalid)?,
            },
        }],
    })
}
