#![cfg(unix)]
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
use std::{os::unix::fs::MetadataExt as _, sync::Arc, time::Duration};
use tonic::transport::Server;

struct NoOperators;

#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(TokenPolicy::new(Scope::Operator))
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn tokenless_local_cli_initializes_and_attaches_a_physical_workspace() {
    let root = tempfile::tempdir().unwrap();
    let socket = root.path().join("native.sock");
    let owner_uid = std::fs::metadata(root.path()).unwrap().uid();
    let host = RepositoryHost::new(
        Arc::new(MemoryStore::default()),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap();
    let rpc = NativeRpc::hosted_local_trusted(
        host,
        Limits {
            timeout: Duration::from_secs(10),
            object_bytes: 4 * 1024 * 1024,
            ..Limits::default()
        },
        m::PrincipalId::new([93; 16]),
        Permission::Admin,
        owner_uid,
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

    let destination = root.path().join("workspace");
    let cache = root.path().join("cache");
    let state = root.path().join("client-state");
    let mut initialization = Initialization::open(Options {
        endpoint: None,
        unix_socket: Some(socket.clone()),
        insecure_loopback: false,
        ca_certificate: None,
        destination: destination.clone(),
        cache: Some(cache.clone()),
        workspace: Some("local-client".into()),
        state: Some(state),
    })
    .unwrap();
    initialization.create().await.unwrap();
    let mut attachment = initialization.attachment().await.unwrap();
    let settings = UserSettings::from_config(StackedConfig::with_defaults()).unwrap();
    attachment.prepare(&settings).await.unwrap();
    let attached = attachment.finish(&settings).await.unwrap();

    assert_eq!(attached.path, destination.canonicalize().unwrap());
    assert_eq!(attached.cache, cache.canonicalize().unwrap());
    assert_eq!(attached.workspace, "local-client");
    assert!(attached.path.join(".jj").is_dir());
    let connection = jj_store_cli::binding::Connection::read(&attached.cache).unwrap();
    assert_eq!(connection.format, 2);
    assert_eq!(
        connection.unix_socket.as_deref(),
        Some(socket.canonicalize().unwrap().as_path())
    );
    connection.connect().await.unwrap().heads().await.unwrap();

    drop(attachment);
    drop(initialization);
    let _ = stop.send(());
    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
}
