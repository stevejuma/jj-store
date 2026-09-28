use super::{browsing_acceptance::BrowsingFixture, *};
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use jj_store_http::{Config, HttpApi};
use jj_store_service::administration::{OperatorPolicies, RepositoryHost};
use serde_json::Value;
use tower::ServiceExt as _;

struct NoOperators;
#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        panic!("browser read API accessed operator policy")
    }
}
fn api(f: &BrowsingFixture, cache: &std::path::Path) -> Router {
    let host = RepositoryHost::new(
        f.f.store.clone(),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap();
    HttpApi::new(host, cache.to_owned(), Config::default())
        .unwrap()
        .router()
}
fn request(token: &BearerToken, path: &str) -> Request<Body> {
    Request::builder()
        .uri(format!("/v1/repos/{REPO}/{path}"))
        .header("host", "127.0.0.1:7443")
        .header("x-jj-store-protocol", "1")
        .header(
            "authorization",
            format!("Bearer {}", token.expose().as_str()),
        )
        .body(Body::empty())
        .unwrap()
}
async fn json(router: &Router, token: &BearerToken, path: &str) -> (StatusCode, Value) {
    let response = router.clone().oneshot(request(token, path)).await.unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), 32 * 1024 * 1024)
        .await
        .unwrap();
    (status, serde_json::from_slice(&body).unwrap())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured() {
    let f = BrowsingFixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(&f, cache.path());
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let routes = [
        (
            "references",
            format!("references?at_op={}&kind=bookmarks&limit=1", f.operation),
        ),
        (
            "commits/{commit}/paths",
            format!(
                "commits/{}/paths?at_op={}&recursive=true&limit=2",
                f.head, f.operation
            ),
        ),
        (
            "operations/{operation}/changes",
            format!(
                "operations/{}/changes?parent={}&limit=2",
                f.operation,
                roots::ROOT_OPERATION
            ),
        ),
        (
            "diff/paths",
            format!(
                "diff/paths?at_op={}&from={}&to={}&limit=1",
                f.operation, f.base, f.head
            ),
        ),
    ];
    let mut samples = vec![];
    let mut first_cursor = None;
    for (template, path) in routes {
        let (status, body) = json(&router, &f.f.reader, &path).await;
        assert_eq!(status, StatusCode::OK, "{path}: {body}");
        assert_eq!(body["snapshot"]["operation_id"], f.operation.to_string());
        if template == "references" {
            first_cursor = body["next_cursor"].as_str().map(str::to_owned);
        }
        samples.push(serde_json::json!({
            "path": format!("/v1/repos/{{repo}}/{template}"),
            "method": "get",
            "status": 200,
            "media": "application/json",
            "data": body,
        }));
    }
    let cursor = first_cursor.expect("two bookmark fixture pages");
    let (status, next) = json(
        &router,
        &f.f.reader,
        &format!(
            "references?at_op={}&kind=bookmarks&limit=1&cursor={}",
            f.operation,
            url::form_urlencoded::byte_serialize(cursor.as_bytes()).collect::<String>()
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(next["previous_cursor"].is_string());
    let (status, error) = json(
        &router,
        &f.f.reader,
        &format!(
            "references?at_op={}&kind=bookmarks&q=changed&limit=1&cursor={}",
            f.operation,
            url::form_urlencoded::byte_serialize(cursor.as_bytes()).collect::<String>()
        ),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(error["code"], "invalid_cursor");
    let unauthorized = router
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/v1/repos/{REPO}/references"))
                .header("host", "127.0.0.1:7443")
                .header("x-jj-store-protocol", "1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
    if let Some(path) = std::env::var_os("JJ_STORE_BROWSING_CONTRACT_CAPTURE") {
        std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
}
