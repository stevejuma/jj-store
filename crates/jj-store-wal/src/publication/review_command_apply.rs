//! Typed ReviewStore command staging against one root-pinned snapshot.
use super::review_command_history::{
    catalog_edit, command_entity_id, command_result, comment_key, events_key, thread_key,
};
use super::review_commands::{AppliedReviewCommand, ReviewCommandError, ReviewCommandRequest};
use super::*;
use jj_review::{
    CommentId, CommentRevisionContentV1, CommentSelectionV1, ReviewCatalogKind, ReviewCommandV1,
    ReviewCommentBodyV1, ReviewCommentRevisionV1, ReviewObjectV1, ReviewThreadEventKindV1,
    ReviewThreadEventV1, ReviewThreadId, ReviewThreadSnapshotV1, ThreadStatusV1,
};

impl Coordinator {
    pub(super) async fn apply_review_command(
        &self,
        store: &ReviewStoreSnapshot,
        request: &ReviewCommandRequest,
        accepted_at_millis: i64,
    ) -> std::result::Result<AppliedReviewCommand, ReviewCommandError> {
        match &request.command {
            ReviewCommandV1::CreateThread { anchor, body } => {
                self.create_review_thread(store, request, anchor, body, accepted_at_millis)
                    .await
            }
            ReviewCommandV1::AppendReply {
                thread_id,
                expected_workflow_version,
                parent_comment_id,
                body,
            } => {
                self.append_review_reply(
                    store,
                    request,
                    *thread_id,
                    *expected_workflow_version,
                    *parent_comment_id,
                    body,
                    accepted_at_millis,
                )
                .await
            }
            ReviewCommandV1::EditComment {
                comment_id,
                expected_revision_id,
                replacement_body,
            } => {
                self.revise_review_comment(
                    store,
                    request,
                    *comment_id,
                    expected_revision_id,
                    ReviewRevisionChange::Visible(replacement_body),
                    accepted_at_millis,
                )
                .await
            }
            ReviewCommandV1::TombstoneComment {
                comment_id,
                expected_revision_id,
                reason,
            } => {
                self.revise_review_comment(
                    store,
                    request,
                    *comment_id,
                    expected_revision_id,
                    ReviewRevisionChange::Tombstoned(reason.as_deref()),
                    accepted_at_millis,
                )
                .await
            }
            ReviewCommandV1::HideComment {
                comment_id,
                expected_revision_id,
                moderation_reason,
            } => {
                self.revise_review_comment(
                    store,
                    request,
                    *comment_id,
                    expected_revision_id,
                    ReviewRevisionChange::Hidden(moderation_reason),
                    accepted_at_millis,
                )
                .await
            }
            ReviewCommandV1::ResolveThread {
                thread_id,
                expected_thread_version,
                reason,
            } => {
                self.change_review_workflow(
                    store,
                    request,
                    *thread_id,
                    expected_thread_version,
                    true,
                    reason.as_deref(),
                    accepted_at_millis,
                )
                .await
            }
            ReviewCommandV1::ReopenThread {
                thread_id,
                expected_thread_version,
                reason,
            } => {
                self.change_review_workflow(
                    store,
                    request,
                    *thread_id,
                    expected_thread_version,
                    false,
                    reason.as_deref(),
                    accepted_at_millis,
                )
                .await
            }
        }
    }

    async fn create_review_thread(
        &self,
        store: &ReviewStoreSnapshot,
        request: &ReviewCommandRequest,
        anchor: &jj_review::ArtifactAnchorV1,
        body: &ReviewCommentBodyV1,
        accepted_at_millis: i64,
    ) -> std::result::Result<AppliedReviewCommand, ReviewCommandError> {
        if anchor.repository_id != self.records.repository {
            return Err(ReviewCommandError::Invalid("anchor repository"));
        }
        let thread_id = ReviewThreadId::new(command_entity_id(
            self.records.repository,
            request,
            b"thread",
        ));
        let comment_id = CommentId::new(command_entity_id(
            self.records.repository,
            request,
            b"initial-comment",
        ));
        let anchor_ref = self
            .stage_review_object(&ReviewObjectV1::Anchor(Box::new(anchor.clone())))
            .await?;
        let body = self.stage_review_body(body).await?;
        let revision = self
            .stage_review_revision(ReviewCommentRevisionV1 {
                schema_version: 1,
                thread_id,
                comment_id,
                author: request.principal,
                actor: request.principal,
                created_at_millis: accepted_at_millis,
                previous_revision: None,
                content: CommentRevisionContentV1::Visible { body },
            })
            .await?;
        let event = self
            .stage_review_event(ReviewThreadEventV1 {
                schema_version: 1,
                thread_id,
                sequence: 1,
                actor: request.principal,
                accepted_at_millis,
                event: ReviewThreadEventKindV1::Created {
                    anchor: anchor_ref,
                    initial_comment_id: comment_id,
                    initial_revision: revision,
                },
            })
            .await?;
        let event_page_root = self.stage_review_event_pages(thread_id, &[event]).await?;
        let thread_snapshot = self
            .stage_review_snapshot(ReviewThreadSnapshotV1 {
                schema_version: 1,
                thread_id,
                original_anchor: anchor_ref,
                comments: vec![CommentSelectionV1 {
                    comment_id,
                    revision,
                    parent_comment_id: None,
                }],
                event_page_root,
                last_event_sequence: 1,
                status: ThreadStatusV1::Open,
                selected_workflow_action: None,
                thread_version: 1,
                content_version: 1,
                workflow_version: 1,
            })
            .await?;
        let mut edits = vec![
            catalog_edit(
                ReviewCatalogKind::Threads,
                thread_key(thread_id)?,
                None,
                thread_snapshot,
            ),
            catalog_edit(
                ReviewCatalogKind::Comments,
                comment_key(comment_id)?,
                None,
                revision,
            ),
            catalog_edit(
                ReviewCatalogKind::ThreadEvents,
                events_key(thread_id)?,
                None,
                event_page_root,
            ),
        ];
        edits.extend(
            self.stage_new_thread_indexes(store, anchor, thread_id, thread_snapshot)
                .await?,
        );
        let root = self.stage_review_command_delta(store, edits).await?;
        Ok(AppliedReviewCommand {
            root,
            result: command_result(
                thread_id,
                Some(comment_id),
                None,
                thread_snapshot,
                Some(revision),
                event,
            ),
        })
    }
}

pub(super) enum ReviewRevisionChange<'a> {
    Visible(&'a ReviewCommentBodyV1),
    Tombstoned(Option<&'a str>),
    Hidden(&'a str),
}
