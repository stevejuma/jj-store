use super::*;
use axum::{
    body::{Body, to_bytes},
    http::Request,
};
use tower::ServiceExt as _;
struct Manager {
    changes: tokio::sync::broadcast::Sender<u64>,
}
#[async_trait::async_trait]
impl agents::Manager for Manager {
    async fn status(&self, actor: m::PrincipalId) -> agents::Result<agents::Status> {
        assert_eq!(actor, m::PrincipalId::new([1; 16]));
        Ok(agents::Status {
            version: 1,
            runtime: "ready".into(),
            revision: 0,
            setups: vec![],
            sessions: vec![],
            runners: vec![],
        })
    }
    async fn targets(
        &self,
        _: m::PrincipalId,
        _: Option<m::RepositoryId>,
    ) -> agents::Result<Vec<agents::Target>> {
        Ok(vec![])
    }
    async fn save_setup(
        &self,
        _: m::PrincipalId,
        _: agents::SetupCommand,
    ) -> agents::Result<agents::Status> {
        Err(agents::Error::Invalid)
    }
    async fn launch(
        &self,
        _: m::PrincipalId,
        _: agents::Launch,
    ) -> agents::Result<agents::Outcome> {
        Err(agents::Error::Invalid)
    }
    async fn command(
        &self,
        _: m::PrincipalId,
        _: agents::Command,
    ) -> agents::Result<agents::Outcome> {
        Err(agents::Error::Invalid)
    }
    async fn outcome(&self, _: m::PrincipalId, _: m::RequestId) -> agents::Result<agents::Outcome> {
        Err(agents::Error::Invalid)
    }
    async fn events(
        &self,
        _: m::PrincipalId,
        _: String,
        _: u64,
    ) -> agents::Result<agents::EventPage> {
        Ok(agents::EventPage {
            version: 1,
            events: vec![],
            next: None,
        })
    }
    async fn feedback(
        &self,
        actor: m::PrincipalId,
        session: String,
    ) -> agents::Result<agents::Feedback> {
        assert_eq!(actor, m::PrincipalId::new([1; 16]));
        assert_eq!(session, "fixture");
        Ok(agents::Feedback {
            messages: vec![],
            profile: None,
            write_unavailable: None,
        })
    }
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<u64> {
        self.changes.subscribe()
    }
    async fn shutdown(&self) -> agents::Result<()> {
        Ok(())
    }
}
#[tokio::test]
async fn feedback_is_read_only_local_admin_and_rejects_unbound_queries() {
    let root = tempfile::tempdir().unwrap();
    for access in [
        Access::Authenticated,
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Read,
        },
    ] {
        let response = api(root.path(), access)
            .router()
            .oneshot(request("/v1/local/agents/feedback?session=fixture", None))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
    let api = api(
        root.path(),
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Admin,
        },
    );
    let response = api
        .router()
        .oneshot(request("/v1/local/agents/feedback?session=fixture", None))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let feedback: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
    assert_eq!(feedback, serde_json::json!({"messages":[]}));
    for path in [
        "/v1/local/agents/feedback?session=fixture&after=1",
        "/v1/local/agents/feedback?session=fixture&actor=caller",
    ] {
        assert_eq!(
            api.router()
                .oneshot(request(path, None))
                .await
                .unwrap()
                .status(),
            StatusCode::BAD_REQUEST
        );
    }
}
fn api(root: &std::path::Path, access: Access) -> HttpApi {
    let (changes, _) = tokio::sync::broadcast::channel(32);
    HttpApi::with_search(
        None,
        root.into(),
        Config {
            access,
            local_agents: Some(Arc::new(Manager { changes })),
            event_concurrency: 1,
            ..Default::default()
        },
        BTreeMap::new(),
    )
    .unwrap()
}
fn request(path: &str, body: Option<serde_json::Value>) -> Request<Body> {
    let mut request = Request::builder()
        .uri(path)
        .header("x-jj-store-protocol", "1")
        .header("host", "127.0.0.1:7443");
    let body = if let Some(body) = body {
        request = request
            .method("POST")
            .header("content-type", "application/json");
        Body::from(body.to_string())
    } else {
        Body::empty()
    };
    request.body(body).unwrap()
}
#[tokio::test]
async fn execution_routes_require_local_admin_and_refuse_browser_launch_configuration() {
    let root = tempfile::tempdir().unwrap();
    for access in [
        Access::Authenticated,
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Read,
        },
    ] {
        let response = api(root.path(), access)
            .router()
            .oneshot(request("/v1/local/agents", None))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
    let api = api(
        root.path(),
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Admin,
        },
    );
    let response = api
        .router()
        .oneshot(request("/v1/local/agents", None))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let status: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
    assert_eq!(status["version"], 1);
    for extra in ["cwd", "executable", "environment", "actor"] {
        let mut body = serde_json::json!({"version":1,"request_id":"11".repeat(16),"target_id":"device-binding","repository":"22".repeat(16),"operation":"33".repeat(64),"commit":"44".repeat(20),"runner":"codex","setup_id":null,"context":null});
        body[extra] = serde_json::json!("untrusted");
        let response = api
            .router()
            .oneshot(request("/v1/local/agents/launch", Some(body)))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    let stream = api
        .router()
        .oneshot(request("/v1/local/agents/stream?session=fixture", None))
        .await
        .unwrap();
    assert_eq!(stream.status(), StatusCode::OK);
    let saturated = api
        .router()
        .oneshot(request("/v1/local/agents/stream?session=fixture", None))
        .await
        .unwrap();
    assert_eq!(saturated.status(), StatusCode::SERVICE_UNAVAILABLE);
    drop(stream);
    let reopened = api
        .router()
        .oneshot(request("/v1/local/agents/stream?session=fixture", None))
        .await
        .unwrap();
    assert_eq!(reopened.status(), StatusCode::OK);
}

