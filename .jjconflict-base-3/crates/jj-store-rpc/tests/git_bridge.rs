#![cfg(unix)]
use jj_store_jj::policy::Protection;
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_rpc::{
    client::{NativeClient, Options},
    server::{Limits, NativeRpc},
};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    auth::{self, Permission, Scope, SystemClock, TokenPolicy},
    native::SessionLimits,
    work::{ProjectId, Request, WorkCommandV1},
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

#[tokio::test]
async fn local_peer_reads_and_replays_typed_git_acceptance_without_code_changes() {
    let root = tempfile::tempdir().unwrap();
    let socket = root.path().join("native.sock");
    let owner_uid = std::fs::metadata(root.path()).unwrap().uid();
    let repository = m::RepositoryId::new([61; 16]);
    let epoch = m::EpochId::new([62; 16]);
    let host = RepositoryHost::new(
        Arc::new(MemoryStore::default()),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        1024 * 1024,
    )
    .unwrap();
    let rpc = NativeRpc::hosted_local_trusted(
        host,
        Limits::default(),
        m::PrincipalId::new([63; 16]),
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
    let client = NativeClient::connect_unix(
        &socket,
        repository,
        Options {
            timeout: Duration::from_secs(5),
            object_bytes: 1024 * 1024,
            ..Options::default()
        },
    )
    .await
    .unwrap();
    client.create_repository_local(epoch).await.unwrap();
    // WorkStore homes retain their existing origin ledger; bridge is local.
    client
        .apply_work_command(&Request {
            version: 1,
            request_id: m::RequestId::new([70; 16]),
            command: WorkCommandV1::CreateProject {
                id: ProjectId::new([70; 16]),
                title: "Home".into(),
                body: String::new(),
            },
        })
        .await
        .unwrap();
    let before = client.snapshot().await.unwrap();
    assert!(client.git_bridge_state().await.unwrap().git.is_none());
    let id = m::RequestId::new([71; 16]);
    let request = jj_store_service::git_bridge::Request {
        version: 1,
        request_id: id,
        transition: None,
        records: vec![jj_store_wal::publication::GitRecordV1::Remote(
            jj_store_wal::publication::GitRemoteV1 {
                remote: id,
                event: id,
                predecessors: Default::default(),
                name: "origin".into(),
                active: true,
            },
        )],
    };
    let first = client.accept_git_bridge(&request).await.unwrap();
    assert_eq!(client.accept_git_bridge(&request).await.unwrap(), first);
    let jj_store_service::git_bridge::Outcome::Committed { receipt } = first else {
        panic!()
    };
    assert_eq!(client.git_bridge_status(id).await.unwrap(), Some(*receipt));
    let state = client.git_bridge_state().await.unwrap();
    assert_eq!(
        state.git.unwrap().remotes().unwrap().get("origin"),
        Some(&id)
    );
    assert_eq!(client.snapshot().await.unwrap().heads, before.heads);
    let selected = client.snapshot().await.unwrap();
    let mut changed = request.clone();
    let jj_store_wal::publication::GitRecordV1::Remote(v) = &mut changed.records[0] else {
        panic!()
    };
    v.name = "different".into();
    assert!(client.accept_git_bridge(&changed).await.is_err());
    assert_eq!(client.snapshot().await.unwrap(), selected);
    let _ = stop.send(());
    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
}
