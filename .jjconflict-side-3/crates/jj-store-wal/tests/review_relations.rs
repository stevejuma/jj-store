#[path = "support/review.rs"]
mod support;

use jj_review::{
    ReviewCatalogEditV1, ReviewCatalogKind, ReviewDeltaV1, ReviewObjectV1, ReviewRelationPageV1,
    ReviewRelationSubjectV1, ReviewStatusPageV1, ReviewThreadRelationV1, ThreadStatusV1,
};
use jj_store_wal as wal;
use support::{edits, fixture, stage_thread};

#[tokio::test]
async fn typed_relation_and_status_catalogs_validate_selected_threads() {
    let coordinator = fixture().await;
    let manifest = coordinator.read().await.unwrap();
    let initial = coordinator
        .review_store(manifest.manifest().review_root.unwrap())
        .await
        .unwrap();
    let thread = stage_thread(&coordinator, 90, None).await;
    let entry = ReviewThreadRelationV1 {
        thread_id: thread.thread_id,
        snapshot: thread.snapshot,
    };
    let anchor_subject = ReviewRelationSubjectV1::OriginalAnchor {
        anchor_id: thread.anchor_id,
    };
    let artifact_subject = ReviewRelationSubjectV1::Artifact {
        artifact_id: thread.artifact_id,
    };
    let anchor_relation = coordinator
        .stage_review_object(&ReviewObjectV1::RelationPage(ReviewRelationPageV1 {
            schema_version: 1,
            subject: anchor_subject.clone(),
            entries: vec![entry],
            next: None,
        }))
        .await
        .unwrap();
    let artifact_relation = coordinator
        .stage_review_object(&ReviewObjectV1::RelationPage(ReviewRelationPageV1 {
            schema_version: 1,
            subject: artifact_subject.clone(),
            entries: vec![entry],
            next: None,
        }))
        .await
        .unwrap();
    let status = coordinator
        .stage_review_object(&ReviewObjectV1::StatusPage(ReviewStatusPageV1 {
            schema_version: 1,
            status: ThreadStatusV1::Open,
            entries: vec![entry],
            next: None,
        }))
        .await
        .unwrap();
    let mut selected = edits(&thread);
    selected.extend([
        ReviewCatalogEditV1 {
            catalog: ReviewCatalogKind::AnchorRelations,
            key: anchor_subject.catalog_key().unwrap(),
            expected: None,
            replacement: Some(anchor_relation),
        },
        ReviewCatalogEditV1 {
            catalog: ReviewCatalogKind::ArtifactRelations,
            key: artifact_subject.catalog_key().unwrap(),
            expected: None,
            replacement: Some(artifact_relation),
        },
        ReviewCatalogEditV1 {
            catalog: ReviewCatalogKind::Statuses,
            key: jj_review::review_status_key(ThreadStatusV1::Open).unwrap(),
            expected: None,
            replacement: Some(status),
        },
    ]);
    let root = coordinator
        .stage_review_delta(
            &initial,
            &ReviewDeltaV1 {
                schema_version: 1,
                edits: selected,
            },
        )
        .await
        .unwrap();
    let snapshot = coordinator.review_store(root).await.unwrap();
    assert_eq!(
        snapshot
            .lookup(
                ReviewCatalogKind::ArtifactRelations,
                artifact_subject.catalog_key().unwrap(),
            )
            .await
            .unwrap(),
        Some(artifact_relation)
    );

    let wrong_status = coordinator
        .stage_review_object(&ReviewObjectV1::StatusPage(ReviewStatusPageV1 {
            schema_version: 1,
            status: ThreadStatusV1::Resolved,
            entries: vec![entry],
            next: None,
        }))
        .await
        .unwrap();
    assert!(matches!(
        coordinator
            .stage_review_delta(
                &snapshot,
                &ReviewDeltaV1 {
                    schema_version: 1,
                    edits: vec![ReviewCatalogEditV1 {
                        catalog: ReviewCatalogKind::Statuses,
                        key: jj_review::review_status_key(ThreadStatusV1::Resolved).unwrap(),
                        expected: None,
                        replacement: Some(wrong_status),
                    }],
                },
            )
            .await,
        Err(wal::Error::Corrupt(_))
    ));
}
