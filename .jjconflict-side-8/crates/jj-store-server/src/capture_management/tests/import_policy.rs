use super::*;
use crate::{
    capture_import_state::{CaptureImportIntent, CaptureImportJournal, CaptureImportSourceBinding},
    capture_runtime::RunningCodexCapture,
    capture_supervisor::{
        CodexCaptureSupervisor, CodexSupervisorLocations, CodexSupervisorResources,
    },
};
use std::sync::{Arc, OnceLock};
mod safety;

pub(super) struct ImportFixture {
    pub(super) f: Fixture,
    pub(super) config: Config,
    pub(super) manager: Manager,
    pub(super) imports: CaptureImportJournal,
    pub(super) control: CaptureControlJournal,
    pub(super) intent: CaptureImportIntent,
    pub(super) runtime: RunningCodexCapture,
}
impl ImportFixture {
    pub(super) async fn new() -> Self {
        let (f, mut config, _, actor, _, control) = prepared().await;
        config.agent_capture.max_sources = 4;
        config.agent_capture.max_readers = 1;
        config.agent_capture.rescan_seconds = 1;
        config.agent_capture.max_pending_bytes = 4 * 1024 * 1024;
        config.agent_capture.harnesses[0].roots = vec![f.transcript.parent().unwrap().to_owned()];
        let path = f.root.path().join("server.toml");
        fs::write(&path, toml::to_string(&config).unwrap()).unwrap();
        let identity = Identity::load(&f.root.path().join("installation.json")).unwrap();
        let generation = control.current().unwrap().generation;
        let binding = CaptureManagementBinding {
            journal: control.clone(),
            generation,
            runtime: Arc::new(OnceLock::new()),
            imports: None,
        };
        let manager = Manager::new(&path, &config, &identity, Some(binding.clone())).unwrap();
        let supervisor = CodexCaptureSupervisor::prepare(
            config.agent_capture.clone(),
            CodexSupervisorLocations {
                home: None,
                codex_home: None,
            },
            CodexSupervisorResources {
                authority: Arc::new(
                    crate::capture_authority::CaptureAuthority::prepare(&config)
                        .unwrap()
                        .unwrap(),
                ),
                host: f.host.clone(),
                admission: f.admission.clone(),
                sources: f.sources.clone(),
                spool: f.spool.fork_handle(),
                privacy: f.privacy(),
            },
            Duration::from_secs(30),
        )
        .unwrap()
        .unwrap();
        let runtime = RunningCodexCapture::start(supervisor).unwrap();
        assert!(binding.runtime.set(runtime.controls()).is_ok());
        let selector = tokio::time::timeout(Duration::from_secs(10), async {
            loop {
                let page = manager
                    .sources(
                        actor,
                        CaptureSourceQuery {
                            limit: Some(100),
                            cursor: None,
                        },
                    )
                    .await
                    .unwrap();
                if let Some(source) = page.sources.first() {
                    break source.selector.clone();
                }
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await
        .unwrap();
        let imports_path = config.state_dir.join("agent-capture/imports");
        use std::os::unix::fs::DirBuilderExt as _;
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(imports_path.parent().unwrap())
            .unwrap();
        let imports = CaptureImportJournal::create(&imports_path, [0xab; 16], generation).unwrap();
        let intent = CaptureImportIntent {
            version: 1,
            generation,
            principal: actor,
            request: RequestId::new([5; 16]),
            expected_revision: 0,
            selector,
        };
        Self {
            f,
            config,
            manager,
            imports,
            control,
            intent,
            runtime,
        }
    }
    pub(super) fn save(&self, config: &Config) {
        fs::write(
            self.f.root.path().join("server.toml"),
            toml::to_string(config).unwrap(),
        )
        .unwrap();
    }
    pub(super) fn source_binding(&self) -> CaptureImportSourceBinding {
        let mut probe =
            jj_store_capture::codex_import::CodexImportProbe::select(self.f.qualified()).unwrap();
        let jj_store_capture::codex_import::CodexImportPoll::Ready(snapshot) =
            probe.poll().unwrap()
        else {
            panic!("small fixture")
        };
        let b = snapshot.binding();
        CaptureImportSourceBinding {
            profile: b.profile,
            product_version: b.profile.product_version().into(),
            physical_digest: b.physical_digest,
            header_digest: b.header_digest,
            session_digest: b.session_digest,
            prefix_digest: b.prefix_digest,
            complete_byte_offset: b.complete_byte_offset,
            complete_records: b.complete_records,
        }
    }
    pub(super) fn select_control(&self, request: u8, expected_revision: u64, paused: bool) {
        self.control
            .record(&CaptureControlChange {
                generation: self.intent.generation,
                principal: self.intent.principal,
                request: RequestId::new([request; 16]),
                expected_revision,
                paused,
            })
            .unwrap();
    }
}

#[tokio::test]
async fn import_policy_retains_management_and_control_gates_without_enrolling_history() {
    let mut f = ImportFixture::new().await;
    let before = fs::read(f.config.state_dir.join("agent-capture/imports/state.json")).unwrap();
    let policy = f
        .manager
        .begin_import(f.intent.clone(), f.imports.clone())
        .unwrap();
    assert!(policy.selected().is_none());
    assert_eq!(f.manager.status(f.intent.principal).await, Err(Error::Busy));
    assert_eq!(
        policy
            .with_current(|check| {
                assert_eq!(
                    f.control.current(),
                    Err(crate::capture_control_state::Error::Unavailable)
                );
                assert!(
                    f.f.sources
                        .lookup_admitted(&f.f.qualified())
                        .unwrap()
                        .is_none()
                );
                check()
            })
            .unwrap(),
        Ok(())
    );
    assert_eq!(
        fs::read(f.config.state_dir.join("agent-capture/imports/state.json")).unwrap(),
        before
    );
    assert!(f.f.spool.pending().unwrap().is_empty());
    drop(policy);
    assert!(f.manager.status(f.intent.principal).await.is_ok());
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn exact_import_receipt_precedes_new_revision_and_runtime_selector_rules() {
    let mut f = ImportFixture::new().await;
    f.intent.selector = format!("{}:0000000000000001", "ef".repeat(16));
    let accepted = f.imports.record(&f.intent, &f.source_binding()).unwrap();
    f.select_control(7, 0, true);
    f.select_control(8, 1, false);
    let policy = f
        .manager
        .begin_import(f.intent.clone(), f.imports.clone())
        .unwrap();
    assert_eq!(policy.selected(), Some(&accepted.receipt));
    assert_eq!(policy.with_current(|check| check()).unwrap(), Ok(()));
    drop(policy);
    let mut changed = f.intent.clone();
    changed.expected_revision = 2;
    assert!(matches!(
        f.manager.begin_import(changed, f.imports.clone()),
        Err(Error::Conflict)
    ));
    let mut new = f.intent.clone();
    new.request = RequestId::new([6; 16]);
    let policy = f
        .manager
        .begin_import(new.clone(), f.imports.clone())
        .unwrap();
    assert_eq!(policy.with_current(|_| ()), Err(Error::Stale));
    drop(policy);
    new.expected_revision = 2;
    let policy = f.manager.begin_import(new, f.imports.clone()).unwrap();
    assert_eq!(policy.with_current(|_| ()), Err(Error::SourceStale));
    drop(policy);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn paused_stopped_and_requested_pause_runtime_withhold_import_work_but_keep_receipts() {
    let mut f = ImportFixture::new().await;
    let receipt = f
        .imports
        .record(&f.intent, &f.source_binding())
        .unwrap()
        .receipt;
    f.select_control(7, 0, true);
    let policy = f
        .manager
        .begin_import(f.intent.clone(), f.imports.clone())
        .unwrap();
    assert_eq!(policy.selected(), Some(&receipt));
    assert_eq!(policy.with_current(|_| ()), Err(Error::Unavailable));
    drop(policy);
    f.select_control(8, 1, false);
    f.runtime.controls().pause().unwrap();
    let policy = f
        .manager
        .begin_import(f.intent.clone(), f.imports.clone())
        .unwrap();
    assert_eq!(policy.with_current(|_| ()), Err(Error::Unavailable));
    drop(policy);
    f.runtime.shutdown().await.unwrap();
    let policy = f
        .manager
        .begin_import(f.intent.clone(), f.imports.clone())
        .unwrap();
    assert_eq!(policy.selected(), Some(&receipt));
    assert_eq!(policy.with_current(|_| ()), Err(Error::Unavailable));
    drop(policy);
    assert_eq!(f.imports.resolve(&f.intent).unwrap(), Some(receipt));
    assert!(
        f.f.sources
            .lookup_admitted(&f.f.qualified())
            .unwrap()
            .is_none()
    );
}
