use super::*;

fn description(root: &Path) -> String {
    stock(
        root,
        &["op", "log", "--no-graph", "-n", "1", "-T", "description"],
    )
}

#[tokio::test]
async fn stack_operations_persist_readable_descriptions() {
    let temp = fixture();
    let root = temp.path();
    let mut state = refresh(root).await;
    let commands = [
        (
            Command::Setup {
                name: "My workspace".into(),
                target: "main".into(),
            },
            "Stacks: Set up workspace My workspace targeting main",
        ),
        (
            Command::Apply {
                branch: "topic-one".into(),
            },
            "Stacks: Apply topic-one",
        ),
        (
            Command::Apply {
                branch: "topic-two".into(),
            },
            "Stacks: Apply topic-two",
        ),
        (
            Command::UnapplyBranches {
                branches: vec!["topic-one".into(), "topic-two".into()],
            },
            "Stacks: Unapply topic-one, topic-two",
        ),
        (
            Command::Apply {
                branch: "topic-one".into(),
            },
            "Stacks: Apply topic-one",
        ),
        (
            Command::Unapply {
                branch: "topic-one".into(),
            },
            "Stacks: Unapply topic-one",
        ),
        (
            Command::SetTarget {
                branch: "topic-two".into(),
            },
            "Stacks: Set target branch to topic-two",
        ),
        (
            Command::Create {
                name: "Second workspace".into(),
            },
            "Stacks: Create workspace Second workspace",
        ),
    ];
    for (command, expected) in commands {
        state = run(root, Some(&state), command).await;
        assert_eq!(description(root), expected);
    }
    let first = state.workspaces[0].id.clone();
    let second = state.active.clone().unwrap();
    state = run(
        root,
        Some(&state),
        Command::Switch {
            workspace: first.clone(),
        },
    )
    .await;
    assert_eq!(
        description(root),
        "Stacks: Switch to workspace My workspace"
    );
    state = run(
        root,
        Some(&state),
        Command::Update {
            workspace: first,
            name: "Renamed workspace".into(),
            target: "main".into(),
        },
    )
    .await;
    assert_eq!(
        description(root),
        "Stacks: Update workspace Renamed workspace targeting main"
    );
    run(root, Some(&state), Command::Delete { workspace: second }).await;
    assert_eq!(
        description(root),
        "Stacks: Delete workspace Second workspace"
    );
}
