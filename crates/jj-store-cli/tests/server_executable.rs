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
struct Fixture {
    root: tempfile::TempDir,
    config: PathBuf,
    operator: BearerToken,
    real: bool,
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
        }
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
#[test]
fn standalone_help_configuration_completions_and_build_info_are_consistent() {
    for args in [
        vec![],
        vec!["--help"],
        vec!["help"],
        vec!["help", "-k", "config"],
        vec!["config", "--help"],
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

fn native(f: &Fixture, token: &BearerToken, args: &[&str]) -> Output {
    native_command(f, token, args).output().unwrap()
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
    let mut running = f.start(false);
    let address = running.event["address"].as_str().unwrap().to_owned();
    let endpoint = format!("http://{address}");
    fs::write(
        &f.config,
        fs::read_to_string(&f.config)
            .unwrap()
            .replace("bind = \"127.0.0.1:0\"", &format!("bind = \"{address}\""))
            .replace("http://127.0.0.1:7443/", &format!("{endpoint}/")),
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
        "--insecure-loopback",
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
        insecure_loopback: true,
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
            "--insecure-loopback",
        ],
    ));
    success(native(
        &f,
        &admin,
        &["-R", "other", "new", "main", "-m", "independent workspace"],
    ));
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
                "--insecure-loopback",
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
    let revoked: serde_json::Value = serde_json::from_str(&success(
        f.command()
            .env("JJ_STORE_TOKEN", admin.expose().as_str())
            .args([
                "token",
                "--endpoint",
                &endpoint,
                "--repo-id",
                &repository.to_string(),
                "--insecure-loopback",
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
                "--insecure-loopback",
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
