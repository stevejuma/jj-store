use super::*;
use crate::{
    capture_runtime::RunningCodexCapture,
    capture_supervisor::{
        CodexCaptureSupervisor, CodexSupervisorLocations, CodexSupervisorResources,
    },
};
use std::sync::OnceLock;
mod recovery;

#[tokio::test]
async fn accepted_import_resumes_tail_after_restart_without_a_second_command() {
    let mut f = ImportFixture::new().await;
    f.f.message("old", "accepted history before automatic restart");
    schedule(&f).await.unwrap();
    published(&f, "accepted history before automatic restart").await;
    pause(&f).await;
    let original = f.imports.entries().unwrap().remove(0);
    f.runtime.shutdown().await.unwrap();
    start_runtime(&mut f);
    f.f.message("tail", "tail recovered without a second import command");
    published(&f, "tail recovered without a second import command").await;
    pause(&f).await;
    assert_eq!(f.imports.entries().unwrap(), vec![original]);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn original_selector_reconciles_after_new_runtime_without_resetting_selected_history() {
    let mut f = ImportFixture::new().await;
    f.f.message("old", "scheduled history before restart");
    schedule(&f).await.unwrap();
    published(&f, "scheduled history before restart").await;
    pause(&f).await;
    let receipt = f.imports.entries().unwrap().remove(0);
    f.runtime.shutdown().await.unwrap();
    start_runtime(&mut f);
    tokio::time::timeout(Duration::from_secs(10), async {
        while f.runtime.snapshot().status.observed_sources != 1 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    f.f.message("tail", "tail reconciled after runtime restart");
    schedule(&f).await.unwrap();
    published(&f, "tail reconciled after runtime restart").await;
    pause(&f).await;
    assert_eq!(f.imports.entries().unwrap(), vec![receipt]);
    let session =
        f.f.host
            .endpoint(A)
            .unwrap()
            .bind_local_trusted(f.f.authority.principal(), Permission::Read)
            .unwrap()
            .into_session();
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    assert_eq!(pager.begin(session, 10).await.unwrap().sessions.len(), 1);
    f.runtime.shutdown().await.unwrap();
}

pub(super) fn start_runtime(f: &mut ImportFixture) {
    let path = f.f.root.path().join("server.toml");
    let identity = Identity::load(&f.f.root.path().join("installation.json")).unwrap();
    let binding = CaptureManagementBinding {
        journal: f.control.clone(),
        generation: f.intent.generation,
        runtime: Arc::new(OnceLock::new()),
        imports: Some(Arc::new(
            CaptureImportManagement::new(f.imports.clone(), &f.f.spool).unwrap(),
        )),
    };
    f.manager = Manager::new(&path, &f.config, &identity, Some(binding.clone())).unwrap();
    let mut supervisor = CodexCaptureSupervisor::prepare(
        f.config.agent_capture.clone(),
        CodexSupervisorLocations {
            home: None,
            codex_home: None,
        },
        CodexSupervisorResources {
            authority: Arc::new(
                crate::capture_authority::CaptureAuthority::prepare(&f.config)
                    .unwrap()
                    .unwrap(),
            ),
            host: f.f.host.clone(),
            admission: f.f.admission.clone(),
            sources: f.f.sources.clone(),
            spool: f.f.spool.fork_handle(),
            privacy: f.f.privacy(),
        },
        Duration::from_secs(30),
    )
    .unwrap()
    .unwrap();
    supervisor
        .retain_import_recovery(f.manager.import_recovery().unwrap())
        .unwrap();
    if f.control.current().unwrap().paused {
        supervisor.pause();
    }
    f.runtime = RunningCodexCapture::start(supervisor).unwrap();
    assert!(binding.runtime.set(f.runtime.controls()).is_ok());
}
