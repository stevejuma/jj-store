use super::*;

pub(super) async fn prepare(
    config: &Config,
    require_loopback: bool,
    verifier: Option<Arc<dyn jj_store_service::local_management::WorkspaceVerifier>>,
    picker: Option<Arc<dyn jj_store_service::local_management::FolderPicker>>,
    config_path: Option<&std::path::Path>,
    folders: Option<Arc<dyn jj_store_service::local_browser::FolderProvider>>,
) -> Result<PreparedServer> {
    config.validate()?;
    capture::supported(config)?;
    if require_loopback && !config.bind.ip().is_loopback() {
        return Err(Error::Config(
            "serve-local requires a literal loopback listener",
        ));
    }
    let policy = PolicyFile::new(config.operator_policy.clone())?;
    policy.read()?;
    let tls = if let Some(tls) = &config.tls {
        let certificate = policy::read_regular(&tls.certificate, 1024 * 1024)?;
        let key = policy::read_regular(&tls.private_key, 1024 * 1024)?;
        Some(crate::listener::tls(&certificate, &key)?)
    } else {
        None
    };
    let state = policy::outside_checkout(&config.state_dir)?;
    let mut state_lock = if matches!(
        config.storage,
        crate::config::Storage::ExternalS3 { .. } | crate::config::Storage::Filesystem
    ) {
        fs::create_dir_all(&state)?;
        Some(policy::exclusive_lock(&state.join("server.lock"))?)
    } else {
        None
    };
    let (installation, catalog) = match &config.access {
        Access::Authenticated {
            installation,
            catalog,
        }
        | Access::LocalTrusted {
            installation,
            catalog,
            ..
        } => (installation, catalog),
    };
    let identity = Identity::load(installation)?;
    let registry = Arc::new(Registry::new(catalog.clone(), identity.clone())?);
    let search_access = match &config.access {
        Access::Authenticated { .. } => crate::search::SearchAccess::Authenticated,
        Access::LocalTrusted { installation, .. } => crate::search::SearchAccess::LocalTrusted(
            Identity::load(installation)?.principal(b"search-index"),
        ),
    };
    let effective = Arc::new(std::sync::RwLock::new(config.clone()));
    let workflow_admission =
        jj_store_service::workflow::HostAdmission::from(config.workflow_hosts.clone());
    let workflow_runtime: jj_store_service::workflow_host::RuntimeSlot = Default::default();
    let timeout = Duration::from_secs(config.limits.request_seconds);
    let (storage, host, search, indexes, replication, rpc) = authority::prepare(
        config,
        &state,
        installation,
        registry.clone(),
        search_access,
        &mut state_lock,
        workflow_admission.clone(),
    )
    .await?;
    let capture = capture::PreparedCapture::prepare_server(
        config,
        capture::CaptureContext {
            identity: Some(&identity),
            host: host.as_ref(),
            storage: storage.as_ref(),
            admission: verifier.clone().map(|v| {
                crate::capture_admission::CaptureAdmission::new(registry.clone(), v)
                    .excluding_execution(state.join("agent-execution/workspaces"))
            }),
        },
        capture::CaptureLocations::environment,
    );
    let http_access = match &config.access {
        Access::Authenticated { .. } => HttpAccess::Authenticated,
        Access::LocalTrusted {
            installation,
            permission,
            ..
        } => {
            let identity = Identity::load(installation)?;
            HttpAccess::LocalTrusted {
                principal: identity.principal(b"loopback-http"),
                permission: *permission,
            }
        }
    };
    let backups = config
        .backup
        .iter()
        .map(|target| BackupStatus {
            target: target.name().to_owned(),
            driver: "wal-copy".into(),
            transfer: BackupTransfer {
                state: "idle".into(),
                completed_objects: 0,
                total_objects: None,
                completed_bytes: 0,
                total_bytes: None,
            },
            last_verified_restore_point: None,
        })
        .collect();
    let local_workspaces = match (&config.access, verifier.clone().filter(|_| host.is_some())) {
        (
            Access::LocalTrusted {
                permission: jj_store_service::auth::Permission::Admin,
                ..
            },
            Some(verifier),
        ) => Some(Arc::new(crate::workspaces::Workspaces::new(
            state.clone(),
            Identity::load(installation)?,
            verifier,
        )?)
            as Arc<dyn jj_store_service::local_management::WorkspaceManager>),
        _ => None,
    };
    let local_search = if host.is_some() {
        config_path
            .map(|path| {
                crate::search_management::Manager::new(
                    path,
                    config,
                    host.as_ref().expect("native authority").clone(),
                    search.as_ref().expect("native search").control(),
                    search_access,
                    replication
                        .as_ref()
                        .expect("native replication")
                        .search_observer(),
                )
            })
            .transpose()?
            .map(Arc::new)
    } else {
        None
    };
    let local_repositories = if matches!(
        config.access,
        Access::LocalTrusted {
            permission: jj_store_service::auth::Permission::Admin,
            ..
        }
    ) {
        folders
            .map(|provider| {
                crate::local_repositories::DeviceRepositories::new(
                    state.clone(),
                    Identity::load(installation)?,
                    provider,
                    registry.clone(),
                )
                .map(|manager| manager.with_native_host(host.clone()))
            })
            .transpose()?
            .map(|manager| {
                Arc::new(manager) as Arc<dyn jj_store_service::local_browser::LocalRepositories>
            })
    } else {
        None
    };
    let local_capture = if matches!(
        config.access,
        Access::LocalTrusted {
            permission: jj_store_service::auth::Permission::Admin,
            ..
        }
    ) && host.is_some()
    {
        config_path
            .map(|path| capture.management(path, config, &identity))
            .transpose()?
    } else {
        None
    };
    let local_storage = match (&http_access, &host) {
        (
            HttpAccess::LocalTrusted {
                principal,
                permission: jj_store_service::auth::Permission::Admin,
                ..
            },
            Some(host),
        ) => Some(Arc::new(crate::storage_management::Manager::new(
            state.clone(),
            Arc::new(jj_store_service::storage_backend::NativeStorageBackend {
                host: host.clone(),
                principal: *principal,
            }),
        )?)
            as Arc<dyn jj_store_service::storage_management::StorageManager>),
        _ => None,
    };
    let agents = if let (
        HttpAccess::LocalTrusted {
            principal,
            permission: jj_store_service::auth::Permission::Admin,
        },
        Some(executable),
    ) = (&http_access, std::env::var_os("JJ_STORE_ACP_EXECUTABLE"))
    {
        let jj = std::env::var_os("JJ_STORE_AGENT_JJ_EXECUTABLE")
            .map(std::path::PathBuf::from)
            .or_else(|| {
                std::env::current_exe()
                    .ok()
                    .and_then(|p| p.parent().map(|p| p.join("jj")))
            });
        jj.map(|jj| {
            crate::agent_execution::Supervisor::open(crate::agent_execution::Resources {
                actor: *principal,
                root: state.join("agent-execution"),
                executable: executable.into(),
                jj_executable: jj.clone(),
                driver: Arc::new(crate::agent_execution::driver::Driver(
                    jj_store_jj::agent_workspaces::Provider { executable: jj },
                )),
                folders: local_repositories.clone(),
                workspaces: local_workspaces.clone(),
                verifier: verifier.clone(),
                registry: registry.clone(),
                host: host.clone(),
                native_socket: match &config.access {
                    Access::LocalTrusted { rpc_socket, .. } => Some(rpc_socket.clone()),
                    _ => None,
                },
            })
        })
        .transpose()
        .map_err(|_| Error::Config("agent execution preparation failed"))?
    } else {
        None
    };
    let directory = storage
        .as_ref()
        .map(|s| {
            crate::directory::Manager::new(
                s.store(),
                identity.installation.clone(),
                match &http_access {
                    HttpAccess::LocalTrusted {
                        principal,
                        permission: jj_store_service::auth::Permission::Admin,
                    } => Some(*principal),
                    _ => None,
                },
            )
        })
        .transpose()
        .map_err(|_| Error::Config("installation directory"))?;
    if let Some(manager) = &directory {
        match manager.audit().await {
            Ok(()) | Err(jj_store_service::directory::Error::Unavailable) => {}
            Err(_) => return Err(Error::Config("installation directory closure")),
        }
    }
    let remote_broker = if matches!(http_access, HttpAccess::Authenticated)
        && host.is_some()
        && (config_path.is_some() || config.remote_agents.is_some())
    {
        Some(
            crate::remote_agents::Broker::open_optional(
                state.join("remote-agents"),
                config.remote_agents.clone(),
                host.clone().unwrap(),
                Arc::new(directory.clone().ok_or(Error::Config("remote directory"))?),
                registry.clone(),
            )
            .map_err(|_| Error::Config("remote agents"))?,
        )
    } else {
        None
    };
    let remote_agents = remote_broker
        .clone()
        .map(|b| Arc::new(b) as Arc<dyn jj_store_service::remote_agents::RemoteAgents>);
    let capture_owner =
        if let (Some(manager), Some(storage), Some(host), Some(verifier), Some(path)) = (
            local_capture.clone(),
            storage.as_ref(),
            host.clone(),
            verifier.clone(),
            config_path,
        ) {
            Some(super::reload_capture::CaptureOwner::new(
                manager,
                storage,
                host,
                crate::capture_admission::CaptureAdmission::new(registry.clone(), verifier)
                    .excluding_execution(state.join("agent-execution/workspaces")),
                identity.clone(),
                path,
                config,
            ))
        } else {
            None
        };
    let runtime_policy = host
        .as_ref()
        .map(|h| h.runtime_policy())
        .unwrap_or_else(|| {
            let mut limits = jj_store_service::native::SessionLimits::default();
            limits.timeout = timeout;
            limits.publication.timeout = timeout;
            limits.policy.timeout = timeout;
            limits.inventory_nodes = config.limits.inventory_nodes;
            jj_store_service::runtime_policy::RuntimePolicy::new(
                jj_store_service::runtime_policy::Configuration {
                    protection: jj_store_jj::policy::Protection::default(),
                    limits,
                    max_blob: config.limits.max_blob_bytes,
                    concurrent: config.limits.concurrent_requests,
                },
            )
        });
    let reload_owners = config_path.map(|path| {
        Arc::new(super::reload::Owners {
            effective: effective.clone(),
            workflow_admission: workflow_admission.clone(),
            workflow_runtime: workflow_runtime.clone(),
            policy: runtime_policy.clone(),
            capture: capture_owner.clone(),
            remote: remote_broker,
            agents: agents.clone(),
            search: local_search.clone(),
            replication: replication.clone(),
            host: host.clone(),
            store: storage.as_ref().map(|s| s.store()),
            path: path.to_owned(),
            running_replication: Default::default(),
            gate: Default::default(),
            stopping: Default::default(),
        })
    });
    let runtime_reload = config_path
        .zip(reload_owners.clone())
        .map(|(path, owners)| {
            crate::runtime_reload::Coordinator::new(
                path,
                config,
                &identity,
                effective.clone(),
                directory
                    .clone()
                    .map(|m| Arc::new(m) as Arc<dyn jj_store_service::directory::Directory>),
                owners,
            )
        })
        .transpose()?
        .map(|m| m as Arc<dyn jj_store_service::runtime_reload::Manager>);
    let local_capture = if let Some(owner) = capture_owner {
        Some(owner as Arc<dyn jj_store_service::capture_management::CaptureManager>)
    } else {
        local_capture
            .map(|m| Arc::new(m) as Arc<dyn jj_store_service::capture_management::CaptureManager>)
    };
    let http = HttpApi::with_search(
        host,
        state.join("native-reads"),
        HttpConfig {
            workflow_hosts: config.workflow_hosts.clone(),
            workflow_admission: Some(workflow_admission.clone()),
            runtime_reload,
            runtime_policy: Some(runtime_policy),
            remote_agents: remote_agents.clone(),
            agent_administration: if matches!(http_access, HttpAccess::Authenticated)
                && !matches!(config.storage, crate::config::Storage::Filesystem)
            {
                config_path
                    .zip(directory.clone())
                    .map(|(path, manager)| {
                        crate::agent_administration::Manager::new(path, config, Arc::new(manager))
                            .map(|manager| manager.with_effective(effective.clone()))
                            .map(|manager| {
                                Arc::new(manager)
                                    as Arc<
                                        dyn jj_store_service::agent_administration::Administration,
                                    >
                            })
                    })
                    .transpose()?
            } else {
                None
            },
            installation_id: Some(identity.installation.clone()),
            local_connections: if matches!(
                http_access,
                HttpAccess::LocalTrusted {
                    permission: jj_store_service::auth::Permission::Admin,
                    ..
                }
            ) {
                Some(Arc::new(
                    crate::connections::Manager::open(
                        state.join("connections"),
                        config.insecure_loopback,
                    )
                    .map_err(|_| Error::Config("connection registry"))?,
                )
                    as Arc<dyn jj_store_service::connections::Connections>)
            } else {
                None
            },
            origin: url::Url::parse(&config.public_url)
                .map_err(|_| Error::Config("public URL"))?
                .origin()
                .ascii_serialization(),
            access: http_access.clone(),
            directory: directory
                .map(|m| Arc::new(m) as Arc<dyn jj_store_service::directory::Directory>),
            directory_operators: Some(Arc::new(policy::PolicyFile::new(
                config.operator_policy.clone(),
            )?)),
            registry: Some(registry as Arc<dyn jj_store_service::registry::RepositoryRegistry>),
            local_search: local_search
                .clone()
                .filter(|_| {
                    matches!(
                        http_access,
                        HttpAccess::LocalTrusted {
                            permission: jj_store_service::auth::Permission::Admin,
                            ..
                        }
                    )
                })
                .map(|m| m as Arc<dyn jj_store_service::search_management::SearchManager>),
            local_sync: matches!(
                config.access,
                Access::LocalTrusted {
                    permission: jj_store_service::auth::Permission::Admin,
                    ..
                }
            )
            .then(|| {
                replication.as_ref().map(|r| {
                    Arc::new(crate::sync_management::LocalSync(r.controls()))
                        as Arc<dyn jj_store_service::sync_management::SyncManager>
                })
            })
            .flatten(),
            local_settings: if matches!(
                config.access,
                Access::LocalTrusted {
                    permission: jj_store_service::auth::Permission::Admin,
                    ..
                }
            ) {
                config_path
                    .map(|path| {
                        crate::server_settings::Manager::new(path, config, &identity).map(|m| {
                            m.with_effective(effective.clone())
                                .with_workflow_runtime(workflow_runtime.clone())
                        })
                    })
                    .transpose()?
                    .map(|m| {
                        Arc::new(m)
                            as Arc<dyn jj_store_service::server_settings::ServerSettingsManager>
                    })
            } else {
                None
            },
            local_capture,
            local_agents: agents
                .clone()
                .map(|m| Arc::new(m) as jj_store_service::agent_execution::AgentManager),
            local_storage,
            local_folder_picker: picker,
            local_repositories,
            local_workspaces,
            storage_ready: true,
            backups,
            concurrency: config.limits.concurrent_requests.min(64),
            timeout,
            read: jj_store_service::reads::ReadLimits {
                max_objects: config.limits.inventory_nodes,
                timeout,
            },
            graph: jj_store_service::reads::query::Limits {
                timeout,
                ..Default::default()
            },
            uploads: jj_store_service::uploads::Limits {
                max_file_bytes: config.limits.max_blob_bytes.min(16 * 1024 * 1024),
                ..Default::default()
            },
            content: jj_store_service::reads::query::content::Limits {
                timeout,
                max_blob_bytes: config.limits.max_blob_bytes,
                ..Default::default()
            },
            ..Default::default()
        },
        indexes,
    )
    .map_err(Error::Config)?;
    if let Some(manager) = local_search {
        manager.bind(http.search_registry());
    }
    let browser = config
        .web_root
        .as_deref()
        .map(jj_store_http::assets::BrowserAssets::load)
        .transpose()
        .map_err(Error::Config)?;
    let listener = tokio::net::TcpListener::bind(config.bind).await?;
    Ok(PreparedServer {
        workflow_runtime,
        reload: reload_owners,
        rpc,
        http,
        browser,
        development_browser_annotations: config.development_browser_annotations,
        listener,
        tls,
        concurrent: 64,
        timeout,
        search,
        replication,
        capture,
        agents,
        remote_agents,
        storage,
        _state_lock: state_lock,
    })
}
