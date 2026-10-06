use super::*;
async fn inventory(
    root: &Path,
    operation: m::OperationId,
    target: String,
) -> local_stacks::TopologyResponse {
    local_stacks::read_topology(
        Arc::new(Access(root.join(".jj/repo"))),
        root.canonicalize().unwrap(),
        local_stacks::TopologyRequest {
            version: 1,
            binding_generation: 1,
            operation,
            target: "main".into(),
            target_commit: target,
        },
    )
    .await
    .unwrap()
}
fn id(root: &Path, rev: &str) -> String {
    stock(root, &["log", "-r", rev, "--no-graph", "-T", "commit_id"])
}
#[tokio::test]
async fn discovery_is_read_only_before_setup_and_retains_shared_dependent_boundaries() {
    let temp = fixture();
    let root = temp.path();
    let base = id(root, "main");
    let lower = id(root, "topic-one");
    stock(
        root,
        &["bookmark", "create", "workspace/alias", "-r", "topic-one"],
    );
    stock(root, &["new", "topic-one", "-m", "upper"]);
    stock(root, &["bookmark", "create", "upper"]);
    stock(root, &["new", "topic-one", "-m", "sibling"]);
    stock(root, &["bookmark", "create", "sibling"]);
    stock(root, &["new", "main", "-m", "target advance"]);
    stock(root, &["bookmark", "set", "main"]);
    let target = id(root, "main");
    let op: m::OperationId = stock(root, &["op", "log", "-n", "1", "--no-graph", "-T", "id"])
        .parse()
        .unwrap();
    let before = fs::read(root.join(".jj/working_copy/checkout")).unwrap();
    let result = inventory(root, op, target.clone()).await;
    assert!(result.topology.complete && result.topology.lanes.is_empty());
    assert!(!root.join(".jj/jj-store/stacks.toml").exists());
    assert_eq!(
        before,
        fs::read(root.join(".jj/working_copy/checkout")).unwrap()
    );
    assert_eq!(
        stock(root, &["op", "log", "-n", "1", "--no-graph", "-T", "id"]),
        op.to_string()
    );
    assert_eq!(result.topology.inventory.len(), 3);
    for name in ["upper", "sibling"] {
        let lane = result
            .topology
            .inventory
            .iter()
            .find(|l| l.heads[0].names[0] == name)
            .unwrap();
        assert_eq!(lane.heads.len(), 2);
        assert_eq!(lane.heads[0].base_commit.as_ref(), Some(&lower));
        assert_eq!(lane.heads[1].base_commit.as_ref(), Some(&base));
        assert_eq!(lane.heads[1].names, ["topic-one", "workspace/alias"]);
        assert!(lane.heads[1].shared);
    }
    stock(root, &["new", "-m", "later operation"]);
    assert_eq!(inventory(root, op, target).await.operation, op);
}
#[tokio::test]
async fn applying_only_upper_allows_lower_commit_and_shared_unapply_retains_membership() {
    let temp = fixture();
    let root = temp.path();
    stock(root, &["new", "topic-one", "-m", "upper"]);
    stock(root, &["bookmark", "create", "upper"]);
    stock(root, &["new", "topic-one", "-m", "sibling"]);
    stock(root, &["bookmark", "create", "sibling"]);
    stock(root, &["new", "main"]);
    let initial = refresh(root).await;
    let mut state = run(
        root,
        Some(&initial),
        Command::Setup {
            name: "Discovery".into(),
            target: "main".into(),
        },
    )
    .await;
    state = run(
        root,
        Some(&state),
        Command::Apply {
            branch: "upper".into(),
        },
    )
    .await;
    assert_eq!(state.workspaces[0].applied, ["upper"]);
    assert_eq!(
        state.topology.as_ref().unwrap().lanes[0].heads[1].names,
        ["topic-one"]
    );
    fs::write(root.join("unfinished.txt"), "keep this WIP delta\n").unwrap();
    state = refresh(root).await;
    state = run(
        root,
        Some(&state),
        Command::Commit {
            branch: "topic-one".into(),
            message: "new lower commit".into(),
            selections: vec![],
        },
    )
    .await;
    assert_eq!(state.workspaces[0].applied, ["upper"]);
    assert_eq!(
        fs::read_to_string(root.join("unfinished.txt")).unwrap(),
        "keep this WIP delta\n"
    );
    for name in ["topic-one", "sibling"] {
        state = run(
            root,
            Some(&state),
            Command::Apply {
                branch: name.into(),
            },
        )
        .await;
    }
    let topology = state.topology.as_ref().unwrap();
    assert_eq!(topology.lanes.len(), 2);
    let upper = topology
        .lanes
        .iter()
        .find(|l| l.heads[0].names[0] == "upper")
        .unwrap();
    assert!(upper.applied.contains(&"topic-one".into()));
    assert_eq!(upper.unapply, ["upper"]);
    state = run(
        root,
        Some(&state),
        Command::UnapplyBranches {
            branches: upper.unapply.clone(),
        },
    )
    .await;
    assert_eq!(state.workspaces[0].applied, ["topic-one", "sibling"]);
}

