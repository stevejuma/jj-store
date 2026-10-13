use super::*;
use serde_json::{Value, json};
fn jj(root: &Path, workspace: Option<&Path>, args: &[&str]) -> Value {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jj"));
    command
        .current_dir(root)
        .env("JJ_CONFIG", root.join("jj.toml"))
        .args(["--no-pager", "--color=never"]);
    if let Some(workspace) = workspace {
        command.arg("-R").arg(workspace);
    }
    command.args(args);
    let text = success(command.output().unwrap());
    if text.trim().is_empty() {
        Value::Null
    } else {
        serde_json::from_str(&text).unwrap()
    }
}
fn host(root: &Path, path: &Path) -> Child {
    command()
        .current_dir(root)
        .env("JJ_CONFIG", root.join("jj.toml"))
        .args([
            "workflow-host",
            "--config",
            path.to_str().unwrap(),
            "--once",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}

#[test]
fn executor_parent_loss_stops_the_owned_process_group() {
    let root = tempfile::tempdir().unwrap();
    let marker = root.path().join("late");
    let mut child = Command::new(env!("CARGO_BIN_EXE_jj"))
        .args([
            jj_store_cli::workflow_host::worker_entry_marker(),
            "30",
            "/bin/sh",
            "-c",
        ])
        .arg(format!("sleep 2; printf late > {}", marker.display()))
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    std::thread::sleep(Duration::from_millis(100));
    drop(child.stdin.take());
    assert!(!child.wait().unwrap().success());
    std::thread::sleep(Duration::from_secs(2));
    assert!(!marker.exists());
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires the pinned embedded RustFS runtime"]
async fn two_hosts_execute_one_pinned_run_and_preserve_history_after_restart() {
    use std::os::unix::fs::MetadataExt as _;
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("server");
    let runtime = fs::canonicalize(
        std::env::var_os("JJ_STORE_RUSTFS_RUNTIME").expect("embedded runtime required"),
    )
    .unwrap();
    success(
        command()
            .args([
                "init",
                "--local",
                "--state-dir",
                state.to_str().unwrap(),
                "--rustfs-executable",
                runtime.to_str().unwrap(),
            ])
            .output()
            .unwrap(),
    );
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let identity =
        jj_store_server::installation::Identity::load(&state.join("installation.json")).unwrap();
    let actor =
        identity.principal(format!("unix-uid:{}", fs::metadata(&state).unwrap().uid()).as_bytes());
    let config = state.join("server.toml");
    let text = fs::read_to_string(&config)
        .unwrap()
        .replace("127.0.0.1:7443", &address.to_string());
    fs::write(&config, format!("workflow_hosts = [\"{actor}\"]\n{text}")).unwrap();
    fs::write(
        root.path().join("jj.toml"),
        "[user]\nname='Workflow acceptance'\nemail='workflow@example.invalid'\n",
    )
    .unwrap();
    let mut running = start_local_trusted_server(root.path(), &config);
    let socket = state.join("native.sock");
    let code = root.path().join("code");
    let home = root.path().join("home");
    let mut repos = Vec::new();
    for (name, workspace) in [("code", &code), ("home", &home)] {
        let cache = root.path().join(format!("{name}-cache"));
        let client = root.path().join(format!("{name}-client"));
        let result = jj(
            root.path(),
            None,
            &[
                "store",
                "--format=json",
                "init",
                workspace.to_str().unwrap(),
                "--name",
                name,
                "--cache",
                cache.to_str().unwrap(),
                "--state-dir",
                client.to_str().unwrap(),
                "--unix-socket",
                socket.to_str().unwrap(),
            ],
        );
        repos.push(result["repository"].as_str().unwrap().to_owned());
    }
    jj(
        root.path(),
        Some(&home),
        &[
            "project",
            "--format=json",
            "create",
            "--request-id",
            &"10".repeat(16),
            "--title",
            "Automation",
        ],
    );
    fs::write(code.join("source.txt"), "pinned source\n").unwrap();
    let mut snapshot = Command::new(env!("CARGO_BIN_EXE_jj"));
    snapshot
        .current_dir(&code)
        .env("JJ_CONFIG", root.path().join("jj.toml"))
        .args(["describe", "-m", "Source revision"]);
    success(snapshot.output().unwrap());
    let definition = root.path().join("workflow.yaml");
    fs::write(&definition,format!("name: Build\nenabled: true\nrepository: {}\ntrigger:\n  on: bookmark_changed\n  bookmark: m?in\nconditions:\n  revset: all()\n  authors: [workflow@example.invalid]\n  text:\n    operator: contains\n    value: Trigger\nactions:\n  - id: build\n    executor:\n      type: command\n      name: build\n    timeout_seconds: 30\n  - id: verify\n    executor:\n      type: command\n      name: verify\n    timeout_seconds: 30\n  - id: publish\n    executor:\n      type: command\n      name: publish\n    timeout_seconds: 30\n",repos[0])).unwrap();
    jj(
        root.path(),
        Some(&home),
        &[
            "workflow",
            "--format=json",
            "put",
            "--request-id",
            &"11".repeat(16),
            definition.to_str().unwrap(),
        ],
    );
    let mut configs = Vec::new();
    for name in ["host-a", "host-b"] {
        let path = root.path().join(format!("{name}.json"));
        fs::write(&path,serde_json::to_vec(&json!({"version":1,"host":name,"principal":actor,"home":home,"state_dir":root.path().join(name),"jj_executable":env!("CARGO_BIN_EXE_jj"),"allowed_owners":[actor],"sources":[{"repository":repos[0],"workspace":code,"target_id":null}],"commands":{"build":{"program":"/bin/sh","args":["-c","cat source.txt; printf artifact > result.txt"],"environment":{"PATH":"/usr/bin:/bin"},"artifacts":["result.txt"]},"verify":{"program":"/bin/sh","args":["-c","set -eu; test -f result.txt; printf verified >> result.txt"],"environment":{"PATH":"/usr/bin:/bin"}},"publish":{"program":"/bin/sh","args":["-c","set -eu; grep -q '^artifactverified$' result.txt; printf 'shared artifact\n'"],"environment":{"PATH":"/usr/bin:/bin"}}},"agent_endpoint":null})).unwrap()).unwrap();
        configs.push(path);
    }
    // Establish each observer's initial source cursor, then publish a new revision.
    for config in &configs {
        assert!(host(root.path(), config).wait().unwrap().success());
    }
    fs::write(code.join("source.txt"), "next pinned source\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_jj"))
        .current_dir(&code)
        .env("JJ_CONFIG", root.path().join("jj.toml"))
        .args(["describe", "-m", "Trigger workflow"])
        .output()
        .unwrap();
    success(output);
    let output = Command::new(env!("CARGO_BIN_EXE_jj"))
        .current_dir(&code)
        .env("JJ_CONFIG", root.path().join("jj.toml"))
        .args(["bookmark", "set", "main", "-r", "@"])
        .output()
        .unwrap();
    success(output);
    let before = jj(
        root.path(),
        Some(&code),
        &["workflow", "--format=json", "events"],
    );
    let a = host(root.path(), &configs[0]);
    let b = host(root.path(), &configs[1]);
    for child in [a, b] {
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let page = jj(
        root.path(),
        Some(&home),
        &["workflow", "--format=json", "list"],
    );
    let workflow = page["workflows"][0]["id"].as_str().unwrap();
    let runs = jj(
        root.path(),
        Some(&home),
        &["workflow", "--format=json", "runs", "--workflow", workflow],
    );
    assert_eq!(runs["runs"].as_array().unwrap().len(), 1, "{runs}");
    let run = runs["runs"][0]["id"].as_str().unwrap();
    let detail = jj(
        root.path(),
        Some(&home),
        &["workflow", "--format=json", "runs", "--run", run],
    );
    assert_eq!(
        runs["runs"][0]["attempts"][0]["status"], "completed",
        "{detail}"
    );
    assert!(
        detail["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .any(|e| e["data"]["type"] == "artifact")
    );
    let logs = detail["evidence"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|e| e["data"]["text"].as_str())
        .collect::<String>();
    assert_eq!(logs, "next pinned source\nshared artifact\n");
    let steps: Vec<_> = detail["evidence"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["data"]["type"] == "step")
        .map(|e| {
            (
                e["data"]["step"].as_str().unwrap(),
                e["data"]["status"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        steps,
        vec![
            ("build", "running"),
            ("build", "completed"),
            ("verify", "running"),
            ("verify", "completed"),
            ("publish", "running"),
            ("publish", "completed")
        ]
    );
    let http = protected_http_client().no_proxy().build().unwrap();
    let response = http
        .get(format!(
            "http://{address}/v1/repos/{}/workflows?run={run}",
            repos[1]
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(response.json::<Value>().await.unwrap(), detail);
    let response = http.post(format!("http://{address}/v1/repos/{}/workflows/commands",repos[1])).json(&json!({"version":1,"request_id":"99".repeat(16),"command":{"type":"evaluate","event":{}}})).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);
    let after = jj(
        root.path(),
        Some(&code),
        &["workflow", "--format=json", "events"],
    );
    assert_eq!(
        before["upper"], after["upper"],
        "source export must not publish source operations"
    );
    running.terminate();
    let mut restarted = start_local_trusted_server(root.path(), &config);
    let cold = jj(
        root.path(),
        Some(&home),
        &["workflow", "--format=json", "runs", "--run", run],
    );
    assert_eq!(cold, detail);
    restarted.terminate();
}
