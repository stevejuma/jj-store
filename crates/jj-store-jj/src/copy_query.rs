//! Bounded derived copy queries. No bucket listing and no persisted query index.
use crate::{
    backend::{BackendLimits, NativeBackend, id_bytes, other},
    codec::NativeCodec,
};
use futures::{
    StreamExt as _,
    stream::{self, BoxStream},
};
use jj_lib::{
    backend::*,
    object_id::ObjectId as _,
    repo_path::{RepoPath, RepoPathBuf},
};
use jj_store_model as m;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

struct Budget {
    limits: BackendLimits,
    nodes: usize,
    work: usize,
    bytes: usize,
}
impl Budget {
    fn new(limits: BackendLimits) -> Self {
        Self {
            limits,
            nodes: 0,
            work: 0,
            bytes: 0,
        }
    }
    async fn spend(&mut self, nodes: usize, work: usize, bytes: usize) -> BackendResult<()> {
        self.nodes = self.nodes.saturating_add(nodes);
        self.work = self.work.saturating_add(work);
        self.bytes = self.bytes.saturating_add(bytes);
        if self.nodes > self.limits.max_nodes
            || self.work > self.limits.max_work
            || self.bytes > self.limits.max_bytes
        {
            return Err(other("copy query resource budget exceeded"));
        }
        // CPU-only traversal must yield so deadlines and cancellation remain effective.
        tokio::task::yield_now().await;
        Ok(())
    }
    async fn object(&mut self, value: &impl NativeCodec) -> BackendResult<()> {
        self.spend(1, 1, value.encode_payload().map_err(other)?.len() + 256)
            .await
    }
}
/// Stable Kahn order with children before parents; duplicate edges count once.
async fn reverse_topo<I: Ord + Clone>(
    graph: &BTreeMap<I, Vec<I>>,
    budget: &mut Budget,
) -> BackendResult<Vec<I>> {
    let mut children: BTreeMap<I, usize> = graph.keys().cloned().map(|id| (id, 0)).collect();
    for parents in graph.values() {
        for parent in parents.iter().collect::<BTreeSet<_>>() {
            budget.spend(0, 1, 0).await?;
            if let Some(count) = children.get_mut(parent) {
                *count += 1;
            }
        }
    }
    let mut ready: BTreeSet<I> = children
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(id, _)| id.clone())
        .collect();
    let mut ordered = Vec::new();
    while let Some(id) = ready.pop_first() {
        for parent in graph[&id].iter().collect::<BTreeSet<_>>() {
            budget.spend(0, 1, 0).await?;
            if let Some(count) = children.get_mut(parent) {
                *count -= 1;
                if *count == 0 {
                    ready.insert(parent.clone());
                }
            }
        }
        ordered.push(id);
    }
    if ordered.len() != graph.len() {
        return Err(other("cycle in native history"));
    }
    Ok(ordered)
}

