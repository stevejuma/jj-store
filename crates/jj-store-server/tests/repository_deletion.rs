use jj_store_model::{EpochId, RepositoryId, RequestId};
use jj_store_object::{
    Condition, Key, ObjectStore as _, StagedUpload,
    s3::{self, S3Store},
};
use jj_store_server::{
    installation::{Identity, initialize_registry},
    registry::Registry,
    repository_deletion::Manager,
    storage::{EmbeddedStorageConfig, EmbeddedStorageRuntime},
};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    auth::{Permission, Scope, SystemClock, TokenPolicy},
    native::SessionLimits,
    registry::RepositoryRegistry as _,
    repository_deletion::*,
};
use std::{sync::Arc, time::Duration};
struct Operators;
#[async_trait::async_trait]
impl OperatorPolicies for Operators {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        Ok(TokenPolicy::new(Scope::Operator))
    }
}
fn make_host(store: Arc<S3Store>) -> RepositoryHost {
    RepositoryHost::new(
        store,
        Arc::new(Operators),
        Arc::new(SystemClock),
        Default::default(),
        SessionLimits::default(),
        1024 * 1024,
    )
    .unwrap()
}
fn client(config: &EmbeddedStorageConfig) -> Arc<S3Store> {
    Arc::new(
        S3Store::new(s3::Config {
            endpoint: format!("http://{}", config.address),
            bucket: config.bucket.clone(),
            region: config.region.clone(),
            credentials: s3::Credentials {
                access_key: config.access_key.clone(),
                secret_key: config.secret_key.clone(),
                session_token: None,
            },
            allow_http_loopback: true,
            max_object_bytes: config.max_object_bytes,
            timeout: config.timeout,
        })
        .unwrap(),
    )
}
#[tokio::test]
#[ignore = "requires the separately built pinned embedded RustFS runtime"]
async fn permanent_deletion_fences_restarts_erases_objects_and_preserves_other_repositories() {
    let root = tempfile::tempdir().unwrap();
    let initialized = initialize_registry(&root.path().join("state")).unwrap();
    let identity = Identity::load(&initialized.installation_file).unwrap();
    let registry = Arc::new(Registry::new(initialized.catalog, identity.clone()).unwrap());
    std::fs::create_dir(root.path().join("volume")).unwrap();
    let config = EmbeddedStorageConfig {
        runtime_executable: std::fs::canonicalize(
            std::env::var_os("JJ_STORE_RUSTFS_RUNTIME").expect("set JJ_STORE_RUSTFS_RUNTIME"),
        )
        .unwrap(),
        volume: root.path().join("volume"),
        address: format!(
            "127.0.0.1:{}",
            std::net::TcpListener::bind("127.0.0.1:0")
                .unwrap()
                .local_addr()
                .unwrap()
                .port()
        ),
        access_key: "repository-deletion-isolated".into(),
        secret_key: "repository-deletion-isolated-qualification".into(),
        region: "us-east-1".into(),
        bucket: "repository-deletion-test".into(),
        max_object_bytes: 1024 * 1024,
        timeout: Duration::from_secs(30),
    };
    let runtime = EmbeddedStorageRuntime::start(config.clone()).await.unwrap();
    let store = runtime.store();
    store.ensure_bucket().await.unwrap();
    let host = make_host(store.clone());
    let repository = RepositoryId::new([0xaa; 16]);
    let other = RepositoryId::new([0xab; 16]);
    let epoch = EpochId::new([0xcc; 16]);
    for (id, name) in [(repository, "deleted"), (other, "preserved")] {
        host.create_local_trusted(id, epoch, Permission::Admin)
            .await
            .unwrap();
        registry.register(id, name.into(), None).await.unwrap();
    }
    let original_session = host
        .endpoint(repository)
        .unwrap()
        .bind_local_trusted(identity.principal(b"test"), Permission::Admin)
        .unwrap()
        .into_session();
    let orphan = Key::new(format!("repos/{repository}/orphan/never-published")).unwrap();
    let upload = StagedUpload::from_bytes(b"unpublished data", 1024)
        .await
        .unwrap();
    for i in 0..40 {
        store
            .put(
                &Key::new(format!("repos/{repository}/orphan/{i}")).unwrap(),
                Condition::Absent,
                &upload,
            )
            .await
            .unwrap();
    }
    store
        .put(&orphan, Condition::Absent, &upload)
        .await
        .unwrap();
    let preserved = Key::new(format!("repos/{other}/keep")).unwrap();
    store
        .put(&preserved, Condition::Absent, &upload)
        .await
        .unwrap();
    let manager = Manager::new(
        &initialized.state_dir,
        identity.clone(),
        store.clone(),
        host.clone(),
        registry.clone(),
    );
    let request = DeleteRepository {
        version: 1,
        request_id: RequestId::new([0xdd; 16]),
        expected_epoch: epoch,
        expected_installation: identity.installation.clone(),
    };
    let mut stale = request.clone();
    stale.expected_epoch = EpochId::new([0xee; 16]);
    assert!(matches!(
        manager.delete(repository, stale).await,
        Err(Error::Stale)
    ));
    original_session.snapshot().await.unwrap();
    let first = manager.delete(repository, request.clone()).await.unwrap();
    assert_eq!(first.state, DeletionState::Pending);
    assert!(original_session.snapshot().await.is_err());
    assert!(
        store
            .put(&orphan, Condition::Absent, &upload)
            .await
            .is_err()
    );
    assert_eq!(registry.snapshot(None).await.unwrap().repositories.len(), 1);
    // A cold owned storage restart ends all old handlers before recovery.
    runtime.shutdown().await;
    let runtime = EmbeddedStorageRuntime::start(config.clone()).await.unwrap();
    let restarted_store = runtime.store();
    Manager::restore(
        &initialized.state_dir,
        &identity,
        &restarted_store,
        &registry,
    )
    .await
    .unwrap();
    assert!(restarted_store.head(&orphan).await.is_err());
    let resumed = Manager::new(
        &initialized.state_dir,
        identity.clone(),
        restarted_store.clone(),
        make_host(restarted_store.clone()),
        registry.clone(),
    );
    assert_eq!(
        resumed
            .status(repository, request.request_id)
            .await
            .unwrap(),
        first
    );
    let mut completed = false;
    for _ in 0..100 {
        if resumed
            .delete(repository, request.clone())
            .await
            .unwrap()
            .state
            == DeletionState::Deleted
        {
            completed = true;
            break;
        }
    }
    assert!(completed);
    assert_eq!(
        resumed
            .delete(repository, request.clone())
            .await
            .unwrap()
            .state,
        DeletionState::Deleted
    );
    let mut conflict = request.clone();
    conflict.request_id = RequestId::new([0xef; 16]);
    assert!(matches!(
        resumed.delete(repository, conflict).await,
        Err(Error::Conflict)
    ));
    assert!(
        registry
            .register(repository, "resurrected".into(), None)
            .await
            .is_err()
    );
    // A separate administrative client proves bytes were erased, not merely hidden.
    let independent = client(&config);
    assert!(matches!(
        independent.head(&orphan).await,
        Err(jj_store_object::Error::NotFound)
    ));
    assert!(matches!(
        independent
            .head(&Key::new(format!("repos/{repository}/manifest.json")).unwrap())
            .await,
        Err(jj_store_object::Error::NotFound)
    ));
    assert_eq!(
        independent
            .get(&preserved, None)
            .await
            .unwrap()
            .read_bounded(1024)
            .await
            .unwrap(),
        b"unpublished data"
    );
    assert!(
        independent
            .head(&Key::new(format!("repos/{other}/manifest.json")).unwrap())
            .await
            .is_ok()
    );
    runtime.shutdown().await;
}
