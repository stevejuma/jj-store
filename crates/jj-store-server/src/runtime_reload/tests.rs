use super::*;
use jj_store_model::{PrincipalId, RequestId};
use std::sync::atomic::{AtomicUsize, Ordering};
struct Apply {
    calls: AtomicUsize,
    effective: Arc<RwLock<Config>>,
    fail: bool,
}
#[async_trait::async_trait]
impl Activation for Apply {
    async fn activate(
        &self,
        domain: Domain,
        config: &Config,
    ) -> Result<(State, Option<String>), Error> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.fail && domain == Domain::Search {
            return Err(Error::Unavailable);
        }
        self.effective.write().unwrap().limits = config.limits.clone();
        if domain == Domain::Workflows {
            self.effective.write().unwrap().workflow_hosts = config.workflow_hosts.clone();
        }
        Ok((State::Applied, None))
    }
}
fn fixture() -> (tempfile::TempDir, PathBuf, Config, Identity) {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("server.toml");
    std::fs::write(&path, crate::config::EXAMPLE).unwrap();
    let mut config = Config::load(&path).unwrap();
    let identity = Identity {
        version: 1,
        installation: "11".repeat(16),
    };
    let installation = root.path().join("installation.json");
    std::fs::write(&installation, serde_json::to_vec(&identity).unwrap()).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&installation, std::fs::Permissions::from_mode(0o600)).unwrap();
    }
    config.access = Access::LocalTrusted {
        installation,
        catalog: root.path().join("repositories.json"),
        rpc_socket: root.path().join("native.sock"),
        permission: Permission::Admin,
    };
    config.search.clear();
    config.replication.clear();
    std::fs::create_dir_all(&config.state_dir).unwrap();
    std::fs::write(&path, toml::to_string(&config).unwrap()).unwrap();
    let config = Config::load(&path).unwrap();
    (root, path, config, identity)
}
fn command(revision: String, id: u8, domains: Vec<Domain>) -> Command {
    Command {
        version: 1,
        request_id: RequestId::new([id; 16]),
        expected_revision: revision,
        domains,
    }
}
async fn settle(manager: &Coordinator, caller: Caller, command: Command) -> Outcome {
    for _ in 0..100 {
        let outcome = manager
            .outcome(caller.clone(), command.clone())
            .await
            .unwrap();
        if outcome.state != State::Running {
            return outcome;
        }
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    }
    panic!("reload did not settle")
}
#[tokio::test]
async fn saved_reload_retains_exact_intent_and_effective_values_without_replay() {
    let (_root, path, mut config, identity) = fixture();
    let effective = Arc::new(RwLock::new(config.clone()));
    let activation = Arc::new(Apply {
        calls: AtomicUsize::new(0),
        effective: effective.clone(),
        fail: false,
    });
    let manager = Coordinator::new(
        &path,
        &config,
        &identity,
        effective.clone(),
        None,
        activation.clone(),
    )
    .unwrap();
    let caller = Caller::Local(identity.principal(b"loopback-http"));
    config.limits.concurrent_requests = 3;
    std::fs::write(&path, toml::to_string(&config).unwrap()).unwrap();
    let status = manager.status(caller.clone()).await.unwrap();
    assert!(status.pending.contains(&Domain::Service));
    let command = command(status.saved_revision, 1, vec![Domain::Service]);
    manager
        .reload(caller.clone(), command.clone())
        .await
        .unwrap();
    let result = settle(&manager, caller.clone(), command.clone()).await;
    assert_eq!(result.state, State::Applied);
    assert!(
        !manager
            .status(caller.clone())
            .await
            .unwrap()
            .pending
            .contains(&Domain::Service)
    );
    let original = manager
        .reload(caller.clone(), command.clone())
        .await
        .unwrap();
    assert_eq!(original.generation, result.generation);
    assert_eq!(activation.calls.load(Ordering::SeqCst), 1);
    let mut conflicting = command.clone();
    conflicting.domains.push(Domain::Protection);
    assert_eq!(
        manager
            .reload(caller.clone(), conflicting)
            .await
            .unwrap_err(),
        Error::Conflict
    );
    assert_eq!(
        manager
            .outcome(Caller::Local(PrincipalId::new([9; 16])), command.clone())
            .await
            .unwrap_err(),
        Error::Denied
    );
    let reopened = Coordinator::new(
        &path,
        &config,
        &identity,
        effective,
        None,
        activation.clone(),
    )
    .unwrap();
    assert_eq!(
        reopened.outcome(caller, command).await.unwrap().state,
        State::Applied
    );
    assert_eq!(activation.calls.load(Ordering::SeqCst), 1);
}
#[tokio::test]
async fn malformed_stale_and_infrastructure_reload_never_dispatch() {
    let (_root, path, config, identity) = fixture();
    let effective = Arc::new(RwLock::new(config.clone()));
    let apply = Arc::new(Apply {
        calls: AtomicUsize::new(0),
        effective: effective.clone(),
        fail: false,
    });
    let manager =
        Coordinator::new(&path, &config, &identity, effective, None, apply.clone()).unwrap();
    let caller = Caller::Local(identity.principal(b"loopback-http"));
    let revision = manager.status(caller.clone()).await.unwrap().saved_revision;
    assert_eq!(
        manager
            .reload(
                caller.clone(),
                command(revision.clone(), 1, vec![Domain::Service, Domain::Service])
            )
            .await
            .unwrap_err(),
        Error::Invalid
    );
    assert_eq!(
        manager
            .reload(
                caller.clone(),
                command("aa".repeat(32), 2, vec![Domain::Service])
            )
            .await
            .unwrap_err(),
        Error::Stale
    );
    let mut changed = config.clone();
    changed.bind = "127.0.0.1:8555".parse().unwrap();
    std::fs::write(&path, toml::to_string(&changed).unwrap()).unwrap();
    let revision = manager.saved().unwrap().0;
    assert_eq!(
        manager
            .reload(caller, command(revision, 3, vec![Domain::Service]))
            .await
            .unwrap_err(),
        Error::Invalid
    );
    assert_eq!(apply.calls.load(Ordering::SeqCst), 0);
}
#[tokio::test]
async fn failure_is_reported_per_domain_and_original_receipt_is_not_reexecuted() {
    let (_root, path, config, identity) = fixture();
    let effective = Arc::new(RwLock::new(config.clone()));
    let apply = Arc::new(Apply {
        calls: AtomicUsize::new(0),
        effective: effective.clone(),
        fail: true,
    });
    let manager =
        Coordinator::new(&path, &config, &identity, effective, None, apply.clone()).unwrap();
    let caller = Caller::Local(identity.principal(b"loopback-http"));
    let revision = manager.status(caller.clone()).await.unwrap().saved_revision;
    let command = command(revision, 1, vec![Domain::Service, Domain::Search]);
    manager
        .reload(caller.clone(), command.clone())
        .await
        .unwrap();
    let result = settle(&manager, caller.clone(), command.clone()).await;
    assert_eq!(result.state, State::Failed);
    assert_eq!(result.results[0].state, State::Applied);
    assert_eq!(result.results[1].state, State::Failed);
    manager.reload(caller, command).await.unwrap();
    assert_eq!(apply.calls.load(Ordering::SeqCst), 2);
}
#[tokio::test]
async fn independent_operator_roles_are_fresh_and_agent_management_cannot_reload_service() {
    use jj_store_service::{
        auth::BearerToken,
        directory::{Caller as D, Directory},
    };
    let (_root, path, mut config, identity) = fixture();
    let installation = match &config.access {
        Access::LocalTrusted { installation, .. } => installation.clone(),
        _ => unreachable!(),
    };
    config.access = Access::Authenticated {
        installation,
        catalog: config.state_dir.join("catalog.json"),
    };
    std::fs::write(&path, toml::to_string(&config).unwrap()).unwrap();
    let config = Config::load(&path).unwrap();
    let actor = PrincipalId::new([1; 16]);
    let agent = BearerToken::generate().unwrap();
    let full = BearerToken::generate().unwrap();
    let policy = policy::PolicyFile::new(config.operator_policy.clone()).unwrap();
    policy
        .initialize_permission(actor, &agent, Permission::ManageAgents)
        .unwrap();
    policy
        .grant_permission(actor, &full, Permission::ManageConfiguration)
        .unwrap();
    let directory = Arc::new(
        crate::directory::Manager::new(
            Arc::new(jj_store_object::memory::MemoryStore::default()),
            identity.installation.clone(),
            None,
        )
        .unwrap(),
    );
    for (id, token) in [(1, &agent), (2, &full)] {
        directory
            .enroll(
                &D::Operator(actor),
                RequestId::new([id; 16]),
                actor,
                token,
                None,
            )
            .await
            .unwrap();
    }
    let effective = Arc::new(RwLock::new(config.clone()));
    let apply = Arc::new(Apply {
        calls: AtomicUsize::new(0),
        effective: effective.clone(),
        fail: false,
    });
    let manager =
        Coordinator::new(&path, &config, &identity, effective, Some(directory), apply).unwrap();
    let status = manager.status(Caller::Token(agent.clone())).await.unwrap();
    assert_eq!(status.supported, vec![Domain::RemoteAgents]);
    assert_eq!(
        manager
            .reload(
                Caller::Token(agent.clone()),
                command(status.saved_revision, 3, vec![Domain::Service])
            )
            .await
            .unwrap_err(),
        Error::Denied
    );
    assert!(
        manager
            .status(Caller::Token(full.clone()))
            .await
            .unwrap()
            .supported
            .contains(&Domain::Service)
    );
    let privileged = manager.status(Caller::Token(full.clone())).await.unwrap();
    assert!(!privileged.supported.contains(&Domain::Workflows));
    assert_eq!(
        manager
            .reload(
                Caller::Token(full),
                command(privileged.saved_revision, 44, vec![Domain::Workflows])
            )
            .await
            .unwrap_err(),
        Error::Denied
    );
    policy.revoke(agent.id()).unwrap();
    assert_eq!(
        manager.status(Caller::Token(agent)).await.unwrap_err(),
        Error::Denied
    );
}
#[tokio::test]
async fn interrupted_reload_observes_current_configuration_without_replaying() {
    let (_root, path, config, identity) = fixture();
    let effective = Arc::new(RwLock::new(config.clone()));
    let apply = Arc::new(Apply {
        calls: AtomicUsize::new(0),
        effective: effective.clone(),
        fail: false,
    });
    let manager = Coordinator::new(
        &path,
        &config,
        &identity,
        effective.clone(),
        None,
        apply.clone(),
    )
    .unwrap();
    let actor = identity.principal(b"loopback-http");
    let caller = Caller::Local(actor);
    let revision = manager.status(caller.clone()).await.unwrap().saved_revision;
    let command = command(revision, 1, vec![Domain::Service]);
    manager
        .journal
        .lock()
        .unwrap()
        .begin(actor, command.clone())
        .unwrap();
    let restored =
        Coordinator::new(&path, &config, &identity, effective, None, apply.clone()).unwrap();
    assert!(
        restored
            .status(caller.clone())
            .await
            .unwrap()
            .original
            .is_some()
    );
    assert_eq!(
        restored
            .outcome(caller.clone(), command)
            .await
            .unwrap()
            .state,
        State::Current
    );
    assert!(restored.status(caller).await.unwrap().original.is_none());
    assert_eq!(apply.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn unresolved_reload_blocks_a_new_identity_until_observed() {
    let (_root, path, config, identity) = fixture();
    let effective = Arc::new(RwLock::new(config.clone()));
    let apply = Arc::new(Apply {
        calls: AtomicUsize::new(0),
        effective: effective.clone(),
        fail: false,
    });
    let manager = Coordinator::new(
        &path,
        &config,
        &identity,
        effective.clone(),
        None,
        apply.clone(),
    )
    .unwrap();
    let actor = identity.principal(b"loopback-http");
    let caller = Caller::Local(actor);
    let revision = manager.status(caller.clone()).await.unwrap().saved_revision;
    let original = command(revision.clone(), 1, vec![Domain::Service]);
    manager
        .journal
        .lock()
        .unwrap()
        .begin(actor, original.clone())
        .unwrap();
    let restored =
        Coordinator::new(&path, &config, &identity, effective, None, apply.clone()).unwrap();
    assert_eq!(
        restored
            .reload(
                caller.clone(),
                command(revision.clone(), 2, vec![Domain::Service])
            )
            .await
            .unwrap_err(),
        Error::Busy
    );
    assert_eq!(
        restored
            .outcome(caller.clone(), original)
            .await
            .unwrap()
            .state,
        State::Current
    );
    restored
        .reload(caller.clone(), command(revision, 2, vec![Domain::Service]))
        .await
        .unwrap();
}

#[tokio::test]
async fn workflow_hosts_participate_in_saved_reload() {
    let (_root, path, config, identity) = fixture();
    let effective = Arc::new(RwLock::new(config.clone()));
    let activation = Arc::new(Apply {
        calls: AtomicUsize::new(0),
        effective: effective.clone(),
        fail: false,
    });
    let manager = Coordinator::new(
        &path,
        &config,
        &identity,
        effective.clone(),
        None,
        activation.clone(),
    )
    .unwrap();
    let caller = Caller::Local(identity.principal(b"loopback-http"));
    let status = manager.status(caller.clone()).await.unwrap();
    let supported = serde_json::to_value(status.supported).unwrap();
    assert!(
        supported
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d == "workflows")
    );
    let mut saved = config.clone();
    saved.workflow_hosts.insert(PrincipalId::new([42; 16]));
    std::fs::write(&path, toml::to_string(&saved).unwrap()).unwrap();
    let status = manager.status(caller.clone()).await.unwrap();
    assert!(status.pending.contains(&Domain::Workflows));
    let request = command(status.saved_revision, 42, vec![Domain::Workflows]);
    manager
        .reload(caller.clone(), request.clone())
        .await
        .unwrap();
    assert_eq!(
        settle(&manager, caller.clone(), request.clone())
            .await
            .state,
        State::Applied
    );
    assert_eq!(
        effective.read().unwrap().workflow_hosts,
        saved.workflow_hosts
    );
    manager.reload(caller, request).await.unwrap();
    assert_eq!(activation.calls.load(Ordering::SeqCst), 1);
}
