use super::super::capture::{CaptureContext, CaptureLocations, PreparedCapture, RunningCapture};
use super::super::*;
use crate::capture_test_fixture as fixture;
use fixture::Fixture;
use std::{path::PathBuf, process::Command};
mod controls;
mod foreground;
mod import_recovery;
mod management;
mod startup;

const MODE: &str = "JJ_STORE_CAPTURE_FACTORY_TEST_MODE";

#[test]
fn cold_capture_factory_prepares_private_resources_and_retains_ordered_lifetime() {
    for mode in [
        "disabled",
        "ready",
        "active",
        "codex-root",
        "home-root",
        "unsupported",
        "missing",
        "privacy",
        "foreign",
        "identity",
        "private",
        "symlink",
        "cancel",
        "foreground-stop",
        "foreground-cancel",
        "foreground-unavailable",
    ] {
        let locations = tempfile::tempdir().unwrap();
        for name in ["home", "codex", "temporary"] {
            std::fs::create_dir(locations.path().join(name)).unwrap();
        }
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .env("HOME", locations.path().join("home"))
            .env("TMPDIR", locations.path().join("temporary"));
        if mode == "home-root" {
            command.env_remove("CODEX_HOME");
        } else {
            command.env("CODEX_HOME", locations.path().join("codex"));
        }
        let output = command
            .args([
                "--exact",
                "process::tests::capture_factory::capture_factory_helper",
                "--nocapture",
            ])
            .env(MODE, mode)
            .env("JJ_STORE_S3_ACCESS_KEY", "capture-factory-fixture-access")
            .env("JJ_STORE_S3_SECRET_KEY", "capture-factory-fixture-secret")
            .env_remove("JJ_STORE_S3_SESSION_TOKEN")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{mode}: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stdout).contains("capture-factory-passed"));
    }
}

