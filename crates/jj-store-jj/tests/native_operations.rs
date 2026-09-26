use jj_lib::{
    object_id::{HexPrefix, ObjectId as _, PrefixResolution},
    op_heads_store::OpHeadsStore as _,
    op_store::{self as o, OpStore as _},
};
use jj_store_jj::{codec::NativeCodec as _, objects::NativeObjects, operation_stores::*, roots};
use jj_store_model as m;
use jj_store_object::memory::{Fault, MemoryStore};
use jj_store_wal::{self as wal, publication::*};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::{Duration, SystemTime},
};

struct Guard(Arc<AtomicBool>);
#[async_trait::async_trait]
impl PublicationGuard for Guard {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        if self.0.load(Ordering::SeqCst) {
            Ok(())
        } else {
            Err(wal::Error::Denied)
        }
    }
    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }
}
type Transitions = BTreeMap<(BTreeSet<m::OperationId>, m::OperationId), Request>;
struct FixtureAuthority {
    coordinator: Coordinator,
    allowed: Arc<AtomicBool>,
    slow: AtomicBool,
    empty: AtomicBool,
    serial: AtomicU64,
    requests: Mutex<Transitions>,
}
#[async_trait::async_trait]
impl OperationAuthority for FixtureAuthority {
    async fn authorize_read(&self) -> AuthorityResult<()> {
        if self.allowed.load(Ordering::SeqCst) {
            Ok(())
        } else {
            Err(wal::Error::Denied.into())
        }
    }
    async fn authorize_write(&self) -> AuthorityResult<()> {
        self.authorize_read().await
    }
    async fn heads(&self) -> AuthorityResult<BTreeSet<m::OperationId>> {
        if self.empty.load(Ordering::SeqCst) {
            return Ok(BTreeSet::new());
        }
        Ok(self.coordinator.read().await?.manifest().heads.clone())
    }
    async fn operation_ids(&self, limit: usize) -> AuthorityResult<BTreeSet<m::OperationId>> {
        if self.slow.load(Ordering::SeqCst) {
            std::future::pending::<()>().await;
        }
        let snapshot = self.coordinator.read().await?;
        let ids: BTreeSet<_> = self
            .coordinator
            .inventory(&snapshot, 1000)
            .await?
            .into_iter()
            .filter_map(|n| match n.key {
                m::ObjectKey::Operation(id) => Some(id),
                _ => None,
            })
            .collect();
        if ids.len() > limit {
            return Err("fixture inventory limit".into());
        }
        Ok(ids)
    }
    async fn publish(
        &self,
        old: BTreeSet<m::OperationId>,
        new: m::OperationId,
    ) -> AuthorityResult<()> {
        let request = {
            let mut requests = self.requests.lock().unwrap();
            requests
                .entry((old.clone(), new))
                .or_insert_with(|| {
                    let n = self.serial.fetch_add(1, Ordering::SeqCst);
                    let mut id = [0; 16];
                    id[..8].copy_from_slice(&n.to_be_bytes());
                    Request {
                        principal: m::PrincipalId::new([1; 16]),
                        request_id: m::RequestId::new(id),
                        publication_id: m::PublicationId::new(id),
                        transition: Some(Transition::Native {
                            remove: old,
                            add: new,
                        }),
                        settings: None,
                        result: new.as_bytes().to_vec(),
                        kind: SemanticKind::OperationPublished,
                    }
                })
                .clone()
        };
        match self.coordinator.publish(&request).await? {
            PublishOutcome::Committed(_) => Ok(()),
            PublishOutcome::OutcomeUnknown { .. } => Err(wal::Error::OutcomeUnknown.into()),
        }
    }
}
struct Fixture {
    storage: Arc<MemoryStore>,
    objects: Arc<NativeObjects>,
    authority: Arc<FixtureAuthority>,
}
impl Fixture {
    fn ops(&self, limits: OperationLimits) -> NativeOpStore {
        NativeOpStore::new(self.objects.clone(), self.authority.clone(), limits).unwrap()
    }
    fn heads(&self) -> NativeOpHeadsStore {
        NativeOpHeadsStore::new(self.authority.clone())
    }
}
async fn fixture() -> Fixture {
    let storage = Arc::new(MemoryStore::default());
    let repository = m::RepositoryId::new([1; 16]);
    let objects = Arc::new(NativeObjects::new(storage.clone(), repository, 1024 * 1024).unwrap());
    let allowed = Arc::new(AtomicBool::new(true));
    let coordinator = Coordinator::new(
        storage.clone(),
        repository,
        objects.clone(),
        Arc::new(Guard(allowed.clone())),
        Limits::default(),
    )
    .unwrap();
    coordinator
        .initialize(m::EpochId::new([1; 16]), vec![])
        .await
        .unwrap();
    Fixture {
        storage,
        objects,
        authority: Arc::new(FixtureAuthority {
            coordinator,
            allowed,
            slow: AtomicBool::new(false),
            empty: AtomicBool::new(false),
            serial: AtomicU64::new(1),
            requests: Mutex::new(BTreeMap::new()),
        }),
    }
}
fn operation(view: o::ViewId, label: &str) -> o::Operation {
    let mut op = roots::operation();
    op.parents = vec![o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())];
    op.view_id = view;
    op.metadata.description = label.to_owned();
    op
}
#[tokio::test]
async fn native_operation_traits_preserve_identity_fields_and_virtual_roots() {
    let f = fixture().await;
    let ops = f.ops(OperationLimits::default());
    assert_eq!(ops.name(), "jj-store");
    assert_eq!(
        ops.read_operation(ops.root_operation_id()).await.unwrap(),
        roots::operation()
    );
    assert_eq!(
        ops.read_view(&o::ViewId::from_bytes(roots::ROOT_VIEW.as_bytes()))
            .await
            .unwrap(),
        roots::view()
    );
    let mut view = roots::view();
    view.wc_commit_ids.insert(
        "independent-client".into(),
        jj_lib::backend::CommitId::from_bytes(roots::ROOT_COMMIT.as_bytes()),
    );
    let view_id = ops.write_view(&view).await.unwrap();
    assert_eq!(view_id.as_bytes(), view.key().as_bytes());
    assert_eq!(ops.read_view(&view_id).await.unwrap(), view);
    let mut op = operation(view_id, "complete native operation");
    op.commit_predecessors = None;
    let absent = ops.write_operation(&op).await.unwrap();
    assert_eq!(absent.as_bytes(), op.key().as_bytes());
    assert_eq!(ops.read_operation(&absent).await.unwrap(), op);
    op.commit_predecessors = Some(BTreeMap::new());
    let present = ops.write_operation(&op).await.unwrap();
    assert_ne!(absent, present);
    f.heads()
        .update_op_heads(&[ops.root_operation_id().clone()], &present)
        .await
        .unwrap();
    let restarted = f.ops(OperationLimits::default());
    assert_eq!(restarted.read_operation(&present).await.unwrap(), op);
    assert_eq!(
        restarted
            .resolve_operation_id_prefix(&HexPrefix::from_id(&present))
            .await
            .unwrap(),
        PrefixResolution::SingleMatch(present)
    );
    assert!(
        ops.read_operation(&o::OperationId::from_bytes(&[1]))
            .await
            .is_err()
    );
    assert!(ops.read_view(&o::ViewId::from_bytes(&[1])).await.is_err());
    assert!(ops.gc(&[], SystemTime::now()).await.is_err());
}
#[tokio::test]
async fn staged_prefixes_are_session_local_and_reads_publish_nothing() {
    let f = fixture().await;
    let one = f.ops(OperationLimits::default());
    let two = f.ops(OperationLimits::default());
    let view = one.write_view(&roots::view()).await.unwrap();
    let op = one
        .write_operation(&operation(view, "staged only"))
        .await
        .unwrap();
    let prefix = HexPrefix::from_id(&op);
    assert_eq!(
        one.resolve_operation_id_prefix(&prefix).await.unwrap(),
        PrefixResolution::SingleMatch(op.clone())
    );
    assert_eq!(
        two.resolve_operation_id_prefix(&prefix).await.unwrap(),
        PrefixResolution::NoMatch
    );
    assert_eq!(
        f.heads().get_op_heads().await.unwrap(),
        vec![one.root_operation_id().clone()]
    );
    assert_eq!(
        f.authority
            .coordinator
            .read()
            .await
            .unwrap()
            .manifest()
            .published,
        m::WalPosition(0)
    );
    f.heads()
        .update_op_heads(&[one.root_operation_id().clone()], &op)
        .await
        .unwrap();
    assert_eq!(
        two.resolve_operation_id_prefix(&prefix).await.unwrap(),
        PrefixResolution::SingleMatch(op)
    );
    let before = f.authority.coordinator.read().await.unwrap().digest();
    two.resolve_operation_id_prefix(&HexPrefix::try_from_hex("").unwrap())
        .await
        .unwrap();
    f.heads().get_op_heads().await.unwrap();
    assert_eq!(
        f.authority.coordinator.read().await.unwrap().digest(),
        before
    );
}
#[tokio::test]
async fn concurrent_native_head_updates_keep_siblings_and_receipt_retries() {
    let f = fixture().await;
    let a = f.ops(OperationLimits::default());
    let b = f.ops(OperationLimits::default());
    let view = a.write_view(&roots::view()).await.unwrap();
    let first = a
        .write_operation(&operation(view.clone(), "first"))
        .await
        .unwrap();
    let second = b.write_operation(&operation(view, "second")).await.unwrap();
    let heads_a = f.heads();
    let heads_b = f.heads();
    let old = [a.root_operation_id().clone()];
    let (ra, rb) = tokio::join!(
        heads_a.update_op_heads(&old, &first),
        heads_b.update_op_heads(&old, &second)
    );
    ra.unwrap();
    rb.unwrap();
    assert_eq!(
        heads_a
            .get_op_heads()
            .await
            .unwrap()
            .into_iter()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([first.clone(), second])
    );
    let before = f
        .authority
        .coordinator
        .read()
        .await
        .unwrap()
        .manifest()
        .published;
    heads_a.update_op_heads(&old, &first).await.unwrap();
    assert_eq!(
        f.authority
            .coordinator
            .read()
            .await
            .unwrap()
            .manifest()
            .published,
        before
    );
    assert_eq!(
        f.authority
            .coordinator
            .recover(1000)
            .await
            .unwrap()
            .manifest()
            .published,
        m::WalPosition(2)
    );
}
#[tokio::test]
async fn failed_upload_and_revocation_never_report_native_success() {
    let f = fixture().await;
    let ops = f.ops(OperationLimits::default());
    f.storage.inject(Fault::BeforeWrite);
    assert!(ops.write_view(&roots::view()).await.is_err());
    let view = ops.write_view(&roots::view()).await.unwrap();
    let id = ops
        .write_operation(&operation(view, "authorized staging"))
        .await
        .unwrap();
    f.authority.allowed.store(false, Ordering::SeqCst);
    assert!(ops.read_operation(&id).await.is_err());
    assert!(ops.read_operation(ops.root_operation_id()).await.is_err());
    assert!(ops.write_view(&roots::view()).await.is_err());
    assert!(
        ops.resolve_operation_id_prefix(&HexPrefix::from_id(&id))
            .await
            .is_err()
    );
    assert!(f.heads().get_op_heads().await.is_err());
    assert!(f.heads().update_op_heads(&[], &id).await.is_err());
    assert_eq!(
        f.authority
            .coordinator
            .read()
            .await
            .unwrap()
            .manifest()
            .published,
        m::WalPosition(0)
    );
}
#[tokio::test]
async fn prefix_queries_fail_on_budget_or_deadline_instead_of_partial_matches() {
    let f = fixture().await;
    assert!(
        NativeOpStore::new(
            f.objects.clone(),
            f.authority.clone(),
            OperationLimits {
                max_operations: 0,
                ..OperationLimits::default()
            }
        )
        .is_err()
    );
    let limits = OperationLimits {
        max_operations: 1,
        timeout: Duration::from_millis(50),
    };
    let ops = f.ops(limits);
    let view = ops.write_view(&roots::view()).await.unwrap();
    let id = ops
        .write_operation(&operation(view, "exceeds root plus staged budget"))
        .await
        .unwrap();
    assert!(
        ops.resolve_operation_id_prefix(&HexPrefix::from_id(&id))
            .await
            .is_err()
    );
    f.authority.slow.store(true, Ordering::SeqCst);
    assert!(
        f.ops(OperationLimits {
            timeout: Duration::from_millis(20),
            ..OperationLimits::default()
        })
        .resolve_operation_id_prefix(&HexPrefix::from_id(&id))
        .await
        .is_err()
    );
}
#[tokio::test]
async fn invalid_heads_and_invalid_id_lengths_fail_before_publication() {
    let f = fixture().await;
    f.authority.empty.store(true, Ordering::SeqCst);
    assert!(f.heads().get_op_heads().await.is_err());
    f.authority.empty.store(false, Ordering::SeqCst);
    assert!(
        f.heads()
            .update_op_heads(&[], &o::OperationId::from_bytes(&[1]))
            .await
            .is_err()
    );
    assert!(
        f.heads()
            .update_op_heads(
                &[o::OperationId::from_bytes(&[1])],
                &o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())
            )
            .await
            .is_err()
    );
    assert_eq!(
        f.authority
            .coordinator
            .read()
            .await
            .unwrap()
            .manifest()
            .published,
        m::WalPosition(0)
    );
}

#[tokio::test]
async fn staged_operation_limit_preserves_identical_write_retry() {
    let f = fixture().await;
    let ops = f.ops(OperationLimits {
        max_operations: 1,
        ..OperationLimits::default()
    });
    let view = ops.write_view(&roots::view()).await.unwrap();
    let operation = operation(view, "idempotent metadata upload");
    let first = ops.write_operation(&operation).await.unwrap();
    assert_eq!(ops.write_operation(&operation).await.unwrap(), first);
    let mut another = operation;
    another.metadata.description.push_str(" different");
    assert!(ops.write_operation(&another).await.is_err());
}
