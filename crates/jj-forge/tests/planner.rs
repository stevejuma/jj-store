use jj_forge::{Segment, StepKind, planner::effects};
#[test]
fn rewritten_branch_needs_both_push_and_retarget() {
    let segment = Segment {
        branch: "api".into(),
        head: "new".into(),
        base: "model".into(),
        title: "API".into(),
        body: String::new(),
        remote_head: Some("old".into()),
        shared: false,
    };
    let plan = effects(&segment, Some("main"));
    assert!(plan.contains(&StepKind::Push));
    assert!(plan.contains(&StepKind::Retarget));
}

fn connection() -> jj_forge::Connection {
    jj_forge::Connection {
        provider: jj_forge::Provider::Github,
        api_url: "https://api.github.com".into(),
        repository: "team/repo".into(),
        source_repository: "team/repo".into(),
        remote: "origin".into(),
        target_remote: "origin".into(),
        target: "main".into(),
        native_stacks: false,
    }
}
fn pr() -> jj_forge::PullRequest {
    jj_forge::PullRequest {
        number: 1,
        url: "https://github.com/team/repo/pull/1".into(),
        title: "Model".into(),
        body: "Human text\n".into(),
        head: "model".into(),
        sha: "a".repeat(40),
        base: "main".into(),
        draft: false,
        state: jj_forge::PrState::Open,
        mergeable: Some(true),
        native_stack: None,
        updated_at: "".into(),
    }
}
#[test]
fn managed_links_preserve_authored_prefix_and_suffix() {
    let p = pr();
    let first = jj_forge::planner::body(&p.body, std::slice::from_ref(&p), 1, false).unwrap();
    let input = format!("{first}\nHuman tail\n");
    let mut landed = p.clone();
    landed.state = jj_forge::PrState::Merged;
    let updated = jj_forge::planner::body(&input, &[landed], 1, false).unwrap();
    assert!(updated.starts_with("Human text\n\n\n"));
    assert!(updated.ends_with("\nHuman tail\n"));
    assert!(updated.contains("Merged"));
    assert_eq!(
        jj_forge::planner::body(&updated, std::slice::from_ref(&p), 1, false)
            .unwrap()
            .matches(jj_forge::planner::START)
            .count(),
        1
    );
}
#[test]
fn malformed_navigation_is_not_overwritten() {
    assert!(
        jj_forge::planner::body("Human text <!-- jj-store:stack:v1 -->", &[pr()], 1, false)
            .is_err()
    );
}
#[test]
fn fork_plan_is_explicitly_cumulative_and_targets_upstream() {
    let mut c = connection();
    c.source_repository = "contributor/repo".into();
    let s = Segment {
        branch: "api".into(),
        head: "a".repeat(40),
        base: "model".into(),
        title: "API".into(),
        body: String::new(),
        remote_head: None,
        shared: false,
    };
    let plan =
        jj_forge::planner::plan("request".into(), "a".repeat(128), c, vec![s], &[], true).unwrap();
    assert!(plan.cumulative);
    assert_eq!(plan.items[0].segment.base, "main");
}
#[test]
fn endpoint_credentials_and_invalid_refs_are_rejected() {
    let mut c = connection();
    c.api_url = "https://token@github.com".into();
    assert!(c.validate().is_err());
    for value in ["-all", "a..b", "branch name", "a:main", "../x"] {
        assert!(!jj_forge::types::safe_name(value));
    }
}
#[test]
fn journal_rejects_conflicting_identity_and_survives_restart() {
    let root = tempfile::tempdir().unwrap();
    let metadata = tempfile::tempdir().unwrap();
    let c = connection();
    let s = Segment {
        branch: "model".into(),
        head: "a".repeat(40),
        base: "main".into(),
        title: "Model".into(),
        body: "".into(),
        remote_head: None,
        shared: false,
    };
    let plan = jj_forge::planner::plan("id".into(), "a".repeat(128), c.clone(), vec![s], &[], true)
        .unwrap();
    let receipt = jj_forge::Receipt {
        plan,
        steps: vec![],
        outcome: jj_forge::Outcome::Unknown,
        pull_requests: vec![],
        message: None,
    };
    {
        let mut d = jj_forge::journal::Device::at(root.path(), metadata.path()).unwrap();
        d.configure(c).unwrap();
        d.put(receipt.clone()).unwrap();
        assert!(jj_forge::journal::Device::at(root.path(), metadata.path()).is_err());
    }
    let mut d = jj_forge::journal::Device::at(root.path(), metadata.path()).unwrap();
    assert_eq!(d.receipt("id").unwrap().outcome, jj_forge::Outcome::Unknown);
    let mut changed = receipt;
    changed.plan.items[0].segment.head = "b".repeat(40);
    assert!(d.put(changed).is_err());
}

