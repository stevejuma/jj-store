//! Review relation and status catalogs derive only from selected thread snapshots.
use super::*;
use jj_review::{
    ReviewCatalogKey, ReviewIndexPageV1, ReviewRelationPageV1, ReviewRelationSubjectV1,
    ReviewRootV1, ReviewStatusPageV1, ReviewThreadRelationV1, review_status_key,
};

impl Coordinator {
    pub(super) async fn audit_review_relation_catalogs(
        &self,
        root: &ReviewRootV1,
        closure: &BTreeSet<ReviewObjectRef>,
    ) -> Result<()> {
        let threads = self.selected_review_threads(root.threads).await?;
        self.audit_relation_index(root.anchor_relations, true, &threads, closure)
            .await?;
        self.audit_relation_index(root.artifact_relations, false, &threads, closure)
            .await?;
        self.audit_status_index(root.statuses, &threads, closure)
            .await
    }

    async fn audit_relation_index(
        &self,
        root: Option<ReviewObjectRef>,
        anchors: bool,
        threads: &BTreeMap<jj_review::ReviewThreadId, ReviewObjectRef>,
        closure: &BTreeSet<ReviewObjectRef>,
    ) -> Result<()> {
        for (key, selected) in self.review_index_selections(root).await? {
            let ReviewObjectV1::RelationPage(page) = self.read_review_object(selected).await?
            else {
                return Err(Error::Corrupt("review relation catalog kind"));
            };
            let subject_matches_catalog = matches!(
                (&page.subject, anchors),
                (ReviewRelationSubjectV1::OriginalAnchor { .. }, true)
                    | (ReviewRelationSubjectV1::Artifact { .. }, false)
            );
            if !subject_matches_catalog
                || page.subject.catalog_key().map_err(review::review_error)? != key
            {
                return Err(Error::Corrupt("review relation catalog key"));
            }
            self.audit_relation_chain(page, threads, closure).await?;
        }
        Ok(())
    }

    async fn audit_relation_chain(
        &self,
        mut page: ReviewRelationPageV1,
        threads: &BTreeMap<jj_review::ReviewThreadId, ReviewObjectRef>,
        closure: &BTreeSet<ReviewObjectRef>,
    ) -> Result<()> {
        let subject = page.subject.clone();
        let mut pages = BTreeSet::new();
        let mut seen_threads = BTreeSet::new();
        loop {
            if page.subject != subject {
                return Err(Error::Corrupt("review relation subject"));
            }
            for entry in &page.entries {
                if !seen_threads.insert(entry.thread_id) {
                    return Err(Error::Corrupt("duplicate review relation thread"));
                }
                self.audit_relation_entry(*entry, Some(&subject), None, threads, closure)
                    .await?;
            }
            let Some(next) = page.next else {
                return Ok(());
            };
            if !pages.insert(next) || pages.len() > 4096 {
                return Err(Error::Corrupt("review relation page cycle"));
            }
            super::review_audit::require_selected(closure, next)?;
            let ReviewObjectV1::RelationPage(next_page) = self.read_review_object(next).await?
            else {
                return Err(Error::Corrupt("review relation continuation kind"));
            };
            page = next_page;
        }
    }

    async fn audit_status_index(
        &self,
        root: Option<ReviewObjectRef>,
        threads: &BTreeMap<jj_review::ReviewThreadId, ReviewObjectRef>,
        closure: &BTreeSet<ReviewObjectRef>,
    ) -> Result<()> {
        for (key, selected) in self.review_index_selections(root).await? {
            let ReviewObjectV1::StatusPage(page) = self.read_review_object(selected).await? else {
                return Err(Error::Corrupt("review status catalog kind"));
            };
            if review_status_key(page.status).map_err(review::review_error)? != key {
                return Err(Error::Corrupt("review status catalog key"));
            }
            self.audit_status_chain(page, threads, closure).await?;
        }
        Ok(())
    }

