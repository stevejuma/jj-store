//! Metadata selects ThreadStore records through the existing publication authorities.
use super::metadata_sources::Capture;
use super::*;
use super::{Error, Result};
use jj_collab::*;
use jj_store_service::collab_mutations::{CollabCommandService, Request};
impl Supervisor {
    pub(super) async fn metadata_recover(
        &self,
        job: super::metadata_jobs::SavedJob,
    ) -> Result<jj_store_service::agent_execution::metadata::Job> {
        let id = job.command.request_id;
        match self
            .metadata_publish(
                &job.capture,
                job.publication.as_ref().ok_or(Error::Invalid)?,
            )
            .await
        {
            Ok(()) => {
                self.update(|s| {
                    let j = s.metadata_jobs.get_mut(&id).ok_or(Error::Stale)?;
                    j.view.state = "completed".into();
                    j.view.error = None;
                    j.capture.transcript.clear();
                    j.config = None;
                    j.publication = None;
                    Ok(())
                })?;
            }
            Err(Error::Stale | Error::Invalid | Error::Conflict | Error::Limit) => {
                // A receipt retry can establish a definitive refusal. Retain the
                // result for a new explicit publication; never repeat inference.
                self.update(|s| {
                    let j = s.metadata_jobs.get_mut(&id).ok_or(Error::Stale)?;
                    j.view.state = "failed".into();
                    j.view.error = Some("metadata_publication_refused".into());
                    j.capture.transcript.clear();
                    j.config = None;
                    Ok(())
                })?;
            }
            Err(Error::Denied) => return Err(Error::Denied),
            Err(_) => {}
        }
        self.execution_changed();
        self.read(|s| Ok(s.metadata_jobs[&id].view.clone()))
    }
    pub(super) async fn metadata_owner(&self, capture: &mut Capture) -> Result<ThreadId> {
        if let Some(owner) = capture.owner {
            return Ok(owner);
        }
        let identity = publication::identity(&format!(
            "conversation-metadata-owner/{}/{}",
            capture.repository,
            capture.target.key().map_err(|_| Error::Invalid)?
        ));
        let owner = ThreadId::new(identity);
        let request = Request {
            version: 1,
            request_id: RequestId::new(identity),
            command: CollabCommandV1::Thread {
                intent: ThreadCommandV1::Create {
                    thread_id: owner,
                    title: "Conversation metadata".into(),
                    baseline: None,
                },
            },
        };
        // A repeated explicit operation recovers its exact creation receipt.
        self.metadata_publish(capture, &request).await?;
        capture.owner = Some(owner);
        Ok(owner)
    }
    pub(super) async fn metadata_request(
        &self,
        capture: &mut Capture,
        request_id: RequestId,
        value: ConversationMetadataValueV1,
    ) -> Result<Request> {
        let owner = self.metadata_owner(capture).await?;
        let expected_thread = if capture.profile == "local_jj_git" {
            let local = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(capture.repository)
                .map_err(|_| Error::Denied)?;
            let pin = local
                .inbox_selection()
                .await
                .map_err(|_| Error::Unavailable)?;
            local
                .inbox_thread(pin.snapshot.metadata_commit, owner)
                .await
                .map_err(|_| Error::Unavailable)?
                .thread
                .reference
        } else {
            let session = self.home_session(capture.repository)?;
            let pin = jj_store_service::collab_reads::PinnedCollabRead::open(
                session.clone(),
                Arc::new(jj_store_service::collab_reads::RepositoryMemberReactionActors),
            )
            .await
            .map_err(|_| Error::Unavailable)?;
            pin.thread(session, owner)
                .await
                .map_err(|_| Error::Unavailable)?
                .reference
        };
        Ok(Request {
            version: 1,
            request_id,
            command: CollabCommandV1::Thread {
                intent: ThreadCommandV1::SetMetadata {
                    thread_id: owner,
                    expected_thread,
                    expected_metadata: capture.revision,
                    target: capture.target.clone(),
                    value: Box::new(value),
                },
            },
        })
    }
    pub(super) async fn metadata_publish(
        &self,
        capture: &Capture,
        request: &Request,
    ) -> Result<()> {
        self.authorized(capture.repository, &capture.profile, true)
            .await?;
        if !capture.writable {
            return Err(Error::Denied);
        }
        if capture.profile == "local_jj_git" {
            let local = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(capture.repository)
                .map_err(|_| Error::Denied)?;
            let outcome = local
                .apply_collab_command(&jj_store_service::local_repository::LocalCollabRequest {
                    version: 1,
                    request_id: request.request_id,
                    command: request.command.clone(),
                })
                .await
                .map_err(|e| match e {
                    jj_store_service::local_repository::LocalError::Stale => Error::Stale,
                    jj_store_service::local_repository::LocalError::IntentConflict => {
                        Error::Conflict
                    }
                    jj_store_service::local_repository::LocalError::Invalid
                    | jj_store_service::local_repository::LocalError::NotFound
                    | jj_store_service::local_repository::LocalError::Unsupported => Error::Invalid,
                    jj_store_service::local_repository::LocalError::Limit => Error::Limit,
                    _ => Error::Unavailable,
                })?;
            if !matches!(
                outcome,
                jj_store_service::local_repository::LocalCollabOutcome::Committed { .. }
            ) {
                return Err(Error::Unavailable);
            }
        } else {
            let outcome = CollabCommandService
                .apply(self.home_session(capture.repository)?, request)
                .await
                .map_err(|e| match e {
                    jj_store_service::collab_mutations::Error::Stale => Error::Stale,
                    jj_store_service::collab_mutations::Error::IntentConflict => Error::Conflict,
                    jj_store_service::collab_mutations::Error::Denied => Error::Denied,
                    jj_store_service::collab_mutations::Error::Invalid
                    | jj_store_service::collab_mutations::Error::NotFound
                    | jj_store_service::collab_mutations::Error::Unsupported => Error::Invalid,
                    jj_store_service::collab_mutations::Error::Limit => Error::Limit,
                    _ => Error::Unavailable,
                })?;
            if !matches!(
                outcome,
                jj_store_service::collab_mutations::Outcome::Committed { .. }
            ) {
                return Err(Error::Unavailable);
            }
        }
        Ok(())
    }
}
