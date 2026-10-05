//! Exact source operation ancestry; distinct from a single selected capsule.
mod capture;
mod materialize;
use super::*;
pub use capture::{OperationArchiveLimits, capture_operation_archive};
use std::collections::BTreeSet;
pub const MAX_ARCHIVE_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_OPERATIONS: usize = 4096;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationArchiveV1 {
    pub version: u32,
    pub profile: m::portable::CodeProfileV1,
    pub repository: m::RepositoryId,
    pub operation: m::OperationId,
    pub operations: BTreeMap<m::OperationId, GitOperationV1>,
    pub views: BTreeMap<m::ViewId, GitViewV1>,
    pub commits: BTreeMap<m::GitSha1Id, GitCommitV1>,
}
impl OperationArchiveV1 {
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(Error::other)?;
        if bytes.len() > MAX_ARCHIVE_BYTES {
            return Err(Error::Limit);
        }
        Ok(bytes)
    }
    pub fn id(&self) -> Result<m::RecordDigest> {
        let mut hash = Sha256::new_with_prefix(b"jj-store/stack-operation-archive/v1\0");
        hash.update(self.encode()?);
        Ok(m::RecordDigest::new(hash.finalize().into()))
    }
    pub fn decode(id: m::RecordDigest, bytes: &[u8]) -> Result<Self> {
        if bytes.len() > MAX_ARCHIVE_BYTES {
            return Err(Error::Limit);
        }
        let value: Self = serde_json::from_slice(bytes).map_err(Error::other)?;
        if value.encode()? != bytes || value.id()? != id {
            return Err(Error::Invalid("operation archive identity"));
        }
        Ok(value)
    }
    pub fn validate(&self) -> Result<()> {
        use jj_lib::content_hash::blake2b_hash;
        if self.version != 1
            || self.profile != m::portable::CodeProfileV1::GitSha1Jj045
            || self.repository == m::RepositoryId::new([0; 16])
            || self.operations.is_empty()
            || self.operations.len() > MAX_OPERATIONS
            || self.commits.len() > 100_000
        {
            return Err(Error::Invalid("operation archive version/profile/bounds"));
        }
        let mut pending = BTreeSet::from([self.operation]);
        let mut operations = BTreeSet::new();
        let mut views = BTreeSet::new();
        let mut needed = BTreeSet::new();
        while let Some(id) = pending.pop_first() {
            if !operations.insert(id) {
                continue;
            }
            let data = self
                .operations
                .get(&id)
                .ok_or(Error::Invalid("missing operation ancestor"))?;
            let native = o::Operation::try_from(data.clone())?;
            if id != m::OperationId::new([0; 64]) && data.parents.is_empty() {
                return Err(Error::Invalid("parentless non-root operation"));
            }
            let matches = if id == m::OperationId::new([0; 64]) {
                native == o::Operation::make_root(native_view_id(m::ViewId::new([0; 64])))
            } else {
                m::OperationId::new(blake2b_hash(&native).into()) == id
            };
            if !matches {
                return Err(Error::Invalid("operation archive operation identity"));
            }
            pending.extend(data.parents.iter().copied());
            views.insert(data.view_id);
            let view = self
                .views
                .get(&data.view_id)
                .ok_or(Error::Invalid("missing operation view"))?;
            let native = o::View::try_from(view.clone())?;
            let matches = if data.view_id == m::ViewId::new([0; 64]) {
                native == o::View::make_root(git_native_commit_id(m::GitSha1Id::new([0; 20])))
            } else {
                m::ViewId::new(blake2b_hash(&native).into()) == data.view_id
            };
            if !matches {
                return Err(Error::Invalid("operation archive view identity"));
            }
            needed.extend(self.context(id, data, view).direct_commits()?);
        }
        if operations.len() != self.operations.len()
            || views.len() != self.views.len()
            || self.closure(needed)?.len() != self.commits.len()
        {
            return Err(Error::Invalid("unrelated operation archive records"));
        }
        Ok(())
    }
    fn context(
        &self,
        id: m::OperationId,
        operation: &GitOperationV1,
        view: &GitViewV1,
    ) -> GitEvidenceCapsuleV1 {
        GitEvidenceCapsuleV1 {
            version: 1,
            profile: self.profile,
            repository: self.repository,
            operation_id: id,
            view_id: operation.view_id,
            operation: operation.clone(),
            view: view.clone(),
            commits: BTreeMap::new(),
        }
    }
    fn closure(
        &self,
        mut needed: BTreeSet<m::GitSha1Id>,
    ) -> Result<BTreeMap<m::GitSha1Id, GitCommitV1>> {
        let mut commits = BTreeMap::new();
        while let Some(id) = needed.pop_first() {
            if commits.contains_key(&id) {
                continue;
            }
            let data = self
                .commits
                .get(&id)
                .ok_or(Error::Invalid("incomplete archive code closure"))?;
            b::Commit::try_from(data.clone())?;
            needed.extend(data.parents.iter().copied());
            needed.extend(data.predecessors.iter().copied());
            commits.insert(id, data.clone());
        }
        Ok(commits)
    }
    pub fn capsule(&self) -> Result<GitEvidenceCapsuleV1> {
        self.validate()?;
        let op = &self.operations[&self.operation];
        let mut capsule = self.context(self.operation, op, &self.views[&op.view_id]);
        capsule.commits = self.closure(capsule.direct_commits()?)?;
        capsule.validate()?;
        Ok(capsule)
    }
    pub(crate) fn retention(
        &self,
        blob: crate::local_metadata::GitOid,
    ) -> Result<BTreeMap<crate::local_metadata::GitOid, crate::local_metadata::RetainedKind>> {
        use crate::local_metadata::RetainedKind;
        self.validate()?;
        let mut retained = BTreeMap::from([(blob, RetainedKind::Blob)]);
        for (id, data) in &self.commits {
            if *id != m::GitSha1Id::new([0; 20]) {
                retained.insert(id.to_string().parse()?, RetainedKind::Commit);
            }
            for tree in &data.root_tree {
                retained.insert(tree.to_string().parse()?, RetainedKind::Tree);
            }
        }
        Ok(retained)
    }
}
