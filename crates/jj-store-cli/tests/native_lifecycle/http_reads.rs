use super::*;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use jj_store_http::{Config, HttpApi};
use jj_store_service::administration::{OperatorPolicies, RepositoryHost};
use serde_json::Value;
use tower::ServiceExt as _;

struct NoOperators;
#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        panic!("read API accessed operator policy")
    }
}
fn api(f: &ContentFixture, cache: &std::path::Path, config: Config) -> Router {
    let host = RepositoryHost::new(
        f.f.store.clone(),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap();
    HttpApi::new(host, cache.to_owned(), config)
        .unwrap()
        .router()
}
fn request(token: &BearerToken, path: &str) -> Request<Body> {
    Request::builder()
        .uri(format!("/v1/repos/{REPO}/{path}"))
        .header(
            "authorization",
            format!("Bearer {}", token.expose().as_str()),
        )
        .body(Body::empty())
        .unwrap()
}

/// Captures real handler serialization for the offline JSON Schema acceptance lane.
/// Normal Rust runs exercise the same fixture without writing an artifact.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_contract_capture_matches_registered_routes() {
    let f = ContentFixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(&f, cache.path(), Config::default());
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let operation = f.read.snapshot().await.unwrap().operation_id;
    let id = m::CommitId::from_slice(f.commit.key().as_bytes()).unwrap();
    let mut samples = vec![];
    let mut record = |path: &str, method: &str, status: u16, media: &str, data: Value| {
        samples.push(serde_json::json!({"path":format!("/v1/repos/{{repo}}/{path}"),"method":method,"status":status,"media":media,"data":data}));
    };
    for (template, path) in [
        ("capabilities", "capabilities".into()),
        ("snapshot", "snapshot".into()),
        (
            "log",
            format!("log?at_time_millis=1790000000000&at_op={operation}"),
        ),
        ("operations", format!("operations?heads={operation}")),
        ("operations/{operation}", format!("operations/{operation}")),
        (
            "operations/{operation}/diff",
            format!("operations/{operation}/diff"),
        ),
        (
            "evolution",
            format!("evolution?commit={id}&at_op={operation}"),
        ),
        ("commits/{commit}", format!("commits/{id}")),
        (
            "commits/{commit}/tree",
            format!("commits/{id}/tree?limit=2"),
        ),
        (
            "commits/{commit}/conflicts",
            format!("commits/{id}/conflicts"),
        ),
        ("diff", format!("diff?from={id}&to={id}&content=true")),
    ] {
        let (status, body) = json(&router, &f.f.reader, &path).await;
        assert_eq!(status, StatusCode::OK, "{path}: {body}");
        record(template, "get", status.as_u16(), "application/json", body);
    }
    for (kind, key) in [
        ("commit", f.commit.key()),
        ("tree", f.tree.key()),
        ("copy", f.copy.key()),
        ("view", f.view.key()),
        ("operation", f.operation.key()),
        ("file", m::ObjectKey::File(f.file)),
        ("symlink", f.symlink),
    ] {
        let id = m::CommitId::from_slice(key.as_bytes()).unwrap();
        let (status, body) = json(&router, &f.f.reader, &format!("objects/{kind}/{id}")).await;
        assert_eq!(status, StatusCode::OK, "{kind}: {body}");
        record("objects/{kind}/{id}", "get", 200, "application/json", body);
    }
    let mut prepare = request(&f.f.reader, "index/tasks");
    *prepare.method_mut() = axum::http::Method::POST;
    prepare
        .headers_mut()
        .insert("content-type", "application/json".parse().unwrap());
    *prepare.body_mut() =
        Body::from(serde_json::to_vec(&serde_json::json!({"operation":operation})).unwrap());
    let response = router.clone().oneshot(prepare).await.unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);
    let task: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 64 * 1024).await.unwrap()).unwrap();
    let task_id = task["id"].as_str().unwrap().to_owned();
    record("index/tasks", "post", 202, "application/json", task);
    let mut done = false;
    for _ in 0..100 {
        let (status, task) = json(&router, &f.f.reader, &format!("index/tasks/{task_id}")).await;
        assert_eq!(status, StatusCode::OK);
        if task["status"] == "ready" {
            record("index/tasks/{task}", "get", 200, "application/json", task);
            done = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    assert!(done, "small index fixture did not complete");
    let mut cancel = request(&f.f.reader, &format!("index/tasks/{task_id}/cancel"));
    *cancel.method_mut() = axum::http::Method::POST;
    let response = router.clone().oneshot(cancel).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let task =
        serde_json::from_slice(&to_bytes(response.into_body(), 64 * 1024).await.unwrap()).unwrap();
    record(
        "index/tasks/{task}/cancel",
        "post",
        200,
        "application/json",
        task,
    );
    for path in [
        "events".into(),
        format!("events?after=v1:{REPO}:{}:{}", before.epoch, u64::MAX),
    ] {
        let response = router
            .clone()
            .oneshot(request(&f.f.reader, &path))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let mut stream = response.into_body().into_data_stream();
        let chunk = tokio::time::timeout(Duration::from_secs(2), stream.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        let text = std::str::from_utf8(&chunk).unwrap();
        let data = text
            .lines()
            .find_map(|line| line.strip_prefix("data: "))
            .unwrap();
        record(
            "events",
            "get",
            200,
            "text/event-stream",
            serde_json::from_str(data).unwrap(),
        );
    }
    let (status, error) = json(&router, &f.f.reader, "snapshot?unknown=1").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    record("snapshot", "get", 400, "application/json", error);
    // Range media itself is binary. Capture a real range error's JSON contract.
    let mut range = request(&f.f.reader, &format!("files/{}", f.empty));
    range
        .headers_mut()
        .insert("range", "bytes=0-1".parse().unwrap());
    let response = router.clone().oneshot(range).await.unwrap();
    assert_eq!(response.status(), StatusCode::RANGE_NOT_SATISFIABLE);
    record(
        "files/{file}",
        "get",
        416,
        "application/json",
        serde_json::from_slice(&to_bytes(response.into_body(), 64 * 1024).await.unwrap()).unwrap(),
    );
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
    if let Some(path) = std::env::var_os("JJ_STORE_CONTRACT_CAPTURE") {
        std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
}
async fn json(router: &Router, token: &BearerToken, path: &str) -> (StatusCode, Value) {
    let response = router.clone().oneshot(request(token, path)).await.unwrap();
    assert_eq!(response.headers()["cache-control"], "no-store");
    assert_eq!(response.headers()["x-content-type-options"], "nosniff");
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 32 * 1024 * 1024)
        .await
        .unwrap();
    (status, serde_json::from_slice(&bytes).unwrap())
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_reads_match_native_snapshots_objects_graphs_and_differences_without_publication() {
    let f = ContentFixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(&f, cache.path(), Config::default());
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let (status, snapshot) = json(&router, &f.f.reader, "snapshot").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        snapshot["heads"],
        serde_json::json!([f.read.snapshot().await.unwrap().operation_id])
    );
    let id = m::CommitId::from_slice(f.commit.key().as_bytes()).unwrap();
    let (status, object) = json(&router, &f.f.reader, &format!("objects/commit/{id}")).await;
    assert_eq!(status, StatusCode::OK);
    let reader = ContentReader::new(c::Limits::default(), 1).unwrap();
    let native = f.read.object(&reader, f.commit.key()).await.unwrap();
    assert_eq!(
        object["data"]["data"],
        serde_json::to_value(native.data).unwrap()
    );
    let operation = f.read.snapshot().await.unwrap().operation_id;
    for path in [
        "capabilities".to_owned(),
        format!("log?at_time_millis=1790000000000&revset=all()&at_op={operation}"),
        format!("operations?heads={operation}"),
        format!("operations/{operation}"),
        format!("operations/{operation}/diff"),
        format!("evolution?commit={id}&at_op={operation}"),
        format!("commits/{id}"),
        format!("commits/{id}/tree?limit=2"),
        format!("commits/{id}/conflicts"),
        format!("diff?from={id}&to={id}&content=true"),
    ] {
        let (status, body) = json(&router, &f.f.reader, &path).await;
        assert_eq!(status, StatusCode::OK, "{path}: {body}");
    }
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_reads_authentication_queries_and_binary_ranges_fail_closed() {
    let f = ContentFixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(&f, cache.path(), Config::default());
    for (path, code) in [
        ("snapshot?unknown=1", StatusCode::BAD_REQUEST),
        ("objects/commit/nope", StatusCode::BAD_REQUEST),
    ] {
        assert_eq!(json(&router, &f.f.reader, path).await.0, code);
    }
    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/v1/repos/{REPO}/snapshot"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    for (range, start, end) in [
        ("bytes=2-7", 2, 8),
        ("bytes=-7", f.binary.len() - 7, f.binary.len()),
        ("bytes=199990-", 199990, 200000),
        ("bytes=199998-999999", 199998, 200000),
    ] {
        let mut req = request(&f.f.reader, &format!("files/{}", f.file));
        req.headers_mut().insert("range", range.parse().unwrap());
        let response = router.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::PARTIAL_CONTENT);
        assert_eq!(
            response.headers()["content-type"],
            "application/octet-stream"
        );
        assert!(
            response.headers()["content-disposition"]
                .to_str()
                .unwrap()
                .starts_with("attachment;")
        );
        assert_eq!(
            response.headers()["content-range"],
            format!("bytes {start}-{}/{len}", end - 1, len = f.binary.len())
        );
        assert_eq!(
            to_bytes(response.into_body(), 300_000)
                .await
                .unwrap()
                .as_ref(),
            &f.binary[start..end]
        );
    }
    let mut req = request(&f.f.reader, &format!("files/{}", f.file));
    req.headers_mut()
        .insert("range", "bytes=0-1,4-5".parse().unwrap());
    assert_eq!(
        router.clone().oneshot(req).await.unwrap().status(),
        StatusCode::RANGE_NOT_SATISFIABLE
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_reads_revocation_stops_inflight_media_and_credentials_never_become_query_data() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    let f = ContentFixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(&f, cache.path(), Config::default());
    let response = router
        .clone()
        .oneshot(request(&f.f.reader, &format!("files/{}", f.file)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let mut stream = response.into_body().into_data_stream();
    assert_eq!(
        stream.next().await.unwrap().unwrap().as_ref(),
        &f.binary[..65536]
    );
    let before = f.f.session.snapshot().await.unwrap().manifest().snapshot();
    f.f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([217; 16]),
            expected: before,
            change: TokenChange::Revoke {
                token: f.f.reader.id(),
            },
        })
        .await
        .unwrap();
    assert!(stream.next().await.unwrap().is_err());
    let (status, body) = json(&router, &f.f.reader, "snapshot").await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["code"], "access_denied");
    assert!(!body.to_string().contains(f.f.reader.expose().as_str()));
    let mut duplicate = request(&f.f.token, "snapshot");
    duplicate
        .headers_mut()
        .append("authorization", "Bearer invalid".parse().unwrap());
    assert_eq!(
        router.clone().oneshot(duplicate).await.unwrap().status(),
        StatusCode::UNAUTHORIZED
    );
    let unknown = Request::builder()
        .uri(format!(
            "/v1/repos/{}/snapshot",
            m::RepositoryId::new([219; 16])
        ))
        .header(
            "authorization",
            format!("Bearer {}", f.f.token.expose().as_str()),
        )
        .body(Body::empty())
        .unwrap();
    assert_eq!(
        router.clone().oneshot(unknown).await.unwrap().status(),
        StatusCode::FORBIDDEN
    );
    let mut cross_origin = request(&f.f.token, "snapshot");
    cross_origin
        .headers_mut()
        .insert("origin", "https://untrusted.invalid".parse().unwrap());
    assert_eq!(
        router.clone().oneshot(cross_origin).await.unwrap().status(),
        StatusCode::FORBIDDEN
    );
    let cookie = Request::builder()
        .uri(format!("/v1/repos/{REPO}/snapshot"))
        .header("cookie", format!("token={}", f.f.token.expose().as_str()))
        .body(Body::empty())
        .unwrap();
    assert_eq!(
        router.clone().oneshot(cookie).await.unwrap().status(),
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_reads_unpolled_response_expiry_releases_transport_and_native_slots() {
    let f = ContentFixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(
        &f,
        cache.path(),
        Config {
            concurrency: 1,
            workers: 1,
            timeout: Duration::from_secs(1),
            ..Config::default()
        },
    );
    let response = router
        .clone()
        .oneshot(request(&f.f.reader, &format!("files/{}", f.file)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        json(&router, &f.f.reader, "capabilities").await.0,
        StatusCode::SERVICE_UNAVAILABLE
    );
    tokio::time::sleep(Duration::from_millis(1100)).await;
    assert_eq!(
        json(&router, &f.f.reader, "capabilities").await.0,
        StatusCode::OK
    );
    assert!(to_bytes(response.into_body(), 300_000).await.is_err());
    let response = router
        .clone()
        .oneshot(request(&f.f.reader, &format!("files/{}", f.file)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    drop(response);
    assert_eq!(
        json(&router, &f.f.reader, "snapshot").await.0,
        StatusCode::OK
    );
    let limited = api(
        &f,
        cache.path(),
        Config {
            max_json_bytes: 128,
            max_uri_bytes: 512,
            ..Config::default()
        },
    );
    let id = m::CommitId::from_slice(f.commit.key().as_bytes()).unwrap();
    let (status, body) = json(&limited, &f.f.reader, &format!("objects/commit/{id}")).await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(body["code"], "resource_limit");
    assert_eq!(
        json(
            &limited,
            &f.f.reader,
            &format!("snapshot?{}", "x".repeat(600))
        )
        .await
        .0,
        StatusCode::PAYLOAD_TOO_LARGE
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_mutation_previews_share_native_worker_admission_with_file_reads() {
    let f = ContentFixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(
        &f,
        cache.path(),
        Config {
            workers: 1,
            ..Config::default()
        },
    );
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let response = router
        .clone()
        .oneshot(request(&f.f.reader, &format!("files/{}", f.file)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let make = || {
        let mut req = request(&f.f.token, "mutations/preview");
        *req.method_mut() = axum::http::Method::POST;
        req.headers_mut()
            .insert("content-type", "application/json".parse().unwrap());
        *req.body_mut() = Body::from(serde_json::json!({"version":1,"request_id":m::RequestId::new([240;16]),"expected":before.snapshot(),"command":{"type":"new","parents":[roots::ROOT_COMMIT],"description":"admission","workspace":null}}).to_string());
        req
    };
    let busy = router.clone().oneshot(make()).await.unwrap();
    assert_eq!(busy.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body: Value =
        serde_json::from_slice(&to_bytes(busy.into_body(), 4096).await.unwrap()).unwrap();
    assert_eq!(body["code"], "busy");
    drop(response);
    let available = router.oneshot(make()).await.unwrap();
    assert_eq!(available.status(), StatusCode::OK);
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_reads_index_task_and_signed_cursor_preserve_snapshot_across_publication() {
    let f = ContentFixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(
        &f,
        cache.path(),
        Config {
            graph: q::Limits {
                max_inline_rebuild_commits: 0,
                ..q::Limits::default()
            },
            ..Config::default()
        },
    );
    let path = "log?at_time_millis=1790000000000&revset=all()&limit=1";
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let (status, rebuild) = json(&router, &f.f.reader, path).await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(rebuild["code"], "index_rebuild_required");
    let operation = rebuild["snapshot"]["operation_id"].as_str().unwrap();
    let mut req = request(&f.f.reader, "index/tasks");
    *req.method_mut() = axum::http::Method::POST;
    req.headers_mut()
        .insert("content-type", "application/json".parse().unwrap());
    *req.body_mut() = Body::from(serde_json::json!({"operation":operation}).to_string());
    let response = router.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);
    let task: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 65536).await.unwrap()).unwrap();
    let task_path = format!("index/tasks/{}", task["id"].as_str().unwrap());
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let (status, body) = json(&router, &f.f.reader, &task_path).await;
            assert_eq!(status, StatusCode::OK);
            match body["status"].as_str().unwrap() {
                "ready" => break,
                "running" => tokio::time::sleep(Duration::from_millis(5)).await,
                _ => panic!("task failed: {body}"),
            }
        }
    })
    .await
    .unwrap();
    let (status, first) = json(&router, &f.f.reader, path).await;
    assert_eq!(status, StatusCode::OK);
    let cursor = first["next_cursor"].as_str().unwrap();
    let original: m::OperationId = operation.parse().unwrap();
    let mut later = f.operation.clone();
    later.parents = vec![o::OperationId::from_bytes(original.as_bytes())];
    later.metadata.description = "later publication".into();
    let key = f.f.objects.put(&later).await.unwrap().0;
    f.f.session
        .publish(
            BTreeSet::from([original]),
            m::OperationId::from_slice(key.as_bytes()).unwrap(),
        )
        .await
        .unwrap();
    let (status, second) = json(&router, &f.f.reader, &format!("{path}&cursor={cursor}")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(second["snapshot"], first["snapshot"]);
    assert_eq!(
        json(
            &router,
            &f.f.reader,
            &format!("{}&cursor={cursor}", path.replace("limit=1", "limit=2"))
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    later.metadata.description = "concurrent sibling".into();
    let key = f.f.objects.put(&later).await.unwrap().0;
    f.f.session
        .publish(
            BTreeSet::from([original]),
            m::OperationId::from_slice(key.as_bytes()).unwrap(),
        )
        .await
        .unwrap();
    let (status, heads) = json(&router, &f.f.reader, "operations").await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(heads["code"], "select_operation");
    assert_eq!(heads["heads"]["heads"].as_array().unwrap().len(), 2);
    assert_eq!(
        f.f.session.snapshot().await.unwrap().manifest().published.0,
        before.published.0 + 2
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_reads_range_selection_cannot_reset_consumed_or_preselected_files() {
    let f = ContentFixture::new().await;
    let reader = ContentReader::new(c::Limits::default(), 1).unwrap();
    let mut file = f.read.file(&reader, f.file, None).await.unwrap();
    file.next_chunk().await.unwrap();
    assert_eq!(
        file.select_range(ByteRange::new(0, 1).unwrap())
            .await
            .unwrap_err(),
        q::Error::Range
    );
    let file = f
        .read
        .file(&reader, f.file, Some(ByteRange::new(0, 1).unwrap()))
        .await
        .unwrap();
    assert_eq!(
        file.select_range(ByteRange::new(0, 1).unwrap())
            .await
            .unwrap_err(),
        q::Error::Range
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn http_reads_reject_unframed_request_content() {
    let f = ContentFixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(&f, cache.path(), Config::default());
    let mut req = request(&f.f.reader, "snapshot");
    *req.body_mut() = Body::from_stream(futures::stream::iter([Ok::<_, std::io::Error>(
        axum::body::Bytes::from_static(b"unexpected"),
    )]));
    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
