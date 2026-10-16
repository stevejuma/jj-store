use super::*;
use f::client::Transport;
use serde_json::{Value, json};
use std::sync::Mutex;
struct AccessFixture(PathBuf);
#[async_trait::async_trait]
impl Access for AccessFixture {
    fn repository(&self) -> jj_store_model::RepositoryId {
        jj_store_model::RepositoryId::new([77; 16])
    }
    fn generation(&self) -> u64 {
        1
    }
    fn metadata_path(&self) -> &std::path::Path {
        &self.0
    }
    async fn authorize(&self) -> crate::local_browser::Result<()> {
        Ok(())
    }
}
fn stock(root: &std::path::Path, args: &[&str]) -> String {
    let out = std::process::Command::new("jj")
        .current_dir(root)
        .args([
            "--no-pager",
            "--config",
            "user.name='Fixture'",
            "--config",
            "user.email='fixture@invalid'",
        ])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}
struct Remote {
    local: Arc<LocalForge>,
    prs: Mutex<Vec<Value>>,
}
#[async_trait::async_trait(?Send)]
impl Transport for Remote {
    async fn request(&self, method: &str, path: &str, body: Option<Value>) -> f::Result<Value> {
        if path.contains("pulls?") {
            return Ok(json!(*self.prs.lock().unwrap()));
        }
        if method == "POST" && path.ends_with("/pulls") {
            let body = body.unwrap();
            let branch = body["head"]
                .as_str()
                .unwrap()
                .split(':')
                .next_back()
                .unwrap();
            let sha = self.local.remote_head(branch).await?.unwrap();
            let mut prs = self.prs.lock().unwrap();
            let n = prs.len() + 1;
            let pr = json!({"number":n,"html_url":format!("https://github.com/team/repo/pull/{n}"),"title":body["title"],"body":body["body"],"state":"open","draft":true,"mergeable":true,"head":{"ref":branch,"sha":sha,"repo":{"full_name":"team/repo"}},"base":{"ref":body["base"]}});
            prs.push(pr.clone());
            return Ok(pr);
        }
        let n = path.rsplit('/').next().unwrap().parse::<usize>().unwrap();
        let mut prs = self.prs.lock().unwrap();
        let pr = &mut prs[n - 1];
        if let Some(body) = body {
            if let Some(v) = body.get("body") {
                pr["body"] = v.clone();
            }
            if let Some(v) = body.get("base") {
                pr["base"]["ref"] = v.clone();
            }
        }
        Ok(pr.clone())
    }
}
#[tokio::test]
async fn frozen_local_chain_pushes_only_members_and_restacks_after_squash() {
    let source = tempfile::tempdir().unwrap();
    let root = source.path().canonicalize().unwrap();
    let remote = tempfile::tempdir().unwrap();
    let out = std::process::Command::new("git")
        .args(["init", "--bare", "--initial-branch=main", "--quiet"])
        .arg(remote.path())
        .output()
        .unwrap();
    assert!(out.status.success());
    stock(&root, &["git", "init", "--colocate"]);
    std::fs::write(root.join("base.txt"), "base\n").unwrap();
    stock(&root, &["commit", "-m", "base"]);
    stock(&root, &["bookmark", "create", "main", "-r", "@-"]);
    stock(
        &root,
        &[
            "git",
            "remote",
            "add",
            "origin",
            remote.path().to_str().unwrap(),
        ],
    );
    stock(
        &root,
        &["git", "push", "--remote", "origin", "--bookmark", "main"],
    );
    stock(&root, &["new", "main", "-m", "model"]);
    std::fs::write(root.join("model.txt"), "model\n").unwrap();
    stock(&root, &["bookmark", "create", "model"]);
    stock(&root, &["new", "model", "-m", "api"]);
    std::fs::write(root.join("api.txt"), "api\n").unwrap();
    stock(&root, &["new", "-m", "Empty final commit"]);
    stock(&root, &["bookmark", "create", "api"]);
    stock(&root, &["new", "main"]);
    let access: Arc<dyn Access> =
        Arc::new(AccessFixture(root.join(".jj/repo").canonicalize().unwrap()));
    let state = execute(
        access.clone(),
        root.clone(),
        Request {
            version: 1,
            binding_generation: 1,
            operation: None,
            revision: None,
            command: Command::Refresh,
        },
    )
    .await
    .unwrap();
    let state = execute(
        access.clone(),
        root.clone(),
        Request {
            version: 1,
            binding_generation: 1,
            operation: Some(state.operation),
            revision: Some(state.revision),
            command: Command::Setup {
                name: "Work".into(),
                target: "main".into(),
            },
        },
    )
    .await
    .unwrap();
    execute(
        access.clone(),
        root.clone(),
        Request {
            version: 1,
            binding_generation: 1,
            operation: Some(state.operation),
            revision: Some(state.revision),
            command: Command::Apply {
                branch: "api".into(),
            },
        },
    )
    .await
    .unwrap();
    let connection = f::Connection {
        provider: f::Provider::Github,
        api_url: "https://api.github.com".into(),
        repository: "team/repo".into(),
        source_repository: "team/repo".into(),
        remote: "origin".into(),
        target_remote: "origin".into(),
        target: "main".into(),
        native_stacks: false,
    };
    let local = Arc::new(LocalForge {
        access: access.clone(),
        root: root.clone(),
        connection: connection.clone(),
        credential: None,
        account_id: None,
    });
    let op = stock(
        &root,
        &[
            "--ignore-working-copy",
            "op",
            "log",
            "-n",
            "1",
            "--no-graph",
            "-T",
            "id",
        ],
    );
    let (operation, segments) = local.segments("api").await.unwrap();
    assert_eq!(op, operation);
    assert_eq!(
        segments
            .iter()
            .map(|s| s.branch.as_str())
            .collect::<Vec<_>>(),
        vec!["model", "api"]
    );
    let transport = Arc::new(Remote {
        local: local.clone(),
        prs: Mutex::new(vec![]),
    });
    let client = f::Client::with_transport(connection.clone(), transport.clone()).unwrap();
    let device_root = tempfile::tempdir().unwrap();
    let mut device = Device::at(device_root.path(), access.metadata_path()).unwrap();
    device.configure(connection.clone()).unwrap();
    let plan =
        f::planner::plan("first".into(), operation, connection, segments, &[], true).unwrap();
    let receipt = f::engine::submit(local.as_ref(), &client, &mut device, plan, false)
        .await
        .unwrap();
    assert_eq!(receipt.outcome, f::Outcome::Complete);
    assert_eq!(receipt.pull_requests.len(), 2);
    assert!(local.remote_head("api").await.unwrap().is_some());
    assert!(local.remote_head("model").await.unwrap().is_some());
    let upstream = tempfile::tempdir().unwrap();
    stock(
        upstream.path(),
        &["git", "clone", remote.path().to_str().unwrap(), "upstream"],
    );
    let upstream = upstream.path().join("upstream");
    stock(&upstream, &["bookmark", "track", "main@origin"]);
    stock(&upstream, &["new", "main", "-m", "Squash model"]);
    std::fs::write(upstream.join("model.txt"), "model\n").unwrap();
    stock(&upstream, &["commit", "-m", "Squash model"]);
    stock(&upstream, &["bookmark", "set", "main", "-r", "@-"]);
    stock(&upstream, &["git", "push", "--bookmark", "main"]);
    {
        let mut prs = transport.prs.lock().unwrap();
        prs[0]["state"] = "closed".into();
        prs[0]["merged_at"] = "2026-10-15T10:00:00Z".into();
    }
    let before_maintenance = stock(
        &root,
        &[
            "--ignore-working-copy",
            "op",
            "log",
            "-n",
            "1",
            "--no-graph",
            "-T",
            "id",
        ],
    );
    let observed = f::engine::observe(&client, &device).await.unwrap();
    f::engine::maintain(local.as_ref(), &client, &mut device, &observed)
        .await
        .unwrap();
    assert!(
        transport.prs.lock().unwrap()[1]["body"]
            .as_str()
            .unwrap()
            .contains("Merged")
    );
    assert_eq!(transport.prs.lock().unwrap()[1]["base"]["ref"], "model");
    assert_eq!(
        before_maintenance,
        stock(
            &root,
            &[
                "--ignore-working-copy",
                "op",
                "log",
                "-n",
                "1",
                "--no-graph",
                "-T",
                "id"
            ]
        )
    );
    std::fs::write(root.join("draft.txt"), "keep draft\n").unwrap();
    execute(
        access.clone(),
        root.clone(),
        Request {
            version: 1,
            binding_generation: 1,
            operation: None,
            revision: None,
            command: Command::Refresh,
        },
    )
    .await
    .unwrap();
    let before = stock(
        &root,
        &[
            "--ignore-working-copy",
            "log",
            "-r",
            "api",
            "--no-graph",
            "-T",
            "change_id",
        ],
    );
    let preview = restack::preview(&local, &client, &device, "api")
        .await
        .unwrap();
    assert_eq!(preview.plan.merged, vec!["model"]);
    let (_, result) = restack::apply(&local, &client, &mut device, preview.plan)
        .await
        .unwrap();
    assert_eq!(result.unwrap().outcome, f::Outcome::Complete);
    assert_eq!(
        std::fs::read_to_string(root.join("draft.txt")).unwrap(),
        "keep draft\n"
    );
    assert_eq!(
        std::fs::read_to_string(root.join("api.txt")).unwrap(),
        "api\n"
    );
    assert_eq!(
        before,
        stock(
            &root,
            &[
                "--ignore-working-copy",
                "log",
                "-r",
                "api",
                "--no-graph",
                "-T",
                "change_id"
            ]
        )
    );
    assert_eq!(transport.prs.lock().unwrap()[1]["base"]["ref"], "main");
}

