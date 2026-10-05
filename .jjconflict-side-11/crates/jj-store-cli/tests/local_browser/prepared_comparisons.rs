use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::Cancellation};
use std::{fs, sync::Arc};

struct ChangedAccess {
    binding: ReadBinding,
    generation: u64,
    denied: bool,
}
#[async_trait::async_trait]
impl q::Access for ChangedAccess {
    fn repository(&self) -> jj_store_model::RepositoryId {
        q::Access::repository(&self.binding)
    }
    fn generation(&self) -> u64 {
        self.generation
    }
    fn metadata_path(&self) -> &std::path::Path {
        q::Access::metadata_path(&self.binding)
    }
    async fn authorize(&self) -> q::Result<()> {
        if self.denied {
            Err(q::Error::Denied)
        } else {
            Ok(())
        }
    }
}

fn operation(root: &std::path::Path) -> jj_store_model::OperationId {
    jj(root, &["op", "log", "--no-graph", "-n", "1", "-T", "id"])
        .parse()
        .unwrap()
}
async fn prepare(
    reader: &q::Executor,
    access: Arc<dyn q::Access>,
    operation: jj_store_model::OperationId,
    output: &std::path::Path,
) -> Arc<q::preparation::Prepared> {
    Arc::new(
        reader
            .prepare_index(
                access,
                operation,
                output.into(),
                16 * 1024 * 1024,
                q::preparation::Limits::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    )
}

#[tokio::test]
async fn prepared_operation_comparison_skips_shared_history_and_git_objects() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    for n in 0..22 {
        jj(source.path(), &["new", "-m", &format!("History {n}")]);
    }
    let selected = operation(source.path());
    let request = q::Request::OperationDiff(q::operations::Request {
        operation: selected,
        parent: None,
    });
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let reader = q::Executor::new(1).unwrap();
    let q::Response::OperationDiff(expected) = reader
        .read(access.clone(), selected, request.clone())
        .await
        .unwrap()
    else {
        panic!()
    };
    jj(source.path(), &["bookmark", "create", "later"]);
    let outer = operation(source.path());
    let prepared = prepare(&reader, access.clone(), outer, output.path()).await;
    let small = q::Executor::with_workers(
        jj_store_jj::query::workers::Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 4,
            ..Default::default()
        },
    );
    let q::Response::OperationDiff(actual) = small
        .read_prepared(access.clone(), prepared.clone(), request.clone())
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(actual.data, expected.data);
    assert!(actual.stats.metadata_nodes <= 9, "{:?}", actual.stats);
    for (generation, denied, expected_error) in [
        (1, true, q::Error::Denied),
        (2, false, q::Error::EpochChanged),
    ] {
        let changed = Arc::new(ChangedAccess {
            binding: ReadBinding(source.path().join(".jj/repo")),
            generation,
            denied,
        });
        assert!(
            matches!(small.read_prepared(changed, prepared.clone(), request.clone()).await, Err(error) if error == expected_error)
        );
    }
    assert!(matches!(
        small
            .read_prepared(
                access.clone(),
                prepared.clone(),
                q::Request::OperationDiff(q::operations::Request {
                    operation: selected,
                    parent: Some(outer),
                })
            )
            .await,
        Err(q::Error::Invalid)
    ));
    let root_request = q::Request::OperationDiff(q::operations::Request {
        operation: jj_store_model::OperationId::new([0; 64]),
        parent: None,
    });
    let q::Response::OperationDiff(root) = small
        .read_prepared(access.clone(), prepared.clone(), root_request)
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(matches!(root.data, q::operations::Data::Root { .. }));
    jj(source.path(), &["new", "-m", "Excluded future"]);
    let future = operation(source.path());
    assert!(matches!(
        small
            .read_prepared(
                access.clone(),
                prepared.clone(),
                q::Request::OperationDiff(q::operations::Request {
                    operation: future,
                    parent: None,
                })
            )
            .await,
        Err(q::Error::NotCommitted)
    ));
    // No commit or tree is needed to compare an admitted pair of views.
    for entry in fs::read_dir(source.path().join(".git/objects")).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            fs::remove_dir_all(entry.path()).unwrap();
        } else {
            fs::remove_file(entry.path()).unwrap();
        }
    }
    for entry in fs::read_dir(source.path().join(".jj/repo/index/segments")).unwrap() {
        fs::write(entry.unwrap().path(), b"unavailable source index").unwrap();
    }
    fs::remove_dir_all(prepared.path().join("segments")).unwrap();
    let q::Response::Operation(detail) = small
        .read_prepared(
            access.clone(),
            prepared.clone(),
            q::Request::Operation {
                operation: selected,
            },
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(matches!(detail.operation.key, q::ObjectKey::Operation(id) if id == selected));
    let q::Response::OperationDiff(actual) = small
        .read_prepared(access.clone(), prepared.clone(), request.clone())
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(actual.data, expected.data);
    let q::operations::Data::Difference(expected) = expected.data else {
        panic!()
    };
    let view = source
        .path()
        .join(".jj/repo/op_store/views")
        .join(expected.before_view.to_string());
    let captured = fs::read(&view).unwrap();
    let replacement = fs::read(
        source
            .path()
            .join(".jj/repo/op_store/views")
            .join(expected.after_view.to_string()),
    )
    .unwrap();
    assert_ne!(captured, replacement);
    fs::write(&view, replacement).unwrap();
    assert!(matches!(
        small
            .read_prepared(access.clone(), prepared.clone(), request.clone())
            .await,
        Err(q::Error::Corrupt)
    ));
    fs::write(view, captured).unwrap();
    assert!(small.read_prepared(access, prepared, request).await.is_ok());
}

