//! Data-only continuation of jj-lib 0.45.1's Apache-2.0 predecessor walker.
//! Follows src/evolution.rs, including splice, duplicate and legacy semantics.
use super::*;
use crate::query::memory::Retained;
use jj_lib::{
    backend::CommitId,
    evolution::CommitEvolutionEntry,
    repo::{ReadonlyRepo, Repo as _},
};
use std::collections::VecDeque;

#[derive(Clone)]
struct Row {
    commit: CommitId,
    operation: Option<OperationId>,
    // Revalidate the legacy boundary when emitting unknown predecessor data.
    legacy: Option<OperationId>,
}
struct State {
    operation: usize,
    pending: Vec<CommitId>,
    queued: VecDeque<Row>,
    _memory: Arc<Lease>,
}
struct Entry {
    start: CommitId,
    anchor: Row,
    state: Arc<Retained<State>>,
}
#[derive(Default)]
pub(super) struct Cache(VecDeque<Entry>);

impl State {
    fn new(start: &CommitId) -> Result<Self> {
        let memory = Arc::new(Lease::reserve(4096)?);
        Ok(Self {
            operation: 0,
            pending: vec![start.clone()],
            queued: VecDeque::new(),
            _memory: memory,
        })
    }
    fn fork(&self, work: &mut impl FnMut(usize) -> Result<()>) -> Result<Self> {
        work(self.pending.len() + self.queued.len() + 1)?;
        // Includes variable backend ID bytes, queue capacity, anchor and leases.
        let bytes = 4096u64
            .checked_add(self.pending.len() as u64 * 256)
            .and_then(|bytes| bytes.checked_add(self.queued.len() as u64 * 512))
            .ok_or(Error::Limit)?;
        let memory = Arc::new(Lease::reserve(bytes)?);
        Ok(Self {
            operation: self.operation,
            pending: self.pending.clone(),
            queued: self.queued.clone(),
            _memory: memory,
        })
    }
    fn flush(&mut self, legacy: Option<OperationId>) {
        self.queued.extend(self.pending.drain(..).map(|commit| Row {
            commit,
            operation: None,
            legacy,
        }));
    }
    fn visit(
        &mut self,
        operation: &Operation,
        work: &mut impl FnMut(usize) -> Result<()>,
    ) -> Result<()> {
        let map = operation
            .store_operation()
            .commit_predecessors
            .as_ref()
            .ok_or(Error::Corrupt)?;
        let mut terms = 0usize;
        for predecessors in map.values() {
            work(predecessors.len() + 1)?;
            terms = terms
                .checked_add(predecessors.len() + 1)
                .ok_or(Error::Limit)?;
        }
        // Admit pending expansion, emitted IDs, topo scratch and both old/new
        // queue buffers before the synchronous upstream algorithm grows them.
        let items = self
            .pending
            .capacity()
            .checked_add(self.queued.capacity())
            .and_then(|n| n.checked_add(terms))
            .ok_or(Error::Limit)?;
        self._memory = Arc::new(Lease::reserve(
            (items as u64)
                .checked_mul(1024)
                .and_then(|n| n.checked_add(4096))
                .ok_or(Error::Limit)?,
        )?);
        let mut emitted = Vec::new();
        let mut duplicate = false;
        let mut at = 0;
        let lookup_work = map.len().max(1).ilog2() as usize + 2;
        while let Some(id) = self.pending.get(at) {
            work(lookup_work + emitted.len() + 1)?;
            if let Some(predecessors) = operation.predecessors_for_commit(id) {
                if emitted.contains(id) {
                    self.pending.remove(at);
                    duplicate = true;
                    continue;
                }
                work(predecessors.len() + self.pending.len() + 1)?;
                emitted.extend(self.pending.splice(at..=at, predecessors.iter().cloned()));
            } else {
                at += 1;
            }
        }
        let id = OperationId::from_slice(operation.id().as_bytes()).map_err(|_| Error::Corrupt)?;
        let mut enqueue = |commit: &CommitId| {
            self.queued.push_back(Row {
                commit: commit.clone(),
                operation: Some(id),
                legacy: None,
            });
        };
        match &*emitted {
            [] => {}
            [commit] if !duplicate => enqueue(commit),
            _ => {
                let sorted = jj_lib::dag_walk::topo_order_reverse_ok(
                    emitted.iter().map(Ok),
                    |&id| id,
                    |&id| {
                        let predecessors =
                            operation.predecessors_for_commit(id).unwrap_or_default();
                        if let Err(error) = work(lookup_work + predecessors.len() + 1) {
                            return vec![Err(error)];
                        }
                        predecessors.iter().map(Ok).collect()
                    },
                    |_| Error::Corrupt,
                )?;
                for &commit in &sorted {
                    work(lookup_work)?;
                    if operation.predecessors_for_commit(commit).is_some() {
                        enqueue(commit);
                    }
                }
            }
        }
        Ok(())
    }
    async fn next(
        &mut self,
        head: &Operation,
        graph: &Graph,
        current: &mut Option<Operation>,
        work: &mut impl FnMut(usize) -> Result<()>,
    ) -> Result<Option<Row>> {
        work(1)?;
        while !self.pending.is_empty() && self.queued.is_empty() {
            work(1)?;
            if let Some(lookup) = &graph.predecessors {
                self.operation = lookup
                    .next(&self.pending, self.operation, work)?
                    .unwrap_or(graph.ordered.len());
            }
            let Some(&at) = graph.ordered.get(self.operation) else {
                // Reserve queue conversion even when the final op is exhausted.
                self._memory = Arc::new(Lease::reserve(
                    (self.pending.capacity() + self.queued.capacity()) as u64 * 1024 + 4096,
                )?);
                self.flush(None);
                break;
            };
            self.operation += 1;
            let operation = read_operation(head, graph, graph.nodes[at].key.id, current).await?;
            if !operation.stores_commit_predecessors() {
                self._memory = Arc::new(Lease::reserve(
                    (self.pending.capacity() + self.queued.capacity()) as u64 * 1024 + 4096,
                )?);
                self.flush(Some(graph.nodes[at].key.id));
                break;
            }
            self.visit(&operation, work)?;
        }
        Ok(self.queued.pop_front())
    }
}

