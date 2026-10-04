#![cfg(unix)]
use jj_store_jj::policy::Protection;
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_rpc::server::{Limits, NativeRpc};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    auth::{self, Permission, Scope, SystemClock, TokenPolicy},
    native::SessionLimits,
};
use std::{
    fs,
    os::unix::fs::MetadataExt as _,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::{Arc, Mutex},
    time::Duration,
};
use tonic::transport::Server;

const REPOSITORY: m::RepositoryId = m::RepositoryId::new([201; 16]);

struct Policies(Mutex<TokenPolicy>);

#[async_trait::async_trait]
impl OperatorPolicies for Policies {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(self.0.lock().unwrap().clone())
    }
}

fn command(cwd: &Path, config: &Path, token: &auth::BearerToken, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jj"))
        .current_dir(cwd)
        .env("JJ_CONFIG", config)
        .env("JJ_STORE_TOKEN", token.expose().as_str())
        .env_remove("JJ_STORE_PAGER")
        .args(["--no-pager", "--color=never"])
        .args(args)
        .output()
        .unwrap()
}

fn success(output: Output) -> String {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn tree(path: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    fn visit(root: &Path, path: &Path, entries: &mut Vec<(PathBuf, Vec<u8>)>) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                visit(root, &entry.path(), entries);
            } else {
                entries.push((
                    entry.path().strip_prefix(root).unwrap().to_owned(),
                    fs::read(entry.path()).unwrap(),
                ));
            }
        }
    }
    let mut entries = Vec::new();
    visit(path, path, &mut entries);
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn nonempty_workspace_migrates_without_snapshot_and_mutates_with_upstream_offline() {
    let root = tempfile::tempdir().unwrap();
    let config = root.path().join("jj.toml");
    fs::write(
        &config,
        "[user]\nname='Migration Test'\nemail='migration@example.invalid'\n[operation]\nhostname='migration-test'\nusername='test'\n",
    )
    .unwrap();

    let mut policy = TokenPolicy::new(Scope::Operator);
    let operator = policy
        .issue(
            m::PrincipalId::new([202; 16]),
            Permission::CreateRepository,
            None,
        )
        .unwrap();
    let remote_host = RepositoryHost::new(
        Arc::new(MemoryStore::default()),
        Arc::new(Policies(Mutex::new(policy))),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        32 * 1024 * 1024,
    )
    .unwrap();
    let token = auth::BearerToken::generate().unwrap();
    remote_host
        .create(&operator, REPOSITORY, m::EpochId::new([203; 16]), &token)
        .await
        .unwrap();
    let remote_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let remote_address = remote_listener.local_addr().unwrap();
    let remote_endpoint = format!("http://{remote_address}");
    let remote_rpc = NativeRpc::hosted(remote_host, Limits::default()).unwrap();
    let remote_server = tokio::spawn(async move {
        remote_rpc
            .serve(remote_listener, None, true, std::future::pending())
            .await
            .unwrap();
    });

    let workspace = root.path().join("work");
    let remote_cache = root.path().join("remote-cache");
    success(command(
        root.path(),
        &config,
        &token,
        &[
            "store",
            "attach",
            workspace.to_str().unwrap(),
            "--endpoint",
            &remote_endpoint,
            "--repo-id",
            &REPOSITORY.to_string(),
            "--insecure-loopback",
            "--workspace",
            "existing-workspace",
            "--cache",
            remote_cache.to_str().unwrap(),
            "--state-dir",
            root.path().join("remote-state").to_str().unwrap(),
        ],
    ));
    fs::write(
        workspace.join("unsnapshotted.txt"),
        b"must survive migration",
    )
    .unwrap();
    let working_copy_before = tree(&workspace.join(".jj/working_copy"));
    let descriptor_before = fs::read(workspace.join(".jj/repo")).unwrap();

    let socket = root.path().join("local.sock");
    let owner_uid = fs::metadata(root.path()).unwrap().uid();
    let local_host = RepositoryHost::new(
        Arc::new(MemoryStore::default()),
        Arc::new(Policies(Mutex::new(TokenPolicy::new(Scope::Operator)))),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        32 * 1024 * 1024,
    )
    .unwrap();
    let local_rpc = NativeRpc::hosted_local_trusted(
        local_host,
        Limits {
            timeout: Duration::from_secs(10),
            ..Limits::default()
        },
        m::PrincipalId::new([204; 16]),
        Permission::Admin,
        owner_uid,
    )
    .unwrap();
    let local_listener = tokio::net::UnixListener::bind(&socket).unwrap();
    let (stop_local, stopped_local) = tokio::sync::oneshot::channel();
    let local_server = tokio::spawn(async move {
        Server::builder()
            .add_service(local_rpc.service())
            .serve_with_incoming_shutdown(
                tokio_stream::wrappers::UnixListenerStream::new(local_listener),
                async {
                    let _ = stopped_local.await;
                },
            )
            .await
            .unwrap();
    });

    let local_cache = root.path().join("local-cache");
    let output = success(command(
        &workspace,
        &config,
        &token,
        &[
            "store",
            "migrate-local",
            "--unix-socket",
            socket.to_str().unwrap(),
            "--cache",
            local_cache.to_str().unwrap(),
            "--state-dir",
            root.path().join("migration-state").to_str().unwrap(),
            "--format=json",
        ],
    ));
    let result: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(result["workspace"], "existing-workspace");
    assert_eq!(
        tree(&workspace.join(".jj/working_copy")),
        working_copy_before
    );
    assert_eq!(
        fs::read(workspace.join(".jj/repo.jj-store-remote-v1")).unwrap(),
        descriptor_before
    );
    assert_eq!(
        fs::read(workspace.join("unsnapshotted.txt")).unwrap(),
        b"must survive migration"
    );
    let connection = jj_store_cli::binding::Connection::read(&local_cache).unwrap();
    assert_eq!(
        connection.unix_socket.as_deref(),
        Some(socket.canonicalize().unwrap().as_path())
    );

    remote_server.abort();
    let _ = remote_server.await;
    // A connection attempt to the former upstream completes its TCP handshake
    // but cannot provide a response. Count the queued accepts after the local
    // commands so a probe-and-fallback implementation fails this regression.
    let old_remote_probe = std::net::TcpListener::bind(remote_address).unwrap();
    old_remote_probe.set_nonblocking(true).unwrap();
    success(command(
        &workspace,
        &config,
        &token,
        &["describe", "-m", "offline local mutation"],
    ));
    for args in [
        &["status"][..],
        &["log"][..],
        &["op", "log"][..],
        &["evolog"][..],
        &["store", "status"][..],
        &["review", "list", "--format=json"][..],
        &["new", "-m", "offline second operation"][..],
    ] {
        success(command(&workspace, &config, &token, args));
    }
    let mut upstream_connections = 0;
    loop {
        match old_remote_probe.accept() {
            Ok((socket, _)) => {
                upstream_connections += 1;
                drop(socket);
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
            Err(error) => panic!("former upstream listener failed: {error}"),
        }
    }
    assert_eq!(upstream_connections, 0, "foreground contacted upstream");
    assert_eq!(
        fs::read(workspace.join("unsnapshotted.txt")).unwrap(),
        b"must survive migration"
    );
    connection.connect().await.unwrap().heads().await.unwrap();

    let _ = stop_local.send(());
    tokio::time::timeout(Duration::from_secs(5), local_server)
        .await
        .unwrap()
        .unwrap();
}