    async fn audit_status_chain(
        &self,
        mut page: ReviewStatusPageV1,
        threads: &BTreeMap<jj_review::ReviewThreadId, ReviewObjectRef>,
        closure: &BTreeSet<ReviewObjectRef>,
    ) -> Result<()> {
        let status = page.status;
        let mut pages = BTreeSet::new();
        let mut seen_threads = BTreeSet::new();
        loop {
            if page.status != status {
                return Err(Error::Corrupt("review status continuation"));
            }
            for entry in &page.entries {
                if !seen_threads.insert(entry.thread_id) {
                    return Err(Error::Corrupt("duplicate review status thread"));
                }
                self.audit_relation_entry(*entry, None, Some(status), threads, closure)
                    .await?;
            }
            let Some(next) = page.next else {
                return Ok(());
            };
            if !pages.insert(next) || pages.len() > 4096 {
                return Err(Error::Corrupt("review status page cycle"));
            }
            super::review_audit::require_selected(closure, next)?;
            let ReviewObjectV1::StatusPage(next_page) = self.read_review_object(next).await? else {
                return Err(Error::Corrupt("review status continuation kind"));
            };
            page = next_page;
        }
    }

    async fn audit_relation_entry(
        &self,
        entry: ReviewThreadRelationV1,
        subject: Option<&ReviewRelationSubjectV1>,
        status: Option<jj_review::ThreadStatusV1>,
        threads: &BTreeMap<jj_review::ReviewThreadId, ReviewObjectRef>,
        closure: &BTreeSet<ReviewObjectRef>,
    ) -> Result<()> {
        super::review_audit::require_selected(closure, entry.snapshot)?;
        let ReviewObjectV1::ThreadSnapshot(snapshot) =
            self.read_review_object(entry.snapshot).await?
        else {
            return Err(Error::Corrupt("review relation snapshot kind"));
        };
        if threads.get(&entry.thread_id) != Some(&entry.snapshot)
            || snapshot.thread_id != entry.thread_id
            || status.is_some_and(|value| snapshot.status != value)
        {
            return Err(Error::Corrupt("review relation snapshot identity"));
        }
        let Some(subject) = subject else {
            return Ok(());
        };
        super::review_audit::require_selected(closure, snapshot.original_anchor)?;
        let ReviewObjectV1::Anchor(anchor) =
            self.read_review_object(snapshot.original_anchor).await?
        else {
            return Err(Error::Corrupt("review relation anchor kind"));
        };
        let matches = match subject {
            ReviewRelationSubjectV1::OriginalAnchor { anchor_id } => {
                anchor.anchor_id().map_err(review::review_error)? == *anchor_id
            }
            ReviewRelationSubjectV1::Artifact { artifact_id } => {
                anchor.artifact_id().map_err(review::review_error)? == *artifact_id
            }
        };
        if matches {
            Ok(())
        } else {
            Err(Error::Corrupt("review relation anchor identity"))
        }
    }

    async fn review_index_selections(
        &self,
        root: Option<ReviewObjectRef>,
    ) -> Result<Vec<(ReviewCatalogKey, ReviewObjectRef)>> {
        let mut pending: Vec<_> = root.into_iter().collect();
        let mut pages = BTreeSet::new();
        let mut selected = Vec::new();
        while let Some(reference) = pending.pop() {
            if !pages.insert(reference) || pages.len() > 16_512 {
                return Err(Error::Corrupt("review relation index cycle"));
            }
            match self.read_review_object(reference).await? {
                ReviewObjectV1::IndexPage(ReviewIndexPageV1::Leaf { entries, .. }) => {
                    selected.extend(entries);
                }
                ReviewObjectV1::IndexPage(ReviewIndexPageV1::Branch { children, .. }) => {
                    pending.extend(children.into_values());
                }
                _ => return Err(Error::Corrupt("review relation index kind")),
            }
        }
        Ok(selected)
    }

    async fn selected_review_threads(
        &self,
        root: Option<ReviewObjectRef>,
    ) -> Result<BTreeMap<jj_review::ReviewThreadId, ReviewObjectRef>> {
        let mut threads = BTreeMap::new();
        for (_, reference) in self.review_index_selections(root).await? {
            let ReviewObjectV1::ThreadSnapshot(snapshot) =
                self.read_review_object(reference).await?
            else {
                return Err(Error::Corrupt("review relation thread catalog kind"));
            };
            if threads.insert(snapshot.thread_id, reference).is_some() {
                return Err(Error::Corrupt("duplicate review thread selection"));
            }
        }
        Ok(threads)
    }
}