#[tokio::test]
async fn local_inbox_collaboration_routes_require_local_admin_without_actor_input() {
    let root = tempfile::tempdir().unwrap();
    let repository = "11".repeat(16);
    let command = serde_json::json!({"version":1,"request_id":"22".repeat(16),"command":{"domain":"thread","intent":{"type":"create","thread_id":"33".repeat(16),"title":"Reply","baseline":null}}});
    let paths = [
        (
            format!("/v1/local/repos/{repository}/collaboration/snapshot"),
            None,
        ),
        (
            format!("/v1/local/repos/{repository}/collaboration/query"),
            Some(
                serde_json::json!({"metadata_commit":null,"query":{"type":"threads","limit":50},"cursor":null}),
            ),
        ),
        (
            format!("/v1/local/repos/{repository}/collaboration/thread"),
            Some(serde_json::json!({"metadata_commit":null,"thread_id":"33".repeat(16)})),
        ),
        (
            format!("/v1/local/repos/{repository}/collaboration/commands"),
            Some(command.clone()),
        ),
        (
            format!(
                "/v1/local/repos/{repository}/collaboration/commands/{}",
                "22".repeat(16)
            ),
            None,
        ),
        (
            format!(
                "/v1/local/repos/{repository}/collaboration/assets/{}",
                "44".repeat(32)
            ),
            None,
        ),
    ];
    for access in [
        Access::Authenticated,
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Read,
        },
    ] {
        let api = api(root.path(), access);
        for (path, body) in &paths {
            assert_eq!(
                api.router()
                    .oneshot(request(path, body.clone()))
                    .await
                    .unwrap()
                    .status(),
                StatusCode::FORBIDDEN,
                "{path}"
            );
        }
    }
    let api = api(
        root.path(),
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Admin,
        },
    );
    let mut injected = command;
    injected["actor"] = serde_json::json!("55".repeat(16));
    assert_eq!(
        api.router()
            .oneshot(request(&paths[3].0, Some(injected)))
            .await
            .unwrap()
            .status(),
        StatusCode::BAD_REQUEST
    );
}

#[tokio::test]
async fn feedback_preparation_refusals_have_specific_transport_codes() {
    use axum::response::IntoResponse;
    for (error, expected_status, code) in [
        (
            agents::Error::FeedbackLimit,
            StatusCode::PAYLOAD_TOO_LARGE,
            "agent_feedback_limit",
        ),
        (
            agents::Error::FeedbackUnavailable,
            StatusCode::SERVICE_UNAVAILABLE,
            "agent_feedback_unavailable",
        ),
    ] {
        let response = failure(error).into_response();
        assert_eq!(response.status(), expected_status);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(value["code"], code);
    }
}
