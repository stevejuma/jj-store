use super::*;
use jj_lib::operation::Operation;
use std::sync::atomic::{AtomicUsize, Ordering};

/// File-backed operation-head publication can expose both a new operation and
/// its ancestors. Remove only ancestral candidates in memory: never call the
/// resolving loader, merge operations, update the head store or take a writer.
pub(super) async fn unique_maximal(loader: &RepoLoader) -> Result<Operation> {
    let heads = loader.op_heads_store().get_op_heads().await.map_err(jj)?;
    if heads.is_empty() || heads.len() > 64 {
        return Err(Error::Binding);
    }
    if let [id] = heads.as_slice() {
        return loader.load_operation(id).await.map_err(jj);
    }
    let mut operations = Vec::with_capacity(heads.len());
    for id in heads {
        operations.push(loader.load_operation(&id).await.map_err(jj)?);
    }
    let work = AtomicUsize::new(0);
    let frontier = tokio::time::timeout(
        Duration::from_secs(2),
        jj_lib::dag_walk_async::heads(
            operations,
            |operation: &Operation| operation.id().clone(),
            async |operation: &Operation| -> Result<Vec<Operation>> {
                if work.fetch_add(1, Ordering::Relaxed) >= 4096 {
                    return Err(Error::Binding);
                }
                operation.parents().await.map_err(jj)
            },
        ),
    )
    .await
    .map_err(|_| Error::Binding)??;
    let mut frontier = frontier.into_iter();
    let selected = frontier.next().ok_or(Error::Binding)?;
    if frontier.next().is_some() {
        return Err(Error::Binding);
    }
    Ok(selected)
}
