use super::*;
use crate::{
    capture_control_state::{CaptureControlChange, CaptureControlJournal},
    capture_worker::CaptureWorkerPhase,
};
use jj_store_model::{PrincipalId, RequestId};
use std::os::unix::fs::PermissionsExt as _;
const CONTROL_MODE: &str = "JJ_STORE_CAPTURE_CONTROL_FACTORY_MODE";

pub(super) fn initialize(config: &Config, identity: &Identity) -> CaptureControlJournal {
    let root = config.state_dir.join("agent-capture");
    let mut builder = std::fs::DirBuilder::new();
    use std::os::unix::fs::DirBuilderExt as _;
    builder.recursive(true).mode(0o700).create(&root).unwrap();
    let installation = hex::decode(&identity.installation)
        .unwrap()
        .try_into()
        .unwrap();
    CaptureControlJournal::create(&root.join("control"), installation).unwrap()
}

#[test]
fn cold_control_policy_is_revalidated_before_foreground_dispatch() {
    for mode in [
        "persisted",
        "pause-after-prepare",
        "resume-after-prepare",
        "missing-root",
        "missing-before-prepare",
        "missing-after-prepare",
        "generation-after-prepare",
        "identity-after-prepare",
        "lock-after-prepare",
        "directory-after-prepare",
    ] {
        let output = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "process::tests::capture_factory::controls::control_factory_helper",
                "--nocapture",
            ])
            .env(CONTROL_MODE, mode)
            .env("JJ_STORE_S3_ACCESS_KEY", "capture-control-fixture-access")
            .env("JJ_STORE_S3_SECRET_KEY", "capture-control-fixture-secret")
            .env_remove("JJ_STORE_S3_SESSION_TOKEN")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{mode}: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stdout).contains("control-factory-passed"));
    }
}

#[test]
fn control_factory_helper() {
    let Ok(mode) = std::env::var(CONTROL_MODE) else {
        return;
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let f = Fixture::new(4 * 1024 * 1024).await;
        let mut config = Config::load(&f.root.path().join("server.toml")).unwrap();
        config.agent_capture.max_sources = 4;
        config.agent_capture.max_readers = 1;
        config.agent_capture.rescan_seconds = 1;
        config.agent_capture.max_pending_bytes = 4 * 1024 * 1024;
        config.agent_capture.harnesses[0].roots = vec![f.transcript.parent().unwrap().to_owned()];
        let identity_path = f.root.path().join("installation.json");
        let identity = Identity::load(&identity_path).unwrap();
        assert!(PreparedServer::prepare(&config, true).await.is_err());
        assert!(
            !config.state_dir.exists(),
            "incomplete server resources initialized capture"
        );
        let storage = AuthoritativeStorage::start(&config).await.unwrap();
        let root = config.state_dir.join("agent-capture");
        let control = root.join("control");
        let journal = (mode != "missing-root").then(|| initialize(&config, &identity));
        let snapshot = journal.as_ref().map(|j| j.current().unwrap());
        let select = |request, expected_revision, paused| {
            journal
                .as_ref()
                .unwrap()
                .record(&CaptureControlChange {
                    generation: snapshot.unwrap().generation,
                    principal: PrincipalId::new([9; 16]),
                    request: RequestId::new([request; 16]),
                    expected_revision,
                    paused,
                })
                .unwrap()
        };
        if matches!(mode.as_str(), "persisted" | "resume-after-prepare") {
            select(1, 0, true);
        }
        if mode == "missing-before-prepare" {
            std::fs::remove_file(control.join("state.json")).unwrap();
        }
        let prepared = PreparedCapture::prepare(
            &config,
            CaptureContext {
                identity: Some(&identity),
                host: Some(&f.host),
                storage: Some(&storage),
                admission: Some(f.admission.clone()),
            },
            || {
                Ok(CaptureLocations {
                    home: f.root.path().join("home"),
                    codex_home: None,
                    temporary: f.root.path().join("temporary"),
                })
            },
        );
        if matches!(mode.as_str(), "missing-root" | "missing-before-prepare") {
            assert!(matches!(prepared, PreparedCapture::Unavailable));
            assert!(!root.join("sources").exists() && !root.join("spool").exists());
            if mode == "missing-root" {
                assert!(!root.exists());
            } else {
                assert!(!control.join("state.json").exists());
            }
            storage.shutdown().await.unwrap();
            return;
        }
        assert!(matches!(prepared, PreparedCapture::Ready(_)));
        match mode.as_str() {
            "pause-after-prepare" => {
                select(1, 0, true);
            }
            "resume-after-prepare" => {
                select(2, 1, false);
            }
            "missing-after-prepare" => {
                std::fs::remove_file(control.join("state.json")).unwrap();
            }
            "generation-after-prepare" => {
                let path = control.join("state.json");
                let mut state: serde_json::Value =
                    serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
                state["generation"] = serde_json::json!("cd".repeat(16));
                std::fs::write(path, serde_json::to_vec(&state).unwrap()).unwrap();
            }
            "identity-after-prepare" => {
                let mut foreign = identity.clone();
                foreign.installation = "cd".repeat(16);
                std::fs::write(&identity_path, serde_json::to_vec(&foreign).unwrap()).unwrap();
            }
            "lock-after-prepare" => {
                std::fs::rename(
                    control.join("control.lock"),
                    f.root.path().join("held-lock"),
                )
                .unwrap();
                std::fs::write(control.join("control.lock"), []).unwrap();
                std::fs::set_permissions(
                    control.join("control.lock"),
                    std::fs::Permissions::from_mode(0o600),
                )
                .unwrap();
            }
            "directory-after-prepare" => {
                std::fs::rename(&control, f.root.path().join("held-control")).unwrap();
                CaptureControlJournal::create(&control, [0xab; 16]).unwrap();
            }
            _ => {}
        }
        let mut running = prepared.start();
        if matches!(
            mode.as_str(),
            "persisted" | "pause-after-prepare" | "resume-after-prepare"
        ) {
            let RunningCapture::Running(r) = &running else {
                panic!("valid control state refused runtime");
            };
            if mode == "resume-after-prepare" {
                tokio::time::timeout(Duration::from_secs(30), async {
                    while r.snapshot().status.observed_sources != 1 {
                        tokio::time::sleep(Duration::from_millis(5)).await;
                    }
                })
                .await
                .unwrap();
            } else {
                tokio::time::sleep(Duration::from_millis(80)).await;
                assert_eq!(r.snapshot().phase, CaptureWorkerPhase::Paused);
                assert_eq!(r.snapshot().status.observed_sources, 0);
                assert_eq!(r.snapshot().status.tracked, 0);
            }
            let sources: serde_json::Value =
                serde_json::from_slice(&std::fs::read(root.join("sources/sources.json")).unwrap())
                    .unwrap();
            assert!(sources["sources"].as_array().unwrap().is_empty());
            assert!(!root.join("kingfisher").exists());
        } else {
            assert!(
                matches!(running, RunningCapture::Unavailable),
                "changed private policy started capture"
            );
            assert!(!root.join("kingfisher").exists());
            if mode == "missing-after-prepare" {
                assert!(!control.join("state.json").exists());
            }
        }
        running.shutdown().await.unwrap();
        storage.shutdown().await.unwrap();
    });
    println!("control-factory-passed");
}
