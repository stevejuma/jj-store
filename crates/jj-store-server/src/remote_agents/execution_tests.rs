//! Real custom-JJ materialization with a deterministic ACP process, not provider qualification.
use super::*;
use jj_store_object::memory::MemoryStore;
use jj_store_service::{
    administration::OperatorPolicies,
    auth::{self, Permission, Scope as TokenScope, SystemClock, TokenPolicy},
    directory::{Caller as DirectoryCaller, Directory},
    remote_agents::RemoteAgents,
};
struct Operators(TokenPolicy);
#[async_trait::async_trait]
impl OperatorPolicies for Operators {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(self.0.clone())
    }
}
#[tokio::test]
#[ignore = "requires JJ_STORE_REMOTE_TEST_JJ custom native executable"]
async fn native_remote_execution_retains_receipts_and_never_replays_after_restart() {
    let jj =
        PathBuf::from(std::env::var_os("JJ_STORE_REMOTE_TEST_JJ").expect("custom JJ executable"));
    let root = tempfile::Builder::new()
        .prefix("ra-")
        .tempdir_in("/tmp")
        .unwrap();
    let installed = crate::installation::initialize_registry(&root.path().join("state")).unwrap();
    let identity = crate::installation::Identity::load(&installed.installation_file).unwrap();
    let store = Arc::new(MemoryStore::default());
    let principal = PrincipalId::new([1; 16]);
    let mut policy = TokenPolicy::new(TokenScope::Operator);
    let token = policy
        .issue(principal, Permission::CreateRepository, None)
        .unwrap();
    let directory = Arc::new(
        crate::directory::Manager::new(
            store.clone(),
            identity.installation.clone(),
            Some(principal),
        )
        .unwrap(),
    );
    directory
        .enroll(
            &DirectoryCaller::Operator(principal),
            jj_store_model::RequestId::new([1; 16]),
            principal,
            &token,
            None,
        )
        .await
        .unwrap();
    let host = RepositoryHost::new(
        store,
        Arc::new(Operators(policy)),
        Arc::new(SystemClock),
        Default::default(),
        Default::default(),
        16 * 1024 * 1024,
    )
    .unwrap();
    let home = RepositoryId::new([2; 16]);
    let other = RepositoryId::new([3; 16]);
    for repo in [home, other] {
        host.create(
            &token,
            repo,
            jj_store_model::EpochId::new(*repo.as_bytes()),
            &token,
        )
        .await
        .unwrap();
        let session = host
            .endpoint(repo)
            .unwrap()
            .bind(token.clone())
            .unwrap()
            .into_session();
        jj_store_service::work::WorkCommandService
            .apply(
                session,
                &jj_store_service::work::Request {
                    version: 1,
                    request_id: jj_store_model::RequestId::new(*repo.as_bytes()),
                    command: jj_store_service::work::WorkCommandV1::CreateProject {
                        id: jj_store_service::work::ProjectId::new(*repo.as_bytes()),
                        title: "Remote home".into(),
                        body: String::new(),
                    },
                },
            )
            .await
            .unwrap();
    }
    let registry = Arc::new(crate::registry::Registry::new(installed.catalog, identity).unwrap());
    registry
        .register(home, "home".into(), Some(token.id()))
        .await
        .unwrap();
    let setup = Setup {
        id: "fixture".into(),
        name: "Fixture".into(),
        avatar: "🤖".into(),
        description: "Deterministic fixture".into(),
        instructions: String::new(),
        runner: "codex".into(),
        preferences: Preferences {
            model: None,
            effort: None,
            mode: Some("ask".into()),
        },
        options: Default::default(),
    };
    let acp = root.path().join("agent-control");
    std::fs::write(&acp, include_str!("../../tests/fixtures/agent_control.py")).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&acp, std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let config = Configuration {
        acp_executable: acp,
        jj_executable: jj,
        agents: vec![Agent {
            setup,
            principals: BTreeSet::from([principal]),
        }],
    };
    let open = || {
        Broker::open(
            root.path().join("remote"),
            config.clone(),
            host.clone(),
            directory.clone(),
            registry.clone(),
        )
        .unwrap()
    };
    let broker = open();
    let (_, caller) = broker.admit(token.clone()).await.unwrap();
    let scope = Scope {
        principal,
        conversation: "a".repeat(32),
        home,
        repositories: BTreeSet::from([home]),
        agents: BTreeSet::from(["fixture".into()]),
    };
    let owner = broker.owner(scope, token.clone(), false).await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(300), async {
        while !owner.supervisor.ready() {
            assert!(
                !*owner.stop.borrow(),
                "delegation revoked during readiness: {:?}",
                owner.lease.check().await
            );
            assert_ne!(
                owner.supervisor.status(principal).await.unwrap().runtime,
                "unavailable",
                "privacy preparation failed"
            );
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    })
    .await
    .unwrap();
    let socket = broker
        .0
        .root
        .join("sockets")
        .join(format!("{}.sock", &key(principal, &"a".repeat(32))[..16]));
    let diagnostic = tokio::process::Command::new(&config.jj_executable)
        .current_dir(root.path())
        .args(["--no-pager", "store", "attach"])
        .arg(root.path().join("probe"))
        .args(["--repo-id", &home.to_string(), "--unix-socket"])
        .arg(socket)
        .arg("--state-dir")
        .arg(root.path().join("probe-state"))
        .arg("--cache")
        .arg(root.path().join("probe-cache"))
        .output()
        .await
        .unwrap();
    assert!(
        diagnostic.status.success(),
        "native attach: {}",
        String::from_utf8_lossy(&diagnostic.stderr)
    );
    let command = conversations::Send {
        version: 1,
        request_id: jj_store_model::RequestId::new([4; 16]),
        conversation_id: "a".repeat(32),
        origin: conversations::Origin {
            scope: "general".into(),
            home: Some(home),
            thread: None,
            agent_target: None,
            context: String::new(),
        },
        thread_codebase: None,
        agent_ids: vec!["fixture".into()],
        attachments: vec![],
        text: "work".into(),
        comments: None,
        max_duration_ms: 10000,
    };
    let outcome = caller
        .send_conversation(principal, command.clone())
        .await
        .unwrap();
    assert_ne!(outcome.agents[0].state, "failed", "{outcome:?}");
    let session = outcome.agents[0]
        .session_id
        .clone()
        .unwrap_or_else(|| panic!("materialized participation: {outcome:?}"));
    let request = outcome.agents[0].request_id;
    tokio::time::timeout(std::time::Duration::from_secs(20), async {
        loop {
            let value = caller.outcome(principal, request).await.unwrap();
            if value.state == "completed" {
                break;
            }
            assert_ne!(value.state, "failed", "{value:?}");
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    })
    .await
    .unwrap();
    let status = caller.status(principal).await.unwrap();
    assert_eq!(status.sessions[0].repository, home);
    assert!(status.sessions[0].workspace.is_empty());
    let before = caller
        .events(principal, session.clone(), 0)
        .await
        .unwrap()
        .events;
    let mut changed = command.clone();
    changed.origin.home = Some(other);
    assert!(matches!(
        caller.send_conversation(principal, changed).await,
        Err(Error::Stale)
    ));
    let mut changed = command.clone();
    changed.text = "different retry".into();
    assert!(matches!(
        caller.send_conversation(principal, changed).await,
        Err(Error::Conflict)
    ));
    let first = caller
        .send_conversation(principal, command.clone())
        .await
        .unwrap();
    assert_eq!(first.agents[0].request_id, request);
    broker.shutdown().await.unwrap();
    let restarted = open();
    let (_, caller) = restarted.admit(token.clone()).await.unwrap();
    let replay = caller.send_conversation(principal, command).await.unwrap();
    assert_eq!(replay.agents[0].request_id, request);
    assert_eq!(replay.agents[0].state, "completed");
    let after = caller.events(principal, session, 0).await.unwrap().events;
    assert_eq!(
        before
            .iter()
            .filter(|e| e.payload["type"] == "started")
            .count(),
        after
            .iter()
            .filter(|e| e.payload["type"] == "started")
            .count()
    );
    assert!(
        std::fs::read_to_string(root.path().join("remote/scopes.json"))
            .unwrap()
            .find(token.expose().as_str())
            .is_none()
    );
    restarted.shutdown().await.unwrap();
}
