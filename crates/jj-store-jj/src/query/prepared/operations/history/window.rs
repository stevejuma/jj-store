//! Request-local body hydration after exact compact topology selection.
use super::*;
use crate::query::language::operations::{Filter, store_error};
pub(crate) async fn read<T>(
    head: &Operation,
    graph: Arc<Graph>,
    filter: &Filter,
    after: Option<OperationId>,
    limit: usize,
    work: impl FnMut(usize) -> Result<()>,
    mut emit: impl FnMut(&Operation) -> Result<T>,
) -> Result<(Vec<T>, Option<OperationId>)> {
    let id = OperationId::from_slice(head.id().as_bytes()).map_err(|_| Error::Corrupt)?;
    let mut stream = std::pin::pin!(graph.clone().walk(id, work)?);
    let mut found = after.is_none();
    let mut result = vec![];
    let mut next = None;
    let mut last = None;
    while let Some(id) = stream.try_next().await? {
        if !found && after != Some(id) {
            continue;
        }
        let native = jj_lib::op_store::OperationId::from_bytes(id.as_bytes());
        let operation = if &native == head.id() {
            head.clone()
        } else {
            let data = head
                .op_store()
                .read_operation(&native)
                .await
                .map_err(store_error)?;
            Operation::new(head.op_store().clone(), native, data)
        };
        graph.check_operation(&operation)?;
        if !found {
            if !filter.matches(&operation) {
                return Err(Error::Invalid);
            }
            found = true;
            continue;
        }
        if !filter.matches(&operation) {
            continue;
        }
        if result.len() == limit {
            next = last;
            break;
        }
        result.push(emit(&operation)?);
        last = Some(id);
    }
    if !found {
        return Err(Error::Invalid);
    }
    Ok((result, next))
}
