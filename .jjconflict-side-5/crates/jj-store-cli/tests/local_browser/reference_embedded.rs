use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::Cancellation};
use jj_store_server::{config::Config, installation, process::PreparedServer};
use serde_json::{Value, json};
use std::{fs, sync::Arc, time::Duration};

#[tokio::test]
async fn local_embedded_references_prepare_exact_source_and_preserve_outer_cursor_authority() {
    let temp = tempfile::tempdir().unwrap();
    let folder = temp.path().join("repo");
    jj(
        temp.path(),
        &["git", "init", "--colocate", folder.to_str().unwrap()],
    );
    for n in 0..12 {
        jj(&folder, &["new", "-m", &format!("Source needle {n}")]);
    }
    jj(&folder, &["bookmark", "create", "source"]);
    let source = jj(&folder, &["op", "log", "--no-graph", "-n", "1", "-T", "id"]);
    jj(&folder, &["new", "-m", "Outer selection"]);
    let outer = jj(&folder, &["op", "log", "--no-graph", "-n", "1", "-T", "id"]);
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
        json!({"version":1,
        "request_id":"51515151515151515151515151515151", "expected_revision":0,
        "path":folder, "name":"Embedded references"}),
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
    let path = format!("/local/repos/{id}/query");
    let query = json!({"operation":outer,"cursor":null,"request":{"kind":"references",
        "query":{"operation":source,"query":"","exact":null,"kind":"changes",
            "limit":1,"anchor":null,"direction":"forward"}}});
    let cold = post(&path, query.clone()).send().await.unwrap();
    assert_eq!(cold.status(), reqwest::StatusCode::CONFLICT);
    let cold: Value = cold.json().await.unwrap();
    assert_eq!(cold["code"], "index_rebuild_required");
    assert_eq!(cold["local_snapshot"]["operation"], source);
    let prepare = |operation: &str| {
        post(
            &format!("/local/repos/{id}/index/tasks"),
            json!({"operation":operation}),
        )
    };
    let task: Value = prepare(&source)
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
    let warm: Value = post(&path, query.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(warm["snapshot"]["operation"], outer);
    assert_eq!(warm["data"]["data"]["operation"], source);
    assert_eq!(
        warm["data"]["data"]["references"].as_array().unwrap().len(),
        1
    );
    let cursor = warm["next_cursor"].as_str().unwrap();
    let mut continued = query.clone();
    continued["cursor"] = json!(cursor);
    let next: Value = post(&path, continued.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(next["snapshot"], warm["snapshot"]);
    assert_ne!(
        next["data"]["data"]["references"],
        warm["data"]["data"]["references"]
    );
    continued["cursor"] = next["previous_cursor"].clone();
    let previous: Value = post(&path, continued.clone())
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
        warm["data"]["data"]["references"]
    );
    for field in ["outer", "source", "filter"] {
        let mut wrong = query.clone();
        wrong["cursor"] = json!(cursor);
        match field {
            "outer" => wrong["operation"] = json!(source),
            "source" => wrong["request"]["query"]["operation"] = json!(outer),
            _ => wrong["request"]["query"]["query"] = json!("needle"),
        }
        assert_eq!(
            post(&path, wrong).send().await.unwrap().status(),
            reqwest::StatusCode::BAD_REQUEST
        );
    }
    // A cold outer generation cannot hide a warm source generation.
    let mut outer_query = query.clone();
    outer_query["request"]["query"]["operation"] = json!(outer);
    let cold_outer = post(&path, outer_query).send().await.unwrap();
    assert_eq!(cold_outer.status(), reqwest::StatusCode::CONFLICT);
    let cold_outer: Value = cold_outer.json().await.unwrap();
    assert_eq!(cold_outer["local_snapshot"]["operation"], outer);
    // Prepare another current branch. Its retention and warm cache do not make
    // it an ancestor of the independently selected outer operation.
    jj(
        &folder,
        &["--at-op", &source, "new", "-m", "Sibling branch"],
    );
    let sibling = fs::read_dir(folder.join(".jj/repo/op_heads/heads"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_owned())
        .find(|id| id != &outer)
        .unwrap();
    // Use the executor to obtain independent prepared handles and a deliberately
    // tiny ordinary history allowance. Neither path needs the source JJ index.
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(folder.join(".jj/repo")));
    let executor = q::Executor::new(1).unwrap();
    let output = temp.path().join("source-index");
    fs::create_dir(&output).unwrap();
    let prepared_source = Arc::new(
        executor
            .prepare_index(
                access.clone(),
                source.parse().unwrap(),
                output,
                32 * 1024 * 1024,
                q::preparation::Limits::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    let small = q::Executor::with_workers(
        jj_store_jj::query::workers::Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 4,
            ..Default::default()
        },
    );
    for kind in [
        q::references::Kind::Changes,
        q::references::Kind::All,
        q::references::Kind::Bookmarks,
    ] {
        for text in ["", "needle"] {
            for direction in [q::Direction::Forward, q::Direction::Backward] {
                for anchor in [
                    None,
                    Some(
                        if kind == q::references::Kind::Bookmarks {
                            "bookmark:source"
                        } else {
                            "change:8"
                        }
                        .to_owned(),
                    ),
                ] {
                    let r = q::references::Request {
                        operation: source.parse().unwrap(),
                        query: text.into(),
                        exact: None,
                        kind,
                        limit: 1,
                        anchor,
                        direction,
                    };
                    let q::Response::References(expected) = executor
                        .read(
                            access.clone(),
                            r.operation,
                            q::Request::References(r.clone()),
                        )
                        .await
                        .unwrap()
                    else {
                        panic!()
                    };
                    let request = q::Request::References(r);
                    let actual = if request.can_use_prepared_index() {
                        small
                            .read_prepared_at(
                                access.clone(),
                                outer.parse().unwrap(),
                                prepared_source.clone(),
                                request,
                            )
                            .await
                    } else {
                        small
                            .read(access.clone(), outer.parse().unwrap(), request)
                            .await
                    };
                    let q::Response::References(actual) = actual.unwrap() else {
                        panic!()
                    };
                    assert_eq!(actual.references, expected.references);
                    assert_eq!(actual.previous, expected.previous);
                    assert_eq!(actual.next, expected.next);
                    assert_eq!(actual.operation, source.parse().unwrap());
                    assert!(actual.stats.metadata_nodes <= 12, "{:?}", actual.stats);
                }
            }
        }
    }
    for kind in [
        q::references::Kind::Bookmarks,
        q::references::Kind::Workspaces,
    ] {
        let r = q::references::Request {
            operation: source.parse().unwrap(),
            query: "".into(),
            exact: None,
            kind,
            limit: 1,
            anchor: None,
            direction: q::Direction::Forward,
        };
        let q::Response::References(expected) = executor
            .read(
                access.clone(),
                r.operation,
                q::Request::References(r.clone()),
            )
            .await
            .unwrap()
        else {
            panic!()
        };
        let q::Response::References(actual) = small
            .read(
                access.clone(),
                outer.parse().unwrap(),
                q::Request::References(r),
            )
            .await
            .unwrap()
        else {
            panic!()
        };
        assert_eq!(actual.references, expected.references);
        assert_eq!(actual.operation, source.parse().unwrap());
        assert!(actual.stats.metadata_nodes <= 12, "{:?}", actual.stats);
    }
    // A newer prepared selection remains unavailable from the older outer.
    fs::create_dir(temp.path().join("outer-index")).unwrap();
    let prepared_outer = Arc::new(
        executor
            .prepare_index(
                access.clone(),
                outer.parse().unwrap(),
                temp.path().join("outer-index"),
                32 * 1024 * 1024,
                q::preparation::Limits::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    let request = |operation| {
        q::Request::References(q::references::Request {
            operation,
            query: "".into(),
            exact: None,
            kind: q::references::Kind::Changes,
            limit: 1,
            anchor: None,
            direction: q::Direction::Forward,
        })
    };
    assert!(matches!(
        executor
            .read_prepared_at(
                access.clone(),
                source.parse().unwrap(),
                prepared_outer,
                request(outer.parse().unwrap())
            )
            .await,
        Err(q::Error::NotCommitted)
    ));
    fs::create_dir(temp.path().join("sibling-index")).unwrap();
    let prepared_sibling = Arc::new(
        executor
            .prepare_index(
                access.clone(),
                sibling.parse().unwrap(),
                temp.path().join("sibling-index"),
                32 * 1024 * 1024,
                q::preparation::Limits::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    assert!(matches!(
        executor
            .read_prepared_at(
                access.clone(),
                outer.parse().unwrap(),
                prepared_sibling,
                request(sibling.parse().unwrap())
            )
            .await,
        Err(q::Error::NotCommitted)
    ));
    // Explicitly locate the other head without reconciling it.
    let sibling: jj_store_model::OperationId = sibling.parse().unwrap();
    assert!(matches!(
        executor
            .validate_reference_operation(access.clone(), outer.parse().unwrap(), sibling)
            .await,
        Err(q::Error::NotCommitted)
    ));
    assert!(matches!(
        executor
            .validate_reference_operation(
                access.clone(),
                outer.parse().unwrap(),
                "ff".repeat(64).parse().unwrap()
            )
            .await,
        Err(q::Error::NotCommitted)
    ));
    // Operation authority must remain readable with every Git body unavailable.
    let objects = folder.join(".git/objects");
    let held_objects = folder.join("held-objects");
    fs::rename(&objects, &held_objects).unwrap();
    fs::create_dir(&objects).unwrap();
    small
        .validate_reference_operation(
            access.clone(),
            outer.parse().unwrap(),
            source.parse().unwrap(),
        )
        .await
        .unwrap();
    assert!(matches!(
        executor
            .validate_reference_operation(access.clone(), outer.parse().unwrap(), sibling)
            .await,
        Err(q::Error::NotCommitted)
    ));
    fs::remove_dir(&objects).unwrap();
    fs::rename(&held_objects, &objects).unwrap();
    let tight = q::Executor::with_workers(
        jj_store_jj::query::workers::Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 1,
            ..Default::default()
        },
    );
    assert!(matches!(
        tight
            .validate_reference_operation(access, outer.parse().unwrap(), source.parse().unwrap())
            .await,
        Err(q::Error::Limit)
    ));
    stop.send(()).unwrap();
    server.await.unwrap().unwrap();
}
