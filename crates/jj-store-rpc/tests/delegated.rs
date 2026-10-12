#![cfg(unix)]
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_rpc::{
    client::{NativeClient, Options},
    server::{Limits, NativeRpc},
    wire::{self, native_store_client::NativeStoreClient},
};
use jj_store_service::{
    access::Delegation,
    administration::{OperatorPolicies, RepositoryHost},
    auth::{self, BearerToken, Permission, Scope, SystemClock, TokenPolicy},
};
use std::{
    os::unix::fs::MetadataExt,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tonic::{Code, Request};
struct Operators(TokenPolicy);
#[async_trait::async_trait]
impl OperatorPolicies for Operators {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(self.0.clone())
    }
}
struct Lease {
    repository: m::RepositoryId,
    principal: m::PrincipalId,
    token: BearerToken,
    allowed: AtomicBool,
}
#[async_trait::async_trait]
impl Delegation for Lease {
    fn credential(&self, repository: m::RepositoryId) -> auth::Result<BearerToken> {
        if repository != self.repository || !self.allowed.load(Ordering::Acquire) {
            return Err(auth::Error::Denied);
        }
        Ok(self.token.clone())
    }
    async fn principal(
        &self,
        repository: m::RepositoryId,
        required: Permission,
    ) -> auth::Result<m::PrincipalId> {
        self.credential(repository)?;
        if !matches!(required, Permission::Read | Permission::Write) {
            return Err(auth::Error::Denied);
        }
        Ok(self.principal)
    }
}
#[tokio::test]
async fn private_delegated_rpc_checks_uid_scope_no_bearer_admin_and_revocation() {
    let root = tempfile::tempdir().unwrap();
    let repository = m::RepositoryId::new([71; 16]);
    let principal = m::PrincipalId::new([72; 16]);
    let mut operators = TokenPolicy::new(Scope::Operator);
    let token = operators
        .issue(principal, Permission::CreateRepository, None)
        .unwrap();
    let host = RepositoryHost::new(
        Arc::new(MemoryStore::default()),
        Arc::new(Operators(operators)),
        Arc::new(SystemClock),
        Default::default(),
        Default::default(),
        1024 * 1024,
    )
    .unwrap();
    host.create(&token, repository, m::EpochId::new([73; 16]), &token)
        .await
        .unwrap();
    let lease = Arc::new(Lease {
        repository,
        principal,
        token,
        allowed: AtomicBool::new(true),
    });
    let uid = std::fs::metadata(root.path()).unwrap().uid();
    for (name, peer_uid) in [("good", uid), ("wrong", uid.wrapping_add(1))] {
        let socket = root.path().join(name);
        let listener = tokio::net::UnixListener::bind(&socket).unwrap();
        let rpc =
            NativeRpc::delegated(host.clone(), Limits::default(), lease.clone(), peer_uid).unwrap();
        let (stop, stopped) = tokio::sync::oneshot::channel();
        let server = tokio::spawn(async move {
            tonic::transport::Server::builder()
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
        let client = NativeClient::connect_unix(&socket, repository, Options::default())
            .await
            .unwrap();
        if peer_uid != uid {
            assert_eq!(
                client.authorize(false).await.unwrap_err().code(),
                Code::PermissionDenied
            );
        } else {
            client.authorize(true).await.unwrap();
            assert!(!client.heads().await.unwrap().is_empty());
            assert_eq!(
                client.tokens().await.unwrap_err().code(),
                Code::PermissionDenied
            );
            assert_eq!(
                client
                    .create_repository_local(m::EpochId::new([74; 16]))
                    .await
                    .unwrap_err()
                    .code(),
                Code::PermissionDenied
            );
            let other = NativeClient::connect_unix(
                &socket,
                m::RepositoryId::new([75; 16]),
                Options::default(),
            )
            .await
            .unwrap();
            assert_eq!(
                other.authorize(false).await.unwrap_err().code(),
                Code::PermissionDenied
            );
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
                .insert("authorization", "Bearer process-supplied".parse().unwrap());
            assert_eq!(
                raw.authorize(request).await.unwrap_err().code(),
                Code::PermissionDenied
            );
            lease.allowed.store(false, Ordering::Release);
            assert_eq!(
                client.heads().await.unwrap_err().code(),
                Code::PermissionDenied
            );
            lease.allowed.store(true, Ordering::Release);
        }
        let _ = stop.send(());
        tokio::time::timeout(std::time::Duration::from_secs(5), server)
            .await
            .unwrap()
            .unwrap();
    }
}
