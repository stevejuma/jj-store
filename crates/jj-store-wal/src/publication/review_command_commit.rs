//! One thread update plus its direct relation and status indexes.
use super::review_command_history::{
    LoadedReviewThread, catalog_edit, command_result, comment_key, events_key, thread_key,
};
use super::review_commands::{AppliedReviewCommand, ReviewCommandError, ReviewCommandRequest};
use super::*;
use jj_review::{
    CommentId, ReviewActionId, ReviewCatalogKind, ReviewThreadEventKindV1, ReviewThreadEventV1,
};

impl Coordinator {
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn commit_review_thread(
        &self,
        store: &ReviewStoreSnapshot,
        request: &ReviewCommandRequest,
        mut loaded: LoadedReviewThread,
        mut next: jj_review::ReviewThreadSnapshotV1,
        event_kind: ReviewThreadEventKindV1,
        comment_edit: Option<(CommentId, Option<ReviewObjectRef>, ReviewObjectRef)>,
        comment_id: Option<CommentId>,
        action_id: Option<ReviewActionId>,
        comment_revision: Option<ReviewObjectRef>,
        accepted_at_millis: i64,
    ) -> std::result::Result<AppliedReviewCommand, ReviewCommandError> {
        if next.thread_version != loaded.snapshot.thread_version + 1 {
            return Err(Error::Corrupt("review thread version update").into());
        }
        let event = self
            .stage_review_event(ReviewThreadEventV1 {
                schema_version: 1,
                thread_id: next.thread_id,
                sequence: next.thread_version,
                actor: request.principal,
                accepted_at_millis,
                event: event_kind,
            })
            .await?;
        loaded.events.push(event);
        let event_page_root = self
            .stage_review_event_pages(next.thread_id, &loaded.events)
            .await?;
        next.event_page_root = event_page_root;
        next.last_event_sequence = next.thread_version;
        let next_status = next.status;
        let thread_snapshot = self.stage_review_snapshot(next).await?;
        let mut edits = vec![
            catalog_edit(
                ReviewCatalogKind::Threads,
                thread_key(loaded.snapshot.thread_id)?,
                Some(loaded.reference),
                thread_snapshot,
            ),
            catalog_edit(
                ReviewCatalogKind::ThreadEvents,
                events_key(loaded.snapshot.thread_id)?,
                Some(loaded.snapshot.event_page_root),
                event_page_root,
            ),
        ];
        if let Some((comment_id, expected, replacement)) = comment_edit {
            edits.push(catalog_edit(
                ReviewCatalogKind::Comments,
                comment_key(comment_id)?,
                expected,
                replacement,
            ));
        }
        edits.extend(
            self.stage_updated_thread_indexes(store, &loaded, thread_snapshot, next_status)
                .await?,
        );
        let root = self.stage_review_command_delta(store, edits).await?;
        Ok(AppliedReviewCommand {
            root,
            result: command_result(
                loaded.snapshot.thread_id,
                comment_id,
                action_id,
                thread_snapshot,
                comment_revision,
                event,
            ),
        })
    }
}
