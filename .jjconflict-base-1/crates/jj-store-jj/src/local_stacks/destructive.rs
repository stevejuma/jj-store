//! Explicit native abandonment, bookmark deletion and selected WIP discard.
use super::*;
use jj_lib::{
    merge::{Diff, Merge},
    merged_tree::MergedTree,
    repo::MutableRepo,
};
pub(super) async fn validate(
    repo: &ReadonlyRepo,
    workspace: &jj_lib::ref_name::WorkspaceName,
    settings: &jj_lib::settings::UserSettings,
    config: &Config,
    command: &Command,
) -> Result<Preview> {
    let title = match command {
        Command::DeleteBranch { branch } => {
            if branch.contains('@')
                || branch.starts_with(config::INTERNAL_PREFIX)
                || config.workspaces.iter().any(|w| w.target == *branch)
            {
                return Err(Error::Invalid);
            }
            resolve(repo, branch)?;
            "Delete branch bookmark?"
        }
        Command::Abandon { commit } => {
            let c = policy::commit(repo, commit)?;
            if c.parent_ids().len() != 1
                || config.workspaces.iter().any(|w| {
                    [&w.wip_ref, &w.merge_ref].iter().any(|r| {
                        repo.view().get_local_bookmark((*r).as_ref()).as_normal() == Some(c.id())
                    })
                })
            {
                return Err(Error::Invalid);
            }
            policy::rewritable(repo, workspace, settings, config, vec![c.id().clone()])?;
            "Delete commit?"
        }
        Command::Discard { selections } => {
            let wc = repo
                .view()
                .get_wc_commit_id(workspace)
                .ok_or(Error::Invalid)?;
            if selections.iter().any(|s| s.source != wc.hex()) {
                return Err(Error::Invalid);
            }
            content::selections(repo, workspace, settings, config, selections).await?;
            "Discard selected changes?"
        }
        _ => return Err(Error::Invalid),
    };
    Ok(Preview{title:title.into(),summary:match command {Command::DeleteBranch{..}=>"Remove only the chosen bookmark and its saved membership. Lower bookmarks and native commits remain reachable. Native Undo restores the bookmark; TOML configuration remains separate.",_=>"Apply this destructive change with native JJ. Unselected content is retained. Repository-wide Undo can restore code; workspace TOML is separate."}.into(),paths:if let Command::Discard{selections}=command {selections.iter().map(|s|s.path.clone()).collect()} else {vec![]}})
}
pub(super) async fn execute(
    mutable: &mut MutableRepo,
    repo: &ReadonlyRepo,
    workspace: &jj_lib::ref_name::WorkspaceName,
    settings: &jj_lib::settings::UserSettings,
    config: &mut Config,
    wc: &Commit,
    command: &Command,
) -> Result<()> {
    validate(repo, workspace, settings, config, command).await?;
    match command {
        Command::DeleteBranch { branch } => {
            mutable.set_local_bookmark_target(branch.as_ref(), RefTarget::absent());
            for saved in &mut config.workspaces {
                if !saved.applied.contains(branch) {
                    continue;
                }
                saved.applied.retain(|b| b != branch);
                let old = resolve_any(mutable, &saved.wip_ref)?;
                if parents(mutable, saved).is_err() {
                    continue;
                }
                let wip = build(mutable, repo, saved, Some(&old), false).await?;
                if config.active.as_ref() == Some(&saved.id) {
                    mutable
                        .set_wc_commit(workspace.to_owned(), wip.id().clone())
                        .map_err(jj)?;
                }
            }
        }
        Command::Abandon { commit } => {
            mutable.record_abandoned_commit(&policy::commit(repo, commit)?)
        }
        Command::Discard { selections } => {
            let selected =
                content::selections(repo, workspace, settings, config, selections).await?;
            let tree = MergedTree::merge(Merge::from_diffs(
                (wc.tree(), "working copy".into()),
                selected.into_iter().map(|s| {
                    Diff::new(
                        (s.selected_tree, "selected".into()),
                        (s.parent_tree, "discard selection".into()),
                    )
                }),
            ))
            .await
            .map_err(jj)?;
            mutable
                .rewrite_commit(wc)
                .set_tree(tree)
                .write()
                .await
                .map_err(jj)?;
        }
        _ => return Err(Error::Invalid),
    }
    Ok(())
}
