use axum::{
    Router,
    body::Body,
    http::{Response, StatusCode},
};
use jj_store_model::{EpochId, RepositoryId};
use jj_store_server::{
    config::{Config, EXAMPLE},
    search,
};
use jj_store_service::auth::BearerToken;
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
const REPO: RepositoryId = RepositoryId::new([81; 16]);
struct Server(tokio::task::JoinHandle<()>);
impl Drop for Server {
    fn drop(&mut self) {
        self.0.abort();
    }
}
async fn start(router: Router) -> (String, Server) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    (url, Server(task))
}
fn config(root: &std::path::Path, url: &str) -> Config {
    let path = root.join("server.toml");
    std::fs::write(&path, EXAMPLE).unwrap();
    let mut config = Config::load(&path).unwrap();
    config.public_url = url.into();
    config.search = serde_json::from_value(
        json!([{"repository":REPO,"token_environment":"JJ_STORE_SEARCH_TOKEN_FIXTURE"}]),
    )
    .unwrap();
    config
}
#[tokio::test]
async fn search_diagnostics_bound_streams_and_reject_foreign_or_invalid_status() {
    let root = tempfile::tempdir().unwrap();
    let valid = json!({"version":1,"repository":REPO,"epoch":EpochId::new([82;16]),"repository_position":1,"durable_position":1,"indexed_position":1,"index_generation":jj_store_model::RecordDigest::new([84;32]),"is_stale":false,"degraded":false});
    let mut foreign = valid.clone();
    foreign["repository"] = json!(RepositoryId::new([83; 16]));
    let mut ahead = valid.clone();
    ahead["indexed_position"] = json!(2);
    for value in [
        foreign.to_string(),
        ahead.to_string(),
        "not JSON".into(),
        "x".repeat(20000),
    ] {
        let router = Router::new().fallback(move || {
            let value = value.clone();
            async move {
                // No Content-Length: exercise streamed response admission.
                Body::from_stream(futures::stream::iter([Ok::<_, std::io::Error>(value)]))
            }
        });
        let (url, _server) = start(router).await;
        let diagnosis = search::diagnose(
            &config(root.path(), &url),
            REPO,
            &BearerToken::generate().unwrap(),
        )
        .await;
        assert_eq!(diagnosis.state, "unavailable");
        assert!(diagnosis.status.is_none());
    }
}
#[tokio::test]
async fn search_diagnostics_do_not_forward_credentials_through_redirects() {
    let count = Arc::new(AtomicUsize::new(0));
    let received = count.clone();
    let (target, _target) = start(Router::new().fallback(move || {
        let received = received.clone();
        async move {
            received.fetch_add(1, Ordering::SeqCst);
            StatusCode::OK
        }
    }))
    .await;
    let (url, _source) = start(Router::new().fallback(move || {
        let target = target.clone();
        async move {
            Response::builder()
                .status(StatusCode::TEMPORARY_REDIRECT)
                .header("location", target)
                .body(Body::empty())
                .unwrap()
        }
    }))
    .await;
    let root = tempfile::tempdir().unwrap();
    let diagnosis = search::diagnose(
        &config(root.path(), &url),
        REPO,
        &BearerToken::generate().unwrap(),
    )
    .await;
    assert_eq!(diagnosis.state, "unavailable");
    assert!(diagnosis.status.is_none());
    assert_eq!(count.load(Ordering::SeqCst), 0);
}
