//! In-process LocalTrusted collaboration facade. Caller composition owns folder
//! admission; filesystem permissions remain the local access boundary.
use jj_store_git::records;
use jj_store_git::repository::migration::{
    agent_descriptor, agent_evidence, agent_io, collab_external, collab_graph, review_io, work,
};
pub use jj_store_git::repository::{
    LocalAgentCommand, LocalAgentObjectPage, LocalAgentObjectRow, LocalAgentOutcome,
    LocalAgentPage, LocalAgentReceipt, LocalAgentRequest, LocalAgentSessionRow, LocalAgentSnapshot,
    LocalAgentStore, LocalCodeEvidenceV1, LocalCollabCommand, LocalCollabOutcome,
    LocalCollabReceipt, LocalCollabRequest, LocalCollabResultV1, LocalCollabSnapshot,
    LocalEmojiPageV1, LocalError, LocalProfile, LocalReactionPageV1, LocalReactionRowV1,
    LocalReactionStore, LocalResult, LocalReviewCommand, LocalReviewOutcome, LocalReviewReceipt,
    LocalReviewRequest, LocalReviewSnapshot, LocalReviewStore, LocalThreadStore, LocalWorkOutcome,
    LocalWorkReceipt, LocalWorkSnapshot,
};
mod agent_feedback;
mod browser_agents;
mod browser_collab;
mod conversion;
mod conversion_import;
pub use browser_agents::*;
pub use browser_collab::*;
mod browser_reviews;
pub use browser_reviews::*;
mod browser_work;
pub use browser_work::{LocalWorkActivityPage, LocalWorkBrowsePage};
pub use jj_store_jj::local_metadata::GitOid as MetadataCommit;
mod work_cache;
mod work_pages;
use jj_store_jj::local_metadata::{self as metadata, MetadataStore};
use jj_store_model as m;
use std::path::{Path, PathBuf};
pub use work_pages::LocalWorkPage;

#[derive(Clone, Debug)]
pub struct LocalRepository {
    inner: Box<jj_store_git::LocalRepository>,
    pub(crate) metadata: MetadataStore,
    pub(crate) repo_path: PathBuf,
    pub(crate) expected_repository: Option<m::RepositoryId>,
}
impl LocalRepository {
    pub fn new(repo_path: &Path, metadata: MetadataStore) -> Self {
        Self {
            inner: Box::new(jj_store_git::LocalRepository::new(
                repo_path,
                metadata.carrier().clone(),
            )),
            repo_path: repo_path.to_path_buf(),
            metadata,
            expected_repository: None,
        }
    }
    /// Bind an admitted device identity; CAS retries must never follow a replacement.
    pub fn bound_to(mut self, repository: m::RepositoryId) -> Self {
        self.inner = Box::new(self.inner.bound_to(repository));
        self.expected_repository = Some(repository);
        self
    }
    pub(crate) fn check_identity(
        &self,
        selected: Option<&metadata::Snapshot>,
    ) -> jj_store_git::work_api::Result<()> {
        self.inner.check_identity(selected)
    }
    pub fn metadata_store(&self) -> &MetadataStore {
        &self.metadata
    }
    pub fn repository(&self) -> crate::work::Result<Option<m::RepositoryId>> {
        Ok(self
            .metadata
            .read()
            .map_err(map_metadata)?
            .map(|s| s.repository()))
    }
    pub(crate) fn carrier(&self) -> &jj_store_git::LocalRepository {
        &self.inner
    }
    pub(crate) fn work_snapshot(snapshot: Option<&metadata::Snapshot>) -> LocalWorkSnapshot {
        jj_store_git::LocalRepository::work_snapshot(snapshot)
    }
    pub fn apply_work_command(
        &self,
        request: &crate::work::Request,
    ) -> crate::work::Result<LocalWorkOutcome> {
        Ok(self.inner.apply_work_command(request)?)
    }
    pub fn work_command_status(
        &self,
        request: m::RequestId,
    ) -> crate::work::Result<LocalWorkOutcome> {
        Ok(self.inner.work_command_status(request)?)
    }
}
pub(crate) fn map_metadata(error: metadata::Error) -> crate::work::Error {
    use crate::work::Error;
    match error {
        metadata::Error::Stale => Error::Stale,
        metadata::Error::IntentConflict => Error::IntentConflict,
        metadata::Error::Limit => Error::Limit,
        metadata::Error::Invalid(_) => Error::Corrupt,
        _ => Error::Unavailable,
    }
}

impl std::ops::Deref for LocalRepository {
    type Target = jj_store_git::LocalRepository;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
