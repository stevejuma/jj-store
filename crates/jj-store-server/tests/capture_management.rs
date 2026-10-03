#![cfg(unix)]
#[path = "capture_management/api_snapshot.rs"]
mod api_snapshot;
#[path = "capture_management/coverage.rs"]
mod coverage;
#[path = "capture_management/http.rs"]
mod http;
#[path = "capture_management/http_admission.rs"]
mod http_admission;
#[path = "capture_management/imports.rs"]
mod imports;
#[path = "capture_management/rescan.rs"]
mod rescan;
#[path = "capture_source/fixture.rs"]
#[allow(dead_code)]
mod source;
#[path = "capture_management/sources.rs"]
mod sources;
use jj_store_model::{PrincipalId, RequestId};
use jj_store_server::{
    capture_authority::CaptureAuthority,
    capture_control_state::CaptureControlJournal,
    capture_management::{CaptureManagementBinding, Manager},
    capture_runtime::RunningCodexCapture,
    capture_supervisor::{
        CodexCaptureSupervisor, CodexSupervisorLocations, CodexSupervisorResources,
    },
    config::{Access, Config},
    installation::Identity,
};
use jj_store_service::{auth::Permission, capture_management::*};
use std::{
    fs,
    sync::{Arc, OnceLock},
    time::Duration,
};

struct Fixture {
    source: source::Fixture,
    config: Config,
    identity: Identity,
    actor: PrincipalId,
    binding: Option<CaptureManagementBinding>,
    manager: Manager,
}
impl Fixture {
    async fn new(enabled: bool) -> Self {
        let source = source::Fixture::new(4 * 1024 * 1024).await;
        let path = source.root.path().join("server.toml");
        let mut config = Config::load(&path).unwrap();
        let Access::LocalTrusted { permission, .. } = &mut config.access else {
            unreachable!();
        };
        *permission = Permission::Admin;
        config.agent_capture.enabled = enabled;
        config.agent_capture.max_sources = 4;
        config.agent_capture.max_readers = 1;
        config.agent_capture.rescan_seconds = 1;
        config.agent_capture.max_pending_bytes = 4 * 1024 * 1024;
        config.agent_capture.harnesses[0].roots =
            vec![source.transcript.parent().unwrap().to_owned()];
        fs::write(&path, toml::to_string(&config).unwrap()).unwrap();
        let identity = Identity::load(&source.root.path().join("installation.json")).unwrap();
        let actor = identity.principal(b"loopback-http");
        let journal = enabled.then(|| {
            use std::os::unix::fs::DirBuilderExt as _;
            let root = config.state_dir.join("agent-capture");
            fs::DirBuilder::new()
                .recursive(true)
                .mode(0o700)
                .create(&root)
                .unwrap();
            CaptureControlJournal::create(&root.join("control"), [0xab; 16]).unwrap()
        });
        let binding = journal.as_ref().map(|journal| CaptureManagementBinding {
            generation: journal.current().unwrap().generation,
            journal: journal.clone(),
            runtime: Arc::new(OnceLock::new()),
            imports: None,
        });
        let manager = Manager::new(&path, &config, &identity, binding.clone()).unwrap();
        Self {
            source,
            config,
            identity,
            actor,
            binding,
            manager,
        }
    }
    fn command(
        &self,
        request: u8,
        expected_revision: u64,
        action: CaptureControlAction,
    ) -> CaptureControlCommand {
        CaptureControlCommand {
            version: 1,
            request_id: RequestId::new([request; 16]),
            generation: hex::encode(self.binding.as_ref().unwrap().generation),
            expected_revision,
            action,
        }
    }
    fn save(&self, config: &Config) {
        fs::write(
            self.source.root.path().join("server.toml"),
            toml::to_string(config).unwrap(),
        )
        .unwrap();
    }
    fn bytes(&self) -> Vec<u8> {
        fs::read(
            self.config
                .state_dir
                .join("agent-capture/control/state.json"),
        )
        .unwrap()
    }
    fn runtime(&self) -> RunningCodexCapture {
        let supervisor = CodexCaptureSupervisor::prepare(
            self.config.agent_capture.clone(),
            CodexSupervisorLocations {
                home: None,
                codex_home: None,
            },
            CodexSupervisorResources {
                authority: Arc::new(CaptureAuthority::prepare(&self.config).unwrap().unwrap()),
                host: self.source.host.clone(),
                admission: self.source.admission.clone(),
                sources: self.source.sources.clone(),
                spool: self.source.spool.fork_handle(),
                privacy: self.source.privacy(),
            },
            Duration::from_secs(2),
        )
        .unwrap()
        .unwrap();
        let runtime = RunningCodexCapture::start(supervisor).unwrap();
        assert!(
            self.binding
                .as_ref()
                .unwrap()
                .runtime
                .set(runtime.controls())
                .is_ok()
        );
        runtime
    }
}

