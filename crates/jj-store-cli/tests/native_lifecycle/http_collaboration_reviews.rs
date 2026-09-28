use super::*;
use axum::{
    body::{Body, to_bytes},
    http::{Request as HttpRequest, StatusCode},
};
use jj_review::{
    ArtifactAnchorV1, ArtifactTargetV1, CreatedContextV1, ReviewBodyFormatV1,
    ReviewCommandResultV1, ReviewCommandV1, ReviewCommentBodyV1, SensitivityV1,
};
use jj_store_http::{Access, Config, HttpApi};
use jj_store_service::administration::{OperatorPolicies, RepositoryHost};
use serde_json::{Value, json as value};
use tower::ServiceExt as _;

struct NoReviewOperators;

#[async_trait::async_trait]
impl OperatorPolicies for NoReviewOperators {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        panic!("LocalTrusted review API accessed operator policy")
    }
}

fn body(text: &str) -> ReviewCommentBodyV1 {
    ReviewCommentBodyV1 {
        schema_version: 1,
        format: ReviewBodyFormatV1::SafeMarkdown,
        text: text.into(),
        sanitizer_version: "review-markdown-v1".into(),
    }
}

fn command(seed: u8, command: ReviewCommandV1) -> Value {
    value!({"version": 1, "request_id": m::RequestId::new([seed; 16]), "command": command})
}

fn create(seed: u8, text: &str) -> Value {
    command(
        seed,
        ReviewCommandV1::CreateThread {
            anchor: Box::new(ArtifactAnchorV1 {
                schema_version: 1,
                repository_id: REPO,
                target: ArtifactTargetV1::Repository,
                created_context: CreatedContextV1 {
                    publication_position: None,
                    code_state_version: None,
                    agent_root: None,
                },
                sensitivity: SensitivityV1::Repository,
            }),
            body: body(text),
        },
    )
}

fn request(path: &str, body: Option<&Value>) -> HttpRequest<Body> {
    let mut request = HttpRequest::builder()
        .uri(format!("/v1/repos/{REPO}/{path}"))
        .header("host", "127.0.0.1:7443")
        .header("x-jj-store-protocol", "1");
    let body = if let Some(body) = body {
        request = request
            .method("POST")
            .header("content-type", "application/json");
        Body::from(serde_json::to_vec(body).unwrap())
    } else {
        Body::empty()
    };
    request.body(body).unwrap()
}

async fn json(router: &axum::Router, path: &str, body: Option<&Value>) -> (StatusCode, Value) {
    let response = router.clone().oneshot(request(path, body)).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (status, serde_json::from_slice(&bytes).unwrap())
}

