use super::*;
use jj_lib::op_store as o;
use std::{fmt, time::SystemTime};

#[derive(Debug)]
struct SourceOnly;
impl Source for SourceOnly {
    fn check(&self) -> Result<()> {
        Ok(())
    }
    fn work(&self, _: usize) -> Result<()> {
        Ok(())
    }
    fn max_commits(&self) -> usize {
        100
    }
    fn max_parents(&self) -> usize {
        256
    }
    fn commit_len(&self) -> usize {
        20
    }
    fn change_len(&self) -> usize {
        16
    }
    async fn operation(&self, _: &o::OperationId) -> Result<Operation> {
        unreachable!()
    }
    async fn view(&self, _: &Operation) -> Result<jj_lib::view::View> {
        unreachable!()
    }
    async fn commit(&self, _: &backend::CommitId) -> Result<CommitFields> {
        unreachable!()
    }
}
struct Store(Vec<o::Operation>);
impl fmt::Debug for Store {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MetadataOnlyTestStore")
    }
}
fn id(index: usize) -> OperationId {
    OperationId::new([index as u8; 64])
}
fn native(index: usize) -> o::OperationId {
    o::OperationId::from_bytes(id(index).as_bytes())
}
#[async_trait::async_trait]
impl o::OpStore for Store {
    fn name(&self) -> &str {
        "test"
    }
    fn root_operation_id(&self) -> &o::OperationId {
        unreachable!()
    }
    async fn read_operation(&self, id: &o::OperationId) -> o::OpStoreResult<o::Operation> {
        Ok(self.0[id.as_bytes()[0] as usize].clone())
    }
    async fn read_view(&self, _: &o::ViewId) -> o::OpStoreResult<o::View> {
        panic!("topology reads no view")
    }
    async fn write_view(&self, _: &o::View) -> o::OpStoreResult<o::ViewId> {
        unreachable!()
    }
    async fn write_operation(&self, _: &o::Operation) -> o::OpStoreResult<o::OperationId> {
        unreachable!()
    }
    async fn resolve_operation_id_prefix(
        &self,
        _: &HexPrefix,
    ) -> o::OpStoreResult<PrefixResolution<o::OperationId>> {
        unreachable!()
    }
    async fn gc(&self, _: &[o::OperationId], _: SystemTime) -> o::OpStoreResult<()> {
        unreachable!()
    }
}
fn operation(parents: Vec<o::OperationId>, millis: i64, offset: i32) -> o::Operation {
    let time = backend::Timestamp {
        timestamp: backend::MillisSinceEpoch(millis),
        tz_offset: offset,
    };
    o::Operation {
        parents,
        view_id: o::ViewId::from_bytes(&[1; 64]),
        metadata: o::OperationMetadata {
            time: o::TimestampRange {
                start: time,
                end: time,
            },
            description: "operation".into(),
            username: "test".into(),
            hostname: "test".into(),
            is_snapshot: false,
            workspace_name: None,
            attributes: Default::default(),
        },
        commit_predecessors: None,
    }
}
fn artifact(
    rows: &[o::Operation],
    selected: usize,
) -> (tempfile::TempDir, Vec<super::super::Record>) {
    let directory = tempfile::tempdir().unwrap();
    let scratch = directory.path().join("scratch");
    let mut disk = Tables::new(&scratch, Space::new(1024 * 1024), true).unwrap();
    let store: Arc<dyn o::OpStore> = Arc::new(Store(rows.to_vec()));
    for (index, row) in rows.iter().enumerate() {
        capture(
            &mut disk,
            &Operation::new(store.clone(), native(index), row.clone()),
            &SourceOnly,
        )
        .unwrap();
    }
    write(
        directory.path(),
        &native(selected),
        &disk,
        &SourceOnly,
        Space::new(1024 * 1024),
    )
    .unwrap();
    let ids = (0..rows.len())
        .map(|index| super::super::Record {
            id: id(index),
            digest: [1; 32],
            view_digest: [2; 32],
        })
        .collect();
    (directory, ids)
}
#[tokio::test]
async fn compact_operation_topology_matches_jj_for_merges_skewed_times_and_every_subhead() {
    // Timestamp inversions and timezone tie breaking change lookahead chunks.
    // Compare every possible start, not just a filtered global-head sequence.
    for seed in 0..32 {
        let mut rows = vec![operation(vec![], 0, 0)];
        for index in 1..36 {
            let mut parents = vec![native((index - 1) / 2)];
            if index % 3 == 0 {
                parents.push(native((index + seed) % index));
            }
            let millis = ((index * 17 + seed * 11) % 23) as i64 - 12;
            rows.push(operation(parents, millis, (index % 3) as i32 - 1));
        }
        rows.push(operation((0..36).map(native).collect(), 0, 0));
        let (directory, ids) = artifact(&rows, rows.len() - 1);
        let graph = Arc::new(
            Graph::load(
                directory.path(),
                id(rows.len() - 1),
                &ids,
                &mut |_| Ok(()),
                &mut |_| Ok(()),
            )
            .unwrap()
            .unwrap(),
        );
        let store: Arc<dyn o::OpStore> = Arc::new(Store(rows.clone()));
        for (start, row) in rows.iter().enumerate() {
            let heads = [Operation::new(store.clone(), native(start), row.clone())];
            let expected: Vec<_> = jj_lib::op_walk::walk_ancestors(&heads)
                .map_ok(|op| OperationId::from_slice(op.id().as_bytes()).unwrap())
                .try_collect()
                .await
                .unwrap();
            let actual: Vec<_> = graph
                .clone()
                .walk(id(start), |_| Ok(()))
                .unwrap()
                .try_collect()
                .await
                .unwrap();
            assert_eq!(actual, expected, "seed{seed},start{start}");
            let retained: Vec<_> = graph
                .clone()
                .window(id(start), None, |_| Ok(()))
                .unwrap()
                .try_collect()
                .await
                .unwrap();
            assert_eq!(retained, expected, "retained seed{seed},start{start}");
            if start == rows.len() - 1 {
                for (rank, anchor) in expected.iter().enumerate() {
                    let mut work = 0;
                    let tail: Vec<_> = graph
                        .clone()
                        .window(id(start), Some(*anchor), |n| {
                            work += n;
                            Ok(())
                        })
                        .unwrap()
                        .try_collect()
                        .await
                        .unwrap();
                    assert_eq!(tail, expected[rank..], "seed{seed},rank{rank}");
                    assert_eq!(work, graph.nodes.len().ilog2() as usize + 2 + tail.len());
                }
            }
        }
        assert!(matches!(
            graph
                .clone()
                .window(id(rows.len() - 1), Some(id(99)), |_| Ok(())),
            Err(Error::Invalid)
        ));
        assert!(matches!(
            graph
                .clone()
                .window(id(rows.len() - 1), Some(id(0)), |_| Err(Error::Canceled)),
            Err(Error::Canceled)
        ));
        assert!(matches!(
            graph.clone().walk(id(99), |_| Ok(())),
            Err(Error::NotCommitted)
        ));
        assert!(matches!(
            graph.walk(id(0), |_| Err(Error::Canceled)),
            Err(Error::Canceled)
        ));
    }
}
#[test]
fn compact_operation_topology_rejects_selection_checksum_membership_cycles_and_byte_limits() {
    let rows = vec![operation(vec![], 0, 0), operation(vec![native(0)], 1, 0)];
    let (directory, ids) = artifact(&rows, 1);
    let path = directory.path().join(HISTORY_NAME);
    let valid = std::fs::read(&path).unwrap();
    assert!(matches!(
        Graph::load(directory.path(), id(0), &ids, &mut |_| Ok(()), &mut |_| Ok(
            ()
        )),
        Err(Error::Corrupt)
    ));
    assert!(matches!(
        Graph::load(
            directory.path(),
            id(1),
            &ids,
            &mut |_| Ok(()),
            &mut |_| Err(Error::Limit)
        ),
        Err(Error::Limit)
    ));
    let mut changed = valid.clone();
    changed[90] ^= 1;
    std::fs::write(&path, changed).unwrap();
    assert!(matches!(
        Graph::load(directory.path(), id(1), &ids, &mut |_| Ok(()), &mut |_| Ok(
            ()
        )),
        Err(Error::Corrupt)
    ));
    let mut changed = valid.clone();
    changed[88] ^= 1;
    let at = changed.len() - 32;
    let digest = Sha256::digest(&changed[..at]);
    changed[at..].copy_from_slice(&digest);
    std::fs::write(&path, changed).unwrap();
    assert!(matches!(
        Graph::load(directory.path(), id(1), &ids, &mut |_| Ok(()), &mut |_| Ok(
            ()
        )),
        Err(Error::Corrupt)
    ));
    let cyclic = vec![
        operation(vec![native(1)], 0, 0),
        operation(vec![native(0)], 1, 0),
    ];
    let (directory, ids) = artifact(&cyclic, 1);
    assert!(matches!(
        Graph::load(directory.path(), id(1), &ids, &mut |_| Ok(()), &mut |_| Ok(
            ()
        )),
        Err(Error::Corrupt)
    ));
    let (directory, ids) = artifact(&rows, 0);
    assert!(matches!(
        Graph::load(directory.path(), id(0), &ids, &mut |_| Ok(()), &mut |_| Ok(
            ()
        )),
        Err(Error::Corrupt)
    ));
    let directory = tempfile::tempdir().unwrap();
    let scratch = directory.path().join("scratch");
    let mut disk = Tables::new(&scratch, Space::new(1024 * 1024), true).unwrap();
    let store: Arc<dyn o::OpStore> = Arc::new(Store(rows.clone()));
    for (index, row) in rows.iter().enumerate() {
        capture(
            &mut disk,
            &Operation::new(store.clone(), native(index), row.clone()),
            &SourceOnly,
        )
        .unwrap();
    }
    let maximum = valid.len() as u64 - 1;
    assert_eq!(
        write(
            directory.path(),
            &native(1),
            &disk,
            &SourceOnly,
            Space::new(maximum)
        )
        .unwrap_err(),
        Error::Limit
    );
    assert!(
        std::fs::metadata(directory.path().join(HISTORY_NAME))
            .unwrap()
            .len()
            <= maximum
    );
}

