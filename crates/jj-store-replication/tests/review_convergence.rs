use jj_store_model::*;
use jj_store_object::{ObjectStore, memory::MemoryStore};
use jj_store_replication::{TransferLimits, transfer_closures};
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, InventoryError, SourceScope, VerifiedNode},
    publication::*,
};
use std::{collections::BTreeMap, sync::Arc};

const REPOSITORY: RepositoryId = RepositoryId::new([71; 16]);

struct Permit;

#[async_trait::async_trait]
impl PublicationGuard for Permit {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }

    async fn authorize_domain(&self, _: &ManifestSnapshot, _: &DomainRequest) -> wal::Result<()> {
        Ok(())
    }

    async fn authorize_replication(
        &self,
        _: &ManifestSnapshot,
        _: &ApplyOriginsRequestV1,
    ) -> wal::Result<()> {
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
}

#[derive(Clone)]
struct VirtualSource;

#[async_trait::async_trait]
impl DependencySource for VirtualSource {
    fn scope(&self) -> SourceScope {
        SourceScope::new(REPOSITORY)
    }

    async fn load_verified(&self, key: ObjectKey, _: u64) -> wal::inventory::Result<VerifiedNode> {
        if key == ObjectKey::Operation(OperationId::new([0; 64])) {
            Ok(VerifiedNode {
                key,
                representation: wal::inventory::Representation::Virtual,
                dependencies: Default::default(),
            })
        } else {
            Err(InventoryError::Source("unexpected native object".into()))
        }
    }
}

struct Replica {
    coordinator: Coordinator,
    store: Arc<dyn ObjectStore>,
}

impl Replica {
    async fn new(seed: u8) -> Self {
        let store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
        let coordinator = Coordinator::new(
            store.clone(),
            REPOSITORY,
            Arc::new(VirtualSource),
            Arc::new(Permit),
            Limits::default(),
        )
        .unwrap();
        coordinator
            .initialize_replica(
                EpochId::new([seed; 16]),
                format!("replica-{seed}").into_bytes(),
                ReplicaId::new([seed.wrapping_add(1); 16]),
                WriterEpoch::new([seed.wrapping_add(2); 16]),
            )
            .await
            .unwrap();
        Self { coordinator, store }
    }

    async fn origins(&self) -> Vec<OriginEnvelopeV1> {
        let snapshot = self.coordinator.read().await.unwrap();
        self.coordinator
            .committed_origin_feed(&snapshot, WalPosition(0), 64)
            .await
            .unwrap()
            .origins
            .into_iter()
            .map(|item| item.envelope)
            .collect()
    }

    async fn transfer_to(&self, destination: &Replica) {
        let snapshot = self.coordinator.read().await.unwrap();
        let feed = self
            .coordinator
            .committed_origin_feed(&snapshot, WalPosition(0), 64)
            .await
            .unwrap();
        let mut closures = Vec::new();
        for item in feed.origins {
            closures.push(
                self.coordinator
                    .origin_closure(&snapshot, item.envelope.publication.origin_key, 20_000)
                    .await
                    .unwrap(),
            );
        }
        transfer_closures(
            self.store.clone(),
            destination.store.clone(),
            REPOSITORY,
            &closures,
            TransferLimits::default(),
        )
        .await
        .unwrap();
    }

    async fn apply(&self, seed: u8, origins: Vec<OriginEnvelopeV1>) -> ApplyOriginsOutcomeV1 {
        self.coordinator
            .apply_origins(&ApplyOriginsRequestV1 {
                schema_version: 1,
                principal: PrincipalId::new([seed; 16]),
                request_id: RequestId::new([seed.wrapping_add(1); 16]),
                publication_id: PublicationId::new([seed.wrapping_add(2); 16]),
                origins,
            })
            .await
            .unwrap_or_else(|error| panic!("origin apply {seed} failed: {error:?}"))
    }

    async fn review_store(&self) -> ReviewStoreSnapshot {
        let snapshot = self.coordinator.read().await.unwrap();
        self.coordinator
            .review_store(snapshot.manifest().domains().unwrap().review.root)
            .await
            .unwrap()
    }

    async fn cold_copy(&self) -> Self {
        self.coordinator.checkpoint().await.unwrap();
        let store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
        let source: Arc<dyn DependencySource> = Arc::new(VirtualSource);
        self.coordinator
            .copy_native_to(store.clone(), source.clone(), 512)
            .await
            .unwrap();
        let coordinator = Coordinator::new(
            store.clone(),
            REPOSITORY,
            source,
            Arc::new(Permit),
            Limits::default(),
        )
        .unwrap();
        coordinator.recover(512).await.unwrap();
        Self { coordinator, store }
    }
}