pub(crate) async fn related(
    backend: &NativeBackend,
    id: &CopyId,
) -> BackendResult<Vec<RelatedCopy>> {
    backend.authority.authorize_read().await.map_err(other)?;
    let requested = m::CopyId::new(id_bytes(id, "copy")?);
    let mut ids = backend
        .authority
        .copy_ids(backend.limits.max_nodes)
        .await
        .map_err(other)?;
    if ids.len() > backend.limits.max_nodes {
        return Err(other("copy inventory limit"));
    }
    ids.extend(
        backend
            .staged_copies
            .lock()
            .map_err(|_| other("copy lock poisoned"))?
            .iter()
            .copied(),
    );
    ids.insert(requested);
    if ids.len() > backend.limits.max_nodes {
        return Err(other("copy inventory limit"));
    }
    let mut pending: BTreeSet<CopyId> = ids
        .into_iter()
        .map(|id| CopyId::from_bytes(id.as_bytes()))
        .collect();
    let mut copies = BTreeMap::new();
    let mut budget = Budget::new(backend.limits);
    while let Some(id) = pending.pop_first() {
        if copies.contains_key(&id) {
            continue;
        }
        let copy = backend.read_copy(&id).await?;
        budget.object(&copy).await?;
        for parent in &copy.parents {
            budget.spend(0, 1, 64).await?;
            if !copies.contains_key(parent) {
                pending.insert(parent.clone());
            }
        }
        copies.insert(id, copy);
    }
    let graph = copies
        .iter()
        .map(|(id, copy)| (id.clone(), copy.parents.clone()))
        .collect();
    let order = reverse_topo(&graph, &mut budget).await?;
    backend.authority.authorize_read().await.map_err(other)?;
    Ok(order
        .into_iter()
        .map(|id| RelatedCopy {
            history: copies.remove(&id).expect("topological node"),
            id,
        })
        .collect())
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct File {
    path: RepoPathBuf,
    id: FileId,
    copy: CopyId,
}
struct Query<'a> {
    backend: &'a NativeBackend,
    budget: Budget,
    commits: BTreeMap<CommitId, Commit>,
    files: BTreeMap<CommitId, Vec<File>>,
    copies: BTreeMap<CopyId, CopyHistory>,
    order: VecDeque<CommitId>,
    paths: Option<BTreeSet<RepoPathBuf>>,
}
impl<'a> Query<'a> {
    async fn ancestors(&mut self, id: &CommitId) -> BackendResult<BTreeSet<CommitId>> {
        let mut visited = BTreeSet::new();
        let mut pending = BTreeSet::from([id.clone()]);
        while let Some(id) = pending.pop_first() {
            if !visited.insert(id.clone()) {
                continue;
            }
            self.budget.spend(0, 1, 128).await?;
            if !self.commits.contains_key(&id) {
                let commit = self.backend.read_commit(&id).await?;
                self.budget.object(&commit).await?;
                self.commits.insert(id.clone(), commit);
            }
            for parent in &self.commits[&id].parents {
                self.budget.spend(0, 1, 64).await?;
                if !visited.contains(parent) {
                    pending.insert(parent.clone());
                }
            }
        }
        Ok(visited)
    }
    async fn initialize(&mut self, root: &CommitId, head: &CommitId) -> BackendResult<()> {
        self.backend
            .authority
            .authorize_read()
            .await
            .map_err(other)?;
        let excluded = self.ancestors(root).await?;
        let included = self.ancestors(head).await?;
        // Validate the full graph, including ancestors needed for source lookup.
        let graph = self
            .commits
            .iter()
            .map(|(id, c)| (id.clone(), c.parents.clone()))
            .collect();
        self.order = reverse_topo(&graph, &mut self.budget)
            .await?
            .into_iter()
            .filter(|id| included.contains(id) && !excluded.contains(id))
            .collect();
        Ok(())
    }
    async fn files(&mut self, id: &CommitId) -> BackendResult<Vec<File>> {
        if !self.files.contains_key(id) {
            let mut files = BTreeSet::new();
            // Flatten every signed term before simplification. Reading only
            // positive terms would invent files deleted by base cancellation.
            let roots: Vec<_> = self.commits[id].root_tree.iter().cloned().collect();
            let mut terms = Vec::new();
            let mut paths = BTreeSet::new();
            for root in roots {
                let mut leaves = BTreeMap::new();
                let mut pending = vec![(RepoPathBuf::root(), root)];
                while let Some((path, tree)) = pending.pop() {
                    let tree = self.backend.read_tree(&path, &tree).await?;
                    self.budget.object(&tree).await?;
                    for entry in tree.entries() {
                        let path = path.join(entry.name());
                        self.budget
                            .spend(0, 1, path.as_internal_file_string().len() * 2 + 512)
                            .await?;
                        match entry.value() {
                            TreeValue::Tree(id) => pending.push((path, id.clone())),
                            value => {
                                paths.insert(path.clone());
                                leaves.insert(path, value.clone());
                            }
                        }
                    }
                }
                terms.push(leaves);
            }
            for path in paths {
                // Native simplification may compare each term against every
                // other term, so charge the worst case before doing CPU work.
                self.budget
                    .spend(0, terms.len().saturating_mul(terms.len()), 0)
                    .await?;
                let values: Vec<_> = terms.iter().map(|term| term.get(&path)).collect();
                let merged = jj_lib::merge::Merge::from_vec(values).simplify();
                for value in merged.adds().flatten() {
                    if let TreeValue::File { id, copy_id, .. } = value {
                        self.budget
                            .spend(0, 1, path.as_internal_file_string().len() + 256)
                            .await?;
                        files.insert(File {
                            path: path.clone(),
                            id: id.clone(),
                            copy: copy_id.clone(),
                        });
                    }
                }
            }
            self.files.insert(id.clone(), files.into_iter().collect());
        }
        let bytes = self.files[id]
            .iter()
            .map(|file| file.path.as_internal_file_string().len() + 256)
            .sum();
        self.budget.spend(0, self.files[id].len(), bytes).await?;
        Ok(self.files[id].clone())
    }
    async fn copy(&mut self, id: &CopyId) -> BackendResult<CopyHistory> {
        if !self.copies.contains_key(id) {
            let copy = self.backend.read_copy(id).await?;
            self.budget.object(&copy).await?;
            self.copies.insert(id.clone(), copy);
        }
        self.budget.spend(0, 1, 0).await?;
        Ok(self.copies[id].clone())
    }
    /// Nearest ancestor on each lineage. A shared ancestor is visited once;
    /// branches with a closer source stop without suppressing other branches.
    async fn sources(
        &mut self,
        start: &[CommitId],
        copy_id: &CopyId,
        path: &RepoPath,
    ) -> BackendResult<Vec<(CommitId, File)>> {
        let mut pending: VecDeque<_> = start.iter().cloned().collect();
        let mut visited = BTreeSet::new();
        let mut result = Vec::new();
        while let Some(id) = pending.pop_front() {
            if !visited.insert(id.clone()) {
                continue;
            }
            self.budget.spend(0, 1, 0).await?;
            let files = self.files(&id).await?;
            let matches: Vec<_> = files
                .into_iter()
                .filter(|file| file.path.as_ref() == path && file.copy == *copy_id)
                .collect();
            if matches.is_empty() {
                pending.extend(self.commits[&id].parents.iter().cloned());
            } else {
                result.extend(matches.into_iter().map(|file| (id.clone(), file)));
            }
        }
        Ok(result)
    }
    async fn for_commit(&mut self, id: CommitId) -> BackendResult<VecDeque<CopyRecord>> {
        let parents = self.commits[&id].parents.clone();
        let mut inherited = BTreeSet::new();
        for parent in &parents {
            inherited.extend(
                self.files(parent)
                    .await?
                    .into_iter()
                    .map(|file| (file.path, file.copy)),
            );
        }
        let mut records = BTreeSet::new();
        for target in self.files(&id).await? {
            if self
                .paths
                .as_ref()
                .is_some_and(|paths| !paths.contains(&target.path))
                || target.copy.as_bytes().is_empty()
                || inherited.contains(&(target.path.clone(), target.copy.clone()))
            {
                continue;
            }
            let history = self.copy(&target.copy).await?;
            if history.current_path != target.path {
                return Err(other("copy history path does not match tree path"));
            }
            let mut pending = history.parents;
            let mut visited = BTreeSet::new();
            while let Some(source_id) = pending.pop() {
                if !visited.insert(source_id.clone()) {
                    continue;
                }
                let source = self.copy(&source_id).await?;
                let found = self
                    .sources(&parents, &source_id, &source.current_path)
                    .await?;
                if found.is_empty() {
                    // A sequence of copies within a commit can have intermediate
                    // histories that never appeared in any ancestor tree.
                    // A new file copied before its first commit has no ancestor source.
                    pending.extend(source.parents);
                } else {
                    for (source_commit, source) in found {
                        self.budget
                            .spend(
                                0,
                                1,
                                512 + source.path.as_internal_file_string().len()
                                    + target.path.as_internal_file_string().len(),
                            )
                            .await?;
                        records.insert((
                            target.path.clone(),
                            source.path,
                            source.id,
                            source_commit,
                        ));
                    }
                }
            }
        }
        Ok(records
            .into_iter()
            .map(|(target, source, source_file, source_commit)| CopyRecord {
                target,
                target_commit: id.clone(),
                source,
                source_file,
                source_commit,
            })
            .collect())
    }
}

