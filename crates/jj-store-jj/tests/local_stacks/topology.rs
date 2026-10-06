use super::*;

#[tokio::test]
async fn topology_groups_long_dependencies_and_aliases_without_flattening_merges() {
    let temp = fixture();
    let root = temp.path();
    stock(root, &["bookmark", "create", "integrated", "-r", "main"]);
    stock(root, &["new", "main", "-m", "target advance"]);
    stock(root, &["bookmark", "set", "main"]);
    stock(
        root,
        &["bookmark", "create", "empty-one", "empty-two", "-r", "main"],
    );
    stock(root, &["bookmark", "create", "alias", "-r", "topic-one"]);
    stock(root, &["new", "topic-one"]);
    for _ in 0..45 {
        stock(root, &["new"]);
    }
    stock(root, &["bookmark", "create", "dependent"]);
    stock(root, &["new", "topic-one", "topic-two", "-m", "user merge"]);
    stock(root, &["bookmark", "create", "merged"]);
    stock(root, &["new", "main"]);
    let initial = refresh(root).await;
    let mut state = run(
        root,
        Some(&initial),
        Command::Setup {
            name: "Topology".into(),
            target: "main".into(),
        },
    )
    .await;
    for branch in [
        "integrated",
        "topic-one",
        "alias",
        "dependent",
        "topic-two",
        "merged",
        "empty-one",
        "empty-two",
    ] {
        state = run(
            root,
            Some(&state),
            Command::Apply {
                branch: branch.into(),
            },
        )
        .await;
    }
    let topology = state.topology.as_ref().unwrap();
    assert!(topology.complete);
    assert_eq!(topology.integrated, ["integrated"]);
    assert_eq!(topology.lanes.len(), 5);
    assert_eq!(topology.lanes[0].heads[0].names, ["dependent"]);
    assert_eq!(topology.lanes[0].heads[1].names, ["topic-one", "alias"]);
    assert_eq!(topology.lanes[1].applied, ["topic-two"]);
    assert_eq!(topology.lanes[2].applied, ["merged"]);
    assert_eq!(topology.lanes[3].applied, ["empty-one"]);
    assert_eq!(topology.lanes[4].applied, ["empty-two"]);
    // The native merge includes target and every distinct applied tip.
    assert_eq!(
        stock(
            root,
            &["log", "-r", "@-", "--no-graph", "-T", "parents.len()"]
        ),
        "6"
    );
    let before = state.operation;
    for branches in [vec!["topic-one", "absent"], vec!["alias", "alias"], vec![]] {
        let result = local_stacks::execute(
            Arc::new(Access(root.join(".jj/repo"))),
            root.canonicalize().unwrap(),
            Request {
                version: 1,
                binding_generation: 1,
                operation: Some(state.operation),
                revision: Some(state.revision),
                command: Command::UnapplyBranches {
                    branches: branches.into_iter().map(String::from).collect(),
                },
            },
        )
        .await;
        assert!(matches!(result, Err(local_stacks::Error::Invalid)));
        assert_eq!(refresh(root).await.operation, before);
    }
    let request = Request {
        version: 1,
        binding_generation: 1,
        operation: Some(state.operation),
        revision: Some(state.revision),
        command: Command::UnapplyBranches {
            branches: vec!["dependent".into(), "topic-one".into(), "alias".into()],
        },
    };
    let preview = local_stacks::preview(
        Arc::new(Access(root.join(".jj/repo"))),
        root.canonicalize().unwrap(),
        request.clone(),
    )
    .await
    .unwrap();
    assert!(preview.summary.contains("dependent, topic-one, alias"));
    state = run(root, Some(&state), request.command.clone()).await;
    assert_eq!(
        state.workspaces[0].applied,
        [
            "integrated",
            "topic-two",
            "merged",
            "empty-one",
            "empty-two"
        ]
    );
    assert!(matches!(
        local_stacks::execute(
            Arc::new(Access(root.join(".jj/repo"))),
            root.canonicalize().unwrap(),
            request
        )
        .await,
        Err(local_stacks::Error::Stale)
    ));
    // Individual application does not pull an unapplied descendant into membership.
    state = run(
        root,
        Some(&state),
        Command::Apply {
            branch: "topic-one".into(),
        },
    )
    .await;
    assert!(
        !state.workspaces[0]
            .applied
            .contains(&"dependent".to_string())
    );
    assert_eq!(
        state.topology.unwrap().lanes.last().unwrap().applied,
        ["topic-one"]
    );
}

