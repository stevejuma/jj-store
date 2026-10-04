//! Point membership from retained operation/view roots and exact commit edges.
//! No trees, content, source indexes or unrelated operation heads are traversed.
use super::*;
use std::collections::VecDeque;

pub(super) async fn prove_commit(
    selected: m::GitSha1Id,
    operation: &jj_lib::operation::Operation,
    loader: &RepoLoader,
    pin: &Pin,
    guard: &Arc<scoped::Guard>,
    budget: &Budget,
    limits: Limits,
) -> Result<()> {
    let target = ObjectKey::Commit(selected);
    if guard.contains(target) {
        return Ok(());
    }
    let mut discovered: BTreeSet<_> = pin
        .inventory
        .iter()
        .filter_map(|key| match key {
            ObjectKey::Commit(id) => Some(*id),
            _ => None,
        })
        .collect();
    let mut commits: VecDeque<_> = discovered.iter().copied().collect();
    let mut operations: VecDeque<_> = operation.parent_ids().iter().cloned().collect();
    let mut visited_operations = BTreeSet::from([operation.id().clone()]);
    let store = guard.store();
    while !commits.is_empty() || !operations.is_empty() {
        // Interleave older operation references so abandoned/replaced commits
        // remain reachable without first draining current commit ancestry.
        for _ in 0..32 {
            let Some(id) = commits.pop_front() else { break };
            budget.authorize(pin).await?;
            // Bypass Store's commit cache: proof retains IDs, not commit bodies.
            let value = store
                .backend()
                .read_commit(&b::CommitId::from_bytes(id.as_bytes()))
                .await
                .map_err(native::map_backend)?;
            if guard.contains(target) {
                return Ok(());
            }
            for id in value.parents.iter().chain(&value.predecessors) {
                budget.work(1)?;
                let id = git_id(id)?;
                if discovered.insert(id) {
                    commits.push_back(id);
                }
            }
        }
        while let Some(id) = operations.pop_front() {
            budget.authorize(pin).await?;
            if visited_operations.contains(&id) {
                continue;
            }
            if visited_operations.len() >= limits.max_history_visits {
                return Err(Error::Limit);
            }
            visited_operations.insert(id.clone());
            bounded_op_file(pin.binding.metadata_path(), "operations", &id.hex(), budget)?;
            let operation = loader
                .load_operation(&id)
                .await
                .map_err(|_| Error::Incomplete)?;
            let data = dto::Operation::try_from(operation.store_operation())
                .map_err(|_| Error::Corrupt)?;
            budget.metadata(&data)?;
            bounded_op_file(
                pin.binding.metadata_path(),
                "views",
                &data.view_id.to_string(),
                budget,
            )?;
            let view = operation.view().await.map_err(|_| Error::Incomplete)?;
            budget
                .metadata(&dto::View::try_from(view.store_view()).map_err(|_| Error::Corrupt)?)?;
            for id in operation
                .all_referenced_commit_ids()
                .chain(view.all_referenced_commit_ids())
            {
                budget.work(1)?;
                let id = git_id(id)?;
                guard.grant([ObjectKey::Commit(id)])?;
                if discovered.insert(id) {
                    commits.push_back(id);
                }
            }
            if guard.contains(target) {
                return Ok(());
            }
            for parent in operation.parent_ids() {
                budget.work(1)?;
                if !visited_operations.contains(parent) {
                    operations.push_back(parent.clone());
                }
            }
            break;
        }
    }
    Err(Error::NotCommitted)
}