#[tokio::test]
async fn discovery_preserves_cards_above_merges_empty_targets_and_deep_boundaries() {
    let temp = fixture();
    let root = temp.path();
    let base = id(root, "main");
    stock(
        root,
        &["bookmark", "create", "empty-one", "empty-two", "-r", "main"],
    );
    stock(root, &["bookmark", "create", "integrated", "-r", "root()"]);
    stock(
        root,
        &["new", "topic-one", "topic-two", "-m", "native merge"],
    );
    stock(root, &["bookmark", "create", "merged"]);
    let merged = id(root, "merged");
    for _ in 0..42 {
        stock(root, &["new", "-m", "above merge"]);
    }
    stock(root, &["bookmark", "create", "upper"]);
    let op = stock(root, &["op", "log", "-n", "1", "--no-graph", "-T", "id"])
        .parse()
        .unwrap();
    let topology = inventory(root, op, base.clone()).await.topology;
    assert!(topology.complete);
    assert!(topology.integrated.contains(&"integrated".into()));
    for name in ["empty-one", "empty-two"] {
        let lane = topology
            .inventory
            .iter()
            .find(|lane| lane.heads[0].names[0] == name)
            .unwrap();
        assert_eq!(lane.heads.len(), 1);
        assert_eq!(lane.heads[0].base_commit.as_ref(), Some(&base));
    }
    let lane = topology
        .inventory
        .iter()
        .find(|lane| lane.heads[0].names[0] == "upper")
        .unwrap();
    assert_eq!(lane.heads.len(), 2);
    assert_eq!(lane.heads[0].base_commit.as_ref(), Some(&merged));
    assert_eq!(lane.heads[1].names, ["merged"]);
    assert_eq!(lane.heads[1].base_commit, None);
    // Both native sides remain independent; no first-parent-only invented segment.
    assert!(
        topology
            .inventory
            .iter()
            .any(|lane| lane.heads[0].names[0] == "topic-one")
    );
    assert!(
        topology
            .inventory
            .iter()
            .any(|lane| lane.heads[0].names[0] == "topic-two")
    );
}

#[tokio::test]
async fn oversized_bookmark_inventory_does_not_claim_complete_grouping() {
    let temp = fixture();
    let root = temp.path();
    let names = (0..4097)
        .map(|i| format!("candidate-{i}"))
        .collect::<Vec<_>>();
    let mut args = vec!["bookmark", "create", "-r", "main"];
    args.extend(names.iter().map(String::as_str));
    stock(root, &args);
    let op = stock(root, &["op", "log", "-n", "1", "--no-graph", "-T", "id"])
        .parse()
        .unwrap();
    let result = inventory(root, op, id(root, "main")).await;
    assert!(!result.topology.complete);
    assert!(result.topology.inventory.is_empty());
    assert!(result.topology.lanes.is_empty());
    assert!(!root.join(".jj/jj-store/stacks.toml").exists());
}
