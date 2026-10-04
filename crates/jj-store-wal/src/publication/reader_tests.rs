use super::*;
use jj_store_object::{ByteRange, Condition, Download, Metadata, StagedUpload, WriteOutcome};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
struct CountedStore {
    inner: jj_store_object::memory::MemoryStore,
    reads: AtomicUsize,
}
#[async_trait::async_trait]
impl ObjectStore for CountedStore {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.inner.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        self.reads.fetch_add(1, Ordering::Relaxed);
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
