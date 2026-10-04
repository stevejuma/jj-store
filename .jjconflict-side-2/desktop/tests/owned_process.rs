//! Exercise the production pipe owner with actual disposable subprocesses.
#![cfg(unix)]
use jj_store_desktop::process::OwnedProcess;
use std::{process::Command, time::Duration};

#[test]
fn owner_finish_closes_stdin_and_waits_for_clean_child_exit() {
    let mut command = Command::new("/bin/sh");
    command.args(["-c", "printf 'READY\n'; cat >/dev/null"]);
    let mut process = OwnedProcess::spawn(&mut command).unwrap();
    assert_eq!(
        process.first_line(Duration::from_secs(3)).unwrap(),
        "READY\n"
    );
    assert!(process.running().unwrap());
    process.finish(Duration::from_secs(3)).unwrap();
    assert!(!process.running().unwrap());
}

#[test]
fn oversized_readiness_is_rejected_without_unbounded_capture() {
    let mut command = Command::new("/usr/bin/head");
    command.args(["-c", "65537", "/dev/zero"]);
    let mut process = OwnedProcess::spawn(&mut command).unwrap();
    let error = process.first_line(Duration::from_secs(3)).unwrap_err();
    assert!(error.contains("exceeded 64 KiB"));
    process.finish(Duration::from_secs(3)).unwrap();
}

#[test]
fn stderr_capture_remains_bounded_while_child_output_is_drained() {
    let mut command = Command::new("/bin/sh");
    command.args([
        "-c",
        "head -c 131072 /dev/zero >&2; printf 'READY\n'; cat >/dev/null",
    ]);
    let mut process = OwnedProcess::spawn(&mut command).unwrap();
    assert_eq!(
        process.first_line(Duration::from_secs(3)).unwrap(),
        "READY\n"
    );
    assert!(process.diagnostics().len() <= 65536);
    process.finish(Duration::from_secs(3)).unwrap();
}
