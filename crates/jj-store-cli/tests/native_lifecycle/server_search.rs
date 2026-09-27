use super::*;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use jj_store_server::search::{PreparedSearch, Repository};
use jj_store_service::administration::{OperatorPolicies, RepositoryHost};
use serde_json::{Value, json};
use tower::ServiceExt as _;
struct NoOperators;
#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        panic!("search accessed operator policy")
    }
}
fn host(f: &Fixture) -> RepositoryHost {
    RepositoryHost::new(
        f.store.clone(),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap()
}
fn entries() -> Vec<Repository> {
    serde_json::from_value(json!([{"repository":REPO,"token_environment":"JJ_STORE_SEARCH_TOKEN_FIXTURE","poll_millis":10,"source":{"scopes":{"main":{"kind":"bookmark","name":"main"}}}}])).unwrap()
}
fn prepare(f: &Fixture, root: &std::path::Path) -> (PreparedSearch, Router) {
    let host = host(f);
    let (prepared, indexes) =
        PreparedSearch::new(&host, root, &entries(), &Default::default(), |_| {
            Ok(f.reader.clone())
        })
        .unwrap();
    let router =
        jj_store_http::HttpApi::with_search(host, root.join("native"), Default::default(), indexes)
            .unwrap()
            .router();
    (prepared, router)
}
async fn get(router: &Router, token: &BearerToken, path: &str) -> (StatusCode, Value) {
    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/v1/repos/{REPO}/{path}"))
                .header(
                    "authorization",
                    format!("Bearer {}", token.expose().as_str()),
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let value = serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
        .unwrap();
    (status, value)
}
async fn wait_index(router: &Router, token: &BearerToken, position: u64) -> Value {
    let (status, page) = get(
        router,
        token,
        &format!("search?q=metafixture&minimum_position={position}&wait_millis=5000"),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{page}");
    assert_eq!(page["indexed_position"], position);
    page
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn server_search_runs_recovers_and_rebuilds_after_cache_loss() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let (first, _) =
                publish_text_scope(&f, roots::ROOT_COMMIT, 1, &[("code.rs", b"source signal")])
                    .await;
            let root = tempfile::tempdir().unwrap();
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            let (prepared, router) = prepare(&f, root.path());
            assert_eq!(
                get(&router, &f.reader, "search/status").await.1["indexed_position"],
                Value::Null
            );
            let running = prepared.start();
            let initial = wait_index(&router, &f.reader, 1).await;
            assert_eq!(before, *f.session.snapshot().await.unwrap().manifest());
            publish_text_scope(&f, first, 2, &[("code.rs", b"updated signal")]).await;
            let second = wait_index(&router, &f.reader, 2).await;
            assert_ne!(initial["index_generation"], second["index_generation"]);
            let authority = f.session.snapshot().await.unwrap().manifest().clone();
            let (status, source) = get(&router, &f.reader, "search?q=updated&scope=main").await;
            assert_eq!(status, StatusCode::OK);
            assert_eq!(source["results"].as_array().unwrap().len(), 1);
            // Source terms are indexed, not echoed as stored document bodies.
            // Verify the exact file referenced by the hit through native HTTP.
            let file = source["results"][0]["file"].as_str().unwrap();
            let operation = source["selected_view"]["operation_id"].as_str().unwrap();
            let response = router
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(format!("/v1/repos/{REPO}/files/{file}?at_op={operation}"))
                        .header(
                            "authorization",
                            format!("Bearer {}", f.reader.expose().as_str()),
                        )
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                &to_bytes(response.into_body(), 128).await.unwrap()[..],
                b"updated signal"
            );
            running.shutdown().await.unwrap();
            drop(router);
            let (prepared, router) = prepare(&f, root.path());
            let running = prepared.start();
            let recovered = wait_index(&router, &f.reader, 2).await;
            assert_eq!(second["index_generation"], recovered["index_generation"]);
            assert_eq!(second["results"], recovered["results"]);
            running.shutdown().await.unwrap();
            drop(router);
            std::fs::remove_dir_all(root.path().join("search")).unwrap();
            let (prepared, router) = prepare(&f, root.path());
            let running = prepared.start();
            let rebuilt = wait_index(&router, &f.reader, 2).await;
            assert_ne!(rebuilt["index_generation"], recovered["index_generation"]);
            // Equal-score ties may use different document addresses after a
            // rebuild. Compare every document field independent of tie order;
            // reopening the exact generation above retains exact page order.
            let rows = |page: &Value| {
                let mut rows = page["results"].as_array().unwrap().clone();
                rows.sort_by_key(|row| row["key"].to_string());
                rows
            };
            assert_eq!(rows(&rebuilt), rows(&recovered));
            assert_eq!(rebuilt["scope_snapshots"], recovered["scope_snapshots"]);
            assert_eq!(authority, *f.session.snapshot().await.unwrap().manifest());
            running.shutdown().await.unwrap();
            drop(router);
        })
        .await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn server_search_revoked_worker_degrades_without_revoking_other_readers() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            publish_text_scope(&f, roots::ROOT_COMMIT, 1, &[]).await;
            let root = tempfile::tempdir().unwrap();
            let (prepared, router) = prepare(&f, root.path());
            let running = prepared.start();
            wait_index(&router, &f.reader, 1).await;
            f.session
                .change_token(&jj_store_service::administration::TokenRequest {
                    request_id: m::RequestId::new([233; 16]),
                    expected: f.session.snapshot().await.unwrap().manifest().snapshot(),
                    change: jj_store_service::administration::TokenChange::Revoke {
                        token: f.reader.id(),
                    },
                })
                .await
                .unwrap();
            let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
            loop {
                let (status, health) = get(&router, &f.token, "search/status").await;
                assert_eq!(status, StatusCode::OK, "{health}");
                if health["degraded"] == true {
                    break;
                }
                assert!(
                    tokio::time::Instant::now() < deadline,
                    "worker did not observe revocation"
                );
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            assert_eq!(
                get(&router, &f.reader, "search?q=metafixture").await.0,
                StatusCode::FORBIDDEN
            );
            assert_eq!(get(&router, &f.token, "snapshot").await.0, StatusCode::OK);
            running.shutdown().await.unwrap();
        })
        .await;
}

