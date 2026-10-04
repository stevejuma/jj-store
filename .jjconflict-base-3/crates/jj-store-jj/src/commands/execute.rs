use super::*;
use crate::{
    backend::BackendLimits, factories::NativeStores, objects::NativeObjects,
    operation_stores::OperationLimits, query::access::ReadOnlyAuthority, roots,
};
use jj_lib::{
    backend as b,
    config::{ConfigLayer, ConfigSource, StackedConfig},
    object_id::ObjectId as _,
    op_store as o,
    repo::{ReadonlyRepo, Repo as _},
    settings::UserSettings,
};
use std::collections::BTreeSet;

fn unavailable(_: impl std::fmt::Debug) -> Error {
    query::Error::Unavailable.into()
}
fn cid(id: &b::CommitId) -> m::CommitId {
    m::CommitId::from_slice(id.as_bytes()).expect("native commit width")
}
pub(super) fn reference(value: &o::RefTarget) -> crate::native_dto::RefTarget {
    crate::native_dto::RefTarget(
        value
            .as_merge()
            .iter()
            .map(|id| id.as_ref().map(cid))
            .collect(),
    )
}
fn name(value: &str, limit: usize) -> bool {
    !value.is_empty() && value.len() <= limit && !value.chars().any(char::is_control)
}
fn target(value: &crate::native_dto::RefTarget) -> bool {
    !value.0.is_empty() && value.0.len() <= 65 && value.0.len() % 2 == 1
}
pub(super) fn validate(command: &Command) -> Result<()> {
    let valid = match command {
        Command::New {
            parents,
            description,
            workspace,
        } => {
            !parents.is_empty()
                && parents.len() <= 64
                && parents.iter().collect::<BTreeSet<_>>().len() == parents.len()
                && description.len() <= 65536
                && workspace.as_ref().is_none_or(|value| name(value, 256))
        }
        Command::Describe { description, .. } => description.len() <= 65536,
        Command::EditFile {
            bookmark,
            expected,
            source,
            destination,
            contents,
            description,
        } => {
            name(bookmark, 1024)
                && expected.0.len() == 1
                && expected.0[0].is_some()
                && super::resolve::path(source).is_ok()
                && super::resolve::path(destination).is_ok()
                && contents.len() <= 256 * 1024
                && description.len() <= 65536
        }
        Command::ReconcileOperations { .. } => true,
        Command::ResolveConflicts { paths, .. } => super::resolve::validate(paths).is_ok(),
        Command::Abandon { commits, .. } => {
            !commits.is_empty()
                && commits.len() <= 256
                && commits.iter().collect::<BTreeSet<_>>().len() == commits.len()
        }
        Command::BookmarkCreate { name: value, .. } => name(value, 1024),
        Command::BookmarkMove {
            name: value,
            expected,
            ..
        }
        | Command::BookmarkDelete {
            name: value,
            expected,
        } => name(value, 1024) && target(expected),
    };
    if valid { Ok(()) } else { Err(Error::Invalid) }
}
pub(super) async fn commit(
    repo: &ReadonlyRepo,
    access: &dyn ReadAccess,
    id: m::CommitId,
    visible: bool,
    budget: &Budget,
) -> Result<jj_lib::commit::Commit> {
    budget.check()?;
    if !access.contains(m::ObjectKey::Commit(id)).await? {
        return Err(query::Error::NotCommitted.into());
    }
    let id = b::CommitId::from_bytes(id.as_bytes());
    if visible {
        let mut found = false;
        for head in repo.view().heads() {
            budget.work(1)?;
            if repo
                .index()
                .is_ancestor(&id, head)
                .await
                .map_err(unavailable)?
            {
                found = true;
                break;
            }
        }
        if !found {
            return Err(Error::Hidden);
        }
    }
    repo.store()
        .get_commit_async(&id)
        .await
        .map_err(unavailable)
}
fn settings(principal: m::PrincipalId) -> Result<UserSettings> {
    let mut config = StackedConfig::with_defaults();
    let mut layer = ConfigLayer::empty(ConfigSource::CommandArg);
    for (key, value) in [
        ("user.name", principal.to_string()),
        ("user.email", format!("{principal}@jj-store.invalid")),
        ("operation.username", principal.to_string()),
        ("operation.hostname", "jj-store".into()),
        ("signing.behavior", "drop".into()),
    ] {
        layer.set_value(key, value).map_err(unavailable)?;
    }
    config.add_layer(layer);
    UserSettings::from_config(config).map_err(unavailable)
}
pub(super) async fn run(
    store: Arc<dyn ObjectStore>,
    mutation: Arc<dyn MutationAccess>,
    command: Command,
    limits: Limits,
    budget: Arc<Budget>,
) -> Result<Prepared> {
    let uploaded_files = mutation.uploaded_files().clone();
    let principal = mutation.principal();
    let heads = mutation.heads().clone();
    let access: Arc<dyn ReadAccess> = Arc::new(super::access::Access(mutation));
    if access
        .inventory()
        .await?
        .iter()
        .filter(|key| matches!(key, m::ObjectKey::Commit(_)))
        .count()
        > limits.query.max_index_commits
    {
        return Err(query::Error::IndexBudget.into());
    }
    let store = Arc::new(super::access::Store::new(
        store,
        access.clone(),
        budget.clone(),
        limits,
    ));
    let objects = Arc::new(
        NativeObjects::new(
            store.clone(),
            access.repository(),
            limits.query.max_object_bytes,
        )
        .map_err(unavailable)?,
    );
    let authority = Arc::new(super::access::Authority(ReadOnlyAuthority {
        access: access.clone(),
        budget: budget.clone(),
    }));
    let stores = NativeStores::new(
        objects,
        authority,
        BackendLimits::default(),
        OperationLimits::default(),
    )
    .map_err(unavailable)?;
    let settings = settings(principal)?;
    let cache = tempfile::tempdir().map_err(unavailable)?;
    let loader = stores
        .initialize_cache(&settings, cache.path())
        .await
        .map_err(unavailable)?;
    let repo = stores
        .load_at(&loader, access.operation())
        .await
        .map_err(unavailable)?;
    budget.check()?;
    let resolution = match &command {
        Command::ResolveConflicts { commit, paths } => Some((
            *commit,
            paths.iter().map(|p| p.path.clone()).collect::<Vec<_>>(),
        )),
        _ => None,
    };
    let mut abandoned = vec![];
    let mut skipped_hidden = vec![];
    let (staged, mut created) = match command {
        Command::EditFile {
            bookmark,
            expected,
            source,
            destination,
            contents,
            description,
        } => {
            let result = super::edit::stage(
                &repo,
                access.as_ref(),
                bookmark,
                expected,
                source,
                destination,
                contents,
                description,
                principal,
                &budget,
            )
            .await?;
            (result.repo, vec![result.created])
        }
        Command::ResolveConflicts { commit, paths } => (
            super::resolve::stage(
                &repo,
                access.as_ref(),
                commit,
                paths,
                principal,
                &budget,
                &uploaded_files,
            )
            .await?,
            vec![],
        ),
        Command::Abandon {
            commits,
            retain_bookmarks,
            restore_descendants,
        } => {
            let result = super::abandon::stage(
                &repo,
                access.as_ref(),
                commits,
                retain_bookmarks,
                restore_descendants,
                principal,
                &budget,
            )
            .await?;
            abandoned = result.abandoned;
            skipped_hidden = result.skipped_hidden;
            (result.repo, vec![])
        }
        Command::ReconcileOperations { base } => (
            super::reconcile::merge(&repo, &heads, base, principal, &budget).await?,
            vec![],
        ),
        command => transact(&repo, access.as_ref(), command, principal, &budget).await?,
    };
    let mut rewrites = vec![];
    for (id, predecessors) in staged
        .operation()
        .store_operation()
        .commit_predecessors
        .as_ref()
        .ok_or(query::Error::Corrupt)?
    {
        budget.work(1)?;
        if predecessors.is_empty() {
            let id = cid(id);
            if !created.contains(&id) {
                created.push(id);
            }
            continue;
        }
        let commit = staged
            .store()
            .get_commit_async(id)
            .await
            .map_err(unavailable)?;
        rewrites.push(Rewrite {
            commit: cid(id),
            change: m::ChangeId::from_slice(commit.change_id().as_bytes()).map_err(unavailable)?,
            predecessors: predecessors.iter().map(cid).collect(),
        });
    }
    let mut bookmarks = BTreeMap::new();
    let names: BTreeSet<_> = repo
        .view()
        .local_bookmarks()
        .chain(staged.view().local_bookmarks())
        .map(|(name, _)| name.to_owned())
        .collect();
    for name in names {
        budget.work(1)?;
        let before = reference(repo.view().get_local_bookmark(&name));
        let after = reference(staged.view().get_local_bookmark(&name));
        if before != after {
            bookmarks.insert(name.as_str().to_owned(), Change { before, after });
        }
    }
    let mut workspaces = BTreeMap::new();
    let names: BTreeSet<_> = repo
        .view()
        .wc_commit_ids()
        .keys()
        .chain(staged.view().wc_commit_ids().keys())
        .cloned()
        .collect();
    for name in names {
        budget.work(1)?;
        let before = repo.view().get_wc_commit_id(&name).map(cid);
        let after = staged.view().get_wc_commit_id(&name).map(cid);
        if before != after {
            workspaces.insert(name.as_str().to_owned(), Change { before, after });
        }
    }
    let conflict_resolution = match resolution {
        Some((before, paths)) => Some(
            super::report::resolution(
                &staged,
                access.as_ref(),
                before,
                paths,
                &rewrites,
                &created,
                &budget,
            )
            .await?,
        ),
        None => None,
    };
    let (staged_objects, staged_bytes) = store.stats();
    let result = Prepared {
        version: 1,
        operation: m::OperationId::from_slice(staged.op_id().as_bytes()).map_err(unavailable)?,
        created,
        rewrites,
        bookmarks,
        workspaces,
        stats: budget.stats(),
        staged_objects,
        staged_bytes,
        abandoned,
        skipped_hidden,
        conflict_resolution,
    };
    budget.authorize(access.as_ref()).await?;
    budget.complete_output(&result)?;
    Ok(result)
}