pub(crate) fn records<'a>(
    backend: &'a NativeBackend,
    paths: Option<&[RepoPathBuf]>,
    root: &CommitId,
    head: &CommitId,
) -> BackendResult<BoxStream<'a, BackendResult<CopyRecord>>> {
    if paths.is_some_and(|p| p.len() > backend.limits.max_work) {
        return Err(other("copy path filter limit"));
    }
    let path_bytes = paths
        .unwrap_or_default()
        .iter()
        .try_fold(0usize, |total, path| {
            total.checked_add(path.as_internal_file_string().len() + 64)
        })
        .ok_or_else(|| other("copy path filter limit"))?;
    if path_bytes > backend.limits.max_bytes {
        return Err(other("copy path filter limit"));
    }
    let mut budget = Budget::new(backend.limits);
    budget.bytes = path_bytes;
    let query = Query {
        backend,
        budget,
        commits: BTreeMap::new(),
        files: BTreeMap::new(),
        copies: BTreeMap::new(),
        order: VecDeque::new(),
        paths: paths.map(|p| p.iter().cloned().collect()),
    };
    let bounds = Some((root.clone(), head.clone()));
    let deadline = tokio::time::Instant::now()
        .checked_add(backend.limits.timeout)
        .ok_or_else(|| other("invalid copy deadline"))?;
    // Bounded graph discovery is required for ordering. File/copy work is lazy
    // per commit, and dropping the stream cancels all remaining traversal.
    Ok(stream::try_unfold(
        (query, bounds, VecDeque::<CopyRecord>::new()),
        move |(mut query, bounds, mut pending)| async move {
            tokio::time::timeout_at(deadline, async {
                query
                    .backend
                    .authority
                    .authorize_read()
                    .await
                    .map_err(other)?;
                if let Some((root, head)) = bounds {
                    query.initialize(&root, &head).await?;
                }
                loop {
                    if let Some(record) = pending.pop_front() {
                        return Ok(Some((record, (query, None, pending))));
                    }
                    let Some(id) = query.order.pop_front() else {
                        return Ok(None);
                    };
                    pending = query.for_commit(id).await?;
                }
            })
            .await
            .map_err(|_| other("copy query deadline exceeded"))?
        },
    )
    .boxed())
}
