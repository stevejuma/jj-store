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
