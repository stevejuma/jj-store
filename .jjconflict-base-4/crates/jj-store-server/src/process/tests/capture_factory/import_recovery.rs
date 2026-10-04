use super::*;
use crate::capture_import_state::{
    CaptureImportIntent, CaptureImportJournal, CaptureImportSourceBinding,
};
use jj_store_model::RequestId;
use jj_store_service::capture_management::{
    CaptureImportPhase, CaptureImportQuery, CaptureManager,
};
use std::{fs, os::unix::fs::PermissionsExt as _, time::Duration};
const MODE: &str = "JJ_STORE_CAPTURE_IMPORT_RECOVERY_FACTORY";

#[test]
fn cold_prepared_foreground_recovers_selected_history_without_an_http_command() {
    let output = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "process::tests::capture_factory::import_recovery::import_recovery_factory_helper",
            "--nocapture",
        ])
        .env(MODE, "1")
        .env("JJ_STORE_S3_ACCESS_KEY", "capture-recovery-fixture-access")
        .env("JJ_STORE_S3_SECRET_KEY", "capture-recovery-fixture-secret")
        .env_remove("JJ_STORE_S3_SESSION_TOKEN")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("prepared-import-recovery-passed"));
}

#[test]
fn import_recovery_factory_helper() {
    if std::env::var_os(MODE).is_none() {
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
        config.agent_capture.max_sources = 4;
        config.agent_capture.max_readers = 1;
        config.agent_capture.rescan_seconds = 1;
        config.agent_capture.max_pending_bytes = 4 * 1024 * 1024;
        config.agent_capture.harnesses[0].roots = vec![f.transcript.parent().unwrap().to_owned()];
        let Access::LocalTrusted { permission, .. } = &mut config.access else {
            unreachable!()
        };
        *permission = jj_store_service::auth::Permission::Admin;
        fs::write(&path, toml::to_string(&config).unwrap()).unwrap();
        let identity = Identity::load(&f.root.path().join("installation.json")).unwrap();
        let control = controls::initialize(&config, &identity);
        let selected = control.current().unwrap();
        let root = config.state_dir.join("agent-capture");
        let imports =
            CaptureImportJournal::create(&root.join("imports"), [0xab; 16], selected.generation)
                .unwrap();
        f.message("history", "history recovered by prepared foreground wiring");
        let mut probe =
            jj_store_capture::codex_import::CodexImportProbe::select(f.qualified()).unwrap();
        let jj_store_capture::codex_import::CodexImportPoll::Ready(snapshot) =
            probe.poll().unwrap()
        else {
            panic!("small fixture")
        };
        let actor = identity.principal(b"loopback-http");
        let intent = CaptureImportIntent {
            version: 1,
            generation: selected.generation,
            principal: actor,
            request: RequestId::new([5; 16]),
            expected_revision: 0,
            selector: format!("{}:0000000000000001", "ef".repeat(16)),
        };
        let original = imports
            .record(
                &intent,
                &CaptureImportSourceBinding::from_codex(snapshot.binding()),
            )
            .unwrap()
            .receipt;
        let storage = AuthoritativeStorage::start(&config).await.unwrap();
        let retained = Arc::downgrade(&storage.store());
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
        assert!(matches!(prepared, PreparedCapture::Ready(_)));
        let manager = prepared.management(&path, &config, &identity).unwrap();
        // The fresh process uses a qualified compiled rule cache, not a fresh compiler cache.
        let cache = root.join("kingfisher");
        assert!(!cache.exists());
        fs::create_dir(&cache).unwrap();
        fs::set_permissions(&cache, fs::Permissions::from_mode(0o700)).unwrap();
        if let Some(source_cache) = std::env::var_os("JJ_STORE_TRANSCRIPT_CACHE") {
            for entry in fs::read_dir(source_cache).unwrap() {
                let entry = entry.unwrap();
                if entry.path().extension().is_some_and(|e| e == "vscdb") {
                    fs::copy(entry.path(), cache.join(entry.file_name())).unwrap();
                }
            }
        }
        // Warm rules before the measured runtime; cache compilation has its own gate.
        jj_store_capture::transcript_sanitization::TranscriptSanitizer::new(
            &f.workspace(fixture::A),
            &f.root.path().join("home"),
            &f.root.path().join("temporary"),
            &cache,
        )
        .unwrap();
        let mut running = prepared.start();
        let RunningCapture::Running(active) = &running else {
            panic!("foreground capture unavailable")
        };
        let completed = tokio::time::timeout(Duration::from_secs(10), async {
            loop {
                match manager
                    .import_status(
                        actor,
                        CaptureImportQuery {
                            request_id: intent.request,
                        },
                    )
                    .await
                {
                    Ok(status) if status.progress.phase == CaptureImportPhase::Completed => break,
                    Ok(_) | Err(jj_store_service::capture_management::Error::Busy) => {}
                    Err(error) => panic!("foreground inspection: {error:?}"),
                }
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await;
        assert!(
            completed.is_ok(),
            "recovery runtime: {:?}",
            active.snapshot()
        );
        active.pause().unwrap();
        tokio::time::timeout(Duration::from_secs(10), async {
            while active.snapshot().phase != crate::capture_worker::CaptureWorkerPhase::Paused {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await
        .unwrap();
        assert_eq!(imports.entries().unwrap(), vec![original]);
        assert!(retained.upgrade().is_some());
        running.shutdown().await.unwrap();
        storage.shutdown().await.unwrap();
        assert!(retained.upgrade().is_none());
        println!("prepared-import-recovery-passed");
    });
}
