//! HTTP/1 upgrades and HTTP/2 RPC share the already-admitted TCP/TLS listener.
use super::*;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
    service::TowerToHyperService,
};
use tokio::{sync::watch, task::JoinSet};

pub(crate) async fn serve(
    incoming: impl Stream<Item = io::Result<Connection>> + Send,
    router: axum::Router,
    concurrent: usize,
    grace: Duration,
    shutdown: impl std::future::Future<Output = ()> + Send,
) -> Result<()> {
    let mut tasks = JoinSet::new();
    let (stop, _) = watch::channel(());
    // This sender outlives graceful HTTP drain. Dropping it also wakes upgraded
    // sockets, whose ownership has moved out of the HTTP connection future.
    let (socket_lifetime, _) = watch::channel(());
    futures::pin_mut!(incoming, shutdown);
    loop {
        tokio::select! {
            _ = &mut shutdown => break,
            Some(_) = tasks.join_next(), if !tasks.is_empty() => {},
            connection = incoming.next() => {
                let Some(connection) = connection else { break; };
                let Ok(connection) = connection else { continue; };
                let mut stopped = stop.subscribe();
                let mut lifetime = socket_lifetime.subscribe();
                let socket = connection.stop_with(async move { let _ = lifetime.changed().await; });
                let service = TowerToHyperService::new(tower::limit::ConcurrencyLimit::new(router.clone(), concurrent));
                tasks.spawn(async move {
                    let mut builder = Builder::new(TokioExecutor::new());
                    builder.http2().max_concurrent_streams(concurrent as u32).max_header_list_size(16 * 1024);
                    let connection = builder.serve_connection_with_upgrades(TokioIo::new(socket), service);
                    tokio::pin!(connection);
                    tokio::select! {
                        _ = &mut connection => return,
                        _ = stopped.changed() => {},
                        _ = tokio::time::sleep(Duration::from_secs(300)) => {},
                    }
                    connection.as_mut().graceful_shutdown();
                    let _ = tokio::time::timeout(grace, connection).await;
                });
            }
        }
    }
    let _ = stop.send(());
    while tasks.join_next().await.is_some() {}
    drop(socket_lifetime);
    Ok(())
}

#[cfg(test)]
mod tests;