#[tokio::test]
async fn prepared_operation_change_pages_hydrate_only_selected_groups() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    for n in 0..18 {
        jj(source.path(), &["new", "-m", &format!("History {n}")]);
    }
    let parent = operation(source.path());
    jj(
        source.path(),
        &["describe", "-r", "all() ~ root()", "-m", "Rewritten"],
    );
    let selected = operation(source.path());
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let reader = q::Executor::new(1).unwrap();
    let request = |limit, anchor, direction| {
        q::Request::OperationChanges(q::operation_changes::Request {
            operation: selected,
            parent,
            limit,
            anchor,
            direction,
        })
    };
    let q::Response::OperationChanges(all) = reader
        .read(
            access.clone(),
            selected,
            request(100, None, q::Direction::Forward),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(all.groups.len() > 10);
    let prepared = prepare(&reader, access.clone(), selected, output.path()).await;
    let small = q::Executor::with_workers(
        jj_store_jj::query::workers::Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 128,
            ..Default::default()
        },
    );
    let mut joined = vec![];
    let mut anchor = None;
    loop {
        let req = request(2, anchor, q::Direction::Forward);
        let q::Response::OperationChanges(expected) = reader
            .read(access.clone(), selected, req.clone())
            .await
            .unwrap()
        else {
            panic!()
        };
        let q::Response::OperationChanges(actual) = small
            .read_prepared(access.clone(), prepared.clone(), req)
            .await
            .unwrap()
        else {
            panic!()
        };
        assert_eq!(actual.groups, expected.groups);
        if joined.is_empty() {
            eprintln!(
                "prepared change page: {} groups, {:?}",
                actual.groups.len(),
                actual.stats
            );
        }
        assert_eq!(
            (&actual.previous, &actual.next),
            (&expected.previous, &expected.next)
        );
        assert!(actual.stats.metadata_nodes <= 9, "{:?}", actual.stats);
        if let Some(previous) = actual.previous.clone() {
            let req = request(2, Some(previous), q::Direction::Backward);
            let q::Response::OperationChanges(expected) = reader
                .read(access.clone(), selected, req.clone())
                .await
                .unwrap()
            else {
                panic!()
            };
            let q::Response::OperationChanges(back) = small
                .read_prepared(access.clone(), prepared.clone(), req)
                .await
                .unwrap()
            else {
                panic!()
            };
            assert_eq!(back.groups, expected.groups);
            assert_eq!(
                (&back.previous, &back.next),
                (&expected.previous, &expected.next)
            );
        }
        joined.extend(actual.groups);
        anchor = actual.next;
        if anchor.is_none() {
            break;
        }
    }
    assert_eq!(joined, all.groups);
    // Explicit predecessor relationships and indexed change IDs need no Git
    // body hydration, even for the selected groups.
    for entry in fs::read_dir(source.path().join(".git/objects")).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            fs::remove_dir_all(entry.path()).unwrap();
        } else {
            fs::remove_file(entry.path()).unwrap();
        }
    }
    let q::Response::OperationChanges(replay) = small
        .read_prepared(
            access.clone(),
            prepared.clone(),
            request(2, None, q::Direction::Forward),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(replay.groups, all.groups[..2]);
    let tiny = q::Executor::with_workers(
        jj_store_jj::query::workers::Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 4,
            ..Default::default()
        },
    );
    assert!(matches!(
        tiny.read_prepared(access, prepared, request(2, None, q::Direction::Forward))
            .await,
        Err(q::Error::Limit)
    ));
}
