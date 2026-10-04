use super::*;
use jj_store_server::storage::AuthoritativeStorage;
use std::process::Command;

const MODE: &str = "JJ_STORE_CAPTURE_STORAGE_TEST_MODE";

#[test]
fn cold_storage_lifetime_follows_capture_drain_drop_and_cancelled_shutdown() {
    for mode in ["normal", "wait", "drop", "cancel", "late"] {
        let output = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "storage::cold_storage_lifetime_helper",
                "--nocapture",
            ])
            .env(MODE, mode)
            .env("JJ_STORE_S3_ACCESS_KEY", "capture-storage-fixture-access")
            .env("JJ_STORE_S3_SECRET_KEY", "capture-storage-fixture-secret")
            .env_remove("JJ_STORE_S3_SESSION_TOKEN")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{mode}: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stdout).contains("storage-lifetime-helper-passed"));
    }
}

#[test]
fn cold_storage_lifetime_helper() {
    let Ok(mode) = std::env::var(MODE) else {
        return;
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("server.toml");
        std::fs::write(&path, jj_store_server::config::EXAMPLE).unwrap();
        let config = jj_store_server::config::Config::load(&path).unwrap();
        let storage = AuthoritativeStorage::start(&config).await.unwrap();
        let store = Arc::downgrade(&storage.store());
        let lifetime = storage.capture_lifetime();
        if mode == "normal" || mode == "late" {
            let f = Fixture::new(4 * 1024 * 1024).await;
            let mut s = supervisor(&f);
            if mode == "late" {
                s.start_step().unwrap();
                let error = RunningCodexCapture::start_with_storage(s, lifetime)
                    .err()
                    .unwrap();
                assert!(error.to_string().contains("before dispatch"));
            } else {
                let mut r = RunningCodexCapture::start_with_storage(s, lifetime).unwrap();
                wait(&r, |s| s.status.observed_sources == 1).await;
                drop(f.start());
                wait(&r, |s| s.status.caught_up == 1).await;
                r.shutdown().await.unwrap();
            }
            storage.shutdown().await.unwrap();
            assert!(store.upgrade().is_none());
            return;
        }
        let (f, mut r, release) = draining::blocked_with(|s| {
            RunningCodexCapture::start_with_storage(s, lifetime).unwrap()
        })
        .await;
        match mode.as_str() {
            "wait" => {
                assert!(r.shutdown().await.is_err());
                let stop = storage.shutdown();
                tokio::pin!(stop);
                assert!(
                    tokio::time::timeout(Duration::from_millis(100), &mut stop)
                        .await
                        .is_err()
                );
                assert!(store.upgrade().is_some());
                assert!(f.sources.acquire_admitted(f.qualified()).is_err());
                drop(release);
                let _ = r.shutdown().await;
                stop.await.unwrap();
                assert!(store.upgrade().is_none());
            }
            "drop" | "cancel" => {
                if mode == "cancel" {
                    assert!(
                        tokio::time::timeout(Duration::from_millis(100), storage.shutdown())
                            .await
                            .is_err()
                    );
                } else {
                    drop(storage);
                }
                drop(r);
                assert!(store.upgrade().is_some());
                assert!(f.sources.acquire_admitted(f.qualified()).is_err());
                drop(release);
                tokio::time::timeout(Duration::from_secs(10), async {
                    while store.upgrade().is_some() {
                        tokio::time::sleep(Duration::from_millis(5)).await;
                    }
                })
                .await
                .unwrap();
            }
            _ => panic!("unknown fixture mode"),
        }
        assert!(f.spool.pending().unwrap().is_empty());
        drop(f.sources.acquire_admitted(f.qualified()).unwrap().unwrap());
    });
    println!("storage-lifetime-helper-passed");
}
