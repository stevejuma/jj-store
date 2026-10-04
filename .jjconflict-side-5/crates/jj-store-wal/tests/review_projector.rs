#[path = "support/review.rs"]
mod native;
#[path = "support/review_provider.rs"]
mod provider;
use jj_review::*;
use jj_store_model::{PrincipalId, PublicationId, RequestId};
use jj_store_wal::publication::{
    PublishOutcome, ReviewCommandError, ReviewCommandRequest, ReviewObjectIo, ReviewProjector,
    ReviewSnapshotRead,
};

fn body(text: &str) -> ReviewCommentBodyV1 {
    ReviewCommentBodyV1 {
        schema_version: 1,
        format: ReviewBodyFormatV1::SafeMarkdown,
        text: text.into(),
        sanitizer_version: "review-markdown-v1".into(),
    }
}
fn request(byte: u8, command: ReviewCommandV1) -> ReviewCommandRequest {
    ReviewCommandRequest::new(
        PrincipalId::new([3; 16]),
        RequestId::new([byte; 16]),
        PublicationId::new([byte; 16]),
        command,
    )
    .unwrap()
}
fn create() -> ReviewCommandV1 {
    ReviewCommandV1::CreateThread {
        anchor: Box::new(ArtifactAnchorV1 {
            schema_version: 1,
            repository_id: native::REPOSITORY,
            target: ArtifactTargetV1::Repository,
            created_context: CreatedContextV1 {
                publication_position: Some(0),
                code_state_version: Some(0),
                agent_root: None,
            },
            sensitivity: SensitivityV1::Repository,
        }),
        body: body("Initial review"),
    }
}

#[tokio::test]
async fn independent_provider_preserves_native_command_bytes_and_pinned_stale_rules() {
    let coordinator = native::fixture().await;
    let create_request = request(10, create());
    let PublishOutcome::Committed(receipt) = coordinator
        .publish_review_command(&create_request)
        .await
        .unwrap()
    else {
        panic!("native publication")
    };
    let result = decode_command_result(&receipt.result).unwrap();
    let manifest = coordinator.read().await.unwrap();
    let native_store = coordinator
        .review_store(manifest.manifest().review_root.unwrap())
        .await
        .unwrap();
    let ReviewObjectV1::ThreadEvent(event) = native_store.read_object(result.event).await.unwrap()
    else {
        panic!("event")
    };
    let local = provider::Provider::new(native::REPOSITORY);
    let empty = local.pin(ReviewRootV1::empty()).await;
    let engine = ReviewProjector::new(&local);
    let projected = engine
        .stage_command(&empty, &create_request, event.accepted_at_millis)
        .await
        .unwrap();
    assert_eq!(projected.result, result);
    assert_eq!(&projected.root, native_store.root());
    assert_eq!(
        encode_root(&projected.root).unwrap(),
        encode_root(native_store.root()).unwrap()
    );
    assert!(matches!(
        empty.read_object(projected.result.thread_snapshot).await,
        Err(jj_store_wal::Error::Stale)
    ));
    let first = local.pin(projected.root).await;
    let reply = request(
        11,
        ReviewCommandV1::AppendReply {
            thread_id: result.thread_id,
            expected_workflow_version: 1,
            parent_comment_id: result.comment_id,
            body: body("Reply"),
        },
    );
    let second = engine.stage_command(&first, &reply, 100).await.unwrap();
    let second_version = ThreadSnapshotId::new(*second.result.thread_snapshot.id.as_bytes());
    let second = local.pin(second.root).await;
    let stale_resolve = request(
        12,
        ReviewCommandV1::ResolveThread {
            thread_id: result.thread_id,
            expected_thread_version: ThreadSnapshotId::new(*result.thread_snapshot.id.as_bytes()),
            reason: None,
        },
    );
    assert!(matches!(
        engine.stage_command(&second, &stale_resolve, 100).await,
        Err(ReviewCommandError::Stale)
    ));
    let resolve = request(
        13,
        ReviewCommandV1::ResolveThread {
            thread_id: result.thread_id,
            expected_thread_version: second_version,
            reason: None,
        },
    );
    let resolved = engine.stage_command(&second, &resolve, 100).await.unwrap();
    let resolved = local.pin(resolved.root).await;
    assert!(matches!(
        engine.stage_command(&resolved, &reply, 100).await,
        Err(ReviewCommandError::ThreadResolved)
    ));
    let ReviewObjectV1::ThreadSnapshot(first_thread) =
        first.read_object(result.thread_snapshot).await.unwrap()
    else {
        panic!("thread")
    };
    assert_eq!(first_thread.thread_version, 1);
    assert_eq!(first_thread.comments.len(), 1);
    let foreign = provider::Provider::new(jj_store_model::RepositoryId::new([9; 16]));
    assert!(matches!(
        ReviewProjector::new(&foreign)
            .stage_command(&first, &create_request, 100)
            .await,
        Err(ReviewCommandError::Invalid("review store repository"))
    ));
}

#[tokio::test]
async fn independent_provider_rejects_forged_event_reconstruction() {
    let local = provider::Provider::new(native::REPOSITORY);
    let engine = ReviewProjector::new(&local);
    let empty = local.pin(ReviewRootV1::empty()).await;
    let projected = engine
        .stage_command(&empty, &request(10, create()), 100)
        .await
        .unwrap();
    let ReviewObjectV1::ThreadSnapshot(mut thread) = local
        .read_review_object(projected.result.thread_snapshot)
        .await
        .unwrap()
    else {
        panic!("thread")
    };
    thread.content_version += 1;
    thread.thread_version += 1;
    thread.last_event_sequence += 1;
    let forged = local
        .stage_review_object(&ReviewObjectV1::ThreadSnapshot(thread))
        .await
        .unwrap();
    let key = ReviewCatalogKey::derive("thread", projected.result.thread_id.as_bytes()).unwrap();
    let delta = ReviewDeltaV1 {
        schema_version: 1,
        edits: vec![ReviewCatalogEditV1 {
            catalog: ReviewCatalogKind::Threads,
            key,
            expected: None,
            replacement: Some(forged),
        }],
    };
    let root = jj_store_wal::publication::ReviewCatalogEditor::new(&local)
        .apply_delta(&empty, &delta)
        .await
        .unwrap();
    assert!(matches!(
        engine.audit_root(&root).await,
        Err(jj_store_wal::Error::Corrupt("review event history tail"))
    ));
}