async fn read_operation(
    head: &Operation,
    graph: &Graph,
    id: OperationId,
    current: &mut Option<Operation>,
) -> Result<Operation> {
    if let Some(operation) = current
        .as_ref()
        .filter(|operation| operation.id().as_bytes() == id.as_bytes())
    {
        return Ok(operation.clone());
    }
    let native = jj_lib::op_store::OperationId::from_bytes(id.as_bytes());
    let operation = if &native == head.id() {
        head.clone()
    } else {
        let data = head
            .op_store()
            .read_operation(&native)
            .await
            .map_err(crate::query::language::operations::store_error)?;
        Operation::new(head.op_store().clone(), native, data)
    };
    graph.check_operation(&operation)?;
    *current = Some(operation.clone());
    Ok(operation)
}
async fn hydrate(
    repo: &ReadonlyRepo,
    graph: &Graph,
    row: &Row,
    current: &mut Option<Operation>,
    map_backend: impl Fn(backend::BackendError) -> Error,
) -> Result<CommitEvolutionEntry> {
    let operation = if let Some(id) = row.operation {
        let operation = read_operation(repo.operation(), graph, id, current).await?;
        if operation.predecessors_for_commit(&row.commit).is_none() {
            return Err(Error::Corrupt);
        }
        Some(operation)
    } else {
        if let Some(id) = row.legacy {
            if read_operation(repo.operation(), graph, id, current)
                .await?
                .stores_commit_predecessors()
            {
                return Err(Error::Corrupt);
            }
        }
        None
    };
    let commit = repo
        .store()
        .get_commit_async(&row.commit)
        .await
        .map_err(map_backend)?;
    Ok(CommitEvolutionEntry { commit, operation })
}

