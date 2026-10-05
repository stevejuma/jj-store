//! Copy-family selection from admitted descriptor relations, without body I/O.
use jj_store_jj::query::{
    Error,
    copy_index::{Builder, Limits, LookupLimits},
};
use jj_store_model::CopyId;
use std::{collections::BTreeSet, time::Duration};
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
fn id(n: u64) -> CopyId {
    let mut bytes = [0; 64];
    bytes[..8].copy_from_slice(&n.to_be_bytes());
    CopyId::new(bytes)
}
fn deadline() -> Instant {
    Instant::now() + Duration::from_secs(10)
}

#[tokio::test]
async fn copy_family_includes_ancestor_descendants_without_unrelated_parent_branches() {
    let mut build = Builder::new(Limits::default()).unwrap();
    // Requested2 descends from1. A merge5 descends from1 and unrelated4;
    // required descendants include5/6, but not4 or its unrelated child7.
    for (node, parents) in [
        (1, vec![]),
        (2, vec![1]),
        (3, vec![1]),
        (4, vec![]),
        (5, vec![1, 4]),
        (6, vec![5]),
        (7, vec![4]),
    ] {
        build
            .insert(id(node), parents.into_iter().map(id).collect())
            .unwrap();
    }
    for n in 100..10_100 {
        build.insert(id(n), BTreeSet::new()).unwrap();
    }
    let cancel = CancellationToken::new();
    let index = build.finish(deadline(), &cancel).await.unwrap();
    let limits = LookupLimits {
        max_nodes: 5,
        max_work: 32,
        max_bytes: 5 * 512,
    };
    let family = index
        .family(&BTreeSet::from([id(2)]), limits, deadline(), &cancel)
        .await
        .unwrap();
    assert_eq!(family, [1, 2, 3, 5, 6].into_iter().map(id).collect());
    assert_eq!(index.node_count(), 10_007);
    assert_eq!(index.edge_count(), 6);
    assert_eq!(
        index.parents(id(5)).unwrap(),
        &BTreeSet::from([id(1), id(4)])
    );
    assert!(matches!(
        index
            .family(
                &BTreeSet::from([id(2)]),
                LookupLimits {
                    max_nodes: 4,
                    ..limits
                },
                deadline(),
                &cancel
            )
            .await,
        Err(Error::Limit)
    ));
}

async fn small_index() -> jj_store_jj::query::copy_index::Index {
    let mut build = Builder::new(Limits::default()).unwrap();
    for (node, parents) in [
        (1, vec![]),
        (2, vec![1]),
        (3, vec![1]),
        (4, vec![]),
        (5, vec![1, 4]),
        (6, vec![5]),
        (7, vec![4]),
    ] {
        build
            .insert(id(node), parents.into_iter().map(id).collect())
            .unwrap();
    }
    build
        .finish(deadline(), &CancellationToken::new())
        .await
        .unwrap()
}

#[tokio::test]
async fn copy_provider_admits_before_growth_and_baseline_clone() {
    use std::cell::Cell;
    let cancel = CancellationToken::new();
    let empty = Builder::new(Limits::default())
        .unwrap()
        .finish(deadline(), &cancel)
        .await
        .unwrap();
    for limits in [
        Limits {
            max_nodes: 0,
            ..Limits::default()
        },
        Limits {
            max_work: 0,
            ..Limits::default()
        },
        Limits {
            max_bytes: 0,
            ..Limits::default()
        },
    ] {
        assert!(matches!(
            Builder::from_index(&empty, limits, |_| panic!("invalid bounds admitted")),
            Err(Error::IndexBudget)
        ));
    }
    let mut builder = Builder::new(Limits::default()).unwrap();
    builder.insert(id(1), BTreeSet::new()).unwrap();
    let before = builder.allocated_bytes();
    let requested = Cell::new(0);
    assert!(matches!(
        builder.insert_admitted(id(2), BTreeSet::from([id(1)]), |bytes| {
            requested.set(bytes);
            Err(Error::IndexBudget)
        }),
        Err(Error::IndexBudget)
    ));
    assert!(requested.get() > before);
    assert_eq!(builder.allocated_bytes(), before);
    let index = builder.finish(deadline(), &cancel).await.unwrap();
    assert_eq!(index.node_count(), 1);
    assert!(matches!(
        Builder::from_index(&index, Limits::default(), |bytes| {
            assert_eq!(bytes, before);
            Err(Error::IndexBudget)
        }),
        Err(Error::IndexBudget)
    ));
    let cloned = Builder::from_index(&index, Limits::default(), |_| Ok(()))
        .unwrap()
        .finish(deadline(), &cancel)
        .await
        .unwrap();
    assert_eq!(cloned.parents(id(1)), index.parents(id(1)));
    let result = cloned
        .family_with_work(
            &BTreeSet::from([id(1)]),
            LookupLimits::default(),
            deadline(),
            &cancel,
        )
        .await
        .unwrap();
    assert_eq!(result.ids, BTreeSet::from([id(1)]));
    assert_eq!(
        result.work, 3,
        "seed, ancestor and descendant visits are charged"
    );
    assert!(matches!(
        cloned
            .family_with_work(
                &result.ids,
                LookupLimits {
                    max_work: result.work - 1,
                    ..LookupLimits::default()
                },
                deadline(),
                &cancel
            )
            .await,
        Err(Error::Limit)
    ));
}

