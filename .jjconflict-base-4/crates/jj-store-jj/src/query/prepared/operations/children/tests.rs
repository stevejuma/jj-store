use super::*;
fn records() -> Vec<super::super::Record> {
    (1..=4)
        .map(|byte| super::super::Record {
            id: OperationId::new([byte; 64]),
            digest: [4; 32],
            view_digest: [5; 32],
        })
        .collect()
}
fn artifact(selected: OperationId, edges: &[(OperationId, OperationId)]) -> Vec<u8> {
    let mut bytes = MAGIC.to_vec();
    bytes.extend_from_slice(selected.as_bytes());
    for (parent, child) in edges {
        bytes.extend_from_slice(parent.as_bytes());
        bytes.extend_from_slice(child.as_bytes());
    }
    let checksum = Sha256::digest(&bytes);
    bytes.extend_from_slice(&checksum);
    bytes
}
#[test]
fn child_projection_preserves_unique_empty_ambiguous_and_missing_artifact_outcomes() {
    let directory = tempfile::tempdir().unwrap();
    let ids = records();
    let id = |n: usize| ids[n].id;
    let selected = id(3);
    assert!(
        Lookup::load(
            directory.path(),
            selected,
            &ids,
            4,
            &mut |_| Ok(()),
            &mut |_| Ok(())
        )
        .unwrap()
        .is_none()
    );
    std::fs::write(
        directory.path().join(FILE),
        artifact(selected, &[(id(0), id(1)), (id(1), id(2)), (id(1), id(3))]),
    )
    .unwrap();
    let lookup = Lookup::load(
        directory.path(),
        selected,
        &ids,
        4,
        &mut |_| Ok(()),
        &mut |_| Ok(()),
    )
    .unwrap()
    .unwrap();
    std::fs::remove_file(directory.path().join(FILE)).unwrap();
    assert_eq!(
        lookup.child(id(0), |_| Ok(())).unwrap(),
        PrefixResolution::SingleMatch(jj_lib::op_store::OperationId::from_bytes(id(1).as_bytes()))
    );
    assert_eq!(
        lookup.child(id(1), |_| Ok(())).unwrap(),
        PrefixResolution::AmbiguousMatch
    );
    assert_eq!(
        lookup.child(id(3), |_| Ok(())).unwrap(),
        PrefixResolution::NoMatch
    );
    assert_eq!(
        lookup.child(id(0), |_| Err(Error::Canceled)).unwrap_err(),
        Error::Canceled
    );
}
#[test]
fn child_projection_rejects_corruption_outside_membership_and_preallocation_limits() {
    let directory = tempfile::tempdir().unwrap();
    let ids = records();
    let id = |n: usize| ids[n].id;
    let selected = id(3);
    for edges in [
        vec![(id(0), id(0))],
        vec![(id(0), OperationId::new([9; 64]))],
        vec![(id(1), id(2)), (id(0), id(1))],
        vec![(id(0), id(1)), (id(0), id(1))],
    ] {
        std::fs::write(directory.path().join(FILE), artifact(selected, &edges)).unwrap();
        assert!(matches!(
            Lookup::load(
                directory.path(),
                selected,
                &ids,
                4,
                &mut |_| Ok(()),
                &mut |_| Ok(())
            ),
            Err(Error::Corrupt)
        ));
    }
    let mut bytes = artifact(selected, &[(id(0), id(1))]);
    let last = bytes.len() - 1;
    bytes[last] ^= 1;
    std::fs::write(directory.path().join(FILE), bytes).unwrap();
    assert!(matches!(
        Lookup::load(
            directory.path(),
            selected,
            &ids,
            4,
            &mut |_| Ok(()),
            &mut |_| Ok(())
        ),
        Err(Error::Corrupt)
    ));
    std::fs::write(
        directory.path().join(FILE),
        artifact(selected, &[(id(0), id(1))]),
    )
    .unwrap();
    assert!(matches!(
        Lookup::load(
            directory.path(),
            selected,
            &ids,
            0,
            &mut |_| Ok(()),
            &mut |_| Ok(())
        ),
        Err(Error::Limit)
    ));
    assert!(matches!(
        Lookup::load(
            directory.path(),
            selected,
            &ids,
            4,
            &mut |_| Err(Error::Canceled),
            &mut |_| Ok(())
        ),
        Err(Error::Canceled)
    ));
    assert!(matches!(
        Lookup::load(
            directory.path(),
            selected,
            &ids,
            4,
            &mut |_| Ok(()),
            &mut |_| Err(Error::Limit)
        ),
        Err(Error::Limit)
    ));
    assert!(matches!(
        Lookup::load(
            directory.path(),
            id(2),
            &ids,
            4,
            &mut |_| Ok(()),
            &mut |_| Ok(())
        ),
        Err(Error::Corrupt)
    ));
}
