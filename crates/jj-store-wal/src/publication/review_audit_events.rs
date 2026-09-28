//! Deterministic reconstruction of selected ReviewStore thread state.
use super::review_audit::require_selected;
use super::*;
use jj_review::{
    CommentSelectionV1, ReviewThreadEventKindV1, ReviewThreadSnapshotV1, ThreadStatusV1,
};

impl Coordinator {
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn reduce_review_audit_event(
        &self,
        snapshot: &ReviewThreadSnapshotV1,
        comments: &mut Vec<CommentSelectionV1>,
        status: &mut ThreadStatusV1,
        workflow_action: &mut Option<jj_review::ReviewActionId>,
        content_version: &mut u64,
        workflow_version: &mut u64,
        event: jj_review::ReviewThreadEventV1,
        closure: &BTreeSet<ReviewObjectRef>,
    ) -> Result<()> {
        match event.event {
            ReviewThreadEventKindV1::Created {
                anchor,
                initial_comment_id,
                initial_revision,
            } => {
                if event.sequence != 1 || anchor != snapshot.original_anchor || !comments.is_empty()
                {
                    return Err(Error::Corrupt("review creation identity"));
                }
                self.audit_event_revision(
                    snapshot,
                    closure,
                    initial_comment_id,
                    initial_revision,
                    None,
                )
                .await?;
                comments.push(CommentSelectionV1 {
                    comment_id: initial_comment_id,
                    revision: initial_revision,
                    parent_comment_id: None,
                });
                *content_version = 1;
                *workflow_version = 1;
            }
            ReviewThreadEventKindV1::ReplyAdded {
                comment_id,
                parent_comment_id,
                revision,
            } => {
                if *status != ThreadStatusV1::Open
                    || comments.iter().any(|value| value.comment_id == comment_id)
                    || parent_comment_id.is_some_and(|parent| {
                        !comments.iter().any(|value| value.comment_id == parent)
                    })
                {
                    return Err(Error::Corrupt("review reply identity"));
                }
                self.audit_event_revision(snapshot, closure, comment_id, revision, None)
                    .await?;
                comments.push(CommentSelectionV1 {
                    comment_id,
                    revision,
                    parent_comment_id,
                });
                *content_version = content_version
                    .checked_add(1)
                    .ok_or(Error::Limit("review content version"))?;
            }
            ReviewThreadEventKindV1::CommentRevised {
                comment_id,
                previous_revision,
                revision,
            } => {
                let selected = comments
                    .iter_mut()
                    .find(|value| value.comment_id == comment_id)
                    .ok_or(Error::Corrupt("review revision identity"))?;
                if selected.revision != previous_revision {
                    return Err(Error::Corrupt("review revision predecessor"));
                }
                self.audit_event_revision(
                    snapshot,
                    closure,
                    comment_id,
                    revision,
                    Some(previous_revision),
                )
                .await?;
                selected.revision = revision;
                *content_version = content_version
                    .checked_add(1)
                    .ok_or(Error::Limit("review content version"))?;
            }
            ReviewThreadEventKindV1::Resolved { action_id, .. } => {
                if *status != ThreadStatusV1::Open {
                    return Err(Error::Corrupt("review resolve transition"));
                }
                *status = ThreadStatusV1::Resolved;
                *workflow_action = Some(action_id);
                *workflow_version = workflow_version
                    .checked_add(1)
                    .ok_or(Error::Limit("review workflow version"))?;
            }
            ReviewThreadEventKindV1::Reopened { .. } => {
                if *status != ThreadStatusV1::Resolved {
                    return Err(Error::Corrupt("review reopen transition"));
                }
                *status = ThreadStatusV1::Open;
                *workflow_action = None;
                *workflow_version = workflow_version
                    .checked_add(1)
                    .ok_or(Error::Limit("review workflow version"))?;
            }
        }
        Ok(())
    }

    async fn audit_event_revision(
        &self,
        snapshot: &ReviewThreadSnapshotV1,
        closure: &BTreeSet<ReviewObjectRef>,
        comment_id: jj_review::CommentId,
        reference: ReviewObjectRef,
        previous: Option<ReviewObjectRef>,
    ) -> Result<()> {
        require_selected(closure, reference)?;
        let ReviewObjectV1::CommentRevision(revision) = self.read_review_object(reference).await?
        else {
            return Err(Error::Corrupt("review event revision kind"));
        };
        if revision.thread_id != snapshot.thread_id
            || revision.comment_id != comment_id
            || revision.previous_revision != previous
        {
            return Err(Error::Corrupt("review event revision identity"));
        }
        Ok(())
    }
}
