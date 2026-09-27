use super::*;
use crate::{
    backend::{BackendLimits, NativeBackend},
    query::access::ReadOnlyAuthority,
};
use jj_lib::{
    backend::MergedTreeValueExt as _,
    config::StackedConfig,
    repo_path::{RepoPathBuf, RepoPathComponentBuf},
    settings::UserSettings,
    signing::Signer,
    store::Store,
    tree::TreeMergeExt as _,
    tree_merge::MergeOptions,
};
use serde::Deserialize;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TreeRequest {
    pub commit: m::CommitId,
    /// Empty means root; all other paths use canonical native components.
    pub path: String,
    pub limit: usize,
    /// Internal traversal continuation, not an HTTP cursor.
    pub after: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TreeEntry {
    pub name: String,
    /// Native alternating add/remove terms. A sole None means absence and is
    /// excluded from directory listings; unresolved deletions remain explicit.
    pub terms: Vec<Option<dto::TreeValue>>,
    pub conflicted: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TreePage {
    pub version: u32,
    pub commit: m::CommitId,
    pub path: String,
    pub entries: Vec<TreeEntry>,
    pub next: Option<String>,
    pub stats: Stats,
}
pub(super) async fn read(context: Context, request: TreeRequest) -> Result<TreePage> {
    context.member(m::ObjectKey::Commit(request.commit)).await?;
    if request.limit == 0 || request.limit > context.limits.max_page || request.path.len() > 4096 {
        return Err(Error::Limit);
    }
    let path =
        RepoPathBuf::from_internal_string(request.path.clone()).map_err(|_| Error::Invalid)?;
    // Native storage can represent these names, but they are not navigable
    // application paths. Never normalize them into another repository path.
    if request.path.contains('\0')
        || request
            .path
            .split('/')
            .any(|name| matches!(name, "." | ".."))
    {
        return Err(Error::Invalid);
    }
    if path.components().count() > 64 {
        return Err(Error::Limit);
    }
    let after = request
        .after
        .as_ref()
        .map(|name| {
            if name.len() > 4096 {
                return Err(Error::Limit);
            }
            RepoPathComponentBuf::new(name.clone()).map_err(|_| Error::Invalid)
        })
        .transpose()?;
    let authority = Arc::new(ReadOnlyAuthority {
        access: context.access.clone(),
        budget: context.budget.clone(),
    });
    let backend = NativeBackend::new(context.objects.clone(), authority, BackendLimits::default())
        .map_err(|_| Error::Unavailable)?;
    let settings = UserSettings::from_config(StackedConfig::with_defaults())
        .map_err(|_| Error::Unavailable)?;
    let store = Store::new(
        Box::new(backend),
        Signer::new(None, vec![]),
        MergeOptions::from_settings(&settings).map_err(|_| Error::Unavailable)?,
    );
    let commit = store
        .get_commit_async(&b::CommitId::from_bytes(request.commit.as_bytes()))
        .await
        .map_err(map_backend)?;
    let merged = commit.tree();
    let mut trees = merged.trees().await.map_err(map_backend)?;
    for name in path.components() {
        context.budget.authorize(context.access.as_ref()).await?;
        let value = trees.value(name);
        if value.as_resolved() == Some(&None) {
            return Err(Error::NotFound);
        }
        if !value.is_tree() {
            return Err(Error::NotDirectory);
        }
        drop(value);
        trees = trees
            .sub_tree(name)
            .await
            .map_err(map_backend)?
            .ok_or(Error::NotDirectory)?;
    }
    let mut names = BTreeSet::new();
    for tree in trees.iter() {
        for entry in tree.entries_non_recursive() {
            context.budget.work(1)?;
            names.insert(entry.name().to_owned());
        }
    }
    // A continuation must exist in this exact native listing, even if a caller
    // uses the internal API without the transport's signed binding.
    if let Some(name) = &after
        && (!names.contains(name) || trees.value(name).as_resolved() == Some(&None))
    {
        return Err(Error::Invalid);
    }
    let mut entries: Vec<TreeEntry> = vec![];
    let mut next = None;
    for name in names {
        context.budget.authorize(context.access.as_ref()).await?;
        if after.as_ref().is_some_and(|after| &name <= after) {
            continue;
        }
        let value = trees.value(&name);
        if value.as_resolved() == Some(&None) {
            continue;
        }
        if entries.len() == request.limit {
            next = entries.last().map(|e| e.name.clone());
            break;
        }
        let entry = TreeEntry {
            name: name.as_internal_str().to_owned(),
            conflicted: !value.is_resolved(),
            terms: value
                .iter()
                .map(|value| {
                    value
                        .map(dto::TreeValue::try_from)
                        .transpose()
                        .map_err(|_| Error::Corrupt)
                })
                .collect::<Result<_>>()?,
        };
        context.budget.output(&entry)?;
        entries.push(entry);
    }
    let result = TreePage {
        version: 1,
        commit: request.commit,
        path: request.path,
        entries,
        next,
        stats: context.budget.stats(),
    };
    context.budget.complete_output(&result)?;
    Ok(result)
}
fn map_backend(error: b::BackendError) -> Error {
    match error {
        b::BackendError::ObjectNotFound { .. } => Error::Incomplete,
        b::BackendError::ReadObject { source, .. } => source
            .downcast::<objects::Error>()
            .map(|e| map_object(*e))
            .unwrap_or(Error::Unavailable),
        _ => Error::Unavailable,
    }
}
