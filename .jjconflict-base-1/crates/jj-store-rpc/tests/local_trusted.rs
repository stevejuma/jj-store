#![cfg(unix)]
use jj_store_jj::policy::Protection;
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_rpc::{
    client::{NativeClient, Options},
    server::{Limits, NativeRpc},
    wire::{self, native_store_client::NativeStoreClient},
};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    auth::{self, Permission, Scope, SystemClock, TokenPolicy},
    native::SessionLimits,
};
use std::{os::unix::fs::MetadataExt as _, sync::Arc, time::Duration};
use tonic::{Code, Request, transport::Server};

struct NoOperators;
#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(TokenPolicy::new(Scope::Operator))
    }
}

#[tokio::test]
async fn local_unix_rpc_requires_the_owner_peer_and_no_bearer() {
    let root = tempfile::tempdir().unwrap();
    let socket = root.path().join("native.sock");
    let owner_uid = std::fs::metadata(root.path()).unwrap().uid();
    let repository = m::RepositoryId::new([81; 16]);
    let epoch = m::EpochId::new([82; 16]);
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
        m::PrincipalId::new([83; 16]),
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

    let options = Options {
        timeout: Duration::from_secs(5),
        object_bytes: 1024 * 1024,
        ..Options::default()
    };
    let client = NativeClient::connect_unix(&socket, repository, options.clone())
        .await
        .unwrap();
    let created = client.create_repository_local(epoch).await.unwrap();
    assert_eq!(created.epoch, epoch);
    client.authorize(true).await.unwrap();
    assert!(!client.heads().await.unwrap().is_empty());

    let channel = tonic::transport::Endpoint::from_static("http://localhost")
        .connect_with_connector(tower::service_fn({
            let socket = socket.clone();
            move |_| {
                let socket = socket.clone();
                async move {
                    tokio::net::UnixStream::connect(socket)
                        .await
                        .map(hyper_util::rt::TokioIo::new)
                }
            }
        }))
        .await
        .unwrap();
    let mut raw = NativeStoreClient::new(channel);
    let mut request = Request::new(wire::AccessRequest {
        repository: repository.as_bytes().to_vec(),
        write: false,
    });
    request
        .metadata_mut()
        .insert("authorization", "Bearer invalid".parse().unwrap());
    assert_eq!(
        raw.authorize(request).await.unwrap_err().code(),
        Code::Unauthenticated
    );

    let _ = stop.send(());
    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
}
