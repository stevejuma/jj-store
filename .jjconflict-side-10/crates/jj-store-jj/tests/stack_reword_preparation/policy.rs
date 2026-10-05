use super::*;
#[tokio::test]
async fn immutable_head_ancestry_unrelated_heads_and_duplicate_policy_fail_before_candidate_writes()
{
    let f = Fixture::new().await;
    stock(&f.root(), &["new", "-m", "Protected descendant"]);
    let ancestor = stock(
        &f.root(),
        &["log", "--no-graph", "-r", "@-", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let operation = stock(
        &f.root(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let expected = jj_store_jj::local_browser::Executor::new(1)
        .unwrap()
        .working_tree_current(f.access(), f.root(), operation)
        .await
        .unwrap();
    let before = f.state();
    let records = |group: &str| {
        let mut names: Vec<_> = std::fs::read_dir(f.root().join(".jj/repo/op_store").join(group))
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        names.sort();
        names
    };
    let operations = records("operations");
    let views = records("views");
    for (heads, error) in [
        (vec![expected.baseline.commit], Error::Denied),
        (vec![m::GitSha1Id::new([91; 20])], Error::NotCommitted),
        (vec![expected.baseline.commit; 2], Error::Invalid),
    ] {
        let lease = Coordinator::new(1)
            .unwrap()
            .acquire(f.access(), f.root(), expected.clone())
            .await
            .unwrap();
        let result = lease
            .prepare_reword(Reword {
                principal: m::PrincipalId::new([85; 16]),
                commit: ancestor,
                message: "Cannot rewrite immutable ancestor".into(),
                immutable_heads: heads,
            })
            .await;
        match result {
            Err(found) => assert_eq!(found, error),
            Ok(_) => panic!("immutable policy must reject before candidate writes"),
        }
        assert_eq!(records("operations"), operations);
        assert_eq!(records("views"), views);
        assert_eq!(f.state(), before);
    }
}
