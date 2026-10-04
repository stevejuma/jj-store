#![cfg(unix)]
#[path = "capture_cli/fixture.rs"]
mod fixture;
#[path = "capture_cli/imports.rs"]
mod imports;
#[path = "capture_cli/sources.rs"]
mod sources;
#[path = "capture_cli/transport.rs"]
mod transport;
use fixture::*;
use serde_json::json;

#[tokio::test]
async fn disabled_capture_status_requires_no_repository_or_device_state() {
    let f = Fixture::new(false).await;
    let value = successful(f.run(&["status"]).await);
    assert_eq!(
        value,
        json!({"version":1,"enabled":false,"policy":null,"runtime":null})
    );
    assert!(!f.config.state_dir.join("agent-capture").exists());
    assert_eq!(std::fs::read_dir(f.cwd.path()).unwrap().count(), 0);
    f.close().await;
}

#[tokio::test]
async fn capture_controls_preserve_exact_receipt_and_current_policy() {
    let f = Fixture::new(true).await;
    let status = successful(f.run(&["status"]).await);
    let generation = status["policy"]["generation"].as_str().unwrap();
    let request = "01010101010101010101010101010101";
    let pause = [
        "pause",
        "--generation",
        generation,
        "--expected-revision",
        "0",
        "--request-id",
        request,
    ];
    let first = successful(f.run(&pause).await);
    assert_eq!(first["receipt"]["revision"], 1);
    assert_eq!(first["status"]["policy"]["paused"], true);
    assert_eq!(first["runtime_requested"], false);
    assert_eq!(first["replayed"], false);
    let replay = successful(f.run(&pause).await);
    assert_eq!(replay["receipt"], first["receipt"]);
    assert_eq!(replay["replayed"], true);
    rejected(
        f.run(&[
            "rescan",
            "--generation",
            generation,
            "--expected-revision",
            "1",
        ])
        .await,
        "capture is paused",
    );
    successful(
        f.run(&[
            "resume",
            "--generation",
            generation,
            "--expected-revision",
            "1",
            "--request-id",
            "02020202020202020202020202020202",
        ])
        .await,
    );
    let selected = f.bytes();
    let replay = successful(f.run(&pause).await);
    assert_eq!(replay["receipt"], first["receipt"]);
    assert_eq!(replay["status"]["policy"]["revision"], 2);
    assert_eq!(replay["status"]["policy"]["paused"], false);
    for (args, message) in [
        (
            vec![
                "resume",
                "--generation",
                generation,
                "--expected-revision",
                "0",
                "--request-id",
                request,
            ],
            "capture control intent conflicts",
        ),
        (
            vec![
                "pause",
                "--generation",
                generation,
                "--expected-revision",
                "0",
                "--request-id",
                "03030303030303030303030303030303",
            ],
            "capture policy changed",
        ),
        (
            vec![
                "pause",
                "--generation",
                GENERATION,
                "--expected-revision",
                "2",
                "--request-id",
                "03030303030303030303030303030303",
            ],
            "capture policy changed",
        ),
        (
            vec![
                "pause",
                "--generation",
                generation,
                "--expected-revision",
                "2",
                "--request-id",
                "00000000000000000000000000000000",
            ],
            "request identity must be nonzero",
        ),
    ] {
        rejected(f.run(&args).await, message);
        assert_eq!(f.bytes(), selected);
    }
    std::fs::write(
        f.root.path().join("server.toml"),
        f.text
            .replace("permission = \"admin\"", "permission = \"write\""),
    )
    .unwrap();
    rejected(f.run(&["status"]).await, "capture management denied");
    rejected(f.run(&pause).await, "capture management denied");
    assert_eq!(f.bytes(), selected);
    assert_eq!(std::fs::read_dir(f.cwd.path()).unwrap().count(), 0);
    f.close().await;
}

#[tokio::test]
async fn capture_inputs_cannot_initialize_state_or_select_actor() {
    let f = Fixture::new(false).await;
    rejected(
        f.run(&[
            "pause",
            "--generation",
            GENERATION,
            "--expected-revision",
            "0",
            "--request-id",
            "01010101010101010101010101010101",
        ])
        .await,
        "capture is disabled",
    );
    for args in [
        vec!["status", "--actor", "01010101010101010101010101010101"],
        vec!["status", "--state-dir", "private-state"],
        vec![
            "rescan",
            "--generation",
            "ABABABABABABABABABABABABABABABAB",
            "--expected-revision",
            "0",
        ],
        vec![
            "rescan",
            "--generation",
            "00000000000000000000000000000000",
            "--expected-revision",
            "0",
        ],
        vec![
            "pause",
            "--generation",
            GENERATION,
            "--expected-revision",
            "0",
        ],
    ] {
        assert!(!f.run(&args).await.status.success());
    }
    assert!(!f.config.state_dir.join("agent-capture").exists());
    assert_eq!(std::fs::read_dir(f.cwd.path()).unwrap().count(), 0);
    f.close().await;
}

fn rejected(output: std::process::Output, message: &str) {
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains(message),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
