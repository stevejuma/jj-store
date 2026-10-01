//! Exact Git evidence and bounded conversion through the pinned private backend.
mod objects;
mod trees;
use crate::{
    codec::NativeSymlink,
    objects::{NativeObjects, UploadStatus},
};
use futures::{FutureExt as _, future::BoxFuture};
use jj_lib::{
    backend::{self as b, Backend as _},
    git_backend::GitBackend,
    object_id::ObjectId as _,
    repo_path::RepoPath,
    settings::UserSettings,
};
use jj_store_model as m;
use jj_store_wal::inventory::DependencySource as _;
use jj_store_wal::publication::{GIT_PROFILE, GitRecordV1, GitRepresentationV1, GitStoreSnapshot};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::Arc,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[derive(Clone, Copy)]
pub struct Limits {
    pub objects: usize,
    pub bytes: u64,
    pub object_bytes: u64,
    pub depth: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            objects: 100_000,
            bytes: 256 * 1024 * 1024,
            object_bytes: 32 * 1024 * 1024,
            depth: 512,
        }
    }
}
pub struct GitConverter {
    backend: GitBackend,
    native: Arc<NativeObjects>,
    limits: Limits,
    bytes: u64,
    pub records: BTreeMap<m::RecordDigest, GitRecordV1>,
    imported: BTreeMap<String, m::CommitId>,
    exported: BTreeMap<m::CommitId, String>,
    trees: BTreeMap<String, b::TreeId>,
    restored: BTreeSet<String>,
}
impl GitConverter {
    pub fn open(
        settings: &UserSettings,
        state: &Path,
        bare: &Path,
        native: Arc<NativeObjects>,
        limits: Limits,
        accepted: Option<&GitStoreSnapshot>,
    ) -> Result<Self> {
        if accepted.is_some_and(|v| v.repository != native.scope().repository()) {
            return Err("Git representation repository mismatch".into());
        }
        if limits.objects == 0 || limits.object_bytes == 0 || limits.depth == 0 {
            return Err("Git conversion limits".into());
        }
        let repo = gix::open(bare)?;
        if repo.object_hash() != gix::hash::Kind::Sha1 || repo.is_shallow() {
            return Err("unsupported Git object format or shallow history".into());
        }
        if !jj_lib::git::GitSettings::from_settings(settings)?.write_change_id_header {
            return Err("Git profile requires change-id headers".into());
        }
        std::fs::create_dir_all(state)?;
        let backend = GitBackend::init_external(settings, state, bare)?;
        Ok(Self {
            backend,
            native,
            limits,
            bytes: 0,
            records: accepted.map(|v| v.values.clone()).unwrap_or_default(),
            imported: BTreeMap::new(),
            exported: BTreeMap::new(),
            trees: BTreeMap::new(),
            restored: BTreeSet::new(),
        })
    }
    fn record(&mut self, record: GitRecordV1) -> Result<()> {
        let key = record.key()?;
        if let Some(old) = self.records.get(&key) {
            if old != &record {
                return Err("Git representation disagreement".into());
            }
            return Ok(());
        }
        if self.records.len() >= self.limits.objects {
            return Err("Git object limit".into());
        }
        self.records.insert(key, record);
        Ok(())
    }
    pub fn import_commit(&mut self, oid: String) -> BoxFuture<'_, Result<m::CommitId>> {
        self.import_at(oid, 0)
    }
    fn import_at(&mut self, oid: String, depth: usize) -> BoxFuture<'_, Result<m::CommitId>> {
        async move {
            if depth >= self.limits.depth {
                return Err("Git ancestry depth limit".into());
            }
            if let Some(id) = self.imported.get(&oid) {
                return Ok(*id);
            }
            let mapped = self
                .records
                .values()
                .filter_map(|r| match r {
                    GitRecordV1::Representation(v) if v.git_commit == oid => Some(v.native_commit),
                    _ => None,
                })
                .collect::<BTreeSet<_>>();
            if mapped.len() > 1 {
                return Err("ambiguous Git import representation".into());
            }
            if let Some(id) = mapped.first() {
                self.imported.insert(oid, *id);
                return Ok(*id);
            }
            self.capture(oid.clone(), depth).await?;
            let git_id =
                b::CommitId::try_from_hex(oid.as_bytes()).ok_or("invalid Git commit OID")?;
            let mut commit = self.backend.read_commit(&git_id).await?;
            if !commit.root_tree.is_resolved() {
                return Err("Git conflict representation is unsupported".into());
            }
            let mut parents = Vec::new();
            for parent in &commit.parents {
                if parent == self.backend.root_commit_id() {
                    parents.push(b::CommitId::from_bytes(&[0; 64]));
                } else {
                    parents.push(b::CommitId::from_bytes(
                        self.import_at(parent.hex(), depth + 1).await?.as_bytes(),
                    ));
                }
            }
            let tree = self
                .import_tree(commit.root_tree.as_resolved().unwrap().clone(), depth + 1)
                .await?;
            commit.parents = parents;
            commit.root_tree = jj_lib::merge::Merge::resolved(tree);
            let (key, status) = self.native.put(&commit).await?;
            acknowledged(status)?;
            let m::ObjectKey::Commit(id) = key else {
                return Err("native commit kind".into());
            };
            self.record(GitRecordV1::Representation(GitRepresentationV1 {
                native_commit: id,
                git_commit: oid.clone(),
                profile: GIT_PROFILE.into(),
                imported_exact: true,
            }))?;
            self.imported.insert(oid, id);
            Ok(id)
        }
        .boxed()
    }
    pub fn export_commit(&mut self, id: m::CommitId) -> BoxFuture<'_, Result<String>> {
        self.export_at(id, 0)
    }
    fn export_at(&mut self, id: m::CommitId, depth: usize) -> BoxFuture<'_, Result<String>> {
        async move {
            if depth >= self.limits.depth {
                return Err("native ancestry depth limit".into());
            }
            if let Some(oid) = self.exported.get(&id) {
                return Ok(oid.clone());
            }
            let mapped = self
                .records
                .values()
                .filter_map(|r| match r {
                    GitRecordV1::Representation(v) if v.native_commit == id => {
                        Some(v.git_commit.clone())
                    }
                    _ => None,
                })
                .collect::<BTreeSet<_>>();
            if mapped.len() > 1 {
                return Err("ambiguous native Git representation".into());
            }
            if let Some(oid) = mapped.first() {
                let oid = oid.clone();
                self.restore(oid.clone(), depth).await?;
                self.exported.insert(id, oid.clone());
                return Ok(oid);
            }
            let mut commit = self
                .native
                .get::<b::Commit>(m::ObjectKey::Commit(id))
                .await?;
            if commit.secure_sig.is_some()
                || !commit.root_tree.is_resolved()
                || !commit.conflict_labels.is_resolved()
            {
                return Err("native signature or conflict export unsupported".into());
            }
            let mut parents = Vec::new();
            for parent in &commit.parents {
                if parent.as_bytes() == [0; 64] {
                    parents.push(self.backend.root_commit_id().clone());
                } else {
                    let oid = self
                        .export_at(m::CommitId::from_slice(parent.as_bytes())?, depth + 1)
                        .await?;
                    parents.push(
                        b::CommitId::try_from_hex(oid.as_bytes())
                            .ok_or("invalid Git parent OID")?,
                    );
                }
            }
            let tree = self
                .export_tree(commit.root_tree.as_resolved().unwrap().clone(), depth + 1)
                .await?;
            commit.parents = parents;
            commit.root_tree = jj_lib::merge::Merge::resolved(tree);
            // Native evolution is not part of the Git projection or cache extras.
            commit.predecessors.clear();
            let (oid, _) = self.backend.write_commit(commit, None).await?;
            let oid = oid.hex();
            self.capture(oid.clone(), depth).await?;
            self.record(GitRecordV1::Representation(GitRepresentationV1 {
                native_commit: id,
                git_commit: oid.clone(),
                profile: GIT_PROFILE.into(),
                imported_exact: false,
            }))?;
            self.exported.insert(id, oid.clone());
            Ok(oid)
        }
        .boxed()
    }
}
fn acknowledged(status: UploadStatus) -> Result<()> {
    if status == UploadStatus::Unknown {
        Err("native Git object upload outcome unknown".into())
    } else {
        Ok(())
    }
}
