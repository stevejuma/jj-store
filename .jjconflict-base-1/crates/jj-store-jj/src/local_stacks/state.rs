use super::*;
pub(super) async fn state(
    repo: &ReadonlyRepo,
    config: &Config,
    name: &jj_lib::ref_name::WorkspaceName,
    warning: Option<String>,
) -> Result<State> {
    let wc = repo.view().get_wc_commit_id(name);
    let mut workspaces = vec![];
    for w in &config.workspaces {
        let target = resolve(repo, &w.target);
        let merge = resolve_any(repo, &w.merge_ref);
        let wip = resolve_any(repo, &w.wip_ref);
        let unavailable_reason = if merge.is_err() || wip.is_err() {
            Some("Native retention references are missing or conflicted after a CLI/Undo change. Restore them with native history before switching; saved TOML is retained.".into())
        } else if target.is_err() {
            Some("The target bookmark is missing or conflicted. Choose an available target in Settings.".into())
        } else if w.applied.iter().any(|b| resolve(repo, b).is_err()) {
            Some("An applied bookmark is missing or conflicted. Restore it with JJ or explicitly unapply it; configuration is not automatically reapplied.".into())
        } else {
            None
        };
        let membership_changed = merge.as_ref().is_ok_and(|m| {
            parents(repo, w).is_ok_and(|p| {
                m.parent_ids() != p.iter().map(|c| c.id().clone()).collect::<Vec<_>>()
            })
        });
        workspaces.push(NamedWorkspace {
            id: w.id.clone(),
            name: w.name.clone(),
            target: w.target.clone(),
            target_commit: target.as_ref().map(|c| c.id().hex()).unwrap_or_default(),
            applied: w.applied.clone(),
            merge_commit: merge.as_ref().map(|c| c.id().hex()).unwrap_or_default(),
            wip_commit: wip.as_ref().map(|c| c.id().hex()).unwrap_or_default(),
            conflicted: wip.as_ref().is_ok_and(|c| c.has_conflict()),
            unavailable_reason,
            membership_changed,
        });
    }
    let active = config
        .active
        .as_ref()
        .filter(|id| {
            workspaces.iter().any(|w| {
                &w.id == *id
                    && !w.merge_commit.is_empty()
                    && wc.is_some_and(|c| c.hex() == w.wip_commit)
            })
        })
        .cloned();
    let conflicts = if active.is_some() {
        repo.store()
            .get_commit(wc.ok_or(Error::Invalid)?)
            .map_err(jj)?
            .tree()
            .conflicts()
            .take(50)
            .map(|(p, v)| {
                v.map_err(jj)
                    .map(|_| p.as_internal_file_string().to_owned())
            })
            .collect::<Result<Vec<_>>>()?
    } else {
        vec![]
    };
    Ok(State {
        version: 1,
        operation: repo.op_id().hex().parse().map_err(|_| Error::Invalid)?,
        revision: config.revision,
        active,
        warning,
        workspaces,
        can_undo: history::target(repo, false).await.is_ok(),
        can_redo: history::target(repo, true).await.is_ok(),
        conflicts,
    })
}
