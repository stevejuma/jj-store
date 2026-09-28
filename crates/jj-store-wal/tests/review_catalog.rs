use jj_review::{
    ArtifactAnchorV1, ArtifactTargetV1, CommentRevisionContentV1, CommentSelectionV1,
    CreatedContextV1, ReviewBodyFormatV1, ReviewCatalogEditV1, ReviewCatalogKey, ReviewCatalogKind,
    ReviewCommentBodyV1, ReviewCommentRevisionV1, ReviewDeltaV1, ReviewEventPageV1,
    ReviewEventSelectionV1, ReviewIndexPageV1, ReviewObjectKind, ReviewObjectRef, ReviewObjectV1,
    ReviewRootV1, ReviewThreadEventKindV1, ReviewThreadEventV1, ReviewThreadId,
    ReviewThreadSnapshotV1, SensitivityV1, ThreadStatusV1,
};
use jj_store_model::{EpochId, ObjectKey, OperationId, PrincipalId, RepositoryId};
use jj_store_object::memory::MemoryStore;
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication::{Coordinator, Limits, PublicationGuard, Request, Snapshot},
};
use std::{collections::BTreeMap, collections::BTreeSet, sync::Arc};

const REPOSITORY: RepositoryId = RepositoryId::new([1; 16]);

struct Source(SourceScope);
#[async_trait::async_trait]
impl DependencySource for Source {
    fn scope(&self) -> SourceScope {
        self.0
    }
    async fn load_verified(&self, key: ObjectKey, _: u64) -> wal::inventory::Result<VerifiedNode> {
        if key != ObjectKey::Operation(OperationId::new([0; 64])) {
            return Err(wal::inventory::InventoryError::Source(
                "unexpected fixture object".into(),
            ));
        }
        Ok(VerifiedNode {
            key,
            representation: Representation::Virtual,
            dependencies: BTreeSet::new(),
        })
    }
}

struct Permit;
#[async_trait::async_trait]
impl PublicationGuard for Permit {
    async fn authorize(
        &self,
        _: &wal::publication::ManifestSnapshot,
        _: &Request,
    ) -> wal::Result<()> {
        Ok(())
    }
    async fn validate_transition(
        &self,
        _: &wal::publication::ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize_domain(
        &self,
        _: &wal::publication::ManifestSnapshot,
        _: &wal::publication::DomainRequest,
    ) -> wal::Result<()> {
        Ok(())
    }
}

async fn fixture() -> Coordinator {
    let coordinator = Coordinator::new(
        Arc::new(MemoryStore::default()),
        REPOSITORY,
        Arc::new(Source(SourceScope::new(REPOSITORY))),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap();
    coordinator
        .initialize(EpochId::new([2; 16]), Vec::new())
        .await
        .unwrap();
    coordinator
}

struct ThreadObjects {
    thread_id: ReviewThreadId,
    comment_id: jj_review::CommentId,
    snapshot: ReviewObjectRef,
    revision: ReviewObjectRef,
    events: ReviewObjectRef,
}

async fn stage_thread(
    coordinator: &Coordinator,
    byte: u8,
    revision_thread: Option<ReviewThreadId>,
) -> ThreadObjects {
    let thread_id = ReviewThreadId::new([byte; 16]);
    let comment_id = jj_review::CommentId::new([byte.wrapping_add(1); 16]);
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
    let body = coordinator
        .stage_review_object(&ReviewObjectV1::CommentBody(ReviewCommentBodyV1 {
            schema_version: 1,
            format: ReviewBodyFormatV1::SafeMarkdown,
            text: format!("Thread {byte}"),
            sanitizer_version: "review-markdown-v1".into(),
        }))
        .await
        .unwrap();
    let revision = coordinator
        .stage_review_object(&ReviewObjectV1::CommentRevision(Box::new(
            ReviewCommentRevisionV1 {
                schema_version: 1,
                thread_id: revision_thread.unwrap_or(thread_id),
                comment_id,
                author: PrincipalId::new([4; 16]),
                actor: PrincipalId::new([4; 16]),
                created_at_millis: 10,
                previous_revision: None,
                content: CommentRevisionContentV1::Visible { body },
            },
        )))
        .await
        .unwrap();
    let event = coordinator
        .stage_review_object(&ReviewObjectV1::ThreadEvent(Box::new(
            ReviewThreadEventV1 {
                schema_version: 1,
                thread_id,
                sequence: 1,
                actor: PrincipalId::new([4; 16]),
                accepted_at_millis: 10,
                event: ReviewThreadEventKindV1::Created {
                    anchor,
                    initial_comment_id: comment_id,
                    initial_revision: revision,
                },
            },
        )))
        .await
        .unwrap();
    let events = coordinator
        .stage_review_object(&ReviewObjectV1::EventPage(ReviewEventPageV1 {
            schema_version: 1,
            thread_id,
            entries: vec![ReviewEventSelectionV1 { sequence: 1, event }],
            next: None,
        }))
        .await
        .unwrap();
    let snapshot = coordinator
        .stage_review_object(&ReviewObjectV1::ThreadSnapshot(Box::new(
            ReviewThreadSnapshotV1 {
                schema_version: 1,
                thread_id,
                original_anchor: anchor,
                comments: vec![CommentSelectionV1 {
                    comment_id,
                    revision,
                    parent_comment_id: None,
                }],
                event_page_root: events,
                last_event_sequence: 1,
                status: ThreadStatusV1::Open,
                selected_workflow_action: None,
                thread_version: 1,
                content_version: 1,
                workflow_version: 1,
            },
        )))
        .await
        .unwrap();
    ThreadObjects {
        thread_id,
        comment_id,
        snapshot,
        revision,
        events,
    }
}

fn edits(value: &ThreadObjects) -> Vec<ReviewCatalogEditV1> {
    vec![
        ReviewCatalogEditV1 {
            catalog: ReviewCatalogKind::Threads,
            key: ReviewCatalogKey::derive("thread", value.thread_id.as_bytes()).unwrap(),
            expected: None,
            replacement: Some(value.snapshot),
        },
        ReviewCatalogEditV1 {
            catalog: ReviewCatalogKind::Comments,
            key: ReviewCatalogKey::derive("comment", value.comment_id.as_bytes()).unwrap(),
            expected: None,
            replacement: Some(value.revision),
        },
        ReviewCatalogEditV1 {
            catalog: ReviewCatalogKind::ThreadEvents,
            key: ReviewCatalogKey::derive("thread-events", value.thread_id.as_bytes()).unwrap(),
            expected: None,
            replacement: Some(value.events),
        },
    ]
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
