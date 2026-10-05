use super::*;
use jj_store_object::{ByteRange, Condition, Download, Metadata, StagedUpload, WriteOutcome};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
struct CountedStore {
    inner: jj_store_object::memory::MemoryStore,
    reads: AtomicUsize,
    descriptors: AtomicUsize,
    native_bodies: AtomicUsize,
}
#[async_trait::async_trait]
impl ObjectStore for CountedStore {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.inner.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        self.reads.fetch_add(1, Ordering::Relaxed);
        if key.as_str().contains("/inventories/") {
            self.descriptors.fetch_add(1, Ordering::Relaxed);
        }
        if key.as_str().contains("/objects/") {
            self.native_bodies.fetch_add(1, Ordering::Relaxed);
        }
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        self.inner.put(key, condition, upload).await
    }
}

#[tokio::test]
async fn inventory_additions_seek_changed_paths_and_skip_unchanged_descriptors() {
    let store = Arc::new(CountedStore::default());
    let records = Records::new(store.clone(), REPO);
    let catalog = Catalog::new(records.clone());
    let mut entries = BTreeMap::new();
    for n in 0..4096 {
        entries.insert(
            object_key(key(n)),
            put_node(&records, key(n), BTreeSet::new()).await,
        );
    }
    let before = catalog.update(None, entries.clone()).await.unwrap();
    let additions = (9000..9003)
        .map(|n| (object_key(key(n)), key(n)))
        .collect::<BTreeMap<_, _>>();
    let mut changes = BTreeMap::new();
    for (digest, key) in &additions {
        changes.insert(*digest, put_node(&records, *key, BTreeSet::new()).await);
    }
    let after = catalog.update(before, changes).await.unwrap();
    store.reads.store(0, Ordering::Relaxed);
    store.descriptors.store(0, Ordering::Relaxed);
    let mut anchor = None;
    let mut actual = Vec::new();
    loop {
        let page = inventory_additions_page(store.clone(), REPO, before, after, anchor, 2)
            .await
            .unwrap();
        actual.extend(page.keys);
        anchor = page.next;
        if anchor.is_none() {
            break;
        }
    }
    assert_eq!(actual, additions.into_values().collect::<Vec<_>>());
    let reads = store.reads.load(Ordering::Relaxed);
    let descriptors = store.descriptors.load(Ordering::Relaxed);
    eprintln!("inventory additions: {reads} reads, {descriptors} descriptors");
    assert!(
        reads <= 128,
        "addition lookup scanned unrelated catalog paths"
    );
    // The first page's lookahead is catalog-only. Descriptors load once.
    assert_eq!(descriptors, 3, "unchanged descriptors must not be hydrated");
    assert_eq!(catalog.entries(before, 4096).await.unwrap(), entries);
}
const REPO: RepositoryId = RepositoryId::new([1; 16]);
fn key(n: u64) -> ObjectKey {
    let mut bytes = [0; 64];
    bytes[..8].copy_from_slice(&n.to_be_bytes());
    ObjectKey::Commit(jj_store_model::CommitId::new(bytes))
}
async fn put_node(
    records: &Records,
    key: ObjectKey,
    dependencies: BTreeSet<ObjectKey>,
) -> RecordRef {
    records
        .put(
            RecordKind::Inventory,
            None,
            &VerifiedNode {
                key,
                representation: Representation::Virtual,
                dependencies,
            },
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn projection_pages_read_copy_parent_edges_once_under_unrelated_inventory_pressure() {
    let store = Arc::new(CountedStore::default());
    let records = Records::new(store.clone(), REPO);
    let catalog = Catalog::new(records.clone());
    let copies = (1..=4)
        .map(|n| jj_store_model::CopyId::new([n; 64]))
        .collect::<Vec<_>>();
    let parents = [
        vec![],
        vec![copies[0]],
        vec![copies[0]],
        vec![copies[1], copies[2]],
    ];
    let operations = (1..=3)
        .map(|n| OperationId::new([n; 64]))
        .collect::<Vec<_>>();
    let mut entries = BTreeMap::new();
    for n in 0..160 {
        entries.insert(
            object_key(key(n)),
            put_node(&records, key(n), BTreeSet::new()).await,
        );
    }
    for id in &operations {
        let key = ObjectKey::Operation(*id);
        entries.insert(
            object_key(key),
            put_node(&records, key, BTreeSet::new()).await,
        );
    }
    for (id, parents) in copies.iter().zip(&parents) {
        let key = ObjectKey::Copy(*id);
        entries.insert(
            object_key(key),
            put_node(
                &records,
                key,
                parents.iter().copied().map(ObjectKey::Copy).collect(),
            )
            .await,
        );
    }
    let root = catalog.update(None, entries.clone()).await.unwrap();
    store.reads.store(0, Ordering::Relaxed);
    store.descriptors.store(0, Ordering::Relaxed);
    let mut anchor = None;
    let mut actual_operations = BTreeSet::new();
    let mut actual_copies = BTreeMap::new();
    let mut scanned = 0;
    let mut record_reads = 0;
    loop {
        let page = inventory_projection_page(
            store.clone(),
            REPO,
            root,
            anchor,
            InventoryProjectionLimits {
                descriptors: 32,
                copy_edges: 8,
            },
        )
        .await
        .unwrap();
        assert!(page.operations.len() + page.copies.len() <= page.scanned);
        assert!(page.scanned <= 32);
        assert!(page.record_reads <= 1024 && page.record_bytes <= 32 * 1024 * 1024);
        actual_operations.extend(page.operations);
        for copy in page.copies {
            assert!(actual_copies.insert(copy.id, copy.parents).is_none());
        }
        scanned += page.scanned;
        record_reads += page.record_reads;
        if let Some(next) = page.next {
            assert!(anchor.is_none_or(|old| old < next));
            anchor = Some(next);
        } else {
            break;
        }
    }
    assert_eq!(scanned, 167);
    assert_eq!(actual_operations, operations.into_iter().collect());
    assert_eq!(
        actual_copies,
        copies
            .into_iter()
            .zip(
                parents
                    .into_iter()
                    .map(|parents| parents.into_iter().collect())
            )
            .collect()
    );
    assert_eq!(
        store.descriptors.load(Ordering::Relaxed),
        167,
        "projection must retain edges at first descriptor read"
    );
    assert_eq!(record_reads, store.reads.load(Ordering::Relaxed));
    assert_eq!(store.native_bodies.load(Ordering::Relaxed), 0);
}

#[tokio::test]
async fn projection_edge_admission_continues_without_losing_a_copy_or_certifying_a_prefix() {
    let store = Arc::new(CountedStore::default());
    let records = Records::new(store.clone(), REPO);
    let catalog = Catalog::new(records.clone());
    let parents = (1..=3)
        .map(|n| jj_store_model::CopyId::new([n; 64]))
        .collect::<BTreeSet<_>>();
    let children = (4..=6)
        .map(|n| jj_store_model::CopyId::new([n; 64]))
        .collect::<BTreeSet<_>>();
    let mut entries = BTreeMap::new();
    for id in parents.iter().chain(&children) {
        let key = ObjectKey::Copy(*id);
        let dependencies = if children.contains(id) {
            parents.iter().copied().map(ObjectKey::Copy).collect()
        } else {
            BTreeSet::new()
        };
        entries.insert(object_key(key), put_node(&records, key, dependencies).await);
    }
    let root = catalog.update(None, entries.clone()).await.unwrap();
    let mut anchor = None;
    let mut seen = BTreeSet::new();
    let mut scanned = 0;
    let mut windows = 0;
    loop {
        let page = inventory_projection_page(
            store.clone(),
            REPO,
            root,
            anchor,
            InventoryProjectionLimits {
                descriptors: 256,
                copy_edges: 6,
            },
        )
        .await
        .unwrap();
        assert!(page.scanned != 0);
        assert!(
            page.copies
                .iter()
                .map(|copy| copy.parents.len())
                .sum::<usize>()
                <= 6
        );
        for copy in &page.copies {
            assert!(seen.insert(copy.id));
            assert_eq!(
                copy.parents,
                if children.contains(&copy.id) {
                    parents.clone()
                } else {
                    BTreeSet::new()
                }
            );
        }
        scanned += page.scanned;
        windows += 1;
        if let Some(next) = page.next {
            assert!(anchor.is_none_or(|old| old < next));
            anchor = Some(next);
        } else {
            break;
        }
    }
    assert_eq!(windows, 2);
    assert_eq!(scanned, 6);
    assert_eq!(seen, parents.union(&children).copied().collect());
    let key = ObjectKey::Copy(*children.first().unwrap());
    let only = catalog
        .update(
            None,
            BTreeMap::from([(
                object_key(key),
                put_node(
                    &records,
                    key,
                    parents.iter().copied().map(ObjectKey::Copy).collect(),
                )
                .await,
            )]),
        )
        .await
        .unwrap();
    assert!(matches!(
        inventory_projection_page(
            store,
            REPO,
            only,
            None,
            InventoryProjectionLimits {
                descriptors: 256,
                copy_edges: 2
            }
        )
        .await,
        Err(Error::Limit("inventory projection edges"))
    ));
}

#[tokio::test]
async fn projection_additions_retain_copy_edges_without_reading_unchanged_descriptors() {
    let store = Arc::new(CountedStore::default());
    let records = Records::new(store.clone(), REPO);
    let catalog = Catalog::new(records.clone());
    let parent = jj_store_model::CopyId::new([1; 64]);
    let child = jj_store_model::CopyId::new([2; 64]);
    let operation = OperationId::new([10; 64]);
    let mut entries = BTreeMap::new();
    for n in 0..1024 {
        entries.insert(
            object_key(key(n)),
            put_node(&records, key(n), BTreeSet::new()).await,
        );
    }
    let copy = ObjectKey::Copy(parent);
    entries.insert(
        object_key(copy),
        put_node(&records, copy, BTreeSet::new()).await,
    );
    let before = catalog.update(None, entries).await.unwrap();
    let mut additions = BTreeMap::new();
    for n in 2000..2003 {
        additions.insert(
            object_key(key(n)),
            put_node(&records, key(n), BTreeSet::new()).await,
        );
    }
    let copy = ObjectKey::Copy(child);
    additions.insert(
        object_key(copy),
        put_node(&records, copy, BTreeSet::from([ObjectKey::Copy(parent)])).await,
    );
    let key = ObjectKey::Operation(operation);
    additions.insert(
        object_key(key),
        put_node(&records, key, BTreeSet::new()).await,
    );
    let after = catalog.update(before, additions).await.unwrap();
    store.descriptors.store(0, Ordering::Relaxed);
    let mut anchor = None;
    let mut scanned = 0;
    let mut operations = Vec::new();
    let mut copies = Vec::new();
    loop {
        let page = inventory_projection_additions_page(
            store.clone(),
            REPO,
            before,
            after,
            anchor,
            InventoryProjectionLimits {
                descriptors: 2,
                copy_edges: 1,
            },
        )
        .await
        .unwrap();
        scanned += page.scanned;
        operations.extend(page.operations);
        copies.extend(page.copies);
        if let Some(next) = page.next {
            assert!(anchor.is_none_or(|old| old < next));
            anchor = Some(next);
        } else {
            break;
        }
    }
    assert_eq!(scanned, 5);
    assert_eq!(operations, vec![operation]);
    assert_eq!(
        copies,
        vec![CopyParents {
            id: child,
            parents: BTreeSet::from([parent])
        }]
    );
    assert_eq!(store.descriptors.load(Ordering::Relaxed), 5);
    let empty = inventory_projection_additions_page(
        store.clone(),
        REPO,
        after,
        after,
        None,
        InventoryProjectionLimits::default(),
    )
    .await
    .unwrap();
    assert_eq!(empty.scanned, 0);
    assert!(empty.next.is_none());
    assert_eq!(store.descriptors.load(Ordering::Relaxed), 5);
    assert!(matches!(
        inventory_projection_additions_page(
            store,
            REPO,
            after,
            before,
            None,
            InventoryProjectionLimits::default()
        )
        .await,
        Err(Error::Corrupt("catalog base not retained"))
    ));
}

#[tokio::test]
async fn projection_pages_reject_invalid_shapes_limits_and_descriptor_identity() {
    let store = Arc::new(CountedStore::default());
    let records = Records::new(store.clone(), REPO);
    let catalog = Catalog::new(records.clone());
    let copy = ObjectKey::Copy(jj_store_model::CopyId::new([1; 64]));
    let root = catalog
        .update(
            None,
            BTreeMap::from([(
                object_key(copy),
                put_node(&records, copy, BTreeSet::from([key(1)])).await,
            )]),
        )
        .await
        .unwrap();
    assert!(matches!(
        inventory_projection_page(
            store.clone(),
            REPO,
            root,
            None,
            InventoryProjectionLimits::default()
        )
        .await,
        Err(Error::Corrupt("copy inventory dependency kind"))
    ));
    let wrong = catalog
        .update(
            None,
            BTreeMap::from([(
                object_key(copy),
                put_node(&records, key(1), BTreeSet::new()).await,
            )]),
        )
        .await
        .unwrap();
    assert!(matches!(
        inventory_projection_page(
            store.clone(),
            REPO,
            wrong,
            None,
            InventoryProjectionLimits::default()
        )
        .await,
        Err(Error::Corrupt("inventory catalog key"))
    ));
    for limits in [
        InventoryProjectionLimits {
            descriptors: 0,
            copy_edges: 1,
        },
        InventoryProjectionLimits {
            descriptors: 257,
            copy_edges: 1,
        },
        InventoryProjectionLimits {
            descriptors: 1,
            copy_edges: 0,
        },
        InventoryProjectionLimits {
            descriptors: 1,
            copy_edges: 65_537,
        },
    ] {
        store.reads.store(0, Ordering::Relaxed);
        assert!(matches!(
            inventory_projection_page(store.clone(), REPO, root, None, limits).await,
            Err(Error::Limit("inventory projection window"))
        ));
        assert_eq!(store.reads.load(Ordering::Relaxed), 0);
    }
    assert!(matches!(
        inventory_projection_page(
            store,
            REPO,
            None,
            None,
            InventoryProjectionLimits::default()
        )
        .await,
        Err(Error::Corrupt("missing committed inventory"))
    ));
}

#[tokio::test]
async fn point_membership_reads_radix_paths_instead_of_the_inventory() {
    let store = Arc::new(CountedStore::default());
    let records = Records::new(store.clone(), REPO);
    let catalog = Catalog::new(records.clone());
    let mut entries = BTreeMap::new();
    for n in 0..4096 {
        entries.insert(
            object_key(key(n)),
            put_node(&records, key(n), BTreeSet::new()).await,
        );
    }
    let root = catalog.update(None, entries).await.unwrap();
    store.reads.store(0, Ordering::Relaxed);
    let node = inventory_member(store.clone(), REPO, root, key(123))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(node.key, key(123));
    assert!(
        store.reads.load(Ordering::Relaxed) <= 66,
        "lookup exceeded radix depth plus descriptor"
    );
    assert!(
        inventory_member(store.clone(), REPO, root, key(9000))
            .await
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn point_membership_checks_traversed_dependencies_and_descriptor_keys() {
    let store = Arc::new(CountedStore::default());
    let records = Records::new(store.clone(), REPO);
    let catalog = Catalog::new(records.clone());
    let reference = put_node(&records, key(1), BTreeSet::from([key(2)])).await;
    let missing = catalog
        .update(None, BTreeMap::from([(object_key(key(1)), reference)]))
        .await
        .unwrap();
    let node = inventory_member(store.clone(), REPO, missing, key(1))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(node.dependencies, BTreeSet::from([key(2)]));
    // Following the dependency must prove its own selected membership. Merely
    // naming it in an object/descriptor never grants access to a staged object.
    assert!(
        inventory_member(store.clone(), REPO, missing, key(2))
            .await
            .unwrap()
            .is_none()
    );
    let complete = catalog
        .update(
            missing,
            BTreeMap::from([(
                object_key(key(2)),
                put_node(&records, key(2), BTreeSet::new()).await,
            )]),
        )
        .await
        .unwrap();
    assert!(
        inventory_member(store.clone(), REPO, complete, key(2))
            .await
            .unwrap()
            .is_some()
    );
    assert!(
        inventory_member(store.clone(), REPO, missing, key(2))
            .await
            .unwrap()
            .is_none()
    );
    assert!(matches!(
        inventory(store.clone(), REPO, missing, &BTreeSet::new(), 100).await,
        Err(Error::Corrupt("incomplete committed inventory"))
    ));
    let wrong = catalog
        .update(None, BTreeMap::from([(object_key(key(3)), reference)]))
        .await
        .unwrap();
    assert!(matches!(
        inventory_member(store, REPO, wrong, key(3)).await,
        Err(Error::Corrupt("inventory catalog key"))
    ));
}

#[tokio::test]
async fn inventory_pages_seek_deep_anchors_and_keep_old_roots() {
    let store = Arc::new(CountedStore::default());
    let records = Records::new(store.clone(), REPO);
    let catalog = Catalog::new(records.clone());
    let mut entries = BTreeMap::new();
    for n in 0..4096 {
        entries.insert(
            object_key(key(n)),
            put_node(&records, key(n), BTreeSet::new()).await,
        );
    }
    let root = catalog.update(None, entries.clone()).await.unwrap();
    let expected = entries.keys().copied().collect::<Vec<_>>();
    let anchor = expected[4000];
    store.reads.store(0, Ordering::Relaxed);
    let page = inventory_page(store.clone(), REPO, root, Some(anchor), 8)
        .await
        .unwrap();
    assert_eq!(page.next, Some(expected[4008]));
    assert_eq!(
        page.keys
            .iter()
            .copied()
            .map(object_key)
            .collect::<Vec<_>>(),
        expected[4001..4009]
    );
    assert!(
        store.reads.load(Ordering::Relaxed) <= 80,
        "deep seek replayed earlier subtrees"
    );
    let updated = catalog
        .update(
            root,
            BTreeMap::from([(
                object_key(key(9000)),
                put_node(&records, key(9000), BTreeSet::new()).await,
            )]),
        )
        .await
        .unwrap();
    let mut after = None;
    let mut seen = Vec::new();
    loop {
        let page = inventory_page(store.clone(), REPO, root, after, 127)
            .await
            .unwrap();
        seen.extend(page.keys.into_iter().map(object_key));
        after = page.next;
        if after.is_none() {
            break;
        }
    }
    assert_eq!(seen, expected);
    assert!(
        catalog
            .get(updated, object_key(key(9000)))
            .await
            .unwrap()
            .is_some()
    );
    assert!(
        catalog
            .get(root, object_key(key(9000)))
            .await
            .unwrap()
            .is_none()
    );
    let wrong = catalog
        .update(
            None,
            BTreeMap::from([(
                object_key(key(9001)),
                put_node(&records, key(9002), BTreeSet::new()).await,
            )]),
        )
        .await
        .unwrap();
    assert!(matches!(
        inventory_page(store.clone(), REPO, wrong, None, 8).await,
        Err(Error::Corrupt("inventory catalog key"))
    ));
    let mut reads = crate::catalog::ScanBudget::new(1, 8 * 1024 * 1024);
    assert!(matches!(
        catalog.page(root, Some(anchor), 8, &mut reads).await,
        Err(Error::Limit(_))
    ));
    let mut bytes = crate::catalog::ScanBudget::new(1024, 1);
    assert!(catalog.page(root, None, 8, &mut bytes).await.is_err());
}

#[tokio::test]
async fn inventory_pages_continue_before_byte_admission_without_losing_descriptors() {
    let store = Arc::new(CountedStore::default());
    let records = Records::new(store.clone(), REPO);
    let catalog = Catalog::new(records.clone());
    // Structurally valid selected descriptors can approach the existing16MiB
    // record ceiling. This fixture isolates scan admission, not closure audit.
    let dependencies = (0..90_000).map(key).collect::<BTreeSet<_>>();
    let mut entries = BTreeMap::new();
    for n in 90_000..90_003 {
        entries.insert(
            object_key(key(n)),
            put_node(&records, key(n), dependencies.clone()).await,
        );
    }
    let root = catalog.update(None, entries.clone()).await.unwrap();
    let page = inventory_page(store.clone(), REPO, root, None, 256)
        .await
        .unwrap();
    assert!(!page.keys.is_empty() && page.keys.len() < 3);
    let mut actual = page.keys.into_iter().map(object_key).collect::<Vec<_>>();
    let next = page
        .next
        .expect("byte-limited window must retain a continuation");
    assert_eq!(actual.last(), Some(&next));
    let rest = inventory_page(store.clone(), REPO, root, Some(next), 256)
        .await
        .unwrap();
    actual.extend(rest.keys.into_iter().map(object_key));
    assert!(rest.next.is_none());
    assert_eq!(actual, entries.keys().copied().collect::<Vec<_>>());
    let projected = inventory_projection_page(
        store.clone(),
        REPO,
        root,
        None,
        InventoryProjectionLimits {
            descriptors: 256,
            copy_edges: 1,
        },
    )
    .await
    .unwrap();
    assert!(projected.operations.is_empty() && projected.copies.is_empty());
    assert!(projected.scanned > 0 && projected.scanned < 3);
    assert!(projected.record_bytes <= 32 * 1024 * 1024);
    let tail = inventory_projection_page(
        store.clone(),
        REPO,
        root,
        projected.next,
        InventoryProjectionLimits {
            descriptors: 256,
            copy_edges: 1,
        },
    )
    .await
    .unwrap();
    assert!(tail.next.is_none());
    assert_eq!(projected.scanned + tail.scanned, 3);
    assert!(tail.operations.is_empty() && tail.copies.is_empty());
    let before = catalog
        .update(
            None,
            BTreeMap::from([(
                object_key(key(89_999)),
                put_node(&records, key(89_999), BTreeSet::new()).await,
            )]),
        )
        .await
        .unwrap();
    let after = catalog.update(before, entries.clone()).await.unwrap();
    let page = inventory_additions_page(store.clone(), REPO, before, after, None, 256)
        .await
        .unwrap();
    assert!(!page.keys.is_empty() && page.keys.len() < 3);
    let next = page
        .next
        .expect("addition descriptor admission must preserve continuation");
    let mut actual = page.keys.into_iter().map(object_key).collect::<Vec<_>>();
    assert_eq!(actual.last(), Some(&next));
    let rest = inventory_additions_page(store, REPO, before, after, Some(next), 256)
        .await
        .unwrap();
    actual.extend(rest.keys.into_iter().map(object_key));
    assert!(rest.next.is_none());
    assert_eq!(actual, entries.into_keys().collect::<Vec<_>>());
}

#[tokio::test]
async fn catalog_work_admission_continues_after_a_verified_nonempty_prefix() {
    let store = Arc::new(CountedStore::default());
    let records = Records::new(store.clone(), REPO);
    let catalog = Catalog::new(records.clone());
    let mut entries = BTreeMap::new();
    for n in 0..33 {
        let mut digest = [0; 32];
        digest[0] = if n == 32 { 0x10 } else { 0x00 };
        digest[31] = n;
        entries.insert(
            RecordDigest::new(digest),
            put_node(&records, key(n as u64), BTreeSet::new()).await,
        );
    }
    let root = catalog.update(None, entries.clone()).await.unwrap();
    store.reads.store(0, Ordering::Relaxed);
    let mut budget = crate::catalog::ScanBudget::new(2, 1024 * 1024);
    let first = catalog.page(root, None, 256, &mut budget).await.unwrap();
    assert_eq!(first.entries.len(), 32);
    assert_eq!(store.reads.load(Ordering::Relaxed), 2);
    assert_eq!(
        first.next.as_ref(),
        first.entries.last_key_value().map(|(key, _)| key)
    );
    let mut budget = crate::catalog::ScanBudget::new(64, 1024 * 1024);
    let rest = catalog
        .page(root, first.next, 256, &mut budget)
        .await
        .unwrap();
    assert_eq!(rest.entries.len(), 1);
    assert!(rest.next.is_none());
    let mut actual = first.entries;
    actual.extend(rest.entries);
    assert_eq!(actual, entries);
}
