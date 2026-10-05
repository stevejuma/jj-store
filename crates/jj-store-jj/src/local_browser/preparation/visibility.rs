//! Operation-bound visibility and reverse reference labels, built only once.
use super::*;
use crate::query::memory::Lease;
use futures::TryStreamExt as _;
use jj_lib::{
    default_index::DefaultReadonlyIndex,
    index::{
        ChangeIdIndex, Index as _, IndexError, IndexResult, MutableIndex, ReadonlyIndex,
        ResolvedChangeState, ResolvedChangeTargets,
    },
    object_id::{HexPrefix, PrefixResolution},
    revset::ResolvedExpression,
};

pub(super) struct Visibility {
    heads: BTreeSet<b::CommitId>,
    visible: crate::query::prepared::visible::Visible<m::GitSha1Id>,
    changes: Box<dyn ChangeIdIndex>,
    pub(in super::super) references: BTreeMap<String, VisibleChange>,
    bookmarks: BTreeMap<m::GitSha1Id, Vec<Bookmark>>,
    workspaces: BTreeMap<m::GitSha1Id, Vec<String>>,
    _memory: Lease,
}
pub(in super::super) struct VisibleChange {
    pub(in super::super) change: m::ChangeId,
    pub(in super::super) commits: Vec<m::GitSha1Id>,
}
impl Visibility {
    pub(super) fn log_window(
        &self,
        after: Option<m::GitSha1Id>,
        limit: usize,
        budget: &Budget,
    ) -> Result<(&[m::GitSha1Id], Option<m::GitSha1Id>)> {
        self.visible
            .window(after, limit, |amount| budget.work(amount))
    }
    pub(super) async fn build(
        index: &DefaultReadonlyIndex,
        source: &LocalSource,
    ) -> Result<Arc<Self>> {
        let operation = source
            .pin
            .operations
            .values()
            .next()
            .ok_or(Error::Incomplete)?;
        let view = source
            .pin
            .views
            .get(&operation.view_id)
            .ok_or(Error::Incomplete)?;
        // Reserve membership nodes, head IDs and every potential reverse label
        // before growth, including name replication across conflict terms.
        // In addition to visible membership, reserve ordered textual keys,
        // change/target vectors and map allocation before projection growth.
        let mut bytes =
            u64::from(index.num_commits()) * 384 + view.head_ids.len() as u64 * 128 + 1024 * 1024;
        for (name, target) in &view.local_bookmarks {
            bytes += label_weight(name, None, target, &source.budget)?;
        }
        for (remote, refs) in &view.remote_views {
            for (name, target) in &refs.bookmarks {
                bytes += label_weight(name, Some(remote), &target.target, &source.budget)?;
            }
        }
        for name in view.wc_commit_ids.keys() {
            source.budget.work(1)?;
            bytes += 256 + name.len() as u64 * 2;
        }
        let memory = Lease::reserve(bytes)?;
        let mut heads = BTreeSet::new();
        for id in &view.head_ids {
            source.budget.work(1)?;
            let id = b::CommitId::from_bytes(id.as_bytes());
            if !index.has_id(&id).await.map_err(|_| Error::Unavailable)? {
                return Err(Error::Incomplete);
            }
            heads.insert(id);
        }
        let expression = ResolvedExpression::Ancestors {
            heads: Box::new(ResolvedExpression::Commits(heads.iter().cloned().collect())),
            generation: 0..u64::MAX,
            parents_range: 0..u32::MAX,
        };
        let set = index
            .evaluate_revset(&expression, source.loader.store())
            .map_err(|_| Error::Unavailable)?;
        let mut stream = set.commit_change_ids();
        let mut visible = Vec::with_capacity(index.num_commits() as usize);
        let mut references = BTreeMap::new();
        while let Some((id, change)) = stream.try_next().await.map_err(|_| Error::Unavailable)? {
            if visible.len() % 256 == 0 {
                source.budget.authorize(&source.pin).await?;
            }
            source.budget.work(32)?;
            if visible.len() >= index.num_commits() as usize {
                return Err(Error::Corrupt);
            }
            let id = git_id(&id)?;
            let change = m::ChangeId::from_slice(change.as_bytes()).map_err(|_| Error::Corrupt)?;
            visible.push(id);
            references
                .entry(format!("change:{change}"))
                .or_insert_with(|| VisibleChange {
                    change,
                    commits: vec![],
                })
                .commits
                .push(id);
        }
        let visible = crate::query::prepared::visible::Visible::new(visible, |amount| {
            source.budget.work(amount)
        })?;
        for targets in references.values_mut() {
            source.budget.work(targets.commits.len() * 32)?;
            targets.commits.sort_unstable();
        }
        // Preserve the upstream prefix index and ordered target offsets. With
        // no heads its bit set has no parent traversal; visibility is overlaid.
        let changes = index.change_id_index(&mut std::iter::empty());
        source
            .budget
            .work((index.num_commits() as usize).div_ceil(64))?;
        changes
            .resolve_prefix(&HexPrefix::from_bytes(
                source.loader.store().backend().root_change_id().as_bytes(),
            ))
            .await
            .map_err(|_| Error::Unavailable)?;
        source.budget.check()?;
        let mut value = Self {
            heads,
            visible,
            changes,
            references,
            bookmarks: BTreeMap::new(),
            workspaces: BTreeMap::new(),
            _memory: memory,
        };
        for (name, target) in &view.local_bookmarks {
            value.add_bookmark(name, None, target, &source.budget)?;
        }
        // Reverse grouping is subsequently ordered by JJ's (name, remote) symbol.
        for (remote, refs) in &view.remote_views {
            for (name, target) in &refs.bookmarks {
                value.add_bookmark(name, Some(remote), &target.target, &source.budget)?;
            }
        }
        for (name, id) in &view.wc_commit_ids {
            source.budget.work(1)?;
            value.workspaces.entry(*id).or_default().push(name.clone());
        }
        for labels in value.bookmarks.values_mut() {
            source.budget.work(labels.len() * 16)?;
            labels.sort_by(|a, b| {
                (a.remote.is_some(), &a.name, &a.remote).cmp(&(
                    b.remote.is_some(),
                    &b.name,
                    &b.remote,
                ))
            });
        }
        Ok(Arc::new(value))
    }
    fn add_bookmark(
        &mut self,
        name: &str,
        remote: Option<&str>,
        wire: &dto::RefTarget,
        budget: &Budget,
    ) -> Result<()> {
        budget.work(1 + wire.0.len() * wire.0.len())?;
        let target = o::RefTarget::try_from(wire.clone()).map_err(|_| Error::Corrupt)?;
        for id in target
            .added_ids()
            .chain(target.removed_ids())
            .collect::<BTreeSet<_>>()
        {
            budget.work(1)?;
            self.bookmarks
                .entry(git_id(id)?)
                .or_default()
                .push(Bookmark {
                    name: name.to_owned(),
                    remote: remote.map(str::to_owned),
                    added: target.added_ids().any(|value| value == id),
                    removed: target.removed_ids().any(|value| value == id),
                    conflicted: target.has_conflict(),
                });
        }
        Ok(())
    }
    pub(super) fn annotations(
        &self,
        id: m::GitSha1Id,
        budget: &Budget,
    ) -> Result<(Vec<Bookmark>, Vec<String>)> {
        let bookmarks = self
            .bookmarks
            .get(&id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        let workspaces = self
            .workspaces
            .get(&id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        budget.work(1 + bookmarks.len() + workspaces.len())?;
        Ok((bookmarks.to_vec(), workspaces.to_vec()))
    }
    pub(in super::super) fn workspace_names(
        &self,
        id: m::GitSha1Id,
        budget: &Budget,
    ) -> Result<Vec<String>> {
        let names = self
            .workspaces
            .get(&id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        budget.work(1 + names.len())?;
        Ok(names.to_vec())
    }
}
fn label_weight(
    name: &str,
    remote: Option<&str>,
    target: &dto::RefTarget,
    budget: &Budget,
) -> Result<u64> {
    budget.work(1)?;
    if target.0.len() > 65 {
        return Err(Error::Limit);
    }
    Ok(target.0.len() as u64
        * (256 + name.len() as u64 * 2 + remote.map_or(0, str::len) as u64 * 2))
}

pub(super) struct PreparedIndex {
    pub index: DefaultReadonlyIndex,
    pub visibility: Arc<Visibility>,
    pub budget: Arc<Budget>,
}
impl ReadonlyIndex for PreparedIndex {
    fn as_index(&self) -> &dyn jj_lib::index::Index {
        &self.index
    }
    fn change_id_index(
        &self,
        heads: &mut dyn Iterator<Item = &b::CommitId>,
    ) -> Box<dyn ChangeIdIndex> {
        Box::new(Changes {
            visibility: self.visibility.clone(),
            budget: self.budget.clone(),
            matches: heads.cloned().collect::<BTreeSet<_>>() == self.visibility.heads,
        })
    }
    fn start_modification(&self) -> Box<dyn MutableIndex> {
        ReadonlyIndex::start_modification(&self.index)
    }
}
struct Changes {
    visibility: Arc<Visibility>,
    budget: Arc<Budget>,
    matches: bool,
}
fn error(value: Error) -> IndexError {
    IndexError::Other(Box::new(value))
}
impl Changes {
    fn check(&self) -> IndexResult<()> {
        self.budget.work(1).map_err(error)?;
        if !self.matches {
            return Err(error(Error::NotCommitted));
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
        let mut resolution = self.visibility.changes.resolve_prefix(prefix).await?;
        if let PrefixResolution::SingleMatch(targets) = &mut resolution {
            for (id, state) in &mut targets.targets {
                self.budget.work(32).map_err(error)?;
                *state = if self
                    .visibility
                    .visible
                    .contains(&git_id(id).map_err(error)?)
                {
                    ResolvedChangeState::Visible
                } else {
                    ResolvedChangeState::Hidden
                };
            }
        }
        self.budget.check().map_err(error)?;
        Ok(resolution)
    }
    async fn shortest_unique_prefix_len(&self, id: &b::ChangeId) -> IndexResult<usize> {
        self.check()?;
        self.visibility.changes.shortest_unique_prefix_len(id).await
    }
}
