use super::*;
use std::cell::RefCell;

type Probe = Box<dyn FnOnce(&Path)>;
thread_local! {
    static HASH_PROBE: RefCell<Option<Probe>> = RefCell::new(None);
}
pub(super) fn during_hash(path: &Path) {
    if let Some(probe) = HASH_PROBE.with(|probe| probe.borrow_mut().take()) {
        probe(path);
    }
}
fn observe_hash(root: &Path, policy: Policy, pinned: bool) -> Arc<std::sync::atomic::AtomicBool> {
    let root = root.to_owned();
    let entered = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let observed = entered.clone();
    HASH_PROBE.with(|probe| {
        *probe.borrow_mut() = Some(Box::new(move |path| {
            let budget = Budget::new(Limits::default()).unwrap();
            let namespace = Cache::open(&root, policy, &budget)
                .expect("complete file hashing retained the namespace lock");
            assert!(namespace.bytes() >= 8 * 1024 * 1024);
            assert_eq!(namespace.slots.len(), usize::from(pinned));
            assert_eq!(namespace.reservations.len(), usize::from(!pinned));
            assert!(path.join("data").is_file());
            assert!(
                namespace.bytes() < policy.max_entry_bytes,
                "immutable recovery charged the full construction allowance"
            );
            observed.store(true, std::sync::atomic::Ordering::Relaxed);
        }));
    });
    entered
}

