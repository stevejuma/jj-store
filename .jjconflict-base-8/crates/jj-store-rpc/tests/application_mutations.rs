use jj_store_jj::{
    commands::Executor, objects::NativeObjects, operation_stores::OperationAuthority,
    policy::Protection, roots,
};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_rpc::{
    client::{NativeClient, Options},
    server::{Limits as RpcLimits, NativeRpc},
};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    auth::{self, BearerToken, Permission, Scope, SystemClock, TokenPolicy},
    commands::Command,
    mutations::{Outcome, Request as MutationRequest, Status as MutationStatus},
    native::{NativeSession, SessionLimits, create_repository},
    transport::NativeEndpoint,
};
use jj_store_wal::publication::Limits as PublicationLimits;
use std::{collections::BTreeSet, sync::Arc, time::Duration};

const REPOSITORY: m::RepositoryId = m::RepositoryId::new([81; 16]);
const PRINCIPAL: m::PrincipalId = m::PrincipalId::new([82; 16]);

struct NoOperators;

#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(TokenPolicy::new(Scope::Operator))
    }
}

struct Seed {
    store: Arc<MemoryStore>,
    token: BearerToken,
    expected: jj_store_wal::publication::Snapshot,
}

async fn seed() -> Seed {
    let store = Arc::new(MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), REPOSITORY, 4 * 1024 * 1024).unwrap());
    let mut policy = TokenPolicy::new(Scope::Repository(REPOSITORY));
    let token = policy.issue(PRINCIPAL, Permission::Admin, None).unwrap();
    let mut operators = TokenPolicy::new(Scope::Operator);
    let operator = operators
        .issue(PRINCIPAL, Permission::CreateRepository, None)
        .unwrap();
    create_repository(
        store.clone(),
        objects.clone(),
        &operators,
        &operator,
        &policy,
        m::EpochId::new([83; 16]),
        &SystemClock,
        PublicationLimits::default(),
    )
    .await
    .unwrap();
    let session = Arc::new(
        NativeSession::new(
            store.clone(),
            objects.clone(),
            token.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    let root = session.snapshot().await.unwrap().manifest().snapshot();
    let executor = Executor::new(Default::default(), 2).unwrap();
    let left = session
        .prepare_command(
            &executor,
            &root,
            Command::New {
                parents: vec![roots::ROOT_COMMIT],
                description: "left branch".into(),
                workspace: None,
            },
        )
        .await
        .unwrap();
    let right = session
        .prepare_command(
            &executor,
            &root,
            Command::New {
                parents: vec![roots::ROOT_COMMIT],
                description: "right branch".into(),
                workspace: None,
            },
        )
        .await
        .unwrap();
    let old = BTreeSet::from([roots::ROOT_OPERATION]);
    session.publish(old.clone(), left.operation).await.unwrap();
    session.publish(old, right.operation).await.unwrap();
    let expected = session.snapshot().await.unwrap().manifest().snapshot();
    assert_eq!(expected.heads.len(), 2);
    Seed {
        store,
        token,
        expected,
    }
}

async fn exercise(client: NativeClient, seed: &Seed, request_byte: u8) {
    assert_eq!(client.snapshot().await.unwrap(), seed.expected);
    assert_eq!(client.heads().await.unwrap(), seed.expected.heads);
    assert_eq!(client.snapshot().await.unwrap(), seed.expected);

    let request_id = m::RequestId::new([request_byte; 16]);
    let request = MutationRequest {
        version: 1,
        request_id,
        expected: seed.expected.clone(),
        command: Command::ReconcileOperations {
            base: *seed.expected.heads.first().unwrap(),
        },
    };
    let outcome = client.apply_mutation(&request).await.unwrap();
    let Outcome::Committed { receipt } = &outcome else {
        panic!("expected committed reconciliation")
    };
    assert_eq!(receipt.accepted.position.0, seed.expected.position.0 + 1);
    assert_eq!(receipt.accepted.heads.len(), 1);
    assert!(!seed.expected.heads.contains(&receipt.result.operation));
    assert_eq!(client.apply_mutation(&request).await.unwrap(), outcome);
    assert_eq!(
        client.mutation_status(request_id).await.unwrap(),
        MutationStatus::Committed {
            receipt: receipt.clone()
        }
    );
    assert_eq!(client.snapshot().await.unwrap(), receipt.accepted);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn authenticated_rpc_reconciles_only_through_explicit_receipt_backed_mutation() {
    let seed = seed().await;
    let endpoint = NativeEndpoint::new(
        REPOSITORY,
        seed.store.clone(),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        NativeRpc::new(vec![endpoint], RpcLimits::default())
            .unwrap()
            .serve(listener, None, true, async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let client = NativeClient::connect(
        &format!("http://{address}"),
        REPOSITORY,
        seed.token.clone(),
        Options {
            insecure_loopback: true,
            timeout: Duration::from_secs(5),
            object_bytes: 4 * 1024 * 1024,
            ..Options::default()
        },
    )
    .await
    .unwrap();
    exercise(client, &seed, 84).await;
    let _ = stop.send(());
    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn local_trusted_rpc_uses_the_same_explicit_reconciliation_contract() {
    use std::os::unix::fs::MetadataExt as _;
    use tonic::transport::Server;

    let seed = seed().await;
    let root = tempfile::tempdir().unwrap();
    let socket = root.path().join("native.sock");
    let owner_uid = std::fs::metadata(root.path()).unwrap().uid();
    let host = RepositoryHost::new(
        seed.store.clone(),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap();
    let rpc = NativeRpc::hosted_local_trusted(
        host,
        RpcLimits::default(),
        PRINCIPAL,
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
        REPOSITORY,
        Options {
            timeout: Duration::from_secs(5),
            object_bytes: 4 * 1024 * 1024,
            ..Options::default()
        },
    )
    .await
    .unwrap();
    exercise(client, &seed, 85).await;
    let _ = stop.send(());
    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
}
