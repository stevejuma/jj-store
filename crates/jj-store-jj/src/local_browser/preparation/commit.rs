//! Read-only Git commit decoding, following jj-lib 0.45.1/git_backend.rs
//! (Apache-2.0, Jujutsu Authors). No implicit extra import or retention ref writes.
use super::*;
use jj_lib::merge::Merge;

fn signature(value: gix::actor::SignatureRef<'_>) -> Result<b::Signature> {
    let text = |bytes: &[u8]| {
        if bytes == b"JJ_EMPTY_STRING" {
            String::new()
        } else {
            String::from_utf8_lossy(bytes).into_owned()
        }
    };
    let time = value.time().unwrap_or_default();
    Ok(b::Signature {
        name: text(value.name),
        email: text(value.email),
        timestamp: b::Timestamp {
            timestamp: b::MillisSinceEpoch(time.seconds.checked_mul(1000).ok_or(Error::Corrupt)?),
            tz_offset: time.offset.div_euclid(60),
        },
    })
}
pub(super) fn read(
    backend: &dyn b::Backend,
    git: &gix::Repository,
    extras: &extras::Extras,
    shallow: &BTreeSet<gix::ObjectId>,
    id: &b::CommitId,
    budget: &Budget,
) -> Result<b::Commit> {
    if id == backend.root_commit_id() {
        return Ok(b::make_root_commit(
            backend.root_change_id().clone(),
            backend.empty_tree_id().clone(),
        ));
    }
    bounded_git_object(backend, git_id(id)?, gix::object::Kind::Commit, budget)?;
    let oid = gix::ObjectId::from_bytes_or_panic(id.as_bytes());
    let object = git.find_object(oid).map_err(|_| Error::Incomplete)?;
    let commit = object.try_to_commit_ref().map_err(|_| Error::Corrupt)?;
    let mut parents = vec![];
    if !shallow.contains(&oid) {
        for id in commit.parents() {
            budget.work(1)?;
            if parents.len() >= 256 {
                return Err(Error::Limit);
            }
            parents.push(b::CommitId::from_bytes(id.as_bytes()));
        }
    }
    if parents.is_empty() {
        parents.push(backend.root_commit_id().clone());
    }
    let extra = extras.record(id.as_bytes(), budget)?;
    let change_id = extra
        .change
        .or_else(|| jj_lib::git_backend::extract_change_id_from_commit(&commit))
        .unwrap_or_else(|| jj_lib::git_backend::synthetic_change_id_from_git_commit_id(id));
    let mut root_tree = if let Some(value) = commit.extra_headers().find("jj:trees") {
        let mut terms = vec![];
        for value in value.split(|b| *b == b' ') {
            budget.work(1)?;
            if terms.len() >= 65 {
                return Err(Error::Limit);
            }
            let id = b::TreeId::try_from_hex(value).ok_or(Error::Corrupt)?;
            if id.as_bytes().len() != 20 {
                return Err(Error::Corrupt);
            }
            terms.push(id);
        }
        if terms.len() <= 1 || terms.len().is_multiple_of(2) {
            return Err(Error::Corrupt);
        }
        Merge::from_vec(terms)
    } else {
        Merge::resolved(b::TreeId::from_bytes(commit.tree().as_bytes()))
    };
    if root_tree.is_resolved() && extra.tree_conflict_format && !extra.trees.is_empty() {
        if extra.trees.len().is_multiple_of(2) {
            return Err(Error::Corrupt);
        }
        root_tree = Merge::from_vec(extra.trees);
    }
    let conflict_labels = if let Some(value) = commit.extra_headers().find("jj:conflict-labels") {
        let terms = std::str::from_utf8(value)
            .map_err(|_| Error::Corrupt)?
            .split_terminator('\n')
            .take(66)
            .map(str::to_owned)
            .collect::<Vec<_>>();
        if terms.len() > 65 {
            return Err(Error::Limit);
        }
        if terms.is_empty() || terms.len().is_multiple_of(2) {
            return Err(Error::Corrupt);
        }
        Merge::from_vec(terms)
    } else {
        Merge::resolved(String::new())
    };
    let secure_sig = if commit
        .extra_headers
        .iter()
        .any(|(key, _)| *key == b"gpgsig")
    {
        gix::objs::CommitRefIter::signature(&object.data, oid.kind())
            .map_err(|_| Error::Corrupt)?
            .map(|(sig, data)| b::SecureSig {
                data: data.to_bstring().into(),
                sig: sig.into_owned().into(),
            })
    } else {
        None
    };
    let value = b::Commit {
        parents,
        predecessors: extra.predecessors,
        root_tree,
        conflict_labels,
        change_id,
        description: String::from_utf8_lossy(commit.message).into_owned(),
        author: signature(commit.author().map_err(|_| Error::Corrupt)?)?,
        committer: signature(commit.committer().map_err(|_| Error::Corrupt)?)?,
        secure_sig,
    };
    Ok(value)
}
