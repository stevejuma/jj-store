#[path = "support/review.rs"]
mod support;

use jj_review::{
    AnchorResolutionRequestV1, ArtifactAnchorV1, ArtifactTargetV1, CreatedContextV1,
    ExplicitPlacementTargetV1, PlacementKindV1, ResolverBudgetV1, RetainedTextV1, ReviewActionId,
    ReviewBodyFormatV1, ReviewCatalogKey, ReviewCatalogKind, ReviewCommentBodyV1, ReviewDeltaV1,
    ReviewIndexPageV1, ReviewObjectKind, ReviewObjectV1, ReviewRootV1, ReviewThreadId,
    SensitivityV1, ThreadStatusV1,
};
use jj_store_wal as wal;
use std::collections::BTreeMap;
use support::{REPOSITORY, edits, fixture, stage_thread};

#[tokio::test]
async fn root_pinned_resolver_does_not_publish_repository_state() {
    let coordinator = fixture().await;
    let manifest = coordinator.read().await.unwrap();
    let initial = coordinator
        .review_store(manifest.manifest().review_root.unwrap())
        .await
        .unwrap();
    let thread = stage_thread(&coordinator, 89, None).await;
    let root = coordinator
        .stage_review_delta(
            &initial,
            &ReviewDeltaV1 {
                schema_version: 1,
                edits: edits(&thread),
            },
        )
        .await
        .unwrap();
    let selected = coordinator.review_store(root).await.unwrap();
    let before = coordinator.read().await.unwrap();
    let placement = selected
        .resolve_anchor(
            thread.anchor,
            &AnchorResolutionRequestV1 {
                original_text: RetainedTextV1::Missing,
                target: ExplicitPlacementTargetV1::Repository,
            },
            ResolverBudgetV1::default(),
        )
        .await
        .unwrap();
    let after = coordinator.read().await.unwrap();
    assert_eq!(placement.placement, PlacementKindV1::Exact);
    assert_eq!(before.manifest().published, after.manifest().published);
    assert_eq!(before.manifest().review_root, after.manifest().review_root);
}

#[tokio::test]
async fn persistent_review_catalog_splits_pages_and_keeps_old_root_pinned() {
    let coordinator = fixture().await;
    let manifest = coordinator.read().await.unwrap();
    let initial = coordinator
        .review_store(manifest.manifest().review_root.unwrap())
        .await
        .unwrap();
    let mut delta = Vec::new();
    let mut threads = Vec::new();
    for byte in 20..60 {
        let thread = stage_thread(&coordinator, byte, None).await;
        delta.extend(edits(&thread));
        threads.push(thread);
    }
    let root = coordinator
        .stage_review_delta(
            &initial,
            &ReviewDeltaV1 {
                schema_version: 1,
                edits: delta,
            },
        )
        .await
        .unwrap();
    let selected = coordinator.review_store(root).await.unwrap();
    let first = selected.list_threads(None, 10).await.unwrap();
    let second = selected.list_threads(first.next_after, 10).await.unwrap();
    assert_eq!(first.selections.len(), 10);
    assert_eq!(second.selections.len(), 10);
    assert!(
        first
            .selections
            .iter()
            .all(|entry| !second.selections.contains(entry))
    );

    let key = ReviewCatalogKey::derive("thread", threads[0].thread_id.as_bytes()).unwrap();
    assert_eq!(
        initial
            .lookup(ReviewCatalogKind::Threads, key)
            .await
            .unwrap(),
        None
    );
    assert_eq!(
        selected
            .lookup(ReviewCatalogKind::Threads, key)
            .await
            .unwrap(),
        Some(threads[0].snapshot)
    );
    let unselected = coordinator
        .stage_review_object(&ReviewObjectV1::CommentBody(ReviewCommentBodyV1 {
            schema_version: 1,
            format: ReviewBodyFormatV1::PlainText,
            text: "not selected".into(),
            sanitizer_version: "review-plain-v1".into(),
        }))
        .await
        .unwrap();
    assert!(matches!(
        selected.read_object(unselected).await,
        Err(wal::Error::Stale)
    ));
}

#[tokio::test]
async fn selecting_a_thread_rejects_cross_object_thread_identity_mismatch() {
    let coordinator = fixture().await;
    let manifest = coordinator.read().await.unwrap();
    let initial = coordinator
        .review_store(manifest.manifest().review_root.unwrap())
        .await
        .unwrap();
    let mismatched = stage_thread(&coordinator, 80, Some(ReviewThreadId::new([81; 16]))).await;
    let result = coordinator
        .stage_review_delta(
            &initial,
            &ReviewDeltaV1 {
                schema_version: 1,
                edits: edits(&mismatched),
            },
        )
        .await;
    assert!(matches!(result, Err(wal::Error::Corrupt(_))));
}

#[tokio::test]
async fn review_root_rejects_snapshot_state_not_reconstructed_by_events() {
    let coordinator = fixture().await;
    let manifest = coordinator.read().await.unwrap();
    let initial = coordinator
        .review_store(manifest.manifest().review_root.unwrap())
        .await
        .unwrap();
    let thread = stage_thread(&coordinator, 82, None).await;
    let mut snapshot = thread.snapshot_value.clone();
    snapshot.status = ThreadStatusV1::Resolved;
    snapshot.selected_workflow_action = Some(ReviewActionId::new([83; 16]));
    let tampered = coordinator
        .stage_review_object(&ReviewObjectV1::ThreadSnapshot(Box::new(snapshot)))
        .await
        .unwrap();
    let mut selected = edits(&thread);
    selected[0].replacement = Some(tampered);
    assert!(matches!(
        coordinator
            .stage_review_delta(
                &initial,
                &ReviewDeltaV1 {
                    schema_version: 1,
                    edits: selected,
                },
            )
            .await,
        Err(wal::Error::Corrupt(_))
    ));
}

#[tokio::test]
async fn review_root_rejects_catalog_values_with_the_wrong_semantic_kind() {
    let coordinator = fixture().await;
    let anchor = coordinator
        .stage_review_object(&ReviewObjectV1::Anchor(Box::new(ArtifactAnchorV1 {
            schema_version: 1,
            repository_id: REPOSITORY,
            target: ArtifactTargetV1::Repository,
            created_context: CreatedContextV1 {
                publication_position: Some(0),
                code_state_version: Some(0),
                agent_root: None,
            },
            sensitivity: SensitivityV1::Repository,
        })))
        .await
        .unwrap();
    assert_eq!(anchor.kind, ReviewObjectKind::Anchor);
    let key = ReviewCatalogKey::derive("thread", b"wrong-kind").unwrap();
    let page = coordinator
        .stage_review_object(&ReviewObjectV1::IndexPage(ReviewIndexPageV1::Leaf {
            prefix: String::new(),
            entries: BTreeMap::from([(key, anchor)]),
        }))
        .await
        .unwrap();
    let root = ReviewRootV1 {
        threads: Some(page),
        ..ReviewRootV1::empty()
    };
    assert!(matches!(
        coordinator.stage_review_root(&root).await,
        Err(wal::Error::Corrupt(_))
    ));
}
