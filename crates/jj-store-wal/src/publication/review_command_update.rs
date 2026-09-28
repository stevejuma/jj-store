//! Stale-safe reply, revision and workflow updates for ReviewStore commands.
use super::review_command_apply::ReviewRevisionChange;
use super::review_command_history::{
    LoadedReviewThread, catalog_edit, command_entity_id, command_result, comment_key, events_key,
    selected_id_matches, thread_key,
};
use super::review_commands::{AppliedReviewCommand, ReviewCommandError, ReviewCommandRequest};
use super::*;
use jj_review::{
    CommentId, CommentRevisionContentV1, CommentRevisionId, CommentSelectionV1, ReviewActionId,
    ReviewCatalogKind, ReviewCommentBodyV1, ReviewCommentRevisionV1, ReviewThreadEventKindV1,
    ReviewThreadEventV1, ReviewThreadId, ThreadSnapshotId, ThreadStatusV1,
};

impl Coordinator {
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn append_review_reply(
        &self,
        store: &ReviewStoreSnapshot,
        request: &ReviewCommandRequest,
        thread_id: ReviewThreadId,
        expected_workflow_version: u64,
        parent_comment_id: Option<CommentId>,
        body: &ReviewCommentBodyV1,
        accepted_at_millis: i64,
    ) -> std::result::Result<AppliedReviewCommand, ReviewCommandError> {
        let loaded = self.load_review_thread(store, thread_id).await?;
        if loaded.snapshot.status == ThreadStatusV1::Resolved {
            return Err(ReviewCommandError::ThreadResolved);
        }
        if loaded.snapshot.workflow_version != expected_workflow_version {
            return Err(ReviewCommandError::Stale);
        }
        if parent_comment_id.is_some_and(|parent| {
            !loaded
                .snapshot
                .comments
                .iter()
                .any(|value| value.comment_id == parent)
        }) {
            return Err(ReviewCommandError::NotFound("parent comment"));
        }
        if loaded.snapshot.comments.len() >= 4096 {
            return Err(Error::Limit("review thread comments").into());
        }
        let comment_id = CommentId::new(command_entity_id(
            self.records.repository,
            request,
            b"reply-comment",
        ));
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
        let mut next = loaded.snapshot.clone();
        next.comments.push(CommentSelectionV1 {
            comment_id,
            revision,
            parent_comment_id,
        });
        next.thread_version = increment(next.thread_version, "review thread version")?;
        next.content_version = increment(next.content_version, "review content version")?;
        self.commit_review_thread(
            store,
            request,
            loaded,
            next,
            ReviewThreadEventKindV1::ReplyAdded {
                comment_id,
                parent_comment_id,
                revision,
            },
            Some((comment_id, None, revision)),
            Some(comment_id),
            None,
            Some(revision),
            accepted_at_millis,
        )
        .await
    }

    pub(super) async fn revise_review_comment(
        &self,
        store: &ReviewStoreSnapshot,
        request: &ReviewCommandRequest,
        comment_id: CommentId,
        expected_revision_id: &CommentRevisionId,
        change: ReviewRevisionChange<'_>,
        accepted_at_millis: i64,
    ) -> std::result::Result<AppliedReviewCommand, ReviewCommandError> {
        let (previous, selected) = self.load_review_comment(store, comment_id).await?;
        if !selected_id_matches(previous, expected_revision_id.as_bytes()) {
            return Err(ReviewCommandError::Stale);
        }
        let loaded = self.load_review_thread(store, selected.thread_id).await?;
        let position = loaded
            .snapshot
            .comments
            .iter()
            .position(|value| value.comment_id == comment_id && value.revision == previous)
            .ok_or(Error::Corrupt("review comment thread selection"))?;
        let content = match change {
            ReviewRevisionChange::Visible(body) => CommentRevisionContentV1::Visible {
                body: self.stage_review_body(body).await?,
            },
            ReviewRevisionChange::Tombstoned(reason) => CommentRevisionContentV1::Tombstoned {
                reason: reason.map(str::to_owned),
            },
            ReviewRevisionChange::Hidden(reason) => CommentRevisionContentV1::Hidden {
                reason: reason.to_owned(),
            },
        };
        let revision = self
            .stage_review_revision(ReviewCommentRevisionV1 {
                schema_version: 1,
                thread_id: selected.thread_id,
                comment_id,
                author: selected.author,
                actor: request.principal,
                created_at_millis: accepted_at_millis,
                previous_revision: Some(previous),
                content,
            })
            .await?;
        let mut next = loaded.snapshot.clone();
        next.comments[position].revision = revision;
        next.thread_version = increment(next.thread_version, "review thread version")?;
        next.content_version = increment(next.content_version, "review content version")?;
        self.commit_review_thread(
            store,
            request,
            loaded,
            next,
            ReviewThreadEventKindV1::CommentRevised {
                comment_id,
                previous_revision: previous,
                revision,
            },
            Some((comment_id, Some(previous), revision)),
            Some(comment_id),
            None,
            Some(revision),
            accepted_at_millis,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn change_review_workflow(
        &self,
        store: &ReviewStoreSnapshot,
        request: &ReviewCommandRequest,
        thread_id: ReviewThreadId,
        expected_thread_version: &ThreadSnapshotId,
        resolve: bool,
        reason: Option<&str>,
        accepted_at_millis: i64,
    ) -> std::result::Result<AppliedReviewCommand, ReviewCommandError> {
        let loaded = self.load_review_thread(store, thread_id).await?;
        if !selected_id_matches(loaded.reference, expected_thread_version.as_bytes()) {
            return Err(ReviewCommandError::Stale);
        }
        match (resolve, loaded.snapshot.status) {
            (true, ThreadStatusV1::Resolved) => return Err(ReviewCommandError::ThreadResolved),
            (false, ThreadStatusV1::Open) => return Err(ReviewCommandError::ThreadOpen),
            _ => {}
        }
        let label: &[u8] = if resolve {
            b"resolve-action"
        } else {
            b"reopen-action"
        };
        let action_id =
            ReviewActionId::new(command_entity_id(self.records.repository, request, label));
        let event = if resolve {
            ReviewThreadEventKindV1::Resolved {
                action_id,
                reason: reason.map(str::to_owned),
            }
        } else {
            ReviewThreadEventKindV1::Reopened {
                action_id,
                reason: reason.map(str::to_owned),
            }
        };
        let mut next = loaded.snapshot.clone();
        next.status = if resolve {
            ThreadStatusV1::Resolved
        } else {
            ThreadStatusV1::Open
        };
        next.selected_workflow_action = resolve.then_some(action_id);
        next.thread_version = increment(next.thread_version, "review thread version")?;
        next.workflow_version = increment(next.workflow_version, "review workflow version")?;
        self.commit_review_thread(
            store,
            request,
            loaded,
            next,
            event,
            None,
            None,
            Some(action_id),
            None,
            accepted_at_millis,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn commit_review_thread(
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

fn increment(value: u64, limit: &'static str) -> std::result::Result<u64, ReviewCommandError> {
    value
        .checked_add(1)
        .ok_or_else(|| Error::Limit(limit).into())
}
