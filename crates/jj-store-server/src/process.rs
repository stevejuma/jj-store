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

mod authority;
mod capture;
mod preparation;
mod running;

pub struct PreparedServer {
    rpc: PreparedRpc,
    http: HttpApi,
    browser: Option<jj_store_http::assets::BrowserAssets>,
    development_browser_annotations: bool,
    listener: tokio::net::TcpListener,
    tls: Option<tokio_rustls::TlsAcceptor>,
    concurrent: usize,
    timeout: Duration,
    search: Option<crate::search::PreparedSearch>,
    replication: Option<crate::replication::PreparedReplication>,
    capture: capture::PreparedCapture,
    agents: Option<crate::agent_execution::Supervisor>,
    storage: Option<AuthoritativeStorage>,
    _state_lock: Option<File>,
}
enum PreparedRpc {
    Disabled,
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
        Self::prepare_with_browser(
            config,
            require_loopback,
            verifier,
            picker,
            config_path,
            None,
        )
        .await
    }
    pub async fn prepare_with_browser(
        config: &Config,
        require_loopback: bool,
        verifier: Option<Arc<dyn jj_store_service::local_management::WorkspaceVerifier>>,
        picker: Option<Arc<dyn jj_store_service::local_management::FolderPicker>>,
        config_path: Option<&std::path::Path>,
        folders: Option<Arc<dyn jj_store_service::local_browser::FolderProvider>>,
    ) -> Result<Self> {
        preparation::prepare(
            config,
            require_loopback,
            verifier,
            picker,
            config_path,
            folders,
        )
        .await
    }
    pub fn address(&self) -> Result<SocketAddr> {
        Ok(self.listener.local_addr()?)
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

#[cfg(all(test, unix))]
mod tests;
