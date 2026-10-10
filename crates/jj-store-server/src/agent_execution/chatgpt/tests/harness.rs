use super::*;
use axum::{
    Json,
    extract::State as ExtractState,
    http::StatusCode,
    routing::{get, post},
};
use std::sync::atomic::{AtomicUsize, Ordering};
pub(in crate::agent_execution::chatgpt) struct FixtureIo {
    pub origin: String,
    pub opened: tokio::sync::mpsc::UnboundedSender<url::Url>,
    pub vault: Mutex<BTreeMap<String, Cached>>,
    pub identity: Mutex<(String, String)>,
    pub mode: AtomicUsize,
    pub refreshes: AtomicUsize,
    pub started: tokio::sync::Notify,
    pub release: tokio::sync::Notify,
    pub forms: Mutex<Vec<BTreeMap<String, String>>>,
}
async fn exchange(
    ExtractState(io): ExtractState<Arc<FixtureIo>>,
    axum::extract::Form(form): axum::extract::Form<BTreeMap<String, String>>,
) -> (StatusCode, Json<Value>) {
    assert_eq!(form["resource"], oauth::RESOURCE);
    io.forms.lock().unwrap().push(form.clone());
    if form["grant_type"] == "refresh_token" {
        let count = io.refreshes.fetch_add(1, Ordering::SeqCst) + 1;
        match io.mode.load(Ordering::SeqCst) {
            1 => {
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(json!({"error":"temporarily_unavailable"})),
                );
            }
            2 => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error":"invalid_grant"})),
                );
            }
            3 => {
                io.started.notify_one();
                io.release.notified().await;
            }
            _ => {}
        }
        return (
            StatusCode::OK,
            Json(
                json!({"access_token":format!("fixture-access-renewed-{count}"),"refresh_token":format!("fixture-refresh-renewed-{count}"),"expires_in":3600}),
            ),
        );
    }
    assert_eq!(form["grant_type"], "authorization_code");
    assert_eq!(form["client_id"], "issued-one");
    assert!(form["redirect_uri"].starts_with("http://127.0.0.1:"));
    use base64::Engine as _;
    use sha2::Digest as _;
    assert_eq!(
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(sha2::Sha256::digest(&form["code_verifier"])),
        io.identity.lock().unwrap().1
    );
    let nonce = io.identity.lock().unwrap().0.clone();
    let subject = if io.mode.load(Ordering::SeqCst) == 4 {
        "foreign-subject"
    } else {
        "subject-one"
    };
    let scope = if io.mode.load(Ordering::SeqCst) == 5 {
        "openid email".into()
    } else {
        format!(
            "openid email offline_access resource.invoke {}",
            oauth::PLAN_SCOPE
        )
    };
    (
        StatusCode::OK,
        Json(
            json!({"access_token":"fixture-access-original","refresh_token":"fixture-refresh-original","id_token":token(json!({"iss":oauth::ISSUER,"aud":"issued-one","sub":subject,"exp":now()+3600,"nonce":nonce,"email":"fixture@example.invalid"})),"expires_in":3600,"scope":scope}),
        ),
    )
}
async fn keys() -> Json<Value> {
    Json(serde_json::from_str(include_str!("../fixtures/synthetic-jwks.json")).unwrap())
}
async fn discovery(ExtractState(io): ExtractState<Arc<FixtureIo>>) -> Json<Value> {
    Json(json!({"revocation_endpoint":format!("{}/revoke",io.origin)}))
}
async fn models() -> Json<Value> {
    Json(
        json!({"models":[{"slug":"hidden","display_name":"Hidden","visibility":"hide"},{"slug":"account-model","display_name":"Account model","visibility":"list"}]}),
    )
}
pub(super) struct Fixture {
    pub _root: tempfile::TempDir,
    pub supervisor: Supervisor,
    pub io: Arc<FixtureIo>,
    pub opened: tokio::sync::mpsc::UnboundedReceiver<url::Url>,
    server: tokio::task::JoinHandle<()>,
}
impl Fixture {
    pub async fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        let supervisor = Supervisor::open(Resources {
            actor: PrincipalId::new([1; 16]),
            root: root.path().join("agent"),
            executable: std::env::current_exe().unwrap(),
            jj_executable: PathBuf::from("/usr/bin/true"),
            driver: Arc::new(driver::Driver(jj_store_jj::agent_workspaces::Provider {
                executable: PathBuf::from("/usr/bin/true"),
            })),
            folders: None,
            workspaces: None,
            verifier: None,
            registry: Arc::new(Registry),
            host: None,
            native_socket: None,
        })
        .unwrap();
        tokio::time::timeout(Duration::from_secs(120), async {
            while !supervisor.ready() {
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .unwrap();
        let (opened, receiver) = tokio::sync::mpsc::unbounded_channel();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let io = Arc::new(FixtureIo {
            origin: format!("http://{}", listener.local_addr().unwrap()),
            opened,
            vault: Mutex::new(BTreeMap::new()),
            identity: Mutex::new((String::new(), String::new())),
            mode: AtomicUsize::new(0),
            refreshes: AtomicUsize::new(0),
            started: Default::default(),
            release: Default::default(),
            forms: Default::default(),
        });
        *supervisor.0.chatgpt.fixture.lock().unwrap() = Some(io.clone());
        let router = axum::Router::new()
            .route("/api/accounts/oauth/token", post(exchange))
            .route("/.well-known/jwks.json", get(keys))
            .route("/.well-known/openid-configuration", get(discovery))
            .route("/revoke", post(|| async { StatusCode::OK }))
            .route("/v1/models", get(models))
            .with_state(io.clone());
        let server = tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });
        Self {
            _root: root,
            supervisor,
            io,
            opened: receiver,
            server,
        }
    }
    pub async fn authorization(&mut self) -> url::Url {
        let url = tokio::time::timeout(Duration::from_secs(5), self.opened.recv())
            .await
            .unwrap()
            .unwrap();
        let fields = url.query_pairs().collect::<BTreeMap<_, _>>();
        *self.io.identity.lock().unwrap() = (
            fields["nonce"].to_string(),
            fields["code_challenge"].to_string(),
        );
        url
    }
    pub async fn callback(&self, url: &url::Url, state: Option<&str>) -> reqwest::StatusCode {
        let q = url.query_pairs().collect::<BTreeMap<_, _>>();
        reqwest::Client::new()
            .get(q["redirect_uri"].as_ref())
            .query(&[
                ("state", state.unwrap_or(q["state"].as_ref())),
                ("code", "synthetic-code"),
                ("client_id", "issued-one"),
            ])
            .send()
            .await
            .unwrap()
            .status()
    }
    pub async fn settled(&self, id: &str) -> contract::Attempt {
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let view = self
                    .supervisor
                    .chatgpt_view()
                    .unwrap()
                    .attempts
                    .into_iter()
                    .find(|a| a.id == id)
                    .unwrap();
                if view.state != "authorizing" {
                    return view;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.server.abort();
        self.supervisor.0.chatgpt.shutdown();
        if let Ok(mut tasks) = self.supervisor.0.runtime_tasks.lock() {
            for (_, task) in std::mem::take(&mut *tasks) {
                task.abort();
            }
        }
    }
}
struct Registry;
#[async_trait::async_trait]
impl RepositoryRegistry for Registry {
    async fn snapshot(
        &self,
        _: Option<jj_store_model::TokenId>,
    ) -> std::result::Result<
        jj_store_service::registry::RegistrySnapshot,
        jj_store_service::registry::Error,
    > {
        Ok(jj_store_service::registry::RegistrySnapshot {
            generation: 0,
            repositories: vec![],
        })
    }
    async fn reserve(
        &self,
        _: RepositoryId,
        _: String,
    ) -> std::result::Result<(), jj_store_service::registry::Error> {
        panic!("no repository mutation")
    }
    async fn register(
        &self,
        _: RepositoryId,
        _: String,
        _: Option<jj_store_model::TokenId>,
    ) -> std::result::Result<(), jj_store_service::registry::Error> {
        panic!("no repository mutation")
    }
    async fn token_candidate(
        &self,
        _: RepositoryId,
        _: jj_store_model::TokenId,
        _: bool,
    ) -> std::result::Result<(), jj_store_service::registry::Error> {
        panic!("no repository mutation")
    }
}
