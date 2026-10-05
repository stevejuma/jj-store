use super::*;
use axum::http::StatusCode;
fn outcome() -> serde_json::Value {
    json!({"version":1,"replayed":false,"receipt":{"command":{"version":1,"request_id":"05050505050505050505050505050505", "generation":GENERATION,
        "expected_revision":0,"selector":format!("{GENERATION}:0000000000000001")},"ordinal":1,"profile":"codex-cli-jsonl-v1","product_version":"0.156.1",
        "complete_byte_offset":200,"complete_records":2},"progress":{"phase":"completed","boundary_available":true,"captured_byte_offset":200,"committed_byte_offset":200,
            "tail_phase":"withheld","tail":null}})
}
#[tokio::test]
async fn import_cli_forwards_once_and_withholds_foreign_or_impossible_acknowledgements() {
    let cwd = tempfile::tempdir().unwrap();
    let selector = format!("{GENERATION}:0000000000000001");
    for index in 0..5 {
        let mut body = outcome();
        match index {
            1 => {
                body["receipt"]["command"]["selector"] =
                    json!(format!("{GENERATION}:0000000000000002"))
            }
            2 => body["progress"]["committed_byte_offset"] = json!(199),
            3 => body["version"] = json!(2),
            4 => body["receipt"]["actor"] = json!("private-response-marker"),
            _ => {}
        }
        let (base, calls, stop, task) = transport::serve(body.clone(), StatusCode::OK, false).await;
        let output = run(
            cwd.path(),
            &base,
            &[
                "import",
                "--source",
                &selector,
                "--generation",
                GENERATION,
                "--expected-revision",
                "0",
                "--request-id",
                "05050505050505050505050505050505",
            ],
        )
        .await;
        if index == 0 {
            assert_eq!(successful(output), body)
        } else {
            let message = String::from_utf8_lossy(&output.stderr).to_string();
            rejected(output, "outcome unknown");
            assert!(!message.contains("private-response-marker"));
            assert!(message.contains("original request identity"));
        }
        assert_eq!(calls.lock().unwrap().len(), 1);
        assert_eq!(calls.lock().unwrap()[0].1, "/v1/local/capture/import");
        let _ = stop.send(());
        task.await.unwrap();
    }
    assert_eq!(std::fs::read_dir(cwd.path()).unwrap().count(), 0);
}
#[tokio::test]
async fn inspection_cli_binds_the_exact_request_and_rejects_unaccepted_completion() {
    let cwd = tempfile::tempdir().unwrap();
    for foreign in [false, true] {
        let body = json!({"version":1,"request_id":if foreign {"06060606060606060606060606060606"} else {"05050505050505050505050505050505"},
            "receipt":null,"progress":{"phase":"completed","boundary_available":true,"captured_byte_offset":0,"committed_byte_offset":0,"tail_phase":"unavailable","tail":null}});
        let (base, calls, stop, task) = transport::serve(body, StatusCode::OK, false).await;
        rejected(
            run(
                cwd.path(),
                &base,
                &[
                    "import-status",
                    "--request-id",
                    "05050505050505050505050505050505",
                ],
            )
            .await,
            "progress unavailable",
        );
        assert_eq!(calls.lock().unwrap().len(), 1);
        assert!(calls.lock().unwrap()[0].2.is_null());
        let _ = stop.send(());
        task.await.unwrap();
    }
}
