use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::{memory::MemoryStore, *};
use jj_store_service::{administration::*, auth::*, native::*};
use jj_store_wal::{self as wal, publication::*};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
const REPO: m::RepositoryId = m::RepositoryId::new([80; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([81; 16]);
const EPOCH: m::EpochId = m::EpochId::new([82; 16]);
struct Policies(Mutex<TokenPolicy>);
#[async_trait::async_trait]
impl OperatorPolicies for Policies {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        Ok(self.0.lock().unwrap().clone())
    }
}
#[derive(Default)]
struct Store {
    inner: MemoryStore,
    reads: AtomicUsize,
    writes: Mutex<Vec<Vec<u8>>>,
    drop_manifest_reply: AtomicBool,
    hide_after_reply: AtomicBool,
    hide_manifest_read: AtomicBool,
    pause_manifest: AtomicBool,
    entered: tokio::sync::Notify,
    resume: tokio::sync::Notify,
}
#[async_trait::async_trait]
impl ObjectStore for Store {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        self.inner.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        if key.as_str().ends_with("/manifest.json")
            && self.hide_manifest_read.swap(false, Ordering::SeqCst)
        {
            return Err(jj_store_object::Error::Transport);
        }
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        use tokio::io::AsyncReadExt as _;
        let mut bytes = Vec::new();
        upload.reader()?.read_to_end(&mut bytes).await?;
        self.writes.lock().unwrap().push(bytes);
        if key.as_str().ends_with("/manifest.json")
            && self.pause_manifest.swap(false, Ordering::SeqCst)
        {
            self.entered.notify_one();
            self.resume.notified().await;
        }
        let outcome = self.inner.put(key, condition, upload).await?;
        if key.as_str().ends_with("/manifest.json")
            && self.drop_manifest_reply.swap(false, Ordering::SeqCst)
        {
            if self.hide_after_reply.swap(false, Ordering::SeqCst) {
                self.hide_manifest_read.store(true, Ordering::SeqCst);
            }
            Ok(WriteOutcome::Unknown)
        } else {
            Ok(outcome)
        }
    }
}
struct Fixture {
    store: Arc<Store>,
    policies: Arc<Policies>,
    operator: BearerToken,
    admin: BearerToken,
    host: RepositoryHost,
}
impl Fixture {
    fn new() -> Self {
        let store = Arc::new(Store::default());
        let mut policies = TokenPolicy::new(Scope::Operator);
        let operator = policies
            .issue(ACTOR, Permission::CreateRepository, None)
            .unwrap();
        let policies = Arc::new(Policies(Mutex::new(policies)));
        let host = RepositoryHost::new(
            store.clone(),
            policies.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
            1024 * 1024,
        )
        .unwrap();
        Self {
            store,
            policies,
            operator,
            admin: BearerToken::generate().unwrap(),
            host,
        }
    }
    async fn create(&self) -> Snapshot {
        self.host
            .create(&self.operator, REPO, EPOCH, &self.admin)
            .await
            .unwrap()
    }
    fn session(&self, token: &BearerToken) -> NativeSession {
        NativeSession::new(
            self.store.clone(),
            Arc::new(NativeObjects::new(self.store.clone(), REPO, 1024 * 1024).unwrap()),
            token.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap()
    }
    fn grant(
        &self,
        request: u8,
        expected: Snapshot,
        credential: &BearerToken,
        permission: Permission,
    ) -> TokenRequest {
        TokenRequest {
            request_id: m::RequestId::new([request; 16]),
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
fn committed(outcome: PublishOutcome) -> Receipt {
    match outcome {
        PublishOutcome::Committed(receipt) => receipt,
        other => panic!("{other:?}"),
    }
}
#[tokio::test]
async fn creation_requires_fresh_operator_permission_before_storage_and_is_retryable() {
    let f = Fixture::new();
    assert!(matches!(
        f.host.create(&f.admin, REPO, EPOCH, &f.admin).await,
        Err(wal::Error::Denied)
    ));
    assert_eq!(f.store.reads.load(Ordering::SeqCst), 0);
    assert!(f.store.writes.lock().unwrap().is_empty());
    f.store.drop_manifest_reply.store(true, Ordering::SeqCst);
    let created = f.create().await;
    assert_eq!(created, f.create().await);
    assert!(
        f.host
            .create(&f.operator, REPO, m::EpochId::new([0; 16]), &f.admin)
            .await
            .is_err()
    );
    assert!(
        f.host
            .create(&f.operator, REPO, EPOCH, &BearerToken::generate().unwrap())
            .await
            .is_err()
    );
    let token = BearerToken::generate().unwrap();
    let receipt = committed(
        f.session(&f.admin)
            .change_token(&f.grant(1, created, &token, Permission::Read))
            .await
            .unwrap(),
    );
    assert_eq!(f.create().await, receipt.accepted);
    f.policies.0.lock().unwrap().revoke(f.operator.id());
    let reads = f.store.reads.load(Ordering::SeqCst);
    assert!(matches!(
        f.host.create(&f.operator, REPO, EPOCH, &f.admin).await,
        Err(wal::Error::Denied)
    ));
    assert_eq!(f.store.reads.load(Ordering::SeqCst), reads);
}
#[tokio::test]
async fn token_grants_lists_and_revoke_are_admin_scoped_and_never_persist_secrets() {
    let f = Fixture::new();
    let initial = f.create().await;
    let reader = BearerToken::generate().unwrap();
    let admin = f.session(&f.admin);
    let receipt = committed(
        admin
            .change_token(&f.grant(2, initial, &reader, Permission::Read))
            .await
            .unwrap(),
    );
    let listing = admin.tokens().await.unwrap();
    assert_eq!(listing.snapshot, receipt.accepted);
    assert_eq!(listing.tokens.len(), 2);
    assert!(f.session(&reader).tokens().await.is_err());
    let revoke = TokenRequest {
        request_id: m::RequestId::new([3; 16]),
        expected: receipt.accepted.clone(),
        change: TokenChange::Revoke { token: reader.id() },
    };
    assert!(f.session(&reader).change_token(&revoke).await.is_err());
    assert!(f.session(&f.operator).tokens().await.is_err());
    committed(admin.change_token(&revoke).await.unwrap());
    assert!(
        f.session(&reader)
            .principal(Permission::Read)
            .await
            .is_err()
    );
    for bytes in f.store.writes.lock().unwrap().iter() {
        let text = String::from_utf8_lossy(bytes);
        for token in [&f.operator, &f.admin, &reader] {
            assert!(!text.contains(&token.expose()[38..]));
        }
    }
    let text = serde_json::to_string(&listing).unwrap();
    assert!(!text.contains("verifier"));
    assert!(!text.contains("jst1_"));
}
#[tokio::test]
async fn token_retries_recover_original_receipts_after_later_changes_and_restart() {
    let f = Fixture::new();
    let initial = f.create().await;
    let admin = f.session(&f.admin);
    let token = BearerToken::generate().unwrap();
    let request = f.grant(4, initial.clone(), &token, Permission::Write);
    assert!(admin.token_status(&request).await.unwrap().is_none());
    f.store.drop_manifest_reply.store(true, Ordering::SeqCst);
    let receipt = committed(admin.change_token(&request).await.unwrap());
    let revoke = TokenRequest {
        request_id: m::RequestId::new([5; 16]),
        expected: receipt.accepted.clone(),
        change: TokenChange::Revoke { token: token.id() },
    };
    let later = committed(admin.change_token(&revoke).await.unwrap());
    struct NoPublication;
    #[async_trait::async_trait]
    impl PublicationGuard for NoPublication {
        async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
            Err(wal::Error::Denied)
        }
        async fn validate_transition(
            &self,
            _: &ManifestSnapshot,
            _: &Request,
            _: &Snapshot,
        ) -> wal::Result<()> {
            Err(wal::Error::Denied)
        }
    }
    let maintenance = Coordinator::new(
        f.store.clone(),
        REPO,
        Arc::new(NativeObjects::new(f.store.clone(), REPO, 1024 * 1024).unwrap()),
        Arc::new(NoPublication),
        Limits::default(),
    )
    .unwrap();
    maintenance.checkpoint().await.unwrap();
    drop(admin);
    let restarted = f.session(&f.admin);
    assert_eq!(
        restarted.token_status(&request).await.unwrap(),
        Some(receipt.clone())
    );
    assert_eq!(
        committed(restarted.change_token(&request).await.unwrap()),
        receipt
    );
    assert_eq!(restarted.tokens().await.unwrap().snapshot, later.accepted);
    assert!(f.session(&token).principal(Permission::Read).await.is_err());
    let changed = f.grant(4, initial, &token, Permission::Admin);
    assert!(matches!(
        restarted.change_token(&changed).await,
        Err(wal::Error::IntentConflict)
    ));
    let forged = TokenRequest {
        expected: Snapshot {
            heads: [m::OperationId::new([1; 64])].into(),
            ..request.expected.clone()
        },
        ..request.clone()
    };
    assert!(matches!(
        restarted.change_token(&forged).await,
        Err(wal::Error::Stale)
    ));
}
#[tokio::test]
async fn stale_concurrent_token_mutations_cannot_restore_revoked_grants() {
    let f = Fixture::new();
    let initial = f.create().await;
    let first = BearerToken::generate().unwrap();
    let second = BearerToken::generate().unwrap();
    let base = committed(
        f.session(&f.admin)
            .change_token(&f.grant(10, initial, &first, Permission::Read))
            .await
            .unwrap(),
    )
    .accepted;
    let delayed_request = f.grant(6, base.clone(), &second, Permission::Write);
    let session = f.session(&f.admin);
    f.store.pause_manifest.store(true, Ordering::SeqCst);
    let delayed = tokio::spawn(async move { session.change_token(&delayed_request).await });
    f.store.entered.notified().await;
    let revoke = TokenRequest {
        request_id: m::RequestId::new([7; 16]),
        expected: base,
        change: TokenChange::Revoke { token: first.id() },
    };
    let accepted = committed(f.session(&f.admin).change_token(&revoke).await.unwrap());
    f.store.resume.notify_one();
    assert!(matches!(delayed.await.unwrap(), Err(wal::Error::Stale)));
    let listing = f.session(&f.admin).tokens().await.unwrap();
    assert_eq!(listing.snapshot, accepted.accepted);
    assert!(!listing.tokens.iter().any(|t| t.id == second.id()));
    assert!(!listing.tokens.iter().any(|t| t.id == first.id()));
}
#[tokio::test]
async fn token_action_identity_and_current_authorization_apply_to_receipt_queries() {
    let f = Fixture::new();
    let initial = f.create().await;
    let admin = f.session(&f.admin);
    let request = TokenRequest {
        request_id: m::RequestId::new([8; 16]),
        expected: initial,
        change: TokenChange::Revoke {
            token: m::TokenId::new([2; 16]),
        },
    };
    let receipt = committed(admin.change_token(&request).await.unwrap());
    let altered = TokenRequest {
        change: TokenChange::Revoke {
            token: m::TokenId::new([3; 16]),
        },
        ..request.clone()
    };
    assert!(matches!(
        admin.change_token(&altered).await,
        Err(wal::Error::IntentConflict)
    ));
    let revoke_self = TokenRequest {
        request_id: m::RequestId::new([9; 16]),
        expected: receipt.accepted,
        change: TokenChange::Revoke {
            token: f.admin.id(),
        },
    };
    committed(admin.change_token(&revoke_self).await.unwrap());
    assert!(matches!(
        admin.token_status(&request).await,
        Err(wal::Error::Denied)
    ));
    assert!(matches!(
        admin.change_token(&request).await,
        Err(wal::Error::Denied)
    ));
    assert!(matches!(admin.tokens().await, Err(wal::Error::Denied)));
}
#[test]
fn client_generated_grants_are_exact_idempotent_and_reject_token_id_collisions() {
    let mut policy = TokenPolicy::new(Scope::Repository(REPO));
    let token = BearerToken::generate().unwrap();
    policy.grant(&token, ACTOR, Permission::Read, None).unwrap();
    policy.grant(&token, ACTOR, Permission::Read, None).unwrap();
    assert!(
        policy
            .grant(&token, ACTOR, Permission::Admin, None)
            .is_err()
    );
    let text = token.expose();
    let mut changed = text.to_string();
    changed.replace_range(101..102, if text.ends_with('0') { "1" } else { "0" });
    assert!(
        policy
            .grant(
                &BearerToken::parse(&changed).unwrap(),
                ACTOR,
                Permission::Read,
                None
            )
            .is_err()
    );
    assert_eq!(policy.tokens().len(), 1);
}

#[tokio::test]
async fn unknown_token_publication_resolves_by_original_request_without_issuing_again() {
    let f = Fixture::new();
    let initial = f.create().await;
    let credential = BearerToken::generate().unwrap();
    let request = f.grant(11, initial, &credential, Permission::Read);
    f.store.drop_manifest_reply.store(true, Ordering::SeqCst);
    f.store.hide_after_reply.store(true, Ordering::SeqCst);
    assert!(
        matches!(f.session(&f.admin).change_token(&request).await.unwrap(), PublishOutcome::OutcomeUnknown { request_id, .. } if request_id == request.request_id)
    );
    let session = f.session(&f.admin);
    let recovered = session.token_status(&request).await.unwrap().unwrap();
    assert_eq!(recovered.accepted.position, m::WalPosition(1));
    assert_eq!(
        committed(session.change_token(&request).await.unwrap()),
        recovered
    );
    assert_eq!(session.tokens().await.unwrap().tokens.len(), 2);
    assert!(
        f.session(&credential)
            .principal(Permission::Read)
            .await
            .is_ok()
    );
}
