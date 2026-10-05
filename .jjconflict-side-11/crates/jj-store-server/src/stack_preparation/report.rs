use super::*;
use jj_store_jj::local_execution::PreparedCode;
pub(super) fn check(request: &s::PrepareRequest, context: &Context) -> Result<()> {
    let selected = &request.selection;
    let current = &context.current;
    if selected.repository != current.baseline.repository
        || selected.binding_generation != current.baseline.generation
        || selected.physical_workspace.0 != current.physical_workspace
        || selected.jj_workspace != current.checkout.workspace
        || selected.working_tree != context.working_tree
        || selected.combination.as_ref() != Some(&context.combination)
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
pub(super) fn preview(request: s::PrepareRequest, candidate: &PreparedCode) -> Result<s::Preview> {
    let s::CodeSelection::GitSha1Jj045 {
        operation,
        view,
        working_copy,
    } = &request.selection.code
    else {
        return Err(Error::Unsupported);
    };
    if candidate.source.operation_id != *operation
        || candidate.source.view_id != *view
        || candidate
            .source
            .view
            .wc_commit_ids
            .get(&request.selection.jj_workspace)
            != Some(working_copy)
        || candidate.checkout_before != candidate.checkout_after
        || candidate.history.capsule().map_err(|_| Error::Invalid)? != candidate.capsule
    {
        return Err(Error::Invalid);
    }
    let mut combination = request
        .selection
        .combination
        .clone()
        .ok_or(Error::Invalid)?;
    combination.generation = combination
        .generation
        .checked_add(1)
        .ok_or(Error::Invalid)?;
    let tree = |terms: &[m::GitSha1Id]| s::TreeSelection {
        terms: terms
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
            before: tree(&candidate.checkout_before),
            after: tree(&candidate.checkout_after),
            paths: vec![],
        },
        combination,
        rewrites: candidate
            .rewrites
            .iter()
            .map(|r| s::Rewrite {
                change: r.change,
                before: r
                    .before
                    .iter()
                    .map(|id| s::CommitIdentity::GitSha1(*id))
                    .collect(),
                after: r
                    .after
                    .iter()
                    .map(|id| s::CommitIdentity::GitSha1(*id))
                    .collect(),
            })
            .collect(),
        bookmarks: candidate
            .bookmarks
            .iter()
            .map(|r| s::BookmarkEffect {
                name: r.name.clone(),
                before: r
                    .before
                    .0
                    .iter()
                    .map(|id| id.map(s::CommitIdentity::GitSha1))
                    .collect(),
                after: r
                    .after
                    .0
                    .iter()
                    .map(|id| id.map(s::CommitIdentity::GitSha1))
                    .collect(),
            })
            .collect(),
        conflicts: vec![],
    };
    Ok(s::Preview {
        prepared: plan.identity().map_err(|_| Error::Invalid)?,
        plan,
    })
}
