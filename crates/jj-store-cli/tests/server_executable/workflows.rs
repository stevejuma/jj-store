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
    let mut detail = jj(
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
    // Save/reload the managed adapter through the same endpoints as Settings.
    // Neither listener nor native socket is replaced. An explicit retry below
    // also exercises busy-host deferral through a live three-action attempt.
    let pid = running.child.id();
    let inode = fs::metadata(&socket).unwrap().ino();
    let mut cfg: Value = serde_json::from_slice(&fs::read(&configs[0]).unwrap()).unwrap();
    let script = cfg["commands"]["build"]["args"][1]
        .as_str()
        .unwrap()
        .to_owned();
    cfg["commands"]["build"]["args"][1] = json!(format!("sleep 2; {script}"));
    // Managed runners need no per-repository mapping or manually chosen identity.
    cfg["version"] = json!(2);
    for key in ["principal", "home", "sources"] {
        cfg.as_object_mut().unwrap().remove(key);
    }

    let settings_url = format!("http://{address}/v1/local/settings");
    let reload_url = format!("http://{address}/v1/configuration/reload");
    for (index, enabled) in [false, true, false].into_iter().enumerate() {
        if index == 1 {
            // Normal installation workspace registration, independent of runner settings.
            for (repository, path) in [(&repos[0], &code), (&repos[1], &home)] {
                let url = format!("http://{address}/v1/repos/{repository}/local/workspaces");
                let before = http
                    .get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json::<Value>()
                    .await
                    .unwrap();
                let response = http
                    .post(format!("{url}/link"))
                    .json(&json!({"version":1,"expected_revision":before["revision"],"path":path}))
                    .send()
                    .await
                    .unwrap();
                assert_eq!(response.status(), reqwest::StatusCode::OK);
                let linked = response.json::<Value>().await.unwrap();
                assert!(
                    linked["links"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(
                            |link| link["path"] == json!(std::fs::canonicalize(path).unwrap())
                                && link["availability"] == "ready"
                        ),
                    "registered workspace snapshot: {linked}"
                );
            }
        }
        if index == 2 {
            let retry = root.path().join("retry.json");
            fs::write(&retry, serde_json::to_vec(&json!({"type":"retry","run":run,"expected":detail["runs"][0]["attempts"][0]["id"]})).unwrap()).unwrap();
            jj(
                root.path(),
                Some(&home),
                &[
                    "workflow",
                    "--format=json",
                    "command",
                    "--request-id",
                    &"95".repeat(16),
                    retry.to_str().unwrap(),
                ],
            );
            let mut busy = false;
            for _ in 0..500 {
                let current = http
                    .get(&settings_url)
                    .send()
                    .await
                    .unwrap()
                    .json::<Value>()
                    .await
                    .unwrap();
                busy = current["workflow_runtime"]["busy"] == true;
                if busy {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
            assert!(busy, "managed host did not claim explicit retry");
        }
        let first = http
            .get(&settings_url)
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();
        assert_eq!(first["workflow_runtime"]["available"], true);
        assert_eq!(first["workflow_runtime"]["principal"], json!(actor));
        let admitted = if index == 2 {
            json!([])
        } else {
            json!([actor])
        };
        let desired = json!({"admitted":admitted, "local":{"enabled":enabled,"configuration":cfg}});
        let response = http.post(&settings_url).json(&json!({"version":1,"expected_revision":first["revision"],"update":{"section":"workflow","configuration":desired}})).send().await.unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::OK);
        let saved = response.json::<Value>().await.unwrap();
        for key in ["principal", "home", "sources"] {
            assert!(
                saved["desired"]["workflow"]["local"]["configuration"]
                    .get(key)
                    .is_none()
            );
        }
        assert_eq!(
            saved["effective"]["workflow"],
            first["effective"]["workflow"]
        );
        let status = http
            .get(&reload_url)
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();
        let command = json!({"version":1,"request_id":format!("{:032x}",500+index),"expected_revision":status["saved_revision"],"domains":["workflows"]});
        let response = http.post(&reload_url).json(&command).send().await.unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::OK);
        let mut outcome = response.json::<Value>().await.unwrap();
        for _ in 0..200 {
            if outcome["state"] != "running" {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
            outcome = http
                .post(format!("{reload_url}/outcome"))
                .json(&command)
                .send()
                .await
                .unwrap()
                .json::<Value>()
                .await
                .unwrap();
        }
        if index == 2 {
            assert_eq!(outcome["state"], "deferred", "{outcome}");
            let retained = http
                .get(&settings_url)
                .send()
                .await
                .unwrap()
                .json::<Value>()
                .await
                .unwrap();
            assert_eq!(retained["effective"]["workflow"]["local"]["enabled"], true);
            for _ in 0..500 {
                let current = http
                    .get(&settings_url)
                    .send()
                    .await
                    .unwrap()
                    .json::<Value>()
                    .await
                    .unwrap();
                if current["workflow_runtime"]["busy"] == false {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
            let mut next = command.clone();
            next["request_id"] = json!("94".repeat(16));
            outcome = http
                .post(&reload_url)
                .json(&next)
                .send()
                .await
                .unwrap()
                .json::<Value>()
                .await
                .unwrap();
            for _ in 0..200 {
                if outcome["state"] != "running" {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
                outcome = http
                    .post(format!("{reload_url}/outcome"))
                    .json(&next)
                    .send()
                    .await
                    .unwrap()
                    .json::<Value>()
                    .await
                    .unwrap();
            }
        }
        assert_eq!(outcome["state"], "applied", "{outcome}");
        let effective = http
            .get(&settings_url)
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();
        assert_eq!(
            effective["effective"]["workflow"],
            saved["desired"]["workflow"]
        );
        assert_eq!(effective["workflow_runtime"]["running"], enabled);
        assert_eq!(running.child.id(), pid);
        assert_eq!(fs::metadata(&socket).unwrap().ino(), inode);
    }
    detail = jj(
        root.path(),
        Some(&home),
        &["workflow", "--format=json", "runs", "--run", run],
    );
    assert_eq!(detail["runs"][0]["attempts"].as_array().unwrap().len(), 2);
    assert_eq!(
        detail["runs"][0]["attempts"][1]["status"], "completed",
        "{detail}"
    );
    let denied = json!({"version":1,"request_id":"98".repeat(16),"command":{"type":"claim","run":run,"attempt":"97".repeat(16),"host":"host-a"}});
    let response = http
        .post(format!(
            "http://{address}/v1/repos/{}/workflows/commands",
            repos[1]
        ))
        .json(&denied)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::FORBIDDEN);
    let native_body = root.path().join("denied.json");
    fs::write(
        &native_body,
        serde_json::to_vec(&denied["command"]).unwrap(),
    )
    .unwrap();
    let response = Command::new(env!("CARGO_BIN_EXE_jj"))
        .current_dir(root.path())
        .env("JJ_CONFIG", root.path().join("jj.toml"))
        .args([
            "--no-pager",
            "--ignore-working-copy",
            "-R",
            home.to_str().unwrap(),
            "workflow",
            "command",
            "--request-id",
            &"96".repeat(16),
            native_body.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!response.status.success());
    assert!(
        String::from_utf8_lossy(&response.stderr).contains("denied"),
        "{}",
        String::from_utf8_lossy(&response.stderr)
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires real RustFS, Docker and qualified Forgejo Runner"]
async fn github_yaml_executor_matches_pinned_source_and_retains_retry_history() {
    use std::os::unix::fs::MetadataExt as _;
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("server");
    let runtime = fs::canonicalize(std::env::var_os("JJ_STORE_RUSTFS_RUNTIME").unwrap()).unwrap();
    let runner = fs::canonicalize(std::env::var_os("JJ_STORE_FORGEJO_RUNNER").unwrap()).unwrap();
    let docker = fs::canonicalize(std::env::var_os("JJ_STORE_DOCKER").unwrap()).unwrap();
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
        "[user]\nname='YAML acceptance'\nemail='yaml@example.invalid'\n",
    )
    .unwrap();
    let mut running = start_local_trusted_server(root.path(), &config);
    let socket = state.join("native.sock");
    let code = root.path().join("code");
    let home = root.path().join("home");
    let mut repos = Vec::new();
    for (name, workspace) in [("code", &code), ("home", &home)] {
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
                root.path().join(format!("{name}-cache")).to_str().unwrap(),
                "--state-dir",
                root.path().join(format!("{name}-client")).to_str().unwrap(),
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
            "YAML acceptance",
        ],
    );
    fs::create_dir_all(code.join(".github/workflows")).unwrap();
    let yaml = "name: Demo\non: [push, workflow_dispatch]\njobs:\n  test:\n    runs-on: ubuntu-latest\n    steps:\n      - name: Prepare\n        uses: actions/checkout@v4\n      - name: Test\n        run: test \"$(cat source.txt)\" = pinned; echo '::group::checks'; echo checked; echo '::endgroup::'\n      - name: Summary\n        run: printf 'complete\\n'\n";
    fs::write(code.join("source.txt"), "pinned\n").unwrap();
    fs::write(code.join(".github/workflows/demo.yml"), yaml).unwrap();
    let describe = |message: &str| {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_jj"));
        cmd.current_dir(&code)
            .env("JJ_CONFIG", root.path().join("jj.toml"))
            .args(["describe", "-m", message]);
        success(cmd.output().unwrap());
    };
    describe("Original");
    let definition = root.path().join("definition.yml");
    fs::write(&definition,format!("name: YAML Demo\nenabled: true\nrepository: {}\ntrigger: {{on: code_published}}\nactions:\n  - id: ci\n    executor: {{type: github_workflow, path: .github/workflows/demo.yml}}\n    timeout_seconds: 120\n",repos[0])).unwrap();
    jj(
        root.path(),
        Some(&home),
        &[
            "workflow",
            "--format=json",
            "put",
            definition.to_str().unwrap(),
            "--request-id",
            &"11".repeat(16),
        ],
    );
    let host_config = root.path().join("host.json");
    let endpoint = std::env::var("JJ_STORE_DOCKER_HOST").ok();
    let environment = endpoint
        .map(|value| json!({"PATH":"/usr/bin:/bin","DOCKER_HOST":value}))
        .unwrap_or(json!({"PATH":"/usr/bin:/bin"}));
    fs::write(&host_config,serde_json::to_vec(&json!({"version":1,"host":"yaml-host","principal":actor,"home":home,"state_dir":root.path().join("host-state"),"jj_executable":env!("CARGO_BIN_EXE_jj"),"allowed_owners":[actor],"labels":["ubuntu-latest"],"sources":[{"repository":repos[0],"workspace":code,"target_id":null}],"forgejo":{"program":runner,"docker_program":docker,"image":"alpine:3.23","architecture":"linux/arm64","environment":environment},"agent_endpoint":null})).unwrap()).unwrap();
    assert!(host(root.path(), &host_config).wait().unwrap().success());
    describe("Trigger exact source");
    let output = host(root.path(), &host_config).wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
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
    let id = runs["runs"][0]["id"].as_str().unwrap().to_owned();
    let detail = jj(
        root.path(),
        Some(&home),
        &["workflow", "--format=json", "runs", "--run", &id],
    );
    assert_eq!(
        detail["runs"][0]["attempts"][0]["status"], "completed",
        "{detail}"
    );
    let evidence = detail["evidence"].as_array().unwrap();
    for name in ["Prepare", "Test", "Summary"] {
        assert!(
            evidence
                .iter()
                .any(|e| e["data"]["output"]["name"] == name && e["data"]["status"] == "completed"),
            "{detail}"
        );
    }
    assert!(evidence.iter().any(|e| {
        e["data"]["type"] == "log"
            && e["data"]["step"].is_string()
            && e["data"]["text"]
                .as_str()
                .is_some_and(|t| t.contains("checked"))
    }));
    fs::write(
        code.join("source.txt"),
        "new revision must not affect retry\n",
    )
    .unwrap();
    describe("Newer source");
    let attempt = detail["runs"][0]["attempts"][0]["id"].as_str().unwrap();
    let body = root.path().join("retry.json");
    fs::write(
        &body,
        serde_json::to_vec(&json!({"type":"retry","run":id,"expected":attempt})).unwrap(),
    )
    .unwrap();
    jj(
        root.path(),
        Some(&home),
        &[
            "workflow",
            "--format=json",
            "command",
            "--request-id",
            &"22".repeat(16),
            body.to_str().unwrap(),
        ],
    );
    let output = host(root.path(), &host_config).wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let retained = jj(
        root.path(),
        Some(&home),
        &["workflow", "--format=json", "runs", "--run", &id],
    );
    assert_eq!(
        retained["runs"][0]["attempts"][1]["status"], "completed",
        "{retained}"
    );
    assert_eq!(retained["runs"][0]["source"], detail["runs"][0]["source"]);
    // A filter edit in the published revision must govern that very event.
    fs::write(code.join("source.txt"), "pinned\n").unwrap();
    fs::write(
        code.join(".github/workflows/demo.yml"),
        yaml.replace(
            "on: [push, workflow_dispatch]",
            "on: {push: {paths: ['excluded/**']}, workflow_dispatch: null}",
        ),
    )
    .unwrap();
    describe("YAML change excludes this event");
    let output = host(root.path(), &host_config).wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let skipped = jj(
        root.path(),
        Some(&home),
        &["workflow", "--format=json", "runs", "--workflow", workflow],
    );
    assert_eq!(
        skipped["runs"][0]["attempts"][0]["status"], "skipped",
        "{skipped}"
    );
    fs::write(
        code.join(".github/workflows/demo.yml"),
        yaml.replace(
            "on: [push, workflow_dispatch]",
            "on: {push: {paths: ['.github/workflows/**']}, workflow_dispatch: null}",
        ),
    )
    .unwrap();
    describe("YAML change includes itself");
    let output = host(root.path(), &host_config).wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let matching = jj(
        root.path(),
        Some(&home),
        &["workflow", "--format=json", "runs", "--workflow", workflow],
    );
    assert_eq!(
        matching["runs"][0]["attempts"][0]["status"], "completed",
        "{matching}"
    );
    assert_ne!(matching["runs"][0]["source"], retained["runs"][0]["source"]);
    running.terminate();
    let mut restarted = start_local_trusted_server(root.path(), &config);
    let cold = jj(
        root.path(),
        Some(&home),
        &["workflow", "--format=json", "runs", "--run", &id],
    );
    for key in ["runs", "workflows", "evidence", "event"] {
        assert_eq!(cold[key], retained[key], "immutable {key}");
    }
    restarted.terminate();
}
