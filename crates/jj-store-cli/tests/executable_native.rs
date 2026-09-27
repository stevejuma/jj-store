//! Cold executable acceptance against the production RPC/service stack. The
//! backing memory store is a test fixture, not a deployment conformance claim.
use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_rpc::server::{Limits as RpcLimits, NativeRpc};
use jj_store_service::{auth::*, native::*, transport::NativeEndpoint};
use jj_store_wal::publication::Limits;
use std::{
    fs,
    net::SocketAddr,
    path::Path,
    process::{Command, Output},
    sync::Arc,
    time::Duration,
};
const REPO: m::RepositoryId = m::RepositoryId::new([71; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([72; 16]);
struct Fixture {
    store: Arc<MemoryStore>,
    token: BearerToken,
    reader: BearerToken,
    address: SocketAddr,
    server: Option<tokio::task::JoinHandle<()>>,
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
    root: tempfile::TempDir,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        if let Some(server) = &self.server {
            server.abort();
        }
    }
}
impl Fixture {
    async fn new() -> Self {
        let store = Arc::new(MemoryStore::default());
        let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 32 * 1024 * 1024).unwrap());
        let mut policy = TokenPolicy::new(Scope::Repository(REPO));
        let token = policy.issue(ACTOR, Permission::Admin, None).unwrap();
        let reader = policy
            .issue(m::PrincipalId::new([74; 16]), Permission::Read, None)
            .unwrap();
        let mut operators = TokenPolicy::new(Scope::Operator);
        let operator = operators
            .issue(ACTOR, Permission::CreateRepository, None)
            .unwrap();
        create_repository(
            store.clone(),
            objects,
            &operators,
            &operator,
            &policy,
            m::EpochId::new([73; 16]),
            &SystemClock,
            Limits::default(),
        )
        .await
        .unwrap();
        let mut f = Self {
            store,
            token,
            reader,
            address: "127.0.0.1:0".parse().unwrap(),
            server: None,
            shutdown: None,
            root: tempfile::tempdir().unwrap(),
        };
        fs::write(
            f.root.path().join("jj.toml"),
            "[user]\nname = 'Native Test'\nemail = 'native@example.invalid'\n",
        )
        .unwrap();
        f.start().await;
        f
    }
    async fn start(&mut self) {
        let endpoint = NativeEndpoint::new(
            REPO,
            self.store.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
            32 * 1024 * 1024,
        )
        .unwrap();
        let rpc = NativeRpc::new(vec![endpoint], RpcLimits::default()).unwrap();
        let listener = tokio::net::TcpListener::bind(self.address).await.unwrap();
        self.address = listener.local_addr().unwrap();
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.shutdown = Some(tx);
        self.server = Some(tokio::spawn(async move {
            rpc.serve(listener, None, true, async {
                let _ = rx.await;
            })
            .await
            .unwrap();
        }));
    }
    async fn stop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
        if let Some(server) = self.server.take() {
            tokio::time::timeout(Duration::from_secs(5), server)
                .await
                .unwrap()
                .unwrap();
        }
    }
    fn command(&self, cwd: &Path, args: &[&str]) -> Command {
        let mut cmd = command(cwd, args);
        cmd.env("JJ_CONFIG", self.root.path().join("jj.toml"));
        cmd.env("JJ_STORE_TOKEN", self.token.expose().as_str());
        cmd
    }
    fn run(&self, cwd: &Path, args: &[&str]) -> String {
        success(self.command(cwd, args).output().unwrap())
    }
    fn attach(&self, name: &str) -> serde_json::Value {
        let work = self.root.path().join(name);
        let cache = self.root.path().join(format!("{name}-cache"));
        serde_json::from_str(&self.run(
            self.root.path(),
            &[
                "store",
                "attach",
                work.to_str().unwrap(),
                "--endpoint",
                &format!("http://{}", self.address),
                "--repo-id",
                &REPO.to_string(),
                "--insecure-loopback",
                "--workspace",
                name,
                "--cache",
                cache.to_str().unwrap(),
                "--format",
                "json",
            ],
        ))
        .unwrap()
    }
    async fn snapshot(&self) -> jj_store_wal::publication::Snapshot {
        NativeSession::new(
            self.store.clone(),
            Arc::new(NativeObjects::new(self.store.clone(), REPO, 32 * 1024 * 1024).unwrap()),
            self.token.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap()
        .snapshot()
        .await
        .unwrap()
        .manifest()
        .snapshot()
    }
}
fn command(cwd: &Path, args: &[&str]) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_jj"));
    cmd.current_dir(cwd)
        .args(["--no-pager", "--color=never"])
        .args(args)
        .env("JJ_CONFIG", "")
        .env_remove("JJ_STORE_TOKEN")
        .env_remove("JJ_STORE_PAGER")
        .env("NO_COLOR", "1");
    cmd
}
fn success(output: Output) -> String {
    assert!(
        output.status.success(),
        "exit {:?}\nstdout: {}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn executable_native_create_edit_describe_log_restart_and_evolution() {
    let mut f = Fixture::new().await;
    let attached = f.attach("alpha");
    let work = f.root.path().join("alpha");
    assert_eq!(attached["repository"], REPO.to_string());
    assert!(!work.join(".git").exists());
    fs::write(work.join("hello.txt"), "native executable content\n").unwrap();
    f.run(&work, &["describe", "-m", "first executable description"]);
    let id = f.run(&work, &["log", "--no-graph", "-r", "@", "-T", "commit_id"]);
    assert_eq!(id.trim().len(), 128);
    assert!(
        f.run(
            &work,
            &["log", "--no-graph", "-r", "@", "-T", "description"]
        )
        .contains("first executable description")
    );
    f.run(&work, &["new", "-m", "second change"]);
    fs::write(work.join("hello.txt"), "changed by executable\n").unwrap();
    assert!(f.run(&work, &["diff"]).contains("changed by executable"));
    f.run(&work, &["edit", id.trim()]);
    assert_eq!(
        fs::read_to_string(work.join("hello.txt")).unwrap(),
        "native executable content\n"
    );
    f.run(&work, &["describe", "-m", "described again"]);
    assert!(
        f.run(&work, &["evolog", "--limit", "10"])
            .contains("first executable description")
    );
    assert!(
        f.run(&work, &["op", "log", "--limit", "10"])
            .contains("describe")
    );
    f.run(&work, &["bookmark", "create", "keep"]);
    f.run(&work, &["bookmark", "set", "keep", "-r", "@"]);
    assert!(f.run(&work, &["bookmark", "list"]).contains("keep"));
    f.run(&work, &["bookmark", "delete", "keep"]);
    let before = f.snapshot().await;
    fs::write(work.join("unpublished.txt"), "unsnapshotted\n").unwrap();
    for command in ["status", "doctor"] {
        let status: serde_json::Value =
            serde_json::from_str(&f.run(&work, &["store", command, "--format", "json"])).unwrap();
        assert_eq!(status["ok"], true);
        assert_eq!(
            status["heads"].as_array().unwrap().len(),
            before.heads.len()
        );
    }
    assert_eq!(
        f.snapshot().await,
        before,
        "custom reads published a snapshot"
    );
    f.stop().await;
    f.start().await;
    assert_eq!(f.snapshot().await, before);
    assert!(
        f.run(
            &work,
            &["log", "--no-graph", "-r", "@", "-T", "description"]
        )
        .contains("described again")
    );
    assert_eq!(
        fs::read_to_string(work.join("unpublished.txt")).unwrap(),
        "unsnapshotted\n"
    );
    assert!(
        f.run(&work, &["op", "log", "--limit", "5"])
            .contains("snapshot")
    );
    let binding =
        fs::read_to_string(f.root.path().join("alpha-cache/store-connection.json")).unwrap();
    assert!(!binding.contains(f.token.expose().as_str()));
    assert!(!binding.contains("jst1_"));
    f.stop().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn executable_workspaces_keep_local_edits_and_reject_credential_or_binding_mismatch() {
    let mut f = Fixture::new().await;
    f.attach("left");
    f.attach("right");
    let left = f.root.path().join("left");
    let right = f.root.path().join("right");
    fs::write(left.join("local.txt"), "left local edits\n").unwrap();
    fs::write(right.join("remote.txt"), "right snapshot\n").unwrap();
    f.run(&right, &["describe", "-m", "right publication"]);
    assert_eq!(
        fs::read_to_string(left.join("local.txt")).unwrap(),
        "left local edits\n"
    );
    f.run(&left, &["describe", "-m", "left publication"]);
    let log = f.run(
        &left,
        &["log", "--no-graph", "-r", "all()", "-T", "description"],
    );
    assert!(
        log.contains("left publication") && log.contains("right publication"),
        "{log}"
    );
    let output = f
        .command(&left, &["store", "status"])
        .env("JJ_STORE_TOKEN", "malformed-credential")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("Hint:"));
    assert!(!error.contains("malformed-credential"));
    let binding_path = f.root.path().join("left-cache/store-connection.json");
    let original = fs::read(&binding_path).unwrap();
    let mut binding: serde_json::Value = serde_json::from_slice(&original).unwrap();
    binding["repository"] = serde_json::json!(m::RepositoryId::new([99; 16]));
    fs::write(&binding_path, serde_json::to_vec(&binding).unwrap()).unwrap();
    let before = f.snapshot().await;
    let output = f.command(&left, &["log"]).output().unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("binding mismatch"));
    assert_eq!(f.snapshot().await, before);
    fs::write(binding_path, original).unwrap();
    f.stop().await;
}

