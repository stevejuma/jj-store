use crate::{
    Error, Result,
    config::{Access, Config},
    installation::Identity,
    policy::{self, PolicyFile},
    registry::Registry,
    storage::AuthoritativeStorage,
};
use jj_store_http::{
    Access as HttpAccess, BackupStatus, BackupTransfer, Config as HttpConfig, HttpApi,
};
use jj_store_model::RepositoryId;
use jj_store_rpc::server::{Limits, NativeRpc};
use jj_store_service::auth::BearerToken;
use serde::Serialize;
use std::{
    fs::{self, File},
    future::Future,
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use tonic::{service::Routes, transport::Server};

pub struct PreparedServer {
    rpc: PreparedRpc,
    http: HttpApi,
    browser: Option<jj_store_http::assets::BrowserAssets>,
    development_browser_annotations: bool,
    listener: tokio::net::TcpListener,
    tls: Option<tokio_rustls::TlsAcceptor>,
    concurrent: usize,
    timeout: Duration,
    search: crate::search::PreparedSearch,
    replication: crate::replication::PreparedReplication,
    storage: AuthoritativeStorage,
    _state_lock: Option<File>,
}
enum PreparedRpc {
    Shared(NativeRpc),
    Local(LocalRpc),
}
struct LocalRpc {
    service: NativeRpc,
    listener: tokio::net::UnixListener,
    socket: SocketGuard,
}
struct SocketGuard(std::path::PathBuf);
impl Drop for SocketGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}
impl PreparedServer {
    pub async fn prepare(config: &Config, require_loopback: bool) -> Result<Self> {
        Self::prepare_with_workspaces(config, require_loopback, None).await
    }
    pub async fn prepare_with_workspaces(
        config: &Config,
        require_loopback: bool,
        verifier: Option<Arc<dyn jj_store_service::local_management::WorkspaceVerifier>>,
    ) -> Result<Self> {
        Self::prepare_with_local_management(config, require_loopback, verifier, None).await
    }
    pub async fn prepare_with_local_management(
        config: &Config,
        require_loopback: bool,
        verifier: Option<Arc<dyn jj_store_service::local_management::WorkspaceVerifier>>,
        picker: Option<Arc<dyn jj_store_service::local_management::FolderPicker>>,
    ) -> Result<Self> {
        Self::prepare_with_settings(config, require_loopback, verifier, picker, None).await
    }
    pub async fn prepare_with_settings(
        config: &Config,
        require_loopback: bool,
        verifier: Option<Arc<dyn jj_store_service::local_management::WorkspaceVerifier>>,
        picker: Option<Arc<dyn jj_store_service::local_management::FolderPicker>>,
        config_path: Option<&std::path::Path>,
    ) -> Result<Self> {
        config.validate()?;
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
        let state_lock = if matches!(config.storage, crate::config::Storage::ExternalS3 { .. }) {
            fs::create_dir_all(&state)?;
            Some(policy::exclusive_lock(&state.join("server.lock"))?)
        } else {
            None
        };
        let storage = AuthoritativeStorage::start(config).await?;
        let host = config.host(storage.store())?;
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
        let registry = Arc::new(Registry::new(
            catalog.clone(),
            Identity::load(installation)?,
        )?);
        let rpc_limits = Limits {
            concurrent: config.limits.concurrent_requests,
            timeout: Duration::from_secs(config.limits.request_seconds),
            object_bytes: config.limits.max_blob_bytes
                + jj_store_model::envelope::HEADER_LEN as u64,
        };
        let timeout = Duration::from_secs(config.limits.request_seconds);
        let search_access = match &config.access {
            Access::Authenticated { .. } => crate::search::SearchAccess::Authenticated,
            Access::LocalTrusted { installation, .. } => crate::search::SearchAccess::LocalTrusted(
                Identity::load(installation)?.principal(b"search-index"),
            ),
        };
        let (search, indexes) = crate::search::PreparedSearch::new(
            &host,
            &state,
            &config.search,
            &config.limits,
            search_access,
            crate::search::environment,
        )?;
        let search_health = indexes
            .iter()
            .map(|(repository, index)| (*repository, index.health.clone()))
            .collect();
        let replication_principal = Identity::load(installation)?.principal(b"replication-worker");
        let replication = crate::replication::PreparedReplication::new(
            &host,
            storage.store(),
            &config.replication,
            replication_principal,
            search_health,
        )?;
        let rpc = match &config.access {
            Access::Authenticated { .. } => PreparedRpc::Shared(
                NativeRpc::hosted(host.clone(), rpc_limits)
                    .map_err(|_| Error::Config("RPC limits"))?
                    .with_registry(registry.clone())
                    .with_replication_controls(replication.controls()),
            ),
            Access::LocalTrusted {
                installation,
                rpc_socket,
                permission,
                ..
            } => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::{
                        FileTypeExt as _, MetadataExt as _, PermissionsExt as _,
                    };
                    let owner_uid = fs::metadata(&state)?.uid();
                    let identity = Identity::load(installation)?;
                    if let Ok(metadata) = fs::symlink_metadata(rpc_socket) {
                        if !metadata.file_type().is_socket() {
                            return Err(Error::Config("local RPC path must be a Unix socket"));
                        }
                        fs::remove_file(rpc_socket)?;
                    }
                    let listener = tokio::net::UnixListener::bind(rpc_socket)?;
                    fs::set_permissions(rpc_socket, fs::Permissions::from_mode(0o600))?;
                    let principal = identity.principal(format!("unix-uid:{owner_uid}").as_bytes());
                    PreparedRpc::Local(LocalRpc {
                        service: NativeRpc::hosted_local_trusted(
                            host.clone(),
                            rpc_limits,
                            principal,
                            *permission,
                            owner_uid,
                        )
                        .map_err(|_| Error::Config("local RPC access"))?
                        .with_registry(registry.clone())
                        .with_replication_controls(replication.controls()),
                        listener,
                        socket: SocketGuard(rpc_socket.clone()),
                    })
                }
                #[cfg(not(unix))]
                return Err(Error::Config("local trusted RPC requires Unix"));
            }
        };
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
        let local_workspaces = match (&config.access, verifier) {
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
        ) {
            config_path
                .map(|path| {
                    crate::search_management::Manager::new(
                        path,
                        config,
                        host.clone(),
                        search.control(),
                        search_access,
                        replication.search_observer(),
                    )
                })
                .transpose()?
                .map(Arc::new)
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
                    Arc::new(crate::sync_management::LocalSync(replication.controls()))
                        as Arc<dyn jj_store_service::sync_management::SyncManager>
                }),
                local_folder_picker: local_workspaces.as_ref().and(picker),
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
        Ok(Self {
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
            storage,
            _state_lock: state_lock,
        })
    }
    pub fn address(&self) -> Result<SocketAddr> {
        Ok(self.listener.local_addr()?)
    }
    pub async fn run(self, shutdown: impl Future<Output = ()> + Send + 'static) -> Result<()> {
        let search = self.search.start().await;
        let replication = self.replication.start();
        let (tx, rx) = tokio::sync::oneshot::channel();
        let (mut router, local_rpc) = match self.rpc {
            PreparedRpc::Shared(rpc) => (
                Routes::new(rpc.service())
                    .into_axum_router()
                    .merge(self.http.router())
                    .fallback(unknown_route),
                None,
            ),
            PreparedRpc::Local(rpc) => (self.http.router().fallback(unknown_route), Some(rpc)),
        };
        if let Some(browser) = self.browser {
            router = router.merge(if self.development_browser_annotations {
                browser.router_for_development_annotations()
            } else {
                browser.router()
            });
        }
        let incoming =
            crate::listener::incoming(self.listener, self.tls, self.concurrent * 4, self.timeout);
        let serve = Server::builder()
            .accept_http1(true)
            .max_concurrent_streams(self.concurrent as u32)
            .concurrency_limit_per_connection(self.concurrent)
            .http2_max_header_list_size(16 * 1024)
            .max_connection_age(Duration::from_secs(300))
            .max_connection_age_grace(self.timeout)
            .add_routes(Routes::from(router))
            .serve_with_incoming_shutdown(incoming, async {
                let _ = rx.await;
            });
        let result = if let Some(local_rpc) = local_rpc {
            run_with_local_rpc(serve, local_rpc, shutdown, tx).await
        } else {
            tokio::pin!(serve);
            tokio::select! {
                result=&mut serve=>result.map_err(|_|Error::Service("native listener failed")),
                _=shutdown=>{
                    let _=tx.send(());
                    tokio::time::timeout(Duration::from_secs(5),serve).await
                        .map_err(|_|Error::Service("graceful shutdown exceeded five seconds; in-flight outcomes may be unknown"))
                        .and_then(|result|result.map_err(|_|Error::Service("native listener shutdown failed")))
                }
            }
        };
        let replication = replication.shutdown().await;
        let stopped = search.shutdown().await;
        let storage = self.storage.shutdown().await;
        result.and(replication).and(stopped).and(storage)
    }
}

