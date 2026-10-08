//! The metadata checkout and a task's linked repository are distinct arguments.
use clap::Parser as _;
use jj_store_cli::commands::Extension;

#[derive(clap::Parser)]
struct Cli {
    #[arg(short = 'R', long, global = true)]
    repository: Option<String>,
    #[command(subcommand)]
    command: Extension,
}

#[test]
fn task_target_does_not_shadow_the_global_checkout_argument() {
    let request = "01010101010101010101010101010101";
    let project = "02020202020202020202020202020202";
    let target = "03030303030303030303030303030303";
    let task = "04040404040404040404040404040404";
    for arguments in [
        vec![
            "jj",
            "-R",
            "/tmp/metadata home",
            "task",
            "create",
            "--request-id",
            request,
            "--project",
            project,
            "--expected-project-version",
            "1",
            "--title",
            "Task",
            "--target-repository",
            target,
        ],
        vec![
            "jj",
            "--repository",
            "/tmp/metadata home",
            "task",
            "edit",
            task,
            "--request-id",
            request,
            "--expected-version",
            "2",
            "--expected-project-version",
            "1",
            "--title",
            "Edited task",
            "--body",
            "Body",
            "--state",
            "in-progress",
            "--target-repository",
            target,
        ],
    ] {
        let parsed = Cli::try_parse_from(arguments).unwrap();
        assert_eq!(parsed.repository.as_deref(), Some("/tmp/metadata home"));
        assert!(matches!(parsed.command, Extension::Task(_)));
    }
}