fn api(fixture: &Fixture, cache: &std::path::Path, permission: Permission) -> axum::Router {
    let host = RepositoryHost::new(
        fixture.store.clone(),
        Arc::new(NoReviewOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap();
    HttpApi::new(
        host,
        cache.to_owned(),
        Config {
            access: Access::LocalTrusted {
                principal: ACTOR,
                permission,
            },
            ..Config::default()
        },
    )
    .unwrap()
    .router()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn local_trusted_http_pages_root_pinned_review_threads_and_resolves_detail() {
    let fixture = Fixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(&fixture, cache.path(), Permission::Admin);
    let unknown_id = m::RequestId::new([120; 16]);
    let (status, unknown) = json(&router, &format!("reviews/commands/{unknown_id}"), None).await;
    assert_eq!(status, StatusCode::OK, "{unknown}");
    assert_eq!(unknown["status"], "not_known");

    let first_request = create(121, "Initial review");
    let (status, accepted) = json(&router, "reviews/commands", Some(&first_request)).await;
    assert_eq!(status, StatusCode::OK, "{accepted}");
    assert_eq!(accepted["status"], "committed");
    let first: ReviewCommandResultV1 =
        serde_json::from_value(accepted["receipt"]["result"].clone()).unwrap();
    let second_request = create(123, "Second thread");
    assert_eq!(
        json(&router, "reviews/commands", Some(&second_request))
            .await
            .0,
        StatusCode::OK
    );

    let (status, first_page) = json(&router, "reviews/threads?limit=1", None).await;
    assert_eq!(status, StatusCode::OK, "{first_page}");
    let cursor = first_page["next_cursor"].as_str().unwrap();
    let reply_request = command(
        125,
        ReviewCommandV1::AppendReply {
            thread_id: first.thread_id,
            expected_workflow_version: 1,
            parent_comment_id: first.comment_id,
            body: body("Follow-up"),
        },
    );
    let (status, reply) = json(&router, "reviews/commands", Some(&reply_request)).await;
    assert_eq!(status, StatusCode::OK, "{reply}");
    let reply: ReviewCommandResultV1 =
        serde_json::from_value(reply["receipt"]["result"].clone()).unwrap();

    let (status, second_page) = json(
        &router,
        &format!("reviews/threads?limit=1&cursor={cursor}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{second_page}");
    assert_eq!(second_page["snapshot"], first_page["snapshot"]);
    let (status, detail) = json(
        &router,
        &format!("reviews/threads/{}", first.thread_id),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{detail}");
    assert_eq!(
        detail["thread"]["snapshot_id"],
        reply.thread_snapshot.id.to_string()
    );
    assert_eq!(detail["comments"].as_array().unwrap().len(), 2);
    assert_eq!(detail["events"].as_array().unwrap().len(), 2);

    let (status, missing) = json(
        &router,
        &format!(
            "reviews/threads/{}",
            jj_review::ReviewThreadId::new([255; 16])
        ),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{missing}");

    let restarted = api(&fixture, cache.path(), Permission::Admin);
    let request_id = first_request["request_id"].as_str().unwrap();
    let (status, durable) = json(&restarted, &format!("reviews/commands/{request_id}"), None).await;
    assert_eq!(status, StatusCode::OK, "{durable}");
    assert_eq!(durable, accepted);
    let (status, replay) = json(&restarted, "reviews/commands", Some(&first_request)).await;
    assert_eq!(status, StatusCode::OK, "{replay}");
    assert_eq!(replay, accepted);
    let mut changed = first_request.clone();
    changed["command"]["body"]["text"] = value!("Changed retry");
    let (status, conflict) = json(&restarted, "reviews/commands", Some(&changed)).await;
    assert_eq!(status, StatusCode::CONFLICT, "{conflict}");
    assert_eq!(conflict["code"], "intent_conflict");
    let stale = command(
        127,
        ReviewCommandV1::AppendReply {
            thread_id: first.thread_id,
            expected_workflow_version: 999,
            parent_comment_id: first.comment_id,
            body: body("Stale reply"),
        },
    );
    let (status, stale) = json(&restarted, "reviews/commands", Some(&stale)).await;
    assert_eq!(status, StatusCode::CONFLICT, "{stale}");
    assert_eq!(stale["code"], "stale_review");
    let before_denied = fixture.session.snapshot().await.unwrap().manifest().clone();
    let reader = api(&fixture, cache.path(), Permission::Read);
    assert_eq!(
        json(&reader, "reviews/commands", Some(&create(129, "Denied")))
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        *fixture.session.snapshot().await.unwrap().manifest(),
        before_denied
    );

    if let Some(path) = std::env::var_os("JJ_STORE_REVIEW_CONTRACT_CAPTURE") {
        let samples = [
            ("/v1/repos/{repo}/reviews/threads", "get", first_page),
            ("/v1/repos/{repo}/reviews/threads", "get", second_page),
            ("/v1/repos/{repo}/reviews/threads/{thread}", "get", detail),
            ("/v1/repos/{repo}/reviews/commands", "post", accepted),
            (
                "/v1/repos/{repo}/reviews/commands/{request_id}",
                "get",
                durable,
            ),
            ("/v1/repos/{repo}/reviews/commands", "post", replay),
            (
                "/v1/repos/{repo}/reviews/commands/{request_id}",
                "get",
                unknown,
            ),
        ]
        .map(|(path, method, data)| {
            serde_json::json!({
                "path": path,
                "method": method,
                "status": 200,
                "media": "application/json",
                "data": data,
            })
        });
        std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
}