pub(crate) async fn read<T>(
    repo: &ReadonlyRepo,
    graph: Arc<Graph>,
    start: CommitId,
    after: Option<CommitId>,
    limit: usize,
    mut work: impl FnMut(usize) -> Result<()>,
    map_backend: impl Fn(backend::BackendError) -> Error + Copy,
    mut emit: impl AsyncFnMut(CommitEvolutionEntry) -> Result<T>,
) -> Result<(Vec<T>, Option<CommitId>)> {
    if limit == 0 {
        return Err(Error::Limit);
    }
    if repo.operation().id().as_bytes() != graph.selected.as_bytes() {
        return Err(Error::NotCommitted);
    }
    let _scratch = Lease::reserve(4096)?;
    graph.check_operation(repo.operation())?;
    let cached = {
        let mut cache = graph.evolution.lock().expect("evolution checkpoints");
        work(cache.0.len() + 1)?;
        after.as_ref().and_then(|after| {
            let at = cache
                .0
                .iter()
                .position(|entry| entry.start == start && entry.anchor.commit == *after)?;
            let entry = cache.0.remove(at)?;
            let value = entry.state.get().map(|state| (entry.anchor.clone(), state));
            if value.is_some() {
                cache.0.push_back(entry);
            }
            value
        })
    };
    let mut current = None;
    let (mut state, mut found) = if let Some((anchor, state)) = cached {
        // No request-bound source lives in a checkpoint. Admit a private copy,
        // then rehydrate the exclusive anchor through this request's readers.
        let state = state.fork(&mut work)?;
        hydrate(repo, &graph, &anchor, &mut current, map_backend).await?;
        (state, true)
    } else {
        (State::new(&start)?, after.is_none())
    };
    let mut rows = Vec::new();
    let mut last = None;
    let mut next = None;
    while let Some(row) = state
        .next(repo.operation(), &graph, &mut current, &mut work)
        .await?
    {
        if !found {
            if after.as_ref() == Some(&row.commit) {
                hydrate(repo, &graph, &row, &mut current, map_backend).await?;
                found = true;
            }
            continue;
        }
        if rows.len() == limit {
            next = last;
            break;
        }
        rows.push(emit(hydrate(repo, &graph, &row, &mut current, map_backend).await?).await?);
        last = Some(row.commit.clone());
        if rows.len() == limit && row.operation.is_some() {
            work(row.commit.as_bytes().len() + 3)?;
        }
        if rows.len() == limit
            && row.operation.is_some()
            && graph.unique_evolution_anchor(&row.commit)
        {
            // Snapshot before a continuation probe advances the state.
            let snapshot = Retained::new(Arc::new(state.fork(&mut work)?));
            let mut cache = graph.evolution.lock().expect("evolution checkpoints");
            work(cache.0.len() + 1)?;
            cache
                .0
                .retain(|entry| !(entry.start == start && entry.anchor.commit == row.commit));
            if cache.0.len() == 32 {
                cache.0.pop_front();
            }
            cache.0.push_back(Entry {
                start: start.clone(),
                anchor: row,
                state: snapshot,
            });
        }
    }
    if !found {
        return Err(Error::Invalid);
    }
    Ok((rows, next))
}

#[cfg(test)]
mod tests {
    use super::super::tests::{Store, operation};
    use super::*;
    use jj_lib::op_store as o;
    fn commit(n: u8) -> CommitId {
        CommitId::from_bytes(&[n; 20])
    }
    fn known(rows: &[(u8, &[u8])]) -> Operation {
        let mut data = operation(vec![], 0, 0);
        data.commit_predecessors = Some(
            rows.iter()
                .map(|(id, predecessors)| {
                    (
                        commit(*id),
                        predecessors.iter().copied().map(commit).collect(),
                    )
                })
                .collect(),
        );
        let store: Arc<dyn o::OpStore> = Arc::new(Store(vec![]));
        Operation::new(store, o::OperationId::from_bytes(&[9; 64]), data)
    }
    #[test]
    fn evolution_state_preserves_branch_order_shared_predecessors_and_private_forks() {
        let operation = known(&[(1, &[2, 3]), (2, &[4]), (3, &[4]), (4, &[])]);
        let mut state = State::new(&commit(1)).unwrap();
        state.visit(&operation, &mut |_| Ok(())).unwrap();
        let ids = state
            .queued
            .iter()
            .map(|row| row.commit.clone())
            .collect::<Vec<_>>();
        // Pinned reverse topology retains the ordered branch targets.
        assert_eq!(ids, vec![commit(1), commit(2), commit(3), commit(4)]);
        assert!(state.pending.is_empty());
        assert!(
            state
                .queued
                .iter()
                .all(|row| row.operation == Some(OperationId::new([9; 64])))
        );
        let snapshot = state.fork(&mut |_| Ok(())).unwrap();
        state.queued.pop_front();
        assert_eq!(snapshot.queued.len(), 4);
        assert_eq!(state.queued.len(), 3);
        assert!(matches!(
            snapshot.fork(&mut |_| Err(Error::Canceled)),
            Err(Error::Canceled)
        ));
    }
    #[test]
    fn evolution_state_preserves_legacy_duplicates_and_rejects_cycles_and_work_exhaustion() {
        let mut unknown = State::new(&commit(1)).unwrap();
        unknown.pending.extend([commit(2), commit(2)]);
        unknown.flush(Some(OperationId::new([8; 64])));
        assert_eq!(
            unknown
                .queued
                .iter()
                .map(|row| row.commit.clone())
                .collect::<Vec<_>>(),
            vec![commit(1), commit(2), commit(2)]
        );
        assert!(unknown.queued.iter().all(|row| row.operation.is_none() && row.legacy == Some(OperationId::new([8;64]))));
        let mut cycle = State::new(&commit(1)).unwrap();
        assert_eq!(
            cycle
                .visit(&known(&[(1, &[2]), (2, &[1])]), &mut |_| Ok(()))
                .unwrap_err(),
            Error::Corrupt
        );
        let mut limited = State::new(&commit(1)).unwrap();
        assert_eq!(
            limited
                .visit(&known(&[(1, &[])]), &mut |_| Err(Error::Limit))
                .unwrap_err(),
            Error::Limit
        );
    }
}
