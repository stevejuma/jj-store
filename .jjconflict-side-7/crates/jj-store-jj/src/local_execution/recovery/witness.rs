use super::*;
use crate::local_browser::working_tree::{
    Checkout,
    current::{self, capture, physical},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::fs::File;
const MAX_WITNESS_BYTES: usize = 65536;

/// Installation-private recovery input, never a client command or authorization.
/// Composition must retain its digest with the original request before execution.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryWitnessV1 {
    version: u32,
    profile: String,
    repository: m::RepositoryId,
    generation: u64,
    physical_workspace: m::RecordDigest,
    operation: m::OperationId,
    view: m::ViewId,
    commit: m::GitSha1Id,
    checkout: Checkout,
    fingerprint: m::RecordDigest,
    git_path: PathBuf,
    source_binding: m::RecordDigest,
    heads: Vec<m::OperationId>,
}
/// Data-only source selection for installation-private request composition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoveryBinding {
    pub repository: m::RepositoryId,
    pub generation: u64,
    pub physical_workspace: m::RecordDigest,
    pub operation: m::OperationId,
    pub view: m::ViewId,
    pub commit: m::GitSha1Id,
    pub workspace: String,
}
impl RecoveryWitnessV1 {
    pub fn binding(&self) -> RecoveryBinding {
        RecoveryBinding {
            repository: self.repository,
            generation: self.generation,
            physical_workspace: self.physical_workspace,
            operation: self.operation,
            view: self.view,
            commit: self.commit,
            workspace: self.checkout.workspace.clone(),
        }
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        if self.version != 1
            || self.profile != local_browser::PROFILE
            || self.repository == m::RepositoryId::new([0; 16])
            || self.physical_workspace == m::RecordDigest::new([0; 32])
            || self.fingerprint == m::RecordDigest::new([0; 32])
            || self.source_binding == m::RecordDigest::new([0; 32])
            || self.operation != self.checkout.operation
            || !self.heads.contains(&self.operation)
            || self.heads.is_empty()
            || self.heads.len() > 64
            || self.heads.windows(2).any(|p| p[0] >= p[1])
            || !self.git_path.is_absolute()
            || self.git_path.as_os_str().len() > 4096
            || self.checkout.workspace.is_empty()
            || self.checkout.workspace.len() > 256
            || self.checkout.workspace.chars().any(char::is_control)
            || self.checkout.trees.is_empty()
            || self.checkout.trees.len() > 65
            || self.checkout.labels.len() > 65
            || self.checkout.sparse.len() > 4096
        {
            return Err(Error::Invalid);
        }
        let bytes = serde_json::to_vec(self).map_err(|_| Error::Invalid)?;
        if bytes.len() > MAX_WITNESS_BYTES {
            return Err(Error::Limit);
        }
        Ok(bytes)
    }
    pub fn id(&self) -> Result<m::RecordDigest> {
        let mut hash = Sha256::new_with_prefix(b"jj-store/stack-source-recovery-witness/v1\0");
        hash.update(self.encode()?);
        Ok(m::RecordDigest::new(hash.finalize().into()))
    }
    pub fn decode(id: m::RecordDigest, bytes: &[u8]) -> Result<Self> {
        if bytes.len() > MAX_WITNESS_BYTES {
            return Err(Error::Limit);
        }
        let value: Self = serde_json::from_slice(bytes).map_err(|_| Error::Invalid)?;
        if value.encode()? != bytes || value.id()? != id {
            return Err(Error::Invalid);
        }
        Ok(value)
    }
    pub(super) fn capture(lease: &Foreground, budget: &Budget) -> Result<Self> {
        let loader = local_browser::loader(lease.access.metadata_path())?;
        let git = loader
            .store()
            .backend()
            .downcast_ref::<jj_lib::git_backend::GitBackend>()
            .ok_or(Error::Invalid)?;
        let expected = &lease.expected;
        let value = Self {
            version: 1,
            profile: local_browser::PROFILE.into(),
            repository: lease.access.repository(),
            generation: lease.access.generation(),
            physical_workspace: expected.physical_workspace,
            operation: expected.baseline.operation,
            view: expected.baseline.view,
            commit: expected.baseline.commit,
            checkout: expected.checkout.clone(),
            fingerprint: expected.fingerprint,
            git_path: git.git_repo_path().to_path_buf(),
            source_binding: binding(lease.access.metadata_path(), git.git_repo_path(), budget)?,
            heads: expected.heads.clone(),
        };
        value.encode()?;
        Ok(value)
    }
    pub(super) fn check_key(&self, key: &StackPreparationKey) -> Result<()> {
        self.encode()?;
        if key.repository != self.repository || key.physical_workspace != self.physical_workspace {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    pub(super) fn check_archive(&self, archive: &OperationArchiveV1) -> Result<()> {
        archive.validate().map_err(evidence_error)?;
        let op = archive
            .operations
            .get(&self.operation)
            .ok_or(Error::Incomplete)?;
        let view = archive.views.get(&self.view).ok_or(Error::Incomplete)?;
        let commit = archive.commits.get(&self.commit).ok_or(Error::Incomplete)?;
        if archive.repository != self.repository
            || op.view_id != self.view
            || view.wc_commit_ids.get(&self.checkout.workspace) != Some(&self.commit)
            || commit.root_tree != self.checkout.trees
            || jj_lib::conflict_labels::ConflictLabels::from_merge(jj_lib::merge::Merge::from_vec(
                commit.conflict_labels.clone(),
            ))
            .as_slice()
                != self.checkout.labels
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    pub(super) fn git_path(&self) -> &std::path::Path {
        &self.git_path
    }
    pub(super) fn store(&self) -> Result<MetadataStore> {
        MetadataStore::open(&self.git_path, Default::default()).map_err(evidence_error)
    }
    pub(super) async fn check(
        &self,
        access: &Arc<dyn Access>,
        root: &std::path::Path,
        budget: &Budget,
    ) -> Result<()> {
        budget.until(access.authorize()).await??;
        if access.repository() != self.repository || access.generation() != self.generation {
            return Err(Error::EpochChanged);
        }
        for _ in 0..2 {
            let files = capture::CheckoutFiles::read(root, access.metadata_path(), budget)?;
            let git = gix::open(&self.git_path).map_err(|_| Error::Unavailable)?;
            let policy = capture::policy_at_git(root, &git, &self.git_path, budget)?;
            let heads = heads(access.metadata_path(), budget)?;
            if files.workspace != self.physical_workspace
                || heads != self.heads
                || current::fingerprint(access, &files, &policy, &heads) != self.fingerprint
                || binding(access.metadata_path(), &self.git_path, budget)? != self.source_binding
            {
                return Err(Error::EpochChanged);
            }
        }
        budget.until(access.authorize()).await??;
        Ok(())
    }
}
fn identity(dir: &File, hash: &mut Sha256) -> Result<()> {
    let info = rustix::fs::fstat(dir).map_err(|_| Error::Unavailable)?;
    hash.update((info.st_dev as u64).to_be_bytes());
    hash.update(info.st_ino.to_be_bytes());
    Ok(())
}
fn binding(
    metadata: &std::path::Path,
    git: &std::path::Path,
    budget: &Budget,
) -> Result<m::RecordDigest> {
    let repo = physical::directory(metadata)?;
    let store = physical::child(&repo, "store")?;
    let op_heads = physical::child(&repo, "op_heads")?;
    let mut hash = Sha256::new_with_prefix(b"jj-store/stack-recovery-source-binding/v1\0");
    for dir in [
        &store,
        &op_heads,
        &physical::child(&op_heads, "heads")?,
        &physical::directory(git)?,
    ] {
        identity(dir, &mut hash)?;
    }
    for (dir, name, max, optional) in [
        (&store, "type", 64, false),
        (&store, "git_target", 4096, true),
        (&op_heads, "type", 64, false),
    ] {
        match physical::read(dir, name, max, optional, budget)? {
            Some(bytes) => {
                hash.update([1]);
                hash.update(bytes.fingerprint.as_bytes());
            }
            None => hash.update([0]),
        }
    }
    Ok(m::RecordDigest::new(hash.finalize().into()))
}
fn heads(metadata: &std::path::Path, budget: &Budget) -> Result<Vec<m::OperationId>> {
    let repo = physical::directory(metadata)?;
    let dir = physical::child(&physical::child(&repo, "op_heads")?, "heads")?;
    let mut heads = Vec::new();
    for entry in rustix::fs::Dir::read_from(&dir).map_err(|_| Error::Unavailable)? {
        let entry = entry.map_err(|_| Error::Unavailable)?;
        let name = entry.file_name().to_str().map_err(|_| Error::Invalid)?;
        if matches!(name, "." | "..") {
            continue;
        }
        if name.len() != 128
            || !name
                .bytes()
                .all(|v| v.is_ascii_digit() || (b'a'..=b'f').contains(&v))
        {
            return Err(Error::Invalid);
        }
        physical::read(&dir, name, 0, false, budget)?.ok_or(Error::Unavailable)?;
        heads.push(name.parse().map_err(|_| Error::Invalid)?);
        if heads.len() > 64 {
            return Err(Error::Limit);
        }
    }
    heads.sort();
    Ok(heads)
}
