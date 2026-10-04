use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::Cancellation};
use std::{fs, sync::Arc};

#[tokio::test]
async fn prepared_local_visibility_preserves_divergence_offsets_and_ten_thousand_bookmarks() {
    use jj_lib::object_id::ObjectId as _;
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    jj(source.path(), &["describe", "-m", "Original"]);
    jj(source.path(), &["bookmark", "create", "branch"]);
    let before = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    );
    jj(source.path(), &["describe", "-m", "Left"]);
    jj(
        source.path(),
        &["--at-op", before.trim(), "describe", "-m", "Right"],
    );
    jj(source.path(), &["log", "--no-graph", "-r", "all()"]);
    let metadata = source.path().join(".jj/repo");
    let settings =
        jj_lib::settings::UserSettings::from_config(jj_lib::config::StackedConfig::with_defaults())
            .unwrap();
    let loader = jj_lib::repo::RepoLoader::init_from_file_system(
        &settings,
        &metadata,
        &jj_lib::default_backend_factories::default_backend_factories(),
    )
    .unwrap();
    let repo = loader.load_at_head().await.unwrap();
    let selected = repo.view().wc_commit_ids().values().next().unwrap().clone();
    let target = jj_lib::op_store::RefTarget::normal(selected.clone());
    let conflict = repo.view().get_local_bookmark("branch".as_ref()).clone();
    assert!(conflict.has_conflict());
    let mut transaction = repo.start_transaction();
    for number in 0..10_000 {
        transaction.repo_mut().set_local_bookmark_target(
            format!("many/{number:05}").as_str().as_ref(),
            target.clone(),
        );
    }
    for (name, remote) in [("zeta", "a"), ("alpha", "z"), ("alpha", "a")] {
        transaction.repo_mut().set_remote_bookmark(
            jj_lib::ref_name::RemoteRefSymbol {
                name: name.as_ref(),
                remote: remote.as_ref(),
            },
            jj_lib::op_store::RemoteRef {
                target: conflict.clone(),
                state: jj_lib::op_store::RemoteRefState::Tracked,
            },
        );
    }
    let repo = transaction.commit("Many reference fixture").await.unwrap();
    let operation =
        jj_store_model::OperationId::from_slice(repo.operation().id().as_bytes()).unwrap();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let executor = q::Executor::new(1).unwrap();
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
    // Use the same cold index ordering for the upstream oracle, including hidden
    // offsets when two rewrites have the same commit timestamp.
    for directory in ["segments", "op_links"] {
        for entry in fs::read_dir(prepared.path().join(directory)).unwrap() {
            let entry = entry.unwrap();
            fs::copy(
                entry.path(),
                metadata
                    .join("index")
                    .join(directory)
                    .join(entry.file_name()),
            )
            .unwrap();
        }
    }
    let window = q::GraphWindow {
        limit: 10,
        after: None,
    };
    let change = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@", "-T", "change_id"],
    );
    for expression in [
        "all()".to_owned(),
        format!("{}/0", change.trim()),
        format!("{}/1", change.trim()),
        format!("{}/2", change.trim()),
    ] {
        let request = q::GraphRequest::Log {
            revset: expression.clone(),
            workspace: None,
            user_email: String::new(),
            at_time_millis: 0,
        };
        let q::Response::Graph(expected) = executor
            .read(
                access.clone(),
                operation,
                q::Request::Graph(request.clone(), window.clone()),
            )
            .await
            .unwrap()
        else {
            panic!()
        };
        let actual = executor
            .graph_prepared(access.clone(), prepared.clone(), request, window.clone())
            .await
            .unwrap();
        assert_eq!(actual.data, expected.data, "{expression}");
        assert_eq!(actual.next, expected.next);
        if expression == "all()" {
            let q::Nodes::Log(rows) = actual.data else {
                panic!()
            };
            assert!(rows.iter().any(|row| row.commit.divergent));
            let selected = rows
                .iter()
                .find(|row| row.commit.id.as_bytes() == selected.as_bytes())
                .unwrap();
            assert_eq!(
                selected
                    .commit
                    .bookmarks
                    .iter()
                    .filter(|bookmark| bookmark.name.starts_with("many/"))
                    .count(),
                10_000
            );
            assert!(
                rows.iter()
                    .flat_map(|row| &row.commit.bookmarks)
                    .any(|bookmark| bookmark.conflicted && bookmark.remote.is_some())
            );
        }
    }
}

