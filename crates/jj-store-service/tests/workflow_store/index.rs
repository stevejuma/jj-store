use super::*;
use jj_store_object::{
    ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
};
use std::sync::atomic::{AtomicUsize, Ordering};
#[derive(Default)]
struct Counting {
    inner: MemoryStore,
    workflow_reads: AtomicUsize,
}
#[tokio::test]
async fn persistent_generations_share_requests_but_require_cold_source_admission() {
    let dir = tempfile::tempdir().unwrap();
    let indexes = Arc::new(jj_store_service::read_indexes::Indexes::persistent(
        dir.path().join("indexes"),
    ));
    let store = Arc::new(Counting::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 2 * 1024 * 1024).unwrap());
    create_repository_local_trusted(
        store.clone(),
        objects,
        Permission::Admin,
        m::EpochId::new([3; 16]),
        Default::default(),
    )
    .await
    .unwrap();
    let writer = session(store.clone(), ACTOR, Permission::Write);
    work::WorkCommandService
        .apply(
            writer.clone(),
            &work::Request {
                version: 1,
                request_id: m::RequestId::new([90; 16]),
                command: work::WorkCommandV1::CreateProject {
                    id: work::ProjectId::new([90; 16]),
                    title: "Indexes".into(),
                    body: String::new(),
                },
            },
        )
        .await
        .unwrap();
    let service = w::WorkflowService {
        hosts: Default::default(),
    };
    service
        .apply(
            writer,
            &request(
                91,
                w::Command::Put {
                    workflow: w::WorkflowId::new([91; 16]),
                    expected: None,
                    definition: definition(),
                },
            ),
        )
        .await
        .unwrap();
    let bound = |indexes: Arc<jj_store_service::read_indexes::Indexes>| {
        Arc::new(
            NativeSession::new_local_trusted(
                store.clone(),
                Arc::new(NativeObjects::new(store.clone(), REPO, 2 * 1024 * 1024).unwrap()),
                ACTOR,
                Permission::Read,
                Arc::new(SystemClock),
                Protection::default(),
                SessionLimits::default(),
            )
            .unwrap()
            .with_read_indexes(indexes),
        )
    };
    let first = service
        .page(bound(indexes.clone()), w::Query::default())
        .await
        .unwrap();
    assert_eq!(indexes.stats().built, 3);
    store.workflow_reads.store(0, Ordering::Relaxed);
    let shared = service
        .page(bound(indexes.clone()), w::Query::default())
        .await
        .unwrap();
    assert_eq!(shared.workflows, first.workflows);
    assert_eq!(store.workflow_reads.load(Ordering::Relaxed), 0);
    drop(indexes);
    let concurrent = Arc::new(jj_store_service::read_indexes::Indexes::persistent(
        dir.path().join("concurrent"),
    ));
    store.workflow_reads.store(0, Ordering::Relaxed);
    let (a, b, c, d) = tokio::join!(
        service.page(bound(concurrent.clone()), w::Query::default()),
        service.page(bound(concurrent.clone()), w::Query::default()),
        service.page(bound(concurrent.clone()), w::Query::default()),
        service.page(bound(concurrent.clone()), w::Query::default())
    );
    for page in [a, b, c, d] {
        assert_eq!(page.unwrap().workflows, first.workflows);
    }
    assert_eq!(
        concurrent.stats().built,
        3,
        "concurrent readers coalesce one complete construction"
    );
    assert_eq!(store.workflow_reads.load(Ordering::Relaxed), 2);
    drop(concurrent);
    store.workflow_reads.store(0, Ordering::Relaxed);
    let cold = Arc::new(jj_store_service::read_indexes::Indexes::persistent(
        dir.path().join("indexes"),
    ));
    let reopened = service
        .page(bound(cold.clone()), w::Query::default())
        .await
        .unwrap();
    assert_eq!(reopened.workflows, first.workflows);
    assert!(
        store.workflow_reads.load(Ordering::Relaxed) > 0,
        "disk files cannot skip cold source validation"
    );
    assert_eq!(cold.stats().restored, 3);
    let tight = NativeSession::new_local_trusted(
        store.clone(),
        Arc::new(NativeObjects::new(store.clone(), REPO, 2 * 1024 * 1024).unwrap()),
        ACTOR,
        Permission::Read,
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits {
            inventory_nodes: 1,
            ..Default::default()
        },
    )
    .unwrap()
    .with_read_indexes(cold.clone());
    assert!(
        matches!(
            service.page(Arc::new(tight), w::Query::default()).await,
            Err(w::Error::Limit)
        ),
        "warm data cannot borrow another reader's source budget"
    );
    let foreign = Arc::new(MissingWorkflow {
        inner: store.clone(),
    });
    let foreign_reader = NativeSession::new_local_trusted(
        foreign.clone(),
        Arc::new(NativeObjects::new(foreign, REPO, 2 * 1024 * 1024).unwrap()),
        ACTOR,
        Permission::Read,
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
    )
    .unwrap()
    .with_read_indexes(cold.clone());
    assert!(
        service
            .page(Arc::new(foreign_reader), w::Query::default())
            .await
            .is_err(),
        "same repo/epoch/root cannot reuse a different source's admission"
    );
}
struct MissingWorkflow {
    inner: Arc<Counting>,
}
#[async_trait::async_trait]
impl ObjectStore for MissingWorkflow {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.inner.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        if key.as_str().contains("/workflow-objects/") {
            return Err(jj_store_object::Error::NotFound);
        }
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        self.inner.put(key, condition, upload).await
    }
}
#[async_trait::async_trait]
impl ObjectStore for Counting {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.inner.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        if key.as_str().contains("/workflow-objects/") {
            self.workflow_reads.fetch_add(1, Ordering::Relaxed);
        }
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        self.inner.put(key, condition, upload).await
    }
}
#[tokio::test]
async fn unchanged_workflow_roots_do_not_reconstruct_objects() {
    let store = Arc::new(Counting::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 2 * 1024 * 1024).unwrap());
    create_repository_local_trusted(
        store.clone(),
        objects,
        Permission::Admin,
        m::EpochId::new([3; 16]),
        Default::default(),
    )
    .await
    .unwrap();
    let writer = session(store.clone(), ACTOR, Permission::Write);
    work::WorkCommandService
        .apply(
            writer.clone(),
            &work::Request {
                version: 1,
                request_id: m::RequestId::new([90; 16]),
                command: work::WorkCommandV1::CreateProject {
                    id: work::ProjectId::new([90; 16]),
                    title: "Indexes".into(),
                    body: String::new(),
                },
            },
        )
        .await
        .unwrap();
    let service = w::WorkflowService {
        hosts: Default::default(),
    };
    service
        .apply(
            writer,
            &request(
                91,
                w::Command::Put {
                    workflow: w::WorkflowId::new([91; 16]),
                    expected: None,
                    definition: definition(),
                },
            ),
        )
        .await
        .unwrap();
    let reader = session(store.clone(), ACTOR, Permission::Read);
    let first = service
        .page(reader.clone(), w::Query::default())
        .await
        .unwrap();
    assert!(!first.workflows.is_empty());
    assert!(store.workflow_reads.swap(0, Ordering::Relaxed) > 0);
    let second = service.page(reader, w::Query::default()).await.unwrap();
    assert_eq!(second.root, first.root);
    assert_eq!(second.workflows, first.workflows);
    assert_eq!(
        store.workflow_reads.load(Ordering::Relaxed),
        0,
        "unchanged selected root should reuse its validated data"
    );
}