fn body(text: &str) -> ReviewCommentBodyV1 {
    ReviewCommentBodyV1 {
        schema_version: 1,
        format: ReviewBodyFormatV1::SafeMarkdown,
        text: text.into(),
        sanitizer_version: "replication-test-v1".into(),
    }
}

fn request(seed: u8, command: ReviewCommandV1) -> ReviewCommandRequest {
    ReviewCommandRequest::new(
        PrincipalId::new([90; 16]),
        RequestId::new([seed; 16]),
        PublicationId::new([seed; 16]),
        command,
    )
    .unwrap()
}

async fn publish(replica: &Replica, seed: u8, command: ReviewCommandV1) -> ReviewCommandResultV1 {
    let outcome = replica
        .coordinator
        .publish_review_command(&request(seed, command))
        .await
        .unwrap_or_else(|error| panic!("review command {seed} failed: {error:?}"));
    let PublishOutcome::Committed(receipt) = outcome else {
        panic!("review command did not commit")
    };
    decode_command_result(&receipt.result).unwrap()
}

async fn baseline() -> (Replica, ReviewCommandResultV1) {
    let source = Replica::new(1).await;
    let created = publish(
        &source,
        10,
        ReviewCommandV1::CreateThread {
            anchor: Box::new(ArtifactAnchorV1 {
                schema_version: 1,
                repository_id: REPOSITORY,
                target: ArtifactTargetV1::Repository,
                created_context: CreatedContextV1 {
                    publication_position: None,
                    code_state_version: None,
                    agent_root: None,
                },
                sensitivity: SensitivityV1::Repository,
            }),
            body: body("initial"),
        },
    )
    .await;
    (source, created)
}

async fn branch_from(source: &Replica, seed: u8) -> Replica {
    let branch = Replica::new(seed).await;
    source.transfer_to(&branch).await;
    branch
        .apply(seed.wrapping_add(150), source.origins().await)
        .await;
    branch
}

fn unique_origins(origins: impl IntoIterator<Item = OriginEnvelopeV1>) -> Vec<OriginEnvelopeV1> {
    origins
        .into_iter()
        .map(|origin| (origin.publication.origin_key, origin))
        .collect::<BTreeMap<_, _>>()
        .into_values()
        .collect()
}

async fn merge_branches(first: &Replica, second: &Replica, seed: u8, reverse: bool) -> Replica {
    let destination = Replica::new(seed).await;
    first.transfer_to(&destination).await;
    second.transfer_to(&destination).await;
    let mut origins = unique_origins(
        first
            .origins()
            .await
            .into_iter()
            .chain(second.origins().await),
    );
    if reverse {
        origins.reverse();
    }
    destination.apply(seed.wrapping_add(20), origins).await;
    destination
}

