#![cfg(unix)]
//! Fresh executable commands through owner-admitted native RPC, not RustFS proof.
use jj_lib::{config::StackedConfig, settings::UserSettings};
use jj_store_cli::initialization::{Initialization, Options};
use jj_store_jj::policy::Protection;
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_rpc::server::{Limits, NativeRpc};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    auth::{self, Permission, Scope, SystemClock, TokenPolicy},
    native::SessionLimits,
};
use serde_json::{Value, json};
use std::{
    os::unix::fs::MetadataExt as _,
    path::Path,
    process::{Command, Output},
    sync::Arc,
    time::Duration,
};
use tonic::transport::Server;
struct NoOperators;
#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(TokenPolicy::new(Scope::Operator))
    }
}
fn run(path: &Path, device: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jj-store-jj"))
        .args([
            "--no-pager",
            "agent",
            "--format",
            "json",
            "--state-dir",
            device.to_str().unwrap(),
        ])
        .args(args)
        .current_dir(path)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("PATH", "/usr/bin:/bin")
        .output()
        .unwrap()
}
fn output(path: &Path, device: &Path, args: &[&str]) -> Value {
    let out = run(path, device, args);
    assert!(
        out.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).unwrap()
}
fn id(n: u8) -> String {
    m::RequestId::new([n; 16]).to_string()
}
fn create(path: &Path, device: &Path, n: u8) -> Value {
    output(
        path,
        device,
        &[
            "thread",
            "create",
            "--request-id",
            &id(n),
            "--thread-id",
            &id(n),
            "--title",
            &format!("Thread {n}"),
        ],
    )
}
fn doc(path: &Path, value: &Value) {
    std::fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}