#[tokio::test]
async fn membership_retains_target_merge_identity_and_only_one_live_merge() {
    let temp = fixture();
    let root = temp.path();
    // Target advances independently from both topics.
    stock(root, &["new", "main", "-m", "target advance"]);
    fs::write(root.join("target.txt"), "target\n").unwrap();
    stock(root, &["bookmark", "set", "main"]);
    stock(root, &["new", "main"]);
    fs::write(root.join("draft.txt"), "unfinished\n").unwrap();
    let initial = refresh(root).await;
    let mut state = run(
        root,
        Some(&initial),
        Command::Setup {
            name: "Topology".into(),
            target: "main".into(),
        },
    )
    .await;
    let merge_change = stock(root, &["log", "-r", "@-", "--no-graph", "-T", "change_id"]);
    let wip_change = stock(root, &["log", "-r", "@", "--no-graph", "-T", "change_id"]);
    for branch in ["topic-one", "topic-two"] {
        state = run(
            root,
            Some(&state),
            Command::Apply {
                branch: branch.into(),
            },
        )
        .await;
        assert!(
            root.join("target.txt").exists(),
            "applying a branch dropped target content"
        );
        assert_eq!(
            stock(root, &["log", "-r", "@-", "--no-graph", "-T", "change_id"]),
            merge_change
        );
        assert_eq!(
            stock(root, &["log", "-r", "@", "--no-graph", "-T", "change_id"]),
            wip_change
        );
        assert_eq!(
            stock(
                root,
                &[
                    "log",
                    "-r",
                    "description(exact:'Workspace Topology merge')",
                    "--no-graph",
                    "-T",
                    "commit_id ++ '\n'"
                ]
            )
            .lines()
            .count(),
            1
        );
        assert!(!state.workspaces[0].membership_changed);
    }
    state = run(
        root,
        Some(&state),
        Command::Unapply {
            branch: "topic-one".into(),
        },
    )
    .await;
    assert_eq!(
        fs::read_to_string(root.join("draft.txt")).unwrap(),
        "unfinished\n"
    );
    assert!(root.join("target.txt").exists() && root.join("two.txt").exists());
    assert!(!root.join("one.txt").exists());
    assert_eq!(state.workspaces[0].applied, ["topic-two"]);
}