#[test]
fn custom_help_completion_build_info_and_ordinary_git_commands() {
    let temp = tempfile::tempdir().unwrap();
    let help = success(
        command(temp.path(), &["store", "attach", "--help"])
            .output()
            .unwrap(),
    );
    assert!(help.contains("destination must be empty"));
    assert!(!help.contains("```"));
    assert!(!help.contains('`'));
    assert!(help.contains("help -k"));
    let short = success(
        command(temp.path(), &["store", "attach", "-h"])
            .output()
            .unwrap(),
    );
    assert!(short.len() < help.len());
    let routed = success(
        command(temp.path(), &["help", "store", "attach"])
            .output()
            .unwrap(),
    );
    assert!(routed.contains("destination must be empty"));
    assert!(!routed.contains('`'));
    assert!(success(command(temp.path(), &["store"]).output().unwrap()).contains("attach"));
    let keyword = success(
        command(temp.path(), &["store", "help", "-k", "credentials"])
            .output()
            .unwrap(),
    );
    assert!(keyword.contains("OS credential facility"));
    assert!(!keyword.contains("```"));
    let wrong = command(temp.path(), &["store", "attch"]).output().unwrap();
    assert!(!wrong.status.success());
    let wrong = String::from_utf8(wrong.stderr).unwrap();
    assert!(wrong.contains("attach") && wrong.contains("Hint:"));
    let missing = command(temp.path(), &["store", "attach"]).output().unwrap();
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains("Hint:"));
    let version: serde_json::Value = serde_json::from_str(&success(
        command(temp.path(), &["store", "version", "--format", "json"])
            .output()
            .unwrap(),
    ))
    .unwrap();
    assert_eq!(version["client"]["upstream_jj"], "0.45.1");
    assert!(
        version["client"]["build_target"]
            .as_str()
            .unwrap()
            .contains('-')
    );
    let completion = success(
        command(temp.path(), &["util", "completion", "bash"])
            .output()
            .unwrap(),
    );
    assert!(completion.contains("store"));
    let markdown = success(
        command(temp.path(), &["util", "markdown-help"])
            .output()
            .unwrap(),
    );
    assert!(markdown.contains("jj store attach"));
    assert!(markdown.contains("```"));
    success(
        command(temp.path(), &["git", "init", "git-work"])
            .output()
            .unwrap(),
    );
    let git = temp.path().join("git-work");
    success(
        command(&git, &["describe", "-m", "ordinary Git-backed command"])
            .args([
                "--config",
                "user.name='Test'",
                "--config",
                "user.email='test@example.invalid'",
            ])
            .output()
            .unwrap(),
    );
    assert!(
        success(
            command(&git, &["log", "--no-graph", "-T", "description"])
                .output()
                .unwrap()
        )
        .contains("ordinary Git-backed command")
    );
}

