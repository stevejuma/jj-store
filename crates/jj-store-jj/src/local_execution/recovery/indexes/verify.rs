use super::*;
use crate::query::prepared::Source as _;
use futures::StreamExt as _;
use jj_lib::{
    backend::Backend as _,
    config::StackedConfig,
    default_index::{DefaultIndexStore, DefaultReadonlyIndex},
    git_backend::GitBackend,
    index::{Index as _, IndexStore as _},
    object_id::ObjectId as _,
    revset::ResolvedExpression,
    settings::UserSettings,
    store::Store,
    tree_merge::MergeOptions,
};
use std::{collections::BTreeSet, path::Path};
pub(super) fn store(recovery: &RecoveredHistory) -> Result<Arc<Store>> {
    let settings = UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None)
        .map_err(|_| Error::Unavailable)?;
    let backend = GitBackend::load(&settings, &recovery.access.metadata_path().join("store"))
        .map_err(|_| Error::Unavailable)?;
    if backend.git_repo_path() != recovery.witness.git_path() || backend.commit_id_length() != 20 {
        return Err(Error::EpochChanged);
    }
    Ok(Store::new(
        Box::new(backend),
        jj_lib::signing_factory::signer_from_settings(&settings).map_err(|_| Error::Unavailable)?,
        MergeOptions::from_settings(&settings).map_err(|_| Error::Unavailable)?,
    ))
}
pub(super) async fn signature(
    path: &Path,
    operation: m::OperationId,
    source: &archive::Archive<'_>,
    store: &Arc<Store>,
) -> Result<BTreeSet<m::GitSha1Id>> {
    let budget = source.budget;
    crate::query::memory::framing::weight(path, 20, 16, 100_000, &mut || budget.work(1))?;
    let op = source
        .operation(&jj_lib::op_store::OperationId::from_bytes(
            operation.as_bytes(),
        ))
        .await?;
    let loaded = budget
        .until(DefaultIndexStore::load(path).get_index_at_op(&op, store))
        .await?
        .map_err(|_| Error::Corrupt)?;
    let loaded = loaded
        .downcast_ref::<DefaultReadonlyIndex>()
        .ok_or(Error::Invalid)?;
    if loaded.num_commits() > 100_000 {
        return Err(Error::Limit);
    }
    let heads = loaded
        .all_heads_for_gc()
        .map_err(|_| Error::Corrupt)?
        .collect();
    let expression = ResolvedExpression::Range {
        roots: Box::new(ResolvedExpression::Commits(vec![])),
        heads: Box::new(ResolvedExpression::Commits(heads)),
        generation: 0..u64::MAX,
        parents_range: 0..u32::MAX,
    };
    let revset = loaded
        .evaluate_revset_impl(&expression, store)
        .map_err(|_| Error::Corrupt)?
        .into_inner();
    let mut changes = revset.commit_change_ids();
    let mut ids = BTreeSet::new();
    while let Some(record) = budget.until(changes.next()).await? {
        budget.work(1)?;
        let (id, change) = record.map_err(|_| Error::Corrupt)?;
        let id = m::GitSha1Id::from_slice(id.as_bytes()).map_err(|_| Error::Corrupt)?;
        let expected = source
            .recovery
            .archive
            .commits
            .get(&id)
            .ok_or(Error::Corrupt)?;
        if change.as_bytes() != expected.change_id.as_bytes() || !ids.insert(id) {
            return Err(Error::Corrupt);
        }
    }
    if ids.len() != loaded.num_commits() as usize {
        return Err(Error::Corrupt);
    }
    let mut graph = revset.stream_graph();
    while let Some(node) = budget.until(graph.next()).await? {
        budget.work(1)?;
        let (id, edges) = node.map_err(|_| Error::Corrupt)?;
        let id = m::GitSha1Id::from_slice(id.as_bytes()).map_err(|_| Error::Corrupt)?;
        let expected = source
            .recovery
            .archive
            .commits
            .get(&id)
            .ok_or(Error::Corrupt)?;
        let mut parents = edges
            .iter()
            .map(|edge| {
                m::GitSha1Id::from_slice(edge.target.as_bytes()).map_err(|_| Error::Corrupt)
            })
            .collect::<Result<Vec<_>>>()?;
        parents.sort();
        let mut expected = expected.parents.clone();
        expected.sort();
        if parents != expected {
            return Err(Error::Corrupt);
        }
    }
    Ok(ids)
}
