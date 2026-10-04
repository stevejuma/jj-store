use super::*;

async fn fixture(bytes: &[u8], parents: usize) -> (Fixture, Arc<CommittedRead>, m::CommitId) {
    fixture_dag(bytes, parents, 1).await
}
async fn fixture_dag(
    bytes: &[u8],
    parents: usize,
    generations: usize,
) -> (Fixture, Arc<CommittedRead>, m::CommitId) {
    fixture_tree(bytes, parents, generations, 0, 1).await
}
async fn fixture_tree(
    bytes: &[u8],
    parents: usize,
    generations: usize,
    depth: usize,
    width: usize,
) -> (Fixture, Arc<CommittedRead>, m::CommitId) {
    let f = Fixture::new().await;
    let id = f.objects.put_file(bytes).await.unwrap().0;
    let tree = tree(vec![(
        "file",
        b::TreeValue::File {
            id: b::FileId::from_bytes(id.as_bytes()),
            executable: false,
            copy_id: b::CopyId::placeholder(),
        },
    )]);
    let mut key = f.objects.put(&tree).await.unwrap().0;
    for _ in 0..depth {
        let tree = b::Tree::from_sorted_entries(
            (0..width)
                .map(|i| {
                    (
                        RepoPathComponentBuf::new(format!("dir{i:03}")).unwrap(),
                        b::TreeValue::Tree(b::TreeId::from_bytes(key.as_bytes())),
                    )
                })
                .collect(),
        );
        key = f.objects.put(&tree).await.unwrap().0;
    }
    let mut commit = roots::commit();
    commit.parents = vec![b::CommitId::from_bytes(roots::ROOT_COMMIT.as_bytes())];
    commit.root_tree = Merge::resolved(b::TreeId::from_bytes(key.as_bytes()));
    for generation in 0..generations {
        let mut ids = vec![];
        for i in 0..parents {
            commit.change_id =
                b::ChangeId::from_bytes(&((generation * parents + i + 1) as u128).to_le_bytes());
            commit.description = format!("generation {generation} parent {i}");
            let key = f.objects.put(&commit).await.unwrap().0;
            ids.push(b::CommitId::from_bytes(key.as_bytes()));
        }
        if !ids.is_empty() {
            commit.parents = ids;
        }
    }
    commit.change_id = b::ChangeId::from_bytes(&[99; 16]);
    commit.description = "predicate target".into();
    let key = f.objects.put(&commit).await.unwrap().0;
    let id = m::CommitId::from_slice(key.as_bytes()).unwrap();
    let mut view = roots::view();
    view.head_ids = [b::CommitId::from_bytes(id.as_bytes())].into();
    view.wc_commit_ids
        .insert("fixture".into(), b::CommitId::from_bytes(id.as_bytes()));
    let key = f.objects.put(&view).await.unwrap().0;
    let mut op = o::Operation::make_root(o::ViewId::from_bytes(key.as_bytes()));
    op.parents = vec![o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())];
    let key = f.objects.put(&op).await.unwrap().0;
    f.session
        .publish(
            BTreeSet::from([roots::ROOT_OPERATION]),
            m::OperationId::from_slice(key.as_bytes()).unwrap(),
        )
        .await
        .unwrap();
    let OpenRead::Pinned(read) = f
        .session
        .open_read(None, ReadLimits::default())
        .await
        .unwrap()
    else {
        panic!("sole head")
    };
    (f, Arc::new(read), id)
}
fn log(revset: impl Into<String>) -> q::Request {
    q::Request::Log {
        revset: revset.into(),
        workspace: Some("fixture".into()),
        user_email: String::new(),
        at_time_millis: 1_790_000_000_000,
    }
}
async fn graph(read: &Arc<CommittedRead>, expression: String) -> q::Result<q::GraphPage> {
    let cache = tempfile::tempdir().unwrap();
    let graph = q::GraphExecutor::new(cache.path().to_owned(), q::Limits::default(), 1).unwrap();
    read.graph(
        &graph,
        log(expression),
        q::Page {
            limit: 100,
            after: None,
        },
    )
    .await
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_predicates_bound_file_bytes_before_materialization() {
    let (_, read, id) = fixture(&vec![b'x'; 300_000], 0).await;
    assert_eq!(
        graph(&read, format!("{id} & diff_lines(needle)"))
            .await
            .unwrap_err(),
        q::Error::Limit
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_predicates_bound_line_comparison_work_before_evaluation() {
    let (_, read, id) = fixture("x\n".repeat(2000).as_bytes(), 0).await;
    assert_eq!(
        graph(&read, format!("{id} & diff_lines(needle)"))
            .await
            .unwrap_err(),
        q::Error::Limit
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_predicates_bound_derived_merge_terms_without_limiting_plain_log() {
    let (f, read, id) = fixture(b"needle\n", 34).await;
    let before = f.session.snapshot().await.unwrap().manifest().clone();
    assert_eq!(
        graph(&read, format!("{id} & files(all())"))
            .await
            .unwrap_err(),
        q::Error::Limit
    );
    let page = graph(&read, id.to_string()).await.unwrap();
    let q::Nodes::Log(nodes) = page.data else {
        panic!("log")
    };
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].commit.parents.len(), 34);
    assert_eq!(f.session.snapshot().await.unwrap().manifest(), &before);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_predicates_preserve_admitted_file_and_recursive_merge_semantics() {
    for (parents, generations) in [(0, 1), (2, 1), (2, 3), (3, 2)] {
        let (f, read, id) = fixture_dag(b"needle\nother\n", parents, generations).await;
        let before = f.session.snapshot().await.unwrap().manifest().clone();
        for predicate in [
            "files(all())",
            "diff_lines(needle)",
            "~empty()",
            "present(diff_lines(needle)) | none()",
        ] {
            let expression = format!("{id} & ({predicate})");
            let expected = native_oracle(&f, &read, &expression).await;
            let page = graph(&read, expression).await.unwrap();
            let q::Nodes::Log(nodes) = page.data else {
                panic!("log")
            };

            assert_eq!(
                nodes.iter().map(|n| n.commit.id).collect::<Vec<_>>(),
                expected,
                "{parents} x {generations}: {predicate}"
            );
            if parents == 0 && predicate.contains("diff_lines") {
                assert_eq!(page.stats.predicate_file_bytes, 13);
                assert_eq!(page.stats.predicate_lines, 3);
            }
        }
        assert_eq!(f.session.snapshot().await.unwrap().manifest(), &before);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_predicates_reject_recursive_criss_cross_expansion() {
    let (_, read, id) = fixture_dag(b"needle\n", 3, 8).await;
    assert_eq!(
        graph(&read, format!("{id} & files(all())"))
            .await
            .unwrap_err(),
        q::Error::Limit
    );
    graph(&read, id.to_string()).await.unwrap();
}

async fn native_oracle(
    f: &Fixture,
    read: &Arc<CommittedRead>,
    expression: &str,
) -> Vec<m::CommitId> {
    tokio::task::LocalSet::new()
        .run_until(async {
            use jj_lib::{ref_name::WorkspaceName, revset, ui_path::RepoPathUiConverter};
            let cache = tempfile::tempdir().unwrap();
            f.stores
                .initialize_cache(&settings(), cache.path())
                .await
                .unwrap();
            let loader = f.stores.load_cache(&settings(), cache.path()).unwrap();
            let repo = f
                .stores
                .load_at(&loader, q::ReadAccess::operation(read.as_ref()))
                .await
                .unwrap();
            let extensions = revset::RevsetExtensions::default();
            let converter = RepoPathUiConverter::Fs {
                cwd: "/__jj_store_query_root__".into(),
                base: "/__jj_store_query_root__".into(),
            };
            let context = revset::RevsetParseContext {
                aliases_map: &Default::default(),
                local_variables: Default::default(),
                user_email: "",
                date_pattern_context: jj_lib::time_util::DatePatternContext::Fixed(
                    "2026-09-21T14:13:20+00:00".parse().unwrap(),
                ),
                default_ignored_remote: None,
                fileset_aliases_map: &Default::default(),
                extensions: &extensions,
                workspace: Some(revset::RevsetWorkspaceContext {
                    path_converter: &converter,
                    workspace_name: WorkspaceName::new("fixture"),
                }),
            };
            let parsed = revset::parse(&mut Default::default(), expression, &context).unwrap();
            let resolved = parsed
                .resolve_user_expression(
                    repo.as_ref(),
                    &revset::SymbolResolver::new(repo.as_ref(), extensions.symbol_resolvers()),
                )
                .unwrap();
            let set = resolved.evaluate(repo.as_ref()).unwrap();
            set.stream_graph()
                .map_ok(|(id, _)| cid(&id))
                .try_collect()
                .await
                .unwrap()
        })
        .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_predicates_bound_recursive_tree_depth_for_graphs_and_content() {
    let (_, read, id) = fixture_tree(b"needle\n", 0, 1, 70, 1).await;
    assert_eq!(
        graph(&read, format!("{id} & diff_lines(needle)"))
            .await
            .unwrap_err(),
        q::Error::Limit
    );
    assert_eq!(
        read.diff(
            &reader(),
            c::diff::Request {
                from: roots::ROOT_COMMIT,
                to: id,
                path: String::new(),
                content: false
            }
        )
        .await
        .unwrap_err(),
        q::Error::Limit
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_predicates_admit_boundary_files_and_charge_repeated_tree_paths() {
    let mut bytes = vec![b'x'; 256 * 1024];
    bytes[..7].copy_from_slice(b"needle\n");
    let len = bytes.len();
    bytes[len - 899..].fill(b'\n');
    let (_, read, id) = fixture(&bytes, 0).await;
    for predicate in ["diff_lines(needle)", "diff_lines_added(needle)"] {
        let result = graph(&read, format!("{id} & {predicate}")).await.unwrap();
        let q::Nodes::Log(nodes) = result.data else {
            panic!("log")
        };
        assert_eq!(
            nodes.iter().map(|n| n.commit.id).collect::<Vec<_>>(),
            vec![id]
        );
        assert_eq!(result.stats.predicate_file_bytes, 256 * 1024);
        assert_eq!(result.stats.predicate_lines, 901);
    }
    // Distinct logical paths reuse one immutable file and one tree object. The
    // native cache key includes the path; each materialization must be charged.
    for width in [16, 17] {
        let (_, read, id) = fixture_tree(&vec![b'x'; 256 * 1024], 0, 1, 1, width).await;
        let result = graph(&read, format!("{id} & diff_lines(needle)")).await;
        if width == 16 {
            let result = result.unwrap();
            assert_eq!(result.stats.predicate_file_bytes, 4 * 1024 * 1024);
            assert_eq!(result.stats.predicate_lines, 16);
            let q::Nodes::Log(nodes) = result.data else {
                panic!("log")
            };
            assert!(nodes.is_empty());
        } else {
            assert_eq!(result.unwrap_err(), q::Error::Limit);
        }
    }
}
