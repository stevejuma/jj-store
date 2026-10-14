use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_server::storage::{EmbeddedStorageConfig, EmbeddedStorageRuntime};
use jj_store_service::{
    auth::{Permission, SystemClock},
    native::{NativeSession, SessionLimits, create_repository_local_trusted},
    read_indexes::Indexes,
    work, workflow as w,
};
use std::{sync::Arc, time::Duration};
const REPO: m::RepositoryId = m::RepositoryId::new([71; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([72; 16]);
fn session(runtime: &EmbeddedStorageRuntime, indexes: Arc<Indexes>) -> Arc<NativeSession> {
    let store = runtime.store();
    Arc::new(
        NativeSession::new_local_trusted(
            store.clone(),
            Arc::new(NativeObjects::new(store, REPO, 2 * 1024 * 1024).unwrap()),
            ACTOR,
            Permission::Admin,
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap()
        .with_read_indexes(indexes),
    )
}
fn request(id: u8, command: w::Command) -> w::Request {
    w::Request {
        version: 1,
        request_id: m::RequestId::new([id; 16]),
        command,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires the separately built pinned embedded RustFS runtime"]
async fn custom_indexes_preserve_pinned_history_after_real_storage_restart_and_cache_loss() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("volume")).unwrap();
    let address = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .to_string();
    let config = EmbeddedStorageConfig {
        runtime_executable: std::fs::canonicalize(
            std::env::var_os("JJ_STORE_RUSTFS_RUNTIME").expect("set the pinned runtime executable"),
        )
        .unwrap(),
        volume: dir.path().join("volume"),
        address,
        access_key: "custom-index-qualification-access".into(),
        secret_key: "custom-index-qualification-secret".into(),
        region: "us-east-1".into(),
        bucket: "custom-index-qualification".into(),
        max_object_bytes: 2 * 1024 * 1024,
        timeout: Duration::from_secs(30),
    };
    let first = EmbeddedStorageRuntime::start(config.clone()).await.unwrap();
    first.store().ensure_bucket().await.unwrap();
    create_repository_local_trusted(
        first.store(),
        Arc::new(NativeObjects::new(first.store(), REPO, 2 * 1024 * 1024).unwrap()),
        Permission::Admin,
        m::EpochId::new([73; 16]),
        Default::default(),
    )
    .await
    .unwrap();
    let indexes = Arc::new(Indexes::persistent(dir.path().join("indexes")));
    let writer = session(&first, indexes.clone());
    work::WorkCommandService
        .apply(
            writer.clone(),
            &work::Request {
                version: 1,
                request_id: m::RequestId::new([74; 16]),
                command: work::WorkCommandV1::CreateProject {
                    id: work::ProjectId::new([74; 16]),
                    title: "Index qualification".into(),
                    body: "Recorded native project".into(),
                },
            },
        )
        .await
        .unwrap();
    let service = w::WorkflowService {
        hosts: [ACTOR].into(),
    };
    let workflow = w::WorkflowId::new([75; 16]);
    service
        .apply(
            writer.clone(),
            &request(
                75,
                w::Command::Put {
                    workflow,
                    expected: None,
                    definition: w::Definition {
                        name: "Recorded workflow".into(),
                        description: "First definition".into(),
                        enabled: true,
                        repository: REPO,
                        trigger: w::Trigger::Manual,
                        labels: Default::default(),
                        conditions: Default::default(),
                        actions: vec![w::Action {
                            id: "step".into(),
                            executor: w::Executor::Command {
                                name: "qualification".into(),
                            },
                            inputs: Default::default(),
                            timeout_seconds: 30,
                        }],
                    },
                },
            ),
        )
        .await
        .unwrap();
    let original = service
        .page(writer.clone(), w::Query::default())
        .await
        .unwrap();
    let mut changed = original.workflows[0].definition.clone();
    changed.description = "Second definition".into();
    service
        .apply(
            writer.clone(),
            &request(
                76,
                w::Command::Put {
                    workflow,
                    expected: Some(original.workflows[0].version),
                    definition: changed,
                },
            ),
        )
        .await
        .unwrap();
    let latest = service
        .page(writer.clone(), w::Query::default())
        .await
        .unwrap();
    assert_ne!(latest.root, original.root);
    assert_eq!(indexes.stats().built, 6);
    let projects = work::WorkPager::new(Arc::new(SystemClock))
        .browse(writer.clone(), work::WorkBrowseQuery::default())
        .await
        .unwrap();
    assert_eq!(projects.projects[0].title, "Index qualification");
    drop(writer);
    drop(indexes);
    first.shutdown().await;

    let second = EmbeddedStorageRuntime::start(config).await.unwrap();
    let indexes = Arc::new(Indexes::persistent(dir.path().join("indexes")));
    let reader = session(&second, indexes.clone());
    let reopened = service
        .page(reader.clone(), w::Query::default())
        .await
        .unwrap();
    assert_eq!(reopened.workflows, latest.workflows);
    assert_eq!(indexes.stats().restored, 3);
    let historical = service
        .page(
            reader.clone(),
            w::Query {
                at: Some(original.position),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(historical.root, original.root);
    assert_eq!(historical.workflows, original.workflows);
    assert_eq!(indexes.stats().restored, 6);
    let projects_after = work::WorkPager::new(Arc::new(SystemClock))
        .browse(reader.clone(), work::WorkBrowseQuery::default())
        .await
        .unwrap();
    assert_eq!(projects_after.projects[0].title, projects.projects[0].title);
    drop(reader);
    drop(indexes);
    for file in std::fs::read_dir(dir.path().join("indexes")).unwrap() {
        std::fs::write(file.unwrap().path(), b"damaged derived generation").unwrap();
    }
    let indexes = Arc::new(Indexes::persistent(dir.path().join("indexes")));
    let after_damage = service
        .page(session(&second, indexes.clone()), w::Query::default())
        .await
        .unwrap();
    assert_eq!(after_damage.workflows, latest.workflows);
    assert_eq!(indexes.stats().built, 3);
    drop(indexes);
    std::fs::remove_dir_all(dir.path().join("indexes")).unwrap();
    let indexes = Arc::new(Indexes::persistent(dir.path().join("indexes")));
    assert_eq!(
        service
            .page(session(&second, indexes.clone()), w::Query::default())
            .await
            .unwrap()
            .workflows,
        latest.workflows
    );
    assert_eq!(indexes.stats().built, 3);
    drop(indexes);
    second.shutdown().await;
}
