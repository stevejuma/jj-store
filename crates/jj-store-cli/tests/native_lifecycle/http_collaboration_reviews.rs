use super::*;
use axum::{
    body::{Body, to_bytes},
    http::{Request as HttpRequest, StatusCode},
};
use jj_review::{
    ArtifactAnchorV1, ArtifactTargetV1, CreatedContextV1, ReviewBodyFormatV1,
    ReviewCommandResultV1, ReviewCommandV1, ReviewCommentBodyV1, SensitivityV1,
    decode_command_result,
};
use jj_store_http::{Access, Config, HttpApi};
use jj_store_service::administration::{OperatorPolicies, RepositoryHost};
use serde_json::Value;
use tower::ServiceExt as _;

struct NoReviewOperators;

#[async_trait::async_trait]
impl OperatorPolicies for NoReviewOperators {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        panic!("LocalTrusted review read accessed operator policy")
    }
}

struct AllowReview;

#[async_trait::async_trait]
impl PublicationGuard for AllowReview {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }

    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }

    async fn authorize_domain(&self, _: &ManifestSnapshot, _: &DomainRequest) -> wal::Result<()> {
        Ok(())
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

async fn publish(
    coordinator: &Coordinator,
    seed: u8,
    command: ReviewCommandV1,
) -> ReviewCommandResultV1 {
    let request = ReviewCommandRequest::new(
        ACTOR,
        m::RequestId::new([seed; 16]),
        m::PublicationId::new([seed + 1; 16]),
        command,
    )
    .unwrap();
    let PublishOutcome::Committed(receipt) =
        coordinator.publish_review_command(&request).await.unwrap()
    else {
        panic!("review command did not commit")
    };
    decode_command_result(&receipt.result).unwrap()
}

async fn create(coordinator: &Coordinator, seed: u8, text: &str) -> ReviewCommandResultV1 {
    publish(
        coordinator,
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
    .await
}

fn request(path: &str) -> HttpRequest<Body> {
    HttpRequest::builder()
        .uri(format!("/v1/repos/{REPO}/{path}"))
        .header("host", "127.0.0.1:7443")
        .header("x-jj-store-protocol", "1")
        .body(Body::empty())
        .unwrap()
}

async fn json(router: &axum::Router, path: &str) -> (StatusCode, Value) {
    let response = router.clone().oneshot(request(path)).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (status, serde_json::from_slice(&bytes).unwrap())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn local_trusted_http_pages_root_pinned_review_threads_and_resolves_detail() {
    let fixture = Fixture::new().await;
    let coordinator = Coordinator::new(
        fixture.store.clone(),
        REPO,
        fixture.objects.clone(),
        Arc::new(AllowReview),
        Limits::default(),
    )
    .unwrap();
    let first = create(&coordinator, 121, "Initial review").await;
    let _second = create(&coordinator, 123, "Second thread").await;

    let host = RepositoryHost::new(
        fixture.store.clone(),
        Arc::new(NoReviewOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap();
    let cache = tempfile::tempdir().unwrap();
    let router = HttpApi::new(
        host,
        cache.path().to_owned(),
        Config {
            access: Access::LocalTrusted {
                principal: ACTOR,
                permission: Permission::Admin,
            },
            ..Config::default()
        },
    )
    .unwrap()
    .router();

    let (status, first_page) = json(&router, "reviews/threads?limit=1").await;
    assert_eq!(status, StatusCode::OK, "{first_page}");
    let cursor = first_page["next_cursor"].as_str().unwrap();
    let reply = publish(
        &coordinator,
        125,
        ReviewCommandV1::AppendReply {
            thread_id: first.thread_id,
            expected_workflow_version: 1,
            parent_comment_id: first.comment_id,
            body: body("Follow-up"),
        },
    )
    .await;

    let (status, second_page) =
        json(&router, &format!("reviews/threads?limit=1&cursor={cursor}")).await;
    assert_eq!(status, StatusCode::OK, "{second_page}");
    assert_eq!(second_page["snapshot"], first_page["snapshot"]);
    let (status, detail) = json(&router, &format!("reviews/threads/{}", first.thread_id)).await;
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
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{missing}");

    if let Some(path) = std::env::var_os("JJ_STORE_REVIEW_CONTRACT_CAPTURE") {
        let samples = [
            ("/v1/repos/{repo}/reviews/threads", first_page),
            ("/v1/repos/{repo}/reviews/threads", second_page),
            ("/v1/repos/{repo}/reviews/threads/{thread}", detail),
        ]
        .map(|(path, data)| {
            serde_json::json!({
                "path": path,
                "method": "get",
                "status": 200,
                "media": "application/json",
                "data": data,
            })
        });
        std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
}
