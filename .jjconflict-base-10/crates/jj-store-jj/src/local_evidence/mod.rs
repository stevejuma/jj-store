//! Exact source context for the pinned ordinary GitBackend profile.
mod capture;
mod operation_archive;
pub(crate) use operation_archive::MAX_ARCHIVE_BYTES;
pub use operation_archive::{
    OperationArchiveLimits, OperationArchiveV1, capture_operation_archive,
};
mod commits;
mod operations;
mod views;
mod wire;
use crate::local_metadata::{Error, Result};
pub use capture::{CapturedEvidence, capture, inspect};
use jj_lib::{backend as b, merge::Merge, object_id::ObjectId as _, op_store as o};
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::BTreeMap;
pub use wire::*;

macro_rules! id_conversion {
    ($to:ident, $from:ident, $native:ty, $owned:ty) => {
        fn $to(id: &$native) -> Result<$owned> {
            <$owned>::from_slice(id.as_bytes()).map_err(Error::other)
        }
        fn $from(id: $owned) -> $native {
            <$native>::from_bytes(id.as_bytes())
        }
    };
}
id_conversion!(
    git_commit_id,
    git_native_commit_id,
    b::CommitId,
    m::GitSha1Id
);
id_conversion!(git_tree_id, git_native_tree_id, b::TreeId, m::GitSha1Id);
id_conversion!(view_id, native_view_id, o::ViewId, m::ViewId);
id_conversion!(
    operation_id,
    native_operation_id,
    o::OperationId,
    m::OperationId
);
id_conversion!(change_id, native_change_id, b::ChangeId, m::ChangeId);
fn merge<T>(terms: Vec<T>) -> Result<Merge<T>> {
    if terms.is_empty() || terms.len().is_multiple_of(2) {
        return Err(Error::Invalid("merge arity"));
    }
    Ok(Merge::from_vec(terms))
}
fn sorted_unique<T: Ord>(items: &[T]) -> Result<()> {
    if items.windows(2).any(|p| p[0] >= p[1]) {
        return Err(Error::Invalid("unsorted or duplicate keys"));
    }
    Ok(())
}

/// A source archive, not a destination operation log or a device path.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitEvidenceCapsuleV1 {
    pub version: u32,
    pub profile: m::portable::CodeProfileV1,
    pub repository: m::RepositoryId,
    pub operation_id: m::OperationId,
    pub view_id: m::ViewId,
    pub operation: GitOperationV1,
    pub view: GitViewV1,
    pub commits: BTreeMap<m::GitSha1Id, GitCommitV1>,
}
impl GitEvidenceCapsuleV1 {
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(Error::other)?;
        if bytes.len() > 16 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        Ok(bytes)
    }
    pub fn decode(expected: m::CodeCapsuleId, bytes: &[u8]) -> Result<Self> {
        if bytes.len() > 16 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        if m::CodeCapsuleId::new(Sha256::digest(bytes).into()) != expected {
            return Err(Error::Invalid("capsule identity"));
        }
        let value: Self = serde_json::from_slice(bytes).map_err(Error::other)?;
        if value.encode()? != bytes {
            return Err(Error::Invalid("noncanonical capsule"));
        }
        Ok(value)
    }
    pub fn id(&self) -> Result<m::CodeCapsuleId> {
        Ok(m::CodeCapsuleId::new(Sha256::digest(self.encode()?).into()))
    }
    pub fn validate(&self) -> Result<()> {
        use jj_lib::content_hash::blake2b_hash;
        if self.version != 1
            || self.profile != m::portable::CodeProfileV1::GitSha1Jj045
            || self.commits.len() > 100_000
        {
            return Err(Error::Invalid("capsule version/profile/bounds"));
        }
        let op = o::Operation::try_from(self.operation.clone())?;
        let view = o::View::try_from(self.view.clone())?;
        let exact_op = if self.operation_id == m::OperationId::new([0; 64]) {
            self.view_id == m::ViewId::new([0; 64])
                && op == o::Operation::make_root(native_view_id(self.view_id))
        } else {
            m::OperationId::new(blake2b_hash(&op).into()) == self.operation_id
        };
        let exact_view = if self.view_id == m::ViewId::new([0; 64]) {
            view == o::View::make_root(git_native_commit_id(m::GitSha1Id::new([0; 20])))
        } else {
            m::ViewId::new(blake2b_hash(&view).into()) == self.view_id
        };
        if self.operation.view_id != self.view_id || !exact_op || !exact_view {
            return Err(Error::Invalid("source operation/view identity"));
        }
        let mut needed = self.direct_commits()?;
        let mut seen = std::collections::BTreeSet::new();
        while let Some(id) = needed.pop_first() {
            if !seen.insert(id) {
                continue;
            }
            let commit = self
                .commits
                .get(&id)
                .ok_or(Error::Invalid("incomplete source extra closure"))?;
            b::Commit::try_from(commit.clone())?;
            needed.extend(commit.parents.iter().copied());
            needed.extend(commit.predecessors.iter().copied());
        }
        if seen.len() != self.commits.len() {
            return Err(Error::Invalid("unrelated source extra record"));
        }
        Ok(())
    }
    pub(crate) fn direct_commits(&self) -> Result<std::collections::BTreeSet<m::GitSha1Id>> {
        let view = o::View::try_from(self.view.clone())?;
        let mut ids = view.head_ids.clone();
        ids.extend(view.wc_commit_ids.values().cloned());
        for target in view
            .local_bookmarks
            .values()
            .chain(view.local_tags.values())
            .chain(view.git_refs.values())
            .chain(view.git_heads.values())
        {
            ids.extend(target.as_merge().iter().flatten().cloned());
        }
        for remote in view.remote_views.values() {
            for reference in remote.bookmarks.values().chain(remote.tags.values()) {
                ids.extend(reference.target.as_merge().iter().flatten().cloned());
            }
        }
        if let Some(map) = &self.operation.commit_predecessors {
            for (id, predecessors) in map {
                ids.insert(git_native_commit_id(*id));
                ids.extend(predecessors.iter().copied().map(git_native_commit_id));
            }
        }
        ids.iter().map(git_commit_id).collect()
    }
}
