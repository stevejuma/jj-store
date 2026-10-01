//! Bounded selected ReviewStore history helpers for typed commands.
use super::review_commands::{ReviewCommandError, ReviewCommandRequest};
use super::*;
use jj_review::{
    CommentId, ReviewActionId, ReviewCatalogEditV1, ReviewCatalogKey, ReviewCatalogKind,
    ReviewCommandResultV1, ReviewCommentRevisionV1, ReviewThreadId, ReviewThreadSnapshotV1,
};

const MAX_THREAD_EVENTS: usize = 4096;

pub(super) struct LoadedReviewThread {
    pub reference: ReviewObjectRef,
    pub snapshot: ReviewThreadSnapshotV1,
    pub events: Vec<ReviewObjectRef>,
    pub graph: Option<ReviewEventGraphV1>,
}

impl ReviewStoreSnapshot {
    pub async fn lookup_thread(
        &self,
        thread_id: ReviewThreadId,
    ) -> Result<Option<ReviewObjectRef>> {
        self.lookup(ReviewCatalogKind::Threads, thread_key(thread_id)?)
            .await
    }
}

impl ReviewProjector<'_> {
    pub(super) async fn load_review_thread(
        &self,
        store: &dyn ReviewSnapshotRead,
        thread_id: ReviewThreadId,
    ) -> std::result::Result<LoadedReviewThread, ReviewCommandError> {
        let reference = store
            .lookup(ReviewCatalogKind::Threads, thread_key(thread_id)?)
            .await?
            .ok_or(ReviewCommandError::NotFound("thread"))?;
        match store.read_object(reference).await? {
            ReviewObjectV1::ThreadSnapshot(snapshot) => {
                let events = self
                    .load_review_events(store, snapshot.event_page_root)
                    .await?;
                Ok(LoadedReviewThread {
                    reference,
                    snapshot: *snapshot,
                    events,
                    graph: None,
                })
            }
            ReviewObjectV1::ThreadMerge(snapshot) => {
                let graph = self
                    .load_review_event_graph(snapshot.event_graph_root)
                    .await?;
                Ok(LoadedReviewThread {
                    reference,
                    snapshot: ReviewThreadSnapshotV1 {
                        schema_version: 1,
                        thread_id: snapshot.thread_id,
                        original_anchor: snapshot.original_anchor,
                        comments: snapshot.comments,
                        event_page_root: snapshot.event_graph_root,
                        last_event_sequence: snapshot.thread_version,
                        status: snapshot.status,
                        selected_workflow_action: snapshot
                            .selected_workflow_actions
                            .first()
                            .copied(),
                        thread_version: snapshot.thread_version,
                        content_version: snapshot.content_version,
                        workflow_version: snapshot.workflow_version,
                    },
                    events: Vec::new(),
                    graph: Some(graph),
                })
            }
            _ => Err(Error::Corrupt("review thread command selection").into()),
        }
    }

    pub(super) async fn load_review_comment(
        &self,
        store: &dyn ReviewSnapshotRead,
        comment_id: CommentId,
    ) -> std::result::Result<(ReviewObjectRef, ReviewCommentRevisionV1), ReviewCommandError> {
        let reference = store
            .lookup(ReviewCatalogKind::Comments, comment_key(comment_id)?)
            .await?
            .ok_or(ReviewCommandError::NotFound("comment"))?;
        let ReviewObjectV1::CommentRevision(revision) = store.read_object(reference).await? else {
            return Err(Error::Corrupt("review comment command selection").into());
        };
        Ok((reference, *revision))
    }

    async fn load_review_events(
        &self,
        store: &dyn ReviewSnapshotRead,
        root: ReviewObjectRef,
    ) -> std::result::Result<Vec<ReviewObjectRef>, ReviewCommandError> {
        let mut events = Vec::new();
        let mut page = Some(root);
        let mut pages = BTreeSet::new();
        while let Some(reference) = page {
            if !pages.insert(reference) {
                return Err(Error::Corrupt("review event page cycle").into());
            }
            let ReviewObjectV1::EventPage(selected) = store.read_object(reference).await? else {
                return Err(Error::Corrupt("review event page selection").into());
            };
            events.extend(selected.entries.into_iter().map(|value| value.event));
            if events.len() > MAX_THREAD_EVENTS {
                return Err(Error::Limit("review thread events").into());
            }
            page = selected.next;
        }
        Ok(events)
    }
}

pub(super) fn thread_key(thread_id: ReviewThreadId) -> Result<ReviewCatalogKey> {
    ReviewCatalogKey::derive("thread", thread_id.as_bytes())
        .map_err(|_| Error::Corrupt("review thread catalog key"))
}

pub(super) fn comment_key(comment_id: CommentId) -> Result<ReviewCatalogKey> {
    ReviewCatalogKey::derive("comment", comment_id.as_bytes())
        .map_err(|_| Error::Corrupt("review comment catalog key"))
}

pub(super) fn events_key(thread_id: ReviewThreadId) -> Result<ReviewCatalogKey> {
    ReviewCatalogKey::derive("thread-events", thread_id.as_bytes())
        .map_err(|_| Error::Corrupt("review event catalog key"))
}

pub(super) fn command_entity_id(
    repository: RepositoryId,
    request: &ReviewCommandRequest,
    label: &[u8],
) -> [u8; 16] {
    let mut hash = Sha256::new_with_prefix(b"jj-store/review-command-entity/v1\0");
    hash.update(repository.as_bytes());
    hash.update(request.principal.as_bytes());
    hash.update(request.request_id.as_bytes());
    hash.update(request.publication_id.as_bytes());
    hash.update((label.len() as u64).to_be_bytes());
    hash.update(label);
    let digest: [u8; 32] = hash.finalize().into();
    let mut id = [0; 16];
    id.copy_from_slice(&digest[..16]);
    id
}

pub(super) fn selected_id_matches(reference: ReviewObjectRef, expected: &[u8; 32]) -> bool {
    reference.id.as_bytes() == expected
}

pub(super) fn catalog_edit(
    catalog: ReviewCatalogKind,
    key: ReviewCatalogKey,
    expected: Option<ReviewObjectRef>,
    replacement: ReviewObjectRef,
) -> ReviewCatalogEditV1 {
    ReviewCatalogEditV1 {
        catalog,
        key,
        expected,
        replacement: Some(replacement),
    }
}

pub(super) fn command_result(
    thread_id: ReviewThreadId,
    comment_id: Option<CommentId>,
    action_id: Option<ReviewActionId>,
    thread_snapshot: ReviewObjectRef,
    comment_revision: Option<ReviewObjectRef>,
    event: ReviewObjectRef,
) -> ReviewCommandResultV1 {
    ReviewCommandResultV1 {
        schema_version: 1,
        thread_id,
        comment_id,
        action_id,
        thread_snapshot,
        comment_revision,
        event,
    }
}