#[test]
fn custom_color_json_and_metadata_validation_do_not_contact_or_leak_credentials() {
    let temp = tempfile::tempdir().unwrap();
    let forced = success(
        command(
            temp.path(),
            &["store", "attach", "--help", "--color=always"],
        )
        .output()
        .unwrap(),
    );
    assert!(forced.contains("\x1b["));
    let auto = success(
        command(temp.path(), &["store", "attach", "--help", "--color=auto"])
            .output()
            .unwrap(),
    );
    assert!(!auto.contains("\x1b["));
    let json = success(
        command(
            temp.path(),
            &["store", "capabilities", "--format=json", "--color=always"],
        )
        .env("JJ_STORE_PAGER", "false")
        .output()
        .unwrap(),
    );
    assert!(!json.contains("\x1b["));
    assert!(json.ends_with('\n'));
    let capabilities: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(capabilities["commands"]["describe"], "verified");
    assert_eq!(capabilities["commands"]["rebase"], "unqualified");
    let error = command(
        temp.path(),
        &[
            "store",
            "attach",
            "work",
            "--endpoint",
            "https://name:secret@example.invalid",
            "--repo-id",
            &REPO.to_string(),
        ],
    )
    .output()
    .unwrap();
    assert!(!error.status.success());
    assert!(!String::from_utf8_lossy(&error.stderr).contains("name:secret"));
    assert!(!temp.path().join("work").exists());
    let a =
        jj_store_cli::binding::Connection::new("https://EXAMPLE.invalid:443", REPO, false, None)
            .unwrap();
    let b = jj_store_cli::binding::Connection::new("https://example.invalid/", REPO, false, None)
        .unwrap();
    assert_eq!(
        jj_store_cli::credentials::account(&a),
        jj_store_cli::credentials::account(&b)
    );
    let c =
        jj_store_cli::binding::Connection::new("https://other.invalid", REPO, false, None).unwrap();
    assert_ne!(
        jj_store_cli::credentials::account(&a),
        jj_store_cli::credentials::account(&c)
    );
}

