use jj_store_model::*;
use jj_store_object::{memory::MemoryStore, *};
use jj_store_service::{auth::*, guard::*};
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication::*,
};
use std::{
    collections::BTreeSet,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    },
};

const REPO: RepositoryId = RepositoryId::new([1; 16]);
const ACTOR: PrincipalId = PrincipalId::new([2; 16]);
const ADMIN: PrincipalId = PrincipalId::new([3; 16]);

#[test]
fn tokens_are_secret_scope_bound_revocable_and_expiring() {
    let mut policy = TokenPolicy::new(Scope::Repository(REPO));
    let token = policy.issue(ACTOR, Permission::Write, Some(100)).unwrap();
    let exposed = token.expose();
    let parsed = BearerToken::parse(&exposed).unwrap();
    assert_eq!(
        policy
            .authorize(&parsed, Permission::Read, 99)
            .unwrap()
            .id(),
        ACTOR
    );
    assert!(policy.authorize(&parsed, Permission::Admin, 99).is_err());
    assert!(policy.authorize(&parsed, Permission::Write, 100).is_err());
    assert!(
        policy
            .authorize(&parsed, Permission::CreateRepository, 99)
            .is_err()
    );
    assert!(!format!("{token:?}").contains(&exposed[38..]));
    let encoded = policy.encode().unwrap();
    assert!(!String::from_utf8_lossy(&encoded).contains(&exposed[38..]));
    assert!(TokenPolicy::decode(&encoded, Scope::Repository(RepositoryId::new([9; 16]))).is_err());
    // Altering the policy's scope alone cannot make a verifier portable.
    let changed = String::from_utf8(encoded.clone())
        .unwrap()
        .replace(&REPO.to_string(), &RepositoryId::new([9; 16]).to_string());
    let forged = TokenPolicy::decode(
        changed.as_bytes(),
        Scope::Repository(RepositoryId::new([9; 16])),
    )
    .unwrap();
    assert!(forged.authorize(&parsed, Permission::Read, 0).is_err());
    let mut reloaded = TokenPolicy::decode(&encoded, Scope::Repository(REPO)).unwrap();
    assert!(reloaded.revoke(token.id()));
    assert!(!reloaded.revoke(token.id()));
    assert!(reloaded.authorize(&parsed, Permission::Read, 0).is_err());
    let mut wrong = exposed.to_string();
    wrong.replace_range(101..102, if exposed.ends_with('0') { "1" } else { "0" });
    assert!(
        policy
            .authorize(&BearerToken::parse(&wrong).unwrap(), Permission::Read, 0)
            .is_err()
    );
}
#[test]
fn operator_capability_never_implies_repository_access() {
    let mut operator = TokenPolicy::new(Scope::Operator);
    assert!(operator.issue(ADMIN, Permission::Admin, None).is_err());
    let token = operator
        .issue(ADMIN, Permission::CreateRepository, None)
        .unwrap();
    assert!(
        operator
            .authorize(&token, Permission::CreateRepository, 0)
            .is_ok()
    );
    assert!(operator.authorize(&token, Permission::Read, 0).is_err());
    let mut repository = TokenPolicy::new(Scope::Repository(REPO));
    assert!(
        repository
            .issue(ADMIN, Permission::CreateRepository, None)
            .is_err()
    );
    let admin = repository.issue(ADMIN, Permission::Admin, None).unwrap();
    for permission in [Permission::Read, Permission::Write, Permission::Admin] {
        assert!(repository.authorize(&admin, permission, 0).is_ok());
    }
    assert!(repository.authorize(&token, Permission::Admin, 0).is_err());
    assert!(
        operator
            .authorize(&admin, Permission::CreateRepository, 0)
            .is_err()
    );
}
#[test]
fn malformed_credentials_policies_and_limits_fail_closed() {
    for malformed in [
        "".to_owned(),
        "jst1_".to_owned(),
        "é".repeat(51),
        format!("jst1_{}", "é".repeat(48)),
        "A".repeat(102),
    ] {
        assert!(BearerToken::parse(&malformed).is_err());
    }
    let mut policy = TokenPolicy::new(Scope::Repository(REPO));
    for _ in 0..128 {
        policy.issue(ACTOR, Permission::Read, None).unwrap();
    }
    assert!(policy.issue(ACTOR, Permission::Read, None).is_err());
    let bytes = policy.encode().unwrap();
    assert!(TokenPolicy::decode(&bytes, Scope::Repository(REPO)).is_ok());
    assert!(TokenPolicy::decode(&vec![b' '; 65537], Scope::Repository(REPO)).is_err());
    let text = String::from_utf8(bytes).unwrap();
    for bad in [
        format!(" {text}"),
        text.replace("\"format\":1", "\"format\":2"),
        text.replace("\"read\"", "\"create_repository\""),
    ] {
        assert!(TokenPolicy::decode(bad.as_bytes(), Scope::Repository(REPO)).is_err());
    }
}
#[test]
fn listener_configuration_rejects_insecure_shared_or_implicit_development_access() {
    assert!(validate_listener("0.0.0.0:8000".parse().unwrap(), false, false).is_err());
    assert!(validate_listener("0.0.0.0:8000".parse().unwrap(), true, true).is_err());
    assert!(validate_listener("0.0.0.0:8000".parse().unwrap(), true, false).is_ok());
    assert!(validate_listener("127.0.0.1:8000".parse().unwrap(), false, true).is_ok());
    assert!(validate_listener("[::1]:8000".parse().unwrap(), false, true).is_ok());
    assert!(validate_listener("[::]:8000".parse().unwrap(), false, true).is_err());
}
struct TestClock(AtomicU64);
impl Clock for TestClock {
    fn unix_seconds(&self) -> jj_store_service::auth::Result<u64> {
        Ok(self.0.load(Ordering::SeqCst))
    }
}
struct Source {
    scope: SourceScope,
    reads: AtomicUsize,
}
#[async_trait::async_trait]
impl DependencySource for Source {
    fn scope(&self) -> SourceScope {
        self.scope
    }
    async fn load_verified(&self, key: ObjectKey, _: u64) -> wal::inventory::Result<VerifiedNode> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        Ok(VerifiedNode {
            key,
            representation: Representation::Virtual,
            dependencies: BTreeSet::new(),
        })
    }
}
struct FixtureSemantics;
#[async_trait::async_trait]
impl SemanticPolicy for FixtureSemantics {
    async fn validate(
        &self,
        _: &ManifestSnapshot,
        _: Principal,
        request: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        if request.result == [0xde] {
            return Err(wal::Error::Denied);
        }
        Ok(())
    }
}
struct PausedStore {
    inner: MemoryStore,
    pause: AtomicBool,
    entered: tokio::sync::Notify,
    resume: tokio::sync::Notify,
}
impl Default for PausedStore {
    fn default() -> Self {
        Self {
            inner: MemoryStore::default(),
            pause: AtomicBool::new(false),
            entered: tokio::sync::Notify::new(),
            resume: tokio::sync::Notify::new(),
        }
    }
}
#[async_trait::async_trait]
impl ObjectStore for PausedStore {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.inner.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        if key.as_str().ends_with("/manifest.json") && self.pause.swap(false, Ordering::SeqCst) {
            self.entered.notify_one();
            self.resume.notified().await;
        }
        self.inner.put(key, condition, upload).await
    }
}
fn coordinator(
    store: Arc<PausedStore>,
    source: Arc<Source>,
    clock: Arc<TestClock>,
    token: BearerToken,
) -> Coordinator {
    let access = RepositoryAccess::new(REPO, store.clone(), token, clock);
    Coordinator::new(
        store,
        REPO,
        source,
        Arc::new(AuthenticatedGuard::new(access, Arc::new(FixtureSemantics))),
        Limits::default(),
    )
    .unwrap()
}
fn request(n: u8, actor: PrincipalId) -> Request {
    Request {
        principal: actor,
        request_id: RequestId::new([n; 16]),
        publication_id: PublicationId::new([n; 16]),
        transition: Some(Transition::Native {
            remove: BTreeSet::from([OperationId::new([0; 64])]),
            add: OperationId::new([n; 64]),
        }),
        settings: None,
        result: vec![n],
        kind: SemanticKind::OperationPublished,
    }
}
struct Fixture {
    store: Arc<PausedStore>,
    source: Arc<Source>,
    clock: Arc<TestClock>,
    policy: TokenPolicy,
    admin: Coordinator,
    writer: Coordinator,
    reader: Coordinator,
    reader_token: BearerToken,
    writer_token: BearerToken,
}
async fn fixture() -> Fixture {
    let store = Arc::new(PausedStore::default());
    let source = Arc::new(Source {
        scope: SourceScope::new(REPO),
        reads: AtomicUsize::new(0),
    });
    let clock = Arc::new(TestClock(AtomicU64::new(1)));
    let mut policy = TokenPolicy::new(Scope::Repository(REPO));
    let admin_token = policy.issue(ADMIN, Permission::Admin, None).unwrap();
    let writer_token = policy.issue(ACTOR, Permission::Write, Some(10)).unwrap();
    let reader_token = policy.issue(ACTOR, Permission::Read, None).unwrap();
    let admin = coordinator(store.clone(), source.clone(), clock.clone(), admin_token);
    let writer = coordinator(
        store.clone(),
        source.clone(),
        clock.clone(),
        writer_token.clone(),
    );
    let reader = coordinator(
        store.clone(),
        source.clone(),
        clock.clone(),
        reader_token.clone(),
    );
    admin
        .initialize(EpochId::new([1; 16]), policy.encode().unwrap())
        .await
        .unwrap();
    Fixture {
        store,
        source,
        clock,
        policy,
        admin,
        writer,
        reader,
        reader_token,
        writer_token,
    }
}
#[tokio::test]
async fn actor_permission_expiry_and_semantic_denial_precede_dependency_reads() {
    let f = fixture().await;
    let before = f.source.reads.load(Ordering::SeqCst);
    assert!(matches!(
        f.reader.publish(&request(1, ACTOR)).await,
        Err(wal::Error::Denied)
    ));
    assert!(matches!(
        f.writer.publish(&request(1, ADMIN)).await,
        Err(wal::Error::Denied)
    ));
    let mut denied = request(1, ACTOR);
    denied.result = vec![0xde];
    assert!(matches!(
        f.writer.publish(&denied).await,
        Err(wal::Error::Denied)
    ));
    let mut settings = request(1, ACTOR);
    settings.settings = Some(f.policy.encode().unwrap());
    settings.kind = SemanticKind::SettingsChanged;
    assert!(matches!(
        f.writer.publish(&settings).await,
        Err(wal::Error::Denied)
    ));
    f.clock.0.store(10, Ordering::SeqCst);
    assert!(matches!(
        f.writer.publish(&request(1, ACTOR)).await,
        Err(wal::Error::Denied)
    ));
    assert_eq!(f.source.reads.load(Ordering::SeqCst), before);
    assert_eq!(
        f.admin.read().await.unwrap().manifest().published,
        WalPosition(0)
    );
}
#[tokio::test]
async fn revocation_wins_publication_race_and_blocks_receipt_replay() {
    let mut f = fixture().await;
    let first = request(1, ACTOR);
    assert!(matches!(
        f.writer.publish(&first).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    f.store.pause.store(true, Ordering::SeqCst);
    let writer = f.writer.clone();
    let delayed = tokio::spawn(async move { writer.publish(&request(3, ACTOR)).await });
    f.store.entered.notified().await;
    f.policy.revoke(f.writer_token.id());
    f.policy.revoke(f.reader_token.id());
    let mut revoke = request(2, ADMIN);
    revoke.transition = None;
    revoke.settings = Some(f.policy.encode().unwrap());
    revoke.kind = SemanticKind::SettingsChanged;
    assert!(matches!(
        f.admin.publish(&revoke).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    f.store.resume.notify_one();
    assert!(matches!(delayed.await.unwrap(), Err(wal::Error::Denied)));
    assert!(matches!(
        f.writer.status(&first).await,
        Err(wal::Error::Denied)
    ));
    assert!(matches!(
        f.writer.publish(&first).await,
        Err(wal::Error::Denied)
    ));
    let reader = RepositoryAccess::new(REPO, f.store.clone(), f.reader_token, f.clock.clone());
    assert!(matches!(
        reader.authorize_current(Permission::Read).await,
        Err(wal::Error::Denied)
    ));
    let snapshot = f.admin.recover(100).await.unwrap();
    assert_eq!(snapshot.manifest().published, WalPosition(2));
    assert_eq!(
        snapshot.manifest().heads,
        BTreeSet::from([OperationId::new([1; 64])])
    );
}
#[tokio::test]
async fn replacement_policy_must_retain_scope_and_can_be_read_after_restart() {
    let f = fixture().await;
    let access = RepositoryAccess::new(
        REPO,
        f.store.clone(),
        BearerToken::parse(&f.reader_token.expose()).unwrap(),
        f.clock.clone(),
    );
    assert_eq!(
        access
            .authorize_current(Permission::Read)
            .await
            .unwrap()
            .id(),
        ACTOR
    );
    assert!(access.authorize_current(Permission::Write).await.is_err());
    let mut wrong = request(1, ADMIN);
    wrong.transition = None;
    wrong.kind = SemanticKind::SettingsChanged;
    wrong.settings = Some(TokenPolicy::new(Scope::Operator).encode().unwrap());
    assert!(f.admin.publish(&wrong).await.is_err());
    wrong.settings = Some(
        TokenPolicy::new(Scope::Repository(RepositoryId::new([9; 16])))
            .encode()
            .unwrap(),
    );
    assert!(f.admin.publish(&wrong).await.is_err());
    assert_eq!(
        f.admin.read().await.unwrap().manifest().published,
        WalPosition(0)
    );
}
