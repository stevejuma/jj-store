use super::*;
use jj_lib::{merge::Merge, repo_path::RepoPathComponentBuf};
use jj_store_jj::query::content::{self as c, ContentReader};
use jj_store_service::pagination::{
    self as paging, SnapshotCursors, browsing as browse, operation_browsing as operation_browse,
};
use jj_store_service::reads::{CommittedRead, OpenRead, ReadLimits};
use std::collections::BTreeMap;

pub(super) struct BrowsingFixture {
    pub(super) f: Fixture,
    read: Arc<CommittedRead>,
    pub(super) operation: m::OperationId,
    metadata_operation: m::OperationId,
    pub(super) base: m::CommitId,
    pub(super) head: m::CommitId,
}
impl BrowsingFixture {
    pub(super) async fn new() -> Self {
        let f = Fixture::new().await;
        let old = f.objects.put_file(&b"old\n"[..]).await.unwrap().0;
        let same = f.objects.put_file(&b"same\n"[..]).await.unwrap().0;
        let new = f.objects.put_file(&b"new\n"[..]).await.unwrap().0;
        let nested = f.objects.put_file(&b"nested\n"[..]).await.unwrap().0;
        let base_tree = tree(vec![
            ("a.txt", file(old)),
            ("b.txt", file(same)),
            ("dir", subtree(&f, vec![("c.txt", file(nested))]).await),
        ]);
        let base_tree = f.objects.put(&base_tree).await.unwrap().0;
        let mut base = b::make_root_commit(
            b::ChangeId::from_bytes(&[21; 16]),
            b::TreeId::from_bytes(base_tree.as_bytes()),
        );
        base.parents = vec![b::CommitId::from_bytes(roots::ROOT_COMMIT.as_bytes())];
        base.description = "base files".into();
        let base_key = f.objects.put(&base).await.unwrap().0;
        let base_id = b::CommitId::from_bytes(base_key.as_bytes());
        let head_tree = tree(vec![
            ("a.txt", file(new)),
            ("b.txt", file(same)),
            ("d.txt", file(new)),
            ("dir", subtree(&f, vec![("c.txt", file(nested))]).await),
        ]);
        let head_tree = f.objects.put(&head_tree).await.unwrap().0;
        let mut head = b::make_root_commit(
            b::ChangeId::from_bytes(&[22; 16]),
            b::TreeId::from_bytes(head_tree.as_bytes()),
        );
        head.parents = vec![base_id.clone()];
        head.predecessors = vec![base_id.clone()];
        head.description = "rewrite files".into();
        let head_key = f.objects.put(&head).await.unwrap().0;
        let head_id = b::CommitId::from_bytes(head_key.as_bytes());
        let mut view = o::View::make_root(head_id.clone());
        view.local_bookmarks
            .insert("main".into(), o::RefTarget::normal(head_id.clone()));
        view.local_bookmarks.insert(
            "conflicted".into(),
            o::RefTarget::from_merge(Merge::from_vec(vec![
                Some(head_id.clone()),
                None,
                Some(base_id.clone()),
            ])),
        );
        view.wc_commit_ids.insert("desktop".into(), head_id.clone());
        let view_key = f.objects.put(&view).await.unwrap().0;
        let mut operation = o::Operation::make_root(o::ViewId::from_bytes(view_key.as_bytes()));
        operation.parents = vec![o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())];
        operation.metadata.description = "rewrite operation".into();
        operation.commit_predecessors =
            Some(BTreeMap::from([(head_id.clone(), vec![base_id.clone()])]));
        let operation_key = f.objects.put(&operation).await.unwrap().0;
        let operation = m::OperationId::from_slice(operation_key.as_bytes()).unwrap();
        f.session
            .publish(BTreeSet::from([roots::ROOT_OPERATION]), operation)
            .await
            .unwrap();
        let mut metadata = o::Operation::make_root(o::ViewId::from_bytes(view_key.as_bytes()));
        metadata.parents = vec![o::OperationId::from_bytes(operation.as_bytes())];
        metadata.metadata.description = "metadata only".into();
        let metadata_key = f.objects.put(&metadata).await.unwrap().0;
        let metadata_operation = m::OperationId::from_slice(metadata_key.as_bytes()).unwrap();
        f.session
            .publish(BTreeSet::from([operation]), metadata_operation)
            .await
            .unwrap();
        let OpenRead::Pinned(read) = f
            .session
            .open_read(Some(operation), ReadLimits::default())
            .await
            .unwrap()
        else {
            panic!("explicit operation pins")
        };
        Self {
            f,
            read: Arc::new(read),
            operation,
            metadata_operation,
            base: m::CommitId::from_slice(base_key.as_bytes()).unwrap(),
            head: m::CommitId::from_slice(head_key.as_bytes()).unwrap(),
        }
    }
}
fn reader() -> ContentReader {
    ContentReader::new(c::Limits::default(), 2).unwrap()
}
fn file(id: m::FileId) -> b::TreeValue {
    b::TreeValue::File {
        id: b::FileId::from_bytes(id.as_bytes()),
        executable: false,
        copy_id: b::CopyId::placeholder(),
    }
}
fn tree(entries: Vec<(&str, b::TreeValue)>) -> b::Tree {
    b::Tree::from_sorted_entries(
        entries
            .into_iter()
            .map(|(name, value)| (RepoPathComponentBuf::new(name).unwrap(), value))
            .collect(),
    )
}
async fn subtree(f: &Fixture, entries: Vec<(&str, b::TreeValue)>) -> b::TreeValue {
    let key = f.objects.put(&tree(entries)).await.unwrap().0;
    b::TreeValue::Tree(b::TreeId::from_bytes(key.as_bytes()))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_browser_windows_preserve_references_paths_groups_and_exact_diffs() {
    let f = BrowsingFixture::new().await;
    let reader = reader();
    let first = f
        .read
        .references(
            &reader,
            c::references::Request {
                exact: None,
                operation: f.operation,
                query: String::new(),
                kind: c::references::Kind::Bookmarks,
                limit: 1,
                anchor: None,
                direction: c::Direction::Forward,
            },
        )
        .await
        .unwrap();
    assert_eq!(first.references.len(), 1);
    let c::references::Reference::Bookmark {
        name,
        conflicted,
        targets,
        ..
    } = &first.references[0]
    else {
        panic!("bookmark row")
    };
    assert_eq!(name, "conflicted");
    assert!(*conflicted);
    assert_eq!(targets.len(), 2);
    let second = f
        .read
        .references(
            &reader,
            c::references::Request {
                exact: None,
                operation: f.operation,
                query: String::new(),
                kind: c::references::Kind::Bookmarks,
                limit: 1,
                anchor: first.next,
                direction: c::Direction::Forward,
            },
        )
        .await
        .unwrap();
    assert!(
        matches!(&second.references[0], c::references::Reference::Bookmark { name, workspaces, .. } if name == "main" && workspaces == &["desktop"])
    );
    assert!(second.previous.is_some());

    let paths = f
        .read
        .paths(
            &reader,
            c::paths::Request {
                include_last_change: true,
                last_change_mode: Default::default(),
                commit: f.head,
                path: String::new(),
                recursive: true,
                limit: 3,
                anchor: None,
                direction: c::Direction::Forward,
            },
        )
        .await
        .unwrap();
    assert_eq!(
        paths
            .entries
            .iter()
            .map(|entry| entry.path.as_str())
            .collect::<Vec<_>>(),
        ["a.txt", "b.txt", "d.txt"]
    );
    assert!(
        matches!(paths.entries[0].last_change, c::paths::LastChange::Available { commit, .. } if commit == f.head)
    );
    assert!(
        matches!(paths.entries[1].last_change, c::paths::LastChange::Available { commit, .. } if commit == f.base)
    );
    assert!(paths.next.is_some());

    let groups = f
        .read
        .operation_changes(
            &reader,
            c::operation_changes::Request {
                operation: f.operation,
                parent: roots::ROOT_OPERATION,
                limit: 50,
                anchor: None,
                direction: c::Direction::Forward,
            },
        )
        .await
        .unwrap();
    assert!(groups.groups.iter().any(|group| matches!(group.comparison, c::operation_changes::Comparison::Exact { from, to } if from == f.base && to == f.head)));
    let OpenRead::Pinned(metadata_read) =
        f.f.session
            .open_read(Some(f.metadata_operation), ReadLimits::default())
            .await
            .unwrap()
    else {
        panic!("metadata operation pins")
    };
    let metadata = Arc::new(metadata_read)
        .operation_changes(
            &reader,
            c::operation_changes::Request {
                operation: f.metadata_operation,
                parent: f.operation,
                limit: 50,
                anchor: None,
                direction: c::Direction::Forward,
            },
        )
        .await
        .unwrap();
    assert!(metadata.groups.is_empty());

    let diff = f
        .read
        .diff_summaries(
            &reader,
            c::diff_summary::Request {
                from: f.base,
                to: f.head,
                path: String::new(),
                limit: 1,
                anchor: None,
                direction: c::Direction::Forward,
            },
        )
        .await
        .unwrap();
    assert_eq!(diff.changes.len(), 1);
    assert_eq!(diff.changes[0].path, "a.txt");
    assert_eq!(
        diff.changes[0].content,
        c::diff::Content::Unavailable {
            reason: c::diff::Unavailable::NotRequested
        }
    );
    assert!(diff.next.is_some());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn signed_browser_cursors_bind_queries_support_backward_windows_and_reset_on_restart() {
    let f = BrowsingFixture::new().await;
    let reader = reader();
    let cursors =
        Arc::new(SnapshotCursors::new(paging::Limits::default(), Arc::new(SystemClock)).unwrap());
    let browser = browse::BrowserPager::with_cursors(reader.clone(), cursors.clone());
    let request = browse::ReferenceRequest {
        exact: None,
        query: String::new(),
        kind: c::references::Kind::Bookmarks,
        limit: 1,
    };
    let browse::Start::Window(first) = browser
        .references(
            f.f.session.clone(),
            Some(f.operation),
            request.clone(),
            None,
            None,
            c::Direction::Forward,
        )
        .await
        .unwrap()
    else {
        panic!("explicit operation returns a window")
    };
    let next = first.next_cursor.clone().unwrap();
    let browse::Start::Window(second) = browser
        .references(
            f.f.session.clone(),
            Some(f.operation),
            request.clone(),
            Some(&next),
            None,
            c::Direction::Forward,
        )
        .await
        .unwrap()
    else {
        panic!("cursor resumes")
    };
    assert_ne!(first.data.references, second.data.references);
    let previous = second.previous_cursor.clone().unwrap();
    let browse::Start::Window(back) = browser
        .references(
            f.f.session.clone(),
            Some(f.operation),
            request.clone(),
            Some(&previous),
            None,
            c::Direction::Forward,
        )
        .await
        .unwrap()
    else {
        panic!("backward cursor resumes")
    };
    assert_eq!(back.data.references, first.data.references);
    for changed in [
        browse::ReferenceRequest {
            kind: c::references::Kind::Workspaces,
            ..request.clone()
        },
        browse::ReferenceRequest {
            exact: Some("main".into()),
            ..request.clone()
        },
    ] {
        assert!(matches!(
            browser
                .references(
                    f.f.session.clone(),
                    Some(f.operation),
                    changed,
                    Some(&next),
                    None,
                    c::Direction::Forward
                )
                .await,
            Err(paging::Error::InvalidCursor)
        ));
    }
    let changed = browse::ReferenceRequest {
        exact: None,
        query: "main".into(),
        ..request.clone()
    };
    assert!(matches!(
        browser
            .references(
                f.f.session.clone(),
                Some(f.operation),
                changed,
                Some(&next),
                None,
                c::Direction::Forward,
            )
            .await,
        Err(paging::Error::InvalidCursor)
    ));
    let restarted = browse::BrowserPager::with_cursors(
        reader.clone(),
        Arc::new(SnapshotCursors::new(paging::Limits::default(), Arc::new(SystemClock)).unwrap()),
    );
    assert!(matches!(
        restarted
            .references(
                f.f.session.clone(),
                Some(f.operation),
                request,
                Some(&next),
                None,
                c::Direction::Forward,
            )
            .await,
        Err(paging::Error::CursorReset)
    ));

    let path_request = browse::PathRequest {
        commit: f.head,
        path: String::new(),
        recursive: false,
        include_last_change: false,
        last_change_mode: Default::default(),
        limit: 1,
    };
    let browse::Start::Window(rows) = browser
        .paths(
            f.f.session.clone(),
            Some(f.operation),
            path_request.clone(),
            None,
            None,
            c::Direction::Forward,
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(matches!(
        rows.data.entries[0].last_change,
        c::paths::LastChange::Unavailable {
            reason: c::paths::LastChangeUnavailable::NotRequested
        }
    ));
    assert!(matches!(
        browser
            .paths(
                f.f.session.clone(),
                Some(f.operation),
                browse::PathRequest {
                    include_last_change: true,
                    last_change_mode: Default::default(),
                    ..path_request
                },
                rows.next_cursor.as_deref(),
                None,
                c::Direction::Forward
            )
            .await,
        Err(paging::Error::InvalidCursor)
    ));

    let operations = operation_browse::OperationBrowserPager::with_cursors(reader, cursors);
    let browse::Start::Window(groups) = operations
        .groups(
            f.f.session,
            operation_browse::GroupRequest {
                operation: f.operation,
                parent: roots::ROOT_OPERATION,
                limit: 1,
            },
            None,
            None,
            c::Direction::Forward,
        )
        .await
        .unwrap()
    else {
        panic!("operation group window")
    };
    assert_eq!(groups.snapshot.operation_id, f.operation);
    assert_eq!(groups.data.groups.len(), 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn direct_directory_rows_include_directories_and_bookmark_pages_hydrate_only_the_window() {
    let f = BrowsingFixture::new().await;
    let page = f
        .read
        .paths(
            &reader(),
            c::paths::Request {
                include_last_change: true,
                last_change_mode: Default::default(),
                commit: f.head,
                path: String::new(),
                recursive: false,
                limit: 20,
                anchor: None,
                direction: Default::default(),
            },
        )
        .await
        .unwrap();
    assert_eq!(
        page.entries
            .iter()
            .map(|entry| entry.path.as_str())
            .collect::<Vec<_>>(),
        ["a.txt", "b.txt", "d.txt", "dir"]
    );
    assert!(matches!(
        page.entries[3].terms[0],
        Some(jj_store_jj::native_dto::TreeValue::Tree { .. })
    ));
    let page = f
        .read
        .references(
            &reader(),
            c::references::Request {
                exact: None,
                operation: f.operation,
                query: String::new(),
                kind: c::references::Kind::Bookmarks,
                limit: 1,
                anchor: Some("bookmark:conflicted".into()),
                direction: Default::default(),
            },
        )
        .await
        .unwrap();
    assert_eq!(page.references.len(), 1);
    // Metadata nodes count JSON values; object_reads measures the actual
    // operation, view and selected target reads, excluding other bookmarks.
    assert_eq!(page.stats.object_reads, 3, "{:?}", page.stats);
}

#[path = "reference_windows.rs"]
mod reference_windows;

#[path = "reference_filters.rs"]
mod reference_filters;

#[path = "reference_embedded.rs"]
mod reference_embedded;
