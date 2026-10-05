use super::*;
pub(super) fn check(
    source: &GitEvidenceCapsuleV1,
    visible: &BTreeSet<m::GitSha1Id>,
    request: &Reword,
) -> Result<()> {
    let mut pending: BTreeSet<_> = request.immutable_heads.iter().copied().collect();
    if pending.len() != request.immutable_heads.len() {
        return Err(Error::Invalid);
    }
    if !pending.is_subset(visible) {
        return Err(Error::NotCommitted);
    }
    let mut visited = BTreeSet::new();
    while let Some(id) = pending.pop_first() {
        if !visited.insert(id) {
            continue;
        }
        if id == request.commit {
            return Err(Error::Denied);
        }
        if visited.len() > MAX_COMMITS {
            return Err(Error::Limit);
        }
        pending.extend(
            source
                .commits
                .get(&id)
                .ok_or(Error::Incomplete)?
                .parents
                .iter()
                .copied(),
        );
    }
    Ok(())
}