#[tokio::test(flavor = "current_thread")]
async fn current_authority_and_configuration_withhold_without_resetting_policy() {
    let f = Fixture::new(true).await;
    let initial = f.bytes();
    let command = f.command(1, 0, CaptureControlAction::Pause);
    assert_eq!(
        f.manager
            .control(PrincipalId::new([9; 16]), command.clone())
            .await,
        Err(Error::Denied)
    );
    let mut changed = f.config.clone();
    let Access::LocalTrusted { permission, .. } = &mut changed.access else {
        unreachable!();
    };
    *permission = Permission::Write;
    f.save(&changed);
    assert_eq!(f.manager.status(f.actor).await, Err(Error::Denied));
    assert_eq!(
        f.manager.control(f.actor, command.clone()).await,
        Err(Error::Denied)
    );
    changed = f.config.clone();
    changed.agent_capture.harnesses[0]
        .roots
        .push(f.source.root.path().join("other"));
    f.save(&changed);
    assert_eq!(f.manager.status(f.actor).await, Err(Error::Unavailable));
    f.save(&f.config);
    let identity_path = f.source.root.path().join("installation.json");
    let mut foreign = f.identity.clone();
    foreign.installation = "cd".repeat(16);
    fs::write(&identity_path, serde_json::to_vec(&foreign).unwrap()).unwrap();
    assert_eq!(
        f.manager.control(f.actor, command.clone()).await,
        Err(Error::Denied)
    );
    fs::write(&identity_path, serde_json::to_vec(&f.identity).unwrap()).unwrap();
    changed = f.config.clone();
    changed.agent_capture.enabled = false;
    f.save(&changed);
    let disabled = f.manager.status(f.actor).await.unwrap();
    assert!(!disabled.enabled && disabled.policy.is_none() && disabled.runtime.is_none());
    assert_eq!(
        f.manager.control(f.actor, command.clone()).await,
        Err(Error::Disabled)
    );
    assert_eq!(f.bytes(), initial);
    f.save(&f.config);
    assert_eq!(
        f.manager
            .control(f.actor, command)
            .await
            .unwrap()
            .receipt
            .revision,
        1
    );
}

#[tokio::test(flavor = "current_thread")]
async fn old_receipt_replay_applies_latest_policy_and_cannot_restart_a_stopped_owner() {
    let f = Fixture::new(true).await;
    let mut runtime = f.runtime();
    let pause = f.command(1, 0, CaptureControlAction::Pause);
    let selected = f.manager.control(f.actor, pause.clone()).await.unwrap();
    assert!(selected.runtime_requested && selected.status.policy.unwrap().paused);
    tokio::time::timeout(Duration::from_secs(30), async {
        while runtime.snapshot().phase
            != jj_store_server::capture_worker::CaptureWorkerPhase::Paused
        {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    f.manager
        .control(f.actor, f.command(2, 1, CaptureControlAction::Resume))
        .await
        .unwrap();
    let bytes = f.bytes();
    let replay = f.manager.control(f.actor, pause).await.unwrap();
    assert!(replay.replayed && replay.receipt.paused && !replay.status.policy.unwrap().paused);
    assert_eq!(replay.receipt, selected.receipt);
    tokio::time::timeout(Duration::from_secs(30), async {
        while runtime.snapshot().phase
            == jj_store_server::capture_worker::CaptureWorkerPhase::Paused
        {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert_ne!(
        runtime.snapshot().phase,
        jj_store_server::capture_worker::CaptureWorkerPhase::Paused
    );
    assert_eq!(f.bytes(), bytes);
    runtime.shutdown().await.unwrap();
    let selected = f
        .manager
        .control(f.actor, f.command(3, 2, CaptureControlAction::Pause))
        .await
        .unwrap();
    assert!(!selected.runtime_requested && selected.status.policy.unwrap().paused);
    assert_eq!(
        selected.status.runtime.unwrap().phase,
        CaptureRuntimePhase::Stopped
    );
    let selected = f
        .manager
        .control(f.actor, f.command(4, 3, CaptureControlAction::Resume))
        .await
        .unwrap();
    assert!(!selected.runtime_requested && !selected.status.policy.unwrap().paused);
    assert_eq!(
        selected.status.runtime.unwrap().phase,
        CaptureRuntimePhase::Stopped
    );
}

#[tokio::test(flavor = "current_thread")]
async fn disabled_status_and_missing_journal_never_initialize_capture_state() {
    let f = Fixture::new(false).await;
    let root = f.config.state_dir.join("agent-capture");
    assert!(!f.manager.status(f.actor).await.unwrap().enabled);
    assert!(!root.exists());
    let mut command = CaptureControlCommand {
        version: 1,
        request_id: RequestId::new([1; 16]),
        generation: "ab".repeat(16),
        expected_revision: 0,
        action: CaptureControlAction::Pause,
    };
    assert_eq!(
        f.manager.control(f.actor, command.clone()).await,
        Err(Error::Disabled)
    );
    assert!(!root.exists());
    command.generation = "00".repeat(16);
    assert_eq!(
        f.manager.control(f.actor, command).await,
        Err(Error::Invalid)
    );
    let f = Fixture::new(true).await;
    fs::remove_file(f.config.state_dir.join("agent-capture/control/state.json")).unwrap();
    assert_eq!(f.manager.status(f.actor).await, Err(Error::Unavailable));
    assert_eq!(
        f.manager
            .control(f.actor, f.command(1, 0, CaptureControlAction::Pause))
            .await,
        Err(Error::Unavailable)
    );
    assert!(
        !f.config
            .state_dir
            .join("agent-capture/control/state.json")
            .exists()
    );
}
