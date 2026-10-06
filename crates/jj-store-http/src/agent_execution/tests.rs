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
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<u64> {
        self.changes.subscribe()
    }
    async fn shutdown(&self) -> agents::Result<()> {
        Ok(())
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
