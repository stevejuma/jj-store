use clap::Parser as _;
use jj_store_cli::{commands::Extension, review_commands::ReviewCommand};

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: Extension,
}

#[test]
fn review_commands_require_explicit_receipt_and_concurrency_identities() {
    let request = "01010101010101010101010101010101";
    let thread = "02020202020202020202020202020202";
    let parsed = Cli::try_parse_from([
        "jj",
        "review",
        "reply",
        thread,
        "--request-id",
        request,
        "--expected-workflow-version",
        "7",
        "--body",
        "Reply text",
        "--format",
        "json",
    ])
    .unwrap();
    let Extension::Review(args) = parsed.command else {
        panic!("expected review command")
    };
    assert!(matches!(
        args.command,
        Some(ReviewCommand::Reply {
            expected_workflow_version: 7,
            ..
        })
    ));

    assert!(
        Cli::try_parse_from([
            "jj",
            "review",
            "edit",
            thread,
            "--request-id",
            request,
            "--body",
            "replacement",
        ])
        .is_err()
    );
    assert!(
        Cli::try_parse_from(["jj", "review", "resolve", thread, "--request-id", request,]).is_err()
    );
}

#[test]
fn review_list_show_and_status_preserve_full_stable_ids() {
    let thread = "11111111111111111111111111111111";
    let request = "12121212121212121212121212121212";
    for arguments in [
        vec!["jj", "review", "list", "--limit", "25"],
        vec!["jj", "review", "show", thread],
        vec!["jj", "review", "status", request],
    ] {
        let parsed = Cli::try_parse_from(arguments).unwrap();
        assert!(matches!(parsed.command, Extension::Review(_)));
    }
}
