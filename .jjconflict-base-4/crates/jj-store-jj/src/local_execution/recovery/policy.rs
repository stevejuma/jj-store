//! Current trusted policy over the exact retained source; no new preparation.
use super::*;
use std::collections::BTreeSet;
const MAX_VISIBLE: usize = 4096;
impl RecoveredHistory {
    /// Composition supplies current immutable heads after original-request
    /// authorization. This validates data; it cannot construct a write proof.
    pub async fn validate_reword_policy(
        self,
        commit: m::GitSha1Id,
        immutable_heads: Vec<m::GitSha1Id>,
    ) -> Result<Self> {
        if commit == m::GitSha1Id::new([0; 20]) || immutable_heads.len() > 64 {
            return Err(Error::Invalid);
        }
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let mut budget = recovery_budget();
                budget.cancellation = cancellation;
                self.check(&budget).await?;
                validate(&self, commit, &immutable_heads, &budget)?;
                self.check(&budget).await?;
                Ok(self)
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}
fn validate(
    recovery: &RecoveredHistory,
    commit: m::GitSha1Id,
    immutable_heads: &[m::GitSha1Id],
    budget: &Budget,
) -> Result<()> {
    let source = recovery.witness.binding();
    let view = recovery
        .archive
        .views
        .get(&source.view)
        .ok_or(Error::Incomplete)?;
    if view.head_ids.len() > MAX_VISIBLE {
        return Err(Error::Limit);
    }
    budget.work(view.head_ids.len())?;
    let mut pending: BTreeSet<_> = view.head_ids.iter().copied().collect();
    let mut visible = BTreeSet::new();
    while let Some(id) = pending.pop_first() {
        budget.work(1)?;
        if !visible.insert(id) {
            continue;
        }
        if visible.len() > MAX_VISIBLE {
            return Err(Error::Limit);
        }
        let parents = &recovery
            .archive
            .commits
            .get(&id)
            .ok_or(Error::Incomplete)?
            .parents;
        budget.work(parents.len())?;
        pending.extend(parents.iter().copied());
    }
    let mut pending: BTreeSet<_> = immutable_heads.iter().copied().collect();
    if pending.len() != immutable_heads.len() {
        return Err(Error::Invalid);
    }
    if !visible.contains(&commit) || !pending.is_subset(&visible) {
        return Err(Error::NotCommitted);
    }
    let mut visited = BTreeSet::new();
    while let Some(id) = pending.pop_first() {
        budget.work(1)?;
        if !visited.insert(id) {
            continue;
        }
        if id == commit {
            return Err(Error::Denied);
        }
        if visited.len() > MAX_VISIBLE {
            return Err(Error::Limit);
        }
        let parents = &recovery
            .archive
            .commits
            .get(&id)
            .ok_or(Error::Incomplete)?
            .parents;
        budget.work(parents.len())?;
        pending.extend(parents.iter().copied());
    }
    Ok(())
}
