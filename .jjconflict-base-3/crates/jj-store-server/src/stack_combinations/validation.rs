use super::*;
pub(super) fn scope(value: &j::Scope, principal: m::PrincipalId) -> Result<()> {
    value.validate().map_err(|_| Error::Invalid)?;
    if principal.as_bytes() == &[0; 16] || value.profile != j::Profile::GitSha1Jj045 {
        return Err(Error::Invalid);
    }
    Ok(())
}
fn text(value: &str, limit: usize) -> Result<()> {
    if value.trim().is_empty() || value.len() > limit || value.chars().any(char::is_control) {
        return Err(Error::Invalid);
    }
    Ok(())
}
pub(super) fn snapshot(
    scope: &j::Scope,
    principal: m::PrincipalId,
    state: &Snapshot,
) -> Result<()> {
    if state.version != 1
        || state.combinations.len() > MAX_COMBINATIONS
        || state.active.is_some() != !state.combinations.is_empty()
        || state.active.is_some() && state.active().is_none()
    {
        return Err(Error::Invalid);
    }
    let mut ids = BTreeSet::new();
    let mut names = BTreeSet::new();
    for c in &state.combinations {
        text(&c.name, 256)?;
        text(&c.target.name, 1024)?;
        if let Some(remote) = &c.target.remote {
            text(remote, 256)?;
        }
        let s::CommitIdentity::GitSha1(target) = c.target.head else {
            return Err(Error::Invalid);
        };
        if !matches!(c.code, s::CodeSelection::GitSha1Jj045 { .. })
            || c.selection.generation == 0
            || c.selection.id.0.as_bytes() == &[0; 16]
            || !ids.insert(c.selection.id)
            || !names.insert(&c.name)
            || c.immutable_heads.is_empty()
            || c.immutable_heads.len() > 64
            || !c.immutable_heads.contains(&target)
            || c.immutable_heads.iter().collect::<BTreeSet<_>>().len() != c.immutable_heads.len()
            || c.stacks.len() > MAX_STACKS
            || c.retention.key.repository != scope.repository
            || c.retention.key.physical_workspace != scope.physical_workspace.0
            || c.retention.key.principal != principal
            || c.retention.key.request.as_bytes() == &[0; 16]
            || c.retention.key.intent.as_bytes() == &[0; 32]
            || c.retention.key.prepared.as_bytes() == &[0; 32]
            || c.retention.archive.as_bytes() == &[0; 32]
        {
            return Err(Error::Invalid);
        }
        let mut stacks = BTreeSet::new();
        for stack in &c.stacks {
            text(&stack.bookmark, 1024)?;
            if stack.id.0.as_bytes() == &[0; 16] || !stacks.insert(stack.id) {
                return Err(Error::Invalid);
            }
        }
    }
    Ok(())
}
pub(super) fn selection(scope: &j::Scope, before: &Snapshot, preview: &s::Preview) -> Result<()> {
    let request = &preview.plan.request;
    let selection = &request.selection;
    if selection.repository != scope.repository
        || selection.physical_workspace != scope.physical_workspace
        || selection.jj_workspace != scope.jj_workspace
        || !matches!(selection.code, s::CodeSelection::GitSha1Jj045 { .. })
    {
        return Err(Error::Forbidden);
    }
    match before.active() {
        Some(current)
            if selection.combination.as_ref() == Some(&current.selection)
                && selection.code == current.code => {}
        None if selection.combination.is_none()
            && matches!(request.command, s::Command::Setup { .. }) => {}
        _ => return Err(Error::Stale),
    }
    Ok(())
}
pub(super) fn retained(
    scope: &j::Scope,
    principal: m::PrincipalId,
    preview: &s::Preview,
    retained: &StackRetention,
) -> Result<RetainedCode> {
    let history = retained.history.as_ref().ok_or(Error::Invalid)?;
    let capsule = history.capsule().map_err(|_| Error::Invalid)?;
    if retained.key != j::preparation_key(principal, preview)
        || retained.key.repository != scope.repository
        || retained.key.physical_workspace != scope.physical_workspace.0
        || capsule != retained.capsule
        || capsule.operation_id != preview.plan.operation
        || capsule.view_id != preview.plan.view
    {
        return Err(Error::IntentMismatch);
    }
    Ok(RetainedCode {
        key: retained.key.clone(),
        commit: retained.commit,
        archive: history.id().map_err(|_| Error::Invalid)?,
    })
}
pub(super) fn transition(
    scope: &j::Scope,
    principal: m::PrincipalId,
    before: &Snapshot,
    record: &Record,
) -> Result<()> {
    snapshot(scope, principal, &record.after)?;
    selection(scope, before, &record.preview)?;
    let active = record.after.active().ok_or(Error::Invalid)?;
    let s::CodeSelection::GitSha1Jj045 {
        operation, view, ..
    } = active.code
    else {
        return Err(Error::Invalid);
    };
    if active.selection != record.preview.plan.combination
        || operation != record.preview.plan.operation
        || view != record.preview.plan.view
    {
        return Err(Error::IntentMismatch);
    }
    for c in &record.after.combinations {
        let old = before
            .combinations
            .iter()
            .find(|old| old.selection.id == c.selection.id);
        if old == Some(c) {
            continue;
        }
        if c.selection.generation
            != old
                .map_or(Some(1), |o| o.selection.generation.checked_add(1))
                .ok_or(Error::Capacity)?
            || c.retention != record.retention
        {
            return Err(Error::Stale);
        }
    }
    if let s::Command::Setup { target, name } = &record.preview.plan.request.command
        && (before.active.is_some()
            || record.after.combinations.len() != 1
            || &active.target != target
            || &active.name != name
            || !active.stacks.is_empty())
    {
        return Err(Error::Invalid);
    }
    Ok(())
}
pub(super) fn proposed(
    scope: &j::Scope,
    before: &Snapshot,
    record: &Record,
    retained: &StackRetention,
) -> Result<()> {
    let after = &record.after;
    let preview = &record.preview;
    if before.revision != record.before_revision
        || after.revision != before.revision.checked_add(1).ok_or(Error::Capacity)?
    {
        return Err(Error::Stale);
    }
    transition(scope, retained.key.principal, before, record)?;
    let active = after.active().ok_or(Error::Invalid)?;
    let s::CodeSelection::GitSha1Jj045 {
        operation,
        view,
        working_copy,
    } = &active.code
    else {
        return Err(Error::Invalid);
    };
    if active.selection != preview.plan.combination
        || *operation != preview.plan.operation
        || *view != preview.plan.view
        || retained.capsule.view.wc_commit_ids.get(&scope.jj_workspace) != Some(working_copy)
    {
        return Err(Error::IntentMismatch);
    }
    let history = retained.history.as_ref().ok_or(Error::Invalid)?;
    for c in &after.combinations {
        let old = before
            .combinations
            .iter()
            .find(|old| old.selection.id == c.selection.id);
        if old == Some(c) {
            continue;
        }
        if c.selection.generation
            != old
                .map_or(Some(1), |o| o.selection.generation.checked_add(1))
                .ok_or(Error::Capacity)?
            || c.retention != record.retention
        {
            return Err(Error::Stale);
        }
        let s::CodeSelection::GitSha1Jj045 {
            operation,
            view,
            working_copy,
        } = c.code
        else {
            return Err(Error::Invalid);
        };
        if history.operations.get(&operation).map(|o| o.view_id) != Some(view)
            || history
                .views
                .get(&view)
                .and_then(|v| v.wc_commit_ids.get(&scope.jj_workspace))
                != Some(&working_copy)
        {
            return Err(Error::Invalid);
        }
        let s::CommitIdentity::GitSha1(target) = c.target.head else {
            return Err(Error::Invalid);
        };
        let mut ids = vec![working_copy, c.merge, target];
        ids.extend(&c.immutable_heads);
        for stack in &c.stacks {
            ids.extend([stack.head, stack.base]);
        }
        if history
            .commits
            .get(&working_copy)
            .map(|commit| commit.parents.as_slice())
            != Some(&[c.merge][..])
        {
            return Err(Error::Invalid);
        }
        if ids.iter().any(|id| !history.commits.contains_key(id)) {
            return Err(Error::Invalid);
        }
    }
    if let s::Command::Setup { target, name } = &preview.plan.request.command
        && (before.active.is_some()
            || after.combinations.len() != 1
            || &active.target != target
            || &active.name != name
            || !active.stacks.is_empty())
    {
        return Err(Error::Invalid);
    }
    Ok(())
}
