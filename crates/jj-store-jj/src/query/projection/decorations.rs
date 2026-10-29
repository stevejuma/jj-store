//! One admitted immutable visibility and reverse-label derivation.
use super::*;
use futures::TryStreamExt as _;

impl Decorations {
    pub(super) async fn build(
        index: &DefaultReadonlyIndex,
        view: &View,
        store: &Arc<Store>,
        access: &dyn ReadAccess,
        budget: &Budget,
    ) -> Result<Self> {
        let mut weight =
            commit_weight(index.num_commits()) + view.heads().len() as u64 * 256 + 1024 * 1024;
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
        let capacity = index.num_commits() as usize;
        let mut visible = Vec::with_capacity(capacity);
        let mut references = References::with_capacity(capacity);
        let mut stream = set.commit_change_ids();
        while let Some((commit, change)) =
            stream.try_next().await.map_err(|_| Error::Unavailable)?
        {
            if visible.len() % 256 == 0 {
                budget.authorize(access).await?;
            }
            budget.work(32)?;
            if visible.len() >= index.num_commits() as usize {
                return Err(budget.fail(Error::Corrupt));
            }
            let commit = id(&commit)?;
            let change = m::ChangeId::from_slice(change.as_bytes()).map_err(|_| Error::Corrupt)?;
            visible.push(commit);
            references.push(change, commit)?;
        }
        let visible = prepared::visible::Visible::new(visible, |amount| budget.work(amount))?;
        references.finish(budget)?;
        drop(stream);
        drop(set);
        // Upstream retains prefix ambiguity and hidden offsets. Empty heads
        // avoid a second ancestor traversal; this generation overlays visibility.
        let changes = index
            .change_id_index(&mut std::iter::empty())
            .map_err(|_| Error::Unavailable)?;
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
            references,
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
        let terms = target.present_adds().count() + target.present_removes().count();
        budget.work(1 + terms * terms)?;
        for commit in target
            .present_adds()
            .chain(target.present_removes())
            .collect::<BTreeSet<_>>()
        {
            budget.work(1)?;
            self.bookmarks
                .entry(id(commit)?)
                .or_default()
                .push(Bookmark {
                    name: name.to_owned(),
                    remote: remote.map(str::to_owned),
                    added: target.present_adds().any(|id| id == commit),
                    removed: target.present_removes().any(|id| id == commit),
                    conflicted: !target.is_resolved(),
                });
        }
        Ok(())
    }
    pub(super) fn annotations(
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
    let terms = target.present_adds().count() + target.present_removes().count();
    if terms > budget.limits.metadata.max_merge_terms {
        return Err(budget.fail(Error::Limit));
    }
    Ok(terms as u64 * (384 + name.len() as u64 * 2 + remote.map_or(0, str::len) as u64 * 2))
}
