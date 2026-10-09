//! Individually receipt-backed file reviews, referenced by one frozen conversation batch.
use super::workspace_sources::CapturedFile;
use super::*;
use jj_collab::{ReactionTargetV1, WorkspaceCommentDraftV1};
use jj_store_service::{
    local_repository as local, review_model as review, review_mutations as native,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct FilePublication {
    pub draft: WorkspaceCommentDraftV1,
    pub profile: String,
    pub captured: CapturedFile,
    pub command: review::ReviewCommandV1,
    pub result: Option<ReactionTargetV1>,
}
impl Supervisor {
    pub(super) async fn apply_file_publication(&self, id: RequestId) -> Result<ReactionTargetV1> {
        let saved = self.read(|s| s.file_publications.get(&id).cloned().ok_or(Error::Invalid))?;
        let repository = saved.draft.source.repository;
        self.authorized(repository, &saved.profile, true).await?;
        if let Some(result) = saved.result {
            return Ok(result);
        }
        let child = RequestId::new(publication::identity(&format!("workspace-review/{id}")));
        let result = if saved.profile == "local_jj_git" {
            let repository = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(repository)
                .map_err(|_| Error::Denied)?;
            let command = if saved.captured.evidence.is_empty() {
                local::LocalReviewCommand::Apply {
                    intent: Box::new(saved.command),
                }
            } else {
                local::LocalReviewCommand::WithEvidence {
                    intent: Box::new(saved.command),
                    evidence: saved.captured.evidence,
                }
            };
            match repository
                .apply_review_command(&local::LocalReviewRequest {
                    version: 1,
                    request_id: child,
                    command,
                })
                .await
            {
                Ok(local::LocalReviewOutcome::Committed { receipt }) => {
                    receipt.command.ok_or(Error::Invalid)?
                }
                Ok(_) => return Err(Error::Unavailable),
                Err(local::LocalError::Stale) => return Err(Error::Stale),
                Err(local::LocalError::Invalid | local::LocalError::IntentConflict) => {
                    return Err(Error::Invalid);
                }
                Err(_) => return Err(Error::Unavailable),
            }
        } else {
            match native::ReviewCommandService
                .apply(
                    self.home_session(repository)?,
                    &native::Request {
                        version: 1,
                        request_id: child,
                        command: saved.command,
                    },
                )
                .await
            {
                Ok(native::Outcome::Committed { receipt }) => receipt.result,
                Ok(_) => return Err(Error::Unavailable),
                Err(native::Error::Denied) => return Err(Error::Denied),
                Err(native::Error::Stale) => return Err(Error::Stale),
                Err(native::Error::Invalid | native::Error::IntentConflict) => {
                    return Err(Error::Invalid);
                }
                Err(_) => return Err(Error::Unavailable),
            }
        };
        let target = ReactionTargetV1::RepositoryReviewComment {
            repository,
            thread_id: result.thread_id,
            comment_id: result.comment_id.ok_or(Error::Invalid)?,
            revision: result.comment_revision.ok_or(Error::Invalid)?,
        };
        self.update(|s| {
            s.file_publications
                .get_mut(&id)
                .ok_or(Error::Invalid)?
                .result = Some(target.clone());
            Ok(())
        })?;
        Ok(target)
    }
    pub(super) async fn publish_captured_file_draft(
        &self,
        draft: &WorkspaceCommentDraftV1,
        profile: &str,
        captured: CapturedFile,
        previous: Option<&ReactionTargetV1>,
    ) -> Result<ReactionTargetV1> {
        if let Some(old) = self.read(|s| Ok(s.file_publications.get(&draft.id).cloned()))? {
            if old.draft != *draft || old.profile != profile {
                return Err(Error::Conflict);
            }
            return self.apply_file_publication(draft.id).await;
        }
        let body = review::ReviewCommentBodyV1 {
            schema_version: 1,
            format: review::ReviewBodyFormatV1::SafeMarkdown,
            text: draft.body.clone(),
            sanitizer_version: "review-markdown-v1".into(),
        };
        let command = if let Some(previous) = previous {
            let ReactionTargetV1::RepositoryReviewComment {
                repository,
                thread_id,
                comment_id,
                revision,
            } = previous
            else {
                return Err(Error::Invalid);
            };
            if *repository != draft.source.repository {
                return Err(Error::Invalid);
            }
            let original = self.qualified_review_source(previous).await?;
            if original.anchor.target != captured.anchor.target {
                return Err(Error::Stale);
            }
            if draft.edit {
                review::ReviewCommandV1::EditComment {
                    comment_id: *comment_id,
                    expected_revision_id: review::CommentRevisionId::new(*revision.id.as_bytes()),
                    replacement_body: body,
                }
            } else {
                review::ReviewCommandV1::AppendReply {
                    thread_id: *thread_id,
                    expected_workflow_version: original.workflow_version,
                    parent_comment_id: Some(*comment_id),
                    body,
                }
            }
        } else {
            review::ReviewCommandV1::CreateThread {
                anchor: Box::new(captured.anchor.clone()),
                body,
            }
        };
        command.validate().map_err(|_| Error::Invalid)?;
        let captured = CapturedFile {
            anchor: captured.anchor,
            evidence: if matches!(&command, review::ReviewCommandV1::CreateThread { .. }) {
                captured.evidence
            } else {
                vec![]
            },
        };
        self.update(|s| {
            if s.file_publications.len() >= 4096 {
                return Err(Error::Limit);
            }
            if s.file_publications.contains_key(&draft.id) {
                return Err(Error::Conflict);
            }
            s.file_publications.insert(
                draft.id,
                FilePublication {
                    draft: draft.clone(),
                    profile: profile.into(),
                    captured,
                    command,
                    result: None,
                },
            );
            Ok(())
        })?;
        self.apply_file_publication(draft.id).await
    }
}
