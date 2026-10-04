//! Read-only Git traversal from proven operation/view references. Each followed
//! edge grants only its exact immutable target; raw object existence grants none.
use super::*;
use futures::{AsyncRead, stream::BoxStream};
use jj_lib::{
    index::Index,
    repo_path::{RepoPath, RepoPathBuf},
    store::Store,
};
use std::{pin::Pin as IoPin, sync::Mutex, time::SystemTime};

pub(super) struct Guard {
    pub source: Arc<Store>,
    binding: Arc<dyn Access>,
    budget: Arc<Budget>,
    allowed: Mutex<BTreeSet<ObjectKey>>,
    commits: Mutex<BTreeSet<m::GitSha1Id>>,
    max_commits: usize,
}
impl std::fmt::Debug for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScopedGitRead").finish_non_exhaustive()
    }
}
fn error(value: Error) -> b::BackendError {
    b::BackendError::Other(Box::new(value))
}
fn unsupported<T>() -> b::BackendResult<T> {
    Err(error(Error::Denied))
}
impl Guard {
    pub fn new(source: Arc<Store>, pin: &Pin, budget: Arc<Budget>, limits: Limits) -> Arc<Self> {
        Arc::new(Self {
            source,
            binding: pin.binding.clone(),
            budget,
            allowed: Mutex::new(pin.inventory.clone()),
            commits: Mutex::new(BTreeSet::new()),
            max_commits: limits.max_history_visits,
        })
    }
    pub fn contains(&self, key: ObjectKey) -> bool {
        self.allowed
            .lock()
            .expect("scoped membership")
            .contains(&key)
    }
    async fn admit(&self, key: ObjectKey, path: Option<&RepoPath>) -> Result<()> {
        self.budget.work(1)?;
        self.budget.until(self.binding.authorize()).await??;
        if let Some(path) = path {
            native::path(path.as_internal_file_string())?;
        }
        if !self.contains(key) {
            return Err(Error::NotCommitted);
        }
        Ok(())
    }
    fn grant(&self, keys: impl IntoIterator<Item = ObjectKey>) -> Result<()> {
        let mut allowed = self.allowed.lock().expect("scoped membership");
        for key in keys {
            self.budget.work(1)?;
            allowed.insert(key);
        }
        Ok(())
    }
    pub fn store(self: &Arc<Self>) -> Arc<Store> {
        Store::new(
            Box::new(Scoped(self.clone())),
            jj_lib::signing::Signer::new(None, vec![]),
            self.source.merge_options().clone(),
        )
    }
}
#[derive(Debug)]
struct Scoped(Arc<Guard>);
#[async_trait::async_trait]
impl b::Backend for Scoped {
    fn name(&self) -> &str {
        "git"
    }
    fn commit_id_length(&self) -> usize {
        self.0.source.backend().commit_id_length()
    }
    fn change_id_length(&self) -> usize {
        self.0.source.backend().change_id_length()
    }
    fn root_commit_id(&self) -> &b::CommitId {
        self.0.source.backend().root_commit_id()
    }
    fn root_change_id(&self) -> &b::ChangeId {
        self.0.source.backend().root_change_id()
    }
    fn empty_tree_id(&self) -> &b::TreeId {
        self.0.source.backend().empty_tree_id()
    }
    fn concurrency(&self) -> usize {
        1
    }
    async fn read_commit(&self, id: &b::CommitId) -> b::BackendResult<b::Commit> {
        let id_owned = git_id(id).map_err(error)?;
        self.0
            .admit(ObjectKey::Commit(id_owned), None)
            .await
            .map_err(error)?;
        {
            let mut commits = self.0.commits.lock().expect("scoped commits");
            if !commits.contains(&id_owned) && commits.len() >= self.0.max_commits {
                return Err(error(Error::Limit));
            }
            commits.insert(id_owned);
        }
        bounded_git_object(
            self.0.source.backend(),
            id_owned,
            gix::object::Kind::Commit,
            &self.0.budget,
        )
        .map_err(error)?;
        let commit = self.0.source.backend().read_commit(id).await?;
        let data = dto::Commit::try_from(&commit).map_err(|_| error(Error::Corrupt))?;
        self.0.budget.metadata(&data).map_err(error)?;
        self.0
            .grant(
                data.parents
                    .into_iter()
                    .chain(data.predecessors)
                    .map(ObjectKey::Commit)
                    .chain(data.root_tree.into_iter().map(ObjectKey::Tree)),
            )
            .map_err(error)?;
        Ok(commit)
    }
    async fn read_tree(&self, path: &RepoPath, id: &b::TreeId) -> b::BackendResult<b::Tree> {
        let id_owned = git_id(id).map_err(error)?;
        self.0
            .admit(ObjectKey::Tree(id_owned), Some(path))
            .await
            .map_err(error)?;
        bounded_git_object(
            self.0.source.backend(),
            id_owned,
            gix::object::Kind::Tree,
            &self.0.budget,
        )
        .map_err(error)?;
        let tree = self.0.source.backend().read_tree(path, id).await?;
        for entry in tree.entries() {
            let key = match entry.value() {
                b::TreeValue::File { id, .. } => Some(ObjectKey::File(git_id(id).map_err(error)?)),
                b::TreeValue::Symlink(id) => Some(ObjectKey::Symlink(git_id(id).map_err(error)?)),
                b::TreeValue::Tree(id) => Some(ObjectKey::Tree(git_id(id).map_err(error)?)),
                _ => None,
            };
            if let Some(key) = key {
                self.0.grant([key]).map_err(error)?;
            }
        }
        Ok(tree)
    }
    async fn read_file(
        &self,
        path: &RepoPath,
        id: &b::FileId,
    ) -> b::BackendResult<IoPin<Box<dyn AsyncRead + Send>>> {
        let id_owned = git_id(id).map_err(error)?;
        self.0
            .admit(ObjectKey::File(id_owned), Some(path))
            .await
            .map_err(error)?;
        bounded_git_object(
            self.0.source.backend(),
            id_owned,
            gix::object::Kind::Blob,
            &self.0.budget,
        )
        .map_err(error)?;
        self.0.source.backend().read_file(path, id).await
    }
    async fn read_symlink(&self, path: &RepoPath, id: &b::SymlinkId) -> b::BackendResult<String> {
        let id_owned = git_id(id).map_err(error)?;
        self.0
            .admit(ObjectKey::Symlink(id_owned), Some(path))
            .await
            .map_err(error)?;
        bounded_git_object(
            self.0.source.backend(),
            id_owned,
            gix::object::Kind::Blob,
            &self.0.budget,
        )
        .map_err(error)?;
        self.0.source.backend().read_symlink(path, id).await
    }
    async fn read_copy(&self, _: &b::CopyId) -> b::BackendResult<b::CopyHistory> {
        unsupported()
    }
    async fn get_related_copies(&self, _: &b::CopyId) -> b::BackendResult<Vec<b::RelatedCopy>> {
        unsupported()
    }
    fn get_copy_records(
        &self,
        _: Option<&[RepoPathBuf]>,
        _: &b::CommitId,
        _: &b::CommitId,
    ) -> b::BackendResult<BoxStream<'_, b::BackendResult<b::CopyRecord>>> {
        unsupported()
    }
    async fn write_file(
        &self,
        _: &RepoPath,
        _: &mut (dyn AsyncRead + Send + Unpin),
    ) -> b::BackendResult<b::FileId> {
        unsupported()
    }
    async fn write_symlink(&self, _: &RepoPath, _: &str) -> b::BackendResult<b::SymlinkId> {
        unsupported()
    }
    async fn write_copy(&self, _: &b::CopyHistory) -> b::BackendResult<b::CopyId> {
        unsupported()
    }
    async fn write_tree(&self, _: &RepoPath, _: &b::Tree) -> b::BackendResult<b::TreeId> {
        unsupported()
    }
    async fn write_commit(
        &self,
        _: b::Commit,
        _: Option<&mut b::SigningFn>,
    ) -> b::BackendResult<(b::CommitId, b::Commit)> {
        unsupported()
    }
    fn gc(&self, _: &dyn Index, _: SystemTime) -> b::BackendResult<()> {
        unsupported()
    }
}
