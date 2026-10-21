use super::*;
use jj_store_model::OperationId;
use std::{collections::BTreeMap, fs};
pub(super) fn source_bytes(path: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, path: &Path, values: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                visit(root, &entry.path(), values);
            } else if entry.file_type().unwrap().is_file() {
                values.insert(
                    entry.path().strip_prefix(root).unwrap().into(),
                    fs::read(entry.path()).unwrap(),
                );
            }
        }
    }
    let mut values = BTreeMap::new();
    visit(path, path, &mut values);
    values
}
fn poison_source_indexes(path: &Path) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            poison_source_indexes(&entry.path());
        } else if entry.file_name() != "type" {
            fs::write(entry.path(), b"unreadable source-derived index").unwrap();
        }
    }
}
fn graph(after: Option<local::ObjectKey>, limit: usize) -> local::Request {
    local::Request::Graph(
        local::GraphRequest::Operations,
        local::GraphWindow { limit, after },
    )
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn prepared_operation_pages_and_parent_deltas_preserve_source_and_pin() {
    let (root, policy) = fixture();
    let cache = tempfile::tempdir().unwrap();
    let host = History::new(policy, cache.path().into(), Config::default()).unwrap();
    let main = caller("main");
    let operation = host.heads(&main, "vault").await.unwrap().heads[0];
    let handle = host.open(&main, "vault", operation).await.unwrap();
    ready(&host, &main, handle).await;
    jj(root.path(), &["describe", "-m", "external future"]);
    fs::write(root.path().join("note.md"), "dirty unsnapshotted content\n").unwrap();
    let future = host.heads(&main, "vault").await.unwrap().heads[0];
    assert_ne!(future, operation);
    poison_source_indexes(&root.path().join(".jj/repo/index"));
    let before = source_bytes(root.path());
    let local::Response::Graph(first) = host
        .read(&main, "vault", handle, graph(None, 1))
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(first.operation, operation);
    let local::Nodes::Operations(nodes) = first.data else {
        panic!()
    };
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].id, operation);
    assert!(!nodes[0].parents.is_empty());
    let parent = nodes[0].parents[0];
    let local::Response::Graph(next) = host
        .read(&main, "vault", handle, graph(first.next, 100))
        .await
        .unwrap()
    else {
        panic!()
    };
    let local::Nodes::Operations(ancestors) = next.data else {
        panic!()
    };
    assert!(ancestors.iter().any(|node| node.id == parent));
    assert!(!ancestors.iter().any(|node| node.id == future));
    let request = local::Request::OperationDiff(local::operations::Request {
        operation,
        parent: Some(parent),
    });
    let local::Response::OperationDiff(delta) =
        host.read(&main, "vault", handle, request).await.unwrap()
    else {
        panic!()
    };
    let local::operations::Data::Difference(delta) = delta.data else {
        panic!()
    };
    assert_eq!(delta.operation, operation);
    assert_eq!(delta.parent, parent);
    let local::Response::OperationChanges(changes) = host
        .read(
            &main,
            "vault",
            handle,
            local::Request::OperationChanges(local::operation_changes::Request {
                operation,
                parent,
                limit: 100,
                anchor: None,
                direction: local::Direction::Forward,
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(changes.operation, operation);
    assert_eq!(changes.parent, parent);

    assert_eq!(
        source_bytes(root.path()),
        before,
        "read-only operation inspection touched the source"
    );
    let foreign = local::Request::OperationDiff(local::operations::Request {
        operation: future,
        parent: None,
    });
    assert!(matches!(
        host.read(&main, "vault", handle, foreign).await,
        Err(Error::NotCommitted)
    ));
    let invalid = local::Request::OperationDiff(local::operations::Request {
        operation,
        parent: Some(OperationId::new([99; 64])),
    });
    assert!(matches!(
        host.read(&main, "vault", handle, invalid).await,
        Err(Error::Invalid)
    ));
    let root = ancestors
        .iter()
        .find(|node| node.parents.is_empty())
        .unwrap();
    let local::Response::OperationDiff(delta) = host
        .read(
            &main,
            "vault",
            handle,
            local::Request::OperationDiff(local::operations::Request {
                operation: root.id,
                parent: None,
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(matches!(delta.data, local::operations::Data::Root { .. }));
    assert!(
        matches!(host.status(&main,"vault",handle).await.unwrap(),Status::Ready(pin) if pin.operation==operation)
    );
    host.shutdown().await.unwrap();
}
