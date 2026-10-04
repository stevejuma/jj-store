use super::*;
use crate::capture_control_state::{CaptureControlChange, CaptureControlJournal};
use jj_store_capture::profile::CaptureProfile;
use jj_store_model::{PrincipalId, RequestId};
use std::os::unix::fs::PermissionsExt as _;

const MODE: &str = "JJ_STORE_CAPTURE_STARTUP_FACTORY_MODE";
#[test]
fn server_capture_initializes_only_absent_state_and_preserves_existing_policy() {
    let output = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "process::tests::capture_factory::startup::server_capture_initialization_helper",
            "--nocapture",
        ])
        .env(MODE, "initialize")
        .env("JJ_STORE_S3_ACCESS_KEY", "capture-startup-fixture-access")
        .env("JJ_STORE_S3_SECRET_KEY", "capture-startup-fixture-secret")
        .env_remove("JJ_STORE_S3_SESSION_TOKEN")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("startup-factory-passed"));
}
#[tokio::test]
async fn server_capture_initialization_helper() {
    if std::env::var_os(MODE).is_none() {
        return;
    }
    for profile in [
        CaptureProfile::CodexCliJsonlV1,
        CaptureProfile::CodexDesktopJsonlV1,
        CaptureProfile::OpencodeReadOnlyApiV1,
    ] {
        let f = Fixture::new(4 * 1024 * 1024).await;
        let mut config = Config::load(&f.root.path().join("server.toml")).unwrap();
        config.agent_capture.harnesses[0].profile = profile;
        config.agent_capture.harnesses[0].product_version = profile.product_version().into();
        config.agent_capture.harnesses[0].roots =
            if profile == CaptureProfile::OpencodeReadOnlyApiV1 {
                vec![]
            } else {
                vec![f.transcript.parent().unwrap().to_owned()]
            };
        config.agent_capture.harnesses[0].endpoint = (profile
            == CaptureProfile::OpencodeReadOnlyApiV1)
            .then(|| "http://127.0.0.1:9/".into());
        let identity = Identity::load(&f.root.path().join("installation.json")).unwrap();
        let storage = AuthoritativeStorage::start(&config).await.unwrap();
        let root = config.state_dir.join("agent-capture");
        let prepare = || {
            PreparedCapture::prepare_server(
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
            )
        };
        assert!(matches!(prepare(), PreparedCapture::Ready(_)));
        for path in [root.clone(), root.join("control"), root.join("imports")] {
            assert_eq!(
                std::fs::metadata(path).unwrap().permissions().mode() & 0o777,
                0o700
            );
        }
        for path in [
            root.join("control/state.json"),
            root.join("imports/state.json"),
        ] {
            assert_eq!(
                std::fs::metadata(path).unwrap().permissions().mode() & 0o777,
                0o600
            );
        }
        assert!(!root.join("kingfisher").exists());
        let journal = CaptureControlJournal::open(&root.join("control"), [0xab; 16]).unwrap();
        let selected = journal.current().unwrap();
        journal
            .record(&CaptureControlChange {
                generation: selected.generation,
                principal: PrincipalId::new([1; 16]),
                request: RequestId::new([2; 16]),
                expected_revision: 0,
                paused: true,
            })
            .unwrap();
        let policy = std::fs::read(root.join("control/state.json")).unwrap();
        let imports = std::fs::read(root.join("imports/state.json")).unwrap();
        assert!(matches!(prepare(), PreparedCapture::Ready(_)));
        assert_eq!(
            std::fs::read(root.join("control/state.json")).unwrap(),
            policy
        );
        assert_eq!(
            std::fs::read(root.join("imports/state.json")).unwrap(),
            imports
        );
        std::fs::remove_file(root.join("imports/state.json")).unwrap();
        assert!(matches!(prepare(), PreparedCapture::Ready(_)));
        assert!(!root.join("imports/state.json").exists());
        std::fs::remove_file(root.join("control/state.json")).unwrap();
        assert!(matches!(prepare(), PreparedCapture::Unavailable));
        assert!(!root.join("control/state.json").exists());
        std::fs::remove_dir_all(root.join("control")).unwrap();
        assert!(matches!(prepare(), PreparedCapture::Unavailable));
        assert!(!root.join("control").exists());
        storage.shutdown().await.unwrap();
    }
    println!("startup-factory-passed");
}

#[tokio::test]
async fn configured_opencode_profile_prepares_without_contacting_an_endpoint() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let mut config = Config::load(&f.root.path().join("server.toml")).unwrap();
    config.agent_capture.harnesses[0] = crate::capture_config::HarnessConfig {
        profile: CaptureProfile::OpencodeReadOnlyApiV1,
        product_version: "1.18.30".into(),
        roots: vec![],
        endpoint: Some("http://127.0.0.1:9/".into()),
    };
    assert!(super::super::super::capture::supported(&config).is_ok());
}
