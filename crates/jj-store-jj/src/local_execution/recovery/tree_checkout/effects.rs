use super::*;
use gix::objs::{Kind, tree::EntryKind};
pub(super) struct Effects {
    pub before_commit: m::GitSha1Id,
    pub after_commit: m::GitSha1Id,
    pub before_tree: m::GitSha1Id,
    pub after_tree: m::GitSha1Id,
    pub changes: Vec<Change>,
}
pub(super) fn capture(recovery: &RecoveredHistory, budget: &Budget) -> Result<Effects> {
    let binding = recovery.witness.binding();
    if recovery.archive.operations[&recovery.archive.operation].parents != [binding.operation] {
        return Err(Error::Invalid);
    }
    let before_commit = binding.commit;
    let view =
        &recovery.archive.views[&recovery.archive.operations[&recovery.archive.operation].view_id];
    let after_commit = *view
        .wc_commit_ids
        .get(&binding.workspace)
        .ok_or(Error::Incomplete)?;
    let root = |id| {
        let commit = recovery.archive.commits.get(&id).ok_or(Error::Incomplete)?;
        if commit.root_tree.len() != 1 || commit.conflict_labels != [String::new()] {
            return Err(Error::Invalid);
        }
        Ok(commit.root_tree[0])
    };
    let before_tree = root(before_commit)?;
    let after_tree = root(after_commit)?;
    if before_tree == after_tree {
        return Err(Error::Invalid);
    }
    let git = gix::open(recovery.witness.git_path()).map_err(|_| Error::Unavailable)?;
    let before = files(&git, before_tree, budget)?;
    let after = files(&git, after_tree, budget)?;
    let mut aliases = BTreeMap::new();
    for path in before.keys().chain(after.keys()) {
        let mut part = path.as_str();
        loop {
            if aliases
                .insert(part.to_ascii_lowercase(), part)
                .is_some_and(|old| old != part)
            {
                return Err(Error::Invalid);
            }
            if let Some((parent, _)) = part.rsplit_once('/') {
                part = parent;
            } else {
                break;
            }
        }
    }
    let files: std::collections::BTreeSet<_> = before
        .keys()
        .chain(after.keys())
        .map(|p| p.to_ascii_lowercase())
        .collect();
    for path in &files {
        let mut prefix = path.as_str();
        while let Some((parent, _)) = prefix.rsplit_once('/') {
            if files.contains(parent) {
                return Err(Error::Invalid); // file/directory transition
            }
            prefix = parent;
        }
    }
    let mut changes = BTreeMap::new();
    for (path, value) in &before {
        if after.get(path) != Some(value) {
            changes.insert(
                path.clone(),
                Change {
                    path: path.clone(),
                    before: Some(value.clone()),
                    after: after.get(path).cloned(),
                    observed: None,
                },
            );
        }
    }
    for (path, value) in &after {
        if !before.contains_key(path) {
            changes.insert(
                path.clone(),
                Change {
                    path: path.clone(),
                    before: None,
                    after: Some(value.clone()),
                    observed: None,
                },
            );
        }
    }
    if changes.is_empty() || changes.len() > tree::MAX_ENTRIES {
        return Err(Error::Limit);
    }
    Ok(Effects {
        before_commit,
        after_commit,
        before_tree,
        after_tree,
        changes: changes.into_values().collect(),
    })
}
pub(super) fn object(
    git: &gix::Repository,
    id: m::GitSha1Id,
    kind: Kind,
    max: usize,
    budget: &Budget,
) -> Result<Vec<u8>> {
    budget.work(1)?;
    let id = gix::ObjectId::from_bytes_or_panic(id.as_bytes());
    let header = git.find_header(id).map_err(|_| Error::Incomplete)?;
    if header.kind() != kind {
        return Err(Error::Corrupt);
    }
    if header.size() > max as u64 {
        return Err(Error::Limit);
    }
    let mut value = git.find_object(id).map_err(|_| Error::Incomplete)?;
    if value.kind != kind || value.data.len() > max {
        return Err(Error::Corrupt);
    }
    budget.read_bytes(value.data.len())?;
    if gix::objs::compute_hash(gix::hash::Kind::Sha1, kind, &value.data)
        .map_err(|_| Error::Corrupt)?
        != id
    {
        return Err(Error::Corrupt);
    }
    Ok(std::mem::take(&mut value.data))
}
fn files(
    git: &gix::Repository,
    root: m::GitSha1Id,
    budget: &Budget,
) -> Result<BTreeMap<String, FileValue>> {
    let mut pending = vec![(String::new(), root)];
    let mut files = BTreeMap::new();
    let mut total = 0usize;
    let mut entries = 0usize;
    while let Some((prefix, id)) = pending.pop() {
        let bytes = object(git, id, Kind::Tree, tree::MAX_FILE_BYTES, budget)?;
        let tree = gix::objs::TreeRef::from_bytes(&bytes, gix::hash::Kind::Sha1)
            .map_err(|_| Error::Corrupt)?;
        let mut names = std::collections::BTreeSet::new();
        for entry in tree.entries {
            budget.work(1)?;
            entries += 1;
            if entries > tree::MAX_ENTRIES {
                return Err(Error::Limit);
            }
            let name = std::str::from_utf8(entry.filename).map_err(|_| Error::Invalid)?;
            if name.contains('/') || !names.insert(name) {
                return Err(Error::Invalid);
            }
            let path = if prefix.is_empty() {
                name.into()
            } else {
                format!("{prefix}/{name}")
            };
            safe_path(&path)?;
            let id = m::GitSha1Id::from_slice(entry.oid.as_bytes()).map_err(|_| Error::Invalid)?;
            match entry.mode.kind() {
                EntryKind::Tree => pending.push((path, id)),
                EntryKind::Blob | EntryKind::BlobExecutable => {
                    let bytes = object(
                        git,
                        id,
                        Kind::Blob,
                        tree::MAX_FILE_BYTES.min(tree::MAX_BYTES - total),
                        budget,
                    )?;
                    total = total.checked_add(bytes.len()).ok_or(Error::Limit)?;
                    if total > tree::MAX_BYTES {
                        return Err(Error::Limit);
                    }
                    if files
                        .insert(
                            path,
                            FileValue {
                                object: id,
                                content: tree::content_digest(&bytes),
                                size: bytes.len() as u64,
                                executable: entry.mode.kind() == EntryKind::BlobExecutable,
                            },
                        )
                        .is_some()
                    {
                        return Err(Error::Corrupt);
                    }
                }
                _ => return Err(Error::Invalid),
            }
        }
    }
    Ok(files)
}
