//! Frozen comment publication precedes dispatch; retries retain receipt identities.
use super::*;
use super::{Error, Result};
use jj_collab::*;
use jj_store_service::{
    agent_execution::conversations as dto,
    collab_mutations::{CollabCommandService, Request},
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SavedSubmission {
    pub repository: RepositoryId,
    pub profile: String,
    pub request: Request,
    pub batch: Option<CollabObjectRef>,
    pub context: Option<context::ContextBatch>,
    #[serde(default)]
    pub refused: bool,
}
impl Supervisor {
    pub(super) fn comment_publication(
        &self,
        request: RequestId,
    ) -> Result<Option<dto::CommentPublication>> {
        self.read(|s| {
            let parent = s
                .deferred_file_prompts
                .get(&request)
                .map_or(request, |d| d.parent);
            let deferred = s
                .deferred_file_prompts
                .values()
                .find(|d| d.parent == parent);
            Ok(s.comment_submissions
                .get(&parent)
                .map(|saved| dto::CommentPublication {
                    request_id: saved.request.request_id,
                    batch: saved.batch,
                    state: if saved.batch.is_some() {
                        "published"
                    } else if saved.refused {
                        "refused"
                    } else {
                        "outcome_unknown"
                    }
                    .into(),
                })
                .or_else(|| {
                    deferred.map(|d| dto::CommentPublication {
                        request_id: RequestId::new(publication::identity(&format!(
                            "chat-comments/{}",
                            d.parent
                        ))),
                        batch: None,
                        state: if s
                            .workspace_submissions
                            .get(&parent)
                            .is_some_and(|s| s.refused)
                        {
                            "refused"
                        } else {
                            "pending"
                        }
                        .into(),
                    })
                }))
        })
    }
    pub(super) async fn publish_chat_comments(
        &self,
        command: &dto::Send,
        profile: &str,
    ) -> Result<Option<context::ContextBatch>> {
        let Some(submission) = &command.comments else {
            return Ok(None);
        };
        let conversation = self.read(|s| {
            s.conversations
                .get(&command.conversation_id)
                .cloned()
                .ok_or(Error::Stale)
        })?;
        if Some(submission.thread_id) != conversation.discussion_thread {
            return Err(Error::Invalid);
        }
        self.publish_prompt_comments(command.request_id, conversation.home, profile, submission)
            .await
            .map(Some)
    }
    pub(super) async fn publish_prompt_comments(
        &self,
        request_id: RequestId,
        repository: RepositoryId,
        profile: &str,
        submission: &ConversationSubmissionV1,
    ) -> Result<context::ContextBatch> {
        if submission.text.is_some() || submission.message_id.is_some() {
            return Err(Error::Invalid);
        }
        submission.validate().map_err(|_| Error::Invalid)?;
        let saved = self.read(|s| Ok(s.comment_submissions.get(&request_id).cloned()))?;
        let saved = if let Some(saved) = saved {
            let expected = CollabCommandV1::Thread {
                intent: ThreadCommandV1::Submit {
                    submission: Box::new(submission.clone()),
                },
            };
            if saved.repository != repository
                || saved.profile != profile
                || saved.request.command != expected
            {
                return Err(Error::Conflict);
            }
            saved
        } else {
            let saved = SavedSubmission {
                repository,
                profile: profile.into(),
                request: Request {
                    version: 1,
                    request_id: RequestId::new(publication::identity(&format!(
                        "chat-comments/{}",
                        request_id
                    ))),
                    command: CollabCommandV1::Thread {
                        intent: ThreadCommandV1::Submit {
                            submission: Box::new(submission.clone()),
                        },
                    },
                },
                batch: None,
                context: None,
                refused: false,
            };
            self.update(|s| {
                if s.comment_submissions.len() >= 4096 {
                    return Err(Error::Limit);
                }
                s.comment_submissions.insert(request_id, saved.clone());
                Ok(())
            })?;
            saved
        };
        if saved.refused {
            return Err(Error::Stale);
        }
        if let Some(context) = saved.context {
            // A receipt is durable; permission to disclose its foreign source is not.
            for comment in &context.comments {
                if matches!(
                    comment.reference.target,
                    ReactionTargetV1::RepositoryReviewComment { .. }
                ) {
                    QualifiedReviewRead::comment(self, &comment.reference.target)
                        .await
                        .map_err(|_| Error::Denied)?;
                }
            }
            return Ok(context);
        }
        let batch = match saved.batch {
            Some(batch) => batch,
            None => {
                let outcome = self
                    .apply_home_thread_command(saved.repository, &saved.profile, &saved.request)
                    .await;
                if matches!(outcome, Err(Error::Stale | Error::Invalid | Error::Denied)) {
                    self.update(|s| {
                        s.comment_submissions
                            .get_mut(&request_id)
                            .ok_or(Error::Invalid)?
                            .refused = true;
                        Ok(())
                    })?;
                }
                let result = outcome?.ok_or(Error::Unavailable)?;
                let batch = result.link.ok_or(Error::Invalid)?;
                self.update(|s| {
                    s.comment_submissions
                        .get_mut(&request_id)
                        .ok_or(Error::Invalid)?
                        .batch = Some(batch);
                    Ok(())
                })?;
                batch
            }
        };
        let context = if saved.profile == "local_jj_git" {
            self.0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(saved.repository)
                .map_err(|_| Error::Denied)?
                .conversation_submission_context_with_reviews(
                    submission.thread_id,
                    batch,
                    Some(self),
                )
                .await?
        } else {
            let session = self.home_session(saved.repository)?;
            let pin = jj_store_service::collab_reads::PinnedCollabRead::open(
                session.clone(),
                Arc::new(jj_store_service::collab_reads::RepositoryMemberReactionActors),
            )
            .await
            .map_err(|_| Error::Unavailable)?;
            pin.conversation_submission_context_with_reviews(
                session,
                submission.thread_id,
                batch,
                Some(self),
            )
            .await?
        };
        self.update(|s| {
            s.comment_submissions
                .get_mut(&request_id)
                .ok_or(Error::Invalid)?
                .context = Some(context.clone());
            Ok(())
        })?;
        Ok(context)
    }
    pub(super) async fn apply_home_thread_command(
        &self,
        repository: RepositoryId,
        profile: &str,
        request: &Request,
    ) -> Result<Option<ThreadCommandResultV1>> {
        self.authorized(repository, profile, true).await?;
        let result = if profile == "local_jj_git" {
            let local = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(repository)
                .map_err(|_| Error::Denied)?;
            match local
                .apply_collab_command_with_reviews(
                    &jj_store_service::local_repository::LocalCollabRequest {
                        version: 1,
                        request_id: request.request_id,
                        command: request.command.clone(),
                    },
                    Some(self),
                )
                .await
            {
                Ok(jj_store_service::local_repository::LocalCollabOutcome::Committed {
                    receipt,
                }) => Some(receipt.result),
                Ok(_) => None,
                Err(jj_store_service::local_repository::LocalError::Stale) => {
                    return Err(Error::Stale);
                }
                Err(jj_store_service::local_repository::LocalError::Invalid) => {
                    return Err(Error::Invalid);
                }
                Err(_) => return Err(Error::Unavailable),
            }
        } else {
            match CollabCommandService
                .apply_with_reviews(self.home_session(repository)?, request, Some(self))
                .await
            {
                Ok(jj_store_service::collab_mutations::Outcome::Committed { receipt }) => {
                    Some(receipt.result)
                }
                Ok(_) => None,
                Err(jj_store_service::collab_mutations::Error::Stale) => return Err(Error::Stale),
                Err(jj_store_service::collab_mutations::Error::Denied) => {
                    return Err(Error::Denied);
                }
                Err(jj_store_service::collab_mutations::Error::Invalid) => {
                    return Err(Error::Invalid);
                }
                Err(_) => return Err(Error::Unavailable),
            }
        };
        match result {
            Some(CollabCommandResultV1::Thread { result }) => Ok(Some(result)),
            None => Ok(None),
            _ => Err(Error::Invalid),
        }
    }
    /// A definitive stale refusal may be rebased only after a receipt read.
    /// Unknown reads remain unknown and must retain the exact original request.
    pub(super) async fn home_thread_command_not_known(
        &self,
        repository: RepositoryId,
        profile: &str,
        request: RequestId,
    ) -> Result<bool> {
        use jj_store_service::collab_mutations::Status;
        self.authorized(repository, profile, true).await?;
        if profile == "local_jj_git" {
            use jj_store_service::local_repository::LocalCollabOutcome;
            let local = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(repository)
                .map_err(|_| Error::Denied)?;
            match local
                .collab_store_status(request)
                .await
                .map_err(|_| Error::Unavailable)?
            {
                // Keep the original command for receipt-checked replay when committed
                // or unknown. Only proven nonpublication permits a new precondition.
                LocalCollabOutcome::NotKnown { .. } => Ok(true),
                _ => Ok(false),
            }
        } else {
            CollabCommandService
                .status(self.home_session(repository)?, request)
                .await
                .map(|status| matches!(status, Status::NotKnown { .. }))
                .map_err(|_| Error::Unavailable)
        }
    }
    pub(super) async fn home_thread_summary(
        &self,
        repository: RepositoryId,
        profile: &str,
        thread: ThreadId,
    ) -> Result<jj_store_service::collab_reads::ThreadSummaryV1> {
        self.authorized(repository, profile, true).await?;
        if profile == "local_jj_git" {
            let local = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(repository)
                .map_err(|_| Error::Denied)?;
            let pin = local
                .inbox_selection()
                .await
                .map_err(|_| Error::Unavailable)?
                .snapshot
                .metadata_commit;
            Ok(local
                .inbox_thread(pin, thread)
                .await
                .map_err(|_| Error::Unavailable)?
                .thread)
        } else {
            let session = self.home_session(repository)?;
            let pin = jj_store_service::collab_reads::PinnedCollabRead::open(
                session.clone(),
                Arc::new(jj_store_service::collab_reads::RepositoryMemberReactionActors),
            )
            .await
            .map_err(|_| Error::Unavailable)?;
            Ok(pin
                .thread(session, thread)
                .await
                .map_err(|_| Error::Unavailable)?)
        }
    }
    pub(super) async fn home_thread_revision(
        &self,
        repository: RepositoryId,
        profile: &str,
        thread: ThreadId,
    ) -> Result<CollabObjectRef> {
        Ok(self
            .home_thread_summary(repository, profile, thread)
            .await?
            .reference)
    }
}
