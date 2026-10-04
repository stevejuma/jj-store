//! Disposable real RustFS qualification; never uses an existing repository.
use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::ObjectStore;
use jj_store_server::{
    config::Config, installation::initialize_local, storage::AuthoritativeStorage,
};
use jj_store_service::{
    auth::{Permission, SystemClock},
    native::{NativeSession, SessionLimits, create_repository_local_trusted},
    work::*,
};
use jj_store_wal::{
    self as wal,
    publication::{Coordinator, Limits, ManifestSnapshot, PublicationGuard},
};
use std::sync::Arc;
const HOME: m::RepositoryId = m::RepositoryId::new([1; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([2; 16]);
struct ReadOnly;
#[async_trait::async_trait]
impl PublicationGuard for ReadOnly {
    async fn authorize(
        &self,
        _: &ManifestSnapshot,
        _: &wal::publication::Request,
    ) -> wal::Result<()> {
        Err(wal::Error::Denied)
    }
    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &wal::publication::Request,
        _: &wal::publication::Snapshot,
    ) -> wal::Result<()> {
        Err(wal::Error::Denied)
    }
}
fn coordinator(store: Arc<dyn ObjectStore>) -> Coordinator {
    Coordinator::new(
        store.clone(),
        HOME,
        Arc::new(NativeObjects::new(store, HOME, 4 * 1024 * 1024).unwrap()),
        Arc::new(ReadOnly),
        Limits::default(),
    )
    .unwrap()
}
fn session(store: Arc<dyn ObjectStore>) -> Arc<NativeSession> {
    Arc::new(
        NativeSession::new_local_trusted(
            store.clone(),
            Arc::new(NativeObjects::new(store, HOME, 4 * 1024 * 1024).unwrap()),
            ACTOR,
            Permission::Admin,
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    )
}
fn request(seed: u8, command: WorkCommandV1) -> Request {
    Request {
        version: 1,
        request_id: m::RequestId::new([seed; 16]),
        command,
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires an explicit separately built pinned RustFS runtime"]
async fn workstore_real_rustfs_restart_checkpoint_copy_and_historical_recovery() {
    let runtime = std::fs::canonicalize(
        std::env::var_os("JJ_STORE_RUSTFS_RUNTIME").expect("explicit pinned runtime required"),
    )
    .unwrap();
    let root = tempfile::tempdir().unwrap();
    let source_config = Config::load(
        &initialize_local(&root.path().join("source"), &runtime)
            .unwrap()
            .config,
    )
    .unwrap();
    let destination_config = Config::load(
        &initialize_local(&root.path().join("destination"), &runtime)
            .unwrap()
            .config,
    )
    .unwrap();
    let source = AuthoritativeStorage::start(&source_config).await.unwrap();
    let store = source.store();
    for repo in [
        HOME,
        m::RepositoryId::new([7; 16]),
        m::RepositoryId::new([8; 16]),
    ] {
        create_repository_local_trusted(
            store.clone(),
            Arc::new(NativeObjects::new(store.clone(), repo, 4 * 1024 * 1024).unwrap()),
            Permission::Admin,
            m::EpochId::new([3; 16]),
            Limits::default(),
        )
        .await
        .unwrap();
    }
    let writer = session(store.clone());
    let project = ProjectId::new([10; 16]);
    let task = TaskId::new([11; 16]);
    let comment = WorkCommentId::new([12; 16]);
    let create = request(
        10,
        WorkCommandV1::CreateProject {
            id: project,
            title: "RustFS home".into(),
            body: "Native authority".into(),
        },
    );
    let accepted = WorkCommandService
        .apply(writer.clone(), &create)
        .await
        .unwrap();
    for (version, repository) in [
        (1, m::RepositoryId::new([7; 16])),
        (2, m::RepositoryId::new([8; 16])),
    ] {
        WorkCommandService
            .apply(
                writer.clone(),
                &request(
                    20 + version as u8,
                    WorkCommandV1::SetRepository {
                        project,
                        expected_version: version,
                        repository,
                        active: true,
                    },
                ),
            )
            .await
            .unwrap();
    }
    WorkCommandService
        .apply(
            writer.clone(),
            &request(
                11,
                WorkCommandV1::CreateTask {
                    id: task,
                    project,
                    expected_project_version: 3,
                    title: "Linked task".into(),
                    body: "Task".into(),
                    repository: Some(m::RepositoryId::new([8; 16])),
                    parent: None,
                },
            ),
        )
        .await
        .unwrap();
    WorkCommandService
        .apply(
            writer.clone(),
            &request(
                12,
                WorkCommandV1::AddComment {
                    id: comment,
                    task,
                    expected_task_version: 1,
                    reply_to: None,
                    body: "Historical body".into(),
                },
            ),
        )
        .await
        .unwrap();
    let current = coordinator(store.clone());
    let historical = current.read().await.unwrap().manifest().work_root.unwrap();
    WorkCommandService
        .apply(
            writer.clone(),
            &request(
                13,
                WorkCommandV1::TombstoneComment {
                    id: comment,
                    expected_version: 1,
                    expected_task_version: 2,
                },
            ),
        )
        .await
        .unwrap();
    current.checkpoint().await.unwrap();
    let selected = current.recover(100).await.unwrap();
    let origin = selected.manifest().replication_root;
    let destination = AuthoritativeStorage::start(&destination_config)
        .await
        .unwrap();
    let destination_store = destination.store();
    current
        .copy_native_to(
            destination_store.clone(),
            Arc::new(NativeObjects::new(destination_store.clone(), HOME, 4 * 1024 * 1024).unwrap()),
            100,
        )
        .await
        .unwrap();
    drop(current);
    drop(writer);
    drop(store);
    drop(destination_store);
    destination.shutdown().await.unwrap();
    source.shutdown().await.unwrap();
    for config in [&source_config, &destination_config] {
        let cold = AuthoritativeStorage::start(config).await.unwrap();
        let store = cold.store();
        let restored = coordinator(store.clone());
        let snapshot = restored.recover(100).await.unwrap();
        assert_eq!(snapshot.manifest().replication_root, origin);
        assert_eq!(snapshot.digest(), selected.digest());
        assert_eq!(
            restored
                .work_store(historical)
                .await
                .unwrap()
                .state()
                .comments[&comment]
                .body
                .as_deref(),
            Some("Historical body")
        );
        let reader = session(store.clone());
        let page = WorkPager::new(Arc::new(SystemClock))
            .page(
                reader.clone(),
                WorkQuery {
                    task: Some(task),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(page.comments[0].body, None);
        assert_eq!(
            page.tasks[0].repository,
            Some(m::RepositoryId::new([8; 16]))
        );
        assert!(reader.replica_descriptor().await.is_ok());
        let Outcome::Committed { receipt } = &accepted else {
            panic!("commit required")
        };
        assert_eq!(
            WorkCommandService
                .status(reader, create.request_id)
                .await
                .unwrap(),
            Status::Committed {
                receipt: receipt.clone()
            }
        );
        drop(restored);
        drop(store);
        cold.shutdown().await.unwrap();
    }
}
