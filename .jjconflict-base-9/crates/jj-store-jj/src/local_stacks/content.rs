//! Selected native tree deltas; staging is UI intent only.
use super::*;
use jj_lib::repo::{MutableRepo, Repo};
use std::collections::BTreeMap;

pub(super) async fn selections(
    repo: &ReadonlyRepo,
    workspace: &jj_lib::ref_name::WorkspaceName,
    settings: &jj_lib::settings::UserSettings,
    config: &Config,
    selections: &[Selection],
) -> Result<Vec<rewrite::CommitWithSelection>> {
    if selections.is_empty() || selections.len() > 256 {
        return Err(Error::Invalid);
    }
    let mut groups = BTreeMap::<String, Vec<Selection>>::new();
    for selection in selections {
        groups
            .entry(selection.source.clone())
            .or_default()
            .push(selection.clone());
    }
    let mut result = Vec::new();
    for (source, paths) in groups {
        let source = policy::commit(repo, &source)?;
        if source.parent_ids().len() != 1 {
            return Err(Error::Invalid);
        }
        policy::rewritable(repo, workspace, settings, config, vec![source.id().clone()])?;
        result.push(mutations::selected(repo, &source, &paths).await?);
    }
    Ok(result)
}
pub(super) async fn remap(
    mutable: &MutableRepo,
    repo: &ReadonlyRepo,
    selections: &[Selection],
) -> Result<Vec<rewrite::CommitWithSelection>> {
    let mut groups = BTreeMap::<String, Vec<Selection>>::new();
    for selection in selections {
        groups
            .entry(selection.source.clone())
            .or_default()
            .push(selection.clone());
    }
    let mut result = Vec::new();
    for (source, paths) in groups {
        let old = policy::commit(repo, &source)?;
        let ids = mutable
            .resolve_change_id(old.change_id())
            .await
            .map_err(jj)?
            .ok_or(Error::Invalid)?
            .into_visible()
            .ok_or(Error::Invalid)?;
        if ids.len() != 1 {
            return Err(Error::Invalid);
        }
        let current = mutable.store().get_commit(&ids[0]).map_err(jj)?;
        let paths = paths
            .into_iter()
            .map(|s| Selection {
                source: current.id().hex(),
                ..s
            })
            .collect::<Vec<_>>();
        result.push(mutations::selected(mutable, &current, &paths).await?);
    }
    Ok(result)
}
pub(super) async fn validate(
    repo: &ReadonlyRepo,
    workspace: &jj_lib::ref_name::WorkspaceName,
    settings: &jj_lib::settings::UserSettings,
    config: &Config,
    command: &Command,
) -> Result<Preview> {
    config.active().ok_or(Error::Invalid)?;
    let (id, selected, title) = match command {
        Command::Transfer { target, selections } => {
            (target, selections.clone(), "Move selected changes")
        }
        Command::Split {
            commit,
            message,
            selections,
        } => {
            mutations::message(message)?;
            if selections.iter().any(|s| &s.source != commit) {
                return Err(Error::Invalid);
            }
            (commit, selections.clone(), "Split selected changes")
        }
        Command::Edit {
            commit,
            message,
            selections,
        } => {
            if message.len() > 65536 || message.contains('\0') {
                return Err(Error::Invalid);
            }
            (
                commit,
                selections
                    .iter()
                    .filter(|s| &s.source != commit)
                    .cloned()
                    .collect(),
                "Edit commit",
            )
        }
        _ => return Err(Error::Invalid),
    };
    let target = policy::commit(repo, id)?;
    if target.parent_ids().len() != 1 {
        return Err(Error::Invalid);
    }
    policy::rewritable(repo, workspace, settings, config, vec![target.id().clone()])?;
    if !matches!(command, Command::Split { .. }) && selected.iter().any(|s| s.source == *id) {
        return Err(Error::Invalid);
    }
    if !selected.is_empty() {
        self::selections(repo, workspace, settings, config, &selected).await?;
    } else if !matches!(command, Command::Edit { .. }) {
        return Err(Error::Invalid);
    }
    Ok(Preview { title: title.into(), summary: "Move only the selected content using native JJ squash. Unselected content and unfinished working-copy changes are retained; descendants follow JJ rewrites.".into(), paths: selected.iter().map(|s| s.path.clone()).collect() })
}
pub(super) async fn squash(
    mutable: &mut MutableRepo,
    selections: &[rewrite::CommitWithSelection],
    target: &Commit,
    message: &str,
) -> Result<Commit> {
    let result = rewrite::squash_commits(mutable, selections, target, true)
        .await
        .map_err(jj)?
        .ok_or(Error::Invalid)?;
    result
        .commit_builder
        .set_description(message)
        .write()
        .await
        .map_err(jj)
}
pub(super) async fn execute(
    mutable: &mut MutableRepo,
    repo: &ReadonlyRepo,
    workspace: &jj_lib::ref_name::WorkspaceName,
    settings: &jj_lib::settings::UserSettings,
    config: &Config,
    command: &Command,
) -> Result<()> {
    validate(repo, workspace, settings, config, command).await?;
    match command {
        Command::Transfer { target, selections } => {
            let target = policy::commit(repo, target)?;
            let sources = self::selections(repo, workspace, settings, config, selections).await?;
            squash(mutable, &sources, &target, target.description()).await?;
        }
        Command::Split {
            commit,
            message,
            selections,
        } => {
            let source = policy::commit(repo, commit)?;
            let target = basic::insert(mutable, repo, &source, Placement::Above, message).await?;
            mutable.rebase_descendants().await.map_err(jj)?;
            let sources = remap(mutable, repo, selections).await?;
            let new = squash(mutable, &sources, &target, message).await?;
            basic::advance_heads(mutable, repo, config, &source, &new);
        }
        Command::Edit {
            commit,
            message,
            selections,
        } => {
            let target = policy::commit(repo, commit)?;
            let other = selections
                .iter()
                .filter(|s| s.source != *commit)
                .cloned()
                .collect::<Vec<_>>();
            if other.is_empty() {
                mutable
                    .rewrite_commit(&target)
                    .set_description(message)
                    .write()
                    .await
                    .map_err(jj)?;
            } else {
                let sources = self::selections(repo, workspace, settings, config, &other).await?;
                squash(mutable, &sources, &target, message).await?;
            }
        }
        _ => return Err(Error::Invalid),
    }
    Ok(())
}
