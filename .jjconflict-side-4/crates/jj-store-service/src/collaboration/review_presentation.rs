//! Rebuildable summary facts from the exact selected thread closure.
use super::review_threads::ReviewSelection;
use super::*;
use jj_store_wal::publication::{CommentRevisionContentV1, CommentSelectionV1, ReviewObjectV1};

pub(super) async fn load(
    pin: &ReviewSelection<'_>,
    thread: &ReviewThreadSummary,
    selections: &[CommentSelectionV1],
    events: &[ReviewEvent],
) -> Result<ReviewSummaryPresentation> {
    let created = events
        .iter()
        .find(|event| matches!(event.event, ReviewEventKind::Created { .. }))
        .ok_or(Error::Corrupt)?;
    let ReviewEventKind::Created {
        initial_comment_id, ..
    } = created.event
    else {
        return Err(Error::Corrupt);
    };
    let selection = selections
        .iter()
        .find(|item| item.comment_id == initial_comment_id)
        .ok_or(Error::Corrupt)?;
    let excerpt = match pin
        .store
        .read_object(selection.revision)
        .await
        .map_err(map_wal)?
    {
        ReviewObjectV1::CommentRevision(value) => {
            if value.thread_id != thread.thread_id || value.comment_id != initial_comment_id {
                return Err(Error::Corrupt);
            }
            match value.content {
                CommentRevisionContentV1::Visible { body } => {
                    let ReviewObjectV1::CommentBody(value) =
                        pin.store.read_object(body).await.map_err(map_wal)?
                    else {
                        return Err(Error::Corrupt);
                    };
                    if value.text.len() > 8 * 1024 * 1024 {
                        return Err(Error::Limit);
                    }
                    value
                        .text
                        .lines()
                        .find(|line| !line.trim().is_empty())
                        .map(|line| line.trim().chars().take(240).collect())
                }
                CommentRevisionContentV1::Hidden { .. }
                | CommentRevisionContentV1::Tombstoned { .. } => None,
            }
        }
        ReviewObjectV1::CommentConflict(value) => {
            if value.thread_id != thread.thread_id || value.comment_id != initial_comment_id {
                return Err(Error::Corrupt);
            }
            None
        }
        _ => return Err(Error::Corrupt),
    };
    Ok(ReviewSummaryPresentation {
        created_by: created.actor,
        created_at_millis: created.accepted_at_millis,
        updated_at_millis: events
            .iter()
            .map(|event| event.accepted_at_millis)
            .max()
            .ok_or(Error::Corrupt)?,
        excerpt,
    })
}
