//! Bounded immutable transfer; no original Git config, hooks or working files execute.
use super::{Error, Result};
use gix::objs::Write as _;
use std::collections::BTreeSet;

pub(super) fn copy(
    source: &gix::Repository,
    destination: &gix::Repository,
    root: &str,
) -> Result<()> {
    if source.object_hash() != gix::hash::Kind::Sha1
        || destination.object_hash() != gix::hash::Kind::Sha1
    {
        return Err(Error::Unsupported);
    }
    let mut pending = vec![gix::ObjectId::from_hex(root.as_bytes()).map_err(|_| Error::Binding)?];
    let mut seen = BTreeSet::new();
    let mut total = 0_u64;
    while let Some(id) = pending.pop() {
        if !seen.insert(id) {
            continue;
        }
        if seen.len() > 100_000 {
            return Err(Error::Limit);
        }
        let header = source.find_header(id).map_err(|_| Error::Native)?;
        if header.size() > 16 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        total = total.checked_add(header.size()).ok_or(Error::Limit)?;
        if total > 256 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        let object = source.find_object(id).map_err(|_| Error::Native)?;
        let actual = gix::objs::compute_hash(gix::hash::Kind::Sha1, object.kind, &object.data)
            .map_err(|_| Error::Native)?;
        if actual != id {
            return Err(Error::Binding);
        }
        match object.kind {
            gix::objs::Kind::Commit => {
                let commit = gix::objs::CommitRef::from_bytes(&object.data, gix::hash::Kind::Sha1)
                    .map_err(|_| Error::Native)?;
                pending.push(commit.tree());
                pending.extend(commit.parents());
                if let Some(trees) = commit.extra_headers().find("jj:trees") {
                    let trees = std::str::from_utf8(trees).map_err(|_| Error::Binding)?;
                    let trees = trees.split(' ').collect::<Vec<_>>();
                    if trees.len() < 3 || trees.len() % 2 == 0 {
                        return Err(Error::Binding);
                    }
                    for tree in trees {
                        pending.push(
                            gix::ObjectId::from_hex(tree.as_bytes()).map_err(|_| Error::Binding)?,
                        );
                    }
                }
            }
            gix::objs::Kind::Tree => {
                let tree = gix::objs::TreeRef::from_bytes(&object.data, gix::hash::Kind::Sha1)
                    .map_err(|_| Error::Native)?;
                for entry in tree.entries {
                    if entry.mode.kind() != gix::objs::tree::EntryKind::Commit {
                        pending.push(entry.oid.to_owned());
                    }
                }
            }
            gix::objs::Kind::Blob => {}
            gix::objs::Kind::Tag => return Err(Error::Unsupported),
        }
        if pending.len() > 100_000 {
            return Err(Error::Limit);
        }
        if destination
            .objects
            .write_buf(object.kind, &object.data)
            .map_err(|_| Error::Native)?
            != id
        {
            return Err(Error::Binding);
        }
        jj_store_git::metadata::objects::sync_object(
            destination,
            jj_store_git::metadata::GitOid::from_git(id).map_err(|_| Error::Binding)?,
        )
        .map_err(|_| Error::Native)?;
    }
    Ok(())
}