#[test]
fn capture_factory_helper() {
    let Ok(mode) = std::env::var(MODE) else {
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
        let mut identity = Identity::load(&f.root.path().join("installation.json")).unwrap();
        let capture_root = config.state_dir.join("agent-capture");
        if mode.starts_with("foreground-") {
            foreground::run(&f, config, &identity, &mode).await;
            return;
        }
        if matches!(mode.as_str(), "codex-root" | "home-root") {
            config.agent_capture.harnesses[0].roots.clear();
            let data = if mode == "codex-root" {
                PathBuf::from(std::env::var_os("CODEX_HOME").unwrap())
            } else {
                PathBuf::from(std::env::var_os("HOME").unwrap()).join(".codex")
            };
            std::fs::create_dir_all(data.join("sessions")).unwrap();
            std::fs::copy(&f.transcript, data.join("sessions/session.jsonl")).unwrap();
        }
        if mode == "disabled" {
            config.agent_capture.enabled = false;
            let prepared = PreparedCapture::prepare(
                &config,
                CaptureContext {
                    identity: None,
                    host: None,
                    storage: None,
                    admission: None,
                },
                || panic!("disabled capture read its environment"),
            );
            assert!(matches!(prepared, PreparedCapture::Disabled));
            assert!(!capture_root.exists());
            return;
        }
        if mode == "unsupported" {
            config.agent_capture.harnesses[0].profile =
                jj_store_capture::profile::CaptureProfile::OpencodeExportJsonV1;
            config.agent_capture.harnesses[0].product_version = config.agent_capture.harnesses[0]
                .profile
                .product_version()
                .into();
        }
        if mode == "foreign" {
            std::fs::create_dir_all(&config.state_dir).unwrap();
            let _foreign = jj_store_capture::source_registry::SourceRegistry::open(
                &capture_root.join("sources"),
                [0xcd; 16],
                4,
            )
            .unwrap();
        }
        if mode == "identity" {
            identity.installation = "cd".repeat(16);
        }
        if mode == "private" {
            use std::os::unix::fs::PermissionsExt as _;
            std::fs::create_dir_all(&capture_root).unwrap();
            std::fs::set_permissions(&capture_root, std::fs::Permissions::from_mode(0o755))
                .unwrap();
        }
        if mode == "symlink" {
            std::fs::create_dir_all(&config.state_dir).unwrap();
            let foreign = f.root.path().join("foreign-capture-state");
            std::fs::create_dir(&foreign).unwrap();
            std::fs::write(foreign.join("sentinel"), "preserve foreign state").unwrap();
            std::os::unix::fs::symlink(foreign, &capture_root).unwrap();
        }
        let storage = AuthoritativeStorage::start(&config).await.unwrap();
        if matches!(
            mode.as_str(),
            "ready" | "active" | "codex-root" | "home-root" | "cancel"
        ) {
            controls::initialize(&config, &identity);
        }
        let store = Arc::downgrade(&storage.store());
        let context = CaptureContext {
            identity: Some(&identity),
            host: Some(&f.host),
            storage: Some(&storage),
            admission: (mode != "missing").then(|| f.admission.clone()),
        };
        let home = if mode == "privacy" {
            PathBuf::from("relative-home")
        } else {
            f.root.path().join("home")
        };
        let prepared = PreparedCapture::prepare(&config, context, || {
            if matches!(mode.as_str(), "codex-root" | "home-root") {
                return CaptureLocations::environment();
            }
            Ok(CaptureLocations {
                home,
                codex_home: None,
                temporary: f.root.path().join("temporary"),
            })
        });
        if matches!(
            mode.as_str(),
            "unsupported" | "missing" | "privacy" | "foreign" | "identity" | "private" | "symlink"
        ) {
            assert!(matches!(prepared, PreparedCapture::Unavailable));
            if !matches!(mode.as_str(), "foreign" | "private" | "symlink") {
                assert!(!capture_root.exists());
            }
            assert!(!capture_root.join("spool").exists());
            if mode == "symlink" {
                assert_eq!(
                    std::fs::read_to_string(capture_root.join("sentinel")).unwrap(),
                    "preserve foreign state"
                );
            }
            if mode == "foreign" {
                assert!(
                    jj_store_capture::source_registry::SourceRegistry::open_existing(
                        &capture_root.join("sources"),
                        [0xcd; 16],
                        4
                    )
                    .unwrap()
                    .is_some()
                );
            }
            let mut running = prepared.start();
            assert!(matches!(running, RunningCapture::Unavailable));
            running.shutdown().await.unwrap();
            storage.shutdown().await.unwrap();
            assert!(store.upgrade().is_none());
            return;
        }
        assert!(matches!(prepared, PreparedCapture::Ready(_)));
        assert!(capture_root.join("sources/sources.json").exists());
        assert!(capture_root.join("spool/pending").exists());
        assert!(
            !capture_root.join("kingfisher").exists(),
            "preparation initialized the body scanner"
        );
        let mut running = prepared.start();
        if matches!(mode.as_str(), "active" | "codex-root" | "home-root") {
            let RunningCapture::Running(r) = &running else {
                panic!("capture failed to start");
            };
            tokio::time::timeout(Duration::from_secs(30), async {
                while r.snapshot().status.observed_sources != 1
                    || r.snapshot().status.resident_source_frames != 0
                {
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
            })
            .await
            .unwrap();
            assert!(!capture_root.join("kingfisher").exists());
            if mode == "active" {
                drop(f.start());
                f.message(
                    "factory-active",
                    "captured by the prepared foreground resource slot",
                );
                // Cold synchronous rules compilation can outlive one bounded step.
                // Keep the configured worker deadline; observe retained overdue work.
                tokio::time::timeout(Duration::from_secs(180), async {
                    while r.snapshot().status.caught_up != 1 {
                        if matches!(
                            r.snapshot().phase,
                            crate::capture_worker::CaptureWorkerPhase::Overdue
                                | crate::capture_worker::CaptureWorkerPhase::Draining
                        ) {
                            assert!(
                                store.upgrade().is_some(),
                                "cold overdue capture released storage"
                            );
                        }
                        tokio::time::sleep(Duration::from_millis(5)).await;
                    }
                })
                .await
                .unwrap();
                running.shutdown().await.unwrap();
                let sources = jj_store_capture::source_registry::SourceRegistry::open_existing(
                    &capture_root.join("sources"),
                    [0xab; 16],
                    4,
                )
                .unwrap()
                .unwrap();
                let source = sources.lookup_admitted(&f.qualified()).unwrap().unwrap();
                let spool = jj_store_capture::agent_capture::AgentCaptureSpool::open(
                    &capture_root.join("spool"),
                    4 * 1024 * 1024,
                )
                .unwrap();
                let progress = spool
                    .latest_reader_progress(
                        serde_json::from_value(serde_json::json!("codex")).unwrap(),
                        source,
                    )
                    .unwrap()
                    .unwrap();
                let position = serde_json::to_value(&progress.cursor.position).unwrap();
                assert_eq!(
                    position["complete_byte_offset"].as_u64(),
                    Some(std::fs::metadata(&f.transcript).unwrap().len())
                );
                assert!(spool.pending().unwrap().is_empty());
                let session = f
                    .host
                    .endpoint(fixture::A)
                    .unwrap()
                    .bind_local_trusted(
                        f.authority.principal(),
                        jj_store_service::auth::Permission::Read,
                    )
                    .unwrap()
                    .into_session();
                let pager = jj_store_service::collaboration::AgentSessionPager::new(
                    jj_store_service::pagination::Limits::default(),
                    Arc::new(jj_store_service::auth::SystemClock),
                )
                .unwrap();
                let page = pager.begin(session.clone(), 10).await.unwrap();
                assert_eq!(page.sessions.len(), 1);
                let detail = pager
                    .detail(session, page.sessions[0].session_id)
                    .await
                    .unwrap();
                let body = serde_json::to_string(&detail).unwrap();
                assert!(body.contains("captured by the prepared foreground resource slot"));
                assert!(!body.contains(f.root.path().to_str().unwrap()));
            }
        }
        if mode == "cancel" {
            drop(storage);
            assert!(store.upgrade().is_some(), "capture lost retained storage");
            drop(running);
            tokio::time::timeout(Duration::from_secs(30), async {
                while store.upgrade().is_some() {
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
            })
            .await
            .unwrap();
            return;
        }
        running.shutdown().await.unwrap();
        storage.shutdown().await.unwrap();
        assert!(store.upgrade().is_none());
        let catalog: serde_json::Value = serde_json::from_slice(
            &std::fs::read(capture_root.join("sources/sources.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            catalog["installation"],
            serde_json::to_value([0xab_u8; 16]).unwrap()
        );
        if mode != "active" {
            assert!(
                catalog["sources"].as_array().unwrap().is_empty(),
                "preparation admitted inactive history"
            );
        }
    });
    println!("capture-factory-passed");
}
