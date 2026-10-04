//! Immutable selected visibility/annotations, with fresh budgets on each reader.
use super::*;
use futures::TryStreamExt as _;
use jj_lib::{
    backend as b,
    default_index::DefaultReadonlyIndex,
    index::{
        ChangeIdIndex, Index as _, IndexError, IndexResult, MutableIndex, ReadonlyIndex,
        ResolvedChangeState, ResolvedChangeTargets,
    },
    object_id::{HexPrefix, ObjectId as _, PrefixResolution},
    op_store::RefTarget,
    revset::ResolvedExpression,
    store::Store,
    view::View,
};
use std::collections::BTreeMap;

#[derive(Clone)]
pub(super) struct Prepared {
    raw: DefaultReadonlyIndex,
    decorations: Arc<Decorations>,
}
impl Prepared {
    pub async fn build(
        raw: DefaultReadonlyIndex,
        view: &View,
        store: &Arc<Store>,
        access: &dyn ReadAccess,
        budget: &Budget,
    ) -> Result<Self> {
        let decorations = Decorations::build(&raw, view, store, access, budget).await?;
        Ok(Self {
            raw,
            decorations: Arc::new(decorations),
        })
    }
    pub fn num_commits(&self) -> u32 {
        self.raw.num_commits()
    }
    pub fn weight(&self) -> u64 {
        self.decorations.weight
    }
    pub fn reader_with_memory(
        &self,
        budget: Arc<Budget>,
        memory: Arc<memory::Indexes>,
    ) -> Box<dyn ReadonlyIndex> {
        Box::new(Reader {
            prepared: self.clone(),
            budget,
            _memory: memory,
        })
    }
}
struct Decorations {
    heads: BTreeSet<b::CommitId>,
    visible: BTreeSet<m::CommitId>,
    changes: Box<dyn ChangeIdIndex>,
    bookmarks: BTreeMap<m::CommitId, Vec<Bookmark>>,
    workspaces: BTreeMap<m::CommitId, Vec<String>>,
    weight: u64,
    _memory: memory::Lease,
}
fn id(id: &b::CommitId) -> Result<m::CommitId> {
    m::CommitId::from_slice(id.as_bytes()).map_err(|_| Error::Corrupt)
}
impl Decorations {
    async fn build(
        index: &DefaultReadonlyIndex,
        view: &View,
        store: &Arc<Store>,
        access: &dyn ReadAccess,
        budget: &Budget,
    ) -> Result<Self> {
        let mut weight =
            u64::from(index.num_commits()) * 256 + view.heads().len() as u64 * 256 + 1024 * 1024;
        for (name, target) in view.local_bookmarks() {
            weight += label_weight(name.as_str(), None, target, budget)?;
        }
        for (symbol, target) in view.all_remote_bookmarks() {
            weight += label_weight(
                symbol.name.as_str(),
                Some(symbol.remote.as_str()),
                &target.target,
                budget,
            )?;
        }
        for name in view.wc_commit_ids().keys() {
            budget.work(1)?;
            weight += 384 + name.as_str().len() as u64 * 2;
        }
        let memory = memory::Lease::reserve(weight).map_err(|e| budget.fail(e))?;
        let heads: BTreeSet<_> = view.heads().iter().cloned().collect();
        for head in &heads {
            budget.work(1)?;
            if !index.has_id(head).await.map_err(|_| Error::Unavailable)? {
                return Err(budget.fail(Error::Incomplete));
            }
        }
        let set = index
            .evaluate_revset(
                &ResolvedExpression::Ancestors {
                    heads: Box::new(ResolvedExpression::Commits(heads.iter().cloned().collect())),
                    generation: 0..u64::MAX,
                    parents_range: 0..u32::MAX,
                },
                store,
            )
            .map_err(|_| Error::Unavailable)?;
        let mut visible = BTreeSet::new();
        let mut stream = set.stream_graph();
        while let Some((commit, edges)) = stream.try_next().await.map_err(|_| Error::Unavailable)? {
            if visible.len() % 256 == 0 {
                budget.authorize(access).await?;
            }
            budget.work(32 + edges.len())?;
            if visible.len() >= index.num_commits() as usize {
                return Err(budget.fail(Error::Corrupt));
            }
            visible.insert(id(&commit)?);
        }
        // Upstream retains prefix ambiguity and hidden offsets. Empty heads
        // avoid a second ancestor traversal; this generation overlays visibility.
        let changes = index.change_id_index(&mut std::iter::empty());
        budget.work((index.num_commits() as usize).div_ceil(64))?;
        changes
            .resolve_prefix(&HexPrefix::from_bytes(
                store.backend().root_change_id().as_bytes(),
            ))
            .await
            .map_err(|_| Error::Unavailable)?;
        let mut value = Self {
            heads,
            visible,
            changes,
            bookmarks: BTreeMap::new(),
            workspaces: BTreeMap::new(),
            weight,
            _memory: memory,
        };
        for (name, target) in view.local_bookmarks() {
            value.add_bookmark(name.as_str(), None, target, budget)?;
        }
        for (symbol, remote) in view.all_remote_bookmarks() {
            value.add_bookmark(
                symbol.name.as_str(),
                Some(symbol.remote.as_str()),
                &remote.target,
                budget,
            )?;
        }
        for (name, commit) in view.wc_commit_ids() {
            budget.work(1)?;
            value
                .workspaces
                .entry(id(commit)?)
                .or_default()
                .push(name.as_str().to_owned());
        }
        for labels in value.bookmarks.values_mut() {
            budget.work(labels.len() * 32)?;
            labels.sort_by(|a, b| {
                (a.remote.is_some(), &a.name, &a.remote).cmp(&(
                    b.remote.is_some(),
                    &b.name,
                    &b.remote,
                ))
            });
        }
        budget.authorize(access).await?;
        Ok(value)
    }
    fn add_bookmark(
        &mut self,
        name: &str,
        remote: Option<&str>,
        target: &RefTarget,
        budget: &Budget,
    ) -> Result<()> {
        let terms = target.added_ids().count() + target.removed_ids().count();
        budget.work(1 + terms * terms)?;
        for commit in target
            .added_ids()
            .chain(target.removed_ids())
            .collect::<BTreeSet<_>>()
        {
            budget.work(1)?;
            self.bookmarks
                .entry(id(commit)?)
                .or_default()
                .push(Bookmark {
                    name: name.to_owned(),
                    remote: remote.map(str::to_owned),
                    added: target.added_ids().any(|id| id == commit),
                    removed: target.removed_ids().any(|id| id == commit),
                    conflicted: target.has_conflict(),
                });
        }
        Ok(())
    }
    fn annotations(
        &self,
        id: m::CommitId,
        budget: &Budget,
    ) -> Result<(Vec<Bookmark>, Vec<String>)> {
        let labels = self
            .bookmarks
            .get(&id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        let names = self
            .workspaces
            .get(&id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        budget.work(1 + labels.len() + names.len())?;
        Ok((labels.to_vec(), names.to_vec()))
    }
}
fn label_weight(
    name: &str,
    remote: Option<&str>,
    target: &RefTarget,
    budget: &Budget,
) -> Result<u64> {
    budget.work(1)?;
    let terms = target.added_ids().count() + target.removed_ids().count();
    if terms > budget.limits.metadata.max_merge_terms {
        return Err(budget.fail(Error::Limit));
    }
    Ok(terms as u64 * (384 + name.len() as u64 * 2 + remote.map_or(0, str::len) as u64 * 2))
}

pub(super) struct Reader {
    pub prepared: Prepared,
    budget: Arc<Budget>,
    _memory: Arc<memory::Indexes>,
}
impl Reader {
    pub fn annotations(&self, id: m::CommitId) -> Result<(Vec<Bookmark>, Vec<String>)> {
        self.prepared.decorations.annotations(id, &self.budget)
    }
}
impl ReadonlyIndex for Reader {
    fn as_index(&self) -> &dyn jj_lib::index::Index {
        &self.prepared.raw
    }
    fn change_id_index(
        &self,
        heads: &mut dyn Iterator<Item = &b::CommitId>,
    ) -> Box<dyn ChangeIdIndex> {
        Box::new(Changes {
            decorations: self.prepared.decorations.clone(),
            budget: self.budget.clone(),
            matches: heads.cloned().collect::<BTreeSet<_>>() == self.prepared.decorations.heads,
        })
    }
    fn start_modification(&self) -> Box<dyn MutableIndex> {
        ReadonlyIndex::start_modification(&self.prepared.raw)
    }
}
struct Changes {
    decorations: Arc<Decorations>,
    budget: Arc<Budget>,
    matches: bool,
}
impl Changes {
    fn check(&self) -> IndexResult<()> {
        self.budget
            .work(1)
            .map_err(|e| IndexError::Other(Box::new(e)))?;
        if !self.matches {
            return Err(IndexError::Other(Box::new(
                self.budget.fail(Error::NotCommitted),
            )));
        }
        Ok(())
    }
}
#[async_trait::async_trait]
impl ChangeIdIndex for Changes {
    async fn resolve_prefix(
        &self,
        prefix: &HexPrefix,
    ) -> IndexResult<PrefixResolution<ResolvedChangeTargets>> {
        self.check()?;
        let mut result = self.decorations.changes.resolve_prefix(prefix).await?;
        if let PrefixResolution::SingleMatch(targets) = &mut result {
            for (commit, state) in &mut targets.targets {
                self.budget
                    .work(32)
                    .map_err(|e| IndexError::Other(Box::new(e)))?;
                *state = if self
                    .decorations
                    .visible
                    .contains(&id(commit).map_err(|e| IndexError::Other(Box::new(e)))?)
                {
                    ResolvedChangeState::Visible
                } else {
                    ResolvedChangeState::Hidden
                };
            }
        }
        self.budget
            .check()
            .map_err(|e| IndexError::Other(Box::new(e)))?;
        Ok(result)
    }
    async fn shortest_unique_prefix_len(&self, id: &b::ChangeId) -> IndexResult<usize> {
        self.check()?;
        self.decorations
            .changes
            .shortest_unique_prefix_len(id)
            .await
    }
}