#[tokio::test]
async fn concurrent_comment_edits_retain_all_maximal_revisions_and_converge() {
    let (source, created) = baseline().await;
    let first = branch_from(&source, 20).await;
    let second = branch_from(&source, 30).await;
    let initial_revision =
        CommentRevisionId::from_slice(created.comment_revision.unwrap().id.as_bytes()).unwrap();
    for (replica, seed, text) in [(&first, 41, "first edit"), (&second, 42, "second edit")] {
        publish(
            replica,
            seed,
            ReviewCommandV1::EditComment {
                comment_id: created.comment_id.unwrap(),
                expected_revision_id: initial_revision,
                replacement_body: body(text),
            },
        )
        .await;
    }

    let forward = merge_branches(&first, &second, 50, false).await;
    let reverse = merge_branches(&first, &second, 60, true).await;
    let forward_snapshot = forward.coordinator.read().await.unwrap();
    let reverse_snapshot = reverse.coordinator.read().await.unwrap();
    assert_eq!(
        forward_snapshot.manifest().domains().unwrap().review.root,
        reverse_snapshot.manifest().domains().unwrap().review.root
    );

    let store = forward.review_store().await;
    let selected = store
        .lookup_thread(created.thread_id)
        .await
        .unwrap()
        .unwrap();
    let ReviewObjectV1::ThreadMerge(thread) = store.read_object(selected).await.unwrap() else {
        panic!("concurrent thread should select an explicit merge")
    };
    let comment = thread
        .comments
        .iter()
        .find(|comment| comment.comment_id == created.comment_id.unwrap())
        .unwrap();
    let ReviewObjectV1::CommentConflict(conflict) =
        store.read_object(comment.revision).await.unwrap()
    else {
        panic!("concurrent edits should select an explicit comment conflict")
    };
    assert_eq!(conflict.alternatives.len(), 2);

    let cold = forward.cold_copy().await;
    let cold_store = cold.review_store().await;
    let cold_selected = cold_store
        .lookup_thread(created.thread_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(cold_selected, selected);
    let ReviewObjectV1::ThreadMerge(cold_thread) =
        cold_store.read_object(cold_selected).await.unwrap()
    else {
        panic!("cold copy should retain the merged review thread")
    };
    let cold_conflict = cold_thread
        .comments
        .iter()
        .find(|comment| comment.comment_id == created.comment_id.unwrap())
        .unwrap()
        .revision;
    assert_eq!(cold_conflict, comment.revision);
    assert!(matches!(
        cold_store.read_object(cold_conflict).await.unwrap(),
        ReviewObjectV1::CommentConflict(_)
    ));

    let before = forward.coordinator.read().await.unwrap();
    let mut duplicate = unique_origins(
        first
            .origins()
            .await
            .into_iter()
            .chain(second.origins().await),
    );
    duplicate.reverse();
    let ApplyOriginsOutcomeV1::Applied(replayed) = forward.apply(200, duplicate).await else {
        panic!("duplicate origin lookup should complete")
    };
    assert_eq!(replayed.publication_position, None);
    assert_eq!(
        before.manifest().domains().unwrap().review,
        forward
            .coordinator
            .read()
            .await
            .unwrap()
            .manifest()
            .domains()
            .unwrap()
            .review
    );
}

#[tokio::test]
async fn explicit_comment_resolution_covers_the_exact_conflict_and_replicates() {
    let (source, created) = baseline().await;
    let first = branch_from(&source, 20).await;
    let second = branch_from(&source, 30).await;
    let initial_revision =
        CommentRevisionId::from_slice(created.comment_revision.unwrap().id.as_bytes()).unwrap();
    for (replica, seed, text) in [(&first, 41, "first edit"), (&second, 42, "second edit")] {
        publish(
            replica,
            seed,
            ReviewCommandV1::EditComment {
                comment_id: created.comment_id.unwrap(),
                expected_revision_id: initial_revision,
                replacement_body: body(text),
            },
        )
        .await;
    }

    let resolved_replica = merge_branches(&first, &second, 50, false).await;
    let peer = merge_branches(&first, &second, 60, true).await;
    let store = resolved_replica.review_store().await;
    let selected = store
        .lookup_thread(created.thread_id)
        .await
        .unwrap()
        .unwrap();
    let ReviewObjectV1::ThreadMerge(thread) = store.read_object(selected).await.unwrap() else {
        panic!("concurrent thread should select an explicit merge")
    };
    let conflict_ref = thread
        .comments
        .iter()
        .find(|selection| selection.comment_id == created.comment_id.unwrap())
        .unwrap()
        .revision;
    let ReviewObjectV1::CommentConflict(conflict) = store.read_object(conflict_ref).await.unwrap()
    else {
        panic!("concurrent edits should select an explicit comment conflict")
    };
    assert_eq!(conflict.alternatives.len(), 2);
    let expected_revision_ids = conflict
        .alternatives
        .iter()
        .map(|reference| CommentRevisionId::from_slice(reference.id.as_bytes()).unwrap())
        .collect::<Vec<_>>();

    let mut incomplete = expected_revision_ids.clone();
    incomplete[1] = CommentRevisionId::new([255; 32]);
    incomplete.sort();
    let error = resolved_replica
        .coordinator
        .publish_review_command(&request(
            70,
            ReviewCommandV1::ResolveCommentConflict {
                comment_id: created.comment_id.unwrap(),
                expected_revision_ids: incomplete,
                replacement_body: body("must not commit"),
            },
        ))
        .await
        .unwrap_err();
    assert!(matches!(error, ReviewCommandError::Stale));

    let resolved = publish(
        &resolved_replica,
        71,
        ReviewCommandV1::ResolveCommentConflict {
            comment_id: created.comment_id.unwrap(),
            expected_revision_ids,
            replacement_body: body("combined resolution"),
        },
    )
    .await;
    let resolved_ref = resolved.comment_revision.unwrap();
    let store = resolved_replica.review_store().await;
    let ReviewObjectV1::CommentRevision(revision) = store.read_object(resolved_ref).await.unwrap()
    else {
        panic!("resolution should select a comment revision")
    };
    assert_eq!(revision.previous_revision, None);
    assert_eq!(revision.superseded_revisions, conflict.alternatives);

    resolved_replica.transfer_to(&peer).await;
    peer.apply(72, resolved_replica.origins().await).await;
    assert_eq!(
        resolved_replica
            .coordinator
            .read()
            .await
            .unwrap()
            .manifest()
            .domains()
            .unwrap()
            .review
            .root,
        peer.coordinator
            .read()
            .await
            .unwrap()
            .manifest()
            .domains()
            .unwrap()
            .review
            .root
    );
}

#[tokio::test]
async fn unseen_reply_and_concurrent_reopen_keep_workflow_open() {
    let (source, created) = baseline().await;
    let reply_branch = branch_from(&source, 80).await;
    let resolve_branch = branch_from(&source, 90).await;
    publish(
        &reply_branch,
        101,
        ReviewCommandV1::AppendReply {
            thread_id: created.thread_id,
            expected_workflow_version: 1,
            parent_comment_id: created.comment_id,
            body: body("unseen reply"),
        },
    )
    .await;
    publish(
        &resolve_branch,
        102,
        ReviewCommandV1::ResolveThread {
            thread_id: created.thread_id,
            expected_thread_version: ThreadSnapshotId::from_slice(
                created.thread_snapshot.id.as_bytes(),
            )
            .unwrap(),
            reason: Some("looked complete".into()),
        },
    )
    .await;
    let merged = merge_branches(&reply_branch, &resolve_branch, 110, true).await;
    let store = merged.review_store().await;
    let selected = store
        .lookup_thread(created.thread_id)
        .await
        .unwrap()
        .unwrap();
    let ReviewObjectV1::ThreadMerge(thread) = store.read_object(selected).await.unwrap() else {
        panic!("workflow race should select an explicit merge")
    };
    assert_eq!(thread.status, ThreadStatusV1::Open);
    assert!(thread.selected_workflow_actions.is_empty());
    assert_eq!(thread.comments.len(), 2);
    let expected_workflow_version = thread.workflow_version;
    let appended = publish(
        &merged,
        111,
        ReviewCommandV1::AppendReply {
            thread_id: created.thread_id,
            expected_workflow_version,
            parent_comment_id: created.comment_id,
            body: body("post-merge reply"),
        },
    )
    .await;
    let resolved_after_merge = publish(
        &merged,
        112,
        ReviewCommandV1::ResolveThread {
            thread_id: created.thread_id,
            expected_thread_version: ThreadSnapshotId::from_slice(
                appended.thread_snapshot.id.as_bytes(),
            )
            .unwrap(),
            reason: Some("all merged activity reviewed".into()),
        },
    )
    .await;
    let store = merged.review_store().await;
    let ReviewObjectV1::ThreadMerge(resolved_thread) = store
        .read_object(resolved_after_merge.thread_snapshot)
        .await
        .unwrap()
    else {
        panic!("post-merge workflow should remain graph based")
    };
    assert_eq!(resolved_thread.status, ThreadStatusV1::Resolved);
    assert_eq!(resolved_thread.selected_workflow_actions.len(), 1);

    let reopen_branch = branch_from(&source, 120).await;
    let other_resolve = branch_from(&source, 130).await;
    let resolved = publish(
        &reopen_branch,
        141,
        ReviewCommandV1::ResolveThread {
            thread_id: created.thread_id,
            expected_thread_version: ThreadSnapshotId::from_slice(
                created.thread_snapshot.id.as_bytes(),
            )
            .unwrap(),
            reason: None,
        },
    )
    .await;
    publish(
        &reopen_branch,
        142,
        ReviewCommandV1::ReopenThread {
            thread_id: created.thread_id,
            expected_thread_version: ThreadSnapshotId::from_slice(
                resolved.thread_snapshot.id.as_bytes(),
            )
            .unwrap(),
            reason: Some("needs another look".into()),
        },
    )
    .await;
    publish(
        &other_resolve,
        143,
        ReviewCommandV1::ResolveThread {
            thread_id: created.thread_id,
            expected_thread_version: ThreadSnapshotId::from_slice(
                created.thread_snapshot.id.as_bytes(),
            )
            .unwrap(),
            reason: Some("independent resolve".into()),
        },
    )
    .await;
    let workflow_merge = merge_branches(&reopen_branch, &other_resolve, 150, false).await;
    let store = workflow_merge.review_store().await;
    let selected = store
        .lookup_thread(created.thread_id)
        .await
        .unwrap()
        .unwrap();
    let ReviewObjectV1::ThreadMerge(thread) = store.read_object(selected).await.unwrap() else {
        panic!("concurrent workflow actions should select an explicit merge")
    };
    assert_eq!(thread.status, ThreadStatusV1::Open);
    assert!(thread.selected_workflow_actions.is_empty());
}
