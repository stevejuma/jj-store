use jj_store_object::{Condition, Key, ObjectStore as _, StagedUpload};
use jj_store_server::storage::{EmbeddedStorageConfig, EmbeddedStorageRuntime};
use jj_store_server::{
    config::{Config, Storage},
    installation::initialize_local,
    process::PreparedServer,
    storage::AuthoritativeStorage,
};
use std::time::Duration;
use tokio::io::AsyncReadExt as _;

fn resident_kib(pid: u32) -> u64 {
    let output = std::process::Command::new("ps")
        .args(["-o", "rss=", "-p", &pid.to_string()])
        .output()
        .unwrap();
    String::from_utf8(output.stdout)
        .unwrap()
        .trim()
        .parse()
        .unwrap()
}

#[tokio::test]
#[ignore = "requires the separately built pinned embedded RustFS runtime"]
async fn embedded_rustfs_preserves_objects_across_graceful_restart() {
    let root = tempfile::tempdir().unwrap();
    std::fs::create_dir(root.path().join("volume")).unwrap();
    let config = EmbeddedStorageConfig {
        runtime_executable: std::fs::canonicalize(
            std::env::var_os("JJ_STORE_RUSTFS_RUNTIME")
                .expect("set JJ_STORE_RUSTFS_RUNTIME to the built embedded runtime"),
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
        access_key: "embedded-access-key-qualification".into(),
        secret_key: "embedded-secret-key-qualification".into(),
        region: "us-east-1".into(),
        bucket: "jj-store-embedded-test".into(),
        max_object_bytes: 1024 * 1024,
        timeout: Duration::from_secs(30),
    };
    let key = Key::new("qualification/persistent-object").unwrap();
    let upload = StagedUpload::from_bytes(b"persistent embedded authority", 1024)
        .await
        .unwrap();
    let debug = format!("{config:?}");
    assert!(!debug.contains(&config.access_key));
    assert!(!debug.contains(&config.secret_key));

    let binary_bytes = std::fs::metadata(&config.runtime_executable).unwrap().len();
    let started = std::time::Instant::now();
    let first = EmbeddedStorageRuntime::start(config.clone()).await.unwrap();
    let first_startup_ms = started.elapsed().as_millis();
    let first_rss_kib = resident_kib(first.process_id().unwrap());
    first.store().ensure_bucket().await.unwrap();
    assert!(matches!(
        first
            .store()
            .put(&key, Condition::Absent, &upload)
            .await
            .unwrap(),
        jj_store_object::WriteOutcome::Applied(_)
    ));
    first.shutdown().await;

    let started = std::time::Instant::now();
    let second = EmbeddedStorageRuntime::start(config).await.unwrap();
    let second_startup_ms = started.elapsed().as_millis();
    let second_rss_kib = resident_kib(second.process_id().unwrap());
    let mut download = second.store().get(&key, None).await.unwrap();
    let mut bytes = Vec::new();
    download.reader.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, b"persistent embedded authority");
    println!(
        "embedded_rustfs binary_bytes={binary_bytes} first_startup_ms={first_startup_ms} first_rss_kib={first_rss_kib} second_startup_ms={second_startup_ms} second_rss_kib={second_rss_kib}"
    );
    second.shutdown().await;
}

