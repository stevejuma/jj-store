use super::*;
use jj_store_service::auth::{Permission, Scope, TokenPolicy};
use std::os::unix::fs::PermissionsExt as _;
const MANAGEMENT_MODE: &str = "JJ_STORE_CAPTURE_MANAGEMENT_FOREGROUND_HELPER";

#[test]
fn cold_foreground_advertises_authenticated_disabled_capture_without_creating_policy() {
    let output = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "process::tests::capture_factory::management::management_foreground_helper",
            "--nocapture",
        ])
        .env(MANAGEMENT_MODE, "1")
        .env(
            "JJ_STORE_S3_ACCESS_KEY",
            "capture-management-fixture-access",
        )
        .env(
            "JJ_STORE_S3_SECRET_KEY",
            "capture-management-fixture-secret",
        )
        .env_remove("JJ_STORE_S3_SESSION_TOKEN")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("capture-management-foreground-passed")
    );
}

#[test]
fn management_foreground_helper() {
    if std::env::var_os(MANAGEMENT_MODE).is_none() {
        return;
    }
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let f = Fixture::new(4 * 1024 * 1024).await;
        let path = f.root.path().join("server.toml");
        let mut config = Config::load(&path).unwrap();
        assert!(PreparedServer::prepare(&config, true).await.is_err());
        assert!(!config.state_dir.exists());
        config.agent_capture.enabled = false;
        config.bind = "127.0.0.1:0".parse().unwrap();
        let Access::LocalTrusted { permission, .. } = &mut config.access else {
            unreachable!();
        };
        *permission = Permission::Admin;
        std::fs::write(&path, toml::to_string(&config).unwrap()).unwrap();
        std::fs::write(
            &config.operator_policy,
            TokenPolicy::new(Scope::Operator).encode().unwrap(),
        )
        .unwrap();
        std::fs::set_permissions(
            &config.operator_policy,
            std::fs::Permissions::from_mode(0o600),
        )
        .unwrap();
        let server = PreparedServer::prepare_with_settings(&config, true, None, None, Some(&path))
            .await
            .unwrap();
        let address = server.address().unwrap();
        let (stop, stopped) = tokio::sync::oneshot::channel();
        let job = tokio::spawn(server.run(async {
            let _ = stopped.await;
        }));
        let client = reqwest::Client::new();
        let get = |path: &str| {
            client
                .get(format!("http://{address}{path}"))
                .header("host", "127.0.0.1:7443")
                .header("x-jj-store-protocol", "1")
        };
        let response = get("/v1/local/capture").send().await.unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::OK);
        let status: serde_json::Value =
            serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();
        assert_eq!(
            status,
            serde_json::json!({"version":1,"enabled":false,"policy":null,"runtime":null})
        );
        let bootstrap = get("/v1/bootstrap")
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        let bootstrap: serde_json::Value = serde_json::from_slice(&bootstrap).unwrap();
        assert_eq!(bootstrap["capabilities"]["local_capture_management"], true);
        assert!(!config.state_dir.join("agent-capture").exists());
        let _ = stop.send(());
        job.await.unwrap().unwrap();
        assert!(policy::exclusive_lock(&config.state_dir.join("server.lock")).is_ok());
        assert!(!config.state_dir.join("agent-capture").exists());
    });
    println!("capture-management-foreground-passed");
}
