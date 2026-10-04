use super::jj;
use jj_store_server::{config::Config, installation, process::PreparedServer};
use serde_json::{Value, json};
use std::{fs, sync::Arc, time::Duration};

async fn server(
    config: &Config,
) -> (
    tokio::sync::oneshot::Sender<()>,
    tokio::task::JoinHandle<()>,
) {
    let prepared = PreparedServer::prepare_with_browser(
        config,
        true,
        None,
        None,
        None,
        Some(Arc::new(jj_store_cli::local_browser::LocalFolders)),
    )
    .await
    .unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let running = tokio::spawn(async move {
        prepared
            .run(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    (stop, running)
}
async fn request(
    client: &reqwest::Client,
    address: std::net::SocketAddr,
    suffix: &str,
    body: Option<&Value>,
    cancel: bool,
) -> (u16, Value) {
    let url = format!("http://{address}/v1{suffix}");
    let mut call = if body.is_some() || cancel {
        client.post(url)
    } else {
        client.get(url)
    }
    .header("x-jj-store-protocol", "1");
    if let Some(body) = body {
        call = call.json(body);
    }
    let response = call.send().await.unwrap();
    let status = response.status().as_u16();
    (status, response.json().await.unwrap())
}
#[tokio::test]
async fn managed_local_http_activates_without_subscribers_resumes_and_captures_tasks() {
    let temp = tempfile::tempdir().unwrap();
    let folder = temp.path().join("repo");
    jj(
        temp.path(),
        &["git", "init", "--colocate", folder.to_str().unwrap()],
    );
    fs::write(folder.join("hello.txt"), "First body\n").unwrap();
    jj(&folder, &["describe", "-m", "First"]);
    jj(&folder, &["bookmark", "create", "main", "another"]);
    let operation = jj(&folder, &["op", "log", "--no-graph", "-n", "1", "-T", "id"]);
    let identity = installation::initialize_filesystem(&temp.path().join("state")).unwrap();
    let mut config = Config::load(&identity.config).unwrap();
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap();
    config.bind = port;
    config.public_url = format!("http://{port}/");
    let (stop, running) = server(&config).await;
    let client = reqwest::Client::new();
    let (status, registered) = request(&client, port, "/local/repositories", Some(&json!({"version":1,"request_id":"36363636363636363636363636363636","expected_revision":0,"path":folder,"name":"Managed code"})), false).await;
    assert_eq!(status, 200, "{registered}");
    let id = registered["binding"]["repository"].as_str().unwrap();
    let base = format!("/local/repos/{id}");
    let (status, caps) = request(&client, port, &format!("{base}/capabilities"), None, false).await;
    assert_eq!(status, 200);
    assert_eq!(caps["automatic_code_index"], true);
    let before = super::preparation_acceptance::files(&folder.join(".jj/repo"));
    let (status, _) = request(&client, port, &format!("{base}/snapshot"), None, false).await;
    assert_eq!(status, 200);
    let refs = json!({"operation":operation,"cursor":null,"request":{"kind":"references","query":{"operation":operation,"kind":"bookmarks","query":"","exact":null,"limit":1,"anchor":null,"direction":"forward"}}});
    let (status, refs) = request(&client, port, &format!("{base}/query"), Some(&refs), false).await;
    assert_eq!(status, 200, "{refs}");
    assert!(
        refs["next_cursor"].is_string(),
        "multiple bookmarks remain paged"
    );
    let log = |operation: &str| json!({"operation":operation,"cursor":null,"request":{"kind":"graph","query":[{"graph":"log","revset":"all()","workspace":null,"user_email":"","at_time_millis":0},{"limit":1,"after":null}]}});
    // No SSE/browser is alive. The retained repository monitor still prepares the opened selection.
    tokio::time::sleep(Duration::from_secs(3)).await;
    let (status, first_log) = request(
        &client,
        port,
        &format!("{base}/query"),
        Some(&log(&operation)),
        false,
    )
    .await;
    assert_eq!(
        status, 200,
        "first open prepared before any history request: {first_log}"
    );
    let mut samples = vec![];
    let (status, started) = request(
        &client,
        port,
        &format!("{base}/code-index/tasks"),
        Some(&json!({"operation":operation})),
        false,
    )
    .await;
    assert_eq!(status, 200, "{started}");
    assert_eq!(started["status"], "ready");
    samples.push(json!({"method":"post","path":"/v1/local/repos/{repo}/code-index/tasks","status":200,"media":"application/json","data":started}));
    let task = started["id"].as_str().unwrap();
    let (status, report) = request(
        &client,
        port,
        &format!("{base}/code-index/tasks/{task}"),
        None,
        false,
    )
    .await;
    assert_eq!(status, 200);
    samples.push(json!({"method":"get","path":"/v1/local/repos/{repo}/code-index/tasks/{task}","status":200,"media":"application/json","data":report}));
    let (status, report) = request(
        &client,
        port,
        &format!("{base}/code-index/tasks/{task}/cancel"),
        None,
        true,
    )
    .await;
    assert_eq!(status, 200);
    samples.push(json!({"method":"post","path":"/v1/local/repos/{repo}/code-index/tasks/{task}/cancel","status":200,"media":"application/json","data":report}));
    assert_eq!(
        super::preparation_acceptance::files(&folder.join(".jj/repo")),
        before
    );
    jj(&folder, &["new", "-m", "External update"]);
    let next = jj(&folder, &["op", "log", "--no-graph", "-n", "1", "-T", "id"]);
    tokio::time::sleep(Duration::from_secs(3)).await;
    let (status, updated) = request(
        &client,
        port,
        &format!("{base}/query"),
        Some(&log(&next)),
        false,
    )
    .await;
    assert_eq!(status, 200, "event-driven update: {updated}");
    let (status, old) = request(
        &client,
        port,
        &format!("{base}/query"),
        Some(&log(&operation)),
        false,
    )
    .await;
    assert_eq!(status, 200, "{old}");
    assert_eq!(old["snapshot"]["operation"], operation);
    stop.send(()).unwrap();
    running.await.unwrap();
    let (stop, running) = server(&config).await;
    tokio::time::sleep(Duration::from_secs(3)).await;
    let (status, restarted) = request(
        &client,
        port,
        &format!("{base}/query"),
        Some(&log(&next)),
        false,
    )
    .await;
    assert_eq!(status, 200, "automatic restart activation: {restarted}");
    let (status, _) = request(
        &client,
        port,
        &format!("{base}/code-index/tasks/{task}"),
        None,
        false,
    )
    .await;
    assert_eq!(status, 410);
    stop.send(()).unwrap();
    running.await.unwrap();
    if let Some(path) = std::env::var_os("JJ_STORE_MANAGED_LOCAL_CAPTURE") {
        fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
}
