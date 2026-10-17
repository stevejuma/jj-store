use super::{Error, GitOid, Result, RetainedKind};
use std::collections::{BTreeMap, BTreeSet};

pub fn object(
    repo: &gix::Repository,
    oid: GitOid,
    kind: gix::objs::Kind,
    max: usize,
) -> Result<Vec<u8>> {
    let header = repo.find_header(oid.0).map_err(Error::other)?;
    if header.kind() != kind {
        return Err(Error::Invalid("Git object kind"));
    }
    if header.size() > max as u64 {
        return Err(Error::Limit);
    }
    let mut object = repo.find_object(oid.0).map_err(Error::other)?;
    if object.kind != kind {
        return Err(Error::Invalid("Git object kind"));
    }
    if object.data.len() > max {
        return Err(Error::Limit);
    }
    let actual =
        gix::objs::compute_hash(gix::hash::Kind::Sha1, kind, &object.data).map_err(Error::other)?;
    if actual != oid.0 {
        return Err(Error::Invalid("Git object identity"));
    }
    Ok(std::mem::take(&mut object.data))
}
/// Audit ordinary Git edges and turn JJ conflict-header trees into explicit
/// retention links. Git's own traversal does not follow extra commit headers.
pub fn audit(
    repo: &gix::Repository,
    roots: &BTreeMap<GitOid, RetainedKind>,
    limits: super::Limits,
    sync: bool,
) -> Result<BTreeMap<GitOid, RetainedKind>> {
    let mut pending = roots
        .iter()
        .map(|(&id, &kind)| (id, kind))
        .collect::<Vec<_>>();
    let mut seen = BTreeSet::new();
    let mut extras = BTreeMap::new();
    let mut total = 0usize;
    while let Some((id, kind)) = pending.pop() {
        if !seen.insert(id) {
            continue;
        }
        if seen.len() > limits.objects {
            return Err(Error::Limit);
        }
        let bytes = object(
            repo,
            id,
            match kind {
                RetainedKind::Blob => gix::objs::Kind::Blob,
                RetainedKind::Tree => gix::objs::Kind::Tree,
                RetainedKind::Commit => gix::objs::Kind::Commit,
            },
            limits.object_bytes,
        )?;
        total = total.checked_add(bytes.len()).ok_or(Error::Limit)?;
        if total > limits.closure_bytes {
            return Err(Error::Limit);
        }
        if sync {
            sync_object(repo, id)?;
        }
        match kind {
            RetainedKind::Blob => {}
            RetainedKind::Tree => {
                for entry in gix::objs::TreeRef::from_bytes(&bytes, gix::hash::Kind::Sha1)
                    .map_err(Error::other)?
                    .entries
                {
                    let kind = match entry.mode.kind() {
                        gix::objs::tree::EntryKind::Tree => RetainedKind::Tree,
                        gix::objs::tree::EntryKind::Commit => continue, // external submodule
                        _ => RetainedKind::Blob,
                    };
                    pending.push((GitOid::from_git(entry.oid.to_owned())?, kind));
                }
            }
            RetainedKind::Commit => {
                let commit = gix::objs::CommitRef::from_bytes(&bytes, gix::hash::Kind::Sha1)
                    .map_err(Error::other)?;
                pending.push((GitOid::from_git(commit.tree())?, RetainedKind::Tree));
                for parent in commit.parents() {
                    pending.push((GitOid::from_git(parent)?, RetainedKind::Commit));
                }
                if let Some(value) = commit.extra_headers().find(super::JJ_TREES_COMMIT_HEADER) {
                    let value = std::str::from_utf8(value).map_err(Error::other)?;
                    let trees = value
                        .split(' ')
                        .map(str::parse::<GitOid>)
                        .collect::<Result<Vec<_>>>()?;
                    if trees.len() < 3 || trees.len() % 2 == 0 {
                        return Err(Error::Invalid("JJ conflict tree header"));
                    }
                    for tree in trees {
                        extras.insert(tree, RetainedKind::Tree);
                        pending.push((tree, RetainedKind::Tree));
                    }
                }
            }
        }
        if pending.len() > limits.objects {
            return Err(Error::Limit);
        }
    }
    Ok(extras)
}
pub fn blob(repo: &gix::Repository, bytes: &[u8]) -> Result<GitOid> {
    let oid = GitOid::from_git(repo.write_blob(bytes).map_err(Error::other)?.detach())?;
    sync_object(repo, oid)?;
    Ok(oid)
}
pub fn tree(
    repo: &gix::Repository,
    links: BTreeMap<String, (GitOid, gix::objs::tree::EntryKind)>,
) -> Result<GitOid> {
    let mut entries = links
        .into_iter()
        .map(|(filename, (oid, mode))| gix::objs::tree::Entry {
            mode: mode.into(),
            filename: filename.into(),
            oid: oid.0,
        })
        .collect::<Vec<_>>();
    entries.sort();
    let oid = GitOid::from_git(
        repo.write_object(gix::objs::Tree { entries })
            .map_err(Error::other)?
            .detach(),
    )?;
    sync_object(repo, oid)?;
    Ok(oid)
}
pub fn tree_links(
    repo: &gix::Repository,
    oid: GitOid,
    max: usize,
) -> Result<BTreeMap<String, (GitOid, gix::objs::tree::EntryKind)>> {
    let bytes = object(repo, oid, gix::objs::Kind::Tree, max)?;
    let mut links = BTreeMap::new();
    for entry in gix::objs::TreeRef::from_bytes(&bytes, gix::hash::Kind::Sha1)
        .map_err(Error::other)?
        .entries
    {
        let name = std::str::from_utf8(entry.filename)
            .map_err(Error::other)?
            .to_string();
        if links
            .insert(
                name,
                (GitOid::from_git(entry.oid.to_owned())?, entry.mode.kind()),
            )
            .is_some()
        {
            return Err(Error::Invalid("duplicate Git tree entry"));
        }
    }
    Ok(links)
}
pub fn retained_links(
    retained: &BTreeMap<GitOid, RetainedKind>,
) -> BTreeMap<String, (GitOid, gix::objs::tree::EntryKind)> {
    retained
        .iter()
        .filter_map(|(id, kind)| {
            Some((
                id.to_string(),
                (
                    *id,
                    match kind {
                        RetainedKind::Blob => gix::objs::tree::EntryKind::Blob,
                        RetainedKind::Tree => gix::objs::tree::EntryKind::Tree,
                        RetainedKind::Commit => return None,
                    },
                ),
            ))
        })
        .collect()
}
pub fn parents(previous: Option<GitOid>, retained: &BTreeMap<GitOid, RetainedKind>) -> Vec<GitOid> {
    let mut ids = previous.into_iter().collect::<Vec<_>>();
    for (&id, kind) in retained {
        if *kind == RetainedKind::Commit && !ids.contains(&id) {
            ids.push(id);
        }
    }
    ids
}
pub fn sync_object(repo: &gix::Repository, oid: GitOid) -> Result<()> {
    let hex = oid.to_string();
    let parent = repo.common_dir().join("objects").join(&hex[..2]);
    let path = parent.join(&hex[2..]);
    // Existing packed objects are already immutable; newly written loose objects
    // must reach disk before the selecting ref can acknowledge publication.
    if path.exists() {
        std::fs::File::open(path)?.sync_all()?;
        sync_dir(&parent)?;
    }
    sync_dir(&repo.common_dir().join("objects"))
}
pub fn sync_dir(path: &std::path::Path) -> Result<()> {
    std::fs::File::open(path)?.sync_all()?;
    Ok(())
}
