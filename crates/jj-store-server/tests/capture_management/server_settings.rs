use super::*;
use jj_store_service::server_settings::{self as s, ServerSettingsManager};
fn manager(f: &Fixture) -> jj_store_server::server_settings::Manager {
    jj_store_server::server_settings::Manager::new(
        &f.source.root.path().join("server.toml"),
        &f.config,
        &f.identity,
    )
    .unwrap()
}
#[tokio::test]
async fn saves_preserve_other_sections_and_apply_only_at_restart() {
    let f = Fixture::new(false).await;
    let manager = manager(&f);
    let first = manager.status(f.actor).await.unwrap();
    assert!(!first.restart_required);
    assert!(
        first
            .inventory
            .iter()
            .any(|v| v.name == "backup" && v.saved == "None")
    );
    assert!(
        first
            .inventory
            .iter()
            .any(|v| v.name == "tls" && v.saved == "Not configured")
    );
    assert!(first.inventory.iter().any(|v| v.name == "bind"));
    assert!(
        first
            .inventory
            .iter()
            .any(|v| v.name.starts_with("storage."))
    );
    let original: toml::Value =
        toml::from_str(&fs::read_to_string(f.source.root.path().join("server.toml")).unwrap())
            .unwrap();
    let mut service = first.desired.service.clone();
    service.concurrent_requests = 3;
    let command = s::SaveServerSettings {
        version: 1,
        expected_revision: first.revision.clone(),
        update: s::ServerSettingsUpdate::Service(service.clone()),
    };
    let saved = manager.save(f.actor, command.clone()).await.unwrap();
    assert_eq!(saved.desired.service, service);
    assert_eq!(saved.effective.service, first.effective.service);
    assert!(saved.restart_required);
    assert_eq!(
        manager.save(f.actor, command).await.unwrap_err(),
        s::Error::Stale
    );
    let mut actual: toml::Value =
        toml::from_str(&fs::read_to_string(f.source.root.path().join("server.toml")).unwrap())
            .unwrap();
    actual.as_table_mut().unwrap().remove("limits");
    let mut expected = original;
    expected.as_table_mut().unwrap().remove("limits");
    assert_eq!(actual, expected);
    let protection = s::ProtectionConfiguration {
        bookmarks: ["main".into()].into(),
        commits: [jj_store_model::CommitId::new([4; 64])].into(),
    };
    let saved = manager
        .save(
            f.actor,
            s::SaveServerSettings {
                version: 1,
                expected_revision: saved.revision,
                update: s::ServerSettingsUpdate::Protection(protection.clone()),
            },
        )
        .await
        .unwrap();
    assert_eq!(saved.desired.protection, protection);
    let entry = s::SyncConfiguration {
        repository: source::A,
        upstream: "https://upstream.example/".into(),
        token_environment: "JJ_STORE_REPLICATION_TOKEN_TEST".into(),
        insecure_loopback: false,
        ca_certificate: None,
        limits: saved.sync_defaults.clone(),
    };
    let saved = manager
        .save(
            f.actor,
            s::SaveServerSettings {
                version: 1,
                expected_revision: saved.revision,
                update: s::ServerSettingsUpdate::Sync(vec![entry.clone()]),
            },
        )
        .await
        .unwrap();
    assert_eq!(saved.desired.sync, vec![entry]);
    assert!(saved.effective.sync.is_empty());
    assert!(!f.config.state_dir.join("agent-capture").exists());
    let next = Config::load(&f.source.root.path().join("server.toml")).unwrap();
    let next = jj_store_server::server_settings::Manager::new(
        &f.source.root.path().join("server.toml"),
        &next,
        &f.identity,
    )
    .unwrap();
    assert!(!next.status(f.actor).await.unwrap().restart_required);
}
#[tokio::test]
async fn settings_revalidate_authority_budgets_locks_and_regular_files() {
    let f = Fixture::new(false).await;
    let manager = manager(&f);
    let initial = manager.status(f.actor).await.unwrap();
    assert_eq!(
        manager.status(PrincipalId::new([9; 16])).await.unwrap_err(),
        s::Error::Denied
    );
    let mut service = initial.desired.service.clone();
    service.concurrent_requests = 0;
    let mut command = s::SaveServerSettings {
        version: 1,
        expected_revision: initial.revision.clone(),
        update: s::ServerSettingsUpdate::Service(service),
    };
    let original = fs::read(f.source.root.path().join("server.toml")).unwrap();
    assert_eq!(
        manager.save(f.actor, command.clone()).await.unwrap_err(),
        s::Error::Invalid
    );
    command.update = s::ServerSettingsUpdate::Service(initial.desired.service);
    command.version = 2;
    assert_eq!(
        manager.save(f.actor, command.clone()).await.unwrap_err(),
        s::Error::Invalid
    );
    command.version = 1;
    let lock = jj_store_server::policy::exclusive_lock(
        &f.source.root.path().join("server.toml.settings.lock"),
    )
    .unwrap();
    assert_eq!(
        manager.save(f.actor, command.clone()).await.unwrap_err(),
        s::Error::Busy
    );
    drop(lock);
    let mut entry = s::SyncConfiguration {
        repository: source::A,
        upstream: "http://remote.example/".into(),
        token_environment: "secret-token".into(),
        insecure_loopback: true,
        ca_certificate: None,
        limits: initial.sync_defaults,
    };
    for kind in 0..4 {
        if kind == 1 {
            entry.upstream = "https://upstream.example/".into();
        }
        if kind == 2 {
            entry.token_environment = "JJ_STORE_REPLICATION_TOKEN_TEST".into();
            entry.ca_certificate = Some("relative.pem".into());
        }
        if kind == 3 {
            entry.ca_certificate = None;
            entry.limits.max_batch_origins = 129;
        }
        command.update = s::ServerSettingsUpdate::Sync(vec![entry.clone()]);
        assert_eq!(
            manager.save(f.actor, command.clone()).await.unwrap_err(),
            s::Error::Invalid
        );
    }
    assert_eq!(
        fs::read(f.source.root.path().join("server.toml")).unwrap(),
        original
    );
    let mut changed = f.config.clone();
    let Access::LocalTrusted { permission, .. } = &mut changed.access else {
        unreachable!()
    };
    *permission = Permission::Write;
    f.save(&changed);
    assert_eq!(manager.status(f.actor).await.unwrap_err(), s::Error::Denied);
    f.save(&f.config);
    let path = f.source.root.path().join("server.toml");
    let other = f.source.root.path().join("other.toml");
    fs::rename(&path, &other).unwrap();
    std::os::unix::fs::symlink(&other, &path).unwrap();
    assert_eq!(
        manager.status(f.actor).await.unwrap_err(),
        s::Error::Unavailable
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn http_settings_contracts_are_actor_free_conditional_and_local_admin_only() {
    use reqwest::{Method, StatusCode};
    use serde_json::{Value, json};
    let f = Fixture::new(false).await;
    let manager = manager(&f);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let api = jj_store_http::HttpApi::new(
        f.source.host.clone(),
        f.source.root.path().join("settings-reads"),
        jj_store_http::Config {
            origin: base.clone(),
            access: jj_store_http::Access::LocalTrusted {
                principal: f.actor,
                permission: Permission::Admin,
            },
            local_settings: Some(Arc::new(manager)),
            ..Default::default()
        },
    )
    .unwrap();
    let task = tokio::spawn(async move { axum::serve(listener, api.router()).await.unwrap() });
    let client = reqwest::Client::new();
    let path = "/v1/local/settings";
    let mut samples = vec![];
    let (status, value) = http::request(&client, &base, Method::GET, path, None, None).await;
    assert_eq!(status, StatusCode::OK);
    samples.push(
        json!({"path":path,"method":"get","status":200,"media":"application/json","data":value}),
    );
    let mut service = value["desired"]["service"].clone();
    service["request_seconds"] = json!(25);
    let body = json!({"version":1,"expected_revision":value["revision"],"update":{"section":"service","configuration":service}});
    let mut actor = body.clone();
    actor["actor"] = json!(f.actor);
    for (body, expected, origin) in [
        (body.clone(), StatusCode::OK, None),
        (body.clone(), StatusCode::CONFLICT, None),
        (actor, StatusCode::BAD_REQUEST, None),
        (body, StatusCode::FORBIDDEN, Some("https://foreign.example")),
    ] {
        let (status, data) =
            http::request(&client, &base, Method::POST, path, Some(body), origin).await;
        assert_eq!(status, expected, "{data}");
        samples.push(json!({"path":path,"method":"post","status":status.as_u16(),"media":"application/json","data":data}));
    }
    let mut config = f.config.clone();
    let Access::LocalTrusted { permission, .. } = &mut config.access else {
        unreachable!()
    };
    *permission = Permission::Write;
    f.save(&config);
    let (status, data) = http::request(&client, &base, Method::GET, path, None, None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    samples.push(
        json!({"path":path,"method":"get","status":403,"media":"application/json","data":data}),
    );
    if let Some(path) = std::env::var_os("JJ_STORE_SETTINGS_CONTRACT_CAPTURE") {
        fs::write(
            path,
            serde_json::to_vec_pretty(&Value::Array(samples)).unwrap(),
        )
        .unwrap();
    }
    task.abort();
    let _ = task.await;
}

#[tokio::test]
async fn workflow_settings_save_admission_and_local_config_atomically_without_execution() {
    let f = Fixture::new(false).await;
    let manager = manager(&f);
    let first = manager.status(f.actor).await.unwrap();
    let host: jj_store_service::workflow_host::HostConfiguration = serde_json::from_value(serde_json::json!({
        "version":1,"host":"test","principal":f.actor,"home":"/tmp/home","state_dir":f.source.root.path().join("host-state"),"jj_executable":"/tmp/jj", "allowed_owners":[f.actor], "sources":[{"repository":source::A,"workspace":"/tmp/source"}],"commands":{"build":{"program":"/bin/sh","secrets":{"TOKEN":"HOST_TOKEN"}}},"agent_endpoint":null
    })).unwrap();
    let configuration = jj_store_service::workflow_host::WorkflowConfiguration {
        admitted: [f.actor].into(),
        local: Some(jj_store_service::workflow_host::ManagedHost {
            enabled: false,
            configuration: host,
        }),
    };
    let saved = manager
        .save(
            f.actor,
            s::SaveServerSettings {
                version: 1,
                expected_revision: first.revision,
                update: s::ServerSettingsUpdate::Workflow(configuration.clone()),
            },
        )
        .await
        .unwrap();
    assert_eq!(saved.desired.workflow, configuration);
    assert_eq!(saved.effective.workflow, first.effective.workflow);
    assert_eq!(saved.desired.service, first.desired.service);
    assert!(
        !configuration
            .local
            .as_ref()
            .unwrap()
            .configuration
            .state_dir
            .exists()
    );
    let loaded = Config::load(&f.source.root.path().join("server.toml")).unwrap();
    assert_eq!(loaded.workflow_hosts, configuration.admitted);
    assert_eq!(loaded.workflow_host, configuration.local);
    let mut invalid = configuration;
    invalid
        .local
        .as_mut()
        .unwrap()
        .configuration
        .commands
        .get_mut("build")
        .unwrap()
        .secrets
        .insert("TOKEN".into(), "secret value".into());
    assert_eq!(
        manager
            .save(
                f.actor,
                s::SaveServerSettings {
                    version: 1,
                    expected_revision: saved.revision.clone(),
                    update: s::ServerSettingsUpdate::Workflow(invalid)
                }
            )
            .await
            .unwrap_err(),
        s::Error::Invalid
    );
    assert_eq!(
        manager.status(f.actor).await.unwrap().revision,
        saved.revision
    );
}
