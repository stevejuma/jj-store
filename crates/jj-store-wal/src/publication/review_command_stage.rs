//! Shared immutable object and typed-delta staging for ReviewStore commands.
use super::review_commands::ReviewCommandError;
use super::*;
use jj_review::{
    ReviewCatalogEditV1, ReviewCommentBodyV1, ReviewCommentRevisionV1, ReviewDeltaV1,
    ReviewEventPageV1, ReviewEventSelectionV1, ReviewObjectV1, ReviewThreadEventV1, ReviewThreadId,
    ReviewThreadSnapshotV1,
};

const MAX_THREAD_EVENTS: usize = 4096;
const EVENT_PAGE_ENTRIES: usize = 256;

impl Coordinator {
    pub(super) async fn stage_review_body(
        &self,
        body: &ReviewCommentBodyV1,
    ) -> std::result::Result<ReviewObjectRef, ReviewCommandError> {
        Ok(self
            .stage_review_object(&ReviewObjectV1::CommentBody(body.clone()))
            .await?)
    }

    pub(super) async fn stage_review_revision(
        &self,
        revision: ReviewCommentRevisionV1,
    ) -> std::result::Result<ReviewObjectRef, ReviewCommandError> {
        Ok(self
            .stage_review_object(&ReviewObjectV1::CommentRevision(Box::new(revision)))
            .await?)
    }

    pub(super) async fn stage_review_event(
        &self,
        event: ReviewThreadEventV1,
    ) -> std::result::Result<ReviewObjectRef, ReviewCommandError> {
        Ok(self
            .stage_review_object(&ReviewObjectV1::ThreadEvent(Box::new(event)))
            .await?)
    }

    pub(super) async fn stage_review_snapshot(
        &self,
        snapshot: ReviewThreadSnapshotV1,
    ) -> std::result::Result<ReviewObjectRef, ReviewCommandError> {
        Ok(self
            .stage_review_object(&ReviewObjectV1::ThreadSnapshot(Box::new(snapshot)))
            .await?)
    }

    pub(super) async fn stage_review_event_pages(
        &self,
        thread_id: ReviewThreadId,
        events: &[ReviewObjectRef],
    ) -> std::result::Result<ReviewObjectRef, ReviewCommandError> {
        if events.is_empty() || events.len() > MAX_THREAD_EVENTS {
            return Err(Error::Limit("review thread events").into());
        }
        let mut next = None;
        let mut end = events.len();
        while end > 0 {
            let start = end.saturating_sub(EVENT_PAGE_ENTRIES);
            let entries = events[start..end]
                .iter()
                .enumerate()
                .map(|(offset, event)| ReviewEventSelectionV1 {
                    sequence: (start + offset + 1) as u64,
                    event: *event,
                })
                .collect();
            next = Some(
                self.stage_review_object(&ReviewObjectV1::EventPage(ReviewEventPageV1 {
                    schema_version: 1,
                    thread_id,
                    entries,
                    next,
                }))
                .await?,
            );
            end = start;
        }
        next.ok_or_else(|| Error::Corrupt("empty review event history").into())
    }

    pub(super) async fn stage_review_command_delta(
        &self,
        store: &ReviewStoreSnapshot,
        edits: Vec<ReviewCatalogEditV1>,
    ) -> std::result::Result<(RecordRef, ReviewDeltaV1), ReviewCommandError> {
        let delta = ReviewDeltaV1 {
            schema_version: 1,
            edits,
        };
        match self.stage_review_delta(store, &delta).await {
            Ok(root) => Ok((root, delta)),
            Err(Error::Stale) => Err(ReviewCommandError::Stale),
            Err(error) => Err(error.into()),
        }
    }
}
