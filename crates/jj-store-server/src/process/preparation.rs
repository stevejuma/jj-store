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
    let timeout = Duration::from_secs(config.limits.request_seconds);
    let (storage, host, search, indexes, replication, rpc) = authority::prepare(
        config,
        &state,
        installation,
        registry.clone(),
        search_access,
        &mut state_lock,
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
    let local_search = if matches!(
        config.access,
        Access::LocalTrusted {
            permission: jj_store_service::auth::Permission::Admin,
            ..
        }
    ) && host.is_some()
    {
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
            .map(|manager| {
                Arc::new(manager) as Arc<dyn jj_store_service::capture_management::CaptureManager>
            })
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
                verifier,
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
    let http = HttpApi::with_search(
        host,
        state.join("native-reads"),
        HttpConfig {
            origin: url::Url::parse(&config.public_url)
                .map_err(|_| Error::Config("public URL"))?
                .origin()
                .ascii_serialization(),
            access: http_access,
            registry: Some(registry as Arc<dyn jj_store_service::registry::RepositoryRegistry>),
            local_search: local_search
                .clone()
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
                    .map(|path| crate::server_settings::Manager::new(path, config, &identity))
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
        rpc,
        http,
        browser,
        development_browser_annotations: config.development_browser_annotations,
        listener,
        tls,
        concurrent: config.limits.concurrent_requests,
        timeout,
        search,
        replication,
        capture,
        agents,
        storage,
        _state_lock: state_lock,
    })
}
