#![cfg(unix)]
use jj_review::{
    ArtifactAnchorV1, ArtifactTargetV1, CreatedContextV1, ReviewBodyFormatV1, ReviewCommandV1,
    ReviewCommentBodyV1, SensitivityV1,
};
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
    review_mutations::{Outcome, Request, Status},
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

fn create(repository: m::RepositoryId, request_id: m::RequestId) -> Request {
    Request {
        version: 1,
        request_id,
        command: ReviewCommandV1::CreateThread {
            anchor: Box::new(ArtifactAnchorV1 {
                schema_version: 1,
                repository_id: repository,
                target: ArtifactTargetV1::Repository,
                created_context: CreatedContextV1 {
                    publication_position: None,
                    code_state_version: None,
                    agent_root: None,
                },
                sensitivity: SensitivityV1::Repository,
            }),
            body: ReviewCommentBodyV1 {
                schema_version: 1,
                format: ReviewBodyFormatV1::SafeMarkdown,
                text: "Native RPC review".into(),
                sanitizer_version: "review-markdown-v1".into(),
            },
        },
    }
}

#[tokio::test]
async fn local_peer_reads_and_replays_receipt_backed_review_commands() {
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
            .review_threads(None, 50)
            .await
            .unwrap()
            .threads
            .is_empty()
    );

    let request_id = m::RequestId::new([64; 16]);
    let request = create(repository, request_id);
    let first = client.apply_review_command(&request).await.unwrap();
    let Outcome::Committed { receipt } = &first else {
        panic!("expected committed review")
    };
    let replay = client.apply_review_command(&request).await.unwrap();
    assert_eq!(replay, first);
    assert_eq!(
        client.review_command_status(request_id).await.unwrap(),
        Status::Committed {
            receipt: receipt.clone()
        }
    );

    let page = client.review_threads(None, 1).await.unwrap();
    assert_eq!(page.threads.len(), 1);
    assert!(page.next_cursor.is_none());
    let detail = client
        .review_thread(receipt.result.thread_id)
        .await
        .unwrap();
    assert_eq!(detail.thread.thread_id, receipt.result.thread_id);
    assert_eq!(detail.comments.len(), 1);
    assert_eq!(detail.events.len(), 1);
    assert_eq!(
        client
            .review_command_status(m::RequestId::new([99; 16]))
            .await
            .unwrap(),
        Status::NotKnown {
            request_id: m::RequestId::new([99; 16])
        }
    );

    let _ = stop.send(());
    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
}
