use super::*;
use crate::local_browser::Limits;

fn encoded(parents: &[Vec<u32>]) -> Vec<u8> {
    let mut bytes = vec![];
    let mut word = |n: u32| bytes.extend(n.to_le_bytes());
    let overflow: usize = parents.iter().filter(|p| p.len() > 2).map(Vec::len).sum();
    for n in [
        6,
        0,
        parents.len() as u32,
        parents.len() as u32,
        overflow as u32,
        0,
    ] {
        word(n);
    }
    let mut generations = vec![];
    let mut overflow = vec![];
    for (pos, parents) in parents.iter().enumerate() {
        let generation = parents
            .iter()
            .map(|p| generations[*p as usize] + 1)
            .max()
            .unwrap_or(0);
        generations.push(generation);
        let (first, second) = match parents.as_slice() {
            [] => (u32::MAX, u32::MAX),
            [a] => (*a, u32::MAX),
            [a, b] => (*a, *b),
            parents => {
                let start = overflow.len() as u32;
                overflow.extend_from_slice(parents);
                (!start, !(parents.len() as u32))
            }
        };
        for n in [generation, first, second, pos as u32] {
            bytes.extend(n.to_le_bytes());
        }
        bytes.extend([pos as u8 + 1; 20]);
    }
    for pos in 0..parents.len() as u32 {
        bytes.extend(pos.to_le_bytes());
    }
    for pos in 0..parents.len() {
        bytes.extend([pos as u8 + 1; 16]);
    }
    for pos in 0..parents.len() as u32 {
        bytes.extend(pos.to_le_bytes());
    }
    for pos in overflow {
        bytes.extend(pos.to_le_bytes());
    }
    bytes
}
fn graph(parents: &[Vec<u32>]) -> Graph {
    let bytes = encoded(parents);
    let lease = Lease::reserve(bytes.len() as u64 + 8192).unwrap();
    Graph::load(bytes, lease, 1000, |_| Ok(())).unwrap()
}
fn heads(mask: usize, nodes: usize) -> BTreeSet<m::GitSha1Id> {
    (0..nodes)
        .filter(|pos| mask & (1 << pos) != 0)
        .map(|pos| m::GitSha1Id::new([pos as u8 + 1; 20]))
        .collect()
}
fn ancestors(mut ids: BTreeSet<m::GitSha1Id>, parents: &[Vec<u32>]) -> BTreeSet<m::GitSha1Id> {
    let mut seen = BTreeSet::new();
    while let Some(id) = ids.pop_first() {
        if seen.insert(id) {
            ids.extend(
                parents[id.as_bytes()[0] as usize - 1]
                    .iter()
                    .map(|p| m::GitSha1Id::new([*p as u8 + 1; 20])),
            );
        }
    }
    seen
}
#[test]
fn frontier_differences_match_exhaustive_ancestry_for_branches_merges_and_redundant_heads() {
    let parents = vec![vec![], vec![0], vec![1], vec![1], vec![2, 3], vec![0, 2, 3]];
    let graph = graph(&parents);
    for before in 0..1 << parents.len() {
        for after in 0..1 << parents.len() {
            let before_heads = heads(before, parents.len());
            let after_heads = heads(after, parents.len());
            let before = ancestors(before_heads.clone(), &parents);
            let after = ancestors(after_heads.clone(), &parents);
            let actual = graph
                .difference(
                    &before_heads,
                    &after_heads,
                    &Budget::new(Limits::default()),
                    64,
                )
                .unwrap();
            assert_eq!(
                actual.added,
                after.difference(&before).copied().collect::<Vec<_>>()
            );
            assert_eq!(
                actual.removed,
                before.difference(&after).copied().collect::<Vec<_>>()
            );
        }
    }
    for pos in 0..parents.len() {
        assert_eq!(
            graph
                .change(
                    m::GitSha1Id::new([pos as u8 + 1; 20]),
                    &Budget::new(Limits::default())
                )
                .unwrap(),
            m::ChangeId::new([pos as u8 + 1; 16])
        );
    }
}
#[test]
fn shared_frontiers_stop_early_and_visit_work_and_cancel_limits_fail_explicitly() {
    let parents: Vec<_> = (0..100)
        .map(|pos| if pos == 0 { vec![] } else { vec![pos - 1] })
        .collect();
    let graph = graph(&parents);
    let before = BTreeSet::from([m::GitSha1Id::new([99; 20])]);
    let after = BTreeSet::from([m::GitSha1Id::new([100; 20])]);
    let delta = graph
        .difference(&before, &after, &Budget::new(Limits::default()), 2)
        .unwrap();
    assert_eq!(delta.added, after.iter().copied().collect::<Vec<_>>());
    assert!(delta.removed.is_empty());
    assert!(matches!(
        graph.difference(&before, &after, &Budget::new(Limits::default()), 1),
        Err(Error::Limit)
    ));
    assert!(
        graph
            .difference(&before, &before, &Budget::new(Limits::default()), 0)
            .unwrap()
            .added
            .is_empty()
    );
    let budget = Budget::new(Limits {
        max_work: 1,
        ..Default::default()
    });
    assert!(matches!(
        graph.difference(&before, &after, &budget, 100),
        Err(Error::Limit)
    ));
    let budget = Budget::new(Limits::default());
    budget.cancellation.cancel();
    assert!(matches!(
        graph.difference(&before, &after, &budget, 100),
        Err(Error::Canceled)
    ));
    assert!(matches!(
        graph.change(
            m::GitSha1Id::new([101; 20]),
            &Budget::new(Limits::default())
        ),
        Err(Error::NotCommitted)
    ));
}
#[test]
fn compact_graph_rejects_invalid_framing_lookup_parents_and_change_offsets() {
    let valid = encoded(&[vec![], vec![0], vec![0, 1], vec![0, 1, 2]]);
    let load = |bytes: Vec<u8>, work: Result<()>| {
        Graph::load(bytes, Lease::reserve(8192).unwrap(), 1000, |_| work.clone())
    };
    assert!(load(valid.clone(), Ok(())).is_ok());
    assert!(matches!(
        load(valid.clone(), Err(Error::Canceled)),
        Err(Error::Canceled)
    ));
    let mut invalid = vec![vec![], valid[..valid.len() - 1].to_vec()];
    for (offset, value) in [
        (0, 7),
        (4, 128),
        (8, u32::MAX),
        (12, 5),
        (24 + 36, 99),
        (24 + 36 + 4, 1),
        (24 + 12, 4),
        (24 + 36 * 4, 1),
        (24 + 36 * 3 + 8, !257),
    ] {
        let mut bytes = valid.clone();
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        invalid.push(bytes);
    }
    for bytes in invalid {
        assert!(matches!(load(bytes, Ok(())), Err(Error::Corrupt)));
    }
}
