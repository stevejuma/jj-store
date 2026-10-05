use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::Cancellation};
use std::{fs, sync::Arc};
struct Denied(ReadBinding);
#[async_trait::async_trait]
impl q::Access for Denied {
    fn repository(&self) -> jj_store_model::RepositoryId {
        q::Access::repository(&self.0)
    }
    fn generation(&self) -> u64 {
        q::Access::generation(&self.0)
    }
    fn metadata_path(&self) -> &std::path::Path {
        q::Access::metadata_path(&self.0)
    }
    async fn authorize(&self) -> q::Result<()> {
        Err(q::Error::Denied)
    }
}

#[tokio::test]
async fn prepared_operation_prefixes_resolve_with_only_selected_metadata() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    let mut operations = vec![];
    for number in 0..18 {
        jj(source.path(), &["new", "-m", &format!("History {number}")]);
        operations.push(jj(
            source.path(),
            &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
        ));
    }
    let operation = operations.last().unwrap().parse().unwrap();
    let selected = &operations[4];
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let executor = q::Executor::new(1).unwrap();
    let request = |selector: &str| {
        q::Request::Graph(
            q::GraphRequest::OperationsFiltered {
                query: format!("from:{selector}"),
            },
            q::GraphWindow {
                limit: 1,
                after: None,
            },
        )
    };
    let selectors = [
        selected.clone(),
        selected[..9].to_string(),
        selected[..12].to_uppercase(),
        format!("{}-", &selected[..9]),
        format!("{}++", &selected[..9]),
        format!("{}+-", &selected[..9]),
        format!("{}-+", &selected[..9]),
        format!("{}++-", &selected[..9]),
        "@-+".to_owned(),
    ];
    let mut expected = vec![];
    for selector in &selectors {
        let q::Response::Graph(page) = executor
            .read(access.clone(), operation, request(selector))
            .await
            .unwrap()
        else {
            panic!()
        };
        expected.push(page);
    }
    let child_selector = format!("{}+", &selected[..9]);
    let q::Response::Graph(child_expected) = executor
        .read(access.clone(), operation, request(&child_selector))
        .await
        .unwrap()
    else {
        panic!()
    };
    let prepared = Arc::new(
        executor
            .prepare_index(
                access.clone(),
                operation,
                output.path().into(),
                16 * 1024 * 1024,
                q::preparation::Limits::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    let small = q::Executor::with_workers(
        jj_store_jj::query::workers::Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 4,
            ..Default::default()
        },
    );
    let q::Response::Graph(actual) = small
        .read_prepared(access.clone(), prepared.clone(), request(&selectors[0]))
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(actual.data, expected[0].data);
    assert!(actual.stats.metadata_nodes <= 8, "{:?}", actual.stats);
    jj(source.path(), &["new", "-m", "Excluded later operation"]);
    let later = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    );
    assert!(matches!(
        small
            .read_prepared(access.clone(), prepared.clone(), request(&later))
            .await,
        Err(q::Error::InvalidQuery(_))
    ));
    // Operation selection needs no commit/tree objects or source commit index.
    for entry in fs::read_dir(source.path().join(".git/objects")).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            fs::remove_dir_all(entry.path()).unwrap();
        } else {
            fs::remove_file(entry.path()).unwrap();
        }
    }
    for entry in fs::read_dir(source.path().join(".jj/repo/index/segments")).unwrap() {
        fs::write(entry.unwrap().path(), b"unavailable source commit index").unwrap();
    }
    fs::remove_file(prepared.path().join("local-operation-children")).unwrap();
    for (selector, expected) in selectors.iter().zip(expected) {
        let q::Response::Graph(actual) = small
            .read_prepared(access.clone(), prepared.clone(), request(selector))
            .await
            .unwrap()
        else {
            panic!()
        };
        assert_eq!(actual.data, expected.data);
        assert_eq!(actual.next, expected.next);
        // Preserve the original single-step bound; each additional child step
        // may read one additional selected operation, never intervening history.
        let maximum = 8 + selector.matches('+').count().saturating_sub(1) as u64;
        assert!(actual.stats.metadata_nodes <= maximum, "{:?}", actual.stats);
    }
    let q::Response::Graph(child_small) = small
        .read_prepared(access.clone(), prepared.clone(), request(&child_selector))
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(child_small.data, child_expected.data);
    assert_eq!(child_small.next, child_expected.next);
    assert!(
        child_small.stats.metadata_nodes <= 8,
        "{:?}",
        child_small.stats
    );
    let q::Response::Graph(child_actual) = executor
        .read_prepared(access.clone(), prepared.clone(), request(&child_selector))
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(child_actual.data, child_expected.data);
    assert_eq!(child_actual.next, child_expected.next);
    let mut prefixes = std::collections::BTreeMap::<u8, usize>::new();
    for id in &operations {
        *prefixes.entry(id.as_bytes()[0]).or_default() += 1;
    }
    let prefix = prefixes
        .into_iter()
        .find(|(_, count)| *count > 1)
        .unwrap()
        .0 as char;
    assert!(matches!(
        small
            .read_prepared(
                access.clone(),
                prepared.clone(),
                request(&prefix.to_string())
            )
            .await,
        Err(q::Error::InvalidQuery(_))
    ));
    assert!(matches!(
        small
            .read_prepared(access.clone(), prepared.clone(), request("@+"))
            .await,
        Err(q::Error::InvalidQuery(_))
    ));
    let file = source
        .path()
        .join(".jj/repo/op_store/operations")
        .join(selected);
    assert!(matches!(
        small
            .read_prepared(
                Arc::new(Denied(ReadBinding(source.path().join(".jj/repo")))),
                prepared.clone(),
                request(selected)
            )
            .await,
        Err(q::Error::Denied)
    ));
    let captured = fs::read(&file).unwrap();
    let replacement = fs::read(
        source
            .path()
            .join(".jj/repo/op_store/operations")
            .join(operation.to_string()),
    )
    .unwrap();
    fs::write(&file, &replacement).unwrap();
    assert!(matches!(
        small
            .read_prepared(access.clone(), prepared.clone(), request(selected))
            .await,
        Err(q::Error::Corrupt)
    ));
    fs::write(file, captured).unwrap();
    assert!(
        small
            .read_prepared(access.clone(), prepared.clone(), request(selected))
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn prepared_child_operation_selectors_reject_retained_branch_ambiguity_and_later_children() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    for n in 0..8 {
        jj(source.path(), &["new", "-m", &format!("History {n}")]);
    }
    let branch = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    );
    jj(source.path(), &["new", "-m", "Left"]);
    jj(source.path(), &["--at-op", &branch, "new", "-m", "Right"]);
    jj(source.path(), &["log", "--no-graph", "-r", "all()"]);
    let operation = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let executor = q::Executor::new(1).unwrap();
    let request = |selector: &str| {
        q::Request::Graph(
            q::GraphRequest::OperationsFiltered {
                query: format!("from:{selector}"),
            },
            q::GraphWindow {
                limit: 1,
                after: None,
            },
        )
    };
    let ordinary = executor
        .read(access.clone(), operation, request(&format!("{branch}+")))
        .await;
    assert!(matches!(ordinary, Err(q::Error::InvalidQuery(_))));
    let prepared = Arc::new(
        executor
            .prepare_index(
                access.clone(),
                operation,
                output.path().into(),
                32 * 1024 * 1024,
                q::preparation::Limits::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    let small = q::Executor::with_workers(
        jj_store_jj::query::workers::Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 4,
            ..Default::default()
        },
    );
    assert!(matches!(
        small
            .read_prepared(
                access.clone(),
                prepared.clone(),
                request(&format!("{branch}+"))
            )
            .await,
        Err(q::Error::InvalidQuery(_))
    ));
    // Current-relative children reuse prepared lookup, with a bounded cold fallback.
    assert!(request("@+").can_use_prepared_index());
    assert!(!request("@+").needs_prepared_index());
    assert!(!request("@-").needs_prepared_index());
    // Merge parents are ambiguous before any parent body is requested.
    assert!(matches!(
        small
            .read_prepared(access.clone(), prepared.clone(), request("@-+"))
            .await,
        Err(q::Error::InvalidQuery(_))
    ));
    jj(source.path(), &["new", "-m", "Later excluded child"]);
    assert!(matches!(
        small
            .read_prepared(access.clone(), prepared.clone(), request("@+"))
            .await,
        Err(q::Error::InvalidQuery(_))
    ));
    assert!(matches!(
        small
            .read_prepared(
                Arc::new(Denied(ReadBinding(source.path().join(".jj/repo")))),
                prepared,
                request(&format!("{branch}+"))
            )
            .await,
        Err(q::Error::Denied)
    ));
}
