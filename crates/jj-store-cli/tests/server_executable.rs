//! Cold server-process and command checks. The regular suite does not assume
//! RustFS is running; deployment acceptance is an explicit external fixture.
use jj_store_model::PrincipalId;
use jj_store_service::auth::{BearerToken, Permission, Scope, TokenPolicy};
use std::{
    fs,
    io::{BufRead as _, BufReader},
    path::{Path, PathBuf},
    process::{Child, Command, Output, Stdio},
    time::{Duration, Instant},
};

fn protected_http_client() -> reqwest::ClientBuilder {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("x-jj-store-protocol", "1".parse().unwrap());
    reqwest::Client::builder().default_headers(headers)
}
struct Fixture {
    root: tempfile::TempDir,
    config: PathBuf,
    operator: BearerToken,
    real: bool,
    search: Option<BearerToken>,
}
fn command() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_jj-store"));
    cmd.args(["--no-pager", "--color=never"])
        .env_remove("JJ_STORE_OPERATOR_TOKEN")
        .env_remove("JJ_STORE_TOKEN")
        .env_remove("JJ_STORE_S3_ACCESS_KEY")
        .env_remove("JJ_STORE_S3_SECRET_KEY")
        .env_remove("JJ_STORE_S3_SESSION_TOKEN");
    cmd
}
fn success(output: Output) -> String {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        let identity = jj_store_server::installation::Identity {
            version: 1,
            installation: "ab".repeat(16),
        };
        write_private(
            &root.path().join("installation.json"),
            &serde_json::to_vec(&identity).unwrap(),
        );
        write_private(
            &root.path().join("repositories.json"),
            &serde_json::to_vec(&jj_store_server::installation::Catalog::empty(&identity)).unwrap(),
        );
        let config = root.path().join("server.toml");
        fs::write(
            &config,
            jj_store_server::config::EXAMPLE
                .replace("bind = \"127.0.0.1:7443\"", "bind = \"127.0.0.1:0\""),
        )
        .unwrap();
        let mut policy = TokenPolicy::new(Scope::Operator);
        let operator = policy
            .issue(
                PrincipalId::new([41; 16]),
                Permission::CreateRepository,
                None,
            )
            .unwrap();
        fs::write(root.path().join("operators.json"), policy.encode().unwrap()).unwrap();
        Self {
            root,
            config,
            operator,
            real: false,
            search: None,
        }
    }
    fn enable_tls(&self) -> Vec<u8> {
        let certificate = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let pem = certificate.cert.pem();
        fs::write(self.root.path().join("server.crt"), &pem).unwrap();
        fs::write(
            self.root.path().join("server.key"),
            certificate.signing_key.serialize_pem(),
        )
        .unwrap();
        let config = fs::read_to_string(&self.config)
            .unwrap()
            .replacen("insecure_loopback = true", "insecure_loopback = false", 1)
            .replace("http://127.0.0.1:7443/", "https://localhost:7443/")
            .replace(
                "# [tls]\n# certificate = \"./server.crt\"\n# private_key = \"./server.key\"",
                "[tls]\ncertificate = \"./server.crt\"\nprivate_key = \"./server.key\"",
            );
        fs::write(&self.config, config).unwrap();
        pem.into_bytes()
    }
    fn command(&self) -> Command {
        let mut cmd = command();
        cmd.current_dir(self.root.path())
            .env("JJ_STORE_S3_ACCESS_KEY", "disposable-process-test")
            .env("JJ_STORE_S3_SECRET_KEY", "disposable-process-test-secret");
        if self.real {
            cmd.env(
                "JJ_STORE_S3_ACCESS_KEY",
                std::env::var("JST_TEST_ACCESS_KEY").unwrap(),
            )
            .env(
                "JJ_STORE_S3_SECRET_KEY",
                std::env::var("JST_TEST_SECRET_KEY").unwrap(),
            );
        }
        cmd
    }
    fn start(&self, local: bool) -> Running {
        let mut cmd = if local {
            let mut cmd = Command::new(env!("CARGO_BIN_EXE_jj"));
            cmd.current_dir(self.root.path())
                .args(["--no-pager", "--color=never", "store", "serve-local"])
                .env("JJ_CONFIG", self.root.path().join("empty-jj.toml"))
                .env("JJ_STORE_S3_ACCESS_KEY", "disposable-process-test")
                .env("JJ_STORE_S3_SECRET_KEY", "disposable-process-test-secret")
                .env_remove("JJ_STORE_S3_SESSION_TOKEN");
            fs::write(self.root.path().join("empty-jj.toml"), "").unwrap();
            cmd
        } else {
            let mut cmd = self.command();
            cmd.arg("run");
            cmd
        };
        cmd.env_remove("JJ_STORE_SEARCH_TOKEN_FIXTURE");
        if let Some(token) = &self.search {
            cmd.env("JJ_STORE_SEARCH_TOKEN_FIXTURE", token.expose().as_str());
        }
        cmd.args([
            if local { "--server-config" } else { "--config" },
            self.config.to_str().unwrap(),
            "--format=json",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
        let mut running = Running {
            child: cmd.spawn().unwrap(),
            event: serde_json::Value::Null,
        };
        let out = running.child.stdout.take().unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut line = String::new();
            let _ = BufReader::new(out).read_line(&mut line);
            let _ = tx.send(line);
        });
        let line = rx
            .recv_timeout(Duration::from_secs(10))
            .expect("server did not report listener");
        if line.is_empty() {
            let mut stderr = String::new();
            use std::io::Read as _;
            running
                .child
                .stderr
                .take()
                .unwrap()
                .read_to_string(&mut stderr)
                .unwrap();
            panic!("server exited before listener: {stderr}");
        }
        running.event = serde_json::from_str(&line).unwrap();
        running
    }
}

