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
    work::{Outcome, ProjectId, Request, Status, WorkCommandV1, WorkQuery},
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

fn create(request_id: m::RequestId) -> Request {
    Request {
        version: 1,
        request_id,
        command: WorkCommandV1::CreateProject {
            id: ProjectId::new(*request_id.as_bytes()),
            title: "Native project".into(),
            body: "".into(),
        },
    }
}
#[tokio::test]
async fn local_peer_reads_and_replays_receipt_backed_work_commands() {
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
    assert!(
        client
            .work_page(&WorkQuery::default())
            .await
            .unwrap()
            .projects
            .is_empty()
    );
    let request_id = m::RequestId::new([64; 16]);
    let request = create(request_id);
    let first = client.apply_work_command(&request).await.unwrap();
    let Outcome::Committed { receipt } = &first else {
        panic!("commit required")
    };
    assert_eq!(client.apply_work_command(&request).await.unwrap(), first);
    assert_eq!(
        client.work_command_status(request_id).await.unwrap(),
        Status::Committed {
            receipt: receipt.clone()
        }
    );
    let second = create(m::RequestId::new([65; 16]));
    client.apply_work_command(&second).await.unwrap();
    let first_page = client
        .work_page(&WorkQuery {
            limit: 1,
            ..Default::default()
        })
        .await
        .unwrap();
    client
        .apply_work_command(&create(m::RequestId::new([66; 16])))
        .await
        .unwrap();
    let next = client
        .work_page(&WorkQuery {
            limit: 1,
            cursor: first_page.next_cursor,
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(next.snapshot, first_page.snapshot);
    assert!(next.next_cursor.is_none());
    let _ = stop.send(());
    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
}