async fn run_with_local_rpc<H, S>(
    http: H,
    local: LocalRpc,
    shutdown: S,
    stop_http: tokio::sync::oneshot::Sender<()>,
) -> Result<()>
where
    H: Future<Output = std::result::Result<(), tonic::transport::Error>> + Send,
    S: Future<Output = ()> + Send,
{
    let LocalRpc {
        service,
        listener,
        socket: _socket,
    } = local;
    let (stop_rpc, stopped_rpc) = tokio::sync::oneshot::channel();
    let rpc = Server::builder()
        .add_service(service.service())
        .serve_with_incoming_shutdown(
            tokio_stream::wrappers::UnixListenerStream::new(listener),
            async {
                let _ = stopped_rpc.await;
            },
        );
    tokio::pin!(http);
    tokio::pin!(rpc);
    let finish = |result: std::result::Result<(), tonic::transport::Error>| {
        result.map_err(|_| Error::Service("native listener failed"))
    };
    tokio::select! {
        result = &mut http => {
            let _ = stop_rpc.send(());
            let _ = tokio::time::timeout(Duration::from_secs(5), &mut rpc).await;
            finish(result)
        },
        result = &mut rpc => {
            let _ = stop_http.send(());
            let _ = tokio::time::timeout(Duration::from_secs(5), &mut http).await;
            finish(result)
        },
        _ = shutdown => {
            let _ = stop_http.send(());
            let _ = stop_rpc.send(());
            tokio::time::timeout(Duration::from_secs(5), async {
                finish((&mut http).await)?;
                finish((&mut rpc).await)
            }).await.map_err(|_| Error::Service("graceful shutdown exceeded five seconds; in-flight outcomes may be unknown"))?
        },
    }
}
/// Install handlers synchronously, before the caller announces readiness.
pub fn shutdown_signal() -> std::io::Result<impl Future<Output = ()> + Send> {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        let mut terminate = signal(SignalKind::terminate())?;
        let mut interrupt = signal(SignalKind::interrupt())?;
        Ok(async move {
            tokio::select! {_=interrupt.recv()=>{},_=terminate.recv()=>{}}
        })
    }
    #[cfg(not(unix))]
    {
        Ok(async {
            let _ = tokio::signal::ctrl_c().await;
        })
    }
}
#[derive(Serialize)]
pub struct Diagnosis {
    pub version: u32,
    pub repository: RepositoryId,
    pub storage_ready: bool,
    pub heads: std::collections::BTreeSet<jj_store_model::OperationId>,
    pub adapters: &'static str,
    pub search: crate::search::Diagnosis,
}
pub async fn doctor(
    config: &Config,
    repository: RepositoryId,
    token: BearerToken,
) -> Result<Diagnosis> {
    PolicyFile::new(config.operator_policy.clone())?.read()?;
    let storage = AuthoritativeStorage::start(config).await?;
    let result = async {
        let session = config
            .host(storage.store())?
            .endpoint(repository)
            .map_err(|_| Error::Service("native endpoint"))?
            .bind(token.clone())
            .map_err(|_| Error::Service("native authorization"))?;
        let heads = tokio::time::timeout(
            Duration::from_secs(config.limits.request_seconds),
            session.heads(),
        )
        .await
        .map_err(|_| Error::Service("storage readiness deadline"))?
        .map_err(|_| Error::Service("repository access or authoritative storage unavailable"))?;
        let search = crate::search::diagnose(config, repository, &token).await;
        Ok(Diagnosis {
            version: 1,
            repository,
            storage_ready: true,
            heads,
            adapters: if search.configured {
                "search configured"
            } else {
                "not configured"
            },
            search,
        })
    }
    .await;
    let stopped = storage.shutdown().await;
    stopped.and(result)
}

// Tonic's default fallback uses HTTP 200 for gRPC errors. Ordinary HTTP
// clients must receive a real 404, including when browser assets are enabled.
async fn unknown_route(
    method: axum::http::Method,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    use axum::{
        body::Body,
        http::{StatusCode, header},
        response::IntoResponse,
    };
    let grpc = headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(';').next())
        .is_some_and(|value| value == "application/grpc" || value.starts_with("application/grpc+"));
    if method == axum::http::Method::POST && grpc {
        tonic::Status::unimplemented("").into_http::<Body>()
    } else {
        (
            StatusCode::NOT_FOUND,
            [
                (header::CACHE_CONTROL, "no-store"),
                (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
            ],
        )
            .into_response()
    }
}