#[test]
#[ignore = "writes and deletes one randomly scoped test credential in the real OS facility"]
fn os_credential_facility_roundtrip() {
    use jj_store_cli::{binding::Connection, credentials};
    assert!(
        !credentials::environment_present(),
        "run this fixture without JJ_STORE_TOKEN"
    );
    let mut random = [0; 16];
    getrandom::fill(&mut random).unwrap();
    let repo = m::RepositoryId::new(random);
    let connection =
        Connection::new("https://credential-test.example.invalid", repo, false, None).unwrap();
    let mut policy = TokenPolicy::new(Scope::Repository(repo));
    let token = policy.issue(ACTOR, Permission::Read, None).unwrap();
    struct Cleanup(Connection);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = credentials::remove(&self.0);
        }
    }
    let cleanup = Cleanup(connection.clone());
    credentials::save(&connection, &token).unwrap();
    let recovered = credentials::resolve(&connection).unwrap();
    assert_eq!(recovered.id(), token.id());
    policy.authorize(&recovered, Permission::Read, 0).unwrap();
    credentials::remove(&connection).unwrap();
    assert!(credentials::resolve(&connection).is_err());
    drop(cleanup);
}

#[cfg(unix)]
#[test]
fn custom_terminal_paging_and_color_policy() {
    let output = Command::new("python3")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/terminal.py"
        ))
        .arg(env!("CARGO_BIN_EXE_jj"))
        .output()
        .unwrap();
    success(output);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn attach_rejects_read_only_access_before_creating_local_state() {
    let mut f = Fixture::new().await;
    let work = f.root.path().join("denied-work");
    let cache = f.root.path().join("denied-cache");
    let output = f
        .command(
            f.root.path(),
            &[
                "store",
                "attach",
                work.to_str().unwrap(),
                "--endpoint",
                &format!("http://{}", f.address),
                "--repo-id",
                &REPO.to_string(),
                "--insecure-loopback",
                "--cache",
                cache.to_str().unwrap(),
            ],
        )
        .env("JJ_STORE_TOKEN", f.reader.expose().as_str())
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(!work.exists(), "denied attach created a physical directory");
    assert!(!cache.exists(), "denied attach created cache metadata");
    f.stop().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "runs login/logout against a generated token in the real OS credential facility"]
