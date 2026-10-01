use super::*;
use crate::local_metadata::{GitOid, MetadataStore, RetainedKind, Snapshot};
use jj_lib::{
    default_backend_factories::default_backend_factories,
    repo::{Repo as _, RepoLoader},
    settings::UserSettings,
};
use std::{collections::BTreeSet, path::Path, time::Duration};

#[derive(Clone, Debug)]
pub struct CapturedEvidence {
    pub capsule: m::CodeCapsuleId,
    pub blob: GitOid,
    pub retained: BTreeMap<GitOid, RetainedKind>,
}

/// Pins an explicit operation. Staging has no ref, operation-head or checkout
/// publication capability. Callers must select the capsule in a domain command.
pub async fn capture(
    settings: &UserSettings,
    repo_path: &Path,
    repository: m::RepositoryId,
    operation: m::OperationId,
    metadata: &MetadataStore,
) -> Result<CapturedEvidence> {
    tokio::time::timeout(
        Duration::from_secs(30),
        capture_inner(settings, repo_path, repository, operation, metadata),
    )
    .await
    .map_err(|_| Error::Limit)?
}
async fn capture_inner(
    settings: &UserSettings,
    repo_path: &Path,
    repository: m::RepositoryId,
    operation: m::OperationId,
    metadata: &MetadataStore,
) -> Result<CapturedEvidence> {
    let expected = crate::local_repository::metadata_store(settings, repo_path)?;
    if expected.git_dir().canonicalize()? != metadata.git_dir().canonicalize()? {
        return Err(Error::Invalid("capsule Git repository binding"));
    }
    let loader =
        RepoLoader::init_from_file_system(settings, repo_path, &default_backend_factories())
            .map_err(Error::other)?;
    let op = loader
        .load_operation(&native_operation_id(operation))
        .await
        .map_err(Error::other)?;
    let repo = loader.load_at(&op).await.map_err(Error::other)?;
    let data = op
        .op_store()
        .read_operation(op.id())
        .await
        .map_err(Error::other)?;
    let mut capsule = GitEvidenceCapsuleV1 {
        version: 1,
        profile: m::portable::CodeProfileV1::GitSha1Jj045,
        repository,
        operation_id: operation,
        view_id: view_id(op.view_id())?,
        operation: GitOperationV1::try_from(&data)?,
        view: GitViewV1::try_from(repo.view().store_view())?,
        commits: BTreeMap::new(),
    };
    let mut pending = capsule.direct_commits()?;
    let mut seen = BTreeSet::new();
    let mut bytes = serde_json::to_vec(&capsule).map_err(Error::other)?.len();
    while let Some(id) = pending.pop_first() {
        if !seen.insert(id) {
            continue;
        }
        if seen.len() > 100_000 {
            return Err(Error::Limit);
        }
        let commit = repo
            .store()
            .backend()
            .read_commit(&git_native_commit_id(id))
            .await
            .map_err(Error::other)?;
        let wire = GitCommitV1::try_from(&commit)?;
        if b::Commit::try_from(wire.clone())? != commit {
            return Err(Error::Invalid("lossy source commit"));
        }
        bytes = bytes
            .checked_add(serde_json::to_vec(&wire).map_err(Error::other)?.len() + 44)
            .ok_or(Error::Limit)?;
        if bytes > 16 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        pending.extend(wire.parents.iter().copied());
        pending.extend(wire.predecessors.iter().copied());
        capsule.commits.insert(id, wire);
    }
    let bytes = capsule.encode()?;
    let id = m::CodeCapsuleId::new(Sha256::digest(&bytes).into());
    let blob = metadata.stage_blob(&bytes)?;
    let retained = capsule.retention(blob)?;
    Ok(CapturedEvidence {
        capsule: id,
        blob,
        retained,
    })
}

impl GitEvidenceCapsuleV1 {
    pub fn retention(&self, blob: GitOid) -> Result<BTreeMap<GitOid, RetainedKind>> {
        self.validate()?;
        let mut retained = BTreeMap::from([(blob, RetainedKind::Blob)]);
        for (id, commit) in &self.commits {
            if *id != m::GitSha1Id::new([0; 20]) {
                retained.insert(id.to_string().parse()?, RetainedKind::Commit);
            }
            for tree in &commit.root_tree {
                retained.insert(tree.to_string().parse()?, RetainedKind::Tree);
            }
        }
        Ok(retained)
    }
}

impl MetadataStore {
    /// Only a capsule selected by this metadata snapshot is readable.
    pub fn read_evidence(
        &self,
        selected: &Snapshot,
        source: m::RepositoryId,
        capsule: m::CodeCapsuleId,
        blob: GitOid,
    ) -> Result<GitEvidenceCapsuleV1> {
        let value =
            GitEvidenceCapsuleV1::decode(capsule, &self.read_retained_blob(selected, blob)?)?;
        if value.repository != source {
            return Err(Error::Invalid("capsule source repository"));
        }
        for (id, kind) in value.retention(blob)? {
            if selected.root.retained.get(&id) != Some(&kind) {
                return Err(Error::Invalid("incomplete retained capsule closure"));
            }
        }
        Ok(value)
    }
}
