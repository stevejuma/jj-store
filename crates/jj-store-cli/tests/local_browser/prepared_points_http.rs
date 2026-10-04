use super::{ReadBinding, jj};
use jj_store_jj::local_browser as q;
use jj_store_server::{config::Config, installation, process::PreparedServer};
use serde_json::{Value, json};
use std::{fs, sync::Arc, time::Duration};

#[tokio::test]
async fn local_point_http_uses_ready_membership_and_only_falls_back_when_cold() {
    let temp = tempfile::tempdir().unwrap();
    let folder = temp.path().join("repo");
    jj(
        temp.path(),
        &["git", "init", "--colocate", folder.to_str().unwrap()],
    );
    fs::write(folder.join("file.txt"), "historical body\n").unwrap();
    jj(&folder, &["describe", "-m", "Historical"]);
    let commit = jj(
        &folder,
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    );
    for n in 0..12 {
        jj(&folder, &["new", "-m", &format!("Later {n}")]);
    }
    let operation = jj(&folder, &["op", "log", "--no-graph", "-n", "1", "-T", "id"]);
    let installed = installation::initialize_filesystem(&temp.path().join("state")).unwrap();
    let mut config = Config::load(&installed.config).unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    config.bind = address;
    config.public_url = format!("http://{address}/");
    let prepared = PreparedServer::prepare_with_browser(
        &config,
        true,
        None,
        None,
        None,
        Some(Arc::new(jj_store_cli::local_browser::LocalFolders)),
    )
    .await
    .unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        prepared
            .run(async {
                let _ = stopped.await;
            })
            .await
    });
    let client = reqwest::Client::new();
    let post = |suffix: &str, value: Value| {
        client
            .post(format!("http://{address}/v1{suffix}"))
            .header("x-jj-store-protocol", "1")
            .json(&value)
    };
    let registered: Value = post(
        "/local/repositories",
        json!({
            "version":1, "request_id":"45454545454545454545454545454545",
            "expected_revision":0, "path":folder, "name":"Point reader fixture"
        }),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    let id = registered["binding"]["repository"].as_str().unwrap();
    let query_path = format!("/local/repos/{id}/query");
    let summary = json!({"operation":operation,"cursor":null,
        "request":{"kind":"commit_summary","query":{"commit":commit}}});
    // No ready task exists: the ordinary content path is independently usable.
    let cold: Value = post(&query_path, summary.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    let references = json!({"operation":operation,"cursor":null,
        "request":{"kind":"references","query":{"operation":operation,
            "query":"","exact":null,"kind":"changes","limit":1,
            "anchor":null,"direction":"forward"}}});
    let cold_references = post(&query_path, references.clone()).send().await.unwrap();
    assert_eq!(cold_references.status(), reqwest::StatusCode::CONFLICT);
    let cold_references: Value = cold_references.json().await.unwrap();
    assert_eq!(cold_references["code"], "index_rebuild_required");
    assert_eq!(cold_references["local_snapshot"]["operation"], operation);
    let task: Value = post(
        &format!("/local/repos/{id}/index/tasks"),
        json!({"operation":operation}),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    let task_id = task["id"].as_str().unwrap();
    tokio::time::timeout(Duration::from_secs(15), async {
        loop {
            let task: Value = client
                .get(format!(
                    "http://{address}/v1/local/repos/{id}/index/tasks/{task_id}"
                ))
                .header("x-jj-store-protocol", "1")
                .send()
                .await
                .unwrap()
                .error_for_status()
                .unwrap()
                .json()
                .await
                .unwrap();
            if task["status"] != "running" && task["status"] != "canceling" {
                assert_eq!(task["status"], "ready", "{task}");
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    let warm: Value = post(&query_path, summary.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(cold["snapshot"], warm["snapshot"]);
    assert_eq!(cold["data"]["data"]["data"], warm["data"]["data"]["data"]);
    let stats = |value: &Value| {
        value["data"]["data"]["stats"]["metadata_nodes"]
            .as_u64()
            .unwrap()
    };
    assert!(stats(&warm) < stats(&cold), "cold={cold}, warm={warm}");
    assert!(stats(&warm) <= 3, "{warm}");
    let first: Value = post(&query_path, references.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(
        first["data"]["data"]["references"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert!(stats(&first) <= 3, "{first}");
    assert!(first["next_cursor"].is_string());
    let mut resumed = references.clone();
    resumed["cursor"] = first["next_cursor"].clone();
    let next: Value = post(&query_path, resumed)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(next["snapshot"], first["snapshot"]);
    assert_ne!(
        next["data"]["data"]["references"],
        first["data"]["data"]["references"]
    );
    let mut backward = references;
    backward["cursor"] = next["previous_cursor"].clone();
    let previous: Value = post(&query_path, backward)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(
        previous["data"]["data"]["references"],
        first["data"]["data"]["references"]
    );
    // Even a valid replacement protobuf cannot trigger ordinary fallback from
    // a corrupt ready generation. SimpleOpStore does not hash filenames.
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(folder.join(".jj/repo")));
    let operation_id = operation.parse().unwrap();
    let q::Response::Operation(detail) = q::Executor::new(1)
        .unwrap()
        .read(
            access.clone(),
            operation_id,
            q::Request::Operation {
                operation: operation_id,
            },
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let q::ObjectKey::View(view) = detail.view.key else {
        panic!()
    };
    let path = access
        .metadata_path()
        .join("op_store/views")
        .join(view.to_string());
    let captured = fs::read(&path).unwrap();
    let replacement = fs::read_dir(path.parent().unwrap())
        .unwrap()
        .find_map(|entry| {
            let bytes = fs::read(entry.unwrap().path()).unwrap();
            (bytes != captured).then_some(bytes)
        })
        .unwrap();
    fs::write(&path, replacement).unwrap();
    let rejected = post(&query_path, summary).send().await.unwrap();
    assert!(!rejected.status().is_success());
    let rejected: Value = rejected.json().await.unwrap();
    assert_eq!(rejected["code"], "corrupt_history");
    fs::write(path, captured).unwrap();
    stop.send(()).unwrap();
    server.await.unwrap().unwrap();
}
