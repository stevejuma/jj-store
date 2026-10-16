//! Cold CLI contract: device configuration never snapshots or stores credentials.
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};
fn stock(root: &Path, args: &[&str]) -> String {
    let out = Command::new("jj")
        .current_dir(root)
        .args([
            "--no-pager",
            "--config",
            "user.name='Fixture'",
            "--config",
            "user.email='fixture@invalid'",
        ])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}
fn cli(root: &Path, state: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jj-store"))
        .current_dir(root)
        .args(["--no-pager", "--format=json", "stack"])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .env("JJ_STORE_FORGE_STATE_DIR", state)
        .env("JJ_STORE_FORGE_TOKEN", "fixture-secret-never-store")
        .output()
        .unwrap()
}
#[test]
fn configure_all_forges_preserves_operation_and_keeps_tokens_out_of_device_records() {
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    stock(root.path(), &["git", "init", "--colocate"]);
    fs::write(root.path().join("base"), "base\n").unwrap();
    stock(root.path(), &["commit", "-m", "base"]);
    fs::write(root.path().join("draft"), "not snapshotted\n").unwrap();
    let operation = stock(
        root.path(),
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
    );
    for (provider, url) in [
        ("github", "https://api.github.com"),
        ("gitlab", "https://gitlab.example/api/v4"),
        ("forgejo", "https://forge.example/api/v1"),
    ] {
        let out = cli(
            root.path(),
            state.path(),
            &[
                "pr",
                "configure",
                "--provider",
                provider,
                "--api-url",
                url,
                "--repository",
                "team/project",
            ],
        );
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(value["connection"]["provider"], provider);
    }
    assert_eq!(
        operation,
        stock(
            root.path(),
            &[
                "--ignore-working-copy",
                "op",
                "log",
                "--no-graph",
                "-n",
                "1",
                "-T",
                "id"
            ]
        )
    );
    assert!(!root.path().join(".jj/jj-store/stacks.toml").exists());
    for entry in fs::read_dir(state.path()).unwrap() {
        let bytes = fs::read(entry.unwrap().path()).unwrap();
        assert!(!String::from_utf8_lossy(&bytes).contains("fixture-secret-never-store"));
    }
    let out = cli(
        root.path(),
        state.path(),
        &[
            "pr",
            "configure",
            "--provider",
            "github",
            "--api-url",
            "https://private-secret@api.github.com",
            "--repository",
            "team/project",
        ],
    );
    assert!(!out.status.success());
    assert!(!String::from_utf8_lossy(&out.stderr).contains("private-secret"));
}