#[tokio::test]
async fn selected_operation_continuation_seeks_under_a_fixed_work_budget() {
    let mut rows = vec![operation(vec![], 0, 0)];
    for index in 1..64 {
        rows.push(operation(vec![native(index - 1)], index as i64, 0));
    }
    let (directory, ids) = artifact(&rows, 63);
    let graph = Arc::new(
        Graph::load(directory.path(), id(63), &ids, &mut |_| Ok(()), &mut |_| {
            Ok(())
        })
        .unwrap()
        .unwrap(),
    );
    let store: Arc<dyn o::OpStore> = Arc::new(Store(rows.clone()));
    let head = Operation::new(store, native(63), rows[63].clone());
    let filter = crate::query::language::operations::Filter::parse("").unwrap();
    for _ in 0..2 {
        let mut work = 0;
        let (nodes, next) = read(
            &head,
            graph.clone(),
            &filter,
            Some(id(3)),
            2,
            |amount| {
                work += amount;
                if work > 16 { Err(Error::Limit) } else { Ok(()) }
            },
            |op| Ok(OperationId::from_slice(op.id().as_bytes()).unwrap()),
        )
        .await
        .unwrap();
        assert_eq!(nodes, [id(2), id(1)]);
        assert_eq!(next, Some(id(1)));
        assert!(work <= 16);
    }
}
