use jj_store_model::{CommitId, EnvelopeDigest, ObjectKey};
use jj_store_wal::inventory::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Mutex,
    time::Duration,
};
fn key(n: u8) -> ObjectKey {
    ObjectKey::Commit(CommitId::new([n; 64]))
}
fn node(n: u8, deps: &[u8]) -> VerifiedNode {
    VerifiedNode {
        key: key(n),
        representation: Representation::Stored {
            envelope_digest: EnvelopeDigest::new([n; 32]),
            bytes: 100,
        },
        dependencies: deps.iter().map(|n| key(*n)).collect(),
    }
}
struct Source {
    scope: SourceScope,
    nodes: BTreeMap<ObjectKey, VerifiedNode>,
    reads: Mutex<Vec<ObjectKey>>,
}
impl Source {
    fn new(nodes: Vec<VerifiedNode>) -> Self {
        Self {
            scope: SourceScope::new(jj_store_model::RepositoryId::new([1; 16])),
            nodes: nodes.into_iter().map(|n| (n.key, n)).collect(),
            reads: Mutex::new(Vec::new()),
        }
    }
}
#[async_trait::async_trait]
impl DependencySource for Source {
    fn scope(&self) -> SourceScope {
        self.scope
    }
    async fn load_verified(&self, k: ObjectKey, _max_bytes: u64) -> Result<VerifiedNode> {
        self.reads.lock().unwrap().push(k);
        self.nodes
            .get(&k)
            .cloned()
            .ok_or_else(|| InventoryError::Source("missing object".into()))
    }
}
#[tokio::test]
async fn complete_dag_closure_retains_history_and_reuses_verified_base() {
    let source = Source::new(vec![
        node(1, &[]),
        node(2, &[1]),
        node(3, &[1, 2]),
        node(4, &[2]),
    ]);
    let base = extend(
        &source,
        &VerifiedInventory::default(),
        [key(3)],
        Budget::default(),
    )
    .await
    .unwrap();
    assert_eq!(base.added, BTreeSet::from([key(1), key(2), key(3)]));
    assert_eq!(base.loaded_bytes, 300);
    source.reads.lock().unwrap().clear();
    let next = extend(&source, &base.complete, [key(4)], Budget::default())
        .await
        .unwrap();
    assert_eq!(next.added, BTreeSet::from([key(4)]));
    assert_eq!(next.complete.entries().len(), 4);
    assert_eq!(*source.reads.lock().unwrap(), vec![key(4)]);
    assert!(next.complete.entries().contains_key(&key(3)));
}
#[tokio::test]
async fn missing_dependencies_cycles_wrong_keys_and_budgets_fail_closed() {
    for source in [
        Source::new(vec![node(1, &[2])]),
        Source::new(vec![node(1, &[2]), node(2, &[1])]),
        Source {
            scope: SourceScope::new(jj_store_model::RepositoryId::new([1; 16])),
            nodes: BTreeMap::from([(key(1), node(2, &[]))]),
            reads: Mutex::new(Vec::new()),
        },
    ] {
        assert!(
            extend(
                &source,
                &VerifiedInventory::default(),
                [key(1)],
                Budget::default()
            )
            .await
            .is_err()
        );
    }
    let source = Source::new(vec![node(1, &[2]), node(2, &[])]);
    for budget in [
        Budget {
            max_nodes: 1,
            ..Budget::default()
        },
        Budget {
            max_edges: 0,
            ..Budget::default()
        },
        Budget {
            max_new_bytes: 199,
            ..Budget::default()
        },
        Budget {
            timeout: Duration::ZERO,
            ..Budget::default()
        },
    ] {
        assert!(matches!(
            extend(&source, &VerifiedInventory::default(), [key(1)], budget).await,
            Err(InventoryError::Limit(_))
        ));
    }
}
#[tokio::test]
async fn deadline_cancels_a_stalled_source() {
    struct Stalled(SourceScope);
    #[async_trait::async_trait]
    impl DependencySource for Stalled {
        fn scope(&self) -> SourceScope {
            self.0
        }
        async fn load_verified(&self, _: ObjectKey, _: u64) -> Result<VerifiedNode> {
            std::future::pending().await
        }
    }
    assert!(matches!(
        extend(
            &Stalled(SourceScope::new(jj_store_model::RepositoryId::new([1; 16]))),
            &VerifiedInventory::default(),
            [key(1)],
            Budget {
                timeout: Duration::from_millis(10),
                ..Budget::default()
            }
        )
        .await,
        Err(InventoryError::Limit("time"))
    ));
}

#[tokio::test]
async fn verified_bases_cannot_cross_storage_sources_even_with_same_repository_id() {
    let first = Source::new(vec![node(1, &[])]);
    let other = Source::new(vec![]);
    let base = extend(
        &first,
        &VerifiedInventory::default(),
        [key(1)],
        Budget::default(),
    )
    .await
    .unwrap();
    assert!(matches!(
        extend(&other, &base.complete, [key(1)], Budget::default()).await,
        Err(InventoryError::Scope)
    ));
}
