use jj_workflow::github::{Context, Workflow, workflow_path};
fn yaml(on: &str) -> Workflow {
    Workflow::parse(&format!(
        "on: {on}\njobs:\n  test:\n    runs-on: ubuntu-latest\n    steps:\n      - run: echo test\n"
    ))
    .unwrap()
}
#[test]
fn supported_events_and_same_revision_edits() {
    let context = || Context {
        event: "push",
        branch: Some("main"),
        paths: None,
    };
    assert!(
        yaml("[push, workflow_dispatch]")
            .matches(context())
            .unwrap()
    );
    assert!(
        !yaml("{push: {branches: [release]}}")
            .matches(context())
            .unwrap()
    );
    assert!(
        yaml("{push: {branches: [main]}}")
            .matches(context())
            .unwrap()
    );
    assert!(
        !yaml("push")
            .matches(Context {
                event: "workflow_dispatch",
                branch: None,
                paths: None
            })
            .unwrap()
    );
}
#[test]
fn branch_and_path_filters_are_both_required() {
    let workflow = yaml("{push: {branches: [main], paths: ['src/**', '!src/docs/**']}}");
    let paths = vec!["src/a.rs".into()];
    assert!(
        workflow
            .matches(Context {
                event: "push",
                branch: Some("main"),
                paths: Some(&paths)
            })
            .unwrap()
    );
    assert!(
        !workflow
            .matches(Context {
                event: "push",
                branch: None,
                paths: Some(&paths)
            })
            .unwrap()
    );
    let paths = vec!["src/docs/a.md".into()];
    assert!(
        !workflow
            .matches(Context {
                event: "push",
                branch: Some("main"),
                paths: Some(&paths)
            })
            .unwrap()
    );
}
#[test]
fn exclusions_and_empty_changes_skip() {
    let workflow = yaml("{push: {paths-ignore: ['docs/**']}}");
    for paths in [vec![], vec!["docs/readme.md".into()]] {
        assert!(
            !workflow
                .matches(Context {
                    event: "push",
                    branch: None,
                    paths: Some(&paths)
                })
                .unwrap()
        );
    }
    assert!(
        workflow
            .matches(Context {
                event: "push",
                branch: None,
                paths: Some(&["src/a.rs".into()])
            })
            .unwrap()
    );
}
#[test]
fn ambiguous_paths_and_unsupported_filters_fail_explicitly() {
    assert!(
        yaml("{push: {paths: ['src/**']}}")
            .matches(Context {
                event: "push",
                branch: None,
                paths: None
            })
            .is_err()
    );
    assert!(
        yaml("{push: {paths: ['src/**'], paths-ignore: ['docs/**']}}")
            .matches(Context {
                event: "push",
                branch: None,
                paths: None
            })
            .is_err()
    );
    assert!(
        yaml("{push: {types: [created]}}")
            .matches(Context {
                event: "push",
                branch: None,
                paths: None
            })
            .is_err()
    );
}
#[test]
fn private_copy_retains_triggers_and_job_expressions() {
    let workflow = yaml("{push: {branches: [main]}}");
    let before = workflow.document.clone();
    let copy = workflow.execution_yaml("attempt-1").unwrap();
    let copy = Workflow::parse(&copy).unwrap();
    assert_eq!(workflow.document, before);
    assert_eq!(copy.document["on"], before["on"]);
    assert_eq!(
        copy.document["jobs"]["test"]["steps"][0]["uses"].as_str(),
        Some("actions/checkout@v4")
    );
    assert_eq!(
        &copy.document["jobs"]["test"]["steps"]
            .as_sequence()
            .unwrap()[1..],
        before["jobs"]["test"]["steps"]
            .as_sequence()
            .unwrap()
            .as_slice()
    );
}
#[test]
fn path_validation_and_invalid_yaml() {
    for path in [".github/workflows/a.yml", ".github/workflows/a.yaml"] {
        assert!(workflow_path(path).is_ok());
    }
    for path in [
        "/tmp/a.yml",
        ".github/workflows/../a.yml",
        ".github/workflows/a.sh",
    ] {
        assert!(workflow_path(path).is_err());
    }
    assert!(Workflow::parse("jobs: [bad]").is_err());
}

#[test]
fn checkout_cannot_replace_the_recorded_source() {
    let workflow=Workflow::parse("on: push\njobs:\n  test:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n        with: {ref: main}\n").unwrap();
    assert!(workflow.execution_yaml("attempt").is_err());
}
