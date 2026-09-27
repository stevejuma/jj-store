use crate::{
    Error, Result,
    config::Config,
    policy::{self, PolicyFile},
};
use jj_store_http::{Config as HttpConfig, HttpApi};
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
use tonic::{service::Routes, transport::Server};

pub struct PreparedServer {
    rpc: NativeRpc,
    http: HttpApi,
    browser: Option<jj_store_http::assets::BrowserAssets>,
    listener: tokio::net::TcpListener,
    tls: Option<tokio_rustls::TlsAcceptor>,
    concurrent: usize,
    timeout: Duration,
    search: crate::search::PreparedSearch,
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
        let tls = if let Some(tls) = &config.tls {
            let certificate = policy::read_regular(&tls.certificate, 1024 * 1024)?;
            let key = policy::read_regular(&tls.private_key, 1024 * 1024)?;
            Some(crate::listener::tls(&certificate, &key)?)
        } else {
            None
        };
        let host = config.host()?;
        let rpc = NativeRpc::hosted(
            host.clone(),
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
        let timeout = Duration::from_secs(config.limits.request_seconds);
        let (search, indexes) = crate::search::PreparedSearch::new(
            &host,
            &state,
            &config.search,
            &config.limits,
            crate::search::environment,
        )?;
        let http = HttpApi::with_search(
            host,
            state.join("native-reads"),
            HttpConfig {
                origin: url::Url::parse(&config.public_url)
                    .map_err(|_| Error::Config("public URL"))?
                    .origin()
                    .ascii_serialization(),
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
            listener,
            tls,
            concurrent: config.limits.concurrent_requests,
            timeout,
            search,
            _lock: lock,
        })
    }
    pub fn address(&self) -> Result<SocketAddr> {
        Ok(self.listener.local_addr()?)
    }
    pub async fn run(self, shutdown: impl Future<Output = ()> + Send + 'static) -> Result<()> {
        let search = self.search.start();
        let (tx, rx) = tokio::sync::oneshot::channel();
        let mut router = Routes::new(self.rpc.service())
            .into_axum_router()
            .merge(self.http.router())
            .fallback(unknown_route);
        if let Some(browser) = self.browser {
            router = router.merge(browser.router());
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
        tokio::pin!(serve);
        let result = tokio::select! {
            result=&mut serve=>result.map_err(|_|Error::Service("native listener failed")),
            _=shutdown=>{
                let _=tx.send(());
                tokio::time::timeout(Duration::from_secs(5),serve).await
                    .map_err(|_|Error::Service("graceful shutdown exceeded five seconds; in-flight outcomes may be unknown"))
                    .and_then(|result|result.map_err(|_|Error::Service("native listener shutdown failed")))
            }
        };
        let stopped = search.shutdown().await;
        result.and(stopped)
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
    let session = config
        .host()?
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