#[test]
fn repeated_acknowledged_pr_action_is_not_executed_again() {
    use jj_forge::{Outcome, PrAction, journal::Device};
    let root = tempfile::tempdir().unwrap();
    let metadata = tempfile::tempdir().unwrap();
    let mut device = Device::at(root.path(), metadata.path()).unwrap();
    device.configure(connection()).unwrap();
    let action = PrAction::Cancel { run: "10".into() };
    assert!(
        device
            .begin_operation("cancel-1", 1, &action)
            .unwrap()
            .is_none()
    );
    device.finish_operation("cancel-1", Ok(None)).unwrap();
    assert_eq!(
        device
            .begin_operation("cancel-1", 1, &action)
            .unwrap()
            .unwrap()
            .outcome,
        Outcome::Complete
    );
}
#[test]
fn edit_keeps_current_managed_navigation() {
    let current = jj_forge::planner::body("Original", &[pr()], 1, false).unwrap();
    let edited = jj_forge::planner::edit_body(&current, "New authored text").unwrap();
    assert!(edited.starts_with("New authored text"));
    assert!(edited.contains(jj_forge::planner::START));
}

#[test]
fn portable_merge_requires_landed_parents_and_a_trunk_target() {
    use jj_forge::*;
    let c = connection();
    let lower = pr();
    let mut upper = lower.clone();
    upper.number = 2;
    upper.head = "api".into();
    upper.base = "model".into();
    let items = vec![
        Segment {
            branch: "model".into(),
            head: "a".repeat(40),
            base: "main".into(),
            title: "Model".into(),
            body: "".into(),
            remote_head: Some("a".repeat(40)),
            shared: false,
        },
        Segment {
            branch: "api".into(),
            head: "b".repeat(40),
            base: "model".into(),
            title: "API".into(),
            body: "".into(),
            remote_head: Some("b".repeat(40)),
            shared: false,
        },
    ];
    let p = planner::plan(
        "merge".into(),
        "a".repeat(128),
        c.clone(),
        items,
        &[lower.clone(), upper.clone()],
        false,
    )
    .unwrap();
    let receipt = Receipt {
        plan: p,
        steps: vec![],
        outcome: Outcome::Complete,
        pull_requests: vec![lower.clone(), upper.clone()],
        message: None,
    };
    assert!(!engine::merge_dependencies_met(
        &upper,
        &c,
        std::slice::from_ref(&receipt),
        &[lower.clone(), upper.clone()]
    ));
    let mut landed = lower;
    landed.state = PrState::Merged;
    assert!(!engine::merge_dependencies_met(
        &upper,
        &c,
        std::slice::from_ref(&receipt),
        &[landed.clone(), upper.clone()]
    ));
    upper.base = "main".into();
    assert!(engine::merge_dependencies_met(
        &upper,
        &c,
        std::slice::from_ref(&receipt),
        &[landed.clone(), upper.clone()]
    ));
    landed.state = PrState::Closed;
    assert!(!engine::merge_dependencies_met(
        &upper,
        &c,
        &[receipt],
        &[landed, upper.clone()]
    ));
}

#[test]
fn connection_history_and_unresolved_actions_cannot_be_rebound_or_replayed() {
    use jj_forge::*;
    let root = tempfile::tempdir().unwrap();
    let metadata = tempfile::tempdir().unwrap();
    let mut d = journal::Device::at(root.path(), metadata.path()).unwrap();
    let c = connection();
    d.configure(c.clone()).unwrap();
    let action = PrAction::Cancel { run: "10".into() };
    d.begin_operation("original", 1, &action).unwrap();
    assert!(matches!(
        d.begin_operation("replacement", 1, &action),
        Err(Error::Unknown)
    ));
    let mut other = c.clone();
    other.repository = "team/other".into();
    assert!(matches!(d.configure(other), Err(Error::Stale)));
    d.configure(c).unwrap();
    d.finish_operation(
        "original",
        Ok(Some(MergeResult {
            state: "enqueued".into(),
            receipt: Some("queue-1".into()),
            sha: None,
        })),
    )
    .unwrap();
    drop(d);
    let mut d = journal::Device::at(root.path(), metadata.path()).unwrap();
    assert_eq!(d.merge_results()[&1].state, "enqueued");
    d.record_merge_status(
        1,
        "queue-1",
        MergeResult {
            state: "merged".into(),
            receipt: Some("queue-1".into()),
            sha: Some("a".repeat(40)),
        },
    )
    .unwrap();
    assert_eq!(d.merge_results()[&1].state, "merged");
}
