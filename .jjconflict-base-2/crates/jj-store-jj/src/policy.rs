//! Native semantic validation shared by in-process and remote publication.
use crate::{codec::NativeCodec, objects::NativeObjects, roots};
use jj_lib::{backend as b, object_id::ObjectId as _, op_store as o};
use jj_store_model as m;
use jj_store_wal::{
    self as wal,
    inventory::DependencySource as _,
    publication::{ManifestSnapshot, Request, Snapshot, Transition},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Duration,
};

#[derive(Clone, Debug, Default)]
pub struct Protection {
    /// Protect these tips and all native commit ancestors. Root is always protected.
    pub commits: BTreeSet<m::CommitId>,
    /// Immutable local bookmark targets; supplied by trusted server configuration.
    pub bookmarks: BTreeSet<String>,
}
#[derive(Clone, Copy, Debug)]
pub struct PolicyLimits {
    pub max_nodes: usize,
    pub max_work: usize,
    pub max_bytes: usize,
    pub timeout: Duration,
}
impl Default for PolicyLimits {
    fn default() -> Self {
        Self {
            max_nodes: 100_000,
            max_work: 1_000_000,
            max_bytes: 64 * 1024 * 1024,
            timeout: Duration::from_secs(30),
        }
    }
}
pub struct NativePolicy {
    objects: Arc<NativeObjects>,
    protection: Protection,
    limits: PolicyLimits,
}
impl NativePolicy {
    pub fn new(
        objects: Arc<NativeObjects>,
        protection: Protection,
        limits: PolicyLimits,
    ) -> wal::Result<Self> {
        if limits.max_nodes == 0
            || limits.max_work == 0
            || limits.max_bytes == 0
            || limits.timeout.is_zero()
            || protection.commits.len() > limits.max_nodes
            || protection.bookmarks.len() > 1024
            || protection
                .bookmarks
                .iter()
                .any(|name| name.is_empty() || name.len() > 1024)
        {
            return Err(wal::Error::Limit("native policy configuration"));
        }
        Ok(Self {
            objects,
            protection,
            limits,
        })
    }
    pub async fn validate(
        &self,
        before: &ManifestSnapshot,
        request: &Request,
        after: &Snapshot,
        committed: &BTreeSet<m::ObjectKey>,
    ) -> wal::Result<()> {
        if before.manifest().repository != self.objects.scope().repository() {
            return Err(wal::Error::Denied);
        }
        tokio::time::timeout(
            self.limits.timeout,
            self.validate_inner(before, request, after, committed),
        )
        .await
        .map_err(|_| wal::Error::Limit("native policy time"))?
    }
    async fn validate_inner(
        &self,
        before: &ManifestSnapshot,
        request: &Request,
        after: &Snapshot,
        committed: &BTreeSet<m::ObjectKey>,
    ) -> wal::Result<()> {
        let Some(transition) = &request.transition else {
            return Ok(());
        };
        let mut graph = Graph::new(self.objects.clone(), self.limits);
        let (required, proposed) = match transition {
            Transition::Native { remove, add } => {
                if remove.is_empty()
                    || remove
                        .iter()
                        .any(|id| !committed.contains(&m::ObjectKey::Operation(*id)))
                {
                    return Err(wal::Error::Denied);
                }
                (remove.clone(), BTreeSet::from([*add]))
            }
            Transition::Strict { .. } => (before.manifest().heads.clone(), after.heads.clone()),
        };
        // Receipt replay happens before this guard. A different request must
        // never reintroduce an already superseded operation as a sibling head.
        if proposed.iter().any(|id| {
            committed.contains(&m::ObjectKey::Operation(*id))
                && !before.manifest().heads.contains(id)
        }) {
            return Err(wal::Error::Denied);
        }
        let mut ancestry = BTreeSet::new();
        for id in proposed {
            ancestry.extend(graph.operation_ancestors(id).await?);
        }
        if !required.is_subset(&ancestry) {
            return Err(wal::Error::Denied);
        }
        let new_ops: Vec<_> = ancestry
            .iter()
            .filter(|id| !committed.contains(&m::ObjectKey::Operation(**id)))
            .copied()
            .collect();
        // Every newly introduced operation is checked, including unpublished
        // reconciliation intermediates, rather than trusting only the last view.
        let mut protected = BTreeSet::from([roots::ROOT_COMMIT]);
        for id in &self.protection.commits {
            protected.extend(graph.commit_ancestors([*id]).await?);
        }
        let mut current_views = Vec::new();
        for head in &before.manifest().heads {
            let op = graph.operation(*head).await?;
            let view = graph.view(&op.view_id).await?;
            for (name, target) in &view.local_bookmarks {
                if self.protection.bookmarks.contains(name.as_str()) {
                    let ids = target
                        .added_ids()
                        .chain(target.removed_ids())
                        .map(commit_id)
                        .collect::<wal::Result<Vec<_>>>()?;
                    protected.extend(graph.commit_ancestors(ids).await?);
                }
            }
            current_views.push(view);
        }
        let mut protected_changes: BTreeMap<b::ChangeId, BTreeSet<m::CommitId>> = BTreeMap::new();
        for id in &protected {
            protected_changes
                .entry(graph.commit(*id).await?.change_id.clone())
                .or_default()
                .insert(*id);
        }
        for id in new_ops {
            let op = graph.operation(id).await?;
            let view = graph.view(&op.view_id).await?;
            if view.head_ids.is_empty() {
                return Err(wal::Error::Denied);
            }
            let visible = graph
                .commit_ancestors(
                    view.head_ids
                        .iter()
                        .map(commit_id)
                        .collect::<wal::Result<Vec<_>>>()?,
                )
                .await?;
            let mut inherited_workspaces = BTreeSet::new();
            for parent in &op.parents {
                let previous = graph.operation(operation_id(parent)?).await?;
                let previous = graph.view(&previous.view_id).await?;
                inherited_workspaces.extend(previous.wc_commit_ids.keys().cloned());
                for name in &self.protection.bookmarks {
                    let find = |view: &o::View| {
                        view.local_bookmarks
                            .iter()
                            .find(|(key, _)| key.as_str() == name)
                            .map(|(_, value)| value.clone())
                    };
                    if find(&previous) != find(&view) {
                        return Err(wal::Error::Denied);
                    }
                }
                let previous_visible = graph
                    .commit_ancestors(
                        previous
                            .head_ids
                            .iter()
                            .map(commit_id)
                            .collect::<wal::Result<Vec<_>>>()?,
                    )
                    .await?;
                if protected
                    .intersection(&previous_visible)
                    .any(|id| !visible.contains(id))
                {
                    return Err(wal::Error::Denied);
                }
            }
            for name in view.wc_commit_ids.keys() {
                if !inherited_workspaces.contains(name)
                    && current_views
                        .iter()
                        .any(|current| current.wc_commit_ids.contains_key(name))
                {
                    return Err(wal::Error::Denied);
                }
            }
            // Evolution metadata includes invisible predecessors; validate their
            // commit graphs too so root rewrites cannot hide outside visible heads.
            let mut inspected = visible;
            if let Some(predecessors) = &op.commit_predecessors {
                for (new, old) in predecessors {
                    let new = commit_id(new)?;
                    for id in old {
                        let id = commit_id(id)?;
                        if protected.contains(&id) && id != new {
                            return Err(wal::Error::Denied);
                        }
                    }
                    inspected.extend(graph.commit_ancestors([new]).await?);
                }
            }
            for id in inspected {
                let commit = graph.commit(id).await?;
                if protected_changes
                    .get(&commit.change_id)
                    .is_some_and(|originals| !originals.contains(&id))
                {
                    return Err(wal::Error::Denied);
                }
                if commit
                    .predecessors
                    .iter()
                    .map(commit_id)
                    .collect::<wal::Result<Vec<_>>>()?
                    .iter()
                    .any(|old| protected.contains(old) && *old != id)
                {
                    return Err(wal::Error::Denied);
                }
            }
        }
        Ok(())
    }
}
fn bad_native(error: impl std::fmt::Display) -> wal::Error {
    wal::Error::Inventory(wal::inventory::InventoryError::Source(error.to_string()))
}
fn commit_id(id: &b::CommitId) -> wal::Result<m::CommitId> {
    m::CommitId::from_slice(id.as_bytes()).map_err(bad_native)
}
fn operation_id(id: &o::OperationId) -> wal::Result<m::OperationId> {
    m::OperationId::from_slice(id.as_bytes()).map_err(bad_native)
}
struct Graph {
    objects: Arc<NativeObjects>,
    limits: PolicyLimits,
    nodes: usize,
    work: usize,
    bytes: usize,
    operations: BTreeMap<m::OperationId, Arc<o::Operation>>,
    commits: BTreeMap<m::CommitId, Arc<b::Commit>>,
    views: BTreeMap<m::ViewId, Arc<o::View>>,
}
impl Graph {
    fn new(objects: Arc<NativeObjects>, limits: PolicyLimits) -> Self {
        Self {
            objects,
            limits,
            nodes: 0,
            work: 0,
            bytes: 0,
            operations: BTreeMap::new(),
            commits: BTreeMap::new(),
            views: BTreeMap::new(),
        }
    }
    async fn charge(&mut self, value: Option<&impl NativeCodec>, edges: usize) -> wal::Result<()> {
        if let Some(value) = value {
            self.nodes += 1;
            self.bytes = self
                .bytes
                .saturating_add(value.encode_payload().map_err(bad_native)?.len());
        }
        self.work = self.work.saturating_add(edges + 1);
        if self.nodes > self.limits.max_nodes
            || self.work > self.limits.max_work
            || self.bytes > self.limits.max_bytes
        {
            return Err(wal::Error::Limit("native policy traversal"));
        }
        tokio::task::yield_now().await;
        Ok(())
    }
    async fn operation(&mut self, id: m::OperationId) -> wal::Result<Arc<o::Operation>> {
        if !self.operations.contains_key(&id) {
            let op: o::Operation = if id == roots::ROOT_OPERATION {
                roots::operation()
            } else {
                self.objects
                    .get(m::ObjectKey::Operation(id))
                    .await
                    .map_err(bad_native)?
            };
            if id != roots::ROOT_OPERATION && op.parents.is_empty() {
                return Err(wal::Error::Denied);
            }
            self.charge(Some(&op), op.parents.len()).await?;
            self.operations.insert(id, Arc::new(op));
        }
        Ok(self.operations[&id].clone())
    }
    async fn view(&mut self, id: &o::ViewId) -> wal::Result<Arc<o::View>> {
        let id = m::ViewId::from_slice(id.as_bytes()).map_err(bad_native)?;
        if !self.views.contains_key(&id) {
            let view: o::View = if id == roots::ROOT_VIEW {
                roots::view()
            } else {
                self.objects
                    .get(m::ObjectKey::View(id))
                    .await
                    .map_err(bad_native)?
            };
            self.charge(Some(&view), view.head_ids.len()).await?;
            self.views.insert(id, Arc::new(view));
        }
        Ok(self.views[&id].clone())
    }
    async fn commit(&mut self, id: m::CommitId) -> wal::Result<Arc<b::Commit>> {
        if !self.commits.contains_key(&id) {
            let commit: b::Commit = if id == roots::ROOT_COMMIT {
                roots::commit()
            } else {
                self.objects
                    .get(m::ObjectKey::Commit(id))
                    .await
                    .map_err(bad_native)?
            };
            if id != roots::ROOT_COMMIT && commit.parents.is_empty() {
                return Err(wal::Error::Denied);
            }
            self.charge(Some(&commit), commit.parents.len()).await?;
            self.commits.insert(id, Arc::new(commit));
        }
        Ok(self.commits[&id].clone())
    }
    async fn operation_ancestors(
        &mut self,
        root: m::OperationId,
    ) -> wal::Result<BTreeSet<m::OperationId>> {
        let mut pending = vec![root];
        let mut done = BTreeSet::new();
        while let Some(id) = pending.pop() {
            if !done.insert(id) {
                continue;
            }
            let op = self.operation(id).await?;
            self.charge(None::<&o::Operation>, op.parents.len()).await?;
            pending.extend(
                op.parents
                    .iter()
                    .map(operation_id)
                    .collect::<wal::Result<Vec<_>>>()?,
            );
        }
        Ok(done)
    }
    async fn commit_ancestors(
        &mut self,
        roots: impl IntoIterator<Item = m::CommitId>,
    ) -> wal::Result<BTreeSet<m::CommitId>> {
        let mut pending: Vec<_> = roots.into_iter().collect();
        let mut done = BTreeSet::new();
        while let Some(id) = pending.pop() {
            if !done.insert(id) {
                continue;
            }
            let c = self.commit(id).await?;
            self.charge(None::<&b::Commit>, c.parents.len()).await?;
            pending.extend(
                c.parents
                    .iter()
                    .map(commit_id)
                    .collect::<wal::Result<Vec<_>>>()?,
            );
        }
        Ok(done)
    }
}
