//! Typed administration over a real loopback RPC transport. Memory storage is
//! deliberately distinct from the later executable RustFS deployment gate.
use jj_store_jj::policy::Protection;
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_rpc::{
    client::{NativeClient, Options},
    server::{Limits, NativeRpc},
    wire,
};
use jj_store_service::{administration::*, auth::*, native::SessionLimits};
use std::{
    net::SocketAddr,
    process::Command,
    sync::{Arc, Mutex},
};
use tonic::{Code, Request, metadata::MetadataValue};
const REPO: m::RepositoryId = m::RepositoryId::new([91; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([92; 16]);
const EPOCH: m::EpochId = m::EpochId::new([93; 16]);
struct Policies(Mutex<TokenPolicy>);
#[async_trait::async_trait]
impl OperatorPolicies for Policies {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        Ok(self.0.lock().unwrap().clone())
    }
}
struct Fixture {
    store: Arc<MemoryStore>,
    policies: Arc<Policies>,
    operator: BearerToken,
    admin: BearerToken,
    address: SocketAddr,
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
    server: Option<tokio::task::JoinHandle<()>>,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        if let Some(server) = &self.server {
            server.abort();
        }
    }
}
impl Fixture {
    async fn new() -> Self {
        let mut policy = TokenPolicy::new(Scope::Operator);
        let operator = policy
            .issue(ACTOR, Permission::CreateRepository, None)
            .unwrap();
        let mut f = Self {
            store: Arc::default(),
            policies: Arc::new(Policies(Mutex::new(policy))),
            operator,
            admin: BearerToken::generate().unwrap(),
            address: "127.0.0.1:0".parse().unwrap(),
            shutdown: None,
            server: None,
        };
        f.start().await;
        f
    }
    async fn start(&mut self) {
        let host = RepositoryHost::new(
            self.store.clone(),
            self.policies.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
            32 * 1024 * 1024,
        )
        .unwrap();
        let rpc = NativeRpc::hosted(host, Limits::default()).unwrap();
        let listener = tokio::net::TcpListener::bind(self.address).await.unwrap();
        self.address = listener.local_addr().unwrap();
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.shutdown = Some(tx);
        self.server = Some(tokio::spawn(async move {
            rpc.serve(listener, None, true, async {
                let _ = rx.await;
            })
            .await
            .unwrap();
        }));
    }
    async fn stop(&mut self) {
        self.shutdown.take().unwrap().send(()).unwrap();
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            self.server.take().unwrap(),
        )
        .await
        .unwrap()
        .unwrap();
    }
    fn endpoint(&self) -> String {
        format!("http://{}", self.address)
    }
    async fn client(&self, token: &BearerToken) -> NativeClient {
        self.other_client(REPO, token).await
    }
    async fn other_client(&self, repository: m::RepositoryId, token: &BearerToken) -> NativeClient {
        NativeClient::connect(
            &self.endpoint(),
            repository,
            token.clone(),
            Options {
                insecure_loopback: true,
                ..Options::default()
            },
        )
        .await
        .unwrap()
    }
    async fn create(&self) -> AdministrationSnapshot {
        self.client(&self.operator)
            .await
            .create_repository(EPOCH, &self.admin)
            .await
            .unwrap()
    }
    fn grant(
        &self,
        n: u8,
        expected: AdministrationSnapshot,
        credential: &BearerToken,
        permission: Permission,
    ) -> TokenRequest {
        TokenRequest {
            request_id: m::RequestId::new([n; 16]),
            expected,
            change: TokenChange::Grant {
                credential: credential.clone(),
                principal: ACTOR,
                permission,
                expires_at: None,
            },
        }
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn rpc_created_repositories_attach_with_real_cli_and_survive_host_restart() {
    let mut f = Fixture::new().await;
    let initial = f.create().await;
    assert_eq!(initial, f.create().await);
    let root = tempfile::tempdir().unwrap();
    let config = root.path().join("jj.toml");
    std::fs::write(
        &config,
        "[user]\nname='RPC Test'\nemail='rpc@example.invalid'\n",
    )
    .unwrap();
    let run = |args: &[&str]| {
        let output = Command::new(env!("CARGO_BIN_EXE_jj"))
            .current_dir(root.path())
            .env("JJ_CONFIG", &config)
            .env("JJ_STORE_TOKEN", f.admin.expose().as_str())
            .env_remove("JJ_STORE_PAGER")
            .args(["--no-pager", "--color=never"])
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    };
    run(&[
        "store",
        "attach",
        "work",
        "--cache",
        "cache",
        "--endpoint",
        &f.endpoint(),
        "--repo-id",
        &REPO.to_string(),
        "--insecure-loopback",
    ]);
    run(&[
        "-R",
        "work",
        "describe",
        "-m",
        "created over authenticated RPC",
    ]);
    assert!(
        run(&["-R", "work", "log", "--no-graph", "-T", "description"])
            .contains("created over authenticated RPC")
    );
    let heads = f.client(&f.admin).await.heads().await.unwrap();
    f.stop().await;
    f.start().await;
    assert_eq!(f.client(&f.admin).await.heads().await.unwrap(), heads);
    assert_eq!(
        f.client(&f.admin)
            .await
            .tokens()
            .await
            .unwrap()
            .tokens
            .len(),
        1
    );
}
#[tokio::test]
async fn rpc_administration_separates_operator_admin_and_cross_repository_access() {
    let f = Fixture::new().await;
    assert_eq!(
        f.client(&f.admin)
            .await
            .create_repository(EPOCH, &f.admin)
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );
    let initial = f.create().await;
    let admin = f.client(&f.admin).await;
    assert_eq!(
        f.client(&f.operator)
            .await
            .tokens()
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );
    let reader = BearerToken::generate().unwrap();
    let grant = f.grant(1, initial, &reader, Permission::Read);
    assert!(admin.token_status(&grant).await.unwrap().is_none());
    let receipt = admin.change_token(&grant).await.unwrap().unwrap();
    assert!(f.client(&reader).await.authorize(false).await.is_ok());
    assert_eq!(
        f.client(&reader).await.tokens().await.unwrap_err().code(),
        Code::PermissionDenied
    );
    assert_eq!(
        f.client(&reader)
            .await
            .change_token(&grant)
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );
    let wrong = BearerToken::generate().unwrap();
    let unknown = m::RepositoryId::new([94; 16]);
    let denied = f.client(&wrong).await.authorize(false).await.unwrap_err();
    let missing = f
        .other_client(unknown, &f.admin)
        .await
        .authorize(false)
        .await
        .unwrap_err();
    assert_eq!(denied.code(), missing.code());
    assert_eq!(denied.message(), missing.message());
    let revoke = TokenRequest {
        request_id: m::RequestId::new([2; 16]),
        expected: receipt.accepted,
        change: TokenChange::Revoke { token: reader.id() },
    };
    admin.change_token(&revoke).await.unwrap().unwrap();
    assert_eq!(
        f.client(&reader)
            .await
            .authorize(false)
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );
    f.policies.0.lock().unwrap().revoke(f.operator.id());
    assert_eq!(
        f.client(&f.operator)
            .await
            .create_repository(EPOCH, &f.admin)
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );
}
#[tokio::test]
async fn rpc_token_retries_and_competing_snapshots_preserve_accepted_policy() {
    let mut f = Fixture::new().await;
    let initial = f.create().await;
    let a = BearerToken::generate().unwrap();
    let b = BearerToken::generate().unwrap();
    let first = f.grant(3, initial.clone(), &a, Permission::Read);
    let second = f.grant(4, initial, &b, Permission::Write);
    let c1 = f.client(&f.admin).await;
    let c2 = f.client(&f.admin).await;
    let (r1, r2) = tokio::join!(c1.change_token(&first), c2.change_token(&second));
    let (request, receipt, loser) = match (r1, r2) {
        (Ok(Some(receipt)), Err(error)) => (&first, receipt, error),
        (Err(error), Ok(Some(receipt))) => (&second, receipt, error),
        other => panic!("{other:?}"),
    };
    assert_eq!(loser.code(), Code::Aborted);
    let revoke = TokenRequest {
        request_id: m::RequestId::new([5; 16]),
        expected: receipt.accepted.clone(),
        change: TokenChange::Revoke { token: a.id() },
    };
    let later = c1.change_token(&revoke).await.unwrap().unwrap();
    drop(c1);
    drop(c2);
    f.stop().await;
    f.start().await;
    let restarted = f.client(&f.admin).await;
    assert_eq!(
        restarted.token_status(request).await.unwrap(),
        Some(receipt.clone())
    );
    assert_eq!(
        restarted.change_token(request).await.unwrap(),
        Some(receipt)
    );
    assert_eq!(restarted.tokens().await.unwrap().snapshot, later.accepted);
    let changed = TokenRequest {
        change: TokenChange::Revoke {
            token: m::TokenId::new([99; 16]),
        },
        ..request.clone()
    };
    assert_eq!(
        restarted.change_token(&changed).await.unwrap_err().code(),
        Code::Aborted
    );
}
fn authorized<T>(body: T, token: &BearerToken) -> Request<T> {
    let mut request = Request::new(body);
    let mut value: MetadataValue<_> = format!("Bearer {}", token.expose().as_str())
        .parse()
        .unwrap();
    value.set_sensitive(true);
    request.metadata_mut().insert("authorization", value);
    request
}
#[tokio::test]
async fn rpc_rejects_malformed_admin_requests_without_publication_and_redacts_secrets() {
    let f = Fixture::new().await;
    let initial = f.create().await;
    let mut raw = wire::native_store_client::NativeStoreClient::connect(f.endpoint())
        .await
        .unwrap();
    let token = BearerToken::generate().unwrap();
    let grant = wire::TokenGrant {
        credential: token.expose().to_string(),
        principal: ACTOR.as_bytes().to_vec(),
        permission: 3,
        expires_at: None,
    };
    assert!(!format!("{grant:?}").contains(&token.expose()[38..]));
    let body = wire::TokenRequest {
        repository: REPO.as_bytes().to_vec(),
        request_id: vec![6; 16],
        expected: Some(wire::Snapshot {
            epoch: EPOCH.as_bytes().to_vec(),
            position: 0,
            heads: initial
                .heads
                .iter()
                .map(|id| id.as_bytes().to_vec())
                .collect(),
        }),
        change: Some(wire::token_request::Change::Grant(grant)),
    };
    assert_eq!(
        raw.change_token(body.clone()).await.unwrap_err().code(),
        Code::Unauthenticated
    );
    for bad in 0..5 {
        let mut request = body.clone();
        match bad {
            0 => request.request_id.clear(),
            1 => request.expected.as_mut().unwrap().heads = vec![vec![0; 64]; 257],
            2 => request.expected.as_mut().unwrap().heads = vec![vec![0; 64]; 2],
            3 => {
                let Some(wire::token_request::Change::Grant(grant)) = &mut request.change else {
                    unreachable!()
                };
                grant.permission = 99;
            }
            _ => request.change = None,
        }
        assert_eq!(
            raw.change_token(authorized(request, &f.admin))
                .await
                .unwrap_err()
                .code(),
            Code::InvalidArgument
        );
    }
    let creation = wire::CreateRepositoryRequest {
        repository: REPO.as_bytes().to_vec(),
        epoch: EPOCH.as_bytes().to_vec(),
        admin_credential: token.expose().to_string(),
    };
    assert!(!format!("{creation:?}").contains(&token.expose()[38..]));
    assert_eq!(
        raw.create_repository(creation).await.unwrap_err().code(),
        Code::Unauthenticated
    );
    assert_eq!(
        f.client(&f.admin).await.tokens().await.unwrap().snapshot,
        initial
    );
}