fn write_private(path: &Path, bytes: &[u8]) {
    use std::io::Write as _;
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let mut file = options.open(path).unwrap();
    file.write_all(bytes).unwrap();
}
struct Running {
    child: Child,
    event: serde_json::Value,
}
impl Drop for Running {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
impl Running {
    #[cfg(unix)]
    fn terminate(&mut self) {
        assert!(
            Command::new("kill")
                .args(["-TERM", &self.child.id().to_string()])
                .status()
                .unwrap()
                .success()
        );
        let deadline = Instant::now() + Duration::from_secs(8);
        loop {
            if let Some(status) = self.child.try_wait().unwrap() {
                if !status.success() {
                    use std::io::Read as _;
                    let mut detail = String::new();
                    self.child
                        .stderr
                        .take()
                        .unwrap()
                        .read_to_string(&mut detail)
                        .unwrap();
                    panic!("server did not shut down cleanly: {status}: {detail}");
                }
                break;
            }
            assert!(Instant::now() < deadline, "server shutdown timed out");
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

#[cfg(unix)]
fn start_local_trusted_server(root: &Path, config: &Path) -> Running {
    let mut cmd = command();
    cmd.current_dir(root)
        .args(["run", "--config", config.to_str().unwrap(), "--format=json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut running = Running {
        child: cmd.spawn().unwrap(),
        event: serde_json::Value::Null,
    };
    let out = running.child.stdout.take().unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut line = String::new();
        let _ = BufReader::new(out).read_line(&mut line);
        let _ = tx.send(line);
    });
    let line = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("LocalTrusted server did not report its listener");
    if line.is_empty() {
        let mut stderr = String::new();
        use std::io::Read as _;
        running
            .child
            .stderr
            .take()
            .unwrap()
            .read_to_string(&mut stderr)
            .unwrap();
        panic!("LocalTrusted server exited before listener: {stderr}");
    }
    running.event = serde_json::from_str(&line).unwrap();
    running
}
#[test]
fn standalone_help_configuration_completions_and_build_info_are_consistent() {
    for args in [
        vec![],
        vec!["--help"],
        vec!["help"],
        vec!["help", "-k", "config"],
        vec!["config", "--help"],
        vec!["init", "--help"],
        vec!["run", "--help"],
        vec!["operator", "--help"],
    ] {
        let output = success(command().args(args).output().unwrap());
        assert!(!output.contains('`'));
        assert!(!output.contains("\u{1b}["));
        assert!(!output.is_empty());
    }
    let short = success(command().args(["run", "-h"]).output().unwrap());
    let long = success(command().args(["run", "--help"]).output().unwrap());
    assert!(long.len() > short.len());
    let version: serde_json::Value = serde_json::from_str(&success(
        command()
            .args(["version", "--format=json"])
            .output()
            .unwrap(),
    ))
    .unwrap();
    assert_eq!(version["server"]["upstream_jj"], "0.45.1");
    for shell in ["bash", "zsh", "fish", "powershell", "elvish"] {
        assert!(
            success(command().args(["completion", shell]).output().unwrap()).contains("jj-store")
        );
    }
    let bad = command().args(["rnu"]).output().unwrap();
    assert!(!bad.status.success());
    assert!(String::from_utf8_lossy(&bad.stderr).contains("Hint:"));
    let f = Fixture::new();
    let check: serde_json::Value = serde_json::from_str(&success(
        command()
            .args([
                "config",
                "check",
                "--config",
                f.config.to_str().unwrap(),
                "--format=json",
            ])
            .output()
            .unwrap(),
    ))
    .unwrap();
    assert_eq!(check["valid"], true);
    assert_eq!(check["storage_readiness"], "not checked");
    assert!(!f.root.path().join("state").exists());
}

#[test]
fn local_init_creates_strict_format_two_configuration() {
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("local");
    let initialized: serde_json::Value = serde_json::from_str(&success(
        command()
            .args([
                "init",
                "--local",
                "--state-dir",
                state.to_str().unwrap(),
                "--rustfs-executable",
                env!("CARGO_BIN_EXE_jj-store"),
                "--format=json",
            ])
            .output()
            .unwrap(),
    ))
    .unwrap();
    assert_eq!(
        initialized["state_dir"],
        state.canonicalize().unwrap().to_string_lossy().as_ref()
    );
    let text = fs::read_to_string(state.join("server.toml")).unwrap();
    assert!(text.contains("mode = \"local-trusted\""));
    assert!(text.contains("mode = \"embedded-rustfs\""));
    assert!(!text.contains("secret_key"));
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires the separately built pinned embedded RustFS runtime"]
async fn local_trusted_executable_bootstraps_registers_and_serves_without_tokens() {
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
    let workspace = root.path().join("workspace");
    let created: serde_json::Value = serde_json::from_str(&success(
        Command::new(env!("CARGO_BIN_EXE_jj"))
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
                "lapis/local-acceptance",
                "--cache",
                root.path().join("cache").to_str().unwrap(),
                "--state-dir",
                root.path().join("client-state").to_str().unwrap(),
                "--workspace",
                "local-acceptance",
                "--unix-socket",
                socket.to_str().unwrap(),
            ])
            .output()
            .unwrap(),
    ))
    .unwrap();
    let repository = created["repository"].as_str().unwrap();
    fs::write(
        workspace.join("search.txt"),
        "searchable local acceptance\n",
    )
    .unwrap();
    success(
        Command::new(env!("CARGO_BIN_EXE_jj"))
            .current_dir(root.path())
            .env("JJ_CONFIG", &jj_config)
            .args([
                "--no-pager",
                "--color=never",
                "-R",
                workspace.to_str().unwrap(),
                "describe",
                "-m",
                "searchable local acceptance",
            ])
            .output()
            .unwrap(),
    );
    let snapshot = http
        .get(format!("{origin}/v1/repos/{repository}/snapshot"))
        .send()
        .await
        .unwrap();
    assert_eq!(snapshot.status(), reqwest::StatusCode::OK);
    let before_link: serde_json::Value = snapshot.json().await.unwrap();
    let linked = http
        .post(format!(
            "{origin}/v1/repos/{repository}/local/workspaces/link"
        ))
        .json(&serde_json::json!({"version":1,"expected_revision":0,"path":workspace}))
        .send()
        .await
        .unwrap();
    assert_eq!(linked.status(), reqwest::StatusCode::OK);
    let linked: serde_json::Value = linked.json().await.unwrap();
    assert_eq!(linked["links"][0]["availability"], "ready");
    assert_eq!(
        linked["links"][0]["path"],
        workspace.canonicalize().unwrap().to_str().unwrap()
    );
    let after_link: serde_json::Value = http
        .get(format!("{origin}/v1/repos/{repository}/snapshot"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(before_link, after_link);
    let doctor: serde_json::Value = serde_json::from_str(&success(
        command()
            .args([
                "doctor",
                "--config",
                config.to_str().unwrap(),
                "--repo-id",
                repository,
                "--format=json",
            ])
            .output()
            .unwrap(),
    ))
    .unwrap();
    assert_eq!(doctor["storage_ready"], true);
    assert_eq!(doctor["repository"], repository);
    assert_eq!(doctor["search"]["state"], "not_configured");
    assert!(
        doctor["heads"]
            .as_array()
            .is_some_and(|heads| !heads.is_empty())
    );
    running.terminate();

    let search = format!(
        r#"
[[search]]
repository = "{repository}"
poll_millis = 10
timeout_seconds = 30
snapshot_every = 1

[search.source.scopes.workspace]
kind = "workspace"
name = "local-acceptance"
"#
    );
    fs::write(
        &config,
        format!(
            "{}{}",
            fs::read_to_string(&config)
                .unwrap()
                .replace("search = []\n", ""),
            search
        ),
    )
    .unwrap();

    let mut restarted = start_local_trusted_server(root.path(), &config);
    let repositories: serde_json::Value = http
        .get(format!("{origin}/v1/repositories"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(repositories["repositories"][0]["id"], repository);
    assert_eq!(
        repositories["repositories"][0]["name"],
        "lapis/local-acceptance"
    );
    let base = format!("{origin}/v1/repos/{repository}");
    let ready = wait_search_ready(&http, None, &base).await;
    assert_eq!(ready["is_stale"], false);
    let capabilities: serde_json::Value = http
        .get(format!("{base}/capabilities"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(capabilities["search"], true);
    assert_eq!(capabilities["local_workspace_links"], true);
    let restored_links: serde_json::Value = http
        .get(format!("{base}/local/workspaces"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(restored_links, linked);

    let result = http
        .get(format!(
            "{base}/search?q=searchable&scope=workspace&mode=historical"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(result.status(), reqwest::StatusCode::OK);
    let result: serde_json::Value = result.json().await.unwrap();
    assert!(
        !result["results"].as_array().unwrap().is_empty(),
        "{result}"
    );
    let doctor: serde_json::Value = serde_json::from_str(&success(
        command()
            .args([
                "doctor",
                "--config",
                config.to_str().unwrap(),
                "--repo-id",
                repository,
                "--format=json",
            ])
            .output()
            .unwrap(),
    ))
    .unwrap();
    assert_eq!(doctor["search"]["state"], "ready");
    restarted.terminate();
}
#[test]
fn operator_commands_provision_list_and_revoke_without_exposing_credentials() {
    let root = tempfile::tempdir().unwrap();
    let policy = root.path().join("operators.json");
    let token = BearerToken::generate().unwrap();
    let principal = PrincipalId::new([42; 16]).to_string();
    for _ in 0..2 {
        let text = success(
            command()
                .env("JJ_STORE_OPERATOR_TOKEN", token.expose().as_str())
                .args([
                    "operator",
                    "init",
                    "--policy",
                    policy.to_str().unwrap(),
                    "--endpoint",
                    "http://127.0.0.1:7443",
                    "--insecure-loopback",
                    "--principal",
                    &principal,
                    "--format=json",
                ])
                .output()
                .unwrap(),
        );
        assert!(!text.contains(&token.expose()[38..]));
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&text).unwrap()["token_id"],
            token.id().to_string()
        );
    }
    let listed = success(
        command()
            .args([
                "operator",
                "list",
                "--policy",
                policy.to_str().unwrap(),
                "--format=json",
            ])
            .output()
            .unwrap(),
    );
    assert!(!listed.contains("verifier"));
    assert!(!listed.contains("jst1_"));
    success(
        command()
            .args([
                "operator",
                "revoke",
                "--policy",
                policy.to_str().unwrap(),
                "--token-id",
                &token.id().to_string(),
            ])
            .output()
            .unwrap(),
    );
    let listed = success(
        command()
            .args([
                "operator",
                "list",
                "--policy",
                policy.to_str().unwrap(),
                "--format=json",
            ])
            .output()
            .unwrap(),
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&listed).unwrap()["tokens"],
        serde_json::json!([])
    );
    assert!(
        !fs::read_to_string(policy)
            .unwrap()
            .contains(&token.expose()[38..])
    );
}

#[test]
fn token_generate_prints_one_valid_manual_credential() {
    let generated: serde_json::Value = serde_json::from_str(&success(
        command()
            .args(["--format=json", "token", "generate"])
            .output()
            .unwrap(),
    ))
    .unwrap();
    let credential = generated["credential"].as_str().unwrap();
    let token = BearerToken::parse(credential).unwrap();
    assert_eq!(generated["version"], 1);
    assert_eq!(generated["token_id"], token.id().to_string());
}

#[test]
fn registry_init_creates_one_private_authenticated_catalog() {
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("authenticated");
    let initialized: serde_json::Value = serde_json::from_str(&success(
        command()
            .args([
                "--format=json",
                "registry",
                "init",
                "--state-dir",
                state.to_str().unwrap(),
            ])
            .output()
            .unwrap(),
    ))
    .unwrap();
    assert_eq!(initialized["version"], 1);
    let canonical = state.canonicalize().unwrap();
    assert_eq!(initialized["state_dir"], canonical.to_str().unwrap());
    assert!(state.join("installation.json").is_file());
    assert!(state.join("repositories.json").is_file());
    assert!(
        !command()
            .args(["registry", "init", "--state-dir", state.to_str().unwrap()])
            .output()
            .unwrap()
            .status
            .success()
    );
}
#[cfg(unix)]
#[test]
fn cold_server_and_serve_local_hold_state_lock_and_stop_on_termination() {
    let f = Fixture::new();
    for local in [false, true] {
        let mut running = f.start(local);
        assert_eq!(running.event["event"], "listening");
        assert_eq!(running.event["storage_readiness"], "unverified");
        let second = f
            .command()
            .args(["run", "--config", f.config.to_str().unwrap()])
            .output()
            .unwrap();
        assert!(!second.status.success());
        assert!(String::from_utf8_lossy(&second.stderr).contains("lock"));
        running.terminate();
    }
}
#[test]
fn server_rejects_missing_credentials_invalid_tls_and_shared_plaintext_before_listening() {
    let f = Fixture::new();
    let no_credentials = command()
        .args(["run", "--config", f.config.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!no_credentials.status.success());
    assert!(!String::from_utf8_lossy(&no_credentials.stdout).contains("listening"));
    fs::write(
        &f.config,
        jj_store_server::config::EXAMPLE.replace("127.0.0.1:7443", "0.0.0.0:7443"),
    )
    .unwrap();
    let insecure = f
        .command()
        .args(["run", "--config", f.config.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!insecure.status.success());
    assert!(String::from_utf8_lossy(&insecure.stderr).contains("TLS"));
    fs::write(f.root.path().join("server.crt"), "not a certificate").unwrap();
    fs::write(
        f.root.path().join("server.key"),
        "secret-that-must-not-appear",
    )
    .unwrap();
    fs::write(
        &f.config,
        jj_store_server::config::EXAMPLE
            .replace("public_url = \"http:", "public_url = \"https:")
            .replace(
                "# [tls]\n# certificate = \"./server.crt\"\n# private_key = \"./server.key\"",
                "[tls]\ncertificate = \"./server.crt\"\nprivate_key = \"./server.key\"",
            ),
    )
    .unwrap();
    let tls = f
        .command()
        .args(["run", "--config", f.config.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!tls.status.success());
    assert!(!String::from_utf8_lossy(&tls.stderr).contains("secret-that-must-not-appear"));
    assert!(!String::from_utf8_lossy(&tls.stdout).contains("listening"));
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cold_tls_server_checks_trust_hostname_and_plaintext() {
    use jj_store_rpc::client::{NativeClient, Options};
    let f = Fixture::new();
    let pem = f.enable_tls();
    let mut running = f.start(false);
    assert_eq!(running.event["transport"], "tls");
    let address = running.event["address"].as_str().unwrap();
    let port = address.parse::<std::net::SocketAddr>().unwrap().port();
    let endpoint = format!("https://localhost:{port}");
    let repo = jj_store_model::RepositoryId::new([43; 16]);
    assert!(
        NativeClient::connect(&endpoint, repo, f.operator.clone(), Options::default())
            .await
            .is_err()
    );
    let trusted = Options {
        ca_certificate_pem: Some(pem),
        ..Options::default()
    };
    assert!(
        NativeClient::connect(
            &format!("https://{address}"),
            repo,
            f.operator.clone(),
            trusted.clone()
        )
        .await
        .is_err()
    );
    // Tonic can return a plaintext HTTP/2 channel before the peer rejects its
    // preface. Drive an RPC so acceptance cannot stop at TCP establishment.
    let plaintext = NativeClient::connect(
        &format!("http://{address}"),
        repo,
        f.operator.clone(),
        Options {
            insecure_loopback: true,
            timeout: Duration::from_secs(2),
            ..Options::default()
        },
    )
    .await;
    match plaintext {
        Ok(client) => {
            let error = client.heads().await.unwrap_err();
            // The h2 preface can race the TLS alert and map to Cancelled,
            // Unknown or Unavailable. A local transport source, rather than a
            // decoded remote application status, is the relevant invariant.
            assert!(
                std::error::Error::source(&error)
                    .is_some_and(|source| source.is::<tonic::transport::Error>()),
                "plaintext reached application handling: {error}"
            );
        }
        Err(error) => assert_eq!(error.code(), tonic::Code::Unavailable),
    }
    // A completed TLS handshake needs no storage access. Authenticated native
    // requests are separately exercised by the real RustFS deployment fixture.
    let client = NativeClient::connect(&endpoint, repo, f.operator.clone(), trusted)
        .await
        .unwrap();
    drop(client);
    running.terminate();
}

fn native(f: &Fixture, token: &BearerToken, args: &[&str]) -> Output {
    native_command(f, token, args).output().unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cold_tls_server_serves_only_the_configured_browser_bundle() {
    let f = Fixture::new();
    let pem = f.enable_tls();
    let root = f.root.path().join("browser");
    fs::create_dir_all(root.join("assets")).unwrap();
    fs::write(
        root.join("index.html"),
        "<!doctype html><title>Native browser</title>",
    )
    .unwrap();
    fs::write(root.join("assets/app-123.js"), "export const version=1;").unwrap();
    fs::write(
        root.join("asset-manifest.json"),
        r#"{"version":1,"files":["index.html","assets/app-123.js"]}"#,
    )
    .unwrap();
    fs::write(
        &f.config,
        fs::read_to_string(&f.config).unwrap().replace(
            "operator_policy = \"./operators.json\"",
            "operator_policy = \"./operators.json\"\nweb_root = \"./browser\"",
        ),
    )
    .unwrap();
    let mut running = f.start(false);
    let port = running.event["address"]
        .as_str()
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap()
        .port();
    let client = protected_http_client()
        .no_proxy()
        .timeout(Duration::from_secs(3))
        .add_root_certificate(reqwest::Certificate::from_pem(&pem).unwrap())
        .build()
        .unwrap();
    let base = format!("https://localhost:{port}");
    let response = client.get(&base).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert!(
        response.headers()["content-security-policy"]
            .to_str()
            .unwrap()
            .contains("script-src 'self'")
    );
    assert!(response.text().await.unwrap().contains("Native browser"));
    let response = client
        .get(format!("{base}/assets/app-123.js"))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(
        response.headers()["content-type"],
        "text/javascript; charset=utf-8"
    );
    assert_eq!(
        client
            .get(format!("{base}/asset-manifest.json"))
            .send()
            .await
            .unwrap()
            .status(),
        reqwest::StatusCode::NOT_FOUND
    );
    let unknown_grpc = client
        .post(format!("{base}/unknown.Service/Call"))
        .header("content-type", "application/grpc+proto")
        .body(Vec::new())
        .send()
        .await
        .unwrap();
    assert_eq!(unknown_grpc.status(), reqwest::StatusCode::OK);
    assert_eq!(unknown_grpc.headers()["grpc-status"], "12");
    assert_eq!(
        client
            .get(format!("{base}/v1/unknown"))
            .header(reqwest::header::HOST, "localhost:7443")
            .send()
            .await
            .unwrap()
            .status(),
        reqwest::StatusCode::NOT_FOUND
    );

    assert_eq!(
        client
            .get(format!(
                "{base}/v1/repos/{}/snapshot",
                jj_store_model::RepositoryId::new([1; 16])
            ))
            .header(reqwest::header::HOST, "localhost:7443")
            .send()
            .await
            .unwrap()
            .status(),
        reqwest::StatusCode::UNAUTHORIZED
    );
    #[cfg(unix)]
    running.terminate();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cold_tls_http_reads_support_http1_and_http2() {
    let f = Fixture::new();
    let pem = f.enable_tls();
    let mut running = f.start(false);
    let port = running.event["address"]
        .as_str()
        .unwrap()
        .parse::<std::net::SocketAddr>()
        .unwrap()
        .port();
    let url = format!(
        "https://localhost:{port}/v1/repos/{}/snapshot",
        jj_store_model::RepositoryId::new([43; 16])
    );
    // An incomplete TLS handshake must not serialize acceptance of good clients.
    let mut stalled_handshake = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .unwrap();
    for version in [reqwest::Version::HTTP_11, reqwest::Version::HTTP_2] {
        let builder = protected_http_client()
            .no_proxy()
            .timeout(Duration::from_secs(3))
            .add_root_certificate(reqwest::Certificate::from_pem(&pem).unwrap());
        let client = if version == reqwest::Version::HTTP_11 {
            builder.http1_only()
        } else {
            builder.http2_prior_knowledge()
        }
        .build()
        .unwrap();
        let response = client
            .get(&url)
            .header(reqwest::header::HOST, "localhost:7443")
            .send()
            .await
            .unwrap();
        assert_eq!(response.version(), version);
        assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
        assert_eq!(response.headers()["cache-control"], "no-store");
        assert_eq!(
            response.json::<serde_json::Value>().await.unwrap()["code"],
            "bearer_required"
        );
    }
    use tokio::io::AsyncReadExt as _;
    assert_eq!(
        tokio::time::timeout(
            Duration::from_secs(7),
            stalled_handshake.read(&mut [0u8; 1])
        )
        .await
        .unwrap()
        .unwrap(),
        0
    );
    running.terminate();
}
fn native_command(f: &Fixture, token: &BearerToken, args: &[&str]) -> Command {
    let config = f.root.path().join("native.toml");
    if !config.exists() {
        fs::write(
            &config,
            "[user]\nname='Native Deployment Test'\nemail='native-deployment@example.invalid'\n",
        )
        .unwrap();
    }
    let mut command = Command::new(env!("CARGO_BIN_EXE_jj"));
    command
        .current_dir(f.root.path())
        .env("JJ_CONFIG", config)
        .env("JJ_STORE_TOKEN", token.expose().as_str())
        .env_remove("JJ_STORE_OPERATOR_TOKEN")
        .env_remove("JJ_STORE_GRANT_TOKEN");
    if args.starts_with(&["store", "init"]) {
        command.env("JJ_STORE_OPERATOR_TOKEN", f.operator.expose().as_str());
    }
    command.args(["--no-pager", "--color=never"]).args(args);
    command
}
fn clear_index(path: &Path) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.file_name() == "type" {
            continue;
        }
        if entry.file_type().unwrap().is_dir() {
            fs::remove_dir_all(entry.path()).unwrap();
        } else {
            fs::remove_file(entry.path()).unwrap();
        }
    }
}
#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "requires the owned pinned RustFS harness; kills/restarts only its supplied container"]
async fn real_rustfs_server_cli_restart_and_revocation() {
    use jj_store_model as m;
    use jj_store_rpc::client::{NativeClient, Options};
    let storage_endpoint =
        std::env::var("JST_TEST_ENDPOINT").expect("run through isolated RustFS harness");
    let container = std::env::var("JST_TEST_CONTAINER").expect("owned test container required");
    assert!(container.starts_with("jj-store-m0-") && container.len() < 64);
    let mut f = Fixture::new();
    f.real = true;
    let text = fs::read_to_string(&f.config)
        .unwrap()
        .replace("http://127.0.0.1:9000/", &format!("{storage_endpoint}/"))
        .replace("bucket = \"jj-store\"", "bucket = \"jj-store-contract\"");
    fs::write(&f.config, text).unwrap();
    let pem = f.enable_tls();
    let mut running = f.start(false);
    assert_eq!(running.event["transport"], "tls");
    let address = running.event["address"].as_str().unwrap().to_owned();
    let port = address.parse::<std::net::SocketAddr>().unwrap().port();
    let endpoint = format!("https://localhost:{port}");
    fs::write(
        &f.config,
        fs::read_to_string(&f.config)
            .unwrap()
            .replace("bind = \"127.0.0.1:0\"", &format!("bind = \"{address}\""))
            .replace("https://localhost:7443/", &format!("{endpoint}/")),
    )
    .unwrap();
    let admin = BearerToken::generate().unwrap();
    let init = [
        "store",
        "init",
        "work",
        "--cache",
        "cache",
        "--state-dir",
        "init-state",
        "--workspace",
        "deployment-a",
        "--endpoint",
        &endpoint,
        "--ca-certificate",
        "server.crt",
        "--format=json",
    ];
    let created: serde_json::Value =
        serde_json::from_str(&success(native(&f, &admin, &init))).unwrap();
    let repeated: serde_json::Value =
        serde_json::from_str(&success(native(&f, &admin, &init))).unwrap();
    assert_eq!(created, repeated);
    let repository: m::RepositoryId = created["repository"].as_str().unwrap().parse().unwrap();
    let journal: serde_json::Value =
        serde_json::from_slice(&fs::read(f.root.path().join("init-state/creation.json")).unwrap())
            .unwrap();
    let epoch: m::EpochId = journal["epoch"].as_str().unwrap().parse().unwrap();
    let options = Options {
        ca_certificate_pem: Some(pem.clone()),
        ..Options::default()
    };
    let operator =
        NativeClient::connect(&endpoint, repository, f.operator.clone(), options.clone())
            .await
            .unwrap();
    operator.create_repository(epoch, &admin).await.unwrap();
    fs::write(
        f.root.path().join("work/readme.txt"),
        "durable native publication\n",
    )
    .unwrap();
    let binary = [0, 255, 0, 128, 42];
    fs::write(f.root.path().join("work/binary.bin"), binary).unwrap();
    success(native(
        &f,
        &admin,
        &["-R", "work", "describe", "-m", "first durable description"],
    ));
    success(native(
        &f,
        &admin,
        &["-R", "work", "describe", "-m", "second durable description"],
    ));
    success(native(
        &f,
        &admin,
        &["-R", "work", "bookmark", "create", "main", "-r", "@"],
    ));
    assert!(
        success(native(
            &f,
            &admin,
            &[
                "-R",
                "work",
                "evolog",
                "--no-graph",
                "-T",
                "commit.description()"
            ]
        ))
        .contains("first durable description")
    );
    success(native(
        &f,
        &admin,
        &[
            "store",
            "attach",
            "other",
            "--cache",
            "other-cache",
            "--workspace",
            "deployment-b",
            "--endpoint",
            &endpoint,
            "--repo-id",
            &repository.to_string(),
            "--ca-certificate",
            "server.crt",
        ],
    ));
    success(native(
        &f,
        &admin,
        &["-R", "other", "new", "main", "-m", "independent workspace"],
    ));
    running.terminate();
    fs::write(
        &f.config,
        format!(
            "{}\n[[search]]\nrepository = \"{repository}\"\ntoken_environment = \"JJ_STORE_SEARCH_TOKEN_FIXTURE\"\npoll_millis = 10\ntimeout_seconds = 30\nsnapshot_every = 1\n\n[search.source.scopes.main]\nkind = \"bookmark\"\nname = \"main\"\n",
            fs::read_to_string(&f.config).unwrap()
        ),
    )
    .unwrap();
    f.search = Some(admin.clone());
    let rebuild_started = Instant::now();
    running = f.start(false);
    let http = protected_http_client()
        .no_proxy()
        .http1_only()
        .timeout(Duration::from_secs(30))
        .add_root_certificate(reqwest::Certificate::from_pem(&pem).unwrap())
        .build()
        .unwrap();
    let base = format!("{endpoint}/v1/repos/{repository}");
    let rebuilt_search = wait_search_ready(&http, Some(&admin), &base).await;
    let rebuilt_generation = rebuilt_search["index_generation"]
        .as_str()
        .unwrap()
        .to_owned();
    let rebuilt_page = http_json(
        &http,
        &admin,
        &format!("{base}/search?q=durable&scope=main"),
    )
    .await;
    assert!(!rebuilt_page["results"].as_array().unwrap().is_empty());
    wait_search_snapshot(&storage_endpoint, repository, epoch, &rebuilt_generation).await;
    let rebuild_millis = rebuild_started.elapsed().as_millis();
    let snapshot = http_json(&http, &admin, &format!("{base}/snapshot")).await;
    let at_op = snapshot["heads"].as_array().unwrap()[0].as_str().unwrap();
    let log_url = format!("{base}/log?at_op={at_op}&at_time_millis=1790000000000&revset=all()");
    let graph = http_json(&http, &admin, &log_url).await;
    let nodes = graph["data"]["data"]["nodes"].as_array().unwrap();
    assert!(graph["next_cursor"].is_null());
    let mut http_ids: Vec<_> = nodes
        .iter()
        .map(|n| n["commit"]["id"].as_str().unwrap().to_owned())
        .collect();
    let native_ids = success(native(
        &f,
        &admin,
        &[
            "-R",
            "work",
            "--ignore-working-copy",
            "--at-op",
            at_op,
            "log",
            "-r",
            "all()",
            "--no-graph",
            "-T",
            "commit_id ++ \"\\n\" ",
        ],
    ));
    let mut native_ids: Vec<_> = native_ids.split_whitespace().map(str::to_owned).collect();
    http_ids.sort();
    native_ids.sort();
    assert_eq!(http_ids, native_ids);
    let commit = nodes
        .iter()
        .find(|n| {
            n["commit"]["description"]
                .as_str()
                .unwrap()
                .contains("second durable description")
        })
        .unwrap()["commit"]["id"]
        .as_str()
        .unwrap();
    let tree = http_json(
        &http,
        &admin,
        &format!("{base}/commits/{commit}/tree?at_op={at_op}"),
    )
    .await;
    let file = tree["data"]["entries"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["name"] == "binary.bin")
        .unwrap()["terms"][0]["id"]
        .as_str()
        .unwrap();
    let file_url = format!("{base}/files/{file}?at_op={at_op}");
    let media = http
        .get(&file_url)
        .bearer_auth(admin.expose().as_str())
        .header("range", "bytes=1-3")
        .send()
        .await
        .unwrap();
    assert_eq!(media.status(), reqwest::StatusCode::PARTIAL_CONTENT);
    assert_eq!(media.headers()["x-jj-store-operation"], at_op);
    assert_eq!(media.bytes().await.unwrap().as_ref(), &binary[1..4]);
    assert_eq!(
        http_json(&http, &admin, &format!("{base}/snapshot")).await,
        snapshot,
        "GETs published history"
    );
    let events_url = format!("{base}/events");
    let mut live = http
        .get(&events_url)
        .bearer_auth(admin.expose().as_str())
        .send()
        .await
        .unwrap();
    assert_eq!(live.status(), reqwest::StatusCode::OK);
    let mut buffer = String::new();
    let ready = next_server_event(&mut live, &mut buffer).await.unwrap();
    assert_eq!(ready["type"], "ready");
    let resume = event_cursor(&ready);
    success(native(
        &f,
        &admin,
        &["-R", "work", "bookmark", "create", "streamed", "-r", "main"],
    ));
    let changed = next_server_event(&mut live, &mut buffer).await.unwrap();
    assert_eq!(changed["type"], "changed");
    assert!(
        changed["cursor"]["position"].as_u64().unwrap()
            > ready["cursor"]["position"].as_u64().unwrap()
    );
    drop(live);
    fs::write(
        f.root.path().join("work/local-only.txt"),
        "unsnapshotted local bytes",
    )
    .unwrap();
    let doctor_args = [
        "doctor",
        "--config",
        f.config.to_str().unwrap(),
        "--repo-id",
        &repository.to_string(),
        "--format=json",
    ];
    let diagnosis: serde_json::Value = serde_json::from_str(&success(
        f.command()
            .env("JJ_STORE_TOKEN", admin.expose().as_str())
            .args(doctor_args)
            .output()
            .unwrap(),
    ))
    .unwrap();
    assert_eq!(diagnosis["storage_ready"], true);
    let rebuilt_search = wait_search_ready(&http, Some(&admin), &base).await;
    let rebuilt_generation = rebuilt_search["index_generation"]
        .as_str()
        .unwrap()
        .to_owned();
    let rebuilt_page = http_json(
        &http,
        &admin,
        &format!("{base}/search?q=durable&scope=main"),
    )
    .await;
    wait_search_snapshot(&storage_endpoint, repository, epoch, &rebuilt_generation).await;
    // Kill the actual server, remove its rebuildable local state, then kill and
    // restart the uniquely owned RustFS container supplied by the harness.
    running.child.kill().unwrap();
    running.child.wait().unwrap();
    fs::remove_dir_all(f.root.path().join("state")).unwrap();
    clear_index(&f.root.path().join("cache/index"));
    assert!(
        Command::new("docker")
            .args(["kill", &container])
            .output()
            .unwrap()
            .status
            .success()
    );
    assert!(
        Command::new("docker")
            .args(["start", &container])
            .output()
            .unwrap()
            .status
            .success()
    );
    let port = success(
        Command::new("docker")
            .args(["port", &container, "9000/tcp"])
            .output()
            .unwrap(),
    )
    .trim()
    .rsplit(':')
    .next()
    .unwrap()
    .to_owned();
    let storage_endpoint_new = format!("http://127.0.0.1:{port}");
    fs::write(
        &f.config,
        fs::read_to_string(&f.config)
            .unwrap()
            .replace(&storage_endpoint, &storage_endpoint_new),
    )
    .unwrap();
    let mut restarted = f.start(false);
    assert_eq!(restarted.event["address"], address);
    let restore_started = Instant::now();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let output = f
            .command()
            .env("JJ_STORE_TOKEN", admin.expose().as_str())
            .args(doctor_args)
            .output()
            .unwrap();
        if output.status.success() {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "RustFS metadata readiness did not recover"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let restored_search = wait_search_ready(&http, Some(&admin), &base).await;
    let restore_millis = restore_started.elapsed().as_millis();
    assert_eq!(restored_search["index_generation"], rebuilt_generation);
    let restored_page = http_json(
        &http,
        &admin,
        &format!("{base}/search?q=durable&scope=main"),
    )
    .await;
    assert_eq!(restored_page["index_generation"], rebuilt_generation);
    assert_eq!(restored_page["results"], rebuilt_page["results"]);
    println!(
        "{}",
        serde_json::json!({
            "event": "search_snapshot_qualification",
            "rebuild_millis": rebuild_millis,
            "restore_millis": restore_millis,
            "budget_millis": 30_000,
            "generation": rebuilt_generation,
            "exact_generation_restored": true
        })
    );
    let mut resumed = http
        .get(&events_url)
        .bearer_auth(admin.expose().as_str())
        .header("last-event-id", &resume)
        .send()
        .await
        .unwrap();
    assert_eq!(resumed.status(), reqwest::StatusCode::OK);
    let mut buffer = String::new();
    assert_eq!(
        next_server_event(&mut resumed, &mut buffer).await.unwrap()["type"],
        "ready"
    );
    assert_eq!(
        next_server_event(&mut resumed, &mut buffer).await.unwrap(),
        changed
    );
    drop(resumed);
    // The authoritative operation survives process/cache/storage restart.
    let recovered_graph = http_json(&http, &admin, &log_url).await;
    assert_eq!(recovered_graph["data"]["data"], graph["data"]["data"]);
    let media = http
        .get(&file_url)
        .bearer_auth(admin.expose().as_str())
        .send()
        .await
        .unwrap();
    assert_eq!(media.status(), reqwest::StatusCode::OK);
    assert_eq!(media.bytes().await.unwrap().as_ref(), binary);
    assert_eq!(
        fs::read(f.root.path().join("work/local-only.txt")).unwrap(),
        b"unsnapshotted local bytes"
    );
    assert!(
        success(native(
            &f,
            &admin,
            &["-R", "work", "log", "--no-graph", "-T", "description"]
        ))
        .contains("second durable description")
    );
    assert!(
        success(native(
            &f,
            &admin,
            &["-R", "work", "op", "log", "--no-graph"]
        ))
        .contains("describe")
    );
    assert!(
        success(native(
            &f,
            &admin,
            &[
                "-R",
                "work",
                "evolog",
                "--no-graph",
                "-T",
                "commit.description()"
            ]
        ))
        .contains("first durable description")
    );
    let file = native(
        &f,
        &admin,
        &["-R", "work", "file", "show", "work/binary.bin"],
    );
    assert!(
        file.status.success(),
        "{}",
        String::from_utf8_lossy(&file.stderr)
    );
    assert_eq!(file.stdout, binary);
    assert!(!f.root.path().join("work/.git").exists());
    assert_eq!(
        fs::read(f.root.path().join("cache/store/type")).unwrap(),
        b"jj-store"
    );
    let client = NativeClient::connect(&endpoint, repository, admin.clone(), options.clone())
        .await
        .unwrap();
    let reader = BearerToken::generate().unwrap();
    let granted: serde_json::Value = serde_json::from_str(&success(
        native_command(
            &f,
            &admin,
            &[
                "store",
                "token",
                "--endpoint",
                &endpoint,
                "--repo-id",
                &repository.to_string(),
                "--ca-certificate",
                "server.crt",
                "--format=json",
                "grant",
                "--principal",
                &PrincipalId::new([72; 16]).to_string(),
                "--permission",
                "read",
                "--request",
                "reader-grant.json",
            ],
        )
        .env("JJ_STORE_GRANT_TOKEN", reader.expose().as_str())
        .output()
        .unwrap(),
    ))
    .unwrap();
    assert_eq!(granted["state"], "committed");
    let reader_client = NativeClient::connect(&endpoint, repository, reader.clone(), options)
        .await
        .unwrap();
    reader_client.heads().await.unwrap();
    http_json(&http, &reader, &log_url).await;
    let mut revoked_stream = http
        .get(&events_url)
        .bearer_auth(reader.expose().as_str())
        .send()
        .await
        .unwrap();
    assert_eq!(revoked_stream.status(), reqwest::StatusCode::OK);
    let mut revoked_buffer = String::new();
    assert_eq!(
        next_server_event(&mut revoked_stream, &mut revoked_buffer)
            .await
            .unwrap()["type"],
        "ready"
    );
    let revoked: serde_json::Value = serde_json::from_str(&success(
        f.command()
            .env("JJ_STORE_TOKEN", admin.expose().as_str())
            .args([
                "token",
                "--endpoint",
                &endpoint,
                "--repo-id",
                &repository.to_string(),
                "--ca-certificate",
                "server.crt",
                "--format=json",
                "revoke",
                "--token-id",
                &reader.id().to_string(),
                "--request",
                "reader-revoke.json",
            ])
            .output()
            .unwrap(),
    ))
    .unwrap();
    assert_eq!(revoked["state"], "committed");
    assert!(
        next_server_event(&mut revoked_stream, &mut revoked_buffer)
            .await
            .is_err()
    );
    drop(revoked_stream);
    let denied = http
        .get(&log_url)
        .bearer_auth(reader.expose().as_str())
        .send()
        .await
        .unwrap();
    assert_eq!(denied.status(), reqwest::StatusCode::FORBIDDEN);
    assert_eq!(
        denied.json::<serde_json::Value>().await.unwrap()["code"],
        "access_denied"
    );
    let replay: serde_json::Value = serde_json::from_str(&success(
        native_command(
            &f,
            &admin,
            &[
                "store",
                "token",
                "--endpoint",
                &endpoint,
                "--repo-id",
                &repository.to_string(),
                "--ca-certificate",
                "server.crt",
                "--format=json",
                "retry",
                "--request",
                "reader-grant.json",
            ],
        )
        .env("JJ_STORE_GRANT_TOKEN", reader.expose().as_str())
        .output()
        .unwrap(),
    ))
    .unwrap();
    assert_eq!(replay["receipt"], granted["receipt"]);
    assert_eq!(
        client.tokens().await.unwrap().snapshot.position.0,
        revoked["receipt"]["accepted"]["position"].as_u64().unwrap()
    );
    assert_eq!(
        reader_client.heads().await.unwrap_err().code(),
        tonic::Code::PermissionDenied
    );
    success(
        f.command()
            .args([
                "operator",
                "revoke",
                "--policy",
                f.root.path().join("operators.json").to_str().unwrap(),
                "--token-id",
                &f.operator.id().to_string(),
            ])
            .output()
            .unwrap(),
    );
    assert_eq!(
        operator
            .create_repository(epoch, &admin)
            .await
            .unwrap_err()
            .code(),
        tonic::Code::PermissionDenied
    );
    drop(reader_client);
    drop(client);
    drop(operator);
    restarted.terminate();
}

#[cfg(unix)]
#[test]
fn standalone_terminal_paging_color_and_json_contract() {
    success(
        Command::new("python3")
            .arg(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/terminal.py"
            ))
            .arg(env!("CARGO_BIN_EXE_jj-store"))
            .arg("server")
            .output()
            .unwrap(),
    );
}

async fn http_json(client: &reqwest::Client, token: &BearerToken, url: &str) -> serde_json::Value {
    let response = client
        .get(url)
        .bearer_auth(token.expose().as_str())
        .send()
        .await
        .unwrap();
    let status = response.status();
    let json = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(status, reqwest::StatusCode::OK, "{json}");
    json
}

async fn wait_search_ready(
    client: &reqwest::Client,
    token: Option<&BearerToken>,
    base: &str,
) -> serde_json::Value {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let mut request = client.get(format!("{base}/search/status"));
        if let Some(token) = token {
            request = request.bearer_auth(token.expose().as_str());
        }
        let response = request.send().await.unwrap();
        let status = response.status();
        let value = response.json::<serde_json::Value>().await.unwrap();
        if status == reqwest::StatusCode::OK
            && value["is_stale"] == false
            && value["degraded"] == false
            && value["index_generation"].is_string()
        {
            return value;
        }
        assert!(
            Instant::now() < deadline,
            "search did not become ready: {value}"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

async fn wait_search_snapshot(
    endpoint: &str,
    repository: jj_store_model::RepositoryId,
    epoch: jj_store_model::EpochId,
    expected_generation: &str,
) {
    use jj_store_object::{Key, ObjectStore as _, s3};
    use sha2::{Digest as _, Sha256};

    let source: jj_store_tantivy::native::Config = serde_json::from_value(serde_json::json!({
        "scopes": {"main": {"kind": "bookmark", "name": "main"}}
    }))
    .unwrap();
    let identity = source.identity().unwrap();
    let digest = jj_store_model::RecordDigest::new(
        Sha256::digest(serde_json::to_vec(&identity).unwrap()).into(),
    );
    let key = Key::new(format!(
        "repos/{repository}/adapters/v1/{}/snapshots/v1/{epoch}/{digest}/snapshot.json",
        identity.name
    ))
    .unwrap();
    let store = s3::S3Store::new(s3::Config {
        endpoint: format!("{}/", endpoint.trim_end_matches('/')),
        bucket: "jj-store-contract".into(),
        region: "us-east-1".into(),
        credentials: s3::Credentials {
            access_key: std::env::var("JST_TEST_ACCESS_KEY").unwrap(),
            secret_key: std::env::var("JST_TEST_SECRET_KEY").unwrap(),
            session_token: None,
        },
        allow_http_loopback: true,
        max_object_bytes: 512 * 1024 * 1024,
        timeout: Duration::from_secs(30),
    })
    .unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if let Ok(download) = store.get(&key, None).await {
            let pointer: serde_json::Value =
                serde_json::from_slice(&download.read_bounded(16 * 1024).await.unwrap()).unwrap();
            if pointer["checkpoint"]["artifact"] == expected_generation {
                return;
            }
        }
        assert!(
            Instant::now() < deadline,
            "remote search snapshot did not select generation {expected_generation}"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

fn event_cursor(event: &serde_json::Value) -> String {
    format!(
        "v1:{}:{}:{}",
        event["cursor"]["repository"].as_str().unwrap(),
        event["cursor"]["epoch"].as_str().unwrap(),
        event["cursor"]["position"].as_u64().unwrap()
    )
}
async fn next_server_event(
    response: &mut reqwest::Response,
    buffer: &mut String,
) -> Result<serde_json::Value, ()> {
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            if let Some(end) = buffer.find("\n\n") {
                let frame = buffer[..end].to_owned();
                buffer.drain(..end + 2);
                if let Some(data) = frame.lines().find_map(|line| line.strip_prefix("data: ")) {
                    return serde_json::from_str(data).map_err(|_| ());
                }
                continue;
            }
            let chunk = response.chunk().await.map_err(|_| ())?.ok_or(())?;
            buffer.push_str(std::str::from_utf8(&chunk).map_err(|_| ())?);
            if buffer.len() > 128 * 1024 {
                return Err(());
            }
        }
    })
    .await
    .map_err(|_| ())?
}

#[test]
fn cold_search_server_requires_injected_credentials_without_echoing_secrets() {
    let f = Fixture::new();
    let entry = format!(
        "\n[[search]]\nrepository = \"{}\"\ntoken_environment = \"JJ_STORE_SEARCH_TOKEN_FIXTURE\"\n",
        jj_store_model::RepositoryId::new([75; 16])
    );
    let config = fs::read_to_string(&f.config).unwrap() + &entry;
    fs::write(&f.config, config).unwrap();
    // A configuration check neither resolves credentials nor starts a writer.
    success(
        f.command()
            .env_remove("JJ_STORE_SEARCH_TOKEN_FIXTURE")
            .args(["config", "check", "--config", f.config.to_str().unwrap()])
            .output()
            .unwrap(),
    );
    for value in [None, Some("must-not-echo-search-secret")] {
        let mut command = f.command();
        command.env_remove("JJ_STORE_SEARCH_TOKEN_FIXTURE");
        if let Some(value) = value {
            command.env("JJ_STORE_SEARCH_TOKEN_FIXTURE", value);
        }
        let result = command
            .args(["run", "--config", f.config.to_str().unwrap()])
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(result.stdout.is_empty());
        let error = String::from_utf8_lossy(&result.stderr);
        assert!(error.contains("Hint:"));
        assert!(!error.contains("must-not-echo-search-secret"));
    }
}

#[cfg(unix)]
#[test]
fn cold_server_registers_shutdown_before_reporting_listener() {
    let fixture = Fixture::new();
    for local in [false, true] {
        for _ in 0..8 {
            let mut server = fixture.start(local);
            server.terminate();
        }
    }
}

#[cfg(unix)]
#[test]
fn cold_search_worker_polls_failed_storage_and_stops_with_foreground_server() {
    use std::io::{Read as _, Write as _};
    use std::sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    };
    struct FailureEndpoint {
        stop: Arc<AtomicBool>,
        worker: Option<std::thread::JoinHandle<()>>,
    }
    impl Drop for FailureEndpoint {
        fn drop(&mut self) {
            self.stop.store(true, Ordering::SeqCst);
            if let Some(worker) = self.worker.take() {
                worker.join().unwrap();
            }
        }
    }
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    listener.set_nonblocking(true).unwrap();
    let stop = Arc::new(AtomicBool::new(false));
    let count = Arc::new(AtomicUsize::new(0));
    let worker_stop = stop.clone();
    let worker_count = count.clone();
    let _endpoint = FailureEndpoint {
        stop,
        worker: Some(std::thread::spawn(move || {
            while !worker_stop.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((mut socket, _)) => {
                        socket
                            .set_read_timeout(Some(Duration::from_millis(100)))
                            .unwrap();
                        let mut buffer = [0; 16384];
                        if socket.read(&mut buffer).is_ok_and(|n| {
                            n > 0
                                && (buffer[..n].starts_with(b"GET ")
                                    || buffer[..n].starts_with(b"HEAD "))
                        }) {
                            worker_count.fetch_add(1, Ordering::SeqCst);
                        }
                        let _=socket.write_all(b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(Duration::from_millis(5))
                    }
                    Err(_) => break,
                }
            }
        })),
    };
    let mut f = Fixture::new();
    f.search = Some(BearerToken::generate().unwrap());
    let text = fs::read_to_string(&f.config)
        .unwrap()
        .replace("http://127.0.0.1:9000/", &format!("http://{address}/"))
        + &format!(
            "\n[[search]]\nrepository = \"{}\"\ntoken_environment = \"JJ_STORE_SEARCH_TOKEN_FIXTURE\"\npoll_millis = 10\ntimeout_seconds = 1\n",
            jj_store_model::RepositoryId::new([76; 16])
        );
    fs::write(&f.config, text).unwrap();
    for local in [false, true] {
        let before = count.load(Ordering::SeqCst);
        let mut server = f.start(local);
        let deadline = Instant::now() + Duration::from_secs(5);
        while count.load(Ordering::SeqCst) == before {
            assert!(
                server.child.try_wait().unwrap().is_none(),
                "index failure stopped server"
            );
            assert!(
                Instant::now() < deadline,
                "configured worker made no storage request"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
        server.terminate();
    }
}

#[path = "server_executable/git_bridge.rs"]
mod git_bridge_acceptance;