/// Reproduce the old composer exactly: new merge change, retained WIP rewrite,
/// and the recorded internal reference transition in a native operation.
async fn legacy_replace(root: &Path) -> String {
    use jj_lib::{
        config::StackedConfig,
        default_backend_factories::{default_backend_factories, default_working_copy_factories},
        object_id::ObjectId as _,
        op_store::RefTarget,
        repo::Repo as _,
        settings::UserSettings,
        workspace::Workspace,
    };
    let settings = UserSettings::from_config(StackedConfig::with_defaults()).unwrap();
    let mut workspace = Workspace::load(
        &settings,
        root,
        &default_backend_factories(),
        &default_working_copy_factories(),
    )
    .unwrap();
    let repo = workspace.repo_loader().load_at_head().await.unwrap();
    let name = workspace.workspace_name().to_owned();
    let mut locked = workspace.start_working_copy_mutation().await.unwrap();
    let wip = repo
        .store()
        .get_commit(repo.view().get_wc_commit_id(&name).unwrap())
        .unwrap();
    let old = wip.parents().await.unwrap().into_iter().next().unwrap();
    let merge_ref = repo
        .view()
        .local_bookmarks()
        .find(|(name, target)| {
            name.as_str().ends_with("/merge") && target.as_normal() == Some(old.id())
        })
        .unwrap()
        .0
        .to_owned();
    let wip_ref = repo
        .view()
        .local_bookmarks()
        .find(|(name, target)| {
            name.as_str().ends_with("/wip") && target.as_normal() == Some(wip.id())
        })
        .unwrap()
        .0
        .to_owned();
    let mut tx = repo.start_transaction();
    let target = repo
        .view()
        .get_local_bookmark("topic-one".as_ref())
        .as_normal()
        .unwrap()
        .clone();
    let tip = repo.store().get_commit(&target).unwrap();
    let new = tx
        .repo_mut()
        .new_commit(vec![target], tip.tree())
        .set_description("Workspace Legacy merge")
        .write()
        .await
        .unwrap();
    let new_wip = jj_lib::rewrite::rebase_commit(tx.repo_mut(), wip, vec![new.id().clone()])
        .await
        .unwrap();
    tx.repo_mut()
        .set_local_bookmark_target(&merge_ref, RefTarget::normal(new.id().clone()));
    tx.repo_mut()
        .set_local_bookmark_target(&wip_ref, RefTarget::normal(new_wip.id().clone()));
    tx.repo_mut()
        .set_wc_commit(name, new_wip.id().clone())
        .unwrap();
    tx.repo_mut().rebase_descendants().await.unwrap();
    let after = tx
        .commit("Stacks: Apply { branch: \"topic-one\" }")
        .await
        .unwrap();
    locked.locked_wc().check_out(&new_wip).await.unwrap();
    locked.finish(after.op_id().clone()).await.unwrap();
    old.id().hex()
}

#[tokio::test]
async fn reconciliation_only_retires_proven_unreferenced_legacy_heads() {
    let temp = fixture();
    let root = temp.path();
    let initial = refresh(root).await;
    let state = run(
        root,
        Some(&initial),
        Command::Setup {
            name: "Legacy".into(),
            target: "main".into(),
        },
    )
    .await;
    run(
        root,
        Some(&state),
        Command::Apply {
            branch: "topic-one".into(),
        },
    )
    .await;
    let protected = legacy_replace(root).await;
    stock(
        root,
        &["bookmark", "create", "keep-this-merge", "-r", &protected],
    );
    let descendant = legacy_replace(root).await;
    stock(
        root,
        &["new", &descendant, "-m", "User child of legacy merge"],
    );
    stock(root, &["bookmark", "create", "user-child"]);
    // Return to the app-owned WIP without changing its identity.
    let config: toml::Value =
        toml::from_str(&fs::read_to_string(root.join(".jj/jj-store/stacks.toml")).unwrap())
            .unwrap();
    let wip_ref = config["workspaces"][0]["wip_ref"].as_str().unwrap();
    stock(root, &["edit", wip_ref]);
    let eligible = legacy_replace(root).await;
    stock(root, &["describe", "-m", "Unfinished work"]);
    let before = refresh(root).await;
    assert!(before.workspaces[0].membership_changed);
    assert!(
        stock(
            root,
            &["log", "-r", "heads(all())", "--no-graph", "-T", "commit_id"]
        )
        .contains(&eligible)
    );
    let request = Request {
        version: 1,
        binding_generation: 1,
        operation: Some(before.operation),
        revision: Some(before.revision),
        command: Command::Reconcile,
    };
    let preview = local_stacks::preview(
        Arc::new(Access(root.join(".jj/repo"))),
        root.canonicalize().unwrap(),
        request,
    )
    .await
    .unwrap();
    assert!(preview.summary.contains(&eligible));
    assert!(!preview.summary.contains(&protected) && !preview.summary.contains(&descendant));
    // Preview and refresh have no graph-repair side effects.
    assert_eq!(refresh(root).await.operation, before.operation);
    let after = run(root, Some(&before), Command::Reconcile).await;
    assert!(!after.workspaces[0].membership_changed);
    assert!(
        !stock(
            root,
            &["log", "-r", "heads(all())", "--no-graph", "-T", "commit_id"]
        )
        .contains(&eligible)
    );
    assert_eq!(
        stock(
            root,
            &[
                "log",
                "-r",
                "keep-this-merge",
                "--no-graph",
                "-T",
                "commit_id"
            ]
        ),
        protected
    );
    assert!(
        !stock(
            root,
            &["log", "-r", "user-child", "--no-graph", "-T", "commit_id"]
        )
        .is_empty()
    );
    assert_eq!(
        stock(
            root,
            &[
                "log",
                "--at-op",
                &before.operation.to_string(),
                "-r",
                &eligible,
                "--no-graph",
                "-T",
                "commit_id"
            ]
        ),
        eligible
    );
}

