use super::*;
use std::sync::atomic::{AtomicBool, AtomicUsize};

struct Compatibility {
    allowed: AtomicBool,
    deny_after: bool,
    calls: AtomicUsize,
}
impl Compatibility {
    fn new(deny_after: bool) -> Self {
        Self {
            allowed: AtomicBool::new(true),
            deny_after,
            calls: AtomicUsize::new(0),
        }
    }
    fn keys(&self) -> Arc<BTreeSet<m::ObjectKey>> {
        Arc::new(BTreeSet::from([
            m::ObjectKey::Operation(m::OperationId::new([1; 64])),
            m::ObjectKey::Operation(m::OperationId::new([2; 64])),
            m::ObjectKey::Copy(m::CopyId::new([3; 64])),
            m::ObjectKey::File(m::FileId::new([4; 64])),
        ]))
    }
}
#[async_trait::async_trait]
impl ReadAccess for Compatibility {
    fn repository(&self) -> m::RepositoryId {
        m::RepositoryId::new([1; 16])
    }
    fn epoch(&self) -> m::EpochId {
        m::EpochId::new([1; 16])
    }
    fn operation(&self) -> m::OperationId {
        m::OperationId::new([1; 64])
    }
    async fn contains(&self, key: m::ObjectKey) -> Result<bool> {
        Ok(self.keys().contains(&key))
    }
    async fn inventory(&self) -> Result<Arc<BTreeSet<m::ObjectKey>>> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        if self.deny_after {
            self.allowed.store(false, Ordering::Relaxed);
        }
        Ok(self.keys())
    }
    async fn authorize(&self) -> Result<()> {
        if self.allowed.load(Ordering::Relaxed) {
            Ok(())
        } else {
            Err(Error::Denied)
        }
    }
}

#[tokio::test]
async fn compatibility_kind_lookup_filters_with_exact_result_bounds() {
    let provider = Compatibility::new(false);
    let deadline = || Instant::now() + Duration::from_secs(1);
    assert_eq!(
        provider
            .inventory_kind(InventoryKind::Operations, 2, deadline())
            .await
            .unwrap(),
        BTreeSet::from([
            m::ObjectKey::Operation(m::OperationId::new([1; 64])),
            m::ObjectKey::Operation(m::OperationId::new([2; 64]))
        ])
    );
    assert_eq!(
        provider
            .inventory_kind(InventoryKind::Copies, 1, deadline())
            .await
            .unwrap(),
        BTreeSet::from([m::ObjectKey::Copy(m::CopyId::new([3; 64]))])
    );
    assert_eq!(
        provider
            .inventory_kind(InventoryKind::Operations, 1, deadline())
            .await
            .unwrap_err(),
        Error::Limit
    );
    let calls = provider.calls.load(Ordering::Relaxed);
    assert_eq!(
        provider
            .inventory_kind(InventoryKind::Copies, 0, deadline())
            .await
            .unwrap_err(),
        Error::Limit
    );
    assert_eq!(
        provider
            .inventory_kind(InventoryKind::Copies, 1, Instant::now())
            .await
            .unwrap_err(),
        Error::Limit
    );
    assert_eq!(provider.calls.load(Ordering::Relaxed), calls);
    let revoked = Compatibility::new(true);
    assert_eq!(
        revoked
            .inventory_kind(InventoryKind::Copies, 1, deadline())
            .await
            .unwrap_err(),
        Error::Denied
    );
}

struct KindsOnly {
    base: Compatibility,
    pause: bool,
}
#[async_trait::async_trait]
impl ReadAccess for KindsOnly {
    fn repository(&self) -> m::RepositoryId {
        self.base.repository()
    }
    fn epoch(&self) -> m::EpochId {
        self.base.epoch()
    }
    fn operation(&self) -> m::OperationId {
        self.base.operation()
    }
    async fn contains(&self, key: m::ObjectKey) -> Result<bool> {
        self.base.contains(key).await
    }
    async fn inventory(&self) -> Result<Arc<BTreeSet<m::ObjectKey>>> {
        Err(Error::IndexBudget)
    }
    async fn inventory_kind(
        &self,
        kind: InventoryKind,
        limit: usize,
        deadline: Instant,
    ) -> Result<BTreeSet<m::ObjectKey>> {
        if self.pause {
            std::future::pending::<()>().await;
        }
        // Delegate only the kind projection; the authority must never invoke
        // this provider's explicitly forbidden complete inventory method.
        self.base.inventory_kind(kind, limit, deadline).await
    }
    async fn authorize(&self) -> Result<()> {
        self.base.authorize().await
    }
}
fn authority(deny_after: bool, pause: bool) -> ReadOnlyAuthority {
    ReadOnlyAuthority {
        access: Arc::new(KindsOnly {
            base: Compatibility::new(deny_after),
            pause,
        }),
        budget: Arc::new(Budget::new(Limits::default()).unwrap()),
    }
}
#[tokio::test]
async fn upstream_authorities_use_kind_lookup_and_recheck_query_budget() {
    let read = authority(false, false);
    assert_eq!(
        read.copy_ids(1).await.unwrap(),
        BTreeSet::from([m::CopyId::new([3; 64])])
    );
    assert_eq!(
        read.operation_ids(2).await.unwrap(),
        BTreeSet::from([m::OperationId::new([1; 64]), m::OperationId::new([2; 64])])
    );
    let tight = authority(false, false);
    assert!(tight.operation_ids(1).await.is_err());
    assert_eq!(tight.budget.failure(), Some(Error::Limit));
    let revoked = authority(true, false);
    assert!(revoked.copy_ids(1).await.is_err());
    assert_eq!(revoked.budget.failure(), Some(Error::Denied));
}

#[tokio::test]
async fn historical_operation_wrapper_preserves_outer_kind_capability() {
    let read = authority(false, false);
    let selected = m::OperationId::new([2; 64]);
    let access = select_operation(read.access, selected, &read.budget)
        .await
        .unwrap();
    assert_eq!(access.operation(), selected);
    let selected_read = ReadOnlyAuthority {
        access,
        budget: read.budget,
    };
    assert_eq!(
        selected_read.copy_ids(1).await.unwrap(),
        BTreeSet::from([m::CopyId::new([3; 64])])
    );
    assert_eq!(selected_read.operation_ids(2).await.unwrap().len(), 2);
}
#[tokio::test]
async fn pending_kind_lookup_obeys_query_cancellation() {
    let read = authority(false, true);
    let cancellation = read.budget.cancellation.clone();
    let cancel = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(10)).await;
        cancellation.cancel();
    });
    assert!(
        tokio::time::timeout(Duration::from_secs(1), read.copy_ids(1))
            .await
            .unwrap()
            .is_err()
    );
    assert_eq!(read.budget.failure(), Some(Error::Canceled));
    cancel.await.unwrap();
}
