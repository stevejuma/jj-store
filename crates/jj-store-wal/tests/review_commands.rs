use jj_review::{
    ArtifactAnchorV1, ArtifactTargetV1, CommentRevisionContentV1, CommentRevisionId,
    CreatedContextV1, ReviewBodyFormatV1, ReviewCatalogKey, ReviewCatalogKind,
    ReviewCommandResultV1, ReviewCommandV1, ReviewCommentBodyV1, ReviewObjectRef, ReviewObjectV1,
    ReviewRelationSubjectV1, ReviewThreadSnapshotV1, SensitivityV1, ThreadSnapshotId,
    ThreadStatusV1, decode_command_result, review_status_key,
};
use jj_store_model::{EpochId, ObjectKey, PrincipalId, PublicationId, RepositoryId, RequestId};
use jj_store_object::memory::MemoryStore;
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication::{
        Coordinator, DomainRequest, Limits, ManifestSnapshot, PublicationGuard, PublishOutcome,
        Request, ReviewCommandError, ReviewCommandRequest, Snapshot,
    },
};
use std::{collections::BTreeSet, sync::Arc};

const REPOSITORY: RepositoryId = RepositoryId::new([1; 16]);
const ACTOR: PrincipalId = PrincipalId::new([3; 16]);

struct Source(SourceScope);
#[async_trait::async_trait]
impl DependencySource for Source {
    fn scope(&self) -> SourceScope {
        self.0
    }
    async fn load_verified(&self, key: ObjectKey, _: u64) -> wal::inventory::Result<VerifiedNode> {
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
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }
    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize_domain(&self, _: &ManifestSnapshot, _: &DomainRequest) -> wal::Result<()> {
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

fn body(text: &str) -> ReviewCommentBodyV1 {
    ReviewCommentBodyV1 {
        schema_version: 1,
        format: ReviewBodyFormatV1::SafeMarkdown,
        text: text.into(),
        sanitizer_version: "review-markdown-v1".into(),
    }
}

fn create() -> ReviewCommandV1 {
    ReviewCommandV1::CreateThread {
        anchor: Box::new(ArtifactAnchorV1 {
            schema_version: 1,
            repository_id: REPOSITORY,
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

fn request(byte: u8, command: ReviewCommandV1) -> ReviewCommandRequest {
    ReviewCommandRequest::new(
        ACTOR,
        RequestId::new([byte; 16]),
        PublicationId::new([byte; 16]),
        command,
    )
    .unwrap()
}

async fn publish(
    coordinator: &Coordinator,
    request: &ReviewCommandRequest,
) -> ReviewCommandResultV1 {
    let receipt = match coordinator.publish_review_command(request).await.unwrap() {
        PublishOutcome::Committed(receipt) => receipt,
        other => panic!("expected committed command, got {other:?}"),
    };
    decode_command_result(&receipt.result).unwrap()
}

async fn store(coordinator: &Coordinator) -> wal::publication::ReviewStoreSnapshot {
    let manifest = coordinator.read().await.unwrap();
    coordinator
        .review_store(manifest.manifest().review_root.unwrap())
        .await
        .unwrap()
}

async fn snapshot(
    store: &wal::publication::ReviewStoreSnapshot,
    reference: ReviewObjectRef,
) -> ReviewThreadSnapshotV1 {
    let ReviewObjectV1::ThreadSnapshot(value) = store.read_object(reference).await.unwrap() else {
        panic!("expected thread snapshot")
    };
    *value
}

fn snapshot_id(reference: ReviewObjectRef) -> ThreadSnapshotId {
    ThreadSnapshotId::from_slice(reference.id.as_bytes()).unwrap()
}

fn revision_id(reference: ReviewObjectRef) -> CommentRevisionId {
    CommentRevisionId::from_slice(reference.id.as_bytes()).unwrap()
}

async fn indexed_snapshot(
    store: &wal::publication::ReviewStoreSnapshot,
    catalog: ReviewCatalogKind,
    key: ReviewCatalogKey,
) -> Option<ReviewObjectRef> {
    let selected = store.lookup(catalog, key).await.unwrap()?;
    match store.read_object(selected).await.unwrap() {
        ReviewObjectV1::RelationPage(page) => page.entries.first().map(|entry| entry.snapshot),
        ReviewObjectV1::StatusPage(page) => page.entries.first().map(|entry| entry.snapshot),
        _ => panic!("expected relation or status page"),
    }
}

#[tokio::test]
async fn create_thread_is_atomic_replayable_and_preserves_sibling_roots() {
    let coordinator = fixture().await;
    let before = coordinator.read().await.unwrap().manifest().clone();
    let create_request = request(10, create());
    let created = publish(&coordinator, &create_request).await;
    let after = coordinator.read().await.unwrap().manifest().clone();
    assert_eq!(after.heads, before.heads);
    assert_eq!(after.code_state_version, before.code_state_version);
    assert_eq!(after.agent_root, before.agent_root);
    assert_eq!(after.agent_state_version, before.agent_state_version);
    assert_eq!(after.review_state_version, before.review_state_version + 1);

    let selected = store(&coordinator).await;
    let thread = snapshot(&selected, created.thread_snapshot).await;
    assert_eq!(thread.comments.len(), 1);
    let ReviewObjectV1::Anchor(anchor) =
        selected.read_object(thread.original_anchor).await.unwrap()
    else {
        panic!("expected original anchor")
    };
    for (catalog, key) in [
        (
            ReviewCatalogKind::AnchorRelations,
            ReviewRelationSubjectV1::OriginalAnchor {
                anchor_id: anchor.anchor_id().unwrap(),
            }
            .catalog_key()
            .unwrap(),
        ),
        (
            ReviewCatalogKind::ArtifactRelations,
            ReviewRelationSubjectV1::Artifact {
                artifact_id: anchor.artifact_id().unwrap(),
            }
            .catalog_key()
            .unwrap(),
        ),
        (
            ReviewCatalogKind::Statuses,
            review_status_key(ThreadStatusV1::Open).unwrap(),
        ),
    ] {
        assert_eq!(
            indexed_snapshot(&selected, catalog, key).await,
            Some(created.thread_snapshot)
        );
    }
    let ReviewObjectV1::CommentRevision(revision) = selected
        .read_object(created.comment_revision.unwrap())
        .await
        .unwrap()
    else {
        panic!("expected initial revision")
    };
    assert_eq!(revision.author, ACTOR);
    assert_eq!(revision.actor, ACTOR);
    publish(
        &coordinator,
        &request(
            11,
            ReviewCommandV1::AppendReply {
                thread_id: created.thread_id,
                expected_workflow_version: 1,
                parent_comment_id: created.comment_id,
                body: body("Later work"),
            },
        ),
    )
    .await;
    assert_eq!(publish(&coordinator, &create_request).await, created);

    let changed = request(
        10,
        ReviewCommandV1::CreateThread {
            anchor: match create() {
                ReviewCommandV1::CreateThread { anchor, .. } => anchor,
                _ => unreachable!(),
            },
            body: body("different intent"),
        },
    );
    assert!(matches!(
        coordinator.publish_review_command(&changed).await,
        Err(ReviewCommandError::Publication(wal::Error::IntentConflict))
    ));
}

#[tokio::test]
async fn replies_with_the_same_workflow_version_both_survive() {
    let coordinator = fixture().await;
    let created = publish(&coordinator, &request(20, create())).await;
    for (byte, text) in [(21, "First reply"), (22, "Second reply")] {
        publish(
            &coordinator,
            &request(
                byte,
                ReviewCommandV1::AppendReply {
                    thread_id: created.thread_id,
                    expected_workflow_version: 1,
                    parent_comment_id: created.comment_id,
                    body: body(text),
                },
            ),
        )
        .await;
    }
    let selected = store(&coordinator).await;
    let current = selected
        .lookup_thread(created.thread_id)
        .await
        .unwrap()
        .unwrap();
    let thread = snapshot(&selected, current).await;
    assert_eq!(thread.comments.len(), 3);
    assert_eq!(thread.thread_version, 3);
    assert_eq!(thread.content_version, 3);
    assert_eq!(thread.workflow_version, 1);
}

#[tokio::test]
async fn edit_requires_the_exact_selected_comment_revision() {
    let coordinator = fixture().await;
    let created = publish(&coordinator, &request(30, create())).await;
    let expected = created.comment_revision.unwrap();
    let edit = |byte, text| {
        request(
            byte,
            ReviewCommandV1::EditComment {
                comment_id: created.comment_id.unwrap(),
                expected_revision_id: revision_id(expected),
                replacement_body: body(text),
            },
        )
    };
    let edited = publish(&coordinator, &edit(31, "Edited")).await;
    assert!(matches!(
        coordinator
            .publish_review_command(&edit(32, "Lost edit"))
            .await,
        Err(ReviewCommandError::Stale)
    ));
    let selected = store(&coordinator).await;
    let ReviewObjectV1::CommentRevision(revision) = selected
        .read_object(edited.comment_revision.unwrap())
        .await
        .unwrap()
    else {
        panic!("expected edited revision")
    };
    assert_eq!(revision.previous_revision, Some(expected));
}

#[tokio::test]
async fn tombstone_and_moderated_hide_keep_append_only_history() {
    let coordinator = fixture().await;
    let created = publish(&coordinator, &request(40, create())).await;
    let tombstone = publish(
        &coordinator,
        &request(
            41,
            ReviewCommandV1::TombstoneComment {
                comment_id: created.comment_id.unwrap(),
                expected_revision_id: revision_id(created.comment_revision.unwrap()),
                reason: Some("withdrawn".into()),
            },
        ),
    )
    .await;
    let hidden = publish(
        &coordinator,
        &request(
            42,
            ReviewCommandV1::HideComment {
                comment_id: created.comment_id.unwrap(),
                expected_revision_id: revision_id(tombstone.comment_revision.unwrap()),
                moderation_reason: "secret detected".into(),
            },
        ),
    )
    .await;
    let selected = store(&coordinator).await;
    let ReviewObjectV1::CommentRevision(hidden_revision) = selected
        .read_object(hidden.comment_revision.unwrap())
        .await
        .unwrap()
    else {
        panic!("expected hidden revision")
    };
    assert!(matches!(
        hidden_revision.content,
        CommentRevisionContentV1::Hidden { .. }
    ));
    assert_eq!(
        hidden_revision.previous_revision,
        tombstone.comment_revision
    );
    let ReviewObjectV1::CommentRevision(tombstoned) = selected
        .read_object(tombstone.comment_revision.unwrap())
        .await
        .unwrap()
    else {
        panic!("expected tombstoned revision")
    };
    assert!(matches!(
        tombstoned.content,
        CommentRevisionContentV1::Tombstoned { .. }
    ));
    assert_eq!(tombstoned.previous_revision, created.comment_revision);
}

#[tokio::test]
async fn reply_losing_a_race_with_resolve_is_rejected() {
    let coordinator = fixture().await;
    let created = publish(&coordinator, &request(50, create())).await;
    let resolved = publish(
        &coordinator,
        &request(
            51,
            ReviewCommandV1::ResolveThread {
                thread_id: created.thread_id,
                expected_thread_version: snapshot_id(created.thread_snapshot),
                reason: None,
            },
        ),
    )
    .await;
    assert!(matches!(
        coordinator
            .publish_review_command(&request(
                52,
                ReviewCommandV1::AppendReply {
                    thread_id: created.thread_id,
                    expected_workflow_version: 1,
                    parent_comment_id: None,
                    body: body("too late"),
                }
            ))
            .await,
        Err(ReviewCommandError::ThreadResolved)
    ));
    let selected = store(&coordinator).await;
    assert_eq!(
        indexed_snapshot(
            &selected,
            ReviewCatalogKind::Statuses,
            review_status_key(ThreadStatusV1::Open).unwrap(),
        )
        .await,
        None
    );
    assert_eq!(
        indexed_snapshot(
            &selected,
            ReviewCatalogKind::Statuses,
            review_status_key(ThreadStatusV1::Resolved).unwrap(),
        )
        .await,
        Some(resolved.thread_snapshot)
    );
}

#[tokio::test]
async fn resolve_rejects_an_unseen_reply() {
    let coordinator = fixture().await;
    let created = publish(&coordinator, &request(60, create())).await;
    publish(
        &coordinator,
        &request(
            61,
            ReviewCommandV1::AppendReply {
                thread_id: created.thread_id,
                expected_workflow_version: 1,
                parent_comment_id: None,
                body: body("unseen"),
            },
        ),
    )
    .await;
    assert!(matches!(
        coordinator
            .publish_review_command(&request(
                62,
                ReviewCommandV1::ResolveThread {
                    thread_id: created.thread_id,
                    expected_thread_version: snapshot_id(created.thread_snapshot),
                    reason: None,
                }
            ))
            .await,
        Err(ReviewCommandError::Stale)
    ));
}

#[tokio::test]
async fn reopen_must_commit_before_another_reply() {
    let coordinator = fixture().await;
    let created = publish(&coordinator, &request(70, create())).await;
    let resolved = publish(
        &coordinator,
        &request(
            71,
            ReviewCommandV1::ResolveThread {
                thread_id: created.thread_id,
                expected_thread_version: snapshot_id(created.thread_snapshot),
                reason: Some("fixed".into()),
            },
        ),
    )
    .await;
    let reopened = publish(
        &coordinator,
        &request(
            72,
            ReviewCommandV1::ReopenThread {
                thread_id: created.thread_id,
                expected_thread_version: snapshot_id(resolved.thread_snapshot),
                reason: Some("regressed".into()),
            },
        ),
    )
    .await;
    let reply = publish(
        &coordinator,
        &request(
            73,
            ReviewCommandV1::AppendReply {
                thread_id: created.thread_id,
                expected_workflow_version: 3,
                parent_comment_id: None,
                body: body("reopened reply"),
            },
        ),
    )
    .await;
    let selected = store(&coordinator).await;
    let thread = snapshot(&selected, reply.thread_snapshot).await;
    assert_eq!(thread.status, ThreadStatusV1::Open);
    assert_eq!(thread.comments.len(), 2);
    assert_eq!(thread.workflow_version, 3);
    assert_ne!(reopened.thread_snapshot, resolved.thread_snapshot);
}
