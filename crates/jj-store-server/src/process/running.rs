use super::*;

impl PreparedServer {
    pub async fn run(self, shutdown: impl Future<Output = ()> + Send + 'static) -> Result<()> {
        use jj_store_service::agent_execution::Manager as _;
        let publication = self.agents.as_ref().map(|a| a.start_publication());
        let agents = self.agents.clone();
        let shutdown = async move {
            shutdown.await;
            if let Some(agents) = agents {
                let _ = agents.shutdown().await;
            }
        };
        let mut capture = self.capture.start();
        let search = if let Some(search) = self.search {
            Some(search.start().await)
        } else {
            None
        };
        let replication = self.replication.map(|r| r.start());
        let (tx, rx) = tokio::sync::oneshot::channel();
        let (mut router, local_rpc) = match self.rpc {
            PreparedRpc::Disabled => (self.http.router().fallback(unknown_route), None),
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
        let agents_stopped = if let Some(agents) = self.agents {
            agents
                .shutdown()
                .await
                .map_err(|_| Error::Service("agent execution shutdown outcome unknown"))
        } else {
            Ok(())
        };
        if let Some(publication) = publication {
            publication.abort();
        }
        let capture_stopped = capture.shutdown().await;
        let replication = if let Some(replication) = replication {
            replication.shutdown().await
        } else {
            Ok(())
        };
        let stopped = if let Some(search) = search {
            search.shutdown().await
        } else {
            Ok(())
        };
        let storage = if let Some(storage) = self.storage {
            storage.shutdown().await
        } else {
            Ok(())
        };
        result
            .and(agents_stopped)
            .and(capture_stopped)
            .and(replication)
            .and(stopped)
            .and(storage)
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