#[tokio::test]
async fn prepared_local_details_preserve_hidden_rewrites_without_history_preflight() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    for number in 0..12 {
        jj(source.path(), &["new", "-m", &format!("Older {number}")]);
    }
    fs::write(source.path().join("selected.txt"), "selected\n").unwrap();
    jj(source.path(), &["describe", "-m", "Original"]);
    let original = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    jj(source.path(), &["describe", "-m", "Rewritten"]);
    jj(source.path(), &["bookmark", "create", "selected"]);
    let selected = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let operation = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let request = q::GraphRequest::Evolution { commit: selected };
    let window = q::GraphWindow {
        limit: 2,
        after: None,
    };
    let executor = q::Executor::new(1).unwrap();
    let q::Response::Graph(expected) = executor
        .read(
            access.clone(),
            operation,
            q::Request::Graph(request.clone(), window.clone()),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let mut details = vec![];
    for commit in [selected, original, jj_store_model::GitSha1Id::new([0; 20])] {
        let q::Response::Commit(expected) = executor
            .read(access.clone(), operation, q::Request::Commit { commit })
            .await
            .unwrap()
        else {
            panic!()
        };
        details.push((commit, expected));
    }
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
            max_history_visits: 8,
            ..Default::default()
        },
    );
    assert!(matches!(
        small
            .read(
                access.clone(),
                operation,
                q::Request::Commit { commit: selected }
            )
            .await,
        Err(q::Error::Limit)
    ));
    // Later heads cannot change this exact generation's visibility or labels.
    jj(source.path(), &["new", "-m", "Later"]);
    let later = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    for entry in fs::read_dir(source.path().join(".jj/repo/index/segments")).unwrap() {
        fs::write(entry.unwrap().path(), b"unavailable source index").unwrap();
    }
    fs::remove_dir_all(prepared.path().join("segments")).unwrap();
    for (commit, expected) in details {
        let q::Response::Commit(actual) = small
            .read_prepared(
                access.clone(),
                prepared.clone(),
                q::Request::Commit { commit },
            )
            .await
            .unwrap()
        else {
            panic!()
        };
        assert_eq!(actual.commit, expected.commit);
        assert_eq!(
            serde_json::to_value(actual.object.data).unwrap(),
            serde_json::to_value(expected.object.data).unwrap()
        );
        assert_eq!(actual.object.virtual_object, expected.object.virtual_object);
        assert!(
            actual.object.stats.metadata_nodes < 12,
            "{:?}",
            actual.object.stats
        );
    }
    assert!(matches!(
        small
            .read_prepared(
                access.clone(),
                prepared.clone(),
                q::Request::Commit { commit: later }
            )
            .await,
        Err(q::Error::NotCommitted)
    ));
    let actual = small
        .graph_prepared(access.clone(), prepared.clone(), request, window)
        .await
        .unwrap();
    assert_eq!(actual.data, expected.data);
    let q::Nodes::Evolution(nodes) = actual.data else {
        panic!()
    };
    assert_eq!(nodes[0].commit.id, selected);
    assert!(nodes[0].commit.visible);
    assert_eq!(nodes[1].commit.id, original);
    assert!(!nodes[1].commit.visible);
    assert_eq!(nodes[0].commit.bookmarks[0].name, "selected");
    assert_eq!(nodes[0].commit.workspaces, ["default"]);
    assert!(actual.stats.metadata_nodes < 20, "{:?}", actual.stats);
    // SimpleOpStore reads valid protobuf without checking its name against its
    // contents. A prepared generation must reject a different valid view.
    let q::Response::Operation(detail) = executor
        .read(
            access.clone(),
            operation,
            q::Request::Operation { operation },
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let q::ObjectKey::View(view) = detail.view.key else {
        panic!()
    };
    let path = access
        .metadata_path()
        .join("op_store/views")
        .join(view.to_string());
    let selected_bytes = fs::read(&path).unwrap();
    let replacement = fs::read_dir(path.parent().unwrap())
        .unwrap()
        .find_map(|entry| {
            let entry = entry.unwrap();
            let bytes = fs::read(entry.path()).unwrap();
            (bytes != selected_bytes).then_some(bytes)
        })
        .unwrap();
    fs::write(&path, replacement).unwrap();
    assert!(matches!(
        small
            .read_prepared(
                access.clone(),
                prepared.clone(),
                q::Request::Commit { commit: selected }
            )
            .await,
        Err(q::Error::Corrupt)
    ));
    fs::write(&path, selected_bytes).unwrap();
    for (generation, denied, expected) in [
        (1, true, q::Error::Denied),
        (2, false, q::Error::EpochChanged),
    ] {
        let fresh = Arc::new(ChangedBinding {
            path: access.metadata_path().into(),
            generation,
            denied,
        });
        assert!(matches!(small.read_prepared(fresh, prepared.clone(),
            q::Request::Commit { commit: selected }).await, Err(error) if error == expected));
    }
}

struct ChangedBinding {
    path: std::path::PathBuf,
    generation: u64,
    denied: bool,
}
#[async_trait::async_trait]
impl q::Access for ChangedBinding {
    fn repository(&self) -> jj_store_model::RepositoryId {
        jj_store_model::RepositoryId::new([20; 16])
    }
    fn generation(&self) -> u64 {
        self.generation
    }
    fn metadata_path(&self) -> &std::path::Path {
        &self.path
    }
    async fn authorize(&self) -> q::Result<()> {
        if self.denied {
            Err(q::Error::Denied)
        } else {
            Ok(())
        }
    }
}
