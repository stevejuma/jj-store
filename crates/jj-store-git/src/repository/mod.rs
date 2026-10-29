use crate::records;
mod actor;
mod agent_commands;
#[doc(hidden)]
pub mod agent_descriptor;
#[doc(hidden)]
pub mod agent_evidence;
#[doc(hidden)]
pub mod agent_io;
mod agent_pages;
mod agent_provenance_pages;
mod agent_store;
mod agent_thread_binding;
mod collab_commands;
#[doc(hidden)]
pub mod collab_external;
#[doc(hidden)]
pub mod collab_graph;
mod collab_store;
mod collab_types;
mod errors;
mod review_agent_evidence;
mod review_commands;
#[doc(hidden)]
pub mod review_io;
mod review_store;
mod review_types;
mod selection;
#[doc(hidden)]
pub mod work;
pub use crate::metadata::GitOid as MetadataCommit;
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
pub use work::*;
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum LocalProfile {
    LocalJjGit,
}
use crate::metadata::{self as metadata, MetadataStore};
use jj_store_model as m;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct LocalRepository {
    pub(crate) host: Option<std::sync::Arc<dyn jj_store_core::StoreHost>>,
    pub(crate) metadata: MetadataStore,
    pub(crate) repo_path: PathBuf,
    pub(crate) expected_repository: Option<m::RepositoryId>,
}
impl LocalRepository {
    pub fn new(repo_path: &Path, metadata: MetadataStore) -> Self {
        Self {
            host: None,
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
    pub fn check_identity(
        &self,
        selected: Option<&metadata::Snapshot>,
    ) -> crate::work_api::Result<()> {
        if self
            .expected_repository
            .is_some_and(|id| selected.is_none_or(|s| s.repository() != id))
        {
            return Err(crate::work_api::Error::Stale);
        }
        Ok(())
    }
    pub fn metadata_store(&self) -> &MetadataStore {
        &self.metadata
    }
    pub fn repository(&self) -> crate::work_api::Result<Option<m::RepositoryId>> {
        Ok(self
            .metadata
            .read()
            .map_err(map_metadata)?
            .map(|s| s.repository()))
    }
    pub(crate) fn proposed_repository() -> crate::work_api::Result<m::RepositoryId> {
        let mut bytes = [0; 16];
        getrandom::fill(&mut bytes).map_err(|_| crate::work_api::Error::Unavailable)?;
        Ok(m::RepositoryId::new(bytes))
    }
}
pub(crate) fn map_metadata(error: metadata::Error) -> crate::work_api::Error {
    use crate::work_api::Error;
    match error {
        metadata::Error::Stale => Error::Stale,
        metadata::Error::IntentConflict => Error::IntentConflict,
        metadata::Error::Limit => Error::Limit,
        metadata::Error::Invalid(_) => Error::Corrupt,
        _ => Error::Unavailable,
    }
}

/// Low-level immutable staging capabilities for explicit repository migration.
/// These never select a root; callers still use metadata prepare and ref CAS.
#[doc(hidden)]
pub mod migration {
    pub use super::{
        agent_descriptor, agent_evidence, agent_io, collab_external, collab_graph, review_io, work,
    };
}

impl std::fmt::Debug for LocalRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalRepository")
            .field("metadata", &self.metadata)
            .field("repo_path", &self.repo_path)
            .field("expected_repository", &self.expected_repository)
            .finish_non_exhaustive()
    }
}
impl LocalRepository {
    pub(crate) fn with_host(mut self, host: std::sync::Arc<dyn jj_store_core::StoreHost>) -> Self {
        self.host = Some(host);
        self
    }
    pub(crate) fn now_millis(&self) -> crate::work_api::Result<i64> {
        if let Some(host) = &self.host {
            return host.now_millis().map_err(Into::into);
        }
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| crate::work_api::Error::Unavailable)?
            .as_millis()
            .try_into()
            .map_err(|_| crate::work_api::Error::Limit)
    }
}

impl LocalRepository {
    /// Admit a domain read only from current or retained committed companion history.
    /// Low-level read_at candidates are deliberately insufficient for this facade.
    pub(crate) fn check_selected(&self, selected: Option<&metadata::Snapshot>) -> LocalResult<()> {
        self.check_identity(selected)?;
        let Some(selected) = selected else {
            return Ok(());
        };
        let current = self.metadata.read()?.ok_or(LocalError::Stale)?;
        self.check_identity(Some(&current))?;
        if current.commit() == selected.commit() {
            return Ok(());
        }
        if self
            .metadata
            .history(&current)?
            .iter()
            .any(|p| p.commit() == selected.commit())
        {
            Ok(())
        } else {
            Err(LocalError::Stale)
        }
    }
}
