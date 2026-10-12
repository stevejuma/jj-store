use super::*;
use axum::{body::Body, http::Request};
use futures::StreamExt;
use std::sync::atomic::{AtomicBool, Ordering};
use tower::ServiceExt;
struct Broker {
    token: BearerToken,
    revoked: AtomicBool,
    changes: tokio::sync::broadcast::Sender<u64>,
}
#[async_trait::async_trait]
impl jj_store_service::remote_agents::RemoteAgents for Broker {
    async fn admit(&self, token: BearerToken) -> a::Result<(m::PrincipalId, a::AgentManager)> {
        if token.id() != self.token.id() || self.revoked.load(Ordering::Acquire) {
            return Err(a::Error::Denied);
        }
        // The fixture lease is revoked independently after admission.
        Ok((
            m::PrincipalId::new([1; 16]),
            Arc::new(Execution {
                changes: self.changes.clone(),
            }),
        ))
    }
    async fn shutdown(&self) -> a::Result<()> {
        Ok(())
    }
}
struct Execution {
    changes: tokio::sync::broadcast::Sender<u64>,
}
#[async_trait::async_trait]
impl a::Manager for Execution {
    async fn status(&self, p: m::PrincipalId) -> a::Result<a::Status> {
        assert_eq!(p, m::PrincipalId::new([1; 16]));
        Ok(a::Status {
            version: 1,
            runtime: "ready".into(),
            revision: 0,
            setups: vec![],
            sessions: vec![],
            runners: vec![],
            live_agents: vec![],
            conversations: vec![],
        })
    }
    async fn targets(
        &self,
        _: m::PrincipalId,
        _: Option<m::RepositoryId>,
    ) -> a::Result<Vec<a::Target>> {
        Ok(vec![])
    }
    async fn save_setup(&self, _: m::PrincipalId, _: a::SetupCommand) -> a::Result<a::Status> {
        Err(a::Error::Denied)
    }
    async fn launch(&self, _: m::PrincipalId, _: a::Launch) -> a::Result<a::Outcome> {
        Err(a::Error::Denied)
    }
    async fn command(&self, _: m::PrincipalId, _: a::Command) -> a::Result<a::Outcome> {
        Err(a::Error::Denied)
    }
    async fn outcome(&self, _: m::PrincipalId, _: m::RequestId) -> a::Result<a::Outcome> {
        Err(a::Error::Denied)
    }
    async fn events(
        &self,
        _: m::PrincipalId,
        session: String,
        after: u64,
    ) -> a::Result<a::EventPage> {
        if session != "owned" || after > 0 {
            return Err(a::Error::Denied);
        }
        Ok(a::EventPage {
            version: 1,
            events: vec![a::Event {
                sequence: 1,
                observed_at_millis: None,
                payload: serde_json::json!({"type":"record"}),
                committed: false,
            }],
            next: None,
        })
    }
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<u64> {
        self.changes.subscribe()
    }
    async fn shutdown(&self) -> a::Result<()> {
        Ok(())
    }
}
fn request(path: &str, token: Option<&BearerToken>) -> Request<Body> {
    let mut request = Request::builder()
        .uri(path)
        .header("host", "127.0.0.1:7443")
        .header("x-jj-store-protocol", "1");
    if let Some(token) = token {
        request = request.header(
            "authorization",
            format!("Bearer {}", token.expose().as_str()),
        );
    }
    request.body(Body::empty()).unwrap()
}
#[tokio::test]
async fn authenticated_execution_requires_caller_admission_and_rechecks_stream_continuation() {
    let root = tempfile::tempdir().unwrap();
    let (changes, _) = tokio::sync::broadcast::channel(8);
    let broker = Arc::new(Broker {
        token: BearerToken::generate().unwrap(),
        revoked: AtomicBool::new(false),
        changes,
    });
    let api = HttpApi::with_search(
        None,
        root.path().into(),
        Config {
            remote_agents: Some(broker.clone()),
            installation_id: Some("ab".repeat(16)),
            event_lifetime: Duration::from_secs(5),
            ..Default::default()
        },
        BTreeMap::new(),
    )
    .unwrap();
    let outsider = BearerToken::generate().unwrap();
    assert_eq!(
        api.router()
            .oneshot(request("/v1/agents", None))
            .await
            .unwrap()
            .status(),
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        api.router()
            .oneshot(request("/v1/agents", Some(&outsider)))
            .await
            .unwrap()
            .status(),
        StatusCode::FORBIDDEN
    );
    let response = api
        .router()
        .oneshot(request("/v1/agents", Some(&broker.token)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()["x-jj-store-installation"],
        "ab".repeat(16)
    );
    for path in [
        "/v1/local/agents",
        "/v1/local/connections",
        "/v1/agents/stream?session=other",
    ] {
        assert_eq!(
            api.router()
                .oneshot(request(path, Some(&broker.token)))
                .await
                .unwrap()
                .status(),
            StatusCode::FORBIDDEN
        );
    }
    let mut wrong = request("/v1/agents", Some(&broker.token));
    wrong.headers_mut().insert(
        "x-jj-store-target-installation",
        "cdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcd".parse().unwrap(),
    );
    assert_eq!(
        api.router().oneshot(wrong).await.unwrap().status(),
        StatusCode::CONFLICT
    );
    assert_eq!(
        api.router()
            .oneshot(request("/v1/agents?actor=spoof", Some(&broker.token)))
            .await
            .unwrap()
            .status(),
        StatusCode::BAD_REQUEST
    );
    let response = api
        .router()
        .oneshot(request(
            "/v1/agents/stream?session=owned",
            Some(&broker.token),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let mut body = response.into_body().into_data_stream();
    assert!(body.next().await.unwrap().is_ok());
    broker.changes.send(1).unwrap();
    assert!(
        tokio::time::timeout(Duration::from_secs(3), body.next())
            .await
            .unwrap()
            .unwrap()
            .is_err()
    );
    broker.revoked.store(true, Ordering::Release);
    assert_eq!(
        api.router()
            .oneshot(request("/v1/agents", Some(&broker.token)))
            .await
            .unwrap()
            .status(),
        StatusCode::FORBIDDEN
    );
}
