use super::*;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::any,
};
use serde_json::Value;
use std::sync::{Arc, Mutex};

pub(super) type Calls = Arc<Mutex<Vec<(String, String, Value)>>>;
fn status() -> Value {
    json!({"version":1,"enabled":true,"policy":{"generation":GENERATION,"revision":0,"paused":false},"runtime":null})
}
pub(super) async fn serve(
    body: Value,
    code: StatusCode,
    redirect: bool,
) -> (
    String,
    Calls,
    tokio::sync::oneshot::Sender<()>,
    tokio::task::JoinHandle<()>,
) {
    let calls: Calls = Default::default();
    let recorded = calls.clone();
    let app = Router::new().fallback(any(move |request: Request<Body>| {
        let recorded = recorded.clone();
        let body = body.clone();
        async move {
            assert_eq!(request.headers()["x-jj-store-protocol"], "1");
            assert!(!request.headers().contains_key("authorization"));
            let method = request.method().as_str().to_owned();
            let path = request.uri().to_string();
            let bytes = to_bytes(request.into_body(), 4096).await.unwrap();
            let input = if bytes.is_empty() {
                Value::Null
            } else {
                serde_json::from_slice(&bytes).unwrap()
            };
            recorded.lock().unwrap().push((method, path, input));
            let mut response: Response = if body.get("oversized").is_some() {
                // Unknown-size streaming body exercises the actual chunk bound.
                let chunks = tokio_stream::iter(vec![Ok::<_, std::io::Error>(
                    serde_json::to_vec(&body).unwrap(),
                )]);
                Response::builder()
                    .status(code)
                    .header("content-type", "application/json")
                    .body(Body::from_stream(chunks))
                    .unwrap()
            } else {
                (code, axum::Json(body)).into_response()
            };
            if redirect {
                response
                    .headers_mut()
                    .insert("location", "/private-response-marker".parse().unwrap());
            }
            response
        }
    }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    (base, calls, stop, task)
}
#[tokio::test]
async fn capture_rescan_forwards_one_actor_free_conditional_hint_without_proxy_or_prefetch() {
    let cwd = tempfile::tempdir().unwrap();
    let response = json!({"version":1,"accepted":true,"status":status()});
    let (base, calls, stop, task) = serve(response.clone(), StatusCode::OK, false).await;
    assert_eq!(
        successful(
            run(
                cwd.path(),
                &base,
                &[
                    "rescan",
                    "--generation",
                    GENERATION,
                    "--expected-revision",
                    "0"
                ]
            )
            .await
        ),
        response
    );
    assert_eq!(
        *calls.lock().unwrap(),
        vec![(
            "POST".into(),
            "/v1/local/capture/rescan".into(),
            json!({"version":1,"generation":GENERATION,"expected_revision":0})
        )]
    );
    for url in [
        "http://localhost:7443",
        "https://127.0.0.1:7443",
        "http://192.0.2.1:7443",
        "http://user:private-response-marker@127.0.0.1",
        "http://127.0.0.1/path",
        "http://127.0.0.1?secret=private-response-marker",
        "http://127.0.0.1#private-response-marker",
    ] {
        let output = run(cwd.path(), url, &["status"]).await;
        rejected(output.clone(), "literal loopback address");
        assert!(!String::from_utf8_lossy(&output.stderr).contains("private-response-marker"));
    }
    assert_eq!(calls.lock().unwrap().len(), 1);
    assert_eq!(std::fs::read_dir(cwd.path()).unwrap().count(), 0);
    let _ = stop.send(());
    task.await.unwrap();
}
#[tokio::test]
async fn capture_unknown_responses_retain_original_command_without_redirect_or_retry() {
    let cwd = tempfile::tempdir().unwrap();
    let mut unknown_version = json!({"version":2,"receipt":{"generation":GENERATION,"request_id":"01010101010101010101010101010101","expected_revision":0,"revision":1,"paused":true},"replayed":false,"runtime_requested":false,"status":status()});
    unknown_version["status"]["version"] = json!(2);
    let mut foreign_receipt = unknown_version.clone();
    foreign_receipt["version"] = json!(1);
    foreign_receipt["status"]["version"] = json!(1);
    foreign_receipt["status"]["policy"]["revision"] = json!(1);
    foreign_receipt["receipt"]["request_id"] = json!("02020202020202020202020202020202");
    for (body, code, redirect, message) in [
        (
            json!({"code":"private-response-marker"}),
            StatusCode::FOUND,
            true,
            "outcome unknown",
        ),
        (
            json!({"version":1,"code":"capture_control_outcome_unknown","details":"private-response-marker"}),
            StatusCode::SERVICE_UNAVAILABLE,
            false,
            "outcome unknown",
        ),
        (
            json!({"unrecognized":"private-response-marker"}),
            StatusCode::OK,
            false,
            "outcome unknown",
        ),
        (
            json!({"oversized":"private-response-marker".repeat(4096)}),
            StatusCode::OK,
            false,
            "outcome unknown",
        ),
        (
            unknown_version,
            StatusCode::OK,
            false,
            "unsupported capture response version",
        ),
        (foreign_receipt, StatusCode::OK, false, "outcome unknown"),
    ] {
        let (base, calls, stop, task) = serve(body, code, redirect).await;
        let output = run(
            cwd.path(),
            &base,
            &[
                "pause",
                "--generation",
                GENERATION,
                "--expected-revision",
                "0",
                "--request-id",
                "01010101010101010101010101010101",
            ],
        )
        .await;
        assert!(!String::from_utf8_lossy(&output.stderr).contains("private-response-marker"));
        assert!(
            String::from_utf8_lossy(&output.stderr)
                .contains("original request identity, generation and revision")
        );
        rejected(output, message);
        assert_eq!(
            *calls.lock().unwrap(),
            vec![(
                "POST".into(),
                "/v1/local/capture/control".into(),
                json!({"version":1,"request_id":"01010101010101010101010101010101","generation":GENERATION,"expected_revision":0,"action":"pause"})
            )]
        );
        let _ = stop.send(());
        task.await.unwrap();
    }
}
#[tokio::test]
async fn capture_status_text_retains_policy_and_cached_harness_counts() {
    let cwd = tempfile::tempdir().unwrap();
    let mut body = status();
    body["runtime"] = json!({
        "phase":"paused","discovery":{"phase":"paused","roots":1,"unchecked_roots":0,"available_roots":1,"unavailable_roots":0,"rescan_pending":false},"degraded_sources":0,
        "harnesses":[{"profile":"codex-cli-jsonl-v1","product_version":"0.156.1","roots":1,"unchecked_roots":0,"available_roots":1,"unavailable_roots":0,"tracked":1,"observed_sources":1,"awaiting_activity":1,"caught_up":0,"running_sources":0,"degraded_sources":0}],
        "pending_capacity_bytes":65536,"ingress_capacity_refusals":0,"ingress_required_capacity_bytes":null,"delivery_required_capacity_bytes":null,"tracked":1,"observed_sources":1,"awaiting_activity":1,"caught_up":0,"running_sources":0,"resident_source_frames":0,"resident_delivery_frames":0,"pending_batches":2,"unavailable":0,"source_limit_reached":false
    });
    let (base, calls, stop, task) = serve(body.clone(), StatusCode::OK, false).await;
    assert_eq!(successful(run(cwd.path(), &base, &["status"]).await), body);
    let output = run_format(cwd.path(), &base, "text", &["status"]).await;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    for needle in [
        GENERATION,
        "revision 0",
        "0.156.1",
        "Codex CLI",
        "1/1 roots available",
        "1 awaiting activity",
        "2 pending batches",
    ] {
        assert!(text.contains(needle), "{text}");
    }
    assert_eq!(calls.lock().unwrap().len(), 2);
    let _ = stop.send(());
    task.await.unwrap();
}

#[tokio::test]
async fn capture_transport_loss_does_not_retry_or_replace_control_intent() {
    use tokio::io::AsyncReadExt as _;
    let cwd = tempfile::tempdir().unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut bytes = vec![0; 4096];
        let n = socket.read(&mut bytes).await.unwrap();
        assert!(
            String::from_utf8_lossy(&bytes[..n]).starts_with("POST /v1/local/capture/control ")
        );
        drop(socket);
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(300), listener.accept())
                .await
                .is_err(),
            "transport loss triggered a replacement request"
        );
    });
    let output = run(
        cwd.path(),
        &base,
        &[
            "pause",
            "--generation",
            GENERATION,
            "--expected-revision",
            "0",
            "--request-id",
            "01010101010101010101010101010101",
        ],
    )
    .await;
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("original request identity, generation and revision")
    );
    rejected(output, "capture command outcome unknown");
    task.await.unwrap();
}
