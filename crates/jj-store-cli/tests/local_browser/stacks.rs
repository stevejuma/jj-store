use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::Cancellation};
use std::{collections::BTreeMap, fs, path::Path, sync::Arc};

fn snapshot(root: &Path) -> BTreeMap<std::path::PathBuf, Vec<u8>> {
    fn visit(root: &Path, path: &Path, out: &mut BTreeMap<std::path::PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let kind = entry.file_type().unwrap();
            if kind.is_dir() {
                visit(root, &entry.path(), out);
            } else if kind.is_file() {
                out.insert(
                    entry.path().strip_prefix(root).unwrap().into(),
                    fs::read(entry.path()).unwrap(),
                );
            }
        }
    }
    let mut out = BTreeMap::new();
    visit(root, root, &mut out);
    out
}
#[tokio::test]
async fn local_stack_history_is_bounded_pinned_and_read_only() {
    let source = tempfile::tempdir().unwrap();
    let scratch = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    fs::write(source.path().join("reverted.txt"), "base\n").unwrap();
    jj(source.path(), &["describe", "-m", "Target"]);
    jj(source.path(), &["bookmark", "create", "trunk"]);
    let base = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let mut commits = vec![];
    for n in 0..5 {
        jj(source.path(), &["new", "-m", &format!("Stack commit {n}")]);
        fs::write(source.path().join("selected.txt"), format!("version {n}\n")).unwrap();
        if n == 1 {
            fs::write(source.path().join("reverted.txt"), "changed\n").unwrap();
        }
        if n == 2 {
            fs::write(source.path().join("reverted.txt"), "base\n").unwrap();
        }
        jj(
            source.path(),
            &["describe", "-m", &format!("Stack commit {n}")],
        );
        commits.push(
            jj(
                source.path(),
                &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
            )
            .parse()
            .unwrap(),
        );
    }
    jj(source.path(), &["bookmark", "create", "feature"]);
    let head = *commits.last().unwrap();
    let operation = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    fs::write(
        source.path().join("unsnapshotted.txt"),
        "do not snapshot or publish\n",
    )
    .unwrap();
    let before = snapshot(source.path());
    let reader = q::Executor::new(1).unwrap();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let prepared = Arc::new(
        reader
            .prepare_index(
                access.clone(),
                operation,
                scratch.path().into(),
                16 * 1024 * 1024,
                q::preparation::Limits::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    let managed_scratch = tempfile::tempdir().unwrap();
    let managed = Arc::new(
        reader
            .prepare_code_index(
                access.clone(),
                operation,
                managed_scratch.path().into(),
                16 * 1024 * 1024,
                Default::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    let mut after = None;
    let mut actual = vec![];
    loop {
        let q::Response::StackHistory(page) = reader
            .read_prepared(
                access.clone(),
                prepared.clone(),
                q::Request::StackHistory(q::stacks::Request {
                    head,
                    base: Some(base),
                    limit: 2,
                    after,
                }),
            )
            .await
            .unwrap()
        else {
            panic!("stack response expected")
        };
        let q::Response::StackHistory(indexed) = reader
            .read_code_index(
                access.clone(),
                managed.clone(),
                q::Request::StackHistory(q::stacks::Request {
                    head,
                    base: Some(base),
                    limit: 2,
                    after,
                }),
            )
            .await
            .unwrap()
        else {
            panic!("managed stack response expected")
        };
        assert_eq!(indexed.commits, page.commits);
        assert_eq!(indexed.next, page.next);
        assert_eq!(indexed.operation, operation);
        assert_eq!(page.operation, operation);
        assert_eq!(page.head, head);
        assert_eq!(page.base, Some(base));
        assert!(page.commits.len() <= 2);
        assert!(page.stats.metadata_nodes < 64, "{:?}", page.stats);
        actual.extend(page.commits.into_iter().map(|node| node.commit.id));
        after = page.next;
        if after.is_none() {
            break;
        }
    }
    commits.reverse();
    assert_eq!(actual, commits);
    let invalid = reader
        .read_prepared(
            access.clone(),
            prepared.clone(),
            q::Request::StackHistory(q::stacks::Request {
                head,
                base: Some(base),
                limit: 2,
                after: Some(base),
            }),
        )
        .await;
    assert!(matches!(invalid, Err(q::Error::Invalid)));
    let empty = reader
        .read_prepared(
            access.clone(),
            prepared.clone(),
            q::Request::StackHistory(q::stacks::Request {
                head: base,
                base: Some(base),
                limit: 2,
                after: None,
            }),
        )
        .await
        .unwrap();
    assert!(
        matches!(empty,q::Response::StackHistory(page) if page.commits.is_empty() && page.next.is_none())
    );
    let q::Response::DiffPaths(net) = reader
        .read_prepared(
            access.clone(),
            prepared.clone(),
            q::Request::DiffPaths(q::diff_summary::Request {
                from: base,
                to: head,
                path: String::new(),
                limit: 10,
                anchor: None,
                direction: Default::default(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(
        net.changes.len(),
        1,
        "branch diff must compare exact boundary trees and exclude reverted paths"
    );
    assert_eq!(net.changes[0].path, "selected.txt");
    let all = reader
        .read_prepared(
            access,
            prepared,
            q::Request::StackHistory(q::stacks::Request {
                head: base,
                base: None,
                limit: 2,
                after: None,
            }),
        )
        .await
        .unwrap();
    assert!(matches!(all,q::Response::StackHistory(page) if page.commits[0].commit.id == base));
    assert_eq!(
        snapshot(source.path()),
        before,
        "reads and private index preparation may not change source or dirty files"
    );
}
