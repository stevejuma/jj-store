use super::*;
use axum::http::StatusCode;

#[tokio::test]
async fn disabled_source_listing_requires_no_repository_or_capture_state() {
    let f = Fixture::new(false).await;
    let page = successful(f.run(&["sources", "--limit", "1"]).await);
    assert_eq!(
        page,
        json!({"version":1,"enabled":false,"runtime_generation":null,"phase":null,"total":0,"sources":[],"next_cursor":null})
    );
    for args in [
        vec!["sources", "--limit", "0"],
        vec!["sources", "--limit", "101"],
        vec!["sources", "--cursor", "private-response-marker"],
        vec!["sources", "--actor", "01010101010101010101010101010101"],
        vec!["sources", "--state-dir", "private-state"],
    ] {
        assert!(!f.run(&args).await.status.success());
    }
    assert!(!f.config.state_dir.join("agent-capture").exists());
    assert_eq!(std::fs::read_dir(f.cwd.path()).unwrap().count(), 0);
    f.close().await;
}

fn page() -> serde_json::Value {
    let id = format!("{GENERATION}:0000000000000001");
    json!({"version":1,"enabled":true,"runtime_generation":GENERATION,"phase":"paused","total":2,
        "sources":[{"selector":id,"profile":"codex-cli-jsonl-v1","product_version":"0.156.1","phase":"paused","observed":true,"awaiting_activity":true,"caught_up":false,"running":false,"degraded":false,"last_bound_repository":null,"ingress_required_capacity_bytes":null}],"next_cursor":id})
}

#[tokio::test]
async fn source_listing_uses_one_encoded_request_and_safe_cached_output() {
    let cwd = tempfile::tempdir().unwrap();
    let body = page();
    let cursor = format!("{GENERATION}:0000000000000001");
    let (base, calls, stop, task) = transport::serve(body.clone(), StatusCode::OK, false).await;
    assert_eq!(
        successful(run(cwd.path(), &base, &["sources", "--limit", "1"]).await),
        body
    );
    let mut next = body.clone();
    let second = format!("{GENERATION}:0000000000000002");
    next["sources"][0]["selector"] = json!(second);
    next["next_cursor"] = json!(null);
    let (next_base, next_calls, next_stop, next_task) =
        transport::serve(next.clone(), StatusCode::OK, false).await;
    assert_eq!(
        successful(
            run(
                cwd.path(),
                &next_base,
                &["sources", "--limit", "1", "--cursor", &cursor]
            )
            .await
        ),
        next
    );
    assert_eq!(
        *next_calls.lock().unwrap(),
        vec![(
            "GET".into(),
            format!("/v1/local/capture/sources?limit=1&cursor={GENERATION}%3A0000000000000001"),
            serde_json::Value::Null
        )]
    );
    let text = run_format(cwd.path(), &base, "text", &["sources", "--limit", "1"]).await;
    assert!(text.status.success());
    let text = String::from_utf8(text.stdout).unwrap();
    assert!(text.contains("Codex CLI") && text.contains("reader Paused") && text.contains(&cursor));
    assert_eq!(calls.lock().unwrap().len(), 2);
    for invalid in [
        "界".repeat(16) + ":",
        format!("{GENERATION}:0000000000000000"),
        cursor.to_uppercase(),
    ] {
        assert!(
            !run(cwd.path(), &base, &["sources", "--cursor", &invalid])
                .await
                .status
                .success()
        );
    }
    assert_eq!(calls.lock().unwrap().len(), 2);
    assert_eq!(std::fs::read_dir(cwd.path()).unwrap().count(), 0);
    let _ = stop.send(());
    let _ = next_stop.send(());
    task.await.unwrap();
    next_task.await.unwrap();
}

#[tokio::test]
async fn source_listing_rejects_stale_and_malformed_pages_without_private_output() {
    let cwd = tempfile::tempdir().unwrap();
    let original = page();
    let mut cases = vec![];
    for (pointer, value) in [
        ("/version", json!(2)),
        ("/total", json!(4097)),
        ("/runtime_generation", json!("private-response-marker")),
        ("/sources/0/selector", json!("private-response-marker")),
        (
            "/sources/0/product_version",
            json!("private-response-marker".repeat(3)),
        ),
        (
            "/next_cursor",
            json!(format!("{GENERATION}:0000000000000002")),
        ),
        ("/enabled", json!(false)),
    ] {
        let mut body = original.clone();
        *body.pointer_mut(pointer).unwrap() = value;
        cases.push((body, StatusCode::OK, "capture source page unavailable"));
    }
    let mut unexpected = original.clone();
    unexpected["source_path"] = json!("private-response-marker");
    cases.push((unexpected, StatusCode::OK, "capture status unavailable"));
    let mut duplicate = original.clone();
    duplicate["sources"]
        .as_array_mut()
        .unwrap()
        .push(original["sources"][0].clone());
    cases.push((duplicate, StatusCode::OK, "capture source page unavailable"));
    cases.push((
        json!({"code":"capture_source_selection_changed","private":"private-response-marker"}),
        StatusCode::CONFLICT,
        "capture source selection changed; list sources again",
    ));
    for (body, status, message) in cases {
        let (base, calls, stop, task) = transport::serve(body, status, false).await;
        let output = run(cwd.path(), &base, &["sources"]).await;
        rejected(output.clone(), message);
        assert!(!String::from_utf8_lossy(&output.stderr).contains("private-response-marker"));
        assert_eq!(calls.lock().unwrap().len(), 1);
        let _ = stop.send(());
        task.await.unwrap();
    }
}