fn restarted(
    host: &RepositoryHost,
    root: &Path,
    timeout: Duration,
) -> (
    tokio::sync::oneshot::Sender<()>,
    tokio::task::JoinHandle<()>,
) {
    let socket = root.join("native.sock");
    std::fs::remove_file(&socket).unwrap();
    let listener = tokio::net::UnixListener::bind(&socket).unwrap();
    let rpc = NativeRpc::hosted_local_trusted(
        host.clone(),
        Limits {
            timeout,
            ..Limits::default()
        },
        m::PrincipalId::new([93; 16]),
        Permission::Admin,
        std::fs::metadata(root).unwrap().uid(),
    )
    .unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        Server::builder()
            .add_service(rpc.service())
            .serve_with_incoming_shutdown(
                tokio_stream::wrappers::UnixListenerStream::new(listener),
                async {
                    let _ = stopped.await;
                },
            )
            .await
            .unwrap();
    });
    (stop, server)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn cold_native_cli_replays_receipts_pins_pages_and_preserves_code_without_agent_setup() {
    let root = tempfile::tempdir().unwrap();
    let socket = root.path().join("native.sock");
    let device = root.path().join("agent-device");
    let host = RepositoryHost::new(
        Arc::new(MemoryStore::default()),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        8 * 1024 * 1024,
    )
    .unwrap();
    let rpc = NativeRpc::hosted_local_trusted(
        host.clone(),
        Limits::default(),
        m::PrincipalId::new([93; 16]),
        Permission::Admin,
        std::fs::metadata(root.path()).unwrap().uid(),
    )
    .unwrap();
    let listener = tokio::net::UnixListener::bind(&socket).unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        Server::builder()
            .add_service(rpc.service())
            .serve_with_incoming_shutdown(
                tokio_stream::wrappers::UnixListenerStream::new(listener),
                async {
                    let _ = stopped.await;
                },
            )
            .await
            .unwrap();
    });
    let path = root.path().join("workspace");
    let cache = root.path().join("cache");
    let mut initialization = Initialization::open(Options {
        name: Some("lapis/collab-cli".into()),
        endpoint: None,
        unix_socket: Some(socket),
        insecure_loopback: false,
        ca_certificate: None,
        destination: path.clone(),
        cache: Some(cache.clone()),
        workspace: Some("collab-client".into()),
        state: Some(root.path().join("client-state")),
    })
    .unwrap();
    initialization.create().await.unwrap();
    let mut attachment = initialization.attachment().await.unwrap();
    let settings =
        UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap();
    attachment.prepare(&settings).await.unwrap();
    attachment.finish(&settings).await.unwrap();
    let client = jj_store_cli::binding::Connection::read(&cache)
        .unwrap()
        .connect()
        .await
        .unwrap();
    let before = client.snapshot().await.unwrap();
    assert!(output(&path, &device, &["thread", "list"])["snapshot"]["thread_root"].is_null());
    assert!(
        output(&path, &device, &["reaction", "emojis"])["items"]["definitions"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        output(&path, &device, &["thread", "status", &id(1)])["status"],
        "not_known"
    );
    assert_eq!(client.snapshot().await.unwrap(), before);
    assert!(!device.exists());
    assert!(
        !run(
            &path,
            &device,
            &[
                "thread",
                "create",
                "--request-id",
                &id(1),
                "--thread-id",
                &id(1),
                "--title",
                "Gated code"
            ]
        )
        .status
        .success()
    );
    use jj_store_service::work;
    client
        .apply_work_command(&work::Request {
            version: 1,
            request_id: m::RequestId::new([99; 16]),
            command: work::WorkCommandV1::CreateProject {
                id: work::ProjectId::new([99; 16]),
                title: "Home".into(),
                body: String::new(),
            },
        })
        .await
        .unwrap();
    std::fs::write(path.join("unsnapshotted.txt"), b"user edit").unwrap();
    let heads = client.heads().await.unwrap();
    let first = create(&path, &device, 1);
    create(&path, &device, 2);
    assert_eq!(create(&path, &device, 1), first);
    assert_eq!(output(&path, &device, &["thread", "status", &id(1)]), first);
    assert_eq!(
        output(&path, &device, &["thread", "show", &id(1)])["thread"]["value"]["title"],
        "Thread 1"
    );
    let page = output(&path, &device, &["thread", "list", "--limit", "1"]);
    let cursor = page["next_cursor"].as_str().unwrap();
    create(&path, &device, 3);
    let second = output(
        &path,
        &device,
        &["thread", "list", "--limit", "1", "--cursor", cursor],
    );
    assert_eq!(second["snapshot"], page["snapshot"]);
    assert!(second["next_cursor"].is_null());
    assert!(
        !run(
            &path,
            &device,
            &["thread", "list", "--limit", "2", "--cursor", cursor]
        )
        .status
        .success()
    );
    assert!(
        !run(&path, &device, &["thread", "list", "--after", &id(1)])
            .status
            .success()
    );
    let target = root.path().join("target.json");
    let value = root.path().join("value.json");
    let observed = root.path().join("observed.json");
    doc(&target, &json!({"type":"thread","thread_id":id(1)}));
    doc(&value, &json!({"type":"emoji","sequence":"👩🏽‍💻"}));
    let reacted = output(
        &path,
        &device,
        &[
            "reaction",
            "set",
            "--request-id",
            &id(4),
            "--target",
            target.to_str().unwrap(),
            "--value",
            value.to_str().unwrap(),
        ],
    );
    let query = output(
        &path,
        &device,
        &["reaction", "query", "--target", target.to_str().unwrap()],
    );
    assert_eq!(query["items"]["reactions"][0]["count"], 1);
    assert_eq!(
        query["items"]["actors"][0]["actor"],
        reacted["receipt"]["actor"]
    );
    doc(&observed, &query["items"]["actors"][0]["active"]);
    output(
        &path,
        &device,
        &[
            "reaction",
            "clear",
            "--request-id",
            &id(5),
            "--target",
            target.to_str().unwrap(),
            "--observed",
            observed.to_str().unwrap(),
        ],
    );
    assert!(
        output(
            &path,
            &device,
            &["reaction", "query", "--target", target.to_str().unwrap()]
        )["items"]["actors"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        output(&path, &device, &["reaction", "status", &id(4)]),
        reacted
    );
    let asset = root.path().join("emoji.png");
    std::fs::write(&asset,hex::decode("89504e470d0a1a0a0000000d49484452000000010000000108060000001f15c4890000000d49444154789c63f8cfc0f01f00050001ff89993d1d0000000049454e44ae426082").unwrap()).unwrap();
    let defined = output(
        &path,
        &device,
        &[
            "reaction",
            "define-emoji",
            "--request-id",
            &id(6),
            "--emoji-id",
            &id(6),
            "--shortcode",
            "ship_it",
            "--label",
            "Ship it",
            "--asset",
            asset.to_str().unwrap(),
        ],
    );
    output(
        &path,
        &device,
        &[
            "reaction",
            "retire-emoji",
            "--request-id",
            &id(7),
            "--emoji-id",
            &id(6),
            "--expected",
            defined["receipt"]["result"]["version"]["id"]
                .as_str()
                .unwrap(),
        ],
    );
    assert_eq!(
        output(&path, &device, &["reaction", "emojis"])["items"]["definitions"][0]["retired"],
        true
    );
    assert_eq!(client.heads().await.unwrap(), heads);
    assert_eq!(
        std::fs::read(path.join("unsnapshotted.txt")).unwrap(),
        b"user edit"
    );
    assert!(!device.exists());
    let _ = stop.send(());
    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();

    // An outer RPC deadline can obscure an accepted command. The CLI retains
    // its identity and never retries; a new normal transport resolves it.
    let (stop, server) = restarted(&host, root.path(), Duration::from_nanos(1));
    let uncertain = create(&path, &device, 8);
    assert_eq!(uncertain["status"], "outcome_unknown");
    assert_eq!(uncertain["request_id"], id(8));
    let _ = stop.send(());
    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
    let (stop, server) = restarted(&host, root.path(), Duration::from_secs(30));
    let accepted = create(&path, &device, 8);
    assert_eq!(accepted["status"], "committed");
    assert_eq!(create(&path, &device, 8), accepted);
    assert_eq!(
        output(&path, &device, &["thread", "status", &id(8)]),
        accepted
    );
    assert!(!device.exists());
    drop(attachment);
    drop(initialization);
    let _ = stop.send(());
    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
}
