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
    listener: std::net::TcpListener,
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
        // Retain the socket reservation across service restarts. Dropping and
        // rebinding an ephemeral port races with other parallel test clients.
        // Standalone-process acceptance separately qualifies cold TCP startup.
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let mut f = Self {
            store,
            token,
            reader,
            address: listener.local_addr().unwrap(),
            listener,
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
        let listener =
            tokio::net::TcpListener::from_std(self.listener.try_clone().unwrap()).unwrap();
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
    assert_eq!(capabilities["commands"]["rebase"], "verified");
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

/// The reference uses the pinned upstream Git backend through the same CliRunner.
/// IDs, timestamps and backend-specific signatures are intentionally excluded
/// from semantic graph comparisons; content and native relationships are not.
struct SemanticPair<'a> {
    fixture: &'a Fixture,
    native: std::path::PathBuf,
    reference: std::path::PathBuf,
}
impl<'a> SemanticPair<'a> {
    fn new(fixture: &'a Fixture) -> Self {
        fixture.attach("semantic-native");
        fixture.run(
            fixture.root.path(),
            &["git", "init", "semantic-reference", "--colocate"],
        );
        Self {
            fixture,
            native: fixture.root.path().join("semantic-native"),
            reference: fixture.root.path().join("semantic-reference"),
        }
    }
    fn roots(&self) -> [&Path; 2] {
        [&self.native, &self.reference]
    }
    fn both(&self, args: &[&str]) -> [String; 2] {
        self.roots().map(|path| {
            let output = self.fixture.command(path, args).output().unwrap();
            assert!(
                output.status.success(),
                "{} {args:?}: {}",
                path.display(),
                String::from_utf8_lossy(&output.stderr)
            );
            String::from_utf8(output.stdout).unwrap()
        })
    }
    fn write(&self, path: &str, bytes: impl AsRef<[u8]>) {
        for root in self.roots() {
            fs::write(root.join(path), bytes.as_ref()).unwrap();
        }
    }
    fn graph(&self) {
        let outputs=self.both(&["log","--no-graph","-r","all() & ~root()","-T","description.trim_end() ++ '|' ++ parents.len() ++ ':' ++ parents.map(|p| p.description().trim_end()).join(',') ++ '|' ++ local_bookmarks ++ '|' ++ conflict ++ '\n'"]);
        let sorted = outputs.map(|s| {
            let mut lines = s.lines().map(str::to_owned).collect::<Vec<_>>();
            lines.sort();
            lines
        });
        assert_eq!(
            sorted[0], sorted[1],
            "native and upstream semantic graphs differ"
        );
    }
    fn same(&self, args: &[&str]) -> String {
        let [native, reference] = self.both(args);
        assert_eq!(native, reference, "{args:?}");
        native
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_cli_semantic_rebase_squash_split_abandon_and_undo_match_upstream() {
    let f = Fixture::new().await;
    let p = SemanticPair::new(&f);
    p.write("base.txt", "base\n");
    p.both(&["describe", "-m", "base"]);
    p.both(&["bookmark", "create", "base"]);
    p.both(&["new", "base", "-m", "left"]);
    p.write("left.txt", "left\n");
    p.both(&["describe", "-m", "left"]);
    p.both(&["bookmark", "create", "left"]);
    p.both(&["new", "base", "-m", "right"]);
    p.write("right.txt", "right\n");
    p.both(&["describe", "-m", "right"]);
    p.both(&["bookmark", "create", "right"]);
    p.both(&["new", "left", "right", "-m", "merge"]);
    p.both(&["bookmark", "create", "merged"]);
    p.graph();
    assert_eq!(p.same(&["file", "show", "left.txt"]), "left\n");
    assert_eq!(p.same(&["file", "show", "right.txt"]), "right\n");
    p.both(&["new", "base", "-m", "target"]);
    p.both(&["bookmark", "create", "target"]);
    p.both(&["rebase", "-s", "left", "-d", "target"]);
    p.graph();
    p.both(&["new", "merged", "-m", "tail"]);
    p.write("tail.txt", "tail\n");
    p.both(&["describe", "-m", "tail"]);
    p.both(&["squash", "-m", "combined"]);
    p.graph();
    assert_eq!(p.same(&["file", "show", "tail.txt"]), "tail\n");
    p.both(&["new", "base", "-m", ""]);
    p.write("a.txt", "selected\n");
    p.write("b.txt", "remainder\n");
    p.both(&["split", "a.txt", "-m", "selected"]);
    assert_eq!(
        p.same(&[
            "log",
            "--no-graph",
            "-r",
            "subject(\"selected\")",
            "-T",
            "description"
        ])
        .trim(),
        "selected"
    );
    p.both(&["edit", "children(subject(\"selected\"))"]);
    p.both(&["describe", "-m", "remainder"]);
    p.both(&["bookmark", "create", "child"]);
    p.graph();
    p.both(&["abandon", "-r", "subject(\"selected\")"]);
    p.graph();
    for root in p.roots() {
        assert!(!root.join("a.txt").exists());
        assert_eq!(fs::read(root.join("b.txt")).unwrap(), b"remainder\n");
    }
    p.same(&[
        "evolog",
        "--no-graph",
        "-r",
        "@",
        "-T",
        "commit.description()",
    ]);
    p.both(&["undo"]);
    p.graph();
    for root in p.roots() {
        assert_eq!(fs::read(root.join("a.txt")).unwrap(), b"selected\n");
    }
    assert!(!p.native.join(".git").exists());
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_cli_semantic_multiway_conflict_snapshot_resolution_matches_upstream() {
    let f = Fixture::new().await;
    let p = SemanticPair::new(&f);
    p.write("conflict.txt", "base\n");
    p.both(&["describe", "-m", "base"]);
    p.both(&["bookmark", "create", "base"]);
    for name in ["left", "right", "third"] {
        p.both(&["new", "base", "-m", name]);
        p.write("conflict.txt", format!("{name}\n"));
        p.both(&["describe", "-m", name]);
        p.both(&["bookmark", "create", name]);
    }
    p.both(&["new", "left", "right", "third", "-m", "multiway"]);
    p.graph();
    let conflicts = p.same(&["resolve", "--list"]);
    assert!(conflicts.contains("conflict.txt"));
    p.same(&["diff", "--summary"]);
    p.write("conflict.txt", "resolved content\n");
    p.both(&["describe", "-m", "resolved"]);
    p.graph();
    assert_eq!(
        p.same(&["file", "show", "conflict.txt"]),
        "resolved content\n"
    );
    assert_eq!(
        p.same(&["log", "--no-graph", "-r", "@", "-T", "conflict"]),
        "false"
    );
    let evolution = p.same(&["evolog", "--no-graph", "-T", "commit.description()"]);
    assert!(evolution.contains("multiway"));
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_cli_semantic_divergence_conflicted_bookmarks_and_operation_reconciliation() {
    let f = Fixture::new().await;
    let p = SemanticPair::new(&f);
    p.write("file.txt", "common\n");
    p.both(&["describe", "-m", "base"]);
    p.both(&["bookmark", "create", "branch"]);
    let before = p.both(&["op", "log", "--no-graph", "--limit", "1", "-T", "id"]);
    p.both(&["describe", "-m", "first divergence"]);
    for (root, id) in p.roots().into_iter().zip(&before) {
        let output = f
            .command(
                root,
                &[
                    "--ignore-working-copy",
                    "--at-op",
                    id.trim(),
                    "describe",
                    "-m",
                    "second divergence",
                ],
            )
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    assert_eq!(f.snapshot().await.heads.len(), 2);
    p.graph(); // ordinary upstream loading reconciles operation siblings
    assert_eq!(f.snapshot().await.heads.len(), 1);
    let divergent = p.both(&[
        "log",
        "--no-graph",
        "-r",
        "divergent()",
        "-T",
        "description",
    ]);
    for output in divergent {
        assert!(output.contains("first divergence") && output.contains("second divergence"));
    }
    for output in p.both(&["bookmark", "list"]) {
        assert!(output.contains("conflicted"), "{output}");
    }
    p.both(&[
        "bookmark",
        "set",
        "branch",
        "-r",
        "subject(\"first divergence\")",
    ]);
    p.graph();
    for output in p.both(&["bookmark", "list"]) {
        assert!(!output.contains("conflicted"));
    }
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_cli_semantic_binary_symlink_executable_and_copied_files_match_upstream() {
    use std::os::unix::fs::{PermissionsExt as _, symlink};
    let f = Fixture::new().await;
    let p = SemanticPair::new(&f);
    let binary = [0, 255, 128, 0, 10, 13];
    p.write("binary.bin", binary);
    p.write("source.txt", "source bytes\n");
    p.write("executable.sh", "#!/bin/sh\nexit 0\n");
    for root in p.roots() {
        fs::set_permissions(
            root.join("executable.sh"),
            fs::Permissions::from_mode(0o755),
        )
        .unwrap();
        symlink("source.txt", root.join("link")).unwrap();
    }
    p.both(&["describe", "-m", "file variants"]);
    p.both(&["bookmark", "create", "files"]);
    p.graph();
    p.both(&["new", "root()", "-m", "empty checkout"]);
    for root in p.roots() {
        assert!(fs::symlink_metadata(root.join("link")).is_err());
        assert!(!root.join("binary.bin").exists());
    }
    p.both(&["new", "files", "-m", "restored checkout"]);
    for root in p.roots() {
        assert_eq!(
            fs::read_link(root.join("link")).unwrap(),
            Path::new("source.txt")
        );
        assert_eq!(
            fs::metadata(root.join("executable.sh"))
                .unwrap()
                .permissions()
                .mode()
                & 0o111,
            0o111
        );
        assert_eq!(fs::read(root.join("binary.bin")).unwrap(), binary);
        let output = f
            .command(root, &["file", "show", "binary.bin"])
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout, binary);
        fs::copy(root.join("source.txt"), root.join("copied.txt")).unwrap();
        fs::rename(root.join("source.txt"), root.join("renamed.txt")).unwrap();
    }
    p.both(&["describe", "-m", "filesystem copy and rename"]);
    p.graph();
    p.same(&["file", "list"]);
    assert_eq!(p.same(&["file", "show", "copied.txt"]), "source bytes\n");
    assert_eq!(p.same(&["file", "show", "renamed.txt"]), "source bytes\n");
    p.both(&["file", "chmod", "n", "executable.sh"]);
    p.graph();
    for root in p.roots() {
        assert_eq!(
            fs::metadata(root.join("executable.sh"))
                .unwrap()
                .permissions()
                .mode()
                & 0o111,
            0
        );
    }
}
#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_cli_semantic_ssh_sign_resign_and_unsign_preserve_verification() {
    let f = Fixture::new().await;
    let p = SemanticPair::new(&f);
    let key = f.root.path().join("disposable-signing-key");
    let output = Command::new("ssh-keygen")
        .args([
            "-q",
            "-t",
            "ed25519",
            "-N",
            "",
            "-C",
            "native-parity-test",
            "-f",
        ])
        .arg(&key)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let allowed = f.root.path().join("allowed-signers");
    fs::write(
        &allowed,
        format!(
            "native@example.invalid {}",
            fs::read_to_string(key.with_extension("pub")).unwrap()
        ),
    )
    .unwrap();
    use std::io::Write as _;
    writeln!(
        fs::OpenOptions::new()
            .append(true)
            .open(f.root.path().join("jj.toml"))
            .unwrap(),
        "\n[signing]\nbackend = 'ssh'\nkey = {}\n[signing.backends.ssh]\nallowed-signers = {}",
        serde_json::to_string(key.to_str().unwrap()).unwrap(),
        serde_json::to_string(allowed.to_str().unwrap()).unwrap()
    )
    .unwrap();
    p.write("signed.txt", "signed native bytes\n");
    p.both(&["describe", "-m", "signed change"]);
    p.both(&["sign", "-r", "@"]);
    assert_eq!(
        p.same(&["log", "--no-graph", "-r", "@", "-T", "signature.status()"]),
        "good"
    );
    p.both(&["sign", "-r", "@"]);
    assert_eq!(
        p.same(&["log", "--no-graph", "-r", "@", "-T", "signature.status()"]),
        "good"
    );
    p.graph();
    p.both(&["unsign", "-r", "@"]);
    assert_eq!(
        p.same(&[
            "log",
            "--no-graph",
            "-r",
            "@",
            "-T",
            "if(signature, 'signed', 'unsigned')"
        ]),
        "unsigned"
    );
    p.graph();
    assert_eq!(
        p.same(&["file", "show", "signed.txt"]),
        "signed native bytes\n"
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_cli_semantic_git_transport_and_gc_fail_without_publication() {
    let f = Fixture::new().await;
    f.attach("unsupported");
    let root = f.root.path().join("unsupported");
    let before = f.snapshot().await;
    for args in [
        vec!["git", "export"],
        vec!["git", "fetch", "--remote", "nonexistent"],
        vec!["util", "gc"],
    ] {
        let output = f.command(&root, &args).output().unwrap();
        assert!(!output.status.success(), "{args:?}");
        assert_eq!(
            f.snapshot().await,
            before,
            "unsupported command published {args:?}"
        );
    }
    assert!(!root.join(".git").exists());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn executable_workstore_creates_tasks_comments_and_recovers_without_an_upstream() {
    let mut f = Fixture::new().await;
    f.attach("workstore");
    let work = f.root.path().join("workstore");
    let request = m::RequestId::new([150; 16]).to_string();
    let accepted: serde_json::Value = serde_json::from_str(&f.run(
        &work,
        &[
            "project",
            "create",
            "--title",
            "Release",
            "--request-id",
            &request,
            "--format",
            "json",
        ],
    ))
    .unwrap();
    let project = accepted["receipt"]["result"]["project"]["id"]
        .as_str()
        .unwrap();
    let task_request = m::RequestId::new([151; 16]).to_string();
    let args = [
        "task",
        "create",
        "--project",
        project,
        "--title",
        "Implement native tasks",
        "--expected-project-version",
        "1",
        "--request-id",
        &task_request,
        "--format",
        "json",
    ];
    let task: serde_json::Value = serde_json::from_str(&f.run(&work, &args)).unwrap();
    let task_id = task["receipt"]["result"]["task"]["id"].as_str().unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&f.run(&work, &args)).unwrap(),
        task
    );
    let comment_request = m::RequestId::new([152; 16]).to_string();
    f.run(
        &work,
        &[
            "task",
            "comment",
            task_id,
            "--expected-task-version",
            "1",
            "--body",
            "Persisted discussion",
            "--request-id",
            &comment_request,
        ],
    );
    let before = f.snapshot().await;
    let denied = f
        .command(
            &work,
            &[
                "project",
                "create",
                "--title",
                "Denied",
                "--request-id",
                &m::RequestId::new([153; 16]).to_string(),
            ],
        )
        .env("JJ_STORE_TOKEN", f.reader.expose().as_str())
        .output()
        .unwrap();
    assert!(!denied.status.success());
    assert_eq!(f.snapshot().await, before);
    f.stop().await;
    f.start().await;
    let status: serde_json::Value = serde_json::from_str(&f.run(
        &work,
        &["task", "status", &task_request, "--format", "json"],
    ))
    .unwrap();
    assert_eq!(status, task);
    let shown: serde_json::Value =
        serde_json::from_str(&f.run(&work, &["task", "show", task_id, "--format", "json"]))
            .unwrap();
    assert_eq!(shown["comments"][0]["body"], "Persisted discussion");
    assert_eq!(shown["tasks"][0]["version"], 2);
    assert_eq!(f.snapshot().await, before);
    f.stop().await;
}
