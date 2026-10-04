use super::*;
use serde_json::json;

async fn call(router: &Router, path: &str, body: Option<Value>) -> (StatusCode, Value) {
    let mut request = Request::builder()
        .uri(format!("/v1/repos/{REPO}/{path}"))
        .header("host", "127.0.0.1:7443")
        .header("x-jj-store-protocol", "1");
    let bytes = if let Some(body) = body {
        request = request
            .method("POST")
            .header("content-type", "application/json");
        serde_json::to_vec(&body).unwrap()
    } else {
        vec![]
    };
    let response = router
        .clone()
        .oneshot(request.body(Body::from(bytes)).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let data = serde_json::from_slice(
        &to_bytes(response.into_body(), 16 * 1024 * 1024)
            .await
            .unwrap(),
    )
    .unwrap();
    (status, data)
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn managed_native_http_preserves_light_reads_restarts_and_captures_tasks() {
    let f = ContentFixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let config = Config {
        access: Access::LocalTrusted {
            principal: m::PrincipalId::new([33; 16]),
            permission: Permission::Read,
        },
        ..Default::default()
    };
    let router = api(&f, cache.path(), config.clone());
    let operation = f.read.snapshot().await.unwrap().operation_id;
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let (status, caps) = call(&router, "capabilities", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(caps["automatic_code_index"], true);
    let (status, refs) = call(
        &router,
        &format!("references?at_op={operation}&kind=bookmarks&limit=1"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{refs}");
    let mut samples = vec![];
    let (status, task) = call(
        &router,
        "code-index/tasks",
        Some(json!({"operation": operation})),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED, "{task}");
    samples.push(json!({"method":"post","path":"/v1/repos/{repo}/code-index/tasks","status":202,"media":"application/json","data":task}));
    let id = task["id"].as_str().unwrap();
    let ready = tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            let (status, data) = call(&router, &format!("code-index/tasks/{id}"), None).await;
            assert_eq!(status, StatusCode::OK, "{data}");
            if data["status"] != "running" && data["status"] != "canceling" {
                break data;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    assert_eq!(ready["status"], "ready", "{ready}");
    samples.push(json!({"method":"get","path":"/v1/repos/{repo}/code-index/tasks/{task}","status":200,"media":"application/json","data":ready}));
    let (status, canceled) = call(
        &router,
        &format!("code-index/tasks/{id}/cancel"),
        Some(json!({})),
    )
    .await;
    // Cancel is bodyless, so confirm admission rejects a JSON object first.
    assert_eq!(status, StatusCode::BAD_REQUEST, "{canceled}");
    let request = Request::builder()
        .method("POST")
        .uri(format!("/v1/repos/{REPO}/code-index/tasks/{id}/cancel"))
        .header("host", "127.0.0.1:7443")
        .header("x-jj-store-protocol", "1")
        .body(Body::empty())
        .unwrap();
    let response = router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let data: Value = serde_json::from_slice(
        &to_bytes(response.into_body(), 16 * 1024 * 1024)
            .await
            .unwrap(),
    )
    .unwrap();
    samples.push(json!({"method":"post","path":"/v1/repos/{repo}/code-index/tasks/{task}/cancel","status":200,"media":"application/json","data":data}));
    let (status, log) = call(
        &router,
        &format!("log?at_op={operation}&limit=1&revset=all()&user_email=&at_time_millis=0"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{log}");
    assert_eq!(before, *f.f.session.snapshot().await.unwrap().manifest());
    drop(router);
    let restarted = api(&f, cache.path(), config);
    let (status, _) = call(&restarted, &format!("code-index/tasks/{id}"), None).await;
    assert_eq!(status, StatusCode::GONE);
    let (status, next) = call(
        &restarted,
        "code-index/tasks",
        Some(json!({"operation":operation})),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED, "{next}");
    let authenticated = api(&f, tempfile::tempdir().unwrap().path(), Config::default());
    let req = Request::builder()
        .method("POST")
        .uri(format!("/v1/repos/{REPO}/code-index/tasks"))
        .header("host", "127.0.0.1:7443")
        .header("x-jj-store-protocol", "1")
        .header(
            "authorization",
            format!("Bearer {}", f.f.token.expose().as_str()),
        )
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_vec(&json!({"operation":operation})).unwrap(),
        ))
        .unwrap();
    assert_eq!(
        authenticated.oneshot(req).await.unwrap().status(),
        StatusCode::FORBIDDEN
    );
    if let Some(path) = std::env::var_os("JJ_STORE_MANAGED_NATIVE_CAPTURE") {
        std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
}
