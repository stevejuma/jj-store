use super::*;
use jj_store_wal::{
    catalog::Catalog,
    inventory::DependencySource as _,
    records::{RecordKind, Records},
};
use sha2::{Digest as _, Sha256};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountedInventory {
    inner: Arc<MemoryStore>,
    reads: AtomicUsize,
    descriptors: AtomicUsize,
    copy_bodies: AtomicUsize,
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn browser_operation_prefix_ignores_unrelated_inventory_pressure() {
    use jj_store_jj::query as q;
    use jj_store_service::reads::{OpenRead, ReadLimits};
    let mut f = Harness::new().await;
    retain_files(&f, 0, 96).await;
    let session = Arc::new(f.local());
    let OpenRead::Pinned(read) = session
        .open_read(
            None,
            ReadLimits {
                max_objects: 8,
                ..ReadLimits::default()
            },
        )
        .await
        .unwrap()
    else {
        panic!("single head")
    };
    let cache = tempfile::tempdir().unwrap();
    let executor = q::GraphExecutor::new(cache.path().to_owned(), q::Limits::default(), 2).unwrap();
    let result = Arc::new(read)
        .graph(
            &executor,
            q::Request::OperationsFiltered {
                query: format!("from:{}", &roots::ROOT_OPERATION.to_string()[..12]),
            },
            q::Page {
                limit: 1,
                after: None,
            },
        )
        .await
        .expect("one operation prefix must not charge unrelated files");
    let q::Nodes::Operations(nodes) = result.data else {
        panic!("operations")
    };
    assert_eq!(nodes[0].id, roots::ROOT_OPERATION);
    f.stop().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn browser_kind_lookup_retains_historical_roots_and_fresh_bounds() {
    use jj_store_jj::query::{self as q, InventoryKind as K, ReadAccess as _};
    use jj_store_service::reads::{OpenRead, ReadLimits};
    let mut f = Harness::new().await;
    retain_files(&f, 0, 96).await;
    let copy = selected_copy(&f).await;
    let counted = CountedInventory::new(f.store.clone());
    let session = Arc::new(
        NativeSession::new(
            counted.clone(),
            f.objects.clone(),
            f.reader.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    let OpenRead::Pinned(old) = session
        .open_read(
            None,
            ReadLimits {
                max_objects: 1,
                ..ReadLimits::default()
            },
        )
        .await
        .unwrap()
    else {
        panic!("single head")
    };
    let deadline = || tokio::time::Instant::now() + Duration::from_secs(10);
    let ops = BTreeSet::from([m::ObjectKey::Operation(roots::ROOT_OPERATION)]);
    let copies = BTreeSet::from([m::ObjectKey::Copy(copy)]);
    assert_eq!(
        old.inventory_kind(K::Operations, 8, deadline())
            .await
            .unwrap(),
        ops
    );
    assert!(counted.descriptors.load(Ordering::Relaxed) > 96);
    counted.reset();
    assert_eq!(
        old.inventory_kind(K::Operations, 8, deadline())
            .await
            .unwrap(),
        ops
    );
    counted.warm("browser warm operations");
    assert_eq!(
        old.inventory_kind(K::Copies, 8, deadline()).await.unwrap(),
        copies
    );
    counted.reset();
    assert_eq!(
        old.inventory_kind(K::Copies, 8, deadline()).await.unwrap(),
        copies
    );
    counted.warm("browser warm copies");

    // A real guarded operation publication followed by selected copy additions
    // must not replace the catalog retained by the older browser capability.
    let later = publish_operation(&f, "browser later operation").await;
    let (m::ObjectKey::Copy(new_copy), _) = f
        .objects
        .put(&b::CopyHistory {
            current_path: jj_lib::repo_path::RepoPathBuf::from_internal_string("later.txt")
                .unwrap(),
            parents: vec![b::CopyId::placeholder()],
            salt: vec![43],
        })
        .await
        .unwrap()
    else {
        panic!("copy")
    };
    select_keys(&f, vec![m::ObjectKey::Copy(new_copy)]).await;
    f.objects
        .put(&b::CopyHistory {
            current_path: jj_lib::repo_path::RepoPathBuf::from_internal_string("staged.txt")
                .unwrap(),
            parents: vec![b::CopyId::placeholder()],
            salt: vec![44],
        })
        .await
        .unwrap();
    let OpenRead::Pinned(current) = session
        .open_read(
            None,
            ReadLimits {
                max_objects: 2,
                ..ReadLimits::default()
            },
        )
        .await
        .unwrap()
    else {
        panic!("single head")
    };
    assert_eq!(
        current
            .inventory_kind(K::Operations, 8, deadline())
            .await
            .unwrap(),
        BTreeSet::from([
            m::ObjectKey::Operation(roots::ROOT_OPERATION),
            m::ObjectKey::Operation(later)
        ])
    );
    assert_eq!(
        current
            .inventory_kind(K::Copies, 8, deadline())
            .await
            .unwrap(),
        BTreeSet::from([m::ObjectKey::Copy(copy), m::ObjectKey::Copy(new_copy)])
    );
    assert_eq!(
        current
            .inventory_kind(K::Operations, 1, deadline())
            .await
            .unwrap_err(),
        q::Error::Limit
    );
    assert_eq!(
        current
            .inventory_kind(K::Copies, 1, deadline())
            .await
            .unwrap_err(),
        q::Error::Limit
    );
    let OpenRead::Pinned(tiny) = session
        .open_read(
            None,
            ReadLimits {
                max_objects: 1,
                ..ReadLimits::default()
            },
        )
        .await
        .unwrap()
    else {
        panic!("single head")
    };
    assert_eq!(
        tiny.inventory_kind(K::Operations, 8, deadline())
            .await
            .unwrap_err(),
        q::Error::Limit
    );
    counted.reset();
    assert_eq!(
        old.inventory_kind(K::Operations, 1, deadline())
            .await
            .unwrap(),
        ops
    );
    counted.warm("browser warm historical operation after publication");
    counted.reset();
    assert_eq!(
        old.inventory_kind(K::Copies, 1, deadline()).await.unwrap(),
        copies
    );
    counted.warm("browser warm historical copy after publication");
    assert_eq!(
        old.inventory_kind(K::Copies, 0, deadline())
            .await
            .unwrap_err(),
        q::Error::Limit
    );
    assert_eq!(
        old.inventory_kind(K::Copies, 8, tokio::time::Instant::now())
            .await
            .unwrap_err(),
        q::Error::Limit
    );
    let OpenRead::Pinned(expiring) = session
        .open_read(
            None,
            ReadLimits {
                max_objects: 2,
                timeout: Duration::from_millis(30),
            },
        )
        .await
        .unwrap()
    else {
        panic!("single head")
    };
    tokio::time::sleep(Duration::from_millis(40)).await;
    assert_eq!(
        expiring
            .inventory_kind(K::Operations, 8, deadline())
            .await
            .unwrap_err(),
        q::Error::Limit
    );
    f.revoke_reader().await;
    assert_eq!(
        old.inventory_kind(K::Operations, 1, deadline())
            .await
            .unwrap_err(),
        q::Error::Denied
    );
    assert_eq!(
        current
            .inventory_kind(K::Copies, 8, deadline())
            .await
            .unwrap_err(),
        q::Error::Denied
    );
    f.stop().await;
}
async fn copy_node(f: &Harness, name: &str, parents: &[m::CopyId]) -> m::CopyId {
    let (m::ObjectKey::Copy(id), _) = f
        .objects
        .put(&b::CopyHistory {
            current_path: jj_lib::repo_path::RepoPathBuf::from_internal_string(name).unwrap(),
            parents: parents
                .iter()
                .map(|id| b::CopyId::from_bytes(id.as_bytes()))
                .collect(),
            salt: vec![],
        })
        .await
        .unwrap()
    else {
        panic!("copy kind")
    };
    id
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_copy_family_ignores_unrelated_inventory_and_body_pressure() {
    use b::Backend as _;
    use jj_store_jj::backend::NativeBackend;
    let mut f = Harness::new().await;
    retain_files(&f, 0, 96).await;
    let a = copy_node(&f, "a", &[]).await;
    let requested = copy_node(&f, "b", &[a]).await;
    let sibling = copy_node(&f, "c", &[a]).await;
    let unrelated = copy_node(&f, "unrelated", &[]).await;
    let merge = copy_node(&f, "merge", &[a, unrelated]).await;
    let leaf = copy_node(&f, "leaf", &[merge]).await;
    let other = copy_node(&f, "other", &[unrelated]).await;
    let mut keys = vec![a, requested, sibling, unrelated, merge, leaf, other];
    for n in 0..96 {
        keys.push(copy_node(&f, &format!("unrelated-{n}"), &[]).await);
    }
    select_keys(&f, keys.into_iter().map(m::ObjectKey::Copy).collect()).await;
    let counted = CountedInventory::new(f.store.clone());
    let objects = Arc::new(NativeObjects::new(counted.clone(), REPO, MAX_BLOB).unwrap());
    let session = Arc::new(
        NativeSession::new(
            counted.clone(),
            objects.clone(),
            f.reader.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    let backend = NativeBackend::new(
        objects,
        session,
        BackendLimits {
            max_nodes: 5,
            max_work: 64,
            max_bytes: 64 * 1024,
            ..BackendLimits::default()
        },
    )
    .unwrap();
    let expected: BTreeSet<_> = [a, requested, sibling, merge, leaf].into_iter().collect();
    let request = b::CopyId::from_bytes(requested.as_bytes());
    let related = backend
        .get_related_copies(&request)
        .await
        .expect("small copy family must not charge unrelated selected copies");
    let actual = related
        .iter()
        .map(|copy| m::CopyId::from_slice(copy.id.as_bytes()).unwrap())
        .collect();
    assert_eq!(expected, actual);
    assert_eq!(counted.copy_bodies.load(Ordering::Relaxed), 5);
    let position = |id: m::CopyId| {
        related
            .iter()
            .position(|copy| copy.id.as_bytes() == id.as_bytes())
            .unwrap()
    };
    assert!(position(leaf) < position(merge) && position(merge) < position(a));
    assert!(position(requested) < position(a) && position(sibling) < position(a));
    counted.reset();
    assert_eq!(backend.get_related_copies(&request).await.unwrap(), related);
    assert_eq!(counted.descriptors.load(Ordering::Relaxed), 0);
    assert_eq!(counted.copy_bodies.load(Ordering::Relaxed), 5);
    f.revoke_reader().await;
    assert!(backend.get_related_copies(&request).await.is_err());
    f.stop().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pinned_copy_families_reuse_additions_without_loading_bodies() {
    use jj_store_jj::query::{self as q, ReadAccess as _, copy_index::LookupLimits};
    use jj_store_service::reads::{OpenRead, ReadLimits};
    let mut f = Harness::new().await;
    retain_files(&f, 0, 96).await;
    let root = copy_node(&f, "root", &[]).await;
    let child = copy_node(&f, "child", &[root]).await;
    select_keys(
        &f,
        vec![m::ObjectKey::Copy(root), m::ObjectKey::Copy(child)],
    )
    .await;
    let counted = CountedInventory::new(f.store.clone());
    let session = Arc::new(
        NativeSession::new(
            counted.clone(),
            f.objects.clone(),
            f.reader.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    let OpenRead::Pinned(first) = session
        .clone()
        .open_read(None, ReadLimits::default())
        .await
        .unwrap()
    else {
        panic!("single head")
    };
    let deadline = || tokio::time::Instant::now() + Duration::from_secs(10);
    let seeds = BTreeSet::from([child]);
    let limits = LookupLimits {
        max_nodes: 3,
        max_work: 64,
        max_bytes: 64 * 1024,
    };
    let expected = BTreeSet::from([root, child]);
    assert_eq!(
        first
            .copy_family(&seeds, &seeds, limits, deadline())
            .await
            .unwrap()
            .unwrap()
            .ids,
        expected
    );
    assert_eq!(counted.copy_bodies.load(Ordering::Relaxed), 0);
    assert!(counted.descriptors.load(Ordering::Relaxed) > 96);
    publish_operation(&f, "copy family operation-only addition").await;
    let OpenRead::Pinned(second) = session
        .clone()
        .open_read(None, ReadLimits::default())
        .await
        .unwrap()
    else {
        panic!("single head")
    };
    counted.reset();
    assert_eq!(
        second
            .copy_family(&seeds, &seeds, limits, deadline())
            .await
            .unwrap()
            .unwrap()
            .ids,
        expected
    );
    assert!(
        counted.descriptors.load(Ordering::Relaxed) <= 3,
        "operation-only increment scanned unchanged descriptors"
    );
    assert_eq!(counted.copy_bodies.load(Ordering::Relaxed), 0);
    let leaf = copy_node(&f, "leaf", &[child]).await;
    select_keys(&f, vec![m::ObjectKey::Copy(leaf)]).await;
    let OpenRead::Pinned(third) = session
        .clone()
        .open_read(None, ReadLimits::default())
        .await
        .unwrap()
    else {
        panic!("single head")
    };
    counted.reset();
    let extended = BTreeSet::from([root, child, leaf]);
    assert_eq!(
        third
            .copy_family(&seeds, &seeds, limits, deadline())
            .await
            .unwrap()
            .unwrap()
            .ids,
        extended
    );
    assert!(
        counted.descriptors.load(Ordering::Relaxed) <= 3,
        "copy increment scanned unchanged descriptors"
    );
    assert_eq!(counted.copy_bodies.load(Ordering::Relaxed), 0);
    counted.reset();
    assert_eq!(
        first
            .copy_family(&seeds, &seeds, limits, deadline())
            .await
            .unwrap()
            .unwrap()
            .ids,
        expected
    );
    counted.warm("historical warm copy family");
    counted.reset();
    assert_eq!(
        second
            .copy_family(&seeds, &seeds, limits, deadline())
            .await
            .unwrap()
            .unwrap()
            .ids,
        expected
    );
    counted.warm("operation-only warm copy family");
    counted.reset();
    assert_eq!(
        third
            .copy_family(&seeds, &seeds, limits, deadline())
            .await
            .unwrap()
            .unwrap()
            .ids,
        extended
    );
    counted.warm("current warm copy family");
    for bounds in [
        LookupLimits {
            max_nodes: 2,
            ..limits
        },
        LookupLimits {
            max_work: 1,
            ..limits
        },
        LookupLimits {
            max_bytes: 1,
            ..limits
        },
    ] {
        assert!(matches!(
            third.copy_family(&seeds, &seeds, bounds, deadline()).await,
            Err(q::Error::Limit)
        ));
    }
    assert!(matches!(
        third
            .copy_family(&seeds, &seeds, limits, tokio::time::Instant::now())
            .await,
        Err(q::Error::Limit)
    ));
    f.revoke_reader().await;
    assert!(matches!(
        first.copy_family(&seeds, &seeds, limits, deadline()).await,
        Err(q::Error::Denied)
    ));
    f.stop().await;
}

impl CountedInventory {
    fn new(inner: Arc<MemoryStore>) -> Arc<Self> {
        Arc::new(Self {
            inner,
            reads: AtomicUsize::new(0),
            descriptors: AtomicUsize::new(0),
            copy_bodies: AtomicUsize::new(0),
        })
    }
    fn reset(&self) {
        self.reads.store(0, Ordering::Relaxed);
        self.descriptors.store(0, Ordering::Relaxed);
        self.copy_bodies.store(0, Ordering::Relaxed);
    }
    fn warm(&self, label: &str) {
        let reads = self.reads.load(Ordering::Relaxed);
        let descriptors = self.descriptors.load(Ordering::Relaxed);
        eprintln!("{label}: {reads} reads, {descriptors} descriptors");
        assert_eq!(
            descriptors, 0,
            "warm kind lookup read unrelated descriptors"
        );
        assert!(
            reads <= 16,
            "warm kind lookup must retain fresh authorization only"
        );
    }
}
#[async_trait::async_trait]
impl jj_store_object::ObjectStore for CountedInventory {
    async fn head(&self, key: &Key) -> jj_store_object::Result<jj_store_object::Metadata> {
        self.inner.head(key).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        self.inner.put(key, condition, upload).await
    }
    async fn get(
        &self,
        key: &Key,
        range: Option<jj_store_object::ByteRange>,
    ) -> jj_store_object::Result<jj_store_object::Download> {
        self.reads.fetch_add(1, Ordering::Relaxed);
        if key.as_str().contains("/inventories/") {
            self.descriptors.fetch_add(1, Ordering::Relaxed);
        }
        if key.as_str().contains("/objects/v1/5/") {
            self.copy_bodies.fetch_add(1, Ordering::Relaxed);
        }
        self.inner.get(key, range).await
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn in_process_kind_inventory_reuses_exact_roots_and_advances_only_additions() {
    use jj_store_jj::{backend::BackendAuthority, operation_stores::OperationAuthority};
    let mut f = Harness::new().await;
    retain_files(&f, 0, 256).await;
    let counted = CountedInventory::new(f.store.clone());
    let session = NativeSession::new(
        counted.clone(),
        f.objects.clone(),
        f.reader.clone(),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
    )
    .unwrap();
    let first = session.operation_ids(8).await.unwrap();
    assert_eq!(first, BTreeSet::from([roots::ROOT_OPERATION]));
    assert!(counted.descriptors.load(Ordering::Relaxed) > 256);
    counted.reset();
    assert_eq!(session.operation_ids(8).await.unwrap(), first);
    counted.warm("native warm kind inventory");
    assert!(session.copy_ids(8).await.unwrap().is_empty());
    counted.reset();
    assert!(session.copy_ids(8).await.unwrap().is_empty());
    counted.warm("native warm empty copy inventory");
    let later = publish_operation(&f, "kind projection increment").await;
    retain_files(&f, 256, 4).await;
    counted.reset();
    let expected = BTreeSet::from([roots::ROOT_OPERATION, later]);
    assert_eq!(session.operation_ids(8).await.unwrap(), expected);
    let descriptors = counted.descriptors.load(Ordering::Relaxed);
    let reads = counted.reads.load(Ordering::Relaxed);
    eprintln!("native incremental kind inventory: {reads} reads, {descriptors} descriptors");
    assert_eq!(
        descriptors, 5,
        "increment must hydrate only selected additions"
    );
    assert!(reads <= 80, "increment scanned unchanged catalog paths");
    assert!(session.operation_ids(1).await.is_err());
    counted.reset();
    assert_eq!(session.operation_ids(8).await.unwrap(), expected);
    counted.warm("native kind inventory after tighter result limit");
    let cold_objects = Arc::new(NativeObjects::new(f.store.clone(), REPO, MAX_BLOB).unwrap());
    let cold = NativeSession::new(
        counted.clone(),
        cold_objects,
        f.reader.clone(),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
    )
    .unwrap();
    counted.reset();
    assert_eq!(cold.operation_ids(8).await.unwrap(), expected);
    assert!(
        counted.descriptors.load(Ordering::Relaxed) > 256,
        "different source capability must rebuild"
    );
    f.revoke_reader().await;
    assert!(session.operation_ids(8).await.is_err());
    f.stop().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rpc_complete_kind_inventory_reuses_exact_roots_and_advances_only_additions() {
    let mut f = Harness::new().await;
    retain_files(&f, 0, 256).await;
    f.stop().await;
    let counted = CountedInventory::new(f.store.clone());
    f.start_on_store(counted.clone(), Protection::default(), RpcLimits::default())
        .await;
    let client = f.client(&f.reader).await;
    assert_eq!(
        client.inventory(false, 8).await.unwrap(),
        vec![*roots::ROOT_OPERATION.as_bytes()]
    );
    counted.reset();
    assert_eq!(
        client.inventory(false, 8).await.unwrap(),
        vec![*roots::ROOT_OPERATION.as_bytes()]
    );
    counted.warm("RPC warm complete kind inventory");
    let later = publish_operation(&f, "RPC kind projection increment").await;
    retain_files(&f, 256, 4).await;
    let mut expected = vec![*roots::ROOT_OPERATION.as_bytes(), *later.as_bytes()];
    expected.sort();
    counted.reset();
    assert_eq!(client.inventory(false, 8).await.unwrap(), expected);
    let descriptors = counted.descriptors.load(Ordering::Relaxed);
    let reads = counted.reads.load(Ordering::Relaxed);
    eprintln!("RPC incremental complete kind inventory: {reads} reads, {descriptors} descriptors");
    assert_eq!(descriptors, 5);
    assert!(reads <= 80);
    assert_eq!(
        client.inventory(false, 1).await.unwrap_err().code(),
        Code::ResourceExhausted
    );
    f.revoke_reader().await;
    assert_eq!(
        client.inventory(false, 8).await.unwrap_err().code(),
        Code::PermissionDenied
    );
    f.stop().await;
}

// Test-only selection of verified descriptors isolates enumeration from code
// publication policy. Each file is real checked native content. No production
// transport accepts a manifest, catalog root or caller-authored descriptor.
async fn retain_files(f: &Harness, start: u64, count: u64) {
    let mut keys = Vec::new();
    for n in start..start + count {
        keys.push(
            f.objects
                .put(&NativeFile(n.to_be_bytes().to_vec()))
                .await
                .unwrap()
                .0,
        );
    }
    select_keys(f, keys).await;
}
async fn select_keys(f: &Harness, keys: Vec<m::ObjectKey>) {
    let base = f.local().snapshot().await.unwrap();
    let mut manifest = base.manifest().clone();
    let records = Records::new(f.store.clone(), REPO);
    let catalog = Catalog::new(records.clone());
    let mut entries = BTreeMap::new();
    for key in keys {
        let node = f.objects.load_verified(key, MAX_BLOB).await.unwrap();
        let reference = records
            .put(RecordKind::Inventory, None, &node)
            .await
            .unwrap();
        let mut hash = Sha256::new_with_prefix(b"jj-store/object/v1");
        hash.update([key.kind() as u8]);
        hash.update(key.as_bytes());
        entries.insert(m::RecordDigest::new(hash.finalize().into()), reference);
    }
    manifest.inventory = catalog.update(manifest.inventory, entries).await.unwrap();
    manifest.revision += 1;
    let bytes = serde_json::to_vec(&manifest).unwrap();
    let upload = StagedUpload::from_bytes(&bytes, 128 * 1024).await.unwrap();
    let path = Key::new(format!("repos/{REPO}/manifest.json")).unwrap();
    let version = f.store.head(&path).await.unwrap().version;
    assert!(matches!(
        f.store
            .put(&path, Condition::Matches(version), &upload)
            .await
            .unwrap(),
        WriteOutcome::Applied(_)
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rpc_inventory_collects_only_requested_ids_when_other_objects_exceed_the_budget() {
    let mut f = Harness::new().await;
    retain_files(&f, 0, 96).await;
    f.stop().await;
    f.start_with_session_limits(
        f.store.clone(),
        Protection::default(),
        RpcLimits::default(),
        SessionLimits {
            inventory_nodes: 32,
            ..SessionLimits::default()
        },
    )
    .await;
    let client = f.client(&f.reader).await;
    assert_eq!(
        client.inventory(false, 16).await.unwrap(),
        vec![*roots::ROOT_OPERATION.as_bytes()]
    );
    assert!(client.inventory(true, 16).await.unwrap().is_empty());
    f.stop().await;
}

async fn publish_operation(f: &Harness, description: &str) -> m::OperationId {
    let mut operation = roots::operation();
    operation.parents = vec![o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())];
    operation.metadata.description = description.into();
    let (m::ObjectKey::Operation(id), _) = f.objects.put(&operation).await.unwrap() else {
        panic!("operation kind")
    };
    let session = f.local();
    let before = session.snapshot().await.unwrap();
    let request = session
        .transition_request(before.manifest().heads.clone(), id)
        .await
        .unwrap();
    assert!(matches!(
        session.publish_request(&request).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    id
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rpc_inventory_pages_pin_selection_and_reauthorize_sparse_windows() {
    let mut f = Harness::new().await;
    retain_files(&f, 0, 96).await;
    let base = f.local().snapshot().await.unwrap();
    let expected_count = base.inventory(f.store.clone(), 1000).await.unwrap().len();
    let client = f.client(&f.reader).await;
    let first = client.inventory_page(false, 16, None).await.unwrap();
    let first_cursor = first.next_cursor.clone().unwrap();
    let mut cursor = first.next_cursor;
    let snapshot = first.snapshot;
    let mut scanned = first.scanned;
    let mut ids = first.ids;
    let mut sparse = ids.is_empty();
    let later = publish_operation(&f, "after the inventory selection").await;
    retain_files(&f, 96, 32).await;
    while let Some(after) = cursor {
        let page = client
            .inventory_page(false, 16, Some(&after))
            .await
            .unwrap();
        assert_eq!(page.snapshot, snapshot);
        assert!(page.scanned <= 16);
        sparse |= page.ids.is_empty() && page.next_cursor.is_some();
        scanned += page.scanned;
        ids.extend(page.ids);
        cursor = page.next_cursor;
    }
    assert_eq!(scanned, expected_count);
    assert_eq!(ids, vec![*roots::ROOT_OPERATION.as_bytes()]);
    assert!(
        sparse,
        "empty kind-filter windows must preserve their continuation"
    );
    let mut expected = vec![*roots::ROOT_OPERATION.as_bytes(), *later.as_bytes()];
    expected.sort();
    assert_eq!(client.inventory(false, 16).await.unwrap(), expected);
    assert_eq!(
        client.inventory(false, 1).await.unwrap_err().code(),
        Code::ResourceExhausted
    );
    // Old tokens remain valid on replay, without adopting the later catalog.
    assert_eq!(
        client
            .inventory_page(false, 16, Some(&first_cursor))
            .await
            .unwrap()
            .snapshot,
        snapshot
    );
    for (copies, size) in [(true, 16), (false, 17)] {
        assert_eq!(
            client
                .inventory_page(copies, size, Some(&first_cursor))
                .await
                .unwrap_err()
                .code(),
            Code::InvalidArgument
        );
    }
    let mut tampered = first_cursor.clone().into_bytes();
    let pos = first_cursor.rfind('.').unwrap() + 1;
    tampered[pos] = if tampered[pos] == b'A' { b'B' } else { b'A' };
    let tampered = String::from_utf8(tampered).unwrap();
    assert_eq!(
        client
            .inventory_page(false, 16, Some(&tampered))
            .await
            .unwrap_err()
            .code(),
        Code::InvalidArgument
    );
    for size in [0, 257] {
        assert_eq!(
            client
                .inventory_page(false, size, None)
                .await
                .unwrap_err()
                .code(),
            Code::ResourceExhausted
        );
    }
    let other = NativeClient::connect(
        &f.address,
        m::RepositoryId::new([99; 16]),
        f.admin.clone(),
        Options {
            insecure_loopback: true,
            ..Options::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(
        other
            .inventory_page(false, 16, Some(&first_cursor))
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );
    f.revoke_reader().await;
    assert_eq!(
        client
            .inventory_page(false, 16, Some(&first_cursor))
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );
    assert_eq!(
        client
            .inventory_page(false, 16, None)
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );
    f.stop().await;
    f.start(Protection::default(), RpcLimits::default()).await;
    let admin = f.client(&f.admin).await;
    assert_eq!(
        admin
            .inventory_page(false, 16, Some(&first_cursor))
            .await
            .unwrap_err()
            .code(),
        Code::FailedPrecondition
    );
    f.stop().await;
}

struct CursorClock(std::sync::atomic::AtomicU64);
impl Clock for CursorClock {
    fn unix_seconds(&self) -> jj_store_service::auth::Result<u64> {
        Ok(self.0.load(std::sync::atomic::Ordering::SeqCst))
    }
}

async fn selected_copy(f: &Harness) -> m::CopyId {
    let (m::ObjectKey::Copy(id), _) = f
        .objects
        .put(&b::CopyHistory {
            current_path: jj_lib::repo_path::RepoPathBuf::from_internal_string("selected.txt")
                .unwrap(),
            parents: vec![b::CopyId::placeholder()],
            salt: vec![42],
        })
        .await
        .unwrap()
    else {
        panic!("copy kind")
    };
    select_keys(f, vec![m::ObjectKey::Copy(id)]).await;
    id
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn complete_kind_inventory_discards_nonretained_baselines_and_rejects_corrupt_additions() {
    use jj_store_jj::backend::BackendAuthority as _;
    let mut f = Harness::new().await;
    retain_files(&f, 0, 96).await;
    let copy = selected_copy(&f).await;
    let counted = CountedInventory::new(f.store.clone());
    let session = NativeSession::new(
        counted.clone(),
        f.objects.clone(),
        f.reader.clone(),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
    )
    .unwrap();
    assert_eq!(session.copy_ids(8).await.unwrap(), BTreeSet::from([copy]));
    let records = Records::new(f.store.clone(), REPO);
    let catalog = Catalog::new(records.clone());
    let base = f.local().snapshot().await.unwrap();
    let mut entries = catalog
        .entries(base.manifest().inventory, 1024)
        .await
        .unwrap();
    let mut hash = Sha256::new_with_prefix(b"jj-store/object/v1");
    hash.update([m::ObjectKey::Copy(copy).kind() as u8]);
    hash.update(copy.as_bytes());
    let digest = m::RecordDigest::new(hash.finalize().into());
    assert!(entries.remove(&digest).is_some());
    let rebuilt = catalog.update(None, entries).await.unwrap();
    // Test-only selections isolate the read adapter's baseline handling. Every
    // retained descriptor is verified native content; no public writer accepts
    // this manifest/root replacement or bypasses historical retention policy.
    let mut manifest = base.manifest().clone();
    manifest.inventory = rebuilt;
    manifest.revision += 1;
    let path = Key::new(format!("repos/{REPO}/manifest.json")).unwrap();
    let version = f.store.head(&path).await.unwrap().version;
    let upload = StagedUpload::from_bytes(&serde_json::to_vec(&manifest).unwrap(), 128 * 1024)
        .await
        .unwrap();
    f.store
        .put(&path, Condition::Matches(version), &upload)
        .await
        .unwrap();
    counted.reset();
    assert!(
        session.copy_ids(8).await.unwrap().is_empty(),
        "old copy must not survive baseline loss"
    );
    assert!(
        counted.descriptors.load(Ordering::Relaxed) > 96,
        "baseline loss requires a bounded cold scan"
    );
    counted.reset();
    assert!(session.copy_ids(8).await.unwrap().is_empty());
    counted.warm("native copy inventory after baseline loss");
    let wrong = f
        .objects
        .load_verified(m::ObjectKey::Operation(roots::ROOT_OPERATION), MAX_BLOB)
        .await
        .unwrap();
    let wrong = records
        .put(RecordKind::Inventory, None, &wrong)
        .await
        .unwrap();
    manifest.inventory = catalog
        .update(rebuilt, BTreeMap::from([(digest, wrong)]))
        .await
        .unwrap();
    manifest.revision += 1;
    let version = f.store.head(&path).await.unwrap().version;
    let upload = StagedUpload::from_bytes(&serde_json::to_vec(&manifest).unwrap(), 128 * 1024)
        .await
        .unwrap();
    f.store
        .put(&path, Condition::Matches(version), &upload)
        .await
        .unwrap();
    for _ in 0..2 {
        let error = session.copy_ids(8).await.unwrap_err();
        assert!(matches!(
            error.downcast_ref::<jj_store_wal::Error>(),
            Some(jj_store_wal::Error::Corrupt("inventory catalog key"))
        ));
    }
    f.stop().await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rpc_legacy_inventory_ignores_unrelated_objects() {
    let mut f = Harness::new().await;
    retain_files(&f, 0, 96).await;
    let copy = selected_copy(&f).await;
    f.stop().await;
    f.start_with_session_limits(
        f.store.clone(),
        Protection::default(),
        RpcLimits::default(),
        SessionLimits {
            inventory_nodes: 32,
            ..SessionLimits::default()
        },
    )
    .await;
    let mut raw = f.raw().await;
    for (copies, expected) in [
        (false, roots::ROOT_OPERATION.as_bytes()),
        (true, copy.as_bytes()),
    ] {
        let ids = raw
            .inventory(req(
                &f.reader,
                wire::InventoryRequest {
                    repository: REPO.as_bytes().to_vec(),
                    copies,
                    limit: 16,
                },
            ))
            .await
            .unwrap()
            .into_inner()
            .ids;
        assert_eq!(ids, vec![expected.to_vec()]);
    }
    f.stop().await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_kind_inventory_ignores_unrelated_objects() {
    use jj_store_jj::{backend::BackendAuthority as _, operation_stores::OperationAuthority as _};
    let mut f = Harness::new().await;
    retain_files(&f, 0, 96).await;
    let copy = selected_copy(&f).await;
    let session = NativeSession::new(
        f.store.clone(),
        f.objects.clone(),
        f.reader.clone(),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits {
            inventory_nodes: 32,
            ..SessionLimits::default()
        },
    )
    .unwrap();
    assert_eq!(
        session.operation_ids(16).await.unwrap(),
        BTreeSet::from([roots::ROOT_OPERATION])
    );
    assert_eq!(session.copy_ids(16).await.unwrap(), BTreeSet::from([copy]));
    assert!(session.operation_ids(0).await.is_err());
    assert!(session.copy_ids(0).await.is_err());
    let later = publish_operation(&f, "in-process result admission").await;
    assert!(session.operation_ids(1).await.is_err());
    assert_eq!(
        session.operation_ids(16).await.unwrap(),
        BTreeSet::from([roots::ROOT_OPERATION, later])
    );
    f.revoke_reader().await;
    assert!(session.operation_ids(16).await.is_err());
    assert!(session.copy_ids(16).await.is_err());
    f.stop().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn inventory_deadlines_reject_ready_page_io_after_the_budget() {
    use jj_store_service::native_inventory::{Error, Limits as InventoryLimits, Pager};
    let mut f = Harness::new().await;
    let pager = Pager::new(
        InventoryLimits {
            timeout: Duration::from_nanos(1),
            ..InventoryLimits::default()
        },
        Arc::new(SystemClock),
    )
    .unwrap();
    assert!(matches!(
        pager.page(Arc::new(f.local()), false, 8, None).await,
        Err(Error::Limit)
    ));
    f.stop().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn inventory_deadlines_reject_ready_complete_io_after_the_budget() {
    use jj_store_jj::operation_stores::OperationAuthority as _;
    let mut f = Harness::new().await;
    let session = NativeSession::new(
        f.store.clone(),
        f.objects.clone(),
        f.reader.clone(),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits {
            timeout: Duration::from_nanos(1),
            ..SessionLimits::default()
        },
    )
    .unwrap();
    let error = session.operation_ids(16).await.unwrap_err();
    assert!(matches!(
        error.downcast_ref::<jj_store_wal::Error>(),
        Some(jj_store_wal::Error::Limit(_))
    ));
    f.stop().await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn inventory_pin_capacity_expiry_and_repository_binding_are_explicit() {
    use jj_store_service::native_inventory::{Error, Limits as InventoryLimits, Pager};
    let mut f = Harness::new().await;
    retain_files(&f, 0, 48).await;
    let clock = Arc::new(CursorClock(std::sync::atomic::AtomicU64::new(1000)));
    let pager = Pager::new(
        InventoryLimits {
            snapshots: 1,
            ttl: Duration::from_secs(10),
            ..InventoryLimits::default()
        },
        clock.clone(),
    )
    .unwrap();
    let first = pager
        .page(Arc::new(f.local()), false, 8, None)
        .await
        .unwrap();
    let cursor = first.next_cursor.unwrap();
    assert!(matches!(
        pager.page(Arc::new(f.local()), false, 8, None).await,
        Err(Error::Busy)
    ));
    clock.0.store(1009, std::sync::atomic::Ordering::SeqCst);
    assert!(
        pager
            .page(Arc::new(f.local()), false, 8, Some(&cursor))
            .await
            .is_ok()
    );
    clock.0.store(1010, std::sync::atomic::Ordering::SeqCst);
    assert!(matches!(
        pager
            .page(Arc::new(f.local()), false, 8, Some(&cursor))
            .await,
        Err(Error::CursorReset)
    ));
    // The continuation did not renew expiry; pruning admits a new selection.
    let new = pager
        .page(Arc::new(f.local()), false, 8, None)
        .await
        .unwrap()
        .next_cursor
        .unwrap();
    let base = f.local().snapshot().await.unwrap();
    let mut manifest = base.manifest().clone();
    manifest.epoch = m::EpochId::new([9; 16]);
    let path = Key::new(format!("repos/{REPO}/manifest.json")).unwrap();
    let version = f.store.head(&path).await.unwrap().version;
    let upload = StagedUpload::from_bytes(&serde_json::to_vec(&manifest).unwrap(), 128 * 1024)
        .await
        .unwrap();
    f.store
        .put(&path, Condition::Matches(version), &upload)
        .await
        .unwrap();
    assert!(matches!(
        pager.page(Arc::new(f.local()), false, 8, Some(&new)).await,
        Err(Error::EpochChanged)
    ));
    f.stop().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rpc_inventory_returns_sorted_copy_ids_and_excludes_staged_objects() {
    let mut f = Harness::new().await;
    retain_files(&f, 0, 96).await;
    let mut copies = Vec::new();
    for salt in 0..8 {
        copies.push(
            f.objects
                .put(&b::CopyHistory {
                    current_path: jj_lib::repo_path::RepoPathBuf::from_internal_string("copy.txt")
                        .unwrap(),
                    parents: vec![b::CopyId::placeholder()],
                    salt: vec![salt],
                })
                .await
                .unwrap()
                .0,
        );
    }
    select_keys(&f, copies[..7].to_vec()).await;
    f.stop().await;
    let counted = CountedInventory::new(f.store.clone());
    f.start_on_store(counted.clone(), Protection::default(), RpcLimits::default())
        .await;
    let client = f.client(&f.reader).await;
    let mut expected = copies[..7]
        .iter()
        .map(|key| *key.as_bytes())
        .collect::<Vec<_>>();
    expected.sort();
    assert_eq!(client.inventory(true, 8).await.unwrap(), expected);
    counted.reset();
    assert_eq!(client.inventory(true, 8).await.unwrap(), expected);
    counted.warm("RPC warm complete copy inventory");
    assert_eq!(
        client.inventory(true, 6).await.unwrap_err().code(),
        Code::ResourceExhausted
    );
    assert_eq!(
        client.inventory(false, 8).await.unwrap(),
        vec![*roots::ROOT_OPERATION.as_bytes()]
    );
    select_keys(&f, copies[7..].to_vec()).await;
    expected.push(*copies[7].as_bytes());
    expected.sort();
    counted.reset();
    assert_eq!(client.inventory(true, 8).await.unwrap(), expected);
    let reads = counted.reads.load(Ordering::Relaxed);
    let descriptors = counted.descriptors.load(Ordering::Relaxed);
    eprintln!("RPC incremental complete copy inventory: {reads} reads, {descriptors} descriptors");
    assert_eq!(descriptors, 1);
    assert!(reads <= 80);
    f.stop().await;
}
