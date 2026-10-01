//! One thread update plus its direct relation and status indexes.
use super::review_command_history::{
    LoadedReviewThread, catalog_edit, command_result, comment_key, events_key, thread_key,
};
use super::review_commands::{AppliedReviewCommand, ReviewCommandError, ReviewCommandRequest};
use super::*;
use jj_review::{
    CommentId, ReviewActionId, ReviewCatalogKind, ReviewThreadEventKindV1, ReviewThreadEventV1,
};

impl ReviewProjector<'_> {
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn commit_review_thread(
        &self,
        store: &dyn ReviewSnapshotRead,
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
        let event_sequence = if let Some(graph) = loaded.graph.as_ref() {
            let mut maximum = 0;
            for node in &graph.nodes {
                let ReviewObjectV1::ThreadEvent(event) =
                    self.read_review_object(node.event).await?
                else {
                    return Err(Error::Corrupt("review event graph selection").into());
                };
                maximum = maximum.max(event.sequence);
            }
            maximum
                .checked_add(1)
                .ok_or(Error::Limit("review event sequence"))?
        } else {
            next.thread_version
        };
        let event = self
            .stage_review_event(ReviewThreadEventV1 {
                schema_version: 1,
                thread_id: next.thread_id,
                sequence: event_sequence,
                actor: request.principal,
                accepted_at_millis,
                event: event_kind,
            })
            .await?;
        let (event_page_root, thread_snapshot, next_status) =
            if let Some(mut graph) = loaded.graph.take() {
                let parents = graph.heads.clone();
                graph.nodes.push(ReviewEventNodeV1 { event, parents });
                graph.nodes.sort_by_key(|node| node.event);
                graph.heads = vec![event];
                let event_page_root = self.stage_review_event_graph(graph.clone()).await?;
                let (status, selected_workflow_actions, content_version, workflow_version) =
                    self.reduce_review_workflow(&graph).await?;
                next.event_page_root = event_page_root;
                next.last_event_sequence = graph.nodes.len() as u64;
                next.thread_version = graph.nodes.len() as u64;
                next.content_version = content_version;
                next.workflow_version = workflow_version;
                next.status = status;
                next.selected_workflow_action = selected_workflow_actions.first().copied();
                let thread_snapshot = self
                    .stage_review_object(&ReviewObjectV1::ThreadMerge(Box::new(
                        ReviewThreadMergeV1 {
                            schema_version: 1,
                            thread_id: next.thread_id,
                            original_anchor: next.original_anchor,
                            comments: next.comments.clone(),
                            event_graph_root: event_page_root,
                            status,
                            selected_workflow_actions,
                            thread_version: next.thread_version,
                            content_version,
                            workflow_version,
                        },
                    )))
                    .await?;
                (event_page_root, thread_snapshot, status)
            } else {
                loaded.events.push(event);
                let event_page_root = self
                    .stage_review_event_pages(next.thread_id, &loaded.events)
                    .await?;
                next.event_page_root = event_page_root;
                next.last_event_sequence = next.thread_version;
                let next_status = next.status;
                let thread_snapshot = self.stage_review_snapshot(next).await?;
                (event_page_root, thread_snapshot, next_status)
            };
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
        let (root, delta) = self.stage_review_command_delta(store, edits).await?;
        Ok(AppliedReviewCommand {
            root,
            delta,
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
