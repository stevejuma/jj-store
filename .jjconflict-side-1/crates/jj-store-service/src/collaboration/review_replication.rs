//! Presentation of explicit multi-writer ReviewStore state.
use super::review_threads::ReviewSelection;
use super::*;
use jj_store_wal::publication::ReviewSnapshotRead;
use jj_store_wal::publication::{
    CommentRevisionContentV1, CommentSelectionV1, ReviewCommentRevisionV1, ReviewEventGraphV1,
    ReviewObjectRef, ReviewObjectV1, ReviewThreadId,
};
use std::collections::{BTreeMap, BTreeSet};

const MAX_THREAD_BODY_BYTES: usize = 8 * 1024 * 1024;

pub(super) async fn load_merged_comments(
    pin: &ReviewSelection<'_>,
    thread_id: ReviewThreadId,
    selections: &[CommentSelectionV1],
) -> Result<(Vec<ReviewComment>, Vec<ReviewCommentConflict>)> {
    let mut comments = Vec::new();
    let mut conflicts = Vec::new();
    let mut body_bytes = 0usize;
    for selection in selections {
        match pin
            .store
            .read_object(selection.revision)
            .await
            .map_err(map_wal)?
        {
            ReviewObjectV1::CommentRevision(value) => {
                comments.push(load_comment(pin.store, selection, *value, &mut body_bytes).await?)
            }
            ReviewObjectV1::CommentConflict(conflict) => {
                if conflict.thread_id != thread_id || conflict.comment_id != selection.comment_id {
                    return Err(Error::Corrupt);
                }
                let mut alternatives = Vec::with_capacity(conflict.alternatives.len());
                for reference in conflict.alternatives {
                    let ReviewObjectV1::CommentRevision(value) =
                        pin.store.read_object(reference).await.map_err(map_wal)?
                    else {
                        return Err(Error::Corrupt);
                    };
                    let comment = load_comment(
                        pin.store,
                        &CommentSelectionV1 {
                            comment_id: selection.comment_id,
                            revision: reference,
                            parent_comment_id: selection.parent_comment_id,
                        },
                        *value,
                        &mut body_bytes,
                    )
                    .await?;
                    alternatives.push(ReviewCommentVersion {
                        revision_id: comment.revision_id,
                        author: comment.author,
                        actor: comment.actor,
                        created_at_millis: comment.created_at_millis,
                        previous_revision_id: comment.previous_revision_id,
                        superseded_revision_ids: comment.superseded_revision_ids,
                        content: comment.content,
                    });
                }
                conflicts.push(ReviewCommentConflict {
                    comment_id: selection.comment_id,
                    parent_comment_id: selection.parent_comment_id,
                    alternatives,
                });
            }
            _ => return Err(Error::Corrupt),
        }
    }
    Ok((comments, conflicts))
}

async fn load_comment(
    store: &dyn ReviewSnapshotRead,
    selection: &CommentSelectionV1,
    value: ReviewCommentRevisionV1,
    body_bytes: &mut usize,
) -> Result<ReviewComment> {
    if value.comment_id != selection.comment_id {
        return Err(Error::Corrupt);
    }
    let content = match value.content {
        CommentRevisionContentV1::Visible { body } => {
            let ReviewObjectV1::CommentBody(body_value) =
                store.read_object(body).await.map_err(map_wal)?
            else {
                return Err(Error::Corrupt);
            };
            *body_bytes = body_bytes
                .checked_add(body_value.text.len())
                .filter(|value| *value <= MAX_THREAD_BODY_BYTES)
                .ok_or(Error::Limit)?;
            ReviewCommentContent::Visible {
                body_id: body.id,
                format: body_value.format,
                text: body_value.text,
                sanitizer_version: body_value.sanitizer_version,
            }
        }
        CommentRevisionContentV1::Tombstoned { reason } => {
            ReviewCommentContent::Tombstoned { reason }
        }
        CommentRevisionContentV1::Hidden { reason } => ReviewCommentContent::Hidden { reason },
    };
    Ok(ReviewComment {
        comment_id: value.comment_id,
        revision_id: selection.revision.id,
        parent_comment_id: selection.parent_comment_id,
        author: value.author,
        actor: value.actor,
        created_at_millis: value.created_at_millis,
        previous_revision_id: value.previous_revision.map(|item| item.id),
        superseded_revision_ids: value
            .superseded_revisions
            .into_iter()
            .map(|reference| reference.id)
            .collect(),
        content,
    })
}

pub(super) async fn load_graph_events(
    pin: &ReviewSelection<'_>,
    thread_id: ReviewThreadId,
    root: ReviewObjectRef,
) -> Result<Vec<ReviewEvent>> {
    let ReviewObjectV1::EventGraph(graph) = pin.store.read_object(root).await.map_err(map_wal)?
    else {
        return Err(Error::Corrupt);
    };
    if graph.thread_id != thread_id {
        return Err(Error::Corrupt);
    }
    ordered_graph_events(pin, graph).await
}

async fn ordered_graph_events(
    pin: &ReviewSelection<'_>,
    graph: ReviewEventGraphV1,
) -> Result<Vec<ReviewEvent>> {
    let mut pending = graph
        .nodes
        .into_iter()
        .map(|node| (node.event, node.parents))
        .collect::<BTreeMap<_, _>>();
    let mut selected = BTreeSet::new();
    let mut events = Vec::with_capacity(pending.len());
    while !pending.is_empty() {
        let ready = pending
            .iter()
            .filter(|(_, parents)| parents.iter().all(|parent| selected.contains(parent)))
            .map(|(event, _)| *event)
            .collect::<Vec<_>>();
        if ready.is_empty() {
            return Err(Error::Corrupt);
        }
        for reference in ready {
            let parents = pending.remove(&reference).ok_or(Error::Corrupt)?;
            let ReviewObjectV1::ThreadEvent(event) =
                pin.store.read_object(reference).await.map_err(map_wal)?
            else {
                return Err(Error::Corrupt);
            };
            if event.thread_id != graph.thread_id {
                return Err(Error::Corrupt);
            }
            events.push(ReviewEvent {
                event_id: reference.id,
                causal_parent_ids: parents.into_iter().map(|parent| parent.id).collect(),
                sequence: event.sequence,
                actor: event.actor,
                accepted_at_millis: event.accepted_at_millis,
                event: review_models::event_kind(event.event),
            });
            selected.insert(reference);
        }
    }
    Ok(events)
}

pub(super) async fn count_comment_conflicts(
    store: &dyn ReviewSnapshotRead,
    selections: &[CommentSelectionV1],
) -> Result<usize> {
    let mut count = 0;
    for selection in selections {
        if matches!(
            store
                .read_object(selection.revision)
                .await
                .map_err(map_wal)?,
            ReviewObjectV1::CommentConflict(_)
        ) {
            count += 1;
        }
    }
    Ok(count)
}
