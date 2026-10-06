//! Native insertion and squash; merge/WIP propagate through the JJ rewrite graph.
use super::*;
use jj_lib::{
    merged_tree_builder::MergedTreeBuilder,
    repo::{MutableRepo, Repo},
    repo_path::RepoPathBuf,
};

pub(super) fn message(value: &str) -> Result<()> {
    if value.trim().is_empty() || value.len() > 65536 || value.contains('\0') {
        return Err(Error::Invalid);
    }
    Ok(())
}
pub(super) async fn selected(
    repo: &dyn Repo,
    wc: &Commit,
    selections: &[Selection],
) -> Result<rewrite::CommitWithSelection> {
    if selections.is_empty() || selections.len() > 256 {
        return Err(Error::Invalid);
    }
    let parent_tree = wc.parent_tree(repo).await.map_err(jj)?;
    let mut builder = MergedTreeBuilder::new(parent_tree.clone());
    let mut paths = std::collections::BTreeSet::new();
    for selection in selections {
        let path = &selection.path;
        if selection.source != wc.id().hex() || !paths.insert(path) {
            return Err(Error::Invalid);
        }
        let path = self::path(path)?;
        let before = parent_tree.path_value(&path).await.map_err(jj)?;
        let after = wc.tree().path_value(&path).await.map_err(jj)?;
        if before == after || !before.is_resolved() || !after.is_resolved() {
            return Err(Error::Invalid);
        }
        let value = if let Some(hunks) = &selection.hunks {
            if hunks.len()
                != hunks
                    .iter()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len()
            {
                return Err(Error::Invalid);
            }
            hunks::selected_value(repo, wc, &path, &before, &after, hunks).await?
        } else {
            after
        };
        builder.set_or_remove(path, value);
    }
    Ok(rewrite::CommitWithSelection {
        commit: wc.clone(),
        parent_tree,
        selected_tree: builder.write_tree().await.map_err(jj)?,
    })
}
pub(super) async fn validate(
    repo: &ReadonlyRepo,
    workspace_name: &jj_lib::ref_name::WorkspaceName,
    settings: &jj_lib::settings::UserSettings,
    config: &Config,
    wc: &Commit,
    branch: &str,
    text: &str,
    selections: &[Selection],
) -> Result<Commit> {
    message(text)?;
    let saved = config.active().ok_or(Error::Invalid)?;
    if !topology::contains(repo, saved, branch).await? || branch.contains('@') {
        return Err(Error::Invalid);
    }
    let head = resolve(repo, branch)?;
    if !repo
        .index()
        .is_ancestor(head.id(), wc.id())
        .await
        .map_err(jj)?
    {
        return Err(Error::Invalid);
    }
    let target = resolve(repo, &saved.target)?;
    let affected = if head.id() == target.id() {
        // An empty bookmark at the target starts a fork, not an insertion into every target child.
        config
            .workspaces
            .iter()
            .filter(|saved| saved.applied.iter().any(|name| name == branch))
            .map(|saved| resolve_any(repo, &saved.merge_ref).map(|merge| merge.id().clone()))
            .collect::<Result<Vec<_>>>()?
    } else {
        basic::children(repo, &head).await?
    };
    policy::rewritable(repo, workspace_name, settings, config, affected)?;
    if !selections.is_empty() {
        content::selections(repo, workspace_name, settings, config, selections).await?;
    }
    Ok(head)
}
#[allow(clippy::too_many_arguments)]
pub(super) async fn commit(
    mut_repo: &mut MutableRepo,
    repo: &ReadonlyRepo,
    workspace_name: &jj_lib::ref_name::WorkspaceName,
    settings: &jj_lib::settings::UserSettings,
    config: &Config,
    wc: &Commit,
    branch: &str,
    text: &str,
    selections: &[Selection],
) -> Result<()> {
    let head = validate(
        repo,
        workspace_name,
        settings,
        config,
        wc,
        branch,
        text,
        selections,
    )
    .await?;
    let new = mut_repo
        .new_commit(vec![head.id().clone()], head.tree())
        .set_description(text)
        .write()
        .await
        .map_err(jj)?;
    mut_repo.set_local_bookmark_target(branch.as_ref(), RefTarget::normal(new.id().clone()));
    let target = resolve(repo, &config.active().ok_or(Error::Invalid)?.target)?;
    if head.id() == target.id() {
        for saved in config
            .workspaces
            .iter()
            .filter(|saved| saved.applied.iter().any(|name| name == branch))
        {
            let merge = resolve_any(repo, &saved.merge_ref)?;
            let parents = parents(mut_repo, saved)?
                .iter()
                .map(|commit| commit.id().clone())
                .collect();
            rewrite::rebase_commit(mut_repo, merge, parents)
                .await
                .map_err(jj)?;
        }
    } else {
        mut_repo
            .transform_descendants(basic::children(repo, &head).await?, async |mut rewriter| {
                rewriter.replace_parent(head.id(), [new.id()]);
                rewriter.rebase().await?.write().await?;
                Ok(())
            })
            .await
            .map_err(jj)?;
    }
    if selections.is_empty() {
        mut_repo.set_local_bookmark_target(branch.as_ref(), RefTarget::normal(new.id().clone()));
        return Ok(());
    }
    mut_repo.rebase_descendants().await.map_err(jj)?;
    let sources = content::remap(mut_repo, repo, selections).await?;
    let new = content::squash(mut_repo, &sources, &new, text).await?;
    mut_repo.set_local_bookmark_target(branch.as_ref(), RefTarget::normal(new.id().clone()));
    Ok(())
}

pub(super) fn path(value: &str) -> Result<RepoPathBuf> {
    if value.is_empty()
        || value.len() > 4096
        || value.starts_with('/')
        || value.contains('\\')
        || value
            .split('/')
            .any(|p| p.is_empty() || p == "." || p == ".." || p == ".jj" || p == ".git")
    {
        return Err(Error::Invalid);
    }
    RepoPathBuf::from_internal_string(value).map_err(jj)
}
