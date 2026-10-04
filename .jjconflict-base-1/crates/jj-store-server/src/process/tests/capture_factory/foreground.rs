use super::*;
use jj_store_service::auth::{Scope, TokenPolicy};
use std::os::unix::fs::PermissionsExt as _;

pub(super) async fn run(f: &Fixture, mut config: Config, identity: &Identity, mode: &str) {
    // No alternate product entrypoint: only unit fixtures assemble the private slot.
    assert!(PreparedServer::prepare(&config, true).await.is_err());
    assert!(
        !config.state_dir.exists(),
        "incomplete server resources created state"
    );
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
    let enabled = config.clone();
    config.agent_capture.enabled = false;
    config.bind = "127.0.0.1:0".parse().unwrap();
    let mut server = PreparedServer::prepare(&config, true).await.unwrap();
    let store = Arc::downgrade(&server.storage.as_ref().unwrap().store());
    if mode != "foreground-unavailable" {
        controls::initialize(&enabled, identity);
    }
    let context = CaptureContext {
        identity: Some(identity),
        host: Some(&f.host),
        storage: server.storage.as_ref(),
        admission: (mode != "foreground-unavailable").then(|| f.admission.clone()),
    };
    server.capture = PreparedCapture::prepare(&enabled, context, || {
        Ok(CaptureLocations {
            home: f.root.path().join("home"),
            codex_home: None,
            temporary: f.root.path().join("temporary"),
        })
    });
    if mode == "foreground-unavailable" {
        assert!(matches!(server.capture, PreparedCapture::Unavailable));
    } else {
        assert!(matches!(server.capture, PreparedCapture::Ready(_)));
    }
    let locked = || policy::exclusive_lock(&config.state_dir.join("server.lock")).is_err();
    assert!(locked());
    if mode == "foreground-cancel" {
        let job = tokio::spawn(server.run(std::future::pending()));
        tokio::time::sleep(Duration::from_millis(30)).await;
        assert!(locked());
        job.abort();
        assert!(job.await.unwrap_err().is_cancelled());
        tokio::time::timeout(Duration::from_secs(30), async {
            while store.upgrade().is_some() {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await
        .unwrap();
    } else {
        server.run(async {}).await.unwrap();
    }
    assert!(store.upgrade().is_none());
    assert!(
        !locked(),
        "settled foreground capture leaked its process lock"
    );
    if mode == "foreground-unavailable" {
        assert!(!config.state_dir.join("agent-capture").exists());
    } else {
        // Fresh handles prove that ordered stop/drop releases the original slot.
        drop(
            jj_store_capture::agent_capture::AgentCaptureSpool::open(
                &config.state_dir.join("agent-capture/spool"),
                4 * 1024 * 1024,
            )
            .unwrap(),
        );
        assert!(!config.state_dir.join("agent-capture/kingfisher").exists());
    }
}
