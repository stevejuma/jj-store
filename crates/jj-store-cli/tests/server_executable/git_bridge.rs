//! Real persistent RustFS authority plus actual Git and cold browser acceptance.
use super::*;
#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires built pinned embedded RustFS and browser bundle"]
async fn real_rustfs_native_git_external_merge_cycle_survives_cold_restart() {
    use std::os::unix::fs::PermissionsExt as _;

    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("local");
    let runtime = std::fs::canonicalize(
        std::env::var_os("JJ_STORE_RUSTFS_RUNTIME")
            .expect("set JJ_STORE_RUSTFS_RUNTIME to the built embedded runtime"),
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
    assert_eq!(running.event["address"], address.to_string());
    let socket = state.join("native.sock");
    assert_eq!(
        fs::metadata(&socket).unwrap().permissions().mode() & 0o777,
        0o600
    );
    let http = protected_http_client()
        .no_proxy()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();
    let origin = format!("http://{address}");
    let bootstrap = http
        .get(format!("{origin}/v1/bootstrap"))
        .send()
        .await
        .unwrap();
    assert_eq!(bootstrap.status(), reqwest::StatusCode::OK);
    let bootstrap: serde_json::Value = bootstrap.json().await.unwrap();
    assert_eq!(bootstrap["access_mode"], "local_trusted");
    assert_eq!(bootstrap["storage"]["ready"], true);
    assert!(bootstrap.get("repositories").is_none());
    let repositories: serde_json::Value = http
        .get(format!("{origin}/v1/repositories"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(repositories["repositories"], serde_json::json!([]));

    let jj_config = root.path().join("jj.toml");
    fs::write(
        &jj_config,
        "[user]\nname='Local Acceptance'\nemail='local@example.invalid'\n",
    )
    .unwrap();
    let workspace = root.path().join("git-cycle");
    let created: serde_json::Value = serde_json::from_str(&success(
        Command::new(env!("CARGO_BIN_EXE_jj-store-jj"))
            .current_dir(root.path())
            .env("JJ_CONFIG", &jj_config)
            .args([
                "--no-pager",
                "--color=never",
                "store",
                "--format=json",
                "init",
                workspace.to_str().unwrap(),
                "--name",
                "lapis/git-cycle",
                "--cache",
                root.path().join("cache").to_str().unwrap(),
                "--state-dir",
                root.path().join("client-state").to_str().unwrap(),
                "--workspace",
                "git-cycle",
                "--unix-socket",
                socket.to_str().unwrap(),
            ])
            .output()
            .unwrap(),
    ))
    .unwrap();
    let repository = created["repository"].as_str().unwrap();
    let device = root.path().join("git-device");
    let run = |args: &[&str]| {
        success(
            Command::new(env!("CARGO_BIN_EXE_jj-store-jj"))
                .current_dir(&workspace)
                .env("JJ_CONFIG", &jj_config)
                .env("JJ_STORE_GIT_HOME", &device)
                .env_remove("JJ_STORE_TOKEN")
                .args(["--no-pager", "--color=never"])
                .args(args)
                .output()
                .unwrap(),
        )
    };
    let bare = root.path().join("upstream.git");
    let git = gix::init_bare(&bare).unwrap();
    use gix::objs::Write as _;
    let tree = git.write_object(gix::objs::Tree::empty()).unwrap();
    let base=git.objects.write_buf(gix::objs::Kind::Commit,format!("tree {tree}\nauthor Test <test@example.invalid> 1000 +0000\ncommitter Test <test@example.invalid> 1000 +0000\n\nGit base\n").as_bytes()).unwrap();
    git.reference(
        "refs/heads/main",
        base,
        gix::refs::transaction::PreviousValue::MustNotExist,
        "fixture",
    )
    .unwrap();
    run(&["git", "remote", "add", "origin", bare.to_str().unwrap()]);
    run(&["git", "fetch"]);
    run(&["bookmark", "track", "main", "--remote", "origin"]);
    run(&["new", "main@origin", "-m", "Native Git feature"]);
    fs::write(workspace.join("feature.txt"), "native feature\n").unwrap();
    run(&["describe", "-m", "Native Git feature"]);
    run(&["bookmark", "create", "feature", "-r", "@"]);
    let native_id = run(&["log", "--no-graph", "-r", "feature", "-T", "commit_id"]);
    let change_id = run(&[
        "log",
        "--no-graph",
        "-r",
        "feature",
        "-T",
        "change_id.normal_hex()",
    ]);
    let operation = run(&["op", "log", "--no-graph", "-n", "1", "-T", "id"]);
    let repo_id: jj_store_model::RepositoryId = repository.parse().unwrap();
    let anchor = jj_review::ArtifactAnchorV1 {
        schema_version: 1,
        repository_id: repo_id,
        target: jj_review::ArtifactTargetV1::Revision {
            operation_id: operation.parse().unwrap(),
            commit_id: native_id.parse().unwrap(),
            change_id: Some(change_id.parse().unwrap()),
        },
        created_context: jj_review::CreatedContextV1 {
            publication_position: None,
            code_state_version: None,
            agent_root: None,
        },
        sensitivity: jj_review::SensitivityV1::Repository,
    };
    let anchor_file = root.path().join("review-anchor.json");
    fs::write(&anchor_file, serde_json::to_vec(&anchor).unwrap()).unwrap();
    let review_request = jj_store_model::RequestId::new([210; 16]).to_string();
    run(&[
        "review",
        "create",
        "--request-id",
        &review_request,
        "--anchor",
        anchor_file.to_str().unwrap(),
        "--body",
        "Retained native review",
    ]);
    let review = run(&["review", "status", &review_request, "--format", "json"]);
    let client =
        jj_store_rpc::client::NativeClient::connect_unix(&socket, repo_id, Default::default())
            .await
            .unwrap();
    use jj_agent::*;
    let batch = SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([211; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([212; 16]),
        sanitizer_version: "mongodb-kingfisher@2.7.0/builtin-medium/v1".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![SanitizedObservationV1 {
            observation_id: ObservationId::new([213; 16]),
            source_sequence: 1,
            occurred_at_millis: Some(1725000000000),
            source_kind: SourceKind::Hook,
            source_trust: SourceTrust::LocalUserHook,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                br#"{"hook_event_name":"SessionStart","session_id":"git-cycle-fixture"}"#,
            )
            .unwrap(),
        }],
    };
    let capture = client
        .prepare_agent_capture(
            jj_store_model::RequestId::new([214; 16]),
            jj_store_model::PublicationId::new([215; 16]),
            &NormalizerProfileV1::codex_0_156_1(),
            &batch,
            b"accepted".to_vec(),
        )
        .await
        .unwrap();
    let agent_receipt = client.publish_agent_delta(&capture).await.unwrap();
    let pushed = run(&["git", "push", "--bookmark", "feature"]);
    assert!(pushed.contains("Push completed"));
    let feature = git
        .find_reference("refs/heads/feature")
        .unwrap()
        .target()
        .try_id()
        .unwrap()
        .to_owned();
    assert_eq!(
        git.find_commit(feature)
            .unwrap()
            .decode()
            .unwrap()
            .parents()
            .next()
            .unwrap(),
        base
    );
    assert!(git.find_reference("refs/heads/main").is_ok());
    assert!(git.find_reference("refs/agent/fixture").is_err());
    fs::remove_dir_all(device.join(repository).join("cache")).unwrap();
    running.terminate();
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let search = format!(
        "\n[[search]]\nrepository = \"{repository}\"\npoll_millis = 10\ntimeout_seconds = 30\nsnapshot_every = 1\n\n[search.source.scopes.workspace]\nkind = \"workspace\"\nname = \"git-cycle\"\n\n[search.source.scopes.imported]\nkind = \"bookmark\"\nname = \"main\"\n"
    );
    let mut text = fs::read_to_string(&config)
        .unwrap()
        .replace("search = []\n", "");
    text = text.replacen(
        "format = 2",
        &format!("format = 2\nweb_root = {:?}", repo_root.join("web/dist")),
        1,
    );
    fs::write(&config, format!("{text}{search}")).unwrap();
    let mut cold = start_local_trusted_server(root.path(), &config);
    assert_eq!(
        run(&["log", "--no-graph", "-r", "feature", "-T", "commit_id"]),
        native_id
    );
    assert_eq!(
        run(&["review", "status", &review_request, "--format", "json"]),
        review
    );
    let client =
        jj_store_rpc::client::NativeClient::connect_unix(&socket, repo_id, Default::default())
            .await
            .unwrap();
    assert_eq!(
        client.publish_agent_delta(&capture).await.unwrap(),
        agent_receipt
    );
    let before_fetch = client.git_bridge_state().await.unwrap().code;
    // The first full observation now includes the newly pushed feature ref.
    run(&["git", "fetch"]);
    let after_fetch = client.git_bridge_state().await.unwrap().code;
    assert_eq!(before_fetch.heads, after_fetch.heads);
    assert_eq!(
        before_fetch.code_state_version,
        after_fetch.code_state_version
    );
    assert!(run(&["git", "fetch"]).contains("Nothing changed"));
    let original_tree = git.find_commit(feature).unwrap().decode().unwrap().tree();
    let mut merge_tree = git
        .find_tree(original_tree)
        .unwrap()
        .decode()
        .unwrap()
        .to_owned();
    let external_file = git
        .write_blob(b"External Git merge searchable content\n")
        .unwrap();
    merge_tree.entries.push(gix::objs::tree::Entry {
        mode: gix::objs::tree::EntryKind::Blob.into(),
        filename: b"external.txt".as_slice().into(),
        oid: external_file.detach(),
    });
    merge_tree
        .entries
        .sort_by(|a, b| a.filename.cmp(&b.filename));
    let merge_tree = git.write_object(merge_tree).unwrap();
    let merged=git.objects.write_buf(gix::objs::Kind::Commit,format!("tree {merge_tree}\nparent {base}\nparent {feature}\nauthor External <external@example.invalid> 1001 +0000\ncommitter External <external@example.invalid> 1001 +0000\n\nExternal Git merge\n").as_bytes()).unwrap();
    git.reference(
        "refs/heads/main",
        merged,
        gix::refs::transaction::PreviousValue::Any,
        "external merge",
    )
    .unwrap();
    // The browser opens before fetch and drives the external import, checking
    // committed SSE freshness and explicit snapshot refresh on real storage.
    success(
        Command::new("pnpm")
            .current_dir(repo_root)
            .args([
                "exec",
                "playwright",
                "test",
                "--config",
                "web/playwright.real.config.ts",
                "git-bridge.spec.ts",
            ])
            .env("JJ_STORE_REAL_BASE_URL", &origin)
            .env("JJ_STORE_REAL_REPOSITORY", repository)
            .env("JJ_STORE_REAL_RUNTIME", root.path())
            .env("JJ_STORE_REAL_ROOT", repo_root)
            .env("JJ_STORE_GIT_CYCLE_WORKSPACE", &workspace)
            .env("JJ_CONFIG", &jj_config)
            .env("JJ_STORE_GIT_HOME", &device)
            .output()
            .unwrap(),
    );
    assert_eq!(
        run(&[
            "log",
            "--no-graph",
            "-r",
            "main",
            "-T",
            "description.trim_end()"
        ]),
        "External Git merge"
    );
    assert_eq!(
        run(&[
            "log",
            "--no-graph",
            "-r",
            "feature",
            "-T",
            "change_id.normal_hex()"
        ]),
        change_id
    );
    assert_eq!(
        run(&["review", "status", &review_request, "--format", "json"]),
        review
    );
    let base_url = format!("{origin}/v1/repos/{repository}");
    wait_search_ready(&http, None, &base_url).await;
    let search: serde_json::Value = http
        .get(format!(
            "{base_url}/search?q=External&scope=imported&mode=historical"
        ))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(
        !search["results"].as_array().unwrap().is_empty(),
        "{search}"
    );
    let git_before = client.git_bridge_state().await.unwrap();
    cold.terminate();
    let mut final_cold = start_local_trusted_server(root.path(), &config);
    let client =
        jj_store_rpc::client::NativeClient::connect_unix(&socket, repo_id, Default::default())
            .await
            .unwrap();
    assert_eq!(
        client.git_bridge_state().await.unwrap().git.unwrap().values,
        git_before.git.unwrap().values
    );
    assert_eq!(
        client.publish_agent_delta(&capture).await.unwrap(),
        agent_receipt
    );
    assert_eq!(
        run(&["review", "status", &review_request, "--format", "json"]),
        review
    );
    // Local JJ remains usable with the Git host gone; no background mirror.
    fs::rename(&bare, root.path().join("offline-upstream.git")).unwrap();
    run(&["new", "main", "-m", "Local work with Git offline"]);
    run(&["describe", "-m", "Local work with Git offline"]);
    assert!(!workspace.join(".git").exists());
    final_cold.terminate();
}
