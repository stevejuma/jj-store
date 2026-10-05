//! Repository-wide native Undo/Redo stacks, including ordinary CLI operations.
use super::*;
use jj_lib::operation::Operation;
const UNDO: &str = "undo: restore to operation ";
const REDO: &str = "redo: restore to operation ";
async fn referred(repo: &ReadonlyRepo, operation: Operation, prefix: &str) -> Result<Operation> {
    if let Some(hex) = operation.metadata().description.strip_prefix(prefix) {
        let id = jj_lib::op_store::OperationId::try_from_hex(hex).ok_or(Error::Invalid)?;
        repo.loader().load_operation(&id).await.map_err(jj)
    } else {
        Ok(operation)
    }
}
pub(super) async fn target(repo: &ReadonlyRepo, redo: bool) -> Result<Operation> {
    let prefix = if redo { REDO } else { UNDO };
    let selected = referred(repo, repo.operation().clone(), prefix).await?;
    if redo && !selected.metadata().description.starts_with(UNDO) {
        return Err(Error::Invalid);
    }
    let mut parents = selected.parents().await.map_err(jj)?;
    if parents.len() != 1 {
        return Err(Error::Invalid);
    }
    referred(repo, parents.remove(0), prefix).await
}
pub(super) async fn restore(
    mutable: &mut jj_lib::repo::MutableRepo,
    repo: &ReadonlyRepo,
    redo: bool,
) -> Result<String> {
    let target = target(repo, redo).await?;
    let mut view = target.view().await.map_err(jj)?.store_view().clone();
    // Stock jj undo/redo restores repository + remote tracking but retains
    // current Git export/import bookkeeping.
    view.git_refs = repo.view().store_view().git_refs.clone();
    view.git_heads = repo.view().store_view().git_heads.clone();
    mutable.set_view(view);
    Ok(format!(
        "{}{}",
        if redo { REDO } else { UNDO },
        target.id().hex()
    ))
}