#[tokio::test]
async fn sample_testy_topology_has_three_columns_and_native_target_plus_wip() {
    let temp = fixture();
    let root = temp.path();
    stock(
        root,
        &["bookmark", "rename", "topic-one", "feature/cloud-runner"],
    );
    stock(
        root,
        &["bookmark", "rename", "topic-two", "feature/export-json"],
    );
    stock(
        root,
        &[
            "bookmark",
            "create",
            "feature/bootstrap-cleanup",
            "-r",
            "main",
        ],
    );
    stock(root, &["new", "main", "-m", "readme polish"]);
    stock(root, &["bookmark", "create", "feature/readme-polish"]);
    stock(root, &["new", "-m", "trunk"]);
    stock(root, &["bookmark", "create", "trunk"]);
    stock(
        root,
        &["new", "feature/cloud-runner", "-m", "observability"],
    );
    stock(root, &["bookmark", "create", "feature/cloud-observability"]);
    stock(
        root,
        &[
            "bookmark",
            "create",
            "workspace/experiment",
            "-r",
            "feature/cloud-runner",
        ],
    );
    stock(root, &["new", "main", "-m", "query filtering"]);
    stock(root, &["bookmark", "create", "feature/query-filtering"]);
    stock(root, &["new", "trunk"]);
    let initial = refresh(root).await;
    let mut state = run(
        root,
        Some(&initial),
        Command::Setup {
            name: "testy".into(),
            target: "trunk".into(),
        },
    )
    .await;
    for branch in [
        "feature/bootstrap-cleanup",
        "feature/cloud-observability",
        "feature/export-json",
        "feature/cloud-runner",
        "feature/query-filtering",
        "feature/readme-polish",
        "workspace/experiment",
    ] {
        state = run(
            root,
            Some(&state),
            Command::Apply {
                branch: branch.into(),
            },
        )
        .await;
    }
    let topology = state.topology.unwrap();
    assert_eq!(
        topology
            .lanes
            .iter()
            .map(|lane| lane.heads[0].names[0].as_str())
            .collect::<Vec<_>>(),
        [
            "feature/cloud-observability",
            "feature/export-json",
            "feature/query-filtering"
        ]
    );
    assert_eq!(
        topology.lanes[0].heads[1].names,
        ["feature/cloud-runner", "workspace/experiment"]
    );
    assert_eq!(
        topology.integrated,
        ["feature/bootstrap-cleanup", "feature/readme-polish", "main"]
    );
    assert_eq!(
        stock(
            root,
            &["log", "-r", "@- & trunk+", "--no-graph", "-T", "commit_id"]
        ),
        state.workspaces[0].merge_commit
    );
    assert_eq!(
        stock(
            root,
            &[
                "log",
                "-r",
                "@",
                "--no-graph",
                "-T",
                "parents.map(|p| p.commit_id())"
            ]
        ),
        state.workspaces[0].merge_commit
    );
    assert_eq!(
        stock(
            root,
            &[
                "log",
                "-r",
                "description(exact:'Workspace testy merge')",
                "--no-graph",
                "-T",
                "commit_id ++ '\n'"
            ]
        )
        .lines()
        .count(),
        1
    );
}
