//! Repository text editing as one new child change and bookmark movement.
use super::*;
use jj_lib::{
    backend as b,
    merge::Merge,
    merged_tree_builder::MergedTreeBuilder,
    object_id::ObjectId as _,
    repo::{ReadonlyRepo, Repo as _},
};

fn unavailable(_: impl std::fmt::Debug) -> Error {
    query::Error::Unavailable.into()
}

pub(super) struct Staged {
    pub repo: Arc<ReadonlyRepo>,
    pub created: m::CommitId,
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn stage(
    repo: &Arc<ReadonlyRepo>,
    access: &dyn ReadAccess,
    bookmark: String,
    expected: crate::native_dto::RefTarget,
    source: String,
    destination: String,
    contents: String,
    description: String,
    principal: m::PrincipalId,
    budget: &Budget,
) -> Result<Staged> {
    let source = super::resolve::path(&source)?;
    let destination = super::resolve::path(&destination)?;
    let target = expected
        .0
        .first()
        .and_then(|target| *target)
        .ok_or(Error::Invalid)?;
    if expected.0.len() != 1 || target == crate::roots::ROOT_COMMIT {
        return Err(Error::Invalid);
    }
    let current = repo.view().get_local_bookmark(bookmark.as_str().as_ref());
    if super::execute::reference(current) != expected {
        return Err(Error::StaleBookmark);
    }
    let old = super::execute::commit(repo, access, target, true, budget).await?;
    let tree = old.tree();
    let value = tree.path_value(&source).await.map_err(unavailable)?;
    let Some(Some(b::TreeValue::File {
        executable,
        copy_id,
        ..
    })) = value.as_resolved()
    else {
        return Err(Error::Invalid);
    };
    if source != destination {
        let destination_value = tree.path_value(&destination).await.map_err(unavailable)?;
        if !matches!(destination_value.as_resolved(), Some(None)) {
            return Err(Error::Invalid);
        }
    }
    budget.authorize(access).await?;
    let mut input = futures::io::Cursor::new(contents.into_bytes());
    let file = repo
        .store()
        .write_file(&destination, &mut input)
        .await
        .map_err(unavailable)?;
    let replacement = b::TreeValue::File {
        id: file,
        executable: *executable,
        copy_id: copy_id.clone(),
    };
    let mut tree_builder = MergedTreeBuilder::new(tree);
    if source != destination {
        tree_builder.set_or_remove(source, Merge::resolved(None));
    }
    tree_builder.set_or_remove(destination, Merge::resolved(Some(replacement)));
    let edited = tree_builder.write_tree().await.map_err(unavailable)?;
    budget.authorize(access).await?;
    let mut tx = repo.start_transaction();
    tx.set_attribute("jj-store.principal".into(), principal.to_string());
    let new = tx
        .repo_mut()
        .new_commit(vec![old.id().clone()], edited)
        .set_description(description)
        .write()
        .await
        .map_err(unavailable)?;
    tx.repo_mut().set_local_bookmark_target(
        bookmark.as_str().as_ref(),
        jj_lib::op_store::RefTarget::normal(new.id().clone()),
    );
    budget.check()?;
    let created = m::CommitId::from_slice(new.id().as_bytes()).map_err(unavailable)?;
    let repo = tx
        .write("jj-store: edit file")
        .await
        .map_err(unavailable)?
        .leave_unpublished();
    Ok(Staged { repo, created })
}