async fn transact(
    repo: &Arc<ReadonlyRepo>,
    access: &dyn ReadAccess,
    command: Command,
    principal: m::PrincipalId,
    budget: &Budget,
) -> Result<(Arc<ReadonlyRepo>, Vec<m::CommitId>)> {
    let mut tx = repo.start_transaction();
    tx.set_attribute("jj-store.principal".into(), principal.to_string());
    let mut created = vec![];
    let description = match command {
        Command::ReconcileOperations { .. }
        | Command::Abandon { .. }
        | Command::ResolveConflicts { .. }
        | Command::EditFile { .. } => {
            return Err(Error::Invalid);
        }
        Command::New {
            parents,
            description,
            workspace,
        } => {
            let mut parent_commits = Vec::with_capacity(parents.len());
            for id in parents {
                parent_commits.push(commit(repo, access, id, false, budget).await?);
            }
            if let Some(name) = &workspace
                && repo
                    .view()
                    .get_wc_commit_id(name.as_str().as_ref())
                    .is_none()
            {
                return Err(Error::Invalid);
            }
            let tree = jj_lib::rewrite::merge_commit_trees(tx.repo(), &parent_commits)
                .await
                .map_err(unavailable)?;
            let new = tx
                .repo_mut()
                .new_commit(
                    parent_commits.iter().map(|c| c.id().clone()).collect(),
                    tree,
                )
                .set_description(description)
                .write()
                .await
                .map_err(unavailable)?;
            created.push(cid(new.id()));
            if let Some(name) = workspace {
                tx.set_workspace_name(name.as_str().as_ref());
                tx.repo_mut()
                    .edit(name.into(), &new)
                    .await
                    .map_err(unavailable)?;
            }
            "new change"
        }
        Command::Describe {
            commit: id,
            description,
        } => {
            if id == roots::ROOT_COMMIT {
                return Err(Error::Root);
            }
            let old = commit(repo, access, id, true, budget).await?;
            if old.description() != description {
                tx.repo_mut()
                    .transform_descendants(vec![old.id().clone()], async |rewriter| {
                        let target = rewriter.old_commit().id() == old.id();
                        let builder = rewriter.reparent();
                        if target {
                            builder.set_description(&description).write().await?;
                        } else {
                            builder.write().await?;
                        }
                        Ok(())
                    })
                    .await
                    .map_err(unavailable)?;
            }
            "describe change"
        }
        Command::BookmarkCreate { name, target } => {
            let old = repo.view().get_local_bookmark(name.as_str().as_ref());
            if old.is_present() {
                return Err(Error::StaleBookmark);
            }
            if repo.view().store_view().remote_views.values().any(|view| {
                view.bookmarks
                    .get(jj_lib::ref_name::RefName::new(&name))
                    .is_some_and(|remote| remote.state == o::RemoteRefState::Tracked)
            }) {
                return Err(Error::Invalid);
            }
            let target = commit(repo, access, target, false, budget).await?;
            tx.repo_mut().set_local_bookmark_target(
                name.as_str().as_ref(),
                o::RefTarget::normal(target.id().clone()),
            );
            "create bookmark"
        }
        Command::BookmarkMove {
            name,
            expected,
            target,
        } => {
            let old = repo.view().get_local_bookmark(name.as_str().as_ref());
            if old.is_absent() || reference(old) != expected {
                return Err(Error::StaleBookmark);
            }
            let target = commit(repo, access, target, false, budget).await?;
            tx.repo_mut().set_local_bookmark_target(
                name.as_str().as_ref(),
                o::RefTarget::normal(target.id().clone()),
            );
            "move bookmark"
        }
        Command::BookmarkDelete { name, expected } => {
            let old = repo.view().get_local_bookmark(name.as_str().as_ref());
            if old.is_absent() || reference(old) != expected {
                return Err(Error::StaleBookmark);
            }
            tx.repo_mut()
                .set_local_bookmark_target(name.as_str().as_ref(), o::RefTarget::absent());
            "delete bookmark"
        }
    };
    tx.repo_mut()
        .rebase_descendants()
        .await
        .map_err(unavailable)?;
    budget.check()?;
    let staged = tx
        .write(format!("jj-store: {description}"))
        .await
        .map_err(unavailable)?
        .leave_unpublished();
    Ok((staged, created))
}
