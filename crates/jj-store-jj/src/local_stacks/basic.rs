//! Native bookmarks, descriptions and positional empty commits.
use super::*;
use futures::TryStreamExt as _;
use jj_lib::{repo::MutableRepo, revset::RevsetExpression};

fn text(value: &str) -> Result<()> {
    if value.len() > 65536 || value.contains('\0') {
        return Err(Error::Invalid);
    }
    Ok(())
}
fn branch(repo: &ReadonlyRepo, name: &str) -> Result<()> {
    if name.is_empty()
        || name != name.trim()
        || name.len() > 128
        || name.chars().any(char::is_control)
        || name.contains('@')
        || name.starts_with(config::INTERNAL_PREFIX)
        || repo.view().get_local_bookmark(name.as_ref()).is_present()
    {
        return Err(Error::Invalid);
    }
    Ok(())
}
pub(super) async fn children(
    repo: &ReadonlyRepo,
    head: &Commit,
) -> Result<Vec<jj_lib::backend::CommitId>> {
    RevsetExpression::commits(vec![head.id().clone()])
        .children()
        .evaluate(repo)
        .map_err(jj)?
        .stream()
        .try_collect()
        .await
        .map_err(jj)
}
pub(super) async fn validate(
    repo: &ReadonlyRepo,
    workspace: &jj_lib::ref_name::WorkspaceName,
    settings: &jj_lib::settings::UserSettings,
    config: &Config,
    command: &Command,
) -> Result<Preview> {
    let saved = config.active().ok_or(Error::Invalid)?;
    let (title, summary) = match command {
        Command::CreateStack { name, parent } => {
            branch(repo, name)?;
            let reference = parent.as_deref().unwrap_or(&saved.target);
            resolve(repo, reference)?;
            if parent.is_some() && !saved.applied.iter().any(|b| b == reference) {
                return Err(Error::Invalid);
            }
            (
                "Create stack",
                format!(
                    "Create and apply {name} on {reference}. Existing commits and unfinished work are retained."
                ),
            )
        }
        Command::CreateBranch {
            commit,
            name,
            placement,
        } => {
            branch(repo, name)?;
            let commit = policy::commit(repo, commit)?;
            if matches!(placement, Placement::Below) && commit.parent_ids().len() != 1 {
                return Err(Error::Invalid);
            }
            (
                "Create branch",
                format!(
                    "Create {name} {placement:?} the selected commit. Commit content is unchanged."
                ),
            )
        }
        Command::RenameBranch { branch: old, name } => {
            branch(repo, name)?;
            if old.contains('@') || config.workspaces.iter().any(|w| w.target == *old) {
                return Err(Error::Invalid);
            }
            resolve(repo, old)?;
            (
                "Rename branch",
                format!("Rename {old} to {name}. Applied membership follows the new name."),
            )
        }
        Command::Reword { commit, message } => {
            text(message)?;
            let commit = policy::commit(repo, commit)?;
            policy::rewritable(repo, workspace, settings, config, vec![commit.id().clone()])?;
            (
                "Reword commit",
                "Update the selected description. JJ propagates the rewrite to descendants.".into(),
            )
        }
        Command::Empty {
            commit,
            placement,
            message,
        } => {
            text(message)?;
            let commit = policy::commit(repo, commit)?;
            let affected = if matches!(placement, Placement::Above) {
                children(repo, &commit).await?
            } else {
                vec![commit.id().clone()]
            };
            policy::rewritable(repo, workspace, settings, config, affected)?;
            (
                "Add empty commit",
                format!(
                    "Insert an empty commit {placement:?} the selected commit and rebase descendants with JJ."
                ),
            )
        }
        _ => return Err(Error::Invalid),
    };
    Ok(Preview {
        title: title.into(),
        summary,
        paths: vec![],
    })
}
/// Same insertion semantics as jj new --no-edit --insert-after/--insert-before.
pub(super) async fn insert(
    mutable: &mut MutableRepo,
    repo: &ReadonlyRepo,
    selected: &Commit,
    placement: Placement,
    message: &str,
) -> Result<Commit> {
    if matches!(placement, Placement::Below) {
        let tree = selected.parent_tree(repo).await.map_err(jj)?;
        let new = mutable
            .new_commit(selected.parent_ids().to_vec(), tree)
            .set_description(message)
            .write()
            .await
            .map_err(jj)?;
        rewrite::rebase_commit(mutable, selected.clone(), vec![new.id().clone()])
            .await
            .map_err(jj)?;
        Ok(new)
    } else {
        let new = mutable
            .new_commit(vec![selected.id().clone()], selected.tree())
            .set_description(message)
            .write()
            .await
            .map_err(jj)?;
        mutable
            .transform_descendants(children(repo, selected).await?, async |mut rewriter| {
                rewriter.replace_parent(selected.id(), [new.id()]);
                rewriter.rebase().await?.write().await?;
                Ok(())
            })
            .await
            .map_err(jj)?;
        Ok(new)
    }
}
#[allow(clippy::too_many_arguments)]
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
        Command::CreateStack { name, parent } => {
            let saved = config.active_mut().ok_or(Error::Invalid)?;
            let tip = resolve(repo, parent.as_deref().unwrap_or(&saved.target))?;
            mutable.set_local_bookmark_target(name.as_ref(), RefTarget::normal(tip.id().clone()));
            saved.applied.push(name.clone());
            if needs_rebuild(mutable, saved)? {
                let wip = build(mutable, repo, saved, Some(wc), false).await?;
                mutable
                    .set_wc_commit(workspace.to_owned(), wip.id().clone())
                    .map_err(jj)?;
            }
        }
        Command::CreateBranch {
            commit,
            name,
            placement,
        } => {
            let commit = policy::commit(repo, commit)?;
            let target = if matches!(placement, Placement::Above) {
                commit.id()
            } else {
                &commit.parent_ids()[0]
            };
            mutable.set_local_bookmark_target(name.as_ref(), RefTarget::normal(target.clone()));
        }
        Command::RenameBranch { branch, name } => {
            let target = repo.view().get_local_bookmark(branch.as_ref()).clone();
            mutable.set_local_bookmark_target(name.as_ref(), target);
            mutable.set_local_bookmark_target(branch.as_ref(), RefTarget::absent());
            for saved in &mut config.workspaces {
                for applied in &mut saved.applied {
                    if applied == branch {
                        *applied = name.clone();
                    }
                }
            }
        }
        Command::Reword { commit, message } => {
            mutable
                .rewrite_commit(&policy::commit(repo, commit)?)
                .set_description(message)
                .write()
                .await
                .map_err(jj)?;
        }
        Command::Empty {
            commit,
            placement,
            message,
        } => {
            let selected = policy::commit(repo, commit)?;
            let new = insert(mutable, repo, &selected, *placement, message).await?;
            // User branch heads advance above the selected tip; retention refs follow native rebases only.
            if matches!(placement, Placement::Above) {
                advance_heads(mutable, repo, config, &selected, &new);
            }
        }
        _ => return Err(Error::Invalid),
    }
    Ok(())
}

pub(super) fn advance_heads(
    mutable: &mut MutableRepo,
    repo: &ReadonlyRepo,
    config: &Config,
    selected: &Commit,
    new: &Commit,
) {
    let heads = repo
        .view()
        .local_bookmarks()
        .filter(|(name, target)| {
            !name.as_str().starts_with(config::INTERNAL_PREFIX)
                && !config
                    .workspaces
                    .iter()
                    .any(|saved| saved.target == name.as_str())
                && target.as_normal() == Some(selected.id())
        })
        .map(|(name, _)| name.to_owned())
        .collect::<Vec<_>>();
    for name in heads {
        mutable.set_local_bookmark_target(&name, RefTarget::normal(new.id().clone()));
    }
}
