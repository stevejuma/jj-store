//! Real JJ workspaces and LocalAgentStore; controlled harness is fixture evidence.
#[path = "agent_execution/comment_answers.rs"]
mod comment_answers;
#[path = "agent_execution/context_delivery.rs"]
mod context_delivery;
#[path = "agent_execution/conversations.rs"]
mod conversations;
#[path = "agent_execution/fleet.rs"]
mod fleet;
#[path = "agent_execution/providers.rs"]
mod providers;
#[path = "agent_execution/real.rs"]
mod real;
#[path = "agent_execution/setups.rs"]
mod setups;
#[path = "agent_execution/tooling.rs"]
mod tooling;
use jj_store_model::{PrincipalId, RequestId};
use jj_store_server::{
    agent_execution::{Resources, Supervisor, driver::Driver},
    installation::{Identity, initialize_registry},
    local_repositories::DeviceRepositories,
    registry::Registry,
};
use jj_store_service::{
    agent_execution::*,
    local_browser::{LocalRepositories as _, RegisterFolder},
};
use std::{fs, path::Path, process::Command as Process, sync::Arc, time::Duration};
fn jj(root: &Path, args: &[&str]) {
    let output = Process::new(env!("CARGO_BIN_EXE_jj"))
        .arg("--no-pager")
        .arg("-R")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
struct Fixture {
    _root: tempfile::TempDir,
    source: std::path::PathBuf,
    resources: Resources,
    actor: PrincipalId,
}
impl Fixture {
    async fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        let source = root.path().join("source");
        let output = Process::new(env!("CARGO_BIN_EXE_jj"))
            .args(["git", "init"])
            .arg(&source)
            .output()
            .unwrap();
        assert!(output.status.success());
        fs::write(source.join("source.txt"), "base\n").unwrap();
        jj(&source, &["status"]);
        fs::write(source.join("source.txt"), "dirty\n").unwrap();
        let installed = initialize_registry(&root.path().join("device")).unwrap();
        let identity = Identity::load(&installed.installation_file).unwrap();
        let actor = identity.principal(b"loopback-http");
        let registry = Arc::new(Registry::new(installed.catalog, identity.clone()).unwrap());
        let folders = Arc::new(
            DeviceRepositories::new(
                installed.state_dir,
                identity,
                Arc::new(jj_store_cli::local_browser::LocalFolders),
                registry.clone(),
            )
            .unwrap(),
        );
        folders
            .register(RegisterFolder {
                version: 1,
                request_id: RequestId::new([1; 16]),
                expected_revision: 0,
                path: source.to_string_lossy().into_owned(),
                name: "Fixture".into(),
            })
            .await
            .unwrap();
        let executable = root.path().join("control.py");
        fs::write(&executable, include_str!("fixtures/agent_control.py")).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
        }
        let resources = Resources {
            actor,
            root: root.path().join("execution"),
            executable,
            jj_executable: env!("CARGO_BIN_EXE_jj").into(),
            driver: Arc::new(Driver(jj_store_jj::agent_workspaces::Provider {
                executable: env!("CARGO_BIN_EXE_jj").into(),
            })),
            folders: Some(folders),
            workspaces: None,
            verifier: None,
            registry,
            host: None,
            native_socket: None,
        };
        Self {
            _root: root,
            source,
            resources,
            actor,
        }
    }
}
async fn wait(
    supervisor: &Supervisor,
    actor: PrincipalId,
    predicate: impl Fn(&Status) -> bool,
) -> Status {
    let mut observed = String::new();
    let result = tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            let status = supervisor.status(actor).await.unwrap();
            observed = status
                .sessions
                .iter()
                .map(|s| format!("{}:{:?}", s.state, s.last_error))
                .collect::<Vec<_>>()
                .join(",");
            if predicate(&status) {
                return status;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await;
    result.unwrap_or_else(|_| panic!("agent state did not reach target: {observed}"))
}
#[tokio::test]
async fn lifecycle_deduplicates_permissions_publishes_and_resumes_without_prompt_replay() {
    let fixture = Fixture::new().await;
    let actor = fixture.actor;
    let source = fixture.source.clone();
    let root = fixture.resources.root.clone();
    let resources = fixture.resources.clone();
    let supervisor = Supervisor::open(fixture.resources).unwrap();
    tokio::time::timeout(Duration::from_secs(240), async {
        while !supervisor.ready() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    let publication = supervisor.start_publication();
    let targets = supervisor.targets(actor, None).await.unwrap();
    let target = &targets[0];
    let launch = Launch {
        version: 1,
        request_id: RequestId::new([2; 16]),
        target_id: target.id.clone(),
        repository: target.repository,
        operation: target.operation,
        commit: target.commit.clone(),
        runner: "codex".into(),
        setup_id: None,
        context: None,
    };
    let outcome = supervisor.launch(actor, launch.clone()).await.unwrap();
    let id = outcome.session_id.unwrap();
    assert_eq!(
        supervisor
            .launch(actor, launch)
            .await
            .unwrap()
            .session_id
            .as_deref(),
        Some(id.as_str())
    );
    wait(&supervisor, actor, |s| s.sessions[0].state == "idle").await;
    let configure = Command {
        version: 1,
        request_id: RequestId::new([3; 16]),
        session_id: id.clone(),
        action: Action::Configure {
            preferences: Preferences {
                mode: Some("ask".into()),
                model: None,
                effort: None,
            },
        },
    };
    supervisor.command(actor, configure).await.unwrap();
    wait(&supervisor, actor, |s| {
        s.sessions[0].capabilities["modeReady"] == true
    })
    .await;
    let prompt = Command {
        version: 1,
        request_id: RequestId::new([4; 16]),
        session_id: id.clone(),
        action: Action::Prompt {
            comments: None,
            text: "permission".into(),
            max_duration_ms: 10000,
        },
    };
    supervisor.command(actor, prompt.clone()).await.unwrap();
    supervisor.command(actor, prompt.clone()).await.unwrap();
    let conflicting = Command {
        action: Action::Prompt {
            comments: None,
            text: "changed".into(),
            max_duration_ms: 10000,
        },
        ..prompt
    };
    assert!(matches!(
        supervisor.command(actor, conflicting).await,
        Err(Error::Conflict)
    ));
    wait(&supervisor, actor, |s| s.sessions[0].state == "running").await;
    let permission = Command {
        version: 1,
        request_id: RequestId::new([5; 16]),
        session_id: id.clone(),
        action: Action::Permission {
            permission_id: "permission-one".into(),
            option_id: Some("invented".into()),
        },
    };
    // Await the live permission, not merely the preceding started frame.
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            if supervisor
                .events(actor, id.clone(), 0)
                .await
                .unwrap()
                .events
                .iter()
                .any(|e| e.payload["type"] == "permission")
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    assert!(matches!(
        supervisor.command(actor, permission.clone()).await,
        Err(Error::Invalid)
    ));
    supervisor
        .command(
            actor,
            Command {
                action: Action::Permission {
                    permission_id: "permission-one".into(),
                    option_id: Some("allow".into()),
                },
                ..permission
            },
        )
        .await
        .unwrap();
    wait(&supervisor, actor, |s| s.sessions[0].state == "idle").await;
    assert!(matches!(
        supervisor
            .command(
                actor,
                Command {
                    version: 1,
                    request_id: RequestId::new([6; 16]),
                    session_id: id.clone(),
                    action: Action::Permission {
                        permission_id: "permission-one".into(),
                        option_id: Some("allow".into())
                    }
                }
            )
            .await,
        Err(Error::Stale)
    ));
    supervisor
        .command(
            actor,
            Command {
                version: 1,
                request_id: RequestId::new([7; 16]),
                session_id: id.clone(),
                action: Action::Prompt {
                    comments: None,
                    text: "edit".into(),
                    max_duration_ms: 10000,
                },
            },
        )
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            if supervisor
                .outcome(actor, RequestId::new([7; 16]))
                .await
                .unwrap()
                .state
                == "completed"
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    let status = wait(&supervisor, actor, |s| {
        s.sessions[0].state == "idle" && s.sessions[0].review_commit != s.sessions[0].base_commit
    })
    .await;
    assert_eq!(
        fs::read_to_string(source.join("source.txt")).unwrap(),
        "dirty\n"
    );
    assert!(!source.join("agent-edit.txt").exists());
    assert_eq!(
        fs::read_to_string(Path::new(&status.sessions[0].workspace).join("agent-edit.txt"))
            .unwrap(),
        "review this\n"
    );
    wait(&supervisor, actor, |s| {
        s.sessions[0].published_sequence == s.sessions[0].last_sequence
    })
    .await;
    assert!(
        !fs::read_to_string(root.join("state.json"))
            .unwrap()
            .contains(&format!("Fixture in {}", status.sessions[0].workspace))
    );
    let events = supervisor.events(actor, id.clone(), 0).await.unwrap();
    assert_eq!(
        events
            .events
            .iter()
            .filter(|e| e.payload["type"] == "prompt")
            .count(),
        2
    );
    assert!(
        events
            .events
            .iter()
            .any(|e| e.payload["decisionKind"] == "allow_once")
    );
    let end = Command {
        version: 1,
        request_id: RequestId::new([8; 16]),
        session_id: id.clone(),
        action: Action::End,
    };
    supervisor.command(actor, end).await.unwrap();
    wait(&supervisor, actor, |s| s.sessions[0].state == "ended").await;
    assert_eq!(
        supervisor
            .outcome(actor, RequestId::new([8; 16]))
            .await
            .unwrap()
            .state,
        "ended"
    );
    tokio::time::sleep(Duration::from_millis(50)).await;
    supervisor
        .command(
            actor,
            Command {
                version: 1,
                request_id: RequestId::new([9; 16]),
                session_id: id.clone(),
                action: Action::Resume,
            },
        )
        .await
        .unwrap();
    wait(&supervisor, actor, |s| s.sessions[0].state == "idle").await;
    let after_resume = supervisor.events(actor, id.clone(), 0).await.unwrap();
    assert_eq!(
        after_resume
            .events
            .iter()
            .filter(|e| e.payload["type"] == "prompt")
            .count(),
        2,
        "resume must never submit a prompt"
    );
    supervisor.shutdown().await.unwrap();
    publication.abort();
    let _ = publication.await;
    assert_eq!(
        supervisor.status(actor).await.unwrap().sessions[0].state,
        "ended"
    );
    drop(supervisor);
    let restarted = Supervisor::open(resources.clone()).unwrap();
    assert_eq!(
        restarted.status(actor).await.unwrap().sessions[0].state,
        "ended"
    );
    assert!(matches!(
        restarted.status(PrincipalId::new([0; 16])).await,
        Err(Error::Denied)
    ));
    // Simulate a crash after durable dispatch but before a result/ready phase.
    // Recovery must retain unknown status and withhold execution, not replay.
    drop(restarted);
    let state_path = root.join("state.json");
    let mut saved: serde_json::Value =
        serde_json::from_slice(&fs::read(&state_path).unwrap()).unwrap();
    saved["sessions"][&id]["view"]["state"] = serde_json::json!("running");
    saved["sessions"][&id]["workspace_ready"] = serde_json::json!(false);
    let interrupted = RequestId::new([99; 16]);
    saved["intents"][interrupted.to_string()] = serde_json::json!({"digest":"crash-fixture", "outcome":{"version":1,"request_id":interrupted,"session_id":id,"state":"dispatching","error":null}});
    fs::write(state_path, serde_json::to_vec(&saved).unwrap()).unwrap();
    let recovered = Supervisor::open(resources).unwrap();
    assert_eq!(
        recovered.outcome(actor, interrupted).await.unwrap().state,
        "outcome_unknown"
    );
    assert_eq!(
        recovered.status(actor).await.unwrap().sessions[0].state,
        "interrupted"
    );
    assert_eq!(
        recovered
            .events(actor, id.clone(), 0)
            .await
            .unwrap()
            .events
            .iter()
            .filter(|e| e.payload["type"] == "prompt")
            .count(),
        2
    );
    assert!(matches!(
        recovered
            .command(
                actor,
                Command {
                    version: 1,
                    request_id: RequestId::new([100; 16]),
                    session_id: id,
                    action: Action::ContinueWithContext
                }
            )
            .await,
        Err(Error::Stale)
    ));
}
