//! Command-maintained direct anchor, artifact and workflow-status indexes.
use super::review_command_history::LoadedReviewThread;
use super::review_commands::ReviewCommandError;
use super::*;
use jj_review::{
    ArtifactAnchorV1, ReviewCatalogEditV1, ReviewCatalogKind, ReviewObjectV1, ReviewRelationPageV1,
    ReviewRelationSubjectV1, ReviewStatusPageV1, ReviewThreadRelationV1, ThreadStatusV1,
    review_status_key,
};

const PAGE_ENTRIES: usize = 256;
const MAX_INDEX_ENTRIES: usize = 4096;

impl ReviewProjector<'_> {
    pub(super) async fn stage_new_thread_indexes(
        &self,
        store: &dyn ReviewSnapshotRead,
        anchor: &ArtifactAnchorV1,
        thread_id: jj_review::ReviewThreadId,
        snapshot: ReviewObjectRef,
    ) -> std::result::Result<Vec<ReviewCatalogEditV1>, ReviewCommandError> {
        let entry = ReviewThreadRelationV1 {
            thread_id,
            snapshot,
        };
        let anchor_subject = ReviewRelationSubjectV1::OriginalAnchor {
            anchor_id: anchor.anchor_id().map_err(review::review_error)?,
        };
        let artifact_subject = ReviewRelationSubjectV1::Artifact {
            artifact_id: anchor.artifact_id().map_err(review::review_error)?,
        };
        Ok(vec![
            self.upsert_relation(
                store,
                ReviewCatalogKind::AnchorRelations,
                anchor_subject,
                entry,
            )
            .await?,
            self.upsert_relation(
                store,
                ReviewCatalogKind::ArtifactRelations,
                artifact_subject,
                entry,
            )
            .await?,
            self.upsert_status(
                store,
                ThreadStatusV1::Open,
                entry.thread_id,
                Some(entry.snapshot),
            )
            .await?,
        ])
    }

    pub(super) async fn stage_updated_thread_indexes(
        &self,
        store: &dyn ReviewSnapshotRead,
        loaded: &LoadedReviewThread,
        snapshot: ReviewObjectRef,
        next_status: ThreadStatusV1,
    ) -> std::result::Result<Vec<ReviewCatalogEditV1>, ReviewCommandError> {
        let ReviewObjectV1::Anchor(anchor) =
            store.read_object(loaded.snapshot.original_anchor).await?
        else {
            return Err(Error::Corrupt("review index anchor kind").into());
        };
        let entry = ReviewThreadRelationV1 {
            thread_id: loaded.snapshot.thread_id,
            snapshot,
        };
        let anchor_subject = ReviewRelationSubjectV1::OriginalAnchor {
            anchor_id: anchor.anchor_id().map_err(review::review_error)?,
        };
        let artifact_subject = ReviewRelationSubjectV1::Artifact {
            artifact_id: anchor.artifact_id().map_err(review::review_error)?,
        };
        let mut edits = vec![
            self.upsert_relation(
                store,
                ReviewCatalogKind::AnchorRelations,
                anchor_subject,
                entry,
            )
            .await?,
            self.upsert_relation(
                store,
                ReviewCatalogKind::ArtifactRelations,
                artifact_subject,
                entry,
            )
            .await?,
        ];
        if loaded.snapshot.status == next_status {
            edits.push(
                self.upsert_status(store, next_status, entry.thread_id, Some(snapshot))
                    .await?,
            );
        } else {
            edits.push(
                self.upsert_status(store, loaded.snapshot.status, entry.thread_id, None)
                    .await?,
            );
            edits.push(
                self.upsert_status(store, next_status, entry.thread_id, Some(snapshot))
                    .await?,
            );
        }
        Ok(edits)
    }

    async fn upsert_relation(
        &self,
        store: &dyn ReviewSnapshotRead,
        catalog: ReviewCatalogKind,
        subject: ReviewRelationSubjectV1,
        entry: ReviewThreadRelationV1,
    ) -> std::result::Result<ReviewCatalogEditV1, ReviewCommandError> {
        let key = subject.catalog_key().map_err(review::review_error)?;
        let expected = store.lookup(catalog, key).await?;
        let mut entries = self
            .load_relation_entries(store, expected, &subject)
            .await?;
        replace_entry(&mut entries, entry);
        check_entry_limit(&entries)?;
        let replacement = self.stage_relation_pages(subject, &entries).await?;
        Ok(ReviewCatalogEditV1 {
            catalog,
            key,
            expected,
            replacement: Some(replacement),
        })
    }

    async fn upsert_status(
        &self,
        store: &dyn ReviewSnapshotRead,
        status: ThreadStatusV1,
        thread_id: jj_review::ReviewThreadId,
        snapshot: Option<ReviewObjectRef>,
    ) -> std::result::Result<ReviewCatalogEditV1, ReviewCommandError> {
        let key = review_status_key(status).map_err(review::review_error)?;
        let expected = store.lookup(ReviewCatalogKind::Statuses, key).await?;
        if snapshot.is_none() && expected.is_none() {
            return Err(Error::Corrupt("missing review status index").into());
        }
        let mut entries = self.load_status_entries(store, expected, status).await?;
        entries.retain(|value| value.thread_id != thread_id);
        if let Some(snapshot) = snapshot {
            entries.push(ReviewThreadRelationV1 {
                thread_id,
                snapshot,
            });
        }
        entries.sort_unstable();
        check_entry_limit(&entries)?;
        let replacement = if entries.is_empty() {
            None
        } else {
            Some(self.stage_status_pages(status, &entries).await?)
        };
        Ok(ReviewCatalogEditV1 {
            catalog: ReviewCatalogKind::Statuses,
            key,
            expected,
            replacement,
        })
    }

    async fn load_relation_entries(
        &self,
        store: &dyn ReviewSnapshotRead,
        mut page: Option<ReviewObjectRef>,
        subject: &ReviewRelationSubjectV1,
    ) -> std::result::Result<Vec<ReviewThreadRelationV1>, ReviewCommandError> {
        let mut entries = Vec::new();
        while let Some(reference) = page {
            let ReviewObjectV1::RelationPage(value) = store.read_object(reference).await? else {
                return Err(Error::Corrupt("review relation command kind").into());
            };
            if value.subject != *subject {
                return Err(Error::Corrupt("review relation command subject").into());
            }
            entries.extend(value.entries);
            page = value.next;
            check_entry_limit(&entries)?;
        }
        Ok(entries)
    }

    async fn load_status_entries(
        &self,
        store: &dyn ReviewSnapshotRead,
        mut page: Option<ReviewObjectRef>,
        status: ThreadStatusV1,
    ) -> std::result::Result<Vec<ReviewThreadRelationV1>, ReviewCommandError> {
        let mut entries = Vec::new();
        while let Some(reference) = page {
            let ReviewObjectV1::StatusPage(value) = store.read_object(reference).await? else {
                return Err(Error::Corrupt("review status command kind").into());
            };
            if value.status != status {
                return Err(Error::Corrupt("review status command identity").into());
            }
            entries.extend(value.entries);
            page = value.next;
            check_entry_limit(&entries)?;
        }
        Ok(entries)
    }

    async fn stage_relation_pages(
        &self,
        subject: ReviewRelationSubjectV1,
        entries: &[ReviewThreadRelationV1],
    ) -> std::result::Result<ReviewObjectRef, ReviewCommandError> {
        let mut next = None;
        for chunk in entries.chunks(PAGE_ENTRIES).rev() {
            next = Some(
                self.stage_review_object(&ReviewObjectV1::RelationPage(ReviewRelationPageV1 {
                    schema_version: 1,
                    subject: subject.clone(),
                    entries: chunk.to_vec(),
                    next,
                }))
                .await?,
            );
        }
        next.ok_or_else(|| Error::Corrupt("empty review relation index").into())
    }

    async fn stage_status_pages(
        &self,
        status: ThreadStatusV1,
        entries: &[ReviewThreadRelationV1],
    ) -> std::result::Result<ReviewObjectRef, ReviewCommandError> {
        let mut next = None;
        for chunk in entries.chunks(PAGE_ENTRIES).rev() {
            next = Some(
                self.stage_review_object(&ReviewObjectV1::StatusPage(ReviewStatusPageV1 {
                    schema_version: 1,
                    status,
                    entries: chunk.to_vec(),
                    next,
                }))
                .await?,
            );
        }
        next.ok_or_else(|| Error::Corrupt("empty review status index").into())
    }
}

fn replace_entry(entries: &mut Vec<ReviewThreadRelationV1>, entry: ReviewThreadRelationV1) {
    entries.retain(|value| value.thread_id != entry.thread_id);
    entries.push(entry);
    entries.sort_unstable();
}

fn check_entry_limit(
    entries: &[ReviewThreadRelationV1],
) -> std::result::Result<(), ReviewCommandError> {
    if entries.len() > MAX_INDEX_ENTRIES {
        Err(Error::Limit("review direct index entries").into())
    } else {
        Ok(())
    }
}