async fn executable_login_logout_uses_os_credentials_across_processes() {
    use std::{io::Write as _, process::Stdio};
    let mut f = Fixture::new().await;
    f.attach("keyring-work");
    let work = f.root.path().join("keyring-work");
    let endpoint = format!("http://{}", f.address);
    let connection = jj_store_cli::binding::Connection::new(&endpoint, REPO, true, None).unwrap();
    struct Cleanup(jj_store_cli::binding::Connection);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = jj_store_cli::credentials::remove(&self.0);
        }
    }
    let cleanup = Cleanup(connection);
    let mut process = f
        .command(
            &work,
            &[
                "store",
                "login",
                "--endpoint",
                &endpoint,
                "--repo-id",
                &REPO.to_string(),
                "--insecure-loopback",
                "--token-stdin",
                "--format",
                "json",
            ],
        )
        .env_remove("JJ_STORE_TOKEN")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    process
        .stdin
        .take()
        .unwrap()
        .write_all(f.token.expose().as_bytes())
        .unwrap();
    let saved: serde_json::Value =
        serde_json::from_str(&success(process.wait_with_output().unwrap())).unwrap();
    assert_eq!(saved["saved"], true);
    let status = success(
        f.command(&work, &["store", "status", "--format=json"])
            .env_remove("JJ_STORE_TOKEN")
            .output()
            .unwrap(),
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&status).unwrap()["repository"],
        REPO.to_string()
    );
    assert!(!status.contains(f.token.expose().as_str()));
    let logout = success(
        f.command(
            &work,
            &[
                "store",
                "logout",
                "--endpoint",
                &endpoint,
                "--repo-id",
                &REPO.to_string(),
                "--insecure-loopback",
                "--format=json",
            ],
        )
        .env_remove("JJ_STORE_TOKEN")
        .output()
        .unwrap(),
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&logout).unwrap()["removed"],
        true
    );
    let denied = f
        .command(&work, &["store", "status"])
        .env_remove("JJ_STORE_TOKEN")
        .output()
        .unwrap();
    assert!(!denied.status.success());
    assert!(String::from_utf8_lossy(&denied.stderr).contains("login"));
    drop(cleanup);
    f.stop().await;
}

#[test]
#[ignore = "writes and deletes one random initial-admin account in the OS credential facility"]
fn os_credential_initial_admin_provisioning_is_repeatable() {
    use jj_store_cli::{binding::Connection, credentials};
    assert!(!credentials::environment_present());
    let mut id = [0; 16];
    getrandom::fill(&mut id).unwrap();
    let connection = Connection::new(
        "https://initial-admin-test.example.invalid",
        jj_store_model::RepositoryId::new(id),
        false,
        None,
    )
    .unwrap();
    struct Cleanup(Connection);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = credentials::remove(&self.0);
        }
    }
    let _cleanup = Cleanup(connection.clone());
    let first = credentials::provision(&connection).unwrap();
    let repeated = credentials::provision(&connection).unwrap();
    assert_eq!(first.expose(), repeated.expose());
    assert_eq!(
        credentials::resolve(&connection).unwrap().expose(),
        first.expose()
    );
}
