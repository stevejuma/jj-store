//! Audit explicit multi-writer ReviewStore state without linearizing its events.
use super::review_audit::require_selected;
use super::*;

impl ReviewProjector<'_> {
    pub(super) async fn audit_replicated_review_thread(
        &self,
        snapshot: &ReviewThreadMergeV1,
        closure: &BTreeSet<ReviewObjectRef>,
    ) -> Result<()> {
        require_selected(closure, snapshot.event_graph_root)?;
        let ReviewObjectV1::EventGraph(graph) =
            self.read_review_object(snapshot.event_graph_root).await?
        else {
            return Err(Error::Corrupt("merged review event graph kind"));
        };
        if graph.thread_id != snapshot.thread_id
            || graph.nodes.len() as u64 != snapshot.thread_version
        {
            return Err(Error::Corrupt("merged review event graph identity"));
        }
        self.audit_review_event_graph(&graph).await?;
        for selection in &snapshot.comments {
            require_selected(closure, selection.revision)?;
            match self.read_review_object(selection.revision).await? {
                ReviewObjectV1::CommentRevision(revision) => {
                    if revision.thread_id != snapshot.thread_id
                        || revision.comment_id != selection.comment_id
                    {
                        return Err(Error::Corrupt("merged review revision identity"));
                    }
                }
                ReviewObjectV1::CommentConflict(conflict) => {
                    if conflict.thread_id != snapshot.thread_id
                        || conflict.comment_id != selection.comment_id
                    {
                        return Err(Error::Corrupt("merged review conflict identity"));
                    }
                    for alternative in conflict.alternatives {
                        require_selected(closure, alternative)?;
                        let ReviewObjectV1::CommentRevision(revision) =
                            self.read_review_object(alternative).await?
                        else {
                            return Err(Error::Corrupt("nested review comment conflict"));
                        };
                        if revision.thread_id != snapshot.thread_id
                            || revision.comment_id != selection.comment_id
                        {
                            return Err(Error::Corrupt("review conflict alternative identity"));
                        }
                    }
                }
                _ => return Err(Error::Corrupt("merged review comment kind")),
            }
        }
        let (status, actions, content_version, workflow_version) =
            self.reduce_review_workflow(&graph).await?;
        if status != snapshot.status
            || actions != snapshot.selected_workflow_actions
            || content_version != snapshot.content_version
            || workflow_version != snapshot.workflow_version
        {
            return Err(Error::Corrupt("merged review reduction"));
        }
        Ok(())
    }
}
