//! Cross-object ReviewStore invariants that require a loaded closure.
use super::*;
use jj_review::{
    ReviewIndexPageV1, ReviewObjectKind, ReviewRootV1, ReviewThreadSnapshotV1, ThreadStatusV1,
};

impl ReviewProjector<'_> {
    pub(super) async fn audit_review_root(
        &self,
        root: &ReviewRootV1,
        objects: &BTreeSet<ReviewObjectRef>,
    ) -> Result<()> {
        if root.work_requests.is_some() || root.aliases.is_some() || root.sources.is_some() {
            return Err(Error::Corrupt("unsupported review catalog"));
        }
        self.audit_review_catalog(root.threads, ReviewObjectKind::ThreadSnapshot, objects)
            .await?;
        self.audit_review_catalog(root.comments, ReviewObjectKind::CommentRevision, objects)
            .await?;
        self.audit_review_catalog(root.thread_events, ReviewObjectKind::EventPage, objects)
            .await?;
        self.audit_review_catalog(
            root.anchor_relations,
            ReviewObjectKind::RelationPage,
            objects,
        )
        .await?;
        self.audit_review_catalog(
            root.artifact_relations,
            ReviewObjectKind::RelationPage,
            objects,
        )
        .await?;
        self.audit_review_catalog(root.statuses, ReviewObjectKind::StatusPage, objects)
            .await?;
        self.audit_review_relation_catalogs(root, objects).await
    }

    async fn audit_review_catalog(
        &self,
        root: Option<ReviewObjectRef>,
        expected: ReviewObjectKind,
        closure: &BTreeSet<ReviewObjectRef>,
    ) -> Result<()> {
        let mut pending: Vec<_> = root.into_iter().collect();
        let mut pages = BTreeSet::new();
        while let Some(reference) = pending.pop() {
            require_selected(closure, reference)?;
            if !pages.insert(reference) || pages.len() > 16_512 {
                return Err(Error::Corrupt("review catalog page cycle"));
            }
            let ReviewObjectV1::IndexPage(page) = self.read_review_object(reference).await? else {
                return Err(Error::Corrupt("review catalog page kind"));
            };
            match page {
                ReviewIndexPageV1::Leaf { entries, .. } => {
                    if entries.values().any(|value| value.kind != expected) {
                        return Err(Error::Corrupt("review catalog value kind"));
                    }
                }
                ReviewIndexPageV1::Branch { children, .. } => {
                    pending.extend(children.into_values());
                }
            }
        }
        Ok(())
    }

    pub(super) async fn audit_review_threads(
        &self,
        objects: &BTreeSet<ReviewObjectRef>,
    ) -> Result<()> {
        for reference in objects
            .iter()
            .filter(|value| value.kind == jj_review::ReviewObjectKind::ThreadSnapshot)
        {
            match self.read_review_object(*reference).await? {
                ReviewObjectV1::ThreadSnapshot(snapshot) => {
                    self.audit_review_thread(&snapshot, objects).await?;
                }
                ReviewObjectV1::ThreadMerge(snapshot) => {
                    self.audit_replicated_review_thread(&snapshot, objects)
                        .await?;
                }
                _ => return Err(Error::Corrupt("review thread snapshot kind")),
            }
        }
        Ok(())
    }

    async fn audit_review_thread(
        &self,
        snapshot: &ReviewThreadSnapshotV1,
        closure: &BTreeSet<ReviewObjectRef>,
    ) -> Result<()> {
        for selected in &snapshot.comments {
            require_selected(closure, selected.revision)?;
            let ReviewObjectV1::CommentRevision(revision) =
                self.read_review_object(selected.revision).await?
            else {
                return Err(Error::Corrupt("review comment selection kind"));
            };
            if revision.thread_id != snapshot.thread_id
                || revision.comment_id != selected.comment_id
            {
                return Err(Error::Corrupt("review comment selection identity"));
            }
        }

        let mut page = Some(snapshot.event_page_root);
        let mut expected_sequence = 1;
        let mut pages = BTreeSet::new();
        let mut comments = Vec::new();
        let mut status = ThreadStatusV1::Open;
        let mut workflow_action = None;
        let mut content_version = 0;
        let mut workflow_version = 0;
        while let Some(reference) = page {
            require_selected(closure, reference)?;
            if !pages.insert(reference) || pages.len() > 4096 {
                return Err(Error::Corrupt("review event page cycle"));
            }
            let ReviewObjectV1::EventPage(value) = self.read_review_object(reference).await? else {
                return Err(Error::Corrupt("review event page kind"));
            };
            if value.thread_id != snapshot.thread_id {
                return Err(Error::Corrupt("review event page thread"));
            }
            for selected in &value.entries {
                if selected.sequence != expected_sequence {
                    return Err(Error::Corrupt("review event history sequence"));
                }
                require_selected(closure, selected.event)?;
                let ReviewObjectV1::ThreadEvent(event) =
                    self.read_review_object(selected.event).await?
                else {
                    return Err(Error::Corrupt("review event selection kind"));
                };
                if event.thread_id != snapshot.thread_id || event.sequence != selected.sequence {
                    return Err(Error::Corrupt("review event selection identity"));
                }
                self.reduce_review_audit_event(
                    snapshot,
                    &mut comments,
                    &mut status,
                    &mut workflow_action,
                    &mut content_version,
                    &mut workflow_version,
                    *event,
                    closure,
                )
                .await?;
                expected_sequence = expected_sequence
                    .checked_add(1)
                    .ok_or(Error::Limit("review event sequence"))?;
            }
            page = value.next;
        }
        if expected_sequence - 1 != snapshot.last_event_sequence
            || comments != snapshot.comments
            || status != snapshot.status
            || workflow_action != snapshot.selected_workflow_action
            || content_version != snapshot.content_version
            || workflow_version != snapshot.workflow_version
        {
            return Err(Error::Corrupt("review event history tail"));
        }
        Ok(())
    }
}

pub(super) fn require_selected(
    closure: &BTreeSet<ReviewObjectRef>,
    reference: ReviewObjectRef,
) -> Result<()> {
    if closure.contains(&reference) {
        Ok(())
    } else {
        Err(Error::Corrupt("review dependency selection"))
    }
}
