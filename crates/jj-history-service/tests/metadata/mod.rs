use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ready_handles_read_metadata_without_rebuilding_or_advancing_the_source() {
    let (root, policy) = fixture();
    jj(root.path(), &["bookmark", "set", "main", "-r", "@"]);
    let cache = tempfile::tempdir().unwrap();
    let host = History::new(policy, cache.path().into(), Config::default()).unwrap();
    let main = caller("main");
    let operation = host.heads(&main, "vault").await.unwrap().heads[0];
    let handle = host.open(&main, "vault", operation).await.unwrap();
    ready(&host, &main, handle).await;
    jj(root.path(), &["describe", "-m", "future description"]);
    let future = host.heads(&main, "vault").await.unwrap().heads[0];
    let before = operations::source_bytes(root.path());
    let local::Response::QueryInfo(_) = host
        .read(&main, "vault", handle, local::Request::QueryInfo)
        .await
        .unwrap()
    else {
        panic!()
    };
    for kind in [
        local::references::Kind::Workspaces,
        local::references::Kind::Bookmarks,
    ] {
        let query = local::Request::References(local::references::Request {
            operation,
            query: String::new(),
            exact: None,
            kind,
            limit: 100,
            anchor: None,
            direction: local::Direction::Forward,
        });
        let local::Response::References(page) =
            host.read(&main, "vault", handle, query).await.unwrap()
        else {
            panic!()
        };
        assert_eq!(page.operation, operation);
        assert_eq!(page.references.len(), 1);
        let targets = match &page.references[0] {
            local::references::Reference::Workspace { targets, .. }
            | local::references::Reference::Bookmark { targets, .. } => targets,
            _ => panic!(),
        };
        assert_eq!(targets[0].description, "Saved\n");
    }
    let local::Response::Operation(detail) = host
        .read(
            &main,
            "vault",
            handle,
            local::Request::Operation { operation },
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let local::ObjectData::Operation(data) = detail.operation.data else {
        panic!()
    };
    let parent = data.parents[0];
    assert!(matches!(
        host.read(
            &main,
            "vault",
            handle,
            local::Request::Operation { operation: parent }
        )
        .await
        .unwrap(),
        local::Response::Operation(_)
    ));
    assert!(matches!(
        host.read(
            &main,
            "vault",
            handle,
            local::Request::Operation { operation: future }
        )
        .await,
        Err(Error::NotCommitted)
    ));
    assert_eq!(operations::source_bytes(root.path()), before);
    host.shutdown().await.unwrap();
}