#[tokio::test]
#[ignore = "requires the separately built pinned embedded RustFS runtime"]
async fn managed_local_storage_locks_verifies_and_fails_closed() {
    let root = tempfile::tempdir().unwrap();
    let runtime = std::fs::canonicalize(
        std::env::var_os("JJ_STORE_RUSTFS_RUNTIME")
            .expect("set JJ_STORE_RUSTFS_RUNTIME to the built embedded runtime"),
    )
    .unwrap();
    let initialized = initialize_local(&root.path().join("state"), &runtime).unwrap();
    let config = Config::load(&initialized.config).unwrap();
    let key = Key::new("qualification/managed-persistent-object").unwrap();
    let upload = StagedUpload::from_bytes(b"managed embedded authority", 1024)
        .await
        .unwrap();

    let first = AuthoritativeStorage::start(&config).await.unwrap();
    let locked = AuthoritativeStorage::start(&config)
        .await
        .unwrap_err_or_display();
    assert!(locked.contains("lock"), "{locked}");
    assert!(matches!(
        first
            .store()
            .put(&key, Condition::Absent, &upload)
            .await
            .unwrap(),
        jj_store_object::WriteOutcome::Applied(_)
    ));
    first.shutdown().await.unwrap();
    let ownership =
        std::fs::read_to_string(initialized.state_dir.join("storage-owner.json")).unwrap();
    assert!(ownership.contains("\"sentinel_installed\":true"));

    let second = AuthoritativeStorage::start(&config).await.unwrap();
    assert_eq!(
        second
            .store()
            .get(&key, None)
            .await
            .unwrap()
            .read_bounded(1024)
            .await
            .unwrap(),
        b"managed embedded authority"
    );
    drop(second);
    tokio::time::sleep(Duration::from_millis(300)).await;
    let after_abrupt = AuthoritativeStorage::start(&config).await.unwrap();
    assert_eq!(
        after_abrupt
            .store()
            .get(&key, None)
            .await
            .unwrap()
            .read_bounded(1024)
            .await
            .unwrap(),
        b"managed embedded authority"
    );
    after_abrupt.shutdown().await.unwrap();

    let volume = initialized.state_dir.join("rustfs");
    let missing = initialized.state_dir.join("rustfs-missing");
    std::fs::rename(&volume, &missing).unwrap();
    assert!(
        AuthoritativeStorage::start(&config)
            .await
            .unwrap_err_or_display()
            .contains("managed storage identity, credentials, volume or ownership")
    );
    std::fs::rename(&missing, &volume).unwrap();

    let identity = initialized.state_dir.join("installation.json");
    let identity_bytes = std::fs::read(&identity).unwrap();
    std::fs::remove_file(&identity).unwrap();
    assert!(
        AuthoritativeStorage::start(&config)
            .await
            .unwrap_err_or_display()
            .contains("managed storage installation identity is missing")
    );
    std::fs::write(&identity, identity_bytes).unwrap();
    set_private(&identity);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let credentials = initialized.state_dir.join("rustfs-credentials.json");
        std::fs::set_permissions(&credentials, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(
            AuthoritativeStorage::start(&config)
                .await
                .unwrap_err_or_display()
                .contains("local installation file permissions")
        );
        std::fs::set_permissions(&credentials, std::fs::Permissions::from_mode(0o600)).unwrap();
    }

    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let mut blocked = config.clone();
    let Storage::EmbeddedRustfs { address, .. } = &mut blocked.storage else {
        panic!("local init must select managed RustFS")
    };
    *address = listener.local_addr().unwrap().to_string();
    let conflict = tokio::time::timeout(
        Duration::from_secs(10),
        AuthoritativeStorage::start(&blocked),
    )
    .await;
    assert!(
        matches!(conflict, Ok(Err(error)) if error.to_string().contains("embedded storage startup failed"))
    );

    let mut served = config.clone();
    served.bind = "127.0.0.1:0".parse().unwrap();
    let server = PreparedServer::prepare(&served, true).await.unwrap();
    assert!(server.address().unwrap().ip().is_loopback());
    server.run(async {}).await.unwrap();
    let after_server = AuthoritativeStorage::start(&config).await.unwrap();
    after_server.shutdown().await.unwrap();

    let absent = initialize_local(&root.path().join("absent"), &runtime).unwrap();
    let owner = absent.state_dir.join("storage-owner.json");
    let mut value: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&owner).unwrap()).unwrap();
    value["sentinel_installed"] = serde_json::Value::Bool(true);
    std::fs::write(&owner, serde_json::to_vec(&value).unwrap()).unwrap();
    set_private(&owner);
    let absent = Config::load(&absent.config).unwrap();
    assert!(
        AuthoritativeStorage::start(&absent)
            .await
            .unwrap_err_or_display()
            .contains("managed storage bucket is missing")
    );
}

trait ResultErrorDisplay<T> {
    fn unwrap_err_or_display(self) -> String;
}

impl<T> ResultErrorDisplay<T> for jj_store_server::Result<T> {
    fn unwrap_err_or_display(self) -> String {
        match self {
            Ok(_) => panic!("expected managed storage startup to fail"),
            Err(error) => error.to_string(),
        }
    }
}

fn set_private(path: &std::path::Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).unwrap();
    }
}
