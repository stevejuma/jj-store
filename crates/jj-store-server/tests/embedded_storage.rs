use jj_store_object::{Condition, Key, ObjectStore as _, StagedUpload};
use jj_store_server::storage::{EmbeddedStorageConfig, EmbeddedStorageRuntime};
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