#[derive(Default)]
struct AccountSecrets(Mutex<std::collections::HashMap<String, String>>);
impl f::accounts::store::SecretStore for AccountSecrets {
    fn get(&self, id: &str) -> f::Result<Option<String>> {
        Ok(self.0.lock().unwrap().get(id).cloned())
    }
    fn set(&self, id: &str, value: &str) -> f::Result<()> {
        self.0.lock().unwrap().insert(id.into(), value.into());
        Ok(())
    }
    fn remove(&self, id: &str) -> f::Result<()> {
        self.0.lock().unwrap().remove(id);
        Ok(())
    }
}
#[derive(Default)]
struct AccountApi {
    identity: Mutex<u64>,
    denied: Mutex<bool>,
}
#[async_trait::async_trait(?Send)]
impl f::accounts::http::Transport for AccountApi {
    async fn request(&self, url: &str, _: Option<&str>, _: Option<Value>) -> f::Result<Value> {
        if *self.denied.lock().unwrap() {
            return Err(f::Error::Denied);
        }
        if url.ends_with("/user") {
            return Ok(json!({"id":*self.identity.lock().unwrap(),"login":"fixture"}));
        }
        Ok(json!({"default_branch":"trunk","fork":false}))
    }
}
#[tokio::test]
async fn github_detection_binds_one_verified_account_and_preserves_ambiguity_and_pending_intent() {
    let source = tempfile::tempdir().unwrap();
    let root = source.path().canonicalize().unwrap();
    stock(&root, &["git", "init", "--colocate"]);
    stock(
        &root,
        &[
            "git",
            "remote",
            "add",
            "origin",
            "https://github.com/team/repo.git",
        ],
    );
    let before = stock(
        &root,
        &[
            "--ignore-working-copy",
            "op",
            "log",
            "--no-graph",
            "-n",
            "1",
            "-T",
            "id",
        ],
    );
    let state = tempfile::tempdir().unwrap();
    let metadata = root.join(".jj/repo").canonicalize().unwrap();
    let access: Arc<dyn Access> = Arc::new(AccessFixture(metadata.clone()));
    let local = LocalForge {
        access,
        root: root.clone(),
        connection: connection::provisional(),
        credential: None,
        account_id: None,
    };
    let secrets = Arc::new(AccountSecrets::default());
    let api = Arc::new(AccountApi {
        identity: Mutex::new(42),
        ..Default::default()
    });
    let manager = f::accounts::Manager::new(
        state.path().join("accounts"),
        secrets.clone(),
        api.clone(),
        None,
    );
    let mut device = Device::at(&state.path().join("device"), &metadata).unwrap();
    let result = connection::detect_with(&local, &mut device, true, &manager)
        .await
        .unwrap();
    assert_eq!(result.readiness, "unconfigured");
    assert!(device.connection().is_none());
    manager
        .execute(
            f::accounts::Command::AddToken {
                api_url: "https://api.github.com".into(),
                token: "fixture-secret".into(),
            },
            1,
        )
        .await
        .unwrap();
    let result = connection::detect_with(&local, &mut device, true, &manager)
        .await
        .unwrap();
    assert_eq!(result.readiness, "connected");
    assert_eq!(device.connection().unwrap().target, "trunk");
    let account = device.account_id().unwrap();
    *api.denied.lock().unwrap() = true;
    assert_eq!(
        connection::detect_with(&local, &mut device, true, &manager)
            .await
            .unwrap()
            .readiness,
        "reconnect_required"
    );
    *api.denied.lock().unwrap() = false;
    let after = stock(
        &root,
        &[
            "--ignore-working-copy",
            "op",
            "log",
            "--no-graph",
            "-n",
            "1",
            "-T",
            "id",
        ],
    );
    assert_eq!(before, after, "discovery must not snapshot or publish code");
    *api.identity.lock().unwrap() = 43;
    manager
        .execute(
            f::accounts::Command::AddToken {
                api_url: "https://api.github.com".into(),
                token: "other-secret".into(),
            },
            1,
        )
        .await
        .unwrap();
    let mut other = Device::at(&state.path().join("ambiguous"), &metadata).unwrap();
    assert_eq!(
        connection::detect_with(&local, &mut other, true, &manager)
            .await
            .unwrap()
            .readiness,
        "selection_required"
    );
    assert!(other.connection().is_none());
    // Existing pending external work retains its original account even after another login.
    let action = f::PrAction::Draft { draft: true };
    device
        .begin_operation("pending-account-operation", 1, &action)
        .unwrap();
    assert!(matches!(
        device.bind_account("different-account".into()),
        Err(f::Error::Stale)
    ));
    assert_eq!(device.account_id(), Some(account));
    stock(
        &root,
        &[
            "git",
            "remote",
            "add",
            "upstream",
            "https://github.com/team/upstream.git",
        ],
    );
    manager
        .execute(
            f::accounts::Command::Forget {
                account: manager.inventory().unwrap().accounts[1].id.clone(),
            },
            1,
        )
        .await
        .unwrap();
    assert_eq!(
        connection::detect_with(&local, &mut other, true, &manager)
            .await
            .unwrap()
            .readiness,
        "selection_required"
    );
}
