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
    let historical_operation = jj(&folder, &["op", "log", "--no-graph", "-n", "1", "-T", "id"]);
    let commit = jj(
        &folder,
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    );
    for n in 0..12 {
        jj(&folder, &["new", "-m", &format!("Later {n}")]);
    }
    let operation = jj(&folder, &["op", "log", "--no-graph", "-n", "1", "-T", "id"]);
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(folder.join(".jj/repo")));
    let q::Response::Operation(selected_detail) = q::Executor::new(1)
        .unwrap()
        .read(
            access.clone(),
            operation.parse().unwrap(),
            q::Request::Operation {
                operation: operation.parse().unwrap(),
            },
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let q::ObjectData::Operation(selected_metadata) = selected_detail.operation.data else {
        panic!()
    };
    let operation_comparison = json!({"operation":operation,"cursor":null,
        "request":{"kind":"operation_diff","query":{"operation":operation,"parent":null}}});
    let operation_changes = json!({"operation":operation,"cursor":null,
        "request":{"kind":"operation_changes","query":{"operation":operation,
            "parent":selected_metadata.parents[0],"limit":1,"anchor":null,"direction":"forward"}}});
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
    let code_summary = json!({"operation":operation,"cursor":null,
        "request":{"kind":"code_summary","query":{"limit":1}}});
    let code: Value = post(&query_path, code_summary.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(code["snapshot"]["operation"], operation);
    assert_eq!(code["data"]["kind"], "code_summary");
    assert_eq!(code["data"]["data"]["commits"], 1);
    assert_eq!(code["data"]["data"]["commits_partial"], true);
    assert_eq!(
        code["data"]["data"]["latest"]["description"]
            .as_str()
            .unwrap()
            .trim(),
        "Later 11"
    );
    assert!(code["next_cursor"].is_null());
    assert!(code["previous_cursor"].is_null());
    if let Some(path) = std::env::var_os("JJ_STORE_CODE_SUMMARY_LOCAL_CAPTURE") {
        fs::write(path, serde_json::to_vec(&vec![json!({"path":"/v1/local/repos/{repo}/query","method":"post","status":200,"media":"application/json","data":code})]).unwrap()).unwrap();
    }
    for limit in [0, 257] {
        let mut invalid = code_summary.clone();
        invalid["request"]["query"]["limit"] = json!(limit);
        assert_eq!(
            post(&query_path, invalid)
                .send()
                .await
                .unwrap()
                .status()
                .as_u16(),
            413
        );
    }
    let mut invalid = code_summary.clone();
    invalid["request"]["query"]["unknown"] = json!(1);
    assert_eq!(
        post(&query_path, invalid)
            .send()
            .await
            .unwrap()
            .status()
            .as_u16(),
        400
    );
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
    let mut filtered_references = references.clone();
    filtered_references["request"]["query"]["query"] = json!("Later");
    let filtered_cold = post(&query_path, filtered_references.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(filtered_cold.status(), reqwest::StatusCode::CONFLICT);
    let filtered_cold: Value = filtered_cold.json().await.unwrap();
    assert_eq!(filtered_cold["code"], "index_rebuild_required");
    assert_eq!(filtered_cold["local_snapshot"]["operation"], operation);
    let operation_prefix = json!({"operation":operation,"cursor":null,
        "request":{"kind":"graph","query":[{"graph":"operations_filtered",
            "query":format!("from:{}",&historical_operation[..9])},{"limit":1,"after":null}]}});
    let cold_prefix = post(&query_path, operation_prefix.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(cold_prefix.status(), reqwest::StatusCode::CONFLICT);
    let cold_prefix: Value = cold_prefix.json().await.unwrap();
    assert_eq!(cold_prefix["code"], "index_rebuild_required");
    assert_eq!(cold_prefix["local_snapshot"]["operation"], operation);
    let child_operation = json!({"operation":operation,"cursor":null,
        "request":{"kind":"graph","query":[{"graph":"operations_filtered","query":"from:@-+"},
            {"limit":1,"after":null}]}});
    let child_cold: Value = post(&query_path, child_operation.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(child_cold["snapshot"]["operation"], operation);
    assert_eq!(
        child_cold["data"]["data"]["data"]["nodes"][0]["id"],
        operation
    );
    assert!(
        child_cold["data"]["data"]["stats"]["metadata_nodes"]
            .as_u64()
            .unwrap()
            <= 8,
        "{child_cold}"
    );
    for query in [&operation_comparison, &operation_changes] {
        let cold = post(&query_path, query.clone()).send().await.unwrap();
        assert_eq!(cold.status(), reqwest::StatusCode::CONFLICT);
        let cold: Value = cold.json().await.unwrap();
        assert_eq!(cold["code"], "index_rebuild_required");
        assert_eq!(cold["local_snapshot"]["operation"], operation);
    }
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
    let prefix: Value = post(&query_path, operation_prefix)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(prefix["snapshot"]["operation"], operation);
    assert_eq!(prefix["data"]["data"]["data"]["graph"], "operations");
    assert_eq!(
        prefix["data"]["data"]["data"]["nodes"][0]["id"],
        historical_operation
    );
    assert!(stats(&prefix) <= 8, "{prefix}");
    let child: Value = post(&query_path, child_operation)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(child["snapshot"]["operation"], operation);
    assert_eq!(child["data"]["data"]["data"]["nodes"][0]["id"], operation);
    assert!(stats(&child) <= 8, "{child}");
    let comparison: Value = post(&query_path, operation_comparison)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(comparison["snapshot"]["operation"], operation);
    assert_eq!(comparison["data"]["data"]["data"]["status"], "difference");
    assert_eq!(comparison["data"]["data"]["data"]["operation"], operation);
    assert!(stats(&comparison) <= 9, "{comparison}");
    let changes: Value = post(&query_path, operation_changes)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(changes["snapshot"]["operation"], operation);
    assert_eq!(
        changes["data"]["data"]["groups"].as_array().unwrap().len(),
        1
    );
    assert!(stats(&changes) <= 9, "{changes}");
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
    let filtered_first: Value = post(&query_path, filtered_references.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(filtered_first["snapshot"]["operation"], operation);
    assert!(stats(&filtered_first) <= 3, "{filtered_first}");
    assert!(filtered_first["next_cursor"].is_string());
    let mut resumed = filtered_references.clone();
    resumed["cursor"] = filtered_first["next_cursor"].clone();
    let filtered_next: Value = post(&query_path, resumed.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(filtered_next["snapshot"], filtered_first["snapshot"]);
    resumed["request"]["query"]["query"] = json!("Historical");
    assert_eq!(
        post(&query_path, resumed).send().await.unwrap().status(),
        reqwest::StatusCode::BAD_REQUEST
    );
    filtered_references["cursor"] = filtered_next["previous_cursor"].clone();
    let filtered_previous: Value = post(&query_path, filtered_references)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(
        filtered_previous["data"]["data"]["references"],
        filtered_first["data"]["data"]["references"]
    );
    // Even a valid replacement protobuf cannot trigger ordinary fallback from
    // a corrupt ready generation. SimpleOpStore does not hash filenames.
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
