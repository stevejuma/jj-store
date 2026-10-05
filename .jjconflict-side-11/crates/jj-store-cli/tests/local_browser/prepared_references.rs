use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::Cancellation};
use std::{fs, sync::Arc};

#[tokio::test]
async fn prepared_filter_windows_preserve_unicode_substrings_and_hydrate_only_selected_targets() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    for n in 0..18 {
        jj(
            source.path(),
            &["new", "-m", &format!("Unrelated history {n}")],
        );
    }
    let unrelated = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@----------", "-T", "commit_id"],
    );
    jj(
        source.path(),
        &["describe", "-m", "ΟΣ İSTANBUL mixed Needle repeated needle"],
    );
    jj(source.path(), &["bookmark", "create", "conflicted"]);
    let original = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    );
    jj(source.path(), &["describe", "-m", "Left ΟΣ needle"]);
    jj(
        source.path(),
        &["--at-op", &original, "describe", "-m", "Right İSTANBUL"],
    );
    jj(source.path(), &["log", "--no-graph", "-r", "all()"]);
    for name in ["a", "NEEDLE-name", "z"] {
        jj(source.path(), &["bookmark", "create", name]);
    }
    let operation = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let ordinary = q::Executor::new(1).unwrap();
    let mut requests = vec![];
    for kind in [
        q::references::Kind::Changes,
        q::references::Kind::All,
        q::references::Kind::Bookmarks,
    ] {
        for query in [
            "NEEDLE",
            "Left",
            "mixed Needle",
            "repeated needle",
            "ΟΣ",
            "İSTANBUL",
            "e",
            "hi",
            "change:",
            "bookmark:",
            "absent text",
            "needle unrelated",
        ] {
            for direction in [q::Direction::Forward, q::Direction::Backward] {
                for anchor in [
                    None,
                    Some("bookmark:NEEDLE-name".into()),
                    Some("change:8".into()),
                    Some("z".into()),
                ] {
                    requests.push(q::references::Request {
                        operation,
                        query: query.into(),
                        kind,
                        limit: 1,
                        exact: None,
                        anchor,
                        direction,
                    });
                }
            }
        }
    }
    let mut expected = vec![];
    for request in &requests {
        let q::Response::References(page) = ordinary
            .read(
                access.clone(),
                operation,
                q::Request::References(request.clone()),
            )
            .await
            .unwrap()
        else {
            panic!()
        };
        expected.push(page);
    }
    let prepared = Arc::new(
        ordinary
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
    jj(source.path(), &["new", "-m", "Later excluded needle"]);
    for entry in fs::read_dir(source.path().join(".jj/repo/index/segments")).unwrap() {
        fs::write(entry.unwrap().path(), b"unavailable source index").unwrap();
    }
    fs::remove_dir_all(prepared.path().join("segments")).unwrap();
    for (request, expected) in requests.into_iter().zip(expected) {
        assert!(!request.query.is_empty());
        let q::Response::References(actual) = small
            .read_prepared(
                access.clone(),
                prepared.clone(),
                q::Request::References(request),
            )
            .await
            .unwrap()
        else {
            panic!()
        };
        assert_eq!(actual.references, expected.references);
        assert_eq!(actual.previous, expected.previous);
        assert_eq!(actual.next, expected.next);
        assert!(actual.stats.metadata_nodes <= 6, "{:?}", actual.stats);
    }
    let unrelated = unrelated.trim();
    fs::remove_file(
        source
            .path()
            .join(".git/objects")
            .join(&unrelated[..2])
            .join(&unrelated[2..]),
    )
    .unwrap();
    let q::Response::References(page) = small
        .read_prepared(
            access,
            prepared,
            q::Request::References(q::references::Request {
                operation,
                query: "NEEDLE".into(),
                kind: q::references::Kind::Changes,
                limit: 1,
                anchor: None,
                direction: q::Direction::Forward,
                exact: None,
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(page.references.len(), 1);
    assert!(matches!(
        &page.references[0],
        q::references::Reference::Change {
            divergent: true,
            ..
        }
    ));
    eprintln!("filtered one-row prepared page: {:?}", page.stats);
}

#[tokio::test]
async fn prepared_reference_windows_seek_visible_changes_before_hydrating_targets() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    for n in 0..12 {
        jj(source.path(), &["new", "-m", &format!("History {n}")]);
    }
    jj(source.path(), &["describe", "-m", "Original"]);
    jj(source.path(), &["bookmark", "create", "conflicted"]);
    let original = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    );
    jj(source.path(), &["describe", "-m", "Left"]);
    jj(
        source.path(),
        &["--at-op", &original, "describe", "-m", "Right"],
    );
    jj(source.path(), &["log", "--no-graph", "-r", "all()"]);
    for name in ["a", "middle", "z", "jj-store/workspaces/fixture/wip"] {
        jj(source.path(), &["bookmark", "create", name]);
    }
    let operation = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let ordinary = q::Executor::new(1).unwrap();
    let request = |kind, anchor, direction, exact| q::references::Request {
        operation,
        query: String::new(),
        exact,
        kind,
        limit: 2,
        anchor,
        direction,
    };
    let q::Response::References(all) = ordinary
        .read(
            access.clone(),
            operation,
            q::Request::References(q::references::Request {
                limit: 100,
                ..request(q::references::Kind::All, None, q::Direction::Forward, None)
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(all.references.iter().any(|r| matches!(
        r,
        q::references::Reference::Change {
            divergent: true,
            ..
        }
    )));
    let bookmark_names: Vec<_> = all
        .references
        .iter()
        .filter_map(|r| match r {
            q::references::Reference::Bookmark { name, .. } => Some(name.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(bookmark_names, vec!["a", "conflicted", "middle", "z"]);
    assert!(all.references.iter().any(|r| matches!(
        r,
        q::references::Reference::Bookmark {
            conflicted: true,
            ..
        }
    )));
    let changes: Vec<_> = all
        .references
        .iter()
        .filter_map(|r| match r {
            q::references::Reference::Change { change, .. } => Some(change.to_string()),
            _ => None,
        })
        .collect();
    let mut requests = vec![];
    for kind in [
        q::references::Kind::Bookmarks,
        q::references::Kind::Changes,
        q::references::Kind::All,
    ] {
        for direction in [q::Direction::Forward, q::Direction::Backward] {
            for anchor in [
                None,
                Some("bookmark:middle".into()),
                Some(format!("change:{}", changes[changes.len() / 2])),
                Some("z".into()),
            ] {
                if kind == q::references::Kind::Bookmarks
                    && anchor
                        .as_deref()
                        .is_some_and(|value: &str| !value.starts_with("bookmark:"))
                {
                    continue;
                }
                requests.push(request(kind, anchor, direction, None));
            }
        }
    }
    requests.push(request(
        q::references::Kind::Bookmarks,
        None,
        q::Direction::Forward,
        Some("jj-store/workspaces/fixture/wip".into()),
    ));
    let q::Response::References(filtered) = ordinary
        .read(
            access.clone(),
            operation,
            q::Request::References(q::references::Request {
                query: "jj-store/workspaces/".into(),
                ..request(
                    q::references::Kind::Bookmarks,
                    None,
                    q::Direction::Forward,
                    None,
                )
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(filtered.references.is_empty());
    requests.push(request(
        q::references::Kind::Changes,
        None,
        q::Direction::Forward,
        Some(changes[changes.len() / 2].clone()),
    ));
    requests.push(request(
        q::references::Kind::Changes,
        None,
        q::Direction::Backward,
        Some("missing".into()),
    ));
    let mut expected = vec![];
    for request in &requests {
        let q::Response::References(page) = ordinary
            .read(
                access.clone(),
                operation,
                q::Request::References(request.clone()),
            )
            .await
            .unwrap()
        else {
            panic!()
        };
        expected.push(page);
    }
    let prepared = Arc::new(
        ordinary
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
    assert!(matches!(
        small
            .read(
                access.clone(),
                operation,
                q::Request::References(request(
                    q::references::Kind::Changes,
                    None,
                    q::Direction::Forward,
                    None
                ))
            )
            .await,
        Err(q::Error::Limit)
    ));
    jj(source.path(), &["new", "-m", "Later excluded change"]);
    let later = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    for entry in fs::read_dir(source.path().join(".jj/repo/index/segments")).unwrap() {
        fs::write(entry.unwrap().path(), b"unavailable source index").unwrap();
    }
    fs::remove_dir_all(prepared.path().join("segments")).unwrap();
    for (request, expected) in requests.into_iter().zip(expected) {
        let query = q::Request::References(request);
        let response = if query.can_use_prepared_index() {
            small
                .read_prepared(access.clone(), prepared.clone(), query)
                .await
        } else {
            small.read(access.clone(), operation, query).await
        };
        let q::Response::References(actual) = response.unwrap() else {
            panic!()
        };
        assert_eq!(actual.references, expected.references);
        assert_eq!(actual.previous, expected.previous);
        assert_eq!(actual.next, expected.next);
        assert!(actual.stats.metadata_nodes <= 6, "{:?}", actual.stats);
    }
    let wrong = q::Request::References(q::references::Request {
        operation: later,
        ..request(
            q::references::Kind::Changes,
            None,
            q::Direction::Forward,
            None,
        )
    });
    assert!(matches!(
        small.read_prepared(access, prepared, wrong).await,
        Err(q::Error::NotCommitted)
    ));
}
