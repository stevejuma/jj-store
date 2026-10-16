//! Native graph rewrites, with retention changes left to JJ descendant propagation.
use super::*;
use jj_lib::{
    repo::MutableRepo,
    rewrite::{MoveCommitsLocation, MoveCommitsTarget, RebaseOptions},
};

fn user_commit(repo: &ReadonlyRepo, config: &Config, id: &str) -> Result<Commit> {
    let commit = policy::commit(repo, id)?;
    if commit.parent_ids().len() != 1
        || config.workspaces.iter().any(|w| {
            [w.merge_ref.as_str(), w.wip_ref.as_str()].iter().any(|r| {
                repo.view().get_local_bookmark((*r).as_ref()).as_normal() == Some(commit.id())
            })
        })
    {
        return Err(Error::Invalid);
    }
    Ok(commit)
}
/// Stop at the next bookmark boundary. Never enumerate repository history.
pub(super) async fn segment(
    repo: &ReadonlyRepo,
    config: &Config,
    branch: &str,
) -> Result<Vec<Commit>> {
    if branch.contains('@') || config.workspaces.iter().any(|w| w.target == branch) {
        return Err(Error::Invalid);
    }
    let topology = topology::read(repo, config.active().ok_or(Error::Invalid)?).await?;
    if !topology.complete {
        return Err(Error::Invalid);
    }
    let boundary = topology
        .inventory
        .iter()
        .chain(&topology.lanes)
        .flat_map(|lane| &lane.heads)
        .find(|head| head.names.iter().any(|name| name == branch))
        .and_then(|head| head.base_commit.as_ref())
        .ok_or(Error::Invalid)?;
    let mut commit = resolve(repo, branch)?;
    let mut result = vec![];
    loop {
        if commit.id().hex() == *boundary {
            break;
        }
        if commit.parent_ids().len() != 1 || result.len() >= 500 {
            return Err(Error::Invalid);
        }
        if !result.is_empty()
            && repo.view().local_bookmarks().any(|(name, r)| {
                !name.as_str().starts_with(config::INTERNAL_PREFIX)
                    && r.as_normal() == Some(commit.id())
            })
        {
            break;
        }
        result.push(commit.clone());
        commit = repo
            .store()
            .get_commit(&commit.parent_ids()[0])
            .map_err(jj)?;
    }
    if result.is_empty() {
        return Err(Error::Invalid);
    }
    Ok(result)
}
async fn location(
    repo: &ReadonlyRepo,
    config: &Config,
    command: &Command,
) -> Result<MoveCommitsLocation> {
    match command {
        Command::Move {
            commit,
            target,
            placement,
        } => {
            let source = user_commit(repo, config, commit)?;
            let target = user_commit(repo, config, target)?;
            if source.id() == target.id() {
                return Err(Error::Invalid);
            }
            let (parents, children) = if matches!(placement, Placement::Above) {
                (
                    vec![target.id().clone()],
                    basic::children(repo, &target).await?,
                )
            } else {
                (target.parent_ids().to_vec(), vec![target.id().clone()])
            };
            Ok(MoveCommitsLocation {
                new_parent_ids: parents,
                new_child_ids: children,
                target: MoveCommitsTarget::Commits(vec![source.id().clone()]),
            })
        }
        Command::ForgeRestack {
            branch,
            target_sha,
            expected_base,
            ..
        } => {
            let commits = segment(repo, config, branch).await?;
            let oldest = commits.last().ok_or(Error::Invalid)?;
            if oldest.parent_ids()[0].hex() != *expected_base {
                return Err(Error::Stale);
            }
            let id = jj_lib::backend::CommitId::try_from_hex(target_sha).ok_or(Error::Invalid)?;
            let target = repo.store().get_commit(&id).map_err(jj)?;
            Ok(MoveCommitsLocation {
                new_parent_ids: vec![target.id().clone()],
                new_child_ids: vec![],
                target: MoveCommitsTarget::Roots(vec![oldest.id().clone()]),
            })
        }
        Command::Rebase { branch } => {
            let commits = segment(repo, config, branch).await?;
            let target = resolve(repo, &config.active().ok_or(Error::Invalid)?.target)?;
            Ok(MoveCommitsLocation {
                new_parent_ids: vec![target.id().clone()],
                new_child_ids: vec![],
                target: MoveCommitsTarget::Roots(vec![
                    commits.last().ok_or(Error::Invalid)?.id().clone(),
                ]),
            })
        }
        _ => Err(Error::Invalid),
    }
}
async fn squash_pair(
    repo: &ReadonlyRepo,
    config: &Config,
    workspace: &jj_lib::ref_name::WorkspaceName,
    command: &Command,
) -> Result<(Vec<Commit>, Commit)> {
    match command {
        Command::Squash { commit, target } => Ok((
            vec![user_commit(repo, config, commit)?],
            user_commit(repo, config, target)?,
        )),
        Command::Uncommit { commit } => Ok((
            vec![user_commit(repo, config, commit)?],
            repo.store()
                .get_commit(
                    repo.view()
                        .get_wc_commit_id(workspace)
                        .ok_or(Error::Invalid)?,
                )
                .map_err(jj)?,
        )),
        Command::SquashBranch { branch } => {
            let mut commits = segment(repo, config, branch).await?;
            let target = commits.pop().ok_or(Error::Invalid)?;
            if commits.is_empty() {
                return Err(Error::Invalid);
            }
            Ok((commits, target))
        }
        _ => Err(Error::Invalid),
    }
}
pub(super) async fn validate(
    repo: &ReadonlyRepo,
    workspace: &jj_lib::ref_name::WorkspaceName,
    settings: &jj_lib::settings::UserSettings,
    config: &Config,
    command: &Command,
) -> Result<Preview> {
    config.active().ok_or(Error::Invalid)?;
    if matches!(
        command,
        Command::Move { .. } | Command::Rebase { .. } | Command::ForgeRestack { .. }
    ) {
        let loc = location(repo, config, command).await?;
        let ids = match &loc.target {
            MoveCommitsTarget::Commits(ids) | MoveCommitsTarget::Roots(ids) => ids,
        };
        let mut affected = ids.clone();
        affected.extend(loc.new_child_ids.clone());
        policy::rewritable(repo, workspace, settings, config, affected)?;
    } else {
        let (sources, target) = squash_pair(repo, config, workspace, command).await?;
        if sources.iter().any(|c| c.id() == target.id()) {
            return Err(Error::Invalid);
        }
        let mut affected = sources.iter().map(|c| c.id().clone()).collect::<Vec<_>>();
        affected.push(target.id().clone());
        policy::rewritable(repo, workspace, settings, config, affected)?;
    }
    Ok(Preview{title:"Rewrite commits".into(),summary:"Use native JJ move/squash semantics. Descendants and workspace WIP follow the rewrite; conflicts remain native and the megamerge is not reconstructed.".into(),paths:vec![]})
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
    if let Command::ForgeRestack { target_sha, .. } = command {
        let id = jj_lib::backend::CommitId::try_from_hex(target_sha).ok_or(Error::Invalid)?;
        let target = repo.store().get_commit(&id).map_err(jj)?;
        mutable.add_head(&target).await.map_err(jj)?;
    }
    if matches!(
        command,
        Command::Move { .. } | Command::Rebase { .. } | Command::ForgeRestack { .. }
    ) {
        rewrite::move_commits(
            mutable,
            &location(repo, config, command).await?,
            &RebaseOptions::default(),
        )
        .await
        .map_err(jj)?;
    } else {
        let (sources, target) = squash_pair(repo, config, workspace, command).await?;
        let mut selections = vec![];
        for commit in sources {
            selections.push(rewrite::CommitWithSelection {
                parent_tree: commit.parent_tree(repo).await.map_err(jj)?,
                selected_tree: commit.tree(),
                commit,
            });
        }
        rewrite::squash_commits(mutable, &selections, &target, false)
            .await
            .map_err(jj)?
            .ok_or(Error::Invalid)?
            .commit_builder
            .set_description(target.description())
            .write()
            .await
            .map_err(jj)?;
    }
    // Single-revision moves deliberately detach ordinary descendants. Keep
    // saved applied edges on the moved tips by rebasing the existing retained
    // merge change only when that native move detached one; never create a merge.
    if matches!(command, Command::Move { .. }) {
        mutable.rebase_descendants().await.map_err(jj)?;
        for saved in &config.workspaces {
            let merge = resolve_any(mutable, &saved.merge_ref)?;
            let desired = parents(mutable, saved)?
                .iter()
                .map(|c| c.id().clone())
                .collect::<Vec<_>>();
            if merge.parent_ids() != desired {
                rewrite::rebase_commit(mutable, merge, desired)
                    .await
                    .map_err(jj)?;
                mutable.rebase_descendants().await.map_err(jj)?;
            }
        }
    }
    Ok(())
}
