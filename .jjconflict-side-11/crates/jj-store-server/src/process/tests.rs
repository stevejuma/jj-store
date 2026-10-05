use super::*;
use crate::{installation, policy};
use jj_store_service::auth::{Scope, TokenPolicy};
use std::process::Command;
mod capture_factory;
const MODE: &str = "JJ_STORE_FOREGROUND_LIFETIME_TEST_MODE";

#[test]
fn cold_foreground_state_lock_follows_capture_storage_after_drop_and_cancel() {
    for mode in ["drop", "cancel", "shutdown", "failure"] {
        let output = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "process::tests::foreground_state_lock_helper",
                "--nocapture",
            ])
            .env(MODE, mode)
            .env(
                "JJ_STORE_S3_ACCESS_KEY",
                "foreground-lifetime-fixture-access",
            )
            .env(
                "JJ_STORE_S3_SECRET_KEY",
                "foreground-lifetime-fixture-secret",
            )
            .env_remove("JJ_STORE_S3_SESSION_TOKEN")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{mode}: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stdout).contains("foreground-lifetime-passed"));
    }
}

#[test]
fn foreground_state_lock_helper() {
    let Ok(mode) = std::env::var(MODE) else {
        return;
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let root = tempfile::tempdir().unwrap();
        let state = root.path().join("state");
        installation::initialize_registry(&state).unwrap();
        let operators = state.join("operators.json");
        std::fs::write(
            &operators,
            TokenPolicy::new(Scope::Operator).encode().unwrap(),
        )
        .unwrap();
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&operators, std::fs::Permissions::from_mode(0o600)).unwrap();
        let path = root.path().join("server.toml");
        let text = crate::config::EXAMPLE
            .replace("./installation.json", "./state/installation.json")
            .replace("./repositories.json", "./state/repositories.json")
            .replace("./operators.json", "./state/operators.json");
        std::fs::write(&path, text).unwrap();
        let mut config = Config::load(&path).unwrap();
        config.bind = "127.0.0.1:0".parse().unwrap();
        let locked = || policy::exclusive_lock(&state.join("server.lock")).is_err();
        if mode == "failure" {
            let occupied = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            config.bind = occupied.local_addr().unwrap();
            assert!(PreparedServer::prepare(&config, true).await.is_err());
            assert!(!locked(), "failed preparation leaked its state lock");
            assert!(!state.join("agent-capture").exists());
            return;
        }
        let server = PreparedServer::prepare(&config, true).await.unwrap();
        assert!(locked());
        let lifetime = server.storage.as_ref().unwrap().capture_lifetime();
        let store = Arc::downgrade(&server.storage.as_ref().unwrap().store());
        match mode.as_str() {
            "drop" => drop(server),
            "cancel" => {
                let job = tokio::spawn(server.run(std::future::pending()));
                tokio::task::yield_now().await;
                job.abort();
                assert!(job.await.unwrap_err().is_cancelled());
            }
            "shutdown" => {
                let mut job = tokio::spawn(server.run(async {}));
                assert!(
                    tokio::time::timeout(Duration::from_millis(100), &mut job)
                        .await
                        .is_err()
                );
                assert!(locked());
                drop(lifetime);
                job.await.unwrap().unwrap();
                assert!(!locked() && store.upgrade().is_none());
                assert!(!state.join("agent-capture").exists());
                return;
            }
            _ => panic!("unknown lifetime fixture mode"),
        }
        assert!(
            store.upgrade().is_some(),
            "capture storage ownership was lost"
        );
        assert!(
            locked(),
            "state lock was released while capture storage remained alive"
        );
        drop(lifetime);
        assert!(store.upgrade().is_none());
        assert!(!locked());
        assert!(!state.join("agent-capture").exists());
    });
    println!("foreground-lifetime-passed");
}
