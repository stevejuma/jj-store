//! Atomic conflict validation and native tree/descendant rewriting.
use super::*;
use crate::{query::content::conflicts, roots};
use jj_lib::{
    backend as b,
    matchers::FilesMatcher,
    merge::Merge,
    merged_tree_builder::MergedTreeBuilder,
    object_id::ObjectId as _,
    repo::{ReadonlyRepo, Repo as _},
    repo_path::RepoPathBuf,
};
use std::collections::BTreeSet;

fn unavailable(_: impl std::fmt::Debug) -> Error {
    query::Error::Unavailable.into()
}
fn path(value: &str) -> Result<RepoPathBuf> {
    if value.is_empty()
        || value.len() > 4096
        || value.contains('\0')
        || value.split('/').any(|part| matches!(part, "." | ".."))
        || value.split('/').count() > 64
    {
        return Err(Error::Invalid);
    }
    RepoPathBuf::from_internal_string(value).map_err(|_| Error::Invalid)
}
pub(super) fn validate(paths: &[ConflictResolution]) -> Result<()> {
    if paths.is_empty() || paths.len() > 256 {
        return Err(Error::Invalid);
    }
    let mut seen = BTreeSet::new();
    for resolution in paths {
        path(&resolution.path)?;
        if !seen.insert(resolution.path.as_str()) {
            return Err(Error::Invalid);
        }
    }
    // Reject ancestor/descendant overlap independent of caller ordering. Looking
    // only at adjacent sorted strings misses a, a-sibling, a/child.
    for name in &seen {
        for (index, _) in name.match_indices('/') {
            if seen.contains(&name[..index]) {
                return Err(Error::Invalid);
            }
        }
    }
    Ok(())
}
pub(super) async fn stage(
    repo: &Arc<ReadonlyRepo>,
    access: &dyn ReadAccess,
    id: m::CommitId,
    paths: Vec<ConflictResolution>,
    principal: m::PrincipalId,
    budget: &Budget,
    uploaded_files: &BTreeMap<m::ConflictUploadId, m::FileId>,
) -> Result<Arc<ReadonlyRepo>> {
    if id == roots::ROOT_COMMIT {
        return Err(Error::Root);
    }
    validate(&paths)?;
    let old = super::execute::commit(repo, access, id, true, budget).await?;
    let change = m::ChangeId::from_slice(old.change_id().as_bytes()).map_err(unavailable)?;
    let tree = old.tree();
    let requested = paths
        .iter()
        .map(|r| Ok((path(&r.path)?, r)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    let matcher = FilesMatcher::new(requested.keys().map(|p| p.as_ref()));
    let mut checked = BTreeMap::new();
    for (path, value) in tree.conflicts_matching(&matcher) {
        budget.authorize(access).await?;
        let Some(resolution) = requested.get(&path) else {
            continue;
        };
        let value = value.map_err(unavailable)?;
        let terms = conflicts::native_terms(&tree, &value)?;
        budget.work(terms.len())?;
        let fingerprint = conflicts::fingerprint(
            access.repository(),
            access.epoch(),
            id,
            change,
            path.as_internal_file_string(),
            &terms,
        )?;
        if fingerprint != resolution.fingerprint {
            return Err(Error::StaleConflict);
        }
        let replacement = match resolution.resolution {
            Resolution::UploadedFile {
                upload,
                executable,
                copy_from_term,
            } => {
                let file = uploaded_files.get(&upload).ok_or(Error::InvalidUpload)?;
                let copy_id = match copy_from_term {
                    None => b::CopyId::placeholder(),
                    Some(term) => match value.iter().nth(term as usize) {
                        Some(Some(b::TreeValue::File { copy_id, .. })) => copy_id.clone(),
                        _ => return Err(Error::Invalid),
                    },
                };
                // The service admits only scoped upload files into this command's
                // read inventory. Read the complete verified native content before
                // any tree write; storage framing and native ID are both checked.
                let mut input = repo
                    .store()
                    .read_file(&path, &b::FileId::from_bytes(file.as_bytes()))
                    .await
                    .map_err(unavailable)?;
                use futures::AsyncReadExt as _;
                let mut buffer = vec![0; 64 * 1024];
                loop {
                    budget.check()?;
                    if input.read(&mut buffer).await.map_err(unavailable)? == 0 {
                        break;
                    }
                }
                Some(b::TreeValue::File {
                    id: b::FileId::from_bytes(file.as_bytes()),
                    executable,
                    copy_id,
                })
            }
            Resolution::Delete => None,
            Resolution::SelectTerm { term } => value
                .iter()
                .nth(term as usize)
                .ok_or(Error::Invalid)?
                .clone(),
        };
        checked.insert(path, replacement);
    }
    if checked.len() != requested.len() {
        return Err(Error::StaleConflict);
    }
    // No write occurs until the complete set of paths has been checked against
    // one original tree. All writes remain uncommitted until the service CAS.
    budget.authorize(access).await?;
    let mut builder = MergedTreeBuilder::new(tree);
    for (path, value) in checked {
        builder.set_or_remove(path, Merge::resolved(value));
    }
    let resolved = builder.write_tree().await.map_err(unavailable)?;
    let mut tx = repo.start_transaction();
    tx.set_attribute("jj-store.principal".into(), principal.to_string());
    tx.repo_mut()
        .rewrite_commit(&old)
        .set_tree(resolved)
        .write()
        .await
        .map_err(unavailable)?;
    tx.repo_mut()
        .rebase_descendants()
        .await
        .map_err(unavailable)?;
    budget.check()?;
    Ok(tx
        .write("jj-store: resolve conflicts")
        .await
        .map_err(unavailable)?
        .leave_unpublished())
}