struct Access;
#[async_trait::async_trait]
impl ReadAccess for Access {
    fn repository(&self) -> m::RepositoryId {
        m::RepositoryId::new([71; 16])
    }
    fn epoch(&self) -> m::EpochId {
        m::EpochId::new([72; 16])
    }
    fn operation(&self) -> m::OperationId {
        m::OperationId::new([73; 64])
    }
    async fn contains(&self, _: m::ObjectKey) -> Result<bool> {
        panic!("cache admission read native membership")
    }
    async fn inventory(&self) -> Result<Arc<BTreeSet<m::ObjectKey>>> {
        panic!("cache admission enumerated native objects")
    }
    async fn authorize(&self) -> Result<()> {
        Ok(())
    }
}
struct NoReads;
#[async_trait::async_trait]
impl ObjectStore for NoReads {
    async fn head(
        &self,
        _: &jj_store_object::Key,
    ) -> jj_store_object::Result<jj_store_object::Metadata> {
        panic!("cache admission read native metadata")
    }
    async fn get(
        &self,
        _: &jj_store_object::Key,
        _: Option<jj_store_object::ByteRange>,
    ) -> jj_store_object::Result<jj_store_object::Download> {
        panic!("cache admission read native bytes")
    }
    async fn put(
        &self,
        _: &jj_store_object::Key,
        _: jj_store_object::Condition,
        _: &jj_store_object::StagedUpload,
    ) -> jj_store_object::Result<jj_store_object::WriteOutcome> {
        panic!("cache admission wrote native state")
    }
}
fn stores() -> NativeStores {
    NativeStores::new(
        Arc::new(NativeObjects::new(Arc::new(NoReads), Access.repository(), 1024).unwrap()),
        Arc::new(ReadOnlyAuthority {
            access: Arc::new(Access),
            budget: Arc::new(Budget::new(Limits::default()).unwrap()),
        }),
        BackendLimits::default(),
        OperationLimits::default(),
    )
    .unwrap()
}
fn cold_slot(root: &Path, policy: Policy) -> PathBuf {
    let budget = Budget::new(Limits::default()).unwrap();
    let cache = Cache::open(root, policy, &budget).unwrap();
    let path = cache.path(0);
    fs::create_dir(&path).unwrap();
    for name in ["store", "op_store", "op_heads"] {
        let component = path.join(name);
        fs::create_dir(&component).unwrap();
        fs::write(component.join("type"), b"jj-store").unwrap();
        fs::write(
            component.join("native-profile"),
            format!(
                "jj-store-client-cache-v1\n{PROFILE}\n{}\n",
                Access.repository()
            ),
        )
        .unwrap();
    }
    fs::write(path.join("data"), vec![0x35; 8 * 1024 * 1024]).unwrap();
    let receipt = Receipt {
        version: 1,
        repository: Access.repository(),
        epoch: Access.epoch(),
        operation: Access.operation(),
        used: 1,
        files: inventory(&path, policy, &budget, true).unwrap().0,
    };
    fs::write(path.join(RECEIPT), serde_json::to_vec(&receipt).unwrap()).unwrap();
    path
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cold_cache_admission_does_not_charge_full_file_hashing_under_namespace() {
    let root = tempfile::tempdir().unwrap();
    let policy = Policy::default();
    cold_slot(root.path(), policy);
    let budget = Budget::new(Limits {
        max_work: 64,
        ..Limits::default()
    })
    .unwrap();
    let mut cache = Cache::open(root.path(), policy, &budget).unwrap();
    let mut entry = cache
        .prepare(
            &Access,
            &stores(),
            &budget,
            &resident::Residents::default(),
            false,
        )
        .expect("cold admission hashed the complete generation under the namespace");
    assert!(
        entry.private(),
        "cold files have no independent quota ownership"
    );
    assert!(!entry.initialize);
    assert!(budget.stats().work <= 64);
    assert!(entry.pending.is_some());
    assert!(
        entry.sealed.is_none(),
        "unhashed files were marked verified"
    );
    drop(cache);
    let entered = observe_hash(root.path(), policy, false);
    assert!(
        entry
            .verify(policy, &Budget::new(Limits::default()).unwrap())
            .unwrap()
    );
    assert!(entered.load(std::sync::atomic::Ordering::Relaxed));
    assert!(entry.sealed.is_some());
    drop(entry);
    assert_eq!(
        Cache::open(
            root.path(),
            policy,
            &Budget::new(Limits::default()).unwrap()
        )
        .unwrap()
        .bytes(),
        0
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cold_checksum_damage_rebuilds_private_files_and_preserves_shared_files() {
    for pinned in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let policy = Policy::default();
        let path = cold_slot(root.path(), policy);
        let pin = pinned.then(|| generation::Pin::acquire(&path).unwrap());
        fs::write(path.join("data"), vec![0x36; 8 * 1024 * 1024]).unwrap();
        let budget = Budget::new(Limits::default()).unwrap();
        let mut cache = Cache::open(root.path(), policy, &budget).unwrap();
        let mut entry = cache
            .prepare(
                &Access,
                &stores(),
                &budget,
                &resident::Residents::default(),
                false,
            )
            .unwrap();
        drop(cache);
        let entered = observe_hash(root.path(), policy, pinned);
        let verified = entry.verify(policy, &budget);
        assert!(entered.load(std::sync::atomic::Ordering::Relaxed));
        assert!(entry.sealed.is_none());
        assert_eq!(
            verified,
            if pinned {
                Err(Error::Unavailable)
            } else {
                Ok(false)
            }
        );
        assert!(
            entry.pending.is_some(),
            "failed proof was cleared for reuse"
        );
        assert_eq!(entry.seal(policy, &budget), Err(Error::Unavailable));
        drop(entry);
        let mut cache = Cache::open(root.path(), policy, &budget).unwrap();
        assert_eq!(path.exists(), pinned);
        assert_eq!(cache.reservations.len(), 0);
        if pinned {
            assert!(cache.bytes() >= 8 * 1024 * 1024);
            assert_eq!(cache.slots.len(), 1);
        } else {
            assert_eq!(cache.bytes(), 0);
            let rebuilt = cache.rebuild(&Access).unwrap();
            assert!(rebuilt.initialize);
            assert_eq!(rebuilt.state, State::Rebuilt);
            assert_eq!(cache.bytes(), policy.max_entry_bytes);
        }
        drop(pin);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cold_proof_budget_and_cancellation_do_not_become_rebuild_success() {
    for canceled in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let policy = Policy::default();
        cold_slot(root.path(), policy);
        let budget = Arc::new(
            Budget::new(Limits {
                max_work: 64,
                ..Limits::default()
            })
            .unwrap(),
        );
        let mut cache = Cache::open(root.path(), policy, &budget).unwrap();
        let mut entry = cache
            .prepare(
                &Access,
                &stores(),
                &budget,
                &resident::Residents::default(),
                false,
            )
            .unwrap();
        drop(cache);
        if canceled {
            let budget = budget.clone();
            HASH_PROBE.with(|probe| {
                *probe.borrow_mut() = Some(Box::new(move |_| budget.cancellation.cancel()))
            });
        }
        assert_eq!(
            entry.verify(policy, &budget),
            Err(if canceled {
                Error::Canceled
            } else {
                Error::Limit
            })
        );
        assert!(entry.sealed.is_none());
        drop(entry);
        assert_eq!(
            Cache::open(
                root.path(),
                policy,
                &Budget::new(Limits::default()).unwrap()
            )
            .unwrap()
            .bytes(),
            0
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn coarse_metadata_rehashes_before_namespace_reacquisition_and_detects_damage() {
    for damaged in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let policy = Policy::default();
        let path = cold_slot(root.path(), policy);
        fs::File::options()
            .write(true)
            .open(path.join("data"))
            .unwrap()
            .set_times(fs::FileTimes::new().set_modified(std::time::UNIX_EPOCH))
            .unwrap();
        let budget = Budget::new(Limits::default()).unwrap();
        let mut cache = Cache::open(root.path(), policy, &budget).unwrap();
        let mut entry = cache
            .prepare(
                &Access,
                &stores(),
                &budget,
                &resident::Residents::default(),
                false,
            )
            .unwrap();
        drop(cache);
        assert!(entry.verify(policy, &budget).unwrap());
        if damaged {
            fs::write(entry.path.join("data"), vec![0x36; 8 * 1024 * 1024]).unwrap();
        }
        let entered = observe_hash(root.path(), policy, false);
        let sealed = entry.seal(policy, &budget);
        assert!(
            entered.load(std::sync::atomic::Ordering::Relaxed),
            "coarse metadata reused an incomplete proof"
        );
        if damaged {
            assert_eq!(sealed, Err(Error::Unavailable));
        } else {
            sealed.unwrap();
            let small = Budget::new(Limits {
                max_work: 64,
                ..Limits::default()
            })
            .unwrap();
            let namespace = Cache::open(root.path(), policy, &small).unwrap();
            assert!(
                entry
                    .sealed
                    .as_ref()
                    .unwrap()
                    .matches(&entry.path, policy, &small)
                    .unwrap()
            );
            assert!(namespace.bytes() >= 8 * 1024 * 1024);
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cold_filter_upgrade_reserves_writable_construction_before_growth() {
    let root = tempfile::tempdir().unwrap();
    let policy = Policy::default();
    cold_slot(root.path(), policy);
    let budget = Budget::new(Limits::default()).unwrap();
    let mut cache = Cache::open(root.path(), policy, &budget).unwrap();
    let mut entry = cache
        .prepare(
            &Access,
            &stores(),
            &budget,
            &resident::Residents::default(),
            true,
        )
        .unwrap();
    assert_eq!(cache.bytes(), policy.max_entry_bytes);
    drop(cache);
    assert!(entry.verify(policy, &budget).unwrap());
    assert!(
        entry.sealed.is_none(),
        "writable upgrade retained an immutable seal"
    );
    assert_eq!(
        Cache::open(root.path(), policy, &budget).unwrap().bytes(),
        policy.max_entry_bytes
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn failed_cold_recovery_admission_preserves_existing_files_and_receipt() {
    let root = tempfile::tempdir().unwrap();
    let policy = Policy {
        max_entry_bytes: 8 * 1024 * 1024 + 4096,
        ..Default::default()
    };
    let path = cold_slot(root.path(), policy);
    let receipt_before = fs::read(path.join(RECEIPT)).unwrap();
    let budget = Budget::new(Limits::default()).unwrap();
    let mut cache = Cache::open(root.path(), policy, &budget).unwrap();
    assert!(matches!(
        cache.prepare(
            &Access,
            &stores(),
            &budget,
            &resident::Residents::default(),
            false
        ),
        Err(Error::Limit)
    ));
    drop(cache);
    assert_eq!(fs::read(path.join(RECEIPT)).unwrap(), receipt_before);
    assert!(path.join("data").is_file());
    let cache = Cache::open(root.path(), policy, &budget).unwrap();
    assert_eq!(cache.slots.len(), 1);
    assert_eq!(cache.reservations.len(), 0);
    assert!(cache.bytes() >= 8 * 1024 * 1024);
}
