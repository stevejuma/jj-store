use super::*;
use http_body::Body as _;
use jj_store_service::registry::{RegistrySnapshot, RepositoryEntry, RepositoryRegistry};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
#[derive(Default)]
pub(super) struct Registry {
    generation: AtomicU64,
    reads: AtomicUsize,
}
#[async_trait::async_trait]
impl RepositoryRegistry for Registry {
    async fn snapshot(
        &self,
        _: Option<m::TokenId>,
    ) -> std::result::Result<RegistrySnapshot, jj_store_service::registry::Error> {
        self.reads.fetch_add(1, Ordering::Relaxed);
        Ok(RegistrySnapshot {
            generation: self.generation.load(Ordering::Acquire),
            repositories: Vec::<RepositoryEntry>::new(),
        })
    }
    async fn reserve(
        &self,
        _: m::RepositoryId,
        _: String,
    ) -> std::result::Result<(), jj_store_service::registry::Error> {
        unreachable!()
    }
    async fn register(
        &self,
        _: m::RepositoryId,
        _: String,
        _: Option<m::TokenId>,
    ) -> std::result::Result<(), jj_store_service::registry::Error> {
        unreachable!()
    }
    async fn token_candidate(
        &self,
        _: m::RepositoryId,
        _: m::TokenId,
        _: bool,
    ) -> std::result::Result<(), jj_store_service::registry::Error> {
        unreachable!()
    }
}
pub(super) fn api(registry: Arc<Registry>, root: &std::path::Path) -> HttpApi {
    HttpApi::with_search(
        None,
        root.into(),
        Config {
            access: Access::LocalTrusted {
                principal: m::PrincipalId::new([1; 16]),
                permission: Permission::Admin,
            },
            registry: Some(registry),
            timeout: Duration::from_millis(20),
            ..Default::default()
        },
        BTreeMap::new(),
    )
    .unwrap()
}
async fn open(state: Arc<State>, after: Option<String>) -> Response {
    events(
        ExtractState(state),
        Ok(axum::extract::Query(Query { after })),
        HeaderMap::new(),
    )
    .await
    .unwrap()
}
async fn frame(body: &mut Body) -> String {
    let data = futures::future::poll_fn(|cx| std::pin::Pin::new(&mut *body).poll_frame(cx))
        .await
        .unwrap()
        .unwrap()
        .into_data()
        .unwrap();
    String::from_utf8(data.to_vec()).unwrap()
}
fn cursor(frame: &str) -> String {
    frame
        .lines()
        .find_map(|s| s.strip_prefix("id: ").or_else(|| s.strip_prefix("id:")))
        .unwrap()
        .trim()
        .into()
}
#[tokio::test]
async fn shared_installation_monitor_replays_scoped_changes_and_unchanged_renewal_is_read_free() {
    let root = tempfile::tempdir().unwrap();
    let registry = Arc::new(Registry::default());
    let api = api(registry.clone(), root.path());
    let mut first = open(api.0.clone(), None).await.into_body();
    let ready = frame(&mut first).await;
    assert!(ready.contains("event: ready") || ready.contains("event:ready"));
    let cursor = cursor(&ready);
    let mut second = open(api.0.clone(), Some(cursor.clone())).await.into_body();
    let renewed = frame(&mut second).await;
    assert!(!renewed.contains("changed"));
    let before = registry.reads.load(Ordering::Relaxed);
    tokio::time::sleep(Duration::from_millis(80)).await;
    assert_eq!(registry.reads.load(Ordering::Relaxed), before);
    assert!(!first.is_end_stream());
    api.0.freshness.changed(Change {
        repository: None,
        domain: Domain::Configuration,
        witness: "changed".into(),
        identity: None,
        audience: None,
    });
    let changed = frame(&mut first).await;
    assert!(changed.contains("configuration"));
    drop(first);
    drop(second);
    let mut replay = open(api.0.clone(), Some(cursor)).await.into_body();
    assert!(frame(&mut replay).await.contains("configuration"));
    assert!(frame(&mut replay).await.contains("ready"));
}
#[tokio::test]
async fn restart_cursor_resets_and_forged_bearer_reveals_no_metadata() {
    let root = tempfile::tempdir().unwrap();
    let api = api(Arc::default(), root.path());
    let mut initial = open(api.0.clone(), None).await.into_body();
    let cursor = cursor(&frame(&mut initial).await);
    let other = self::api(Arc::default(), root.path());
    let mut reset = open(other.0.clone(), Some(cursor)).await.into_body();
    assert!(frame(&mut reset).await.contains("generation_or_scope"));
    assert!(
        futures::future::poll_fn(|cx| std::pin::Pin::new(&mut reset).poll_frame(cx))
            .await
            .is_none()
    );
    let authenticated = HttpApi::with_search(
        None,
        root.path().into(),
        Config {
            registry: Some(Arc::new(Registry::default())),
            ..Default::default()
        },
        BTreeMap::new(),
    )
    .unwrap();
    let mut headers = HeaderMap::new();
    let token = BearerToken::generate().unwrap();
    headers.insert(
        header::AUTHORIZATION,
        format!("Bearer {}", token.expose().as_str())
            .parse()
            .unwrap(),
    );
    let response = events(
        ExtractState(authenticated.0),
        Ok(axum::extract::Query(Query { after: None })),
        headers,
    )
    .await
    .unwrap_err()
    .into_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn application_stream_has_independent_finite_delivery_admission_and_contract_capture() {
    use tower::ServiceExt as _;
    let root = tempfile::tempdir().unwrap();
    let mut config = Config {
        access: Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Admin,
        },
        registry: Some(Arc::new(Registry::default())),
        event_concurrency: 1,
        timeout: Duration::from_millis(40),
        event_lifetime: Duration::from_millis(180),
        ..Default::default()
    };
    let api =
        HttpApi::with_search(None, root.path().into(), config.clone(), BTreeMap::new()).unwrap();
    let request = || {
        axum::http::Request::builder()
            .uri("/v1/events")
            .header("host", "127.0.0.1:7443")
            .header("x-jj-store-protocol", "1")
            .body(Body::empty())
            .unwrap()
    };
    let response = api.router().oneshot(request()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let mut stream = response.into_body();
    let ready = frame(&mut stream).await;
    let mut samples = Vec::new();
    let mut record = |frame: &str| {
        let data = frame
            .lines()
            .find_map(|s| s.strip_prefix("data: ").or_else(|| s.strip_prefix("data:")))
            .unwrap();
        samples.push(serde_json::json!({ "path":"/v1/events", "method":"get", "status":200, "media":"text/event-stream", "data":serde_json::from_str::<serde_json::Value>(data).unwrap() }));
    };
    record(&ready);
    tokio::time::sleep(Duration::from_millis(70)).await;
    assert_eq!(
        api.router().oneshot(request()).await.unwrap().status(),
        StatusCode::SERVICE_UNAVAILABLE
    );
    api.0.freshness.changed(Change {
        repository: None,
        domain: Domain::Configuration,
        witness: "changed".into(),
        identity: None,
        audience: None,
    });
    record(&frame(&mut stream).await);
    tokio::time::sleep(Duration::from_millis(150)).await;
    assert!(
        futures::future::poll_fn(|cx| std::pin::Pin::new(&mut stream).poll_frame(cx))
            .await
            .unwrap()
            .is_err()
    );
    assert_eq!(
        api.router().oneshot(request()).await.unwrap().status(),
        StatusCode::OK
    );
    config.event_lifetime = Duration::from_secs(300);
    let cold = HttpApi::with_search(None, root.path().into(), config, BTreeMap::new()).unwrap();
    let mut reset = open(cold.0, Some(cursor(&ready))).await.into_body();
    record(&frame(&mut reset).await);
    if let Some(path) = std::env::var_os("JJ_STORE_APPLICATION_EVENTS_CONTRACT_CAPTURE") {
        std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
}

struct AgentChanges {
    notices: jj_store_service::freshness::Signals,
}
#[async_trait::async_trait]
impl jj_store_service::agent_execution::Manager for AgentChanges {
    async fn status(
        &self,
        _: m::PrincipalId,
    ) -> jj_store_service::agent_execution::Result<jj_store_service::agent_execution::Status> {
        unreachable!("notices must not poll status")
    }
    async fn targets(
        &self,
        _: m::PrincipalId,
        _: Option<m::RepositoryId>,
    ) -> jj_store_service::agent_execution::Result<Vec<jj_store_service::agent_execution::Target>>
    {
        unreachable!()
    }
    async fn save_setup(
        &self,
        _: m::PrincipalId,
        _: jj_store_service::agent_execution::SetupCommand,
    ) -> jj_store_service::agent_execution::Result<jj_store_service::agent_execution::Status> {
        unreachable!()
    }
    async fn launch(
        &self,
        _: m::PrincipalId,
        _: jj_store_service::agent_execution::Launch,
    ) -> jj_store_service::agent_execution::Result<jj_store_service::agent_execution::Outcome> {
        unreachable!()
    }
    async fn command(
        &self,
        _: m::PrincipalId,
        _: jj_store_service::agent_execution::Command,
    ) -> jj_store_service::agent_execution::Result<jj_store_service::agent_execution::Outcome> {
        unreachable!()
    }
    async fn outcome(
        &self,
        _: m::PrincipalId,
        _: m::RequestId,
    ) -> jj_store_service::agent_execution::Result<jj_store_service::agent_execution::Outcome> {
        unreachable!()
    }
    async fn events(
        &self,
        _: m::PrincipalId,
        _: String,
        _: u64,
    ) -> jj_store_service::agent_execution::Result<jj_store_service::agent_execution::EventPage>
    {
        unreachable!()
    }
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<u64> {
        unreachable!("shared feed uses scoped notices")
    }
    fn freshness(&self) -> Option<tokio::sync::broadcast::Receiver<Change>> {
        Some(self.notices.subscribe())
    }
    async fn shutdown(&self) -> jj_store_service::agent_execution::Result<()> {
        Ok(())
    }
}
#[tokio::test]
async fn agent_owner_notices_join_shared_feed_without_reads_and_remain_local_admin_only() {
    for permission in [Permission::Admin, Permission::Read] {
        let root = tempfile::tempdir().unwrap();
        let notices = jj_store_service::freshness::Signals::default();
        let api = HttpApi::with_search(
            None,
            root.path().into(),
            Config {
                access: Access::LocalTrusted {
                    principal: m::PrincipalId::new([1; 16]),
                    permission,
                },
                registry: Some(Arc::new(Registry::default())),
                local_agents: Some(Arc::new(AgentChanges {
                    notices: notices.clone(),
                })),
                ..Default::default()
            },
            BTreeMap::new(),
        )
        .unwrap();
        let mut body = open(api.0.clone(), None).await.into_body();
        let initial = cursor(&frame(&mut body).await);
        notices.notify(None, Domain::AgentExecution, None);
        notices.notify(None, Domain::AgentSettings, None);
        if permission == Permission::Admin {
            for domain in ["agent_execution", "agent_settings"] {
                let next = tokio::time::timeout(Duration::from_secs(1), frame(&mut body))
                    .await
                    .unwrap();
                assert!(next.contains(domain), "{next}");
                assert!(!next.contains("setups"));
            }
            assert!(
                tokio::time::timeout(Duration::from_millis(40), frame(&mut body))
                    .await
                    .is_err()
            );
            drop(body);
            let mut replay = open(api.0.clone(), Some(initial)).await.into_body();
            assert!(frame(&mut replay).await.contains("agent_execution"));
            assert!(frame(&mut replay).await.contains("agent_settings"));
            assert!(frame(&mut replay).await.contains("ready"));
        } else {
            assert!(
                tokio::time::timeout(Duration::from_millis(60), frame(&mut body))
                    .await
                    .is_err()
            );
        }
    }
}
