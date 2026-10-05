use super::*;
#[tokio::test]
async fn distinct_target_tree_and_other_workspace_selection_survive_setup() {
    let (f, _, target) = target_fixture().await;
    std::fs::write(f.root().join("file.txt"), "original\n").unwrap();
    stock(&f.root(), &["new", "-m", "Original unfinished commit"]);
    std::fs::write(f.root().join("file.txt"), "snapshotted WIP baseline\n").unwrap();
    stock(&f.root(), &["describe", "-m", "Original unfinished commit"]);
    let other = tempfile::tempdir().unwrap();
    let path = other.path().join("workspace");
    stock(
        &f.root(),
        &[
            "workspace",
            "add",
            "--name",
            "other",
            path.to_str().unwrap(),
        ],
    );
    stock(&f.root(), &["describe", "-m", "Original unfinished commit"]);
    let operation = stock(
        &f.root(),
        &[
            "--ignore-working-copy",
            "op",
            "log",
            "--no-graph",
            "-n",
            "1",
            "-T",
            "id",
        ],
    )
    .parse()
    .unwrap();
    std::fs::write(f.root().join("file.txt"), "unsnapshotted dirty bytes\n").unwrap();
    let current = q::Executor::new(1)
        .unwrap()
        .working_tree_current(f.access(), f.root(), operation)
        .await
        .unwrap();
    let before = f.state();
    let physical = guard(0);
    let lease = Coordinator::new(1)
        .unwrap()
        .acquire(f.access(), f.root(), current)
        .await
        .unwrap();
    let (_, prepared) = lease
        .prepare_setup(
            Setup {
                principal: m::PrincipalId::new([91; 16]),
                target,
            },
            physical.clone(),
            physical.expected,
        )
        .await
        .unwrap();
    assert_ne!(
        prepared.capsule.commits[&prepared.merge].root_tree,
        prepared.capsule.commits[&prepared.wip].root_tree
    );
    assert_eq!(
        prepared.capsule.view.wc_commit_ids["other"],
        prepared.source.view.wc_commit_ids["other"]
    );
    assert_eq!(f.state(), before);
}
#[tokio::test]
async fn setup_resolves_only_an_exact_already_known_remote_bookmark() {
    let f = Fixture::new().await;
    f.witness().await;
    let remote = Fixture::new().await;
    std::fs::write(remote.root().join("file.txt"), "original\n").unwrap();
    stock(&remote.root(), &["bookmark", "create", "trunk", "-r", "@"]);
    stock(
        &f.root(),
        &[
            "--ignore-working-copy",
            "git",
            "remote",
            "add",
            "origin",
            remote.root().to_str().unwrap(),
        ],
    );
    std::fs::write(f.root().join("file.txt"), "original\n").unwrap();
    stock(&f.root(), &["git", "fetch", "--remote", "origin"]);
    let head = stock(
        &f.root(),
        &[
            "--ignore-working-copy",
            "log",
            "--no-graph",
            "-r",
            "trunk@origin",
            "-T",
            "commit_id",
        ],
    )
    .parse()
    .unwrap();
    let operation = stock(
        &f.root(),
        &[
            "--ignore-working-copy",
            "op",
            "log",
            "--no-graph",
            "-n",
            "1",
            "-T",
            "id",
        ],
    )
    .parse()
    .unwrap();
    std::fs::write(f.root().join("file.txt"), "dirty external edit\n").unwrap();
    let current = q::Executor::new(1)
        .unwrap()
        .working_tree_current(f.access(), f.root(), operation)
        .await
        .unwrap();
    let before = f.state();
    let physical = guard(0);
    let lease = Coordinator::new(1)
        .unwrap()
        .acquire(f.access(), f.root(), current)
        .await
        .unwrap();
    let (_, prepared) = lease
        .prepare_setup(
            Setup {
                principal: m::PrincipalId::new([91; 16]),
                target: SetupBookmark {
                    name: "trunk".into(),
                    remote: Some("origin".into()),
                    head,
                },
            },
            physical.clone(),
            physical.expected,
        )
        .await
        .unwrap();
    assert_eq!(
        prepared.capsule.view.remote_views,
        prepared.source.view.remote_views
    );
    assert_eq!(prepared.capsule.commits[&prepared.merge].parents, [head]);
    assert_eq!(f.state(), before);
}
