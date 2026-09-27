use crate::{
    Error, Result,
    config::Config,
    policy::{self, PolicyFile},
};
use jj_store_model::RepositoryId;
use jj_store_rpc::server::{Limits, NativeRpc};
use jj_store_service::auth::BearerToken;
use serde::Serialize;
use std::{
    fs::{self, File},
    future::Future,
    net::SocketAddr,
    time::Duration,
};
use tonic::transport::{Identity, Server, ServerTlsConfig};

pub struct PreparedServer {
    rpc: NativeRpc,
    listener: tokio::net::TcpListener,
    identity: Option<Identity>,
    insecure_loopback: bool,
    _lock: File,
}
impl PreparedServer {
    pub async fn prepare(config: &Config, require_loopback: bool) -> Result<Self> {
        config.validate()?;
        if require_loopback && !config.bind.ip().is_loopback() {
            return Err(Error::Config(
                "serve-local requires a literal loopback listener",
            ));
        }
        let policy = PolicyFile::new(config.operator_policy.clone())?;
        policy.read()?;
        let identity = if let Some(tls) = &config.tls {
            let certificate = policy::read_regular(&tls.certificate, 1024 * 1024)?;
            let key = policy::read_regular(&tls.private_key, 1024 * 1024)?;
            let identity = Identity::from_pem(certificate, key);
            Server::builder()
                .tls_config(ServerTlsConfig::new().identity(identity.clone()))
                .map_err(|_| Error::Config("TLS certificate or private key"))?;
            Some(identity)
        } else {
            None
        };
        let host = config.host()?;
        let rpc = NativeRpc::hosted(
            host,
            Limits {
                concurrent: config.limits.concurrent_requests,
                timeout: Duration::from_secs(config.limits.request_seconds),
                object_bytes: config.limits.max_blob_bytes
                    + jj_store_model::envelope::HEADER_LEN as u64,
            },
        )
        .map_err(|_| Error::Config("RPC limits"))?;
        let state = policy::outside_checkout(&config.state_dir)?;
        fs::create_dir_all(&state)?;
        let lock = policy::exclusive_lock(&state.join("server.lock"))?;
        let listener = tokio::net::TcpListener::bind(config.bind).await?;
        Ok(Self {
            rpc,
            listener,
            identity,
            insecure_loopback: config.insecure_loopback,
            _lock: lock,
        })
    }
    pub fn address(&self) -> Result<SocketAddr> {
        Ok(self.listener.local_addr()?)
    }
    pub async fn run(self, shutdown: impl Future<Output = ()> + Send + 'static) -> Result<()> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serve = self.rpc.serve(
            self.listener,
            self.identity,
            self.insecure_loopback,
            async {
                let _ = rx.await;
            },
        );
        tokio::pin!(serve);
        tokio::select! {
            result=&mut serve=>result.map_err(|_|Error::Service("native listener failed")),
            _=shutdown=>{
                let _=tx.send(());
                tokio::time::timeout(Duration::from_secs(5),serve).await.map_err(|_|Error::Service("graceful shutdown exceeded five seconds; in-flight outcomes may be unknown"))?.map_err(|_|Error::Service("native listener shutdown failed"))
            }
        }
    }
}
pub async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        if let Ok(mut terminate) = signal(SignalKind::terminate()) {
            tokio::select! {_=tokio::signal::ctrl_c()=>{},_=terminate.recv()=>{}}
            return;
        }
    }
    let _ = tokio::signal::ctrl_c().await;
}
#[derive(Serialize)]
pub struct Diagnosis {
    pub version: u32,
    pub repository: RepositoryId,
    pub storage_ready: bool,
    pub heads: std::collections::BTreeSet<jj_store_model::OperationId>,
    pub adapters: &'static str,
}
pub async fn doctor(
    config: &Config,
    repository: RepositoryId,
    token: BearerToken,
) -> Result<Diagnosis> {
    PolicyFile::new(config.operator_policy.clone())?.read()?;
    let session = config
        .host()?
        .endpoint(repository)
        .map_err(|_| Error::Service("native endpoint"))?
        .bind(token)
        .map_err(|_| Error::Service("native authorization"))?;
    let heads = tokio::time::timeout(
        Duration::from_secs(config.limits.request_seconds),
        session.heads(),
    )
    .await
    .map_err(|_| Error::Service("storage readiness deadline"))?
    .map_err(|_| Error::Service("repository access or authoritative storage unavailable"))?;
    Ok(Diagnosis {
        version: 1,
        repository,
        storage_ready: true,
        heads,
        adapters: "not configured",
    })
}
