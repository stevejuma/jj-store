use super::*;

#[test]
fn flat_native_groups_preserve_lexical_windows_and_divergent_target_order() {
    let budget = Budget::new(Limits::default()).unwrap();
    let count = 6144;
    let mut references = References::with_capacity(count);
    let mut expected: BTreeMap<String, Vec<m::CommitId>> = BTreeMap::new();
    for n in (0..count).rev() {
        let mut change = [0; 16];
        change[..8].copy_from_slice(&((n % 2048) as u64 * 7919).to_be_bytes());
        let change = m::ChangeId::new(change);
        let mut commit = [0; 64];
        commit[..8].copy_from_slice(&(n as u64).to_be_bytes());
        let commit = m::CommitId::new(commit);
        references.push(change, commit).unwrap();
        expected
            .entry(format!("change:{change}"))
            .or_default()
            .push(commit);
    }
    assert!(matches!(
        references.push(m::ChangeId::new([0; 16]), m::CommitId::new([0; 64])),
        Err(Error::Corrupt)
    ));
    references.finish(&budget).unwrap();
    for targets in expected.values_mut() {
        targets.sort_unstable();
    }
    assert_eq!(references.len(), expected.len());
    let describe = |group: Group<'_>| {
        (
            group.key(),
            group
                .targets
                .iter()
                .map(|row| row.commit)
                .collect::<Vec<_>>(),
        )
    };
    assert_eq!(
        describe(references.first().unwrap()).0,
        *expected.first_key_value().unwrap().0
    );
    assert_eq!(
        describe(references.last().unwrap()).0,
        *expected.last_key_value().unwrap().0
    );
    for (key, targets) in &expected {
        assert_eq!(
            describe(references.get(key).unwrap()),
            (key.clone(), targets.clone())
        );
    }
    let midpoint = expected.keys().nth(expected.len() / 2).unwrap().clone();
    let anchors = [
        None,
        Some(""),
        Some("bookmark:z"),
        Some("change:"),
        Some("change:00000000"),
        Some(midpoint.as_str()),
        Some("change:0000000000F"),
        Some("change:0000000000f"),
        Some("change:ffff"),
        Some("change:☃"),
        Some("z"),
    ];
    let mut windows = 0;
    for direction in [Direction::Forward, Direction::Backward] {
        for anchor in anchors {
            for limit in [1, 3, 256] {
                let oracle = expected.iter().filter(|(key, _)| {
                    anchor.is_none_or(|anchor| match direction {
                        Direction::Forward => key.as_str() > anchor,
                        Direction::Backward => key.as_str() < anchor,
                    })
                });
                let actual = references.window(anchor, direction);
                let (actual, oracle): (Vec<_>, Vec<_>) = match direction {
                    Direction::Forward => (
                        actual.take(limit).map(describe).collect(),
                        oracle
                            .take(limit)
                            .map(|(key, targets)| (key.clone(), targets.clone()))
                            .collect(),
                    ),
                    Direction::Backward => (
                        actual.rev().take(limit).map(describe).collect(),
                        oracle
                            .rev()
                            .take(limit)
                            .map(|(key, targets)| (key.clone(), targets.clone()))
                            .collect(),
                    ),
                };
                assert_eq!(actual, oracle, "{direction:?}/{anchor:?}/{limit}");
                windows += 1;
            }
        }
    }
    assert_eq!(windows, 66);
    assert!(references.get("change:0").is_none());
    let allocated = references.rows.capacity() * size_of::<VisibleTarget>()
        + references.starts.capacity() * size_of::<u32>()
        + count * size_of::<m::CommitId>();
    assert!(allocated as u64 <= super::super::commit_weight(count as u32));
    eprintln!(
        "native compact arrays: {count} identities, {allocated} capacity bytes, {windows} differential windows"
    );
}

#[test]
fn native_projection_sort_is_exact_and_stops_at_work_cancellation_and_deadline_bounds() {
    for length in [0, 1, 2, 3, 31, 32, 257, 4096] {
        let budget = Budget::new(Limits::default()).unwrap();
        let mut values: Vec<_> = (0..length).map(|n| (n * 7919) % 271).collect();
        let mut expected = values.clone();
        expected.sort_unstable();
        sort(&mut values, &budget).unwrap();
        assert_eq!(values, expected);
    }
    let budget = Budget::new(Limits {
        max_work: 5,
        ..Default::default()
    })
    .unwrap();
    let mut values: Vec<_> = (0..4096).rev().collect();
    assert!(matches!(sort(&mut values, &budget), Err(Error::Limit)));
    assert_ne!(values, (0..4096).collect::<Vec<_>>());
    let budget = Budget::new(Limits::default()).unwrap();
    budget.cancellation.cancel();
    let before = values.clone();
    assert!(matches!(sort(&mut values, &budget), Err(Error::Canceled)));
    assert_eq!(values, before);
    let mut budget = Budget::new(Limits::default()).unwrap();
    budget.deadline = tokio::time::Instant::now();
    assert!(matches!(sort(&mut values, &budget), Err(Error::Limit)));
    assert_eq!(values, before);
}
