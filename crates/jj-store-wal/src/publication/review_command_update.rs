//! Stale-safe reply, revision and workflow updates for ReviewStore commands.
use super::review_command_apply::ReviewRevisionChange;
use super::review_command_history::{command_entity_id, selected_id_matches};
use super::review_commands::{AppliedReviewCommand, ReviewCommandError, ReviewCommandRequest};
use super::*;
use jj_review::{
    CommentId, CommentRevisionContentV1, CommentRevisionId, CommentSelectionV1, ReviewActionId,
    ReviewCommentBodyV1, ReviewCommentRevisionV1, ReviewThreadEventKindV1, ReviewThreadId,
    ThreadSnapshotId, ThreadStatusV1,
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
                superseded_revisions: Vec::new(),
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
                superseded_revisions: Vec::new(),
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

    pub(super) async fn resolve_review_comment_conflict(
        &self,
        store: &ReviewStoreSnapshot,
        request: &ReviewCommandRequest,
        comment_id: CommentId,
        expected_revision_ids: &[CommentRevisionId],
        replacement_body: &ReviewCommentBodyV1,
        accepted_at_millis: i64,
    ) -> std::result::Result<AppliedReviewCommand, ReviewCommandError> {
        let conflict_ref = store
            .lookup(
                ReviewCatalogKind::Comments,
                super::review_command_history::comment_key(comment_id)?,
            )
            .await?
            .ok_or(ReviewCommandError::NotFound("comment"))?;
        let ReviewObjectV1::CommentConflict(conflict) = store.read_object(conflict_ref).await?
        else {
            return Err(ReviewCommandError::Stale);
        };
        if conflict.comment_id != comment_id {
            return Err(Error::Corrupt("review comment conflict identity").into());
        }
        let expected = expected_revision_ids
            .iter()
            .map(|id| id.as_bytes())
            .collect::<Vec<_>>();
        let actual = conflict
            .alternatives
            .iter()
            .map(|reference| reference.id.as_bytes())
            .collect::<Vec<_>>();
        if expected != actual {
            return Err(ReviewCommandError::Stale);
        }
        let loaded = self.load_review_thread(store, conflict.thread_id).await?;
        let position = loaded
            .snapshot
            .comments
            .iter()
            .position(|selection| {
                selection.comment_id == comment_id && selection.revision == conflict_ref
            })
            .ok_or(Error::Corrupt("review conflict thread selection"))?;
        let mut author = None;
        for alternative in &conflict.alternatives {
            let ReviewObjectV1::CommentRevision(revision) = store.read_object(*alternative).await?
            else {
                return Err(Error::Corrupt("review conflict alternative kind").into());
            };
            if revision.thread_id != conflict.thread_id || revision.comment_id != comment_id {
                return Err(Error::Corrupt("review conflict alternative identity").into());
            }
            match author {
                Some(selected) if selected != revision.author => {
                    return Err(Error::Corrupt("review conflict author identity").into());
                }
                None => author = Some(revision.author),
                _ => {}
            }
        }
        let revision = self
            .stage_review_revision(ReviewCommentRevisionV1 {
                schema_version: 1,
                thread_id: conflict.thread_id,
                comment_id,
                author: author.ok_or(Error::Corrupt("review conflict author"))?,
                actor: request.principal,
                created_at_millis: accepted_at_millis,
                previous_revision: None,
                superseded_revisions: conflict.alternatives.clone(),
                content: CommentRevisionContentV1::Visible {
                    body: self.stage_review_body(replacement_body).await?,
                },
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
            ReviewThreadEventKindV1::CommentConflictResolved {
                comment_id,
                superseded_revisions: conflict.alternatives,
                revision,
            },
            Some((comment_id, Some(conflict_ref), revision)),
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
}

fn increment(value: u64, limit: &'static str) -> std::result::Result<u64, ReviewCommandError> {
    value
        .checked_add(1)
        .ok_or_else(|| Error::Limit(limit).into())
}
