//! In-process LocalTrusted collaboration facade. Caller composition owns folder
//! admission; filesystem permissions remain the local access boundary.
mod actor;
mod agent_commands;
mod agent_descriptor;
mod agent_evidence;
mod agent_io;
mod agent_pages;
mod agent_provenance_pages;
mod agent_store;
mod agent_thread_binding;
mod collab_commands;
mod collab_external;
mod collab_graph;
mod collab_store;
mod collab_types;
mod conversion;
mod errors;
mod records;
mod review_agent_evidence;
mod review_commands;
mod review_io;
mod review_store;
mod review_types;
pub use agent_commands::{
    LocalAgentCommand, LocalAgentOutcome, LocalAgentReceipt, LocalAgentRequest,
};
pub use agent_evidence::LocalCodeEvidenceV1;
pub use agent_pages::{LocalAgentPage, LocalAgentSessionRow};
pub use agent_provenance_pages::{LocalAgentObjectPage, LocalAgentObjectRow};
pub use agent_store::{LocalAgentSnapshot, LocalAgentStore};
pub use collab_store::{
    LocalCollabSnapshot, LocalEmojiPageV1, LocalReactionPageV1, LocalReactionRowV1,
    LocalReactionStore, LocalThreadStore,
};
pub use collab_types::*;
pub use errors::{LocalError, LocalResult};
pub use review_store::{LocalReviewSnapshot, LocalReviewStore};
pub use review_types::{
    LocalReviewCommand, LocalReviewOutcome, LocalReviewReceipt, LocalReviewRequest,
};
mod browser_agents;
pub use browser_agents::*;
mod browser_reviews;
pub use browser_reviews::*;
mod browser_work;
mod selection;
pub use browser_work::LocalWorkBrowsePage;
pub use jj_store_jj::local_metadata::GitOid as MetadataCommit;
mod work;
mod work_pages;
pub use work::*;
pub use work_pages::LocalWorkPage;
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum LocalProfile {
    LocalJjGit,
}
use jj_store_jj::local_metadata::{self as metadata, MetadataStore};
use jj_store_model as m;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct LocalRepository {
    pub(crate) metadata: MetadataStore,
    pub(crate) repo_path: PathBuf,
    pub(crate) expected_repository: Option<m::RepositoryId>,
}
impl LocalRepository {
    pub fn new(repo_path: &Path, metadata: MetadataStore) -> Self {
        Self {
            repo_path: repo_path.to_path_buf(),
            metadata,
            expected_repository: None,
        }
    }
    /// Bind an admitted device identity; CAS retries must never follow a replacement.
    pub fn bound_to(mut self, repository: m::RepositoryId) -> Self {
        self.expected_repository = Some(repository);
        self
    }
    pub(crate) fn check_identity(
        &self,
        selected: Option<&metadata::Snapshot>,
    ) -> crate::work::Result<()> {
        if self
            .expected_repository
            .is_some_and(|id| selected.is_none_or(|s| s.repository() != id))
        {
            return Err(crate::work::Error::Stale);
        }
        Ok(())
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
    pub(crate) fn proposed_repository() -> crate::work::Result<m::RepositoryId> {
        let mut bytes = [0; 16];
        getrandom::fill(&mut bytes).map_err(|_| crate::work::Error::Unavailable)?;
        Ok(m::RepositoryId::new(bytes))
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
