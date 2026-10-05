#![cfg(unix)]
#[path = "capture_cli/fixture.rs"]
#[allow(dead_code)]
mod cli;
#[path = "../../jj-store-server/tests/capture_source/fixture.rs"]
#[allow(dead_code)]
mod source;
use jj_store_model::RequestId;
use jj_store_server::{
    capture_authority::CaptureAuthority,
    capture_control_state::CaptureControlJournal,
    capture_import_state::CaptureImportJournal,
    capture_management::{CaptureImportManagement, CaptureManagementBinding, Manager},
    capture_runtime::RunningCodexCapture,
    capture_supervisor::{
        CodexCaptureSupervisor, CodexSupervisorLocations, CodexSupervisorResources,
    },
    config::Config,
    installation::Identity,
};
use jj_store_service::{auth::Permission, capture_management::*};
use std::{
    fs,
    sync::{Arc, OnceLock},
    time::Duration,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn explicit_cli_import_inspects_published_history_and_replays_without_a_repository() {
    let source = source::Fixture::new(4 * 1024 * 1024).await;
    source.message("old", "CLI imported history");
    let path = source.root.path().join("server.toml");
    let root =
        serde_json::to_string(source.transcript.parent().unwrap().to_str().unwrap()).unwrap();
    let text = fs::read_to_string(&path).unwrap()
        .replace("permission = \"write\"", "permission = \"admin\"")
        .replace("[agent_capture]\nenabled = true", "[agent_capture]\nenabled = true\nmax_sources = 4\nmax_readers = 1\nrescan_seconds = 1\nmax_pending_bytes = 4194304")
        .replace("\nproduct_version = \"0.156.1\"", &format!("\nproduct_version = \"0.156.1\"\nroots = [{root}]"));
    fs::write(&path, text).unwrap();
    let config = Config::load(&path).unwrap();
    let identity = Identity::load(&source.root.path().join("installation.json")).unwrap();
    let actor = identity.principal(b"loopback-http");
    let root = config.state_dir.join("agent-capture");
    use std::os::unix::fs::DirBuilderExt as _;
    fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(&root)
        .unwrap();
    let control = CaptureControlJournal::create(&root.join("control"), [0xab; 16]).unwrap();
    let generation = control.current().unwrap().generation;
    let journal =
        CaptureImportJournal::create(&root.join("imports"), [0xab; 16], generation).unwrap();
    let binding = CaptureManagementBinding {
        journal: control,
        generation,
        runtime: Arc::new(OnceLock::new()),
        imports: Some(Arc::new(
            CaptureImportManagement::new(journal.clone(), &source.spool).unwrap(),
        )),
    };
    let manager = Manager::new(&path, &config, &identity, Some(binding.clone())).unwrap();
    let supervisor = CodexCaptureSupervisor::prepare(
        config.agent_capture.clone(),
        CodexSupervisorLocations {
            home: None,
            codex_home: None,
        },
        CodexSupervisorResources {
            authority: Arc::new(CaptureAuthority::prepare(&config).unwrap().unwrap()),
            host: source.host.clone(),
            admission: source.admission.clone(),
            sources: source.sources.clone(),
            spool: source.spool.fork_handle(),
            privacy: source.privacy(),
        },
        Duration::from_secs(30),
    )
    .unwrap()
    .unwrap();
    let mut runtime = RunningCodexCapture::start(supervisor).unwrap();
    assert!(binding.runtime.set(runtime.controls()).is_ok());
    let selector = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let page = manager
                .sources(actor, CaptureSourceQuery::default())
                .await
                .unwrap();
            if let Some(row) = page.sources.first() {
                break row.selector.clone();
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let api = jj_store_http::HttpApi::new(
        source.host.clone(),
        source.root.path().join("cli-http"),
        jj_store_http::Config {
            origin: base.clone(),
            access: jj_store_http::Access::LocalTrusted {
                principal: actor,
                permission: Permission::Admin,
            },
            local_capture: Some(Arc::new(manager.clone())),
            ..Default::default()
        },
    )
    .unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(async move {
        axum::serve(listener, api.router())
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let cwd = tempfile::tempdir().unwrap();
    let generation = hex::encode(generation);
    let request = RequestId::new([5; 16]);
    let request_text = request.to_string();
    let args = [
        "import",
        "--source",
        selector.as_str(),
        "--generation",
        generation.as_str(),
        "--expected-revision",
        "0",
        "--request-id",
        request_text.as_str(),
    ];
    let first = cli::successful(cli::run(cwd.path(), &base, &args).await);
    assert_eq!(first["replayed"], false);
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            match manager
                .import_status(
                    actor,
                    CaptureImportQuery {
                        request_id: request,
                    },
                )
                .await
            {
                Ok(status) if status.progress.phase == CaptureImportPhase::Completed => break,
                Ok(_) | Err(Error::Busy | Error::Unavailable) => {}
                other => panic!("{other:?}"),
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    runtime.pause().unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        while runtime.snapshot().phase
            != jj_store_server::capture_worker::CaptureWorkerPhase::Paused
        {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    fs::remove_file(&source.transcript).unwrap();
    let inspect = ["import-status", "--request-id", request_text.as_str()];
    let status = cli::successful(cli::run(cwd.path(), &base, &inspect).await);
    assert_eq!(status["receipt"], first["receipt"]);
    assert_eq!(status["progress"]["phase"], "completed");
    assert_eq!(status["progress"]["tail_phase"], "withheld");
    let replay = cli::successful(cli::run(cwd.path(), &base, &args).await);
    assert_eq!(replay["receipt"], first["receipt"]);
    assert_eq!(replay["replayed"], true);
    let output = cli::run_format(cwd.path(), &base, "text", &inspect).await;
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("Completed") && text.contains("continuing tail Withheld"));
    assert_eq!(journal.entries().unwrap().len(), 1);
    assert_eq!(fs::read_dir(cwd.path()).unwrap().count(), 0);
    runtime.shutdown().await.unwrap();
    let _ = stop.send(());
    task.await.unwrap();
}
