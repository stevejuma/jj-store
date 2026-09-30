use super::*;
use axum::{
    body::{Body, to_bytes},
    http::{Request as HttpRequest, StatusCode},
};
use jj_store_http::{Access, Config, HttpApi};
use jj_store_service::administration::{OperatorPolicies, RepositoryHost};
use serde_json::{Value, json as value};
use tower::ServiceExt as _;

struct NoWorkOperators;

#[async_trait::async_trait]
impl OperatorPolicies for NoWorkOperators {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        panic!("LocalTrusted work API accessed operator policy")
    }
}

use jj_store_service::work::{ProjectId, TaskId, WorkCommandV1, WorkCommentId};
fn command(seed: u8, command: WorkCommandV1) -> Value {
    value!({"version": 1, "request_id": m::RequestId::new([seed; 16]), "command": command})
}
fn create(seed: u8, text: &str) -> Value {
    command(
        seed,
        WorkCommandV1::CreateProject {
            id: ProjectId::new([seed; 16]),
            title: text.into(),
            body: "Project body".into(),
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
        Arc::new(NoWorkOperators),
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
async fn http_workstore_pages_commands_permissions_and_contract_capture() {
    let fixture = Fixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(&fixture, cache.path(), Permission::Admin);
    let first_request = create(121, "Release");
    let (status, accepted) = json(&router, "work/commands", Some(&first_request)).await;
    assert_eq!(status, StatusCode::OK, "{accepted}");
    let (status, _) = json(&router, "work/commands", Some(&create(122, "Second"))).await;
    assert_eq!(status, StatusCode::OK);
    let (_, first_page) = json(&router, "work?limit=1", None).await;
    let cursor = first_page["next_cursor"].as_str().unwrap();
    json(&router, "work/commands", Some(&create(123, "Later"))).await;
    let (status, second_page) = json(&router, &format!("work?limit=1&cursor={cursor}"), None).await;
    assert_eq!(status, StatusCode::OK, "{second_page}");
    assert_eq!(second_page["snapshot"], first_page["snapshot"]);
    assert!(second_page["next_cursor"].is_null());
    assert_eq!(
        json(&router, &format!("work?limit=2&cursor={cursor}"), None)
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    let project = ProjectId::new([121; 16]);
    let task = TaskId::new([124; 16]);
    let task_request = command(
        124,
        WorkCommandV1::CreateTask {
            id: task,
            project,
            expected_project_version: 1,
            title: "Native task".into(),
            body: "Task body".into(),
            repository: None,
            parent: None,
        },
    );
    let (status, task_accepted) = json(&router, "work/commands", Some(&task_request)).await;
    assert_eq!(status, StatusCode::OK, "{task_accepted}");
    let comment = command(
        125,
        WorkCommandV1::AddComment {
            id: WorkCommentId::new([125; 16]),
            task,
            expected_task_version: 1,
            reply_to: None,
            body: "Comment body".into(),
        },
    );
    let (status, comment_accepted) = json(&router, "work/commands", Some(&comment)).await;
    assert_eq!(status, StatusCode::OK, "{comment_accepted}");
    let (_, project_page) = json(&router, &format!("work?project={project}"), None).await;
    let (_, task_page) = json(&router, &format!("work?task={task}"), None).await;
    assert_eq!(task_page["comments"][0]["body"], "Comment body");
    let fresh = api(&fixture, cache.path(), Permission::Admin);
    let (_, durable) = json(
        &fresh,
        &format!("work/commands/{}", m::RequestId::new([121; 16])),
        None,
    )
    .await;
    let (_, replay) = json(&fresh, "work/commands", Some(&first_request)).await;
    assert_eq!(durable, accepted);
    assert_eq!(replay, accepted);
    let stale = command(
        126,
        WorkCommandV1::AddComment {
            id: WorkCommentId::new([126; 16]),
            task,
            expected_task_version: 1,
            reply_to: None,
            body: "Stale".into(),
        },
    );
    let (status, rejected) = json(&fresh, "work/commands", Some(&stale)).await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(rejected["code"], "stale_work");
    let before = fixture.session.snapshot().await.unwrap().digest();
    let reader = api(&fixture, cache.path(), Permission::Read);
    assert_eq!(json(&reader, "work", None).await.0, StatusCode::OK);
    assert_eq!(
        json(&reader, "work/commands", Some(&create(127, "Denied")))
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(fixture.session.snapshot().await.unwrap().digest(), before);
    if let Some(path) = std::env::var_os("JJ_STORE_WORK_CONTRACT_CAPTURE") {
        let samples = [
            ("/v1/repos/{repo}/work", "get", first_page),
            ("/v1/repos/{repo}/work", "get", second_page),
            ("/v1/repos/{repo}/work", "get", project_page),
            ("/v1/repos/{repo}/work", "get", task_page),
            ("/v1/repos/{repo}/work/commands", "post", accepted),
            ("/v1/repos/{repo}/work/commands", "post", task_accepted),
            ("/v1/repos/{repo}/work/commands", "post", comment_accepted),
            (
                "/v1/repos/{repo}/work/commands/{request_id}",
                "get",
                durable,
            ),
        ]
        .map(|(path, method, data)| value!({"path":path,"method":method,"status":200,"media":"application/json","data":data}));
        std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
}
