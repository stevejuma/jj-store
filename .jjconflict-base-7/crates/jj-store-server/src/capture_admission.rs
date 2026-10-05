//! Capture directory admission; registry membership is not publication authorization.
use jj_store_service::{
    local_management::{VerifiedCaptureWorkspace, WorkspaceProfile, WorkspaceVerifier},
    registry::RepositoryRegistry,
};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("capture directory has no registered native binding")]
    Unbound,
    #[error("capture binding is unavailable")]
    Unavailable,
    #[error("capture directory discovery resource limit")]
    Limit,
    #[error("capture binding changed; publication withheld")]
    Changed,
}

#[derive(Clone)]
pub struct CaptureAdmission {
    registry: Arc<dyn RepositoryRegistry>,
    verifier: Arc<dyn WorkspaceVerifier>,
    owned_io: bool,
}
impl CaptureAdmission {
    pub fn new(
        registry: Arc<dyn RepositoryRegistry>,
        verifier: Arc<dyn WorkspaceVerifier>,
    ) -> Self {
        Self {
            registry,
            verifier,
            owned_io: false,
        }
    }
    pub(crate) fn for_owned_worker(mut self) -> Self {
        self.owned_io = true;
        self
    }
    pub(crate) async fn source_io<T: Send + 'static>(
        &self,
        work: impl FnOnce() -> T + Send + 'static,
    ) -> Result<T, ()> {
        if self.owned_io {
            Ok(work())
        } else {
            tokio::task::spawn_blocking(work).await.map_err(|_| ())
        }
    }
    pub async fn inspect(&self, directory: &str) -> Result<VerifiedCaptureWorkspace, Error> {
        if directory.len() > jj_store_service::local_management::MAX_PATH_BYTES {
            return Err(Error::Limit);
        }
        let verifier = self.verifier.clone();
        let directory = directory.to_owned();
        let proof = self
            .source_io(move || verifier.inspect_capture_directory(&directory))
            .await
            .map_err(|_| Error::Unavailable)?
            .map_err(|error| match error {
                jj_store_service::local_management::Error::Unavailable => Error::Unavailable,
                jj_store_service::local_management::Error::Limit => Error::Limit,
                _ => Error::Unbound,
            })?;
        if proof.workspace.profile != WorkspaceProfile::Native {
            return Err(Error::Unbound);
        }
        let registered = self
            .registry
            .snapshot(None)
            .await
            .map_err(|_| Error::Unavailable)?;
        if !registered
            .repositories
            .iter()
            .any(|repository| repository.id == proof.workspace.repository)
        {
            return Err(Error::Unbound);
        }
        Ok(proof)
    }
    /// Resolve this batch's reconstructed directory, never the session's old header cwd.
    pub async fn inspect_batch(
        &self,
        batch: &jj_store_capture::codex_transcript::CodexTranscriptBatch,
    ) -> Result<VerifiedCaptureWorkspace, Error> {
        let directory = batch
            .directory()
            .and_then(|path| path.to_str())
            .ok_or(Error::Unbound)?;
        self.inspect(directory).await
    }
    /// Admission consumes the directory observed with new activity, not the old header.
    pub async fn inspect_activity(
        &self,
        active: &jj_store_capture::codex_activity::ActiveCodexSource,
    ) -> Result<VerifiedCaptureWorkspace, Error> {
        active.revalidate().map_err(|_| Error::Changed)?;
        let directory = active.directory().to_str().ok_or(Error::Unbound)?;
        let proof = self.inspect(directory).await?;
        active.revalidate().map_err(|_| Error::Changed)?;
        Ok(proof)
    }
    /// Repository/activity checks precede durable source allocation and reader locks.
    /// Publication must separately enforce the current service write policy.
    pub async fn acquire_active_source(
        &self,
        sources: Arc<jj_store_capture::source_registry::SourceRegistry>,
        active: jj_store_capture::codex_activity::ActiveCodexSource,
    ) -> Result<
        (
            jj_store_capture::source_registry::RegisteredCodexSource,
            VerifiedCaptureWorkspace,
        ),
        Error,
    > {
        let proof = self.inspect_activity(&active).await?;
        self.revalidate(&proof).await?;
        let registered = self
            .source_io(move || sources.acquire_active(active))
            .await
            .map_err(|_| Error::Unavailable)?
            .map_err(|_| Error::Changed)?;
        self.revalidate(&proof).await?;
        Ok((registered, proof))
    }
    /// Current directory proof and registry membership must both still match.
    /// The publication service must additionally enforce current write policy.
    pub async fn revalidate(&self, observed: &VerifiedCaptureWorkspace) -> Result<(), Error> {
        let current = self.inspect(&observed.directory).await?;
        if current != *observed {
            return Err(Error::Changed);
        }
        Ok(())
    }
}
