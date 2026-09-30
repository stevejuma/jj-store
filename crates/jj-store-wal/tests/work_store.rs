use jj_store_model::{EpochId, ObjectKey, PrincipalId, PublicationId, RepositoryId, RequestId};
use jj_store_object::memory::MemoryStore;
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication::{
        Coordinator, DomainRequest, Limits, ManifestSnapshot, ProjectId, PublicationGuard,
        PublishOutcome, Request, Snapshot, TaskId, TaskStateV1, WorkCommandRequest,
        WorkCommandResultV1, WorkCommandV1, WorkCommentId,
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
    async fn authorize_replication_seed(
        &self,
        _: &ManifestSnapshot,
        _: &wal::publication::SeedReplicationRequestV1,
    ) -> wal::Result<()> {
        Ok(())
    }
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

fn coordinator(store: Arc<MemoryStore>) -> Coordinator {
    Coordinator::new(
        store,
        REPOSITORY,
        Arc::new(Source(SourceScope::new(REPOSITORY))),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap()
}
async fn fixture() -> (Arc<MemoryStore>, Coordinator) {
    let store = Arc::new(MemoryStore::default());
    let coordinator = coordinator(store.clone());
    coordinator
        .initialize(EpochId::new([2; 16]), Vec::new())
        .await
        .unwrap();
    (store, coordinator)
}
fn request(byte: u8, command: WorkCommandV1) -> WorkCommandRequest {
    WorkCommandRequest::new(
        ACTOR,
        RequestId::new([byte; 16]),
        PublicationId::new([byte; 16]),
        command,
    )
    .unwrap()
}
async fn apply(c: &Coordinator, byte: u8, command: WorkCommandV1) -> WorkCommandResultV1 {
    let PublishOutcome::Committed(receipt) = c
        .publish_work_command(&request(byte, command))
        .await
        .unwrap()
    else {
        panic!("commit required")
    };
    serde_json::from_slice(&receipt.result).unwrap()
}
#[tokio::test]
async fn workstore_receipt_survives_checkpoint_and_reconstruction() {
    let (store, c) = fixture().await;
    let request = WorkCommandRequest::new(
        ACTOR,
        RequestId::new([10; 16]),
        PublicationId::new([10; 16]),
        WorkCommandV1::CreateProject {
            id: ProjectId::new([10; 16]),
            title: "Cross-repository project".into(),
            body: "Home metadata".into(),
        },
    )
    .unwrap();
    let before = c.read().await.unwrap();
    let outcome = c.publish_work_command(&request).await.unwrap();
    let PublishOutcome::Committed(receipt) = outcome else {
        panic!("commit required")
    };
    let after = c.read().await.unwrap();
    assert_eq!(after.manifest().format, 3);
    assert_eq!(after.manifest().heads, before.manifest().heads);
    assert_eq!(
        after.manifest().code_state_version,
        before.manifest().code_state_version
    );
    assert_eq!(after.manifest().agent_root, before.manifest().agent_root);
    assert_eq!(after.manifest().review_root, before.manifest().review_root);
    c.checkpoint().await.unwrap();
    c.recover(100).await.unwrap();
    let fresh = coordinator(store);
    assert_eq!(
        fresh.publish_work_command(&request).await.unwrap(),
        PublishOutcome::Committed(receipt)
    );
}

#[tokio::test]
async fn linked_tasks_comments_pins_and_native_copy() {
    let (_, c) = fixture().await;
    let project = ProjectId::new([10; 16]);
    let parent = TaskId::new([11; 16]);
    let child = TaskId::new([12; 16]);
    let a = RepositoryId::new([30; 16]);
    let b = RepositoryId::new([31; 16]);
    apply(
        &c,
        10,
        WorkCommandV1::CreateProject {
            id: project,
            title: "Release".into(),
            body: "Two repositories".into(),
        },
    )
    .await;
    apply(
        &c,
        11,
        WorkCommandV1::SetRepository {
            project,
            expected_version: 1,
            repository: a,
            active: true,
        },
    )
    .await;
    apply(
        &c,
        12,
        WorkCommandV1::SetRepository {
            project,
            expected_version: 2,
            repository: b,
            active: true,
        },
    )
    .await;
    apply(
        &c,
        13,
        WorkCommandV1::CreateTask {
            id: parent,
            project,
            expected_project_version: 3,
            title: "Parent".into(),
            body: "".into(),
            repository: Some(a),
            parent: None,
        },
    )
    .await;
    apply(
        &c,
        14,
        WorkCommandV1::CreateTask {
            id: child,
            project,
            expected_project_version: 3,
            title: "Child".into(),
            body: "".into(),
            repository: Some(b),
            parent: Some(parent),
        },
    )
    .await;
    let pinned_root = c.read().await.unwrap().manifest().work_root.unwrap();
    let pinned = c.work_store(pinned_root).await.unwrap();
    let before = c.read().await.unwrap().digest();
    let cycle = request(
        15,
        WorkCommandV1::UpdateTask {
            id: parent,
            expected_version: 1,
            expected_project_version: 3,
            title: "Parent".into(),
            body: "".into(),
            state: TaskStateV1::Open,
            repository: Some(a),
            parent: Some(child),
        },
    );
    assert!(matches!(
        c.publish_work_command(&cycle).await,
        Err(wal::publication::WorkCommandError::Model(
            jj_work::Error::Invalid("task cycle")
        ))
    ));
    assert_eq!(before, c.read().await.unwrap().digest());
    apply(
        &c,
        16,
        WorkCommandV1::SetRepository {
            project,
            expected_version: 3,
            repository: b,
            active: false,
        },
    )
    .await;
    let retired = request(
        17,
        WorkCommandV1::CreateTask {
            id: TaskId::new([13; 16]),
            project,
            expected_project_version: 4,
            title: "Retired target".into(),
            body: "".into(),
            repository: Some(b),
            parent: None,
        },
    );
    assert!(c.publish_work_command(&retired).await.is_err());
    apply(
        &c,
        18,
        WorkCommandV1::UpdateTask {
            id: child,
            expected_version: 1,
            expected_project_version: 4,
            title: "Child".into(),
            body: "Historical target retained".into(),
            state: TaskStateV1::InProgress,
            repository: Some(b),
            parent: Some(parent),
        },
    )
    .await;
    let first = WorkCommentId::new([40; 16]);
    let reply = WorkCommentId::new([41; 16]);
    apply(
        &c,
        19,
        WorkCommandV1::AddComment {
            id: first,
            task: child,
            expected_task_version: 2,
            reply_to: None,
            body: "Original body".into(),
        },
    )
    .await;
    let comment_root = c.read().await.unwrap().manifest().work_root.unwrap();
    apply(
        &c,
        20,
        WorkCommandV1::AddComment {
            id: reply,
            task: child,
            expected_task_version: 3,
            reply_to: Some(first),
            body: "Threaded reply".into(),
        },
    )
    .await;
    apply(
        &c,
        21,
        WorkCommandV1::EditComment {
            id: first,
            expected_version: 1,
            expected_task_version: 4,
            body: "Edited body".into(),
        },
    )
    .await;
    let stale = request(
        22,
        WorkCommandV1::EditComment {
            id: first,
            expected_version: 1,
            expected_task_version: 5,
            body: "Stale".into(),
        },
    );
    assert!(matches!(
        c.publish_work_command(&stale).await,
        Err(wal::publication::WorkCommandError::Model(
            jj_work::Error::Stale
        ))
    ));
    apply(
        &c,
        23,
        WorkCommandV1::TombstoneComment {
            id: first,
            expected_version: 2,
            expected_task_version: 5,
        },
    )
    .await;
    assert_eq!(pinned.state().tasks[&child].state, TaskStateV1::Open);
    assert!(pinned.state().comments.is_empty());
    assert_eq!(
        c.work_store(comment_root).await.unwrap().state().comments[&first]
            .body
            .as_deref(),
        Some("Original body")
    );
    c.checkpoint().await.unwrap();
    let dest = Arc::new(MemoryStore::default());
    let copied = c
        .copy_native_to(
            dest.clone(),
            Arc::new(Source(SourceScope::new(REPOSITORY))),
            100,
        )
        .await
        .unwrap();
    assert!(copied.control_records > 20);
    let cold = coordinator(dest);
    let recovered = cold.recover(100).await.unwrap();
    let state = cold
        .work_store(recovered.manifest().work_root.unwrap())
        .await
        .unwrap();
    assert_eq!(state.state().comments[&first].body, None);
    assert_eq!(state.state().comments[&reply].reply_to, Some(first));
    assert_eq!(state.state().tasks[&child].repository, Some(b));
    assert_eq!(
        cold.work_store(comment_root)
            .await
            .unwrap()
            .state()
            .comments[&first]
            .body
            .as_deref(),
        Some("Original body")
    );
}

#[tokio::test]
async fn workstore_and_origin_replication_are_mutually_excluded() {
    use jj_store_model::{ReplicaId, WriterEpoch};
    use wal::publication::SeedReplicationRequestV1;
    let (_, c) = fixture().await;
    let seed = SeedReplicationRequestV1 {
        schema_version: 1,
        principal: ACTOR,
        request_id: RequestId::new([60; 16]),
        publication_id: PublicationId::new([60; 16]),
        replica: ReplicaId::new([61; 16]),
        writer_epoch: WriterEpoch::new([62; 16]),
    };
    apply(
        &c,
        10,
        WorkCommandV1::CreateProject {
            id: ProjectId::new([10; 16]),
            title: "Home".into(),
            body: "".into(),
        },
    )
    .await;
    let before = c.read().await.unwrap().digest();
    assert!(c.seed_replication(&seed).await.is_err());
    assert_eq!(c.read().await.unwrap().digest(), before);
    let (_, replicated) = fixture().await;
    replicated.seed_replication(&seed).await.unwrap();
    let before = replicated.read().await.unwrap().digest();
    let r = request(
        11,
        WorkCommandV1::CreateProject {
            id: ProjectId::new([10; 16]),
            title: "Home".into(),
            body: "".into(),
        },
    );
    assert!(matches!(
        replicated.publish_work_command(&r).await,
        Err(wal::publication::WorkCommandError::ReplicationUnsupported)
    ));
    assert_eq!(replicated.read().await.unwrap().digest(), before);
}

#[tokio::test]
async fn native_publication_preserves_work_and_increments_code_version() {
    use jj_store_model::OperationId;
    use wal::publication::{SemanticKind, Transition};
    let (_, c) = fixture().await;
    apply(
        &c,
        10,
        WorkCommandV1::CreateProject {
            id: ProjectId::new([10; 16]),
            title: "Home".into(),
            body: "".into(),
        },
    )
    .await;
    let before = c.read().await.unwrap();
    let request = Request {
        principal: ACTOR,
        request_id: RequestId::new([70; 16]),
        publication_id: PublicationId::new([70; 16]),
        transition: Some(Transition::Strict {
            expected: before.manifest().snapshot(),
            heads: BTreeSet::from([OperationId::new([71; 64])]),
        }),
        settings: None,
        result: Vec::new(),
        kind: SemanticKind::OperationPublished,
    };
    c.publish(&request).await.unwrap();
    let after = c.recover(100).await.unwrap();
    assert_eq!(after.manifest().work_root, before.manifest().work_root);
    assert_eq!(
        after.manifest().work_state_version,
        before.manifest().work_state_version
    );
    assert_eq!(
        after.manifest().code_state_version,
        before.manifest().code_state_version + 1
    );
}

struct ManifestFaultStore {
    inner: MemoryStore,
    armed: std::sync::atomic::AtomicBool,
}
#[async_trait::async_trait]
impl jj_store_object::ObjectStore for ManifestFaultStore {
    async fn head(
        &self,
        key: &jj_store_object::Key,
    ) -> jj_store_object::Result<jj_store_object::Metadata> {
        jj_store_object::ObjectStore::head(&self.inner, key).await
    }
    async fn get(
        &self,
        key: &jj_store_object::Key,
        range: Option<jj_store_object::ByteRange>,
    ) -> jj_store_object::Result<jj_store_object::Download> {
        jj_store_object::ObjectStore::get(&self.inner, key, range).await
    }
    async fn put(
        &self,
        key: &jj_store_object::Key,
        condition: jj_store_object::Condition,
        upload: &jj_store_object::StagedUpload,
    ) -> jj_store_object::Result<jj_store_object::WriteOutcome> {
        if key.as_str().ends_with("/manifest.json")
            && self.armed.swap(false, std::sync::atomic::Ordering::SeqCst)
        {
            return Ok(jj_store_object::WriteOutcome::Unknown);
        }
        jj_store_object::ObjectStore::put(&self.inner, key, condition, upload).await
    }
}
#[tokio::test]
async fn uncertain_manifest_keeps_staged_work_invisible_and_retry_identity_stable() {
    let store = Arc::new(ManifestFaultStore {
        inner: MemoryStore::default(),
        armed: false.into(),
    });
    let c = Coordinator::new(
        store.clone(),
        REPOSITORY,
        Arc::new(Source(SourceScope::new(REPOSITORY))),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap();
    c.initialize(EpochId::new([2; 16]), Vec::new())
        .await
        .unwrap();
    let before = c.read().await.unwrap().digest();
    let request = request(
        80,
        WorkCommandV1::CreateProject {
            id: ProjectId::new([10; 16]),
            title: "Home".into(),
            body: "".into(),
        },
    );
    store.armed.store(true, std::sync::atomic::Ordering::SeqCst);
    assert!(
        matches!(c.publish_work_command(&request).await.unwrap(), PublishOutcome::OutcomeUnknown { request_id, publication_id } if request_id == request.request_id && publication_id == request.publication_id)
    );
    assert_eq!(c.read().await.unwrap().digest(), before);
    assert!(c.read().await.unwrap().manifest().work_root.is_none());
    let accepted = c.publish_work_command(&request).await.unwrap();
    let fresh = Coordinator::new(
        store,
        REPOSITORY,
        Arc::new(Source(SourceScope::new(REPOSITORY))),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap();
    fresh.checkpoint().await.unwrap();
    assert_eq!(
        fresh.publish_work_command(&request).await.unwrap(),
        accepted
    );
}

#[tokio::test]
async fn work_catalog_splits_keep_prior_root_pinned() {
    let (_, c) = fixture().await;
    apply(
        &c,
        1,
        WorkCommandV1::CreateProject {
            id: ProjectId::new([1; 16]),
            title: "First".into(),
            body: "".into(),
        },
    )
    .await;
    let root = c.read().await.unwrap().manifest().work_root.unwrap();
    for byte in 2..=40 {
        apply(
            &c,
            byte,
            WorkCommandV1::CreateProject {
                id: ProjectId::new([byte; 16]),
                title: format!("Project {byte}"),
                body: "".into(),
            },
        )
        .await;
    }
    let current = c.read().await.unwrap();
    assert_eq!(
        c.work_store(current.manifest().work_root.unwrap())
            .await
            .unwrap()
            .state()
            .projects
            .len(),
        40
    );
    assert_eq!(c.work_store(root).await.unwrap().state().projects.len(), 1);
    c.recover(100).await.unwrap();
}