struct Listener(tokio::task::JoinHandle<()>);
impl Drop for Listener {
    fn drop(&mut self) {
        self.0.abort();
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn server_search_diagnostics_use_authorized_live_status_and_handle_unavailability() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            publish_text_scope(&f, roots::ROOT_COMMIT, 1, &[]).await;
            let root = tempfile::tempdir().unwrap();
            let (prepared, router) = prepare(&f, root.path());
            let running = prepared.start();
            wait_index(&router, &f.reader, 1).await;
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let server = Listener(tokio::spawn(async move {
                axum::serve(listener, router).await.unwrap();
            }));
            let path = root.path().join("server.toml");
            std::fs::write(&path, jj_store_server::config::EXAMPLE).unwrap();
            let mut config = jj_store_server::config::Config::load(&path).unwrap();
            config.public_url = format!("http://{address}/");
            config.bind = address;
            config.search = entries();
            let ready = jj_store_server::search::diagnose(&config, REPO, &f.reader).await;
            assert_eq!(ready.state, "ready");
            assert_eq!(
                ready.status.unwrap().indexed_position,
                Some(m::WalPosition(1))
            );
            let denied =
                jj_store_server::search::diagnose(&config, REPO, &BearerToken::generate().unwrap())
                    .await;
            assert_eq!(denied.state, "access_denied");
            assert!(denied.status.is_none());
            drop(server);
            let unavailable = jj_store_server::search::diagnose(&config, REPO, &f.reader).await;
            assert_eq!(unavailable.state, "unavailable");
            assert!(unavailable.status.is_none());
            config.search.clear();
            let none = jj_store_server::search::diagnose(&config, REPO, &f.reader).await;
            assert_eq!(none.state, "not_configured");
            assert!(!none.configured);
            running.shutdown().await.unwrap();
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn server_search_drop_cancels_workers_and_releases_owned_state() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            publish_text_scope(&f, roots::ROOT_COMMIT, 1, &[]).await;
            let root = tempfile::tempdir().unwrap();
            let (prepared, router) = prepare(&f, root.path());
            let running = prepared.start();
            wait_index(&router, &f.reader, 1).await;
            drop(running);
            let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
            loop {
                let (status, health) = get(&router, &f.reader, "search/status").await;
                assert_eq!(status, StatusCode::OK);
                if health["degraded"] == true {
                    break;
                }
                assert!(
                    tokio::time::Instant::now() < deadline,
                    "a stopped runner was reported healthy"
                );
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            drop(router);
            loop {
                if let Ok((prepared, _indexes)) = PreparedSearch::new(
                    &host(&f),
                    root.path(),
                    &entries(),
                    &Default::default(),
                    |_| Ok(f.reader.clone()),
                ) {
                    drop(prepared);
                    break;
                }
                assert!(
                    tokio::time::Instant::now() < deadline,
                    "dropped runtime retained index ownership"
                );
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await;
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn server_search_rejects_state_symlink_escape_and_invalid_programmatic_limits() {
    let f = Fixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let other = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(other.path(), root.path().join("search")).unwrap();
    assert!(
        PreparedSearch::new(
            &host(&f),
            root.path(),
            &entries(),
            &Default::default(),
            |_| Ok(f.reader.clone())
        )
        .is_err()
    );
    assert!(std::fs::read_dir(other.path()).unwrap().next().is_none());
    let root = tempfile::tempdir().unwrap();
    assert!(
        PreparedSearch::new(
            &host(&f),
            root.path(),
            &entries(),
            &jj_store_server::config::Limits {
                request_seconds: 0,
                ..Default::default()
            },
            |_| Ok(f.reader.clone())
        )
        .is_err()
    );
    assert!(std::fs::read_dir(root.path()).unwrap().next().is_none());
}