#[tokio::test]
async fn copy_family_handles_reverse_leaf_and_multiple_seed_requests() {
    let index = small_index().await;
    let cancel = CancellationToken::new();
    let limits = LookupLimits::default();
    let root = index
        .family(&BTreeSet::from([id(1)]), limits, deadline(), &cancel)
        .await
        .unwrap();
    assert_eq!(root, [1, 2, 3, 5, 6].into_iter().map(id).collect());
    let leaf = index
        .family(&BTreeSet::from([id(6)]), limits, deadline(), &cancel)
        .await
        .unwrap();
    assert_eq!(leaf, (1..=7).map(id).collect());
    assert_eq!(
        index
            .family(&BTreeSet::from([id(2), id(7)]), limits, deadline(), &cancel)
            .await
            .unwrap(),
        leaf
    );
    assert!(
        index
            .family(&BTreeSet::new(), limits, deadline(), &cancel)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(matches!(
        index
            .family(&BTreeSet::from([id(99)]), limits, deadline(), &cancel)
            .await,
        Err(Error::NotCommitted)
    ));
}

#[tokio::test]
async fn copy_index_seals_only_complete_acyclic_unchanged_relations() {
    let cancel = CancellationToken::new();
    let mut missing = Builder::new(Limits::default()).unwrap();
    missing.insert(id(2), BTreeSet::from([id(1)])).unwrap();
    assert!(matches!(
        missing.finish(deadline(), &cancel).await,
        Err(Error::Incomplete)
    ));
    let mut cycle = Builder::new(Limits::default()).unwrap();
    cycle.insert(id(1), BTreeSet::from([id(2)])).unwrap();
    cycle.insert(id(2), BTreeSet::from([id(1)])).unwrap();
    assert!(matches!(
        cycle.finish(deadline(), &cancel).await,
        Err(Error::Corrupt)
    ));
    let mut self_cycle = Builder::new(Limits::default()).unwrap();
    self_cycle.insert(id(1), BTreeSet::from([id(1)])).unwrap();
    assert!(matches!(
        self_cycle.finish(deadline(), &cancel).await,
        Err(Error::Corrupt)
    ));
    let mut unchanged = Builder::new(Limits::default()).unwrap();
    unchanged.insert(id(1), BTreeSet::new()).unwrap();
    unchanged.insert(id(2), BTreeSet::from([id(1)])).unwrap();
    let allocation = unchanged.allocated_bytes();
    unchanged.insert(id(2), BTreeSet::from([id(1)])).unwrap();
    assert_eq!(unchanged.allocated_bytes(), allocation);
    assert!(matches!(
        unchanged.insert(id(2), BTreeSet::new()),
        Err(Error::Corrupt)
    ));
    let index = unchanged.finish(deadline(), &cancel).await.unwrap();
    assert_eq!(index.node_count(), 2);
    assert_eq!(index.edge_count(), 1);
    assert_eq!(index.allocated_bytes(), allocation);
}

#[tokio::test]
async fn copy_index_admits_growth_before_mutation_and_bounds_sealing_work() {
    let cancel = CancellationToken::new();
    for limits in [
        Limits {
            max_nodes: 1,
            ..Limits::default()
        },
        Limits {
            max_edges: 0,
            ..Limits::default()
        },
        Limits {
            max_bytes: 1024 * 1024 + 1024,
            ..Limits::default()
        },
    ] {
        let mut build = Builder::new(limits).unwrap();
        build.insert(id(1), BTreeSet::new()).unwrap();
        let allocated = build.allocated_bytes();
        assert!(matches!(
            build.insert(id(2), BTreeSet::from([id(1)])),
            Err(Error::IndexBudget)
        ));
        assert_eq!(build.allocated_bytes(), allocated);
        let index = build.finish(deadline(), &cancel).await.unwrap();
        assert_eq!(index.node_count(), 1);
        assert_eq!(index.edge_count(), 0);
    }
    for limits in [
        Limits {
            max_nodes: 0,
            ..Limits::default()
        },
        Limits {
            max_work: 0,
            ..Limits::default()
        },
        Limits {
            max_bytes: 1,
            ..Limits::default()
        },
    ] {
        assert!(matches!(Builder::new(limits), Err(Error::IndexBudget)));
    }
    let mut work = Builder::new(Limits {
        max_work: 1,
        ..Limits::default()
    })
    .unwrap();
    work.insert(id(1), BTreeSet::new()).unwrap();
    assert!(matches!(
        work.insert(id(2), BTreeSet::new()),
        Err(Error::IndexBudget)
    ));
    assert!(matches!(
        work.finish(deadline(), &cancel).await,
        Err(Error::Limit)
    ));
    let empty = Builder::new(Limits {
        max_edges: 0,
        ..Limits::default()
    })
    .unwrap()
    .finish(deadline(), &cancel)
    .await
    .unwrap();
    assert_eq!(empty.node_count(), 0);
    assert!(matches!(
        Builder::new(Limits::default())
            .unwrap()
            .finish(Instant::now(), &cancel)
            .await,
        Err(Error::Limit)
    ));
}

#[tokio::test]
async fn copy_family_applies_fresh_work_bytes_deadline_and_cancellation_bounds() {
    let index = small_index().await;
    let seeds = BTreeSet::from([id(2)]);
    let cancel = CancellationToken::new();
    for limits in [
        LookupLimits {
            max_work: 1,
            ..LookupLimits::default()
        },
        LookupLimits {
            max_bytes: 5 * 512 - 1,
            ..LookupLimits::default()
        },
        LookupLimits {
            max_nodes: 0,
            ..LookupLimits::default()
        },
        LookupLimits {
            max_work: 0,
            ..LookupLimits::default()
        },
    ] {
        assert!(matches!(
            index.family(&seeds, limits, deadline(), &cancel).await,
            Err(Error::Limit)
        ));
    }
    assert!(matches!(
        index
            .family(&seeds, LookupLimits::default(), Instant::now(), &cancel)
            .await,
        Err(Error::Limit)
    ));
    cancel.cancel();
    assert!(matches!(
        index
            .family(&seeds, LookupLimits::default(), deadline(), &cancel)
            .await,
        Err(Error::Canceled)
    ));
    assert!(
        index
            .family(
                &seeds,
                LookupLimits::default(),
                deadline(),
                &CancellationToken::new()
            )
            .await
            .is_ok()
    );
}

fn long_chain() -> Builder {
    let mut build = Builder::new(Limits::default()).unwrap();
    for n in 1..=10_000 {
        build
            .insert(
                id(n),
                if n == 1 {
                    BTreeSet::new()
                } else {
                    BTreeSet::from([id(n - 1)])
                },
            )
            .unwrap();
    }
    build
}

#[tokio::test]
async fn copy_index_cpu_walks_yield_for_pending_cancellation() {
    let build = long_chain();
    let cancel = CancellationToken::new();
    let trigger = cancel.clone();
    let task = tokio::spawn(async move {
        trigger.cancel();
    });
    assert!(matches!(
        build.finish(deadline(), &cancel).await,
        Err(Error::Canceled)
    ));
    task.await.unwrap();
    let index = long_chain()
        .finish(deadline(), &CancellationToken::new())
        .await
        .unwrap();
    let cancel = CancellationToken::new();
    let trigger = cancel.clone();
    let task = tokio::spawn(async move {
        trigger.cancel();
    });
    assert!(matches!(
        index
            .family(
                &BTreeSet::from([id(10_000)]),
                LookupLimits::default(),
                deadline(),
                &cancel
            )
            .await,
        Err(Error::Canceled)
    ));
    task.await.unwrap();
    let family = index
        .family(
            &BTreeSet::from([id(10_000)]),
            LookupLimits::default(),
            deadline(),
            &CancellationToken::new(),
        )
        .await
        .unwrap();
    assert_eq!(family.len(), 10_000);
}
