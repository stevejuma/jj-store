//! Real fixed JJ boundary; explicitly selects the development binary.
use jj_store_jj::agent_workspaces::Provider;
use std::{fs, path::Path, process::Command};
fn jj(executable: &Path, root: &Path, args: &[&str]) -> String {
    let output = Command::new(executable)
        .current_dir(root)
        .arg("--no-pager")
        .arg("-R")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
#[tokio::test]
#[ignore = "requires JJ_STORE_AGENT_TEST_JJ fixed custom development executable"]
async fn isolated_workspace_preserves_dirty_source_and_retains_cleanup_revision() {
    let executable = std::env::var_os("JJ_STORE_AGENT_TEST_JJ")
        .expect("explicit development executable")
        .into();
    let provider = Provider { executable };
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path().join("source");
    let output = Command::new(&provider.executable)
        .args(["git", "init"])
        .arg(&root)
        .output()
        .unwrap();
    assert!(output.status.success());
    fs::write(root.join("file.txt"), "base\n").unwrap();
    jj(&provider.executable, &root, &["status"]);
    let selected = provider.inspect(&root).await.unwrap();
    let checkout = fs::read(root.join(".jj/working_copy/checkout")).unwrap();
    fs::write(root.join("file.txt"), "dirty source\n").unwrap();
    fs::write(root.join("untracked.txt"), "preserve me").unwrap();
    let reread = provider.inspect(&root).await.unwrap();
    assert_eq!(reread.commit, selected.commit);
    let destination = fixture.path().join("agent-test");
    let created = provider
        .create(
            &selected,
            &destination,
            selected.operation,
            &selected.commit,
            "agent-test",
        )
        .await
        .unwrap();
    assert_eq!(
        fs::read_to_string(destination.join("file.txt")).unwrap(),
        "base\n"
    );
    assert!(!destination.join("untracked.txt").exists());
    assert_eq!(
        fs::read(root.join(".jj/working_copy/checkout")).unwrap(),
        checkout
    );
    assert_eq!(
        fs::read_to_string(root.join("file.txt")).unwrap(),
        "dirty source\n"
    );
    assert_eq!(
        provider
            .create(
                &selected,
                &destination,
                selected.operation,
                &selected.commit,
                "agent-test"
            )
            .await
            .unwrap()
            .commit,
        created.commit
    );
    fs::write(destination.join("file.txt"), "agent edit\n").unwrap();
    fs::write(destination.join(".gitignore"), "ignored.txt\n").unwrap();
    fs::write(destination.join("ignored.txt"), "keep").unwrap();
    assert_eq!(
        provider.cleanup(&created).await.unwrap_err(),
        "cleanup_has_untracked_files"
    );
    assert!(destination.exists());
    fs::remove_file(destination.join("ignored.txt")).unwrap();
    let edited = provider.snapshot(&created).await.unwrap();
    assert_ne!(edited.commit, created.commit);
    assert_eq!(
        provider
            .create(
                &selected,
                &destination,
                selected.operation,
                &selected.commit,
                "agent-test"
            )
            .await
            .unwrap_err(),
        "workspace_base_unverified"
    );
    provider.cleanup(&created).await.unwrap();
    assert!(!destination.exists());
    let content = jj(
        &provider.executable,
        &root,
        &[
            "--ignore-working-copy",
            "file",
            "show",
            "-r",
            "retained-agent-test",
            "file.txt",
        ],
    );
    assert_eq!(content, "agent edit\n");
    assert_eq!(
        fs::read_to_string(root.join("file.txt")).unwrap(),
        "dirty source\n"
    );
}
