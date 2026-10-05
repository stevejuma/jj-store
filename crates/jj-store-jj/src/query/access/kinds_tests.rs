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

struct FamilyOnly {
    base: Compatibility,
    failure: Option<Error>,
    pause: bool,
    missing: bool,
    work: usize,
}
#[async_trait::async_trait]
impl ReadAccess for FamilyOnly {
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
        panic!("family must not enumerate inventory")
    }
    async fn authorize(&self) -> Result<()> {
        self.base.authorize().await
    }
    async fn copy_family(
        &self,
        seeds: &BTreeSet<m::CopyId>,
        _: &BTreeSet<m::CopyId>,
        _: copy_index::LookupLimits,
        _: Instant,
    ) -> Result<Option<copy_index::Family>> {
        if self.pause {
            std::future::pending::<()>().await;
        }
        if let Some(error) = &self.failure {
            return Err(error.clone());
        }
        if self.base.deny_after {
            self.base.allowed.store(false, Ordering::Relaxed);
        }
        Ok(Some(copy_index::Family {
            ids: if self.missing {
                BTreeSet::new()
            } else {
                seeds.clone()
            },
            work: self.work,
        }))
    }
}
fn family_authority(
    failure: Option<Error>,
    pause: bool,
    revoke: bool,
    missing: bool,
    work: usize,
) -> ReadOnlyAuthority {
    ReadOnlyAuthority {
        access: Arc::new(FamilyOnly {
            base: Compatibility::new(revoke),
            failure,
            pause,
            missing,
            work,
        }),
        budget: Arc::new(Budget::new(Limits::default()).unwrap()),
    }
}
#[tokio::test]
async fn copy_family_authority_preserves_pins_typed_failures_and_fresh_bounds() {
    let seeds = BTreeSet::from([m::CopyId::new([3; 64])]);
    let limits = copy_index::LookupLimits {
        max_nodes: 1,
        max_work: 10,
        max_bytes: 1024,
    };
    let deadline = || Instant::now() + Duration::from_secs(1);
    let read = family_authority(None, false, false, false, 3);
    let access = select_operation(read.access, m::OperationId::new([2; 64]), &read.budget)
        .await
        .unwrap();
    let historical = ReadOnlyAuthority {
        access,
        budget: read.budget,
    };
    assert_eq!(
        historical
            .copy_family(&seeds, &seeds, limits, deadline())
            .await
            .unwrap()
            .unwrap()
            .ids,
        seeds
    );
    assert!(historical.budget.stats().work >= 3);
    for (read, error) in [
        (
            family_authority(Some(Error::IndexBudget), false, false, false, 0),
            Error::IndexBudget,
        ),
        (
            family_authority(Some(Error::EpochChanged), false, false, false, 0),
            Error::EpochChanged,
        ),
        (
            family_authority(None, false, false, true, 0),
            Error::NotCommitted,
        ),
        (
            family_authority(None, false, false, false, 11),
            Error::Limit,
        ),
        (family_authority(None, false, true, false, 3), Error::Denied),
    ] {
        assert!(
            read.copy_family(&seeds, &seeds, limits, deadline())
                .await
                .is_err()
        );
        assert_eq!(read.budget.failure(), Some(error));
    }
    let count = family_authority(None, false, false, false, 3);
    assert!(
        count
            .copy_family(
                &seeds,
                &seeds,
                copy_index::LookupLimits {
                    max_nodes: 0,
                    ..limits
                },
                deadline()
            )
            .await
            .is_err()
    );
    assert_eq!(count.budget.failure(), Some(Error::Limit));
    let bytes = family_authority(None, false, false, false, 3);
    assert!(
        bytes
            .copy_family(
                &seeds,
                &seeds,
                copy_index::LookupLimits {
                    max_bytes: 511,
                    ..limits
                },
                deadline()
            )
            .await
            .is_err()
    );
    assert_eq!(bytes.budget.failure(), Some(Error::Limit));
}
#[tokio::test]
async fn pending_copy_family_obeys_cancellation_and_whole_query_deadline() {
    let seeds = BTreeSet::from([m::CopyId::new([3; 64])]);
    for canceled in [true, false] {
        let mut read = family_authority(None, true, false, false, 0);
        read.budget = Arc::new(
            Budget::new(Limits {
                timeout: Duration::from_millis(30),
                ..Limits::default()
            })
            .unwrap(),
        );
        let cancellation = read.budget.cancellation.clone();
        let task = tokio::spawn(async move {
            if canceled {
                tokio::time::sleep(Duration::from_millis(1)).await;
                cancellation.cancel();
            }
        });
        assert!(
            tokio::time::timeout(
                Duration::from_secs(1),
                read.copy_family(
                    &seeds,
                    &seeds,
                    copy_index::LookupLimits::default(),
                    Instant::now() + Duration::from_secs(10)
                )
            )
            .await
            .unwrap()
            .is_err()
        );
        assert_eq!(
            read.budget.failure(),
            Some(if canceled {
                Error::Canceled
            } else {
                Error::Limit
            })
        );
        task.await.unwrap();
    }
    let read = family_authority(None, true, false, false, 0);
    assert!(
        tokio::time::timeout(
            Duration::from_secs(1),
            read.copy_family(
                &seeds,
                &seeds,
                copy_index::LookupLimits::default(),
                Instant::now() + Duration::from_millis(10)
            )
        )
        .await
        .unwrap()
        .is_err()
    );
    assert_eq!(read.budget.failure(), Some(Error::Limit));
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
