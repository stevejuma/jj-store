use super::*;
use jj_store_service::pagination as p;

fn operations(query: &str) -> q::Request {
    q::Request::OperationsFiltered {
        query: query.into(),
    }
}
fn operation_nodes(page: &q::GraphPage) -> &[q::OperationNode] {
    let q::Nodes::Operations(nodes) = &page.data else {
        panic!("operations")
    };
    nodes
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn history_queries_filter_sparse_ancestry_before_paging_and_never_publish() {
    tokio::task::LocalSet::new().run_until(async {
        let f = Fixture::new().await;
        let cache = tempfile::tempdir().unwrap();
        let work = tempfile::tempdir().unwrap();
        let graphs = tempfile::tempdir().unwrap();
        let (repo, a) = initial(&f, cache.path(), work.path()).await;
        let (repo, b) = child(&repo, &a, "KEEP first").await;
        let first_id = oid(repo.op_id());
        let (repo, c) = child(&repo, &b, "noise").await;
        let skipped_id = oid(repo.op_id());
        let (repo, d) = child(&repo, &c, "keep second").await;
        let second_id = oid(repo.op_id());
        let (repo, e) = child(&repo, &d, "noise again").await;
        let read = pin(&f, oid(repo.op_id())).await;
        let exec = executor(graphs.path(), q::Limits::default());
        let before = f.session.snapshot().await.unwrap().manifest().clone();
        let query = operations("description:keep snapshot:false after:1970-01-01 before:2100-01-01");
        let one = read.graph(&exec, query.clone(), page(1)).await.unwrap();
        assert_eq!(operation_nodes(&one)[0].id, second_id);
        // Filter gaps retain real parent IDs, not fabricated links to matching rows.
        assert_eq!(operation_nodes(&one)[0].parents, vec![skipped_id]);
        assert!(!operation_nodes(&one)[0].parents.contains(&first_id));
        let two = read.graph(&exec, query.clone(), q::Page { limit: 1, after: one.next }).await.unwrap();
        assert_eq!(operation_nodes(&two)[0].id, first_id);
        assert_eq!(two.next, None);
        let from = read.graph(&exec, operations("from:@-- description:keep"), page(100)).await.unwrap();
        assert_eq!(operation_nodes(&from).iter().map(|n| n.id).collect::<Vec<_>>(), vec![first_id]);
        let id = read.graph(&exec, operations(&format!("id:{}", &second_id.to_string()[..20])), page(100)).await.unwrap();
        assert_eq!(operation_nodes(&id)[0].id, second_id);
        let none = read.graph(&exec, operations("user:__missing_history_actor__"), page(100)).await.unwrap();
        assert!(operation_nodes(&none).is_empty());
        for alias in ["trunk()", "immutable()", "mutable()", "visible()", "builtin_immutable_heads()"] {
            let result = read.graph(&exec, log(alias), page(100)).await.unwrap();
            if alias == "trunk()" || alias == "immutable()" { assert_eq!(commits(&result)[0].commit.id, roots::ROOT_COMMIT); }
            if alias == "mutable()" { assert!(!commits(&result).iter().any(|n| n.commit.id == roots::ROOT_COMMIT)); }
        }
        assert!(matches!(read.graph(&exec, log("@"), page(10)).await.unwrap_err(), q::Error::InvalidQuery(q::language::Diagnostic { kind, .. }) if kind == "missing_context"));
        assert!(matches!(read.graph(&exec, operations("description:\"unfinished"), page(10)).await.unwrap_err(), q::Error::InvalidQuery(_)));
        assert!(matches!(read.graph(&exec, operations("from:@+"), page(10)).await.unwrap_err(), q::Error::InvalidQuery(_)));
        let bounded = executor(graphs.path(), q::Limits { max_work: 1, ..q::Limits::default() });
        assert_eq!(read.graph(&bounded, operations("description:missing"), page(100)).await.unwrap_err(), q::Error::Limit);
        let pager = p::GraphPager::new(exec, p::Limits::default(), Arc::new(SystemClock)).unwrap();
        let p::Start::Page(first) = pager.begin(f.session.clone(), Some(oid(repo.op_id())), query.clone(), 1).await.unwrap() else { panic!() };
        let cursor = first.next_cursor.unwrap();
        assert!(pager.resume(f.session.clone(), &cursor, None, operations("description:noise"), 1).await.is_err());
        let continued = pager.resume(f.session.clone(), &cursor, None, query, 1).await.unwrap();
        assert_eq!(continued.snapshot, first.snapshot);
        assert_eq!(operation_nodes(&continued.graph)[0].id, first_id);
        assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
        let _ = e;
    }).await;
}
