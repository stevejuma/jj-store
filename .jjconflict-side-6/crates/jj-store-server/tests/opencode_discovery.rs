#![cfg(unix)]
#[path = "capture_source/fixture.rs"]
#[allow(dead_code)]
mod native;
use axum::{
    Router,
    body::Body,
    extract::State,
    http::{Request, Response},
};
use jj_store_capture::profile::CaptureProfile;
use jj_store_server::{
    capture_config::{AgentCaptureConfig, HarnessConfig},
    capture_runtime::RunningCodexCapture,
    capture_supervisor::{
        CodexCaptureSupervisor, CodexSupervisorLocations, CodexSupervisorResources,
    },
};
use native::{A, B, Fixture};
use serde_json::{Value, json};
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU8, Ordering},
    },
    time::Duration,
};
struct Data {
    a: String,
    b: String,
    foreign: String,
    requests: Mutex<Vec<String>>,
    inventory_mode: AtomicU8,
    tail: AtomicBool,
}
struct Api {
    endpoint: String,
    data: Arc<Data>,
    task: tokio::task::JoinHandle<()>,
}
impl Api {
    async fn new(f: &Fixture) -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}/", listener.local_addr().unwrap());
        let data = Arc::new(Data {
            a: f.workspace(A).to_str().unwrap().into(),
            b: f.workspace(B).to_str().unwrap().into(),
            foreign: f.root.path().join("foreign").to_str().unwrap().into(),
            requests: Mutex::new(vec![]),
            inventory_mode: AtomicU8::new(0),
            tail: AtomicBool::new(false),
        });
        let app = Router::new().fallback(handle).with_state(data.clone());
        let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        Self {
            endpoint,
            data,
            task,
        }
    }
}
impl Drop for Api {
    fn drop(&mut self) {
        self.task.abort()
    }
}
async fn handle(State(d): State<Arc<Data>>, request: Request<Body>) -> Response<Body> {
    assert_eq!(request.method(), "GET");
    for header in ["authorization", "cookie", "proxy-authorization", "referer"] {
        assert!(!request.headers().contains_key(header));
    }
    d.requests.lock().unwrap().push(request.uri().to_string());
    let query: std::collections::BTreeMap<_, _> =
        url::form_urlencoded::parse(request.uri().query().unwrap_or_default().as_bytes())
            .into_owned()
            .collect();
    let path = request.uri().path();
    let value = match path {
        "/global/health" => json!({"healthy":true,"version":"1.18.30"}),
        "/project" => match d.inventory_mode.load(Ordering::SeqCst) {
            1 => return Response::builder().status(503).body(Body::empty()).unwrap(),
            4 => json!(
                (0..4097)
                    .map(|i| json!({"id":format!("project_{i}")}))
                    .collect::<Vec<_>>()
            ),
            _ => {
                json!([{"id":"global","worktree":"/","sandboxes":[]},{"id":"code","worktree":d.b,"sandboxes":[]}])
            }
        },
        "/project/global/directories" => json!([]),
        "/project/code/directories" => json!([{"directory":d.b}]),
        "/session" => match d.inventory_mode.load(Ordering::SeqCst) {
            2 => json!(
                (0..4097)
                    .map(|i| json!({"directory":format!("/workspace/overflow-{i}")}))
                    .collect::<Vec<_>>()
            ),
            3 => json!({"unexpected":"shape"}),
            5 => json!(
                (0..4096)
                    .map(|_| json!({"directory":d.a}))
                    .collect::<Vec<_>>()
            ),
            _ => {
                json!([{"directory":d.a},{"directory":d.b},{"directory":d.a},{"directory":d.foreign},{"directory":"/workspace/../unsafe"}])
            }
        },
        "/session/status" => match query.get("directory").map(String::as_str) {
            Some(dir) if dir == d.a => json!({"ses_a":{"type":"busy"}}),
            Some(dir) if dir == d.b => json!({"ses_b":{"type":"retry"}}),
            None => json!({}),
            _ => panic!("unverified directory received activity request"),
        },
        "/session/ses_a" => json!({"id":"ses_a","version":"1.18.30","directory":d.a}),
        "/session/ses_b" => json!({"id":"ses_b","version":"1.18.30","directory":d.b}),
        "/session/ses_a/message" => messages(
            "ses_a",
            &d.a,
            "workspace A transcript",
            d.tail.load(Ordering::SeqCst),
        ),
        "/session/ses_b/message" => messages(
            "ses_b",
            &d.b,
            "workspace B transcript",
            d.tail.load(Ordering::SeqCst),
        ),
        _ => panic!("unexpected discovery request {path}"),
    };
    Response::builder()
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&value).unwrap()))
        .unwrap()
}
fn message(id: &str, directory: &str, text: &str) -> Value {
    json!([{"info":{"id":"msg_first","sessionID":id,"role":"assistant","path":{"cwd":directory}},"parts":[{"id":"prt_first","sessionID":id,"messageID":"msg_first","type":"text","text":text}]}])
}
fn messages(id: &str, directory: &str, text: &str, tail: bool) -> Value {
    let mut rows = message(id, directory, text);
    if tail {
        let mut row = message(id, directory, "offline discovery tail")[0].clone();
        row["info"]["id"] = json!("msg_tail");
        row["parts"][0]["id"] = json!("prt_tail");
        row["parts"][0]["messageID"] = json!("msg_tail");
        rows.as_array_mut().unwrap().push(row);
    }
    rows
}
fn prepare(f: &Fixture, api: &Api) -> CodexCaptureSupervisor {
    CodexCaptureSupervisor::prepare(
        AgentCaptureConfig {
            enabled: true,
            max_readers: 2,
            max_sources: 4,
            rescan_seconds: 1,
            max_pending_bytes: f.spool.pending_capacity(),
            harnesses: vec![HarnessConfig {
                profile: CaptureProfile::OpencodeReadOnlyApiV1,
                product_version: "1.18.30".into(),
                roots: vec![],
                endpoint: Some(api.endpoint.clone()),
            }],
        },
        CodexSupervisorLocations {
            home: None,
            codex_home: None,
        },
        CodexSupervisorResources {
            authority: Arc::new(
                jj_store_server::capture_authority::CaptureAuthority::prepare(
                    &jj_store_server::config::Config::load(&f.root.path().join("server.toml"))
                        .unwrap(),
                )
                .unwrap()
                .unwrap(),
            ),
            host: f.host.clone(),
            admission: f.admission.clone(),
            sources: f.sources.clone(),
            spool: f.spool.fork_handle(),
            privacy: f.privacy(),
        },
        Duration::from_secs(15),
    )
    .unwrap()
    .unwrap()
}
async fn recorded(f: &Fixture, repo: jj_store_model::RepositoryId, text: &str) -> bool {
    let native = f
        .host
        .endpoint(repo)
        .unwrap()
        .bind_local_trusted(
            f.authority.principal(),
            jj_store_service::auth::Permission::Read,
        )
        .unwrap()
        .into_session();
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    let page = pager.begin(native.clone(), 10).await.unwrap();
    for session in page.sessions {
        if serde_json::to_string(
            &pager
                .detail(native.clone(), session.session_id)
                .await
                .unwrap(),
        )
        .unwrap()
        .contains(text)
        {
            return true;
        }
    }
    false
}
async fn shutdown(runtime: &mut RunningCodexCapture) {
    // The public five-second timeout retains its owner and explicitly permits retry.
    tokio::time::timeout(Duration::from_secs(15), async {
        loop {
            match runtime.shutdown().await {
                Ok(()) => break,
                Err(jj_store_server::Error::Service("capture runtime is still draining")) => {
                    continue;
                }
                Err(error) => panic!("capture drain failed: {error}"),
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(
        runtime.snapshot().phase,
        jj_store_server::capture_worker::CaptureWorkerPhase::Stopped
    );
}
#[tokio::test]
async fn runtime_discovers_registered_native_directories_when_default_status_is_empty() {
    let f = Fixture::new(16 * 1024 * 1024).await;
    let api = Api::new(&f).await;
    let mut runtime = RunningCodexCapture::start(prepare(&f, &api)).unwrap();
    let completed = tokio::time::timeout(Duration::from_secs(15), async {
        while !(recorded(&f, A, "workspace A transcript").await
            && recorded(&f, B, "workspace B transcript").await)
        {
            assert!(runtime.snapshot().status.running_sources <= 2);
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await;
    shutdown(&mut runtime).await;
    assert!(
        completed.is_ok(),
        "default-directory discovery missed the linked workspaces"
    );
    let requests = api.data.requests.lock().unwrap();
    assert!(
        requests
            .iter()
            .any(|r| r.starts_with("/session?directory=%2F&scope=project&limit=4096"))
    );
    assert!(!requests.iter().any(|r| r == "/session/status"));
    assert!(
        !requests
            .iter()
            .any(|r| r.contains("foreign") || r.contains("unsafe"))
    );
    assert!(f.spool.pending().unwrap().is_empty());
}
#[tokio::test]
async fn directory_candidates_without_native_write_never_receive_activity_requests() {
    let mut f = Fixture::new(16 * 1024 * 1024).await;
    let api = Api::new(&f).await;
    f.without_native_authority();
    let before = std::fs::read(f.root.path().join("sources/sources.json")).unwrap();
    let mut runtime = RunningCodexCapture::start(prepare(&f, &api)).unwrap();
    tokio::time::sleep(Duration::from_millis(250)).await;
    shutdown(&mut runtime).await;
    let requests = api.data.requests.lock().unwrap();
    assert!(
        requests
            .iter()
            .any(|r| r.starts_with("/session?directory=%2F&scope=project&limit=4096"))
    );
    assert!(!requests.iter().any(|r| r.starts_with("/session/")));
    assert_eq!(
        std::fs::read(f.root.path().join("sources/sources.json")).unwrap(),
        before
    );
    assert!(f.spool.pending().unwrap().is_empty());
}

#[tokio::test]
async fn bounded_inventories_withhold_malformed_overflow_and_unsafe_directory_requests() {
    let f = Fixture::new(16 * 1024 * 1024).await;
    let api = Api::new(&f).await;
    let read = jj_store_server::opencode_read_api::OpenCodeReadApi::new(&api.endpoint).unwrap();
    assert_eq!(
        read.project_inventory().await.unwrap().projects,
        vec!["global", "code"]
    );
    let directories = read.global_session_directories().await.unwrap();
    assert_eq!(directories.directories.len(), 3);
    assert_eq!(directories.unsupported, 1);
    assert!(
        read.project_directories("global")
            .await
            .unwrap()
            .directories
            .is_empty()
    );
    assert_eq!(
        read.project_directories("code").await.unwrap().directories,
        vec![api.data.b.clone()]
    );
    for mode in [2, 3] {
        api.data.inventory_mode.store(mode, Ordering::SeqCst);
        assert!(read.global_session_directories().await.is_err());
    }
    api.data.inventory_mode.store(5, Ordering::SeqCst);
    let limited = read.global_session_directories().await.unwrap();
    assert_eq!(limited.directories, vec![api.data.a.clone()]);
    assert_eq!(limited.unsupported, 1);
    api.data.inventory_mode.store(4, Ordering::SeqCst);
    assert!(read.project_inventory().await.is_err());
    let count = api.data.requests.lock().unwrap().len();
    for path in ["/", "relative", "/a/../b", "/a/./b"] {
        assert!(read.discover_active_in(path).await.is_err());
    }
    assert!(read.project_directories("../foreign").await.is_err());
    assert_eq!(api.data.requests.lock().unwrap().len(), count);
}
#[tokio::test]
async fn admitted_sessions_recover_offline_tails_despite_unavailable_project_inventory() {
    let f = Fixture::new(16 * 1024 * 1024).await;
    let api = Api::new(&f).await;
    let mut runtime = RunningCodexCapture::start(prepare(&f, &api)).unwrap();
    tokio::time::timeout(Duration::from_secs(15), async {
        while !(recorded(&f, A, "workspace A transcript").await
            && recorded(&f, B, "workspace B transcript").await)
        {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    shutdown(&mut runtime).await;
    api.data.inventory_mode.store(1, Ordering::SeqCst);
    api.data.tail.store(true, Ordering::SeqCst);
    let mut runtime = RunningCodexCapture::start(prepare(&f, &api)).unwrap();
    let completed = tokio::time::timeout(Duration::from_secs(15), async {
        while !(recorded(&f, A, "offline discovery tail").await
            && recorded(&f, B, "offline discovery tail").await)
        {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await;
    shutdown(&mut runtime).await;
    assert!(
        completed.is_ok(),
        "new-directory failure discarded retained recovery candidates"
    );
    assert!(f.spool.pending().unwrap().is_empty());
}
