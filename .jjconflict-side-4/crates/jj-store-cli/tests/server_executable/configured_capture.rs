use super::*;
use serde_json::{Value, json};
use std::os::unix::fs::PermissionsExt as _;

#[path = "configured_capture/helpers.rs"]
mod helpers;
use helpers::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires the separately built pinned embedded RustFS runtime"]
async fn cold_configured_codex_capture_initializes_publishes_and_preserves_pause() {
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("local");
    let runtime = fs::canonicalize(std::env::var_os("JJ_STORE_RUSTFS_RUNTIME").unwrap()).unwrap();
    success(
        command()
            .args([
                "init",
                "--local",
                "--state-dir",
                state.to_str().unwrap(),
                "--rustfs-executable",
                runtime.to_str().unwrap(),
                "--format=json",
            ])
            .output()
            .unwrap(),
    );
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let config = state.join("server.toml");
    fs::write(
        &config,
        fs::read_to_string(&config)
            .unwrap()
            .replace("127.0.0.1:7443", &address.to_string()),
    )
    .unwrap();
    let mut running = start_local_trusted_server(root.path(), &config);
    assert!(
        !state.join("agent-capture").exists(),
        "disabled startup created capture state"
    );
    let workspace = root.path().join("workspace");
    let created: Value = serde_json::from_str(&success(
        Command::new(env!("CARGO_BIN_EXE_jj"))
            .current_dir(root.path())
            .env("JJ_CONFIG", "/dev/null")
            .args([
                "--no-pager",
                "--color=never",
                "store",
                "--format=json",
                "init",
                workspace.to_str().unwrap(),
                "--name",
                "lapis/capture-startup",
                "--cache",
                root.path().join("cache").to_str().unwrap(),
                "--state-dir",
                root.path().join("client-state").to_str().unwrap(),
                "--workspace",
                "capture-startup",
                "--unix-socket",
                state.join("native.sock").to_str().unwrap(),
            ])
            .output()
            .unwrap(),
    ))
    .unwrap();
    let repository = created["repository"].as_str().unwrap();
    let origin = format!("http://{address}");
    let client = protected_http_client()
        .no_proxy()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();
    let linked = client
        .post(format!(
            "{origin}/v1/repos/{repository}/local/workspaces/link"
        ))
        .json(&json!({"version":1,"expected_revision":0,"path":workspace}))
        .send()
        .await
        .unwrap();
    assert_eq!(linked.status(), reqwest::StatusCode::OK);
    running.terminate();
    let transcripts = root.path().join("transcripts");
    fs::create_dir(&transcripts).unwrap();
    let transcript = transcripts.join("session.jsonl");
    fs::write(
        &transcript,
        format!(
            "{}\n",
            json!({"type":"session_meta","payload":{
        "id":"00000000-0000-0000-0000-000000000001","cwd":workspace,
        "cli_version":"0.156.1","source":"cli","originator":"codex_cli_rs"}})
        ),
    )
    .unwrap();
    let capture_settings = format!(
        "\n[agent_capture]\nenabled = true\nmax_readers = 1\nmax_sources = 4\nrescan_seconds = 1\nmax_pending_bytes = 4194304\n[[agent_capture.harnesses]]\nprofile = \"codex-cli-jsonl-v1\"\nproduct_version = \"0.156.1\"\nroots = [{}]\n",
        serde_json::to_string(&transcripts).unwrap()
    );
    fs::write(
        &config,
        fs::read_to_string(&config).unwrap() + &capture_settings,
    )
    .unwrap();
    success(
        command()
            .args([
                "config",
                "check",
                "--config",
                config.to_str().unwrap(),
                "--format=json",
            ])
            .output()
            .unwrap(),
    );
    assert!(
        !state.join("agent-capture").exists(),
        "config check initialized state"
    );
    let mut running = start_local_trusted_server(root.path(), &config);
    let status = wait_status(&client, &origin, "observed_sources").await;
    let generation = status["policy"]["generation"].as_str().unwrap().to_owned();
    let capture = state.join("agent-capture");
    for path in [
        capture.clone(),
        capture.join("control"),
        capture.join("imports"),
    ] {
        assert_eq!(
            fs::metadata(path).unwrap().permissions().mode() & 0o777,
            0o700
        );
    }
    assert!(capture.join("imports/state.json").is_file());
    assert!(
        !capture.join("kingfisher").exists(),
        "inactive discovery read bodies"
    );
    append(
        &transcript,
        json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"first"}}),
    );
    append(
        &transcript,
        json!({"type":"response_item","payload":{"type":"message","id":"first",
        "role":"assistant","content":[{"type":"output_text","text":"configured native capture"}]}}),
    );
    wait_status(&client, &origin, "caught_up").await;
    let sessions = get(
        &client,
        &origin,
        &format!("/v1/repos/{repository}/agent/sessions"),
    )
    .await;
    assert_eq!(sessions["sessions"].as_array().unwrap().len(), 1);
    let session = sessions["sessions"][0]["session_id"].as_str().unwrap();
    let detail_path = format!("/v1/repos/{repository}/agent/sessions/{session}");
    let before = get(&client, &origin, &detail_path).await;
    assert!(before.to_string().contains("configured native capture"));
    assert!(!before.to_string().contains(root.path().to_str().unwrap()));
    control(&client, &origin, &generation, 0, true).await;
    running.terminate();
    let policy_bytes = fs::read(capture.join("control/state.json")).unwrap();
    let sources_bytes = fs::read(capture.join("sources/sources.json")).unwrap();
    append(
        &transcript,
        json!({"type":"response_item","payload":{"type":"message","id":"second",
        "role":"assistant","content":[{"type":"output_text","text":"retained while paused"}]}}),
    );
    let mut running = start_local_trusted_server(root.path(), &config);
    let status = get(&client, &origin, "/v1/local/capture").await;
    assert_eq!(status["policy"]["generation"], generation);
    assert_eq!(status["policy"]["paused"], true);
    assert_eq!(status["runtime"]["phase"], "paused");
    assert_eq!(get(&client, &origin, &detail_path).await, before);
    assert_eq!(
        fs::read(capture.join("control/state.json")).unwrap(),
        policy_bytes
    );
    assert_eq!(
        fs::read(capture.join("sources/sources.json")).unwrap(),
        sources_bytes
    );
    control(&client, &origin, &generation, 1, false).await;
    wait_status(&client, &origin, "observed_sources").await;
    append(
        &transcript,
        json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"second"}}),
    );
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            if get(&client, &origin, &detail_path)
                .await
                .to_string()
                .contains("retained while paused")
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    running.terminate();
    assert!(jj_store_server::policy::exclusive_lock(&state.join("server.lock")).is_ok());
    fs::remove_file(capture.join("control/state.json")).unwrap();
    let mut running = start_local_trusted_server(root.path(), &config);
    assert_eq!(
        get(&client, &origin, "/v1/bootstrap").await["storage"]["ready"],
        true
    );
    let unavailable = client
        .get(format!("{origin}/v1/local/capture"))
        .send()
        .await
        .unwrap();
    assert_eq!(
        unavailable.status(),
        reqwest::StatusCode::SERVICE_UNAVAILABLE
    );
    assert!(
        !capture.join("control/state.json").exists(),
        "existing missing state was repaired"
    );
    running.terminate();
}
