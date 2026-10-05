use super::*;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request as HttpRequest, StatusCode},
};
use jj_store_http::{Config as HttpConfig, HttpApi, SearchIndex};
use jj_store_service::administration::{OperatorPolicies, RepositoryHost};
use serde_json::{Value, json};
use tower::ServiceExt as _;

struct NoOperators;
#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        panic!("search read accessed operator policy")
    }
}
fn api(f: &SearchFixture, configured: bool) -> Router {
    let host = RepositoryHost::new(
        f.f.store.clone(),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap();
    let indexes = if configured {
        [(
            REPO,
            SearchIndex {
                search: f.search.clone(),
                health: f.runner.subscribe(),
                limits: Default::default(),
            },
        )]
        .into()
    } else {
        BTreeMap::new()
    };
    HttpApi::with_search(
        host,
        f.root.path().join("http"),
        HttpConfig::default(),
        indexes,
    )
    .unwrap()
    .router()
}
async fn get(router: &Router, token: &BearerToken, path: &str) -> (StatusCode, Value) {
    let response = router
        .clone()
        .oneshot(
            HttpRequest::builder()
                .uri(format!("/v1/repos/{REPO}/{path}"))
                .header("host", "127.0.0.1:7443")
                .header("x-jj-store-protocol", "1")
                .header(
                    "authorization",
                    format!("Bearer {}", token.expose().as_str()),
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.headers()["cache-control"], "no-store");
    let status = response.status();
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (status, serde_json::from_slice(&body).unwrap())
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_search_pins_results_across_writes_and_captures_contracts() {
    tokio::task::LocalSet::new().run_until(async {
        let mut f = SearchFixture::new(Default::default()).await;
        let router = api(&f, true);
        let before = f.f.session.snapshot().await.unwrap().manifest().clone();
        let mut samples = vec![];
        for path in ["capabilities", "search/status", "search?q=metafixture&page_size=1"] {
            let (status, body) = get(&router, &f.f.reader, path).await;
            assert_eq!(status, StatusCode::OK, "{path}: {body}");
            if path == "capabilities" { assert_eq!(body["search"], true); }
            samples.push(json!({"path":format!("/v1/repos/{{repo}}/{}",path.split('?').next().unwrap()), "method":"get", "status":200, "media":"application/json", "data":body}));
        }
        assert_eq!(before, *f.f.session.snapshot().await.unwrap().manifest());
        let first = samples.last().unwrap()["data"].clone();
        let cursor = first["next_cursor"].as_str().unwrap();
        publish_text_scope(&f.f, roots::ROOT_COMMIT, 3, &[]).await;
        f.runner.step(a::CancellationToken::new()).await.unwrap();
        let path = format!("search?q=metafixture&page_size=1&cursor={cursor}");
        let (status, second) = get(&router, &f.f.reader, &path).await;
        assert_eq!(status, StatusCode::OK, "{second}");
        assert_eq!(second["index_generation"], first["index_generation"]);
        assert_eq!(second["selected_view"], first["selected_view"]);
        assert_ne!(second["results"][0]["commit"], first["results"][0]["commit"]);
        assert_eq!(second["is_stale"], true);
        let (status, lag) = get(&router, &f.f.reader, "search?q=metafixture&minimum_position=99").await;
        assert_eq!(status, StatusCode::CONFLICT, "{lag}");
        assert_eq!(lag["code"], "index_lag");
        assert_eq!(lag["search_status"]["indexed_position"], 3);
        samples.push(json!({"path":"/v1/repos/{repo}/search","method":"get","status":409,"media":"application/json","data":lag}));
        if let Some(path) = std::env::var_os("JJ_STORE_SEARCH_CONTRACT_CAPTURE") { std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap(); }
    }).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_search_rejects_bad_inputs_and_unconfigured_repositories() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = SearchFixture::new(Default::default()).await;
            let router = api(&f, true);
            let before = f.f.session.snapshot().await.unwrap().manifest().clone();
            for path in [
                "search",
                "search?q=",
                "search?q=x&typo=x",
                "search?q=x&q=y",
                "search?q=x&page_size=0",
                "search?q=x&page_size=-1",
                "search?q=x&mode=typo",
                "search?q=x&wait_millis=1",
                "search?q=x&minimum_position=1&wait_millis=5001",
                "search/status?typo=x",
                "search?q=x&scope=missing",
            ] {
                let (status, body) = get(&router, &f.f.reader, path).await;
                assert_eq!(status, StatusCode::BAD_REQUEST, "{path}: {body}");
                assert_eq!(body["code"], "invalid_request");
            }
            let (_, first) = get(&router, &f.f.reader, "search?q=metafixture&page_size=1").await;
            let cursor = first["next_cursor"].as_str().unwrap();
            let (status, changed) = get(
                &router,
                &f.f.reader,
                &format!("search?q=changed&page_size=1&cursor={cursor}"),
            )
            .await;
            assert_eq!(status, StatusCode::BAD_REQUEST);
            assert_eq!(changed["code"], "invalid_cursor");
            let (status, reset) = get(
                &api(&f, true),
                &f.f.reader,
                &format!("search?q=metafixture&page_size=1&cursor={cursor}"),
            )
            .await;
            assert_eq!(status, StatusCode::CONFLICT);
            assert_eq!(reset["code"], "cursor_reset");
            let router = api(&f, false);
            assert_eq!(
                get(&router, &f.f.reader, "capabilities").await.1["search"],
                false
            );
            for path in ["search?q=x", "search/status"] {
                let (status, body) = get(&router, &f.f.reader, path).await;
                assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
                assert_eq!(body["code"], "search_not_configured");
            }
            assert_eq!(before, *f.f.session.snapshot().await.unwrap().manifest());
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_search_revocation_and_repository_isolation_precede_query_details() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = SearchFixture::new(Default::default()).await;
            let router = api(&f, true);
            for path in ["search?q=metafixture", "search/status", "capabilities"] {
                let response = router
                    .clone()
                    .oneshot(
                        HttpRequest::builder()
                            .uri(format!("/v1/repos/{REPO}/{path}"))
                            .header("host", "127.0.0.1:7443")
                            .header("x-jj-store-protocol", "1")
                            .body(Body::empty())
                            .unwrap(),
                    )
                    .await
                    .unwrap();
                assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
                let response = router
                    .clone()
                    .oneshot(
                        HttpRequest::builder()
                            .uri(format!(
                                "/v1/repos/{}/{path}",
                                m::RepositoryId::new([99; 16])
                            ))
                            .header("host", "127.0.0.1:7443")
                            .header("x-jj-store-protocol", "1")
                            .header(
                                "authorization",
                                format!("Bearer {}", f.f.reader.expose().as_str()),
                            )
                            .body(Body::empty())
                            .unwrap(),
                    )
                    .await
                    .unwrap();
                assert_eq!(response.status(), StatusCode::FORBIDDEN);
            }
            let (_, first) = get(&router, &f.f.reader, "search?q=metafixture&page_size=1").await;
            let cursor = first["next_cursor"].as_str().unwrap();
            f.f.session
                .change_token(&jj_store_service::administration::TokenRequest {
                    request_id: m::RequestId::new([230; 16]),
                    expected: f.f.session.snapshot().await.unwrap().manifest().snapshot(),
                    change: jj_store_service::administration::TokenChange::Revoke {
                        token: f.f.reader.id(),
                    },
                })
                .await
                .unwrap();
            for path in [
                "search/status".into(),
                "search?typo=x".into(),
                format!("search?q=metafixture&page_size=1&cursor={cursor}"),
                "search?q=x&minimum_position=99".into(),
            ] {
                let (status, body) = get(&router, &f.f.reader, &path).await;
                assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
                assert_eq!(body, json!({"version":1,"code":"access_denied"}));
            }
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_search_reports_degradation_and_head_runs_without_publication() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let mut f = SearchFixture::new(Default::default()).await;
            let router = api(&f, true);
            publish_text_scope(&f.f, roots::ROOT_COMMIT, 3, &[]).await;
            f.faults.once(jj_store_tantivy::Fault::BeforeReader);
            assert!(matches!(
                f.runner.step(a::CancellationToken::new()).await,
                Err(a::Error::Projection)
            ));
            let before = f.f.session.snapshot().await.unwrap().manifest().clone();
            let (status, health) = get(&router, &f.f.reader, "search/status").await;
            assert_eq!(status, StatusCode::OK);
            assert_eq!(health["durable_position"], 3);
            assert_eq!(health["indexed_position"], 2);
            assert_eq!(health["degraded"], true);
            let (status, page) =
                get(&router, &f.f.reader, "search?q=metafixture&mode=historical").await;
            assert_eq!(status, StatusCode::OK, "{page}");
            assert_eq!(page["selected_view"], Value::Null);
            assert_eq!(page["degraded"], true);
            assert_eq!(page["is_stale"], true);
            let response = router
                .clone()
                .oneshot(
                    HttpRequest::builder()
                        .method("HEAD")
                        .uri(format!("/v1/repos/{REPO}/search?q=metafixture"))
                        .header("host", "127.0.0.1:7443")
                        .header("x-jj-store-protocol", "1")
                        .header(
                            "authorization",
                            format!("Bearer {}", f.f.reader.expose().as_str()),
                        )
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            assert!(to_bytes(response.into_body(), 1).await.unwrap().is_empty());
            assert_eq!(before, *f.f.session.snapshot().await.unwrap().manifest());
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_search_requires_head_selection_and_reports_missing_reader() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let mut f = SearchFixture::new(Default::default()).await;
            let current =
                *f.f.session
                    .snapshot()
                    .await
                    .unwrap()
                    .manifest()
                    .heads
                    .first()
                    .unwrap();
            let mut sibling =
                f.f.objects
                    .get::<o::Operation>(m::ObjectKey::Operation(current))
                    .await
                    .unwrap();
            let old = oid(&sibling.parents[0]);
            sibling.metadata.description = "http search sibling".into();
            let key = f.f.objects.put(&sibling).await.unwrap().0;
            f.f.session
                .publish(
                    BTreeSet::from([old]),
                    m::OperationId::from_slice(key.as_bytes()).unwrap(),
                )
                .await
                .unwrap();
            f.runner.step(a::CancellationToken::new()).await.unwrap();
            let router = api(&f, true);
            let before = f.f.session.snapshot().await.unwrap().manifest().clone();
            let (status, body) = get(&router, &f.f.reader, "search?q=metafixture").await;
            assert_eq!(status, StatusCode::CONFLICT);
            assert_eq!(body["code"], "select_operation");
            assert_eq!(body["heads"]["heads"].as_array().unwrap().len(), 2);
            let (status, body) = get(
                &router,
                &f.f.reader,
                &format!("search?q=metafixture&at_op={current}"),
            )
            .await;
            assert_eq!(status, StatusCode::OK, "{body}");
            assert_eq!(body["selected_view"]["operation_id"], current.to_string());
            let (status, body) = get(
                &router,
                &f.f.reader,
                &format!("search?q=metafixture&at_op={current}&mode=historical"),
            )
            .await;
            assert_eq!(status, StatusCode::BAD_REQUEST);
            assert_eq!(body["code"], "invalid_request");
            assert_eq!(before, *f.f.session.snapshot().await.unwrap().manifest());
            let (_projection, search) = TantivyProjection::open(
                f.root.path().join("missing"),
                NativeExtractor::new(Config::default()).unwrap(),
                Default::default(),
            )
            .unwrap();
            f.search = search;
            let router = api(&f, true);
            let (status, body) = get(&router, &f.f.reader, "search?q=metafixture").await;
            assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(body["code"], "search_not_ready");
            let (status, body) = get(&router, &f.f.reader, "search/status").await;
            assert_eq!(status, StatusCode::OK);
            assert_eq!(body["indexed_position"], Value::Null);
            assert_eq!(body["is_stale"], true);
        })
        .await;
}
