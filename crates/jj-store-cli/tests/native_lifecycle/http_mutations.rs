use super::*;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request as HttpRequest, StatusCode},
};
use jj_store_http::{Config, HttpApi};
use jj_store_service::administration::{OperatorPolicies, RepositoryHost};
use serde_json::{Value, json};
use tower::ServiceExt as _;

struct NoOperators;
#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        panic!("mutation must not access operator policy")
    }
}
fn api(f: &Fixture, cache: &std::path::Path, config: Config, protection: Protection) -> Router {
    let host = RepositoryHost::new(
        f.store.clone(),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        protection,
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap();
    HttpApi::new(host, cache.to_owned(), config)
        .unwrap()
        .router()
}
fn request(token: &BearerToken, path: &str, body: Option<&Value>) -> HttpRequest<Body> {
    let mut builder = HttpRequest::builder()
        .uri(format!("/v1/repos/{REPO}/{path}"))
        .header(
            "authorization",
            format!("Bearer {}", token.expose().as_str()),
        );
    let body = if let Some(body) = body {
        builder = builder
            .method("POST")
            .header("content-type", "application/json");
        Body::from(serde_json::to_vec(body).unwrap())
    } else {
        Body::empty()
    };
    builder.body(body).unwrap()
}
async fn send(router: &Router, request: HttpRequest<Body>) -> (StatusCode, Value) {
    let response = router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.headers()["cache-control"], "no-store");
    assert_eq!(response.headers()["x-content-type-options"], "nosniff");
    let status = response.status();
    let data = to_bytes(response.into_body(), 2 * 1024 * 1024)
        .await
        .unwrap();
    (status, serde_json::from_slice(&data).unwrap())
}
async fn new_request(f: &Fixture, id: u8) -> Value {
    json!({"version":1,"request_id":m::RequestId::new([id;16]),"expected":f.session.snapshot().await.unwrap().manifest().snapshot(),"command":{"type":"new","parents":[roots::ROOT_COMMIT],"description":"HTTP native command","workspace":null}})
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_mutations_preview_apply_status_replay_and_contract_capture() {
    let f = Fixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(&f, cache.path(), Config::default(), Protection::default());
    let intent = new_request(&f, 121).await;
    let before = f.session.snapshot().await.unwrap().manifest().clone();
    let mut samples = vec![];
    let mut record = |path: &str, method: &str, status: StatusCode, data: &Value| {
        samples.push(json!({"path":format!("/v1/repos/{{repo}}/{path}"),"method":method,"status":status.as_u16(),"media":"application/json","data":data}));
    };
    let status_path = format!("mutations/{}", intent["request_id"].as_str().unwrap());
    let (status, unknown) = send(&router, request(&f.token, &status_path, None)).await;
    assert_eq!(status, StatusCode::OK, "{unknown}");
    assert_eq!(unknown["status"], "not_known");
    record("mutations/{request_id}", "get", status, &unknown);
    let (status, preview) = send(
        &router,
        request(&f.token, "mutations/preview", Some(&intent)),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{preview}");
    assert_eq!(preview["status"], "prepared");
    record("mutations/preview", "post", status, &preview);
    assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
    let (status, accepted) = send(&router, request(&f.token, "mutations", Some(&intent))).await;
    assert_eq!(status, StatusCode::OK, "{accepted}");
    assert_eq!(accepted["status"], "committed");
    assert_eq!(accepted["receipt"]["accepted"]["position"], 1);
    assert_eq!(
        accepted["receipt"]["request_digest"],
        preview["preview"]["request_digest"]
    );
    record("mutations", "post", status, &accepted);
    // Reconstruct transport state and query/apply the original durable receipt.
    let restarted = api(&f, cache.path(), Config::default(), Protection::default());
    for (path, method, body) in [
        (status_path.as_str(), "get", None),
        ("mutations", "post", Some(&intent)),
        ("mutations/preview", "post", Some(&intent)),
    ] {
        let (status, replay) = send(&restarted, request(&f.token, path, body)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(replay, accepted);
        record(
            if method == "get" {
                "mutations/{request_id}"
            } else {
                path
            },
            method,
            status,
            &replay,
        );
    }
    let (status, caps) = send(&router, request(&f.reader, "capabilities", None)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        caps["typed_mutations"],
        json!([
            "new",
            "describe",
            "bookmark_create",
            "bookmark_move",
            "bookmark_delete",
            "reconcile_operations",
            "abandon"
        ])
    );
    record("capabilities", "get", status, &caps);
    let small = api(
        &f,
        cache.path(),
        Config {
            max_json_bytes: 128,
            ..Default::default()
        },
        Protection::default(),
    );
    for (path, method, body) in [
        ("mutations", "post", Some(&intent)),
        (status_path.as_str(), "get", None),
    ] {
        let (status, unknown) = send(&small, request(&f.token, path, body)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(unknown["status"], "outcome_unknown");
        record(
            if method == "get" {
                "mutations/{request_id}"
            } else {
                path
            },
            method,
            status,
            &unknown,
        );
    }
    assert_eq!(
        f.session.snapshot().await.unwrap().manifest().published,
        m::WalPosition(1)
    );
    if let Some(path) = std::env::var_os("JJ_STORE_MUTATION_CONTRACT_CAPTURE") {
        std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_mutations_authentication_strict_json_limits_and_stale_requests_publish_nothing() {
    let f = Fixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(&f, cache.path(), Config::default(), Protection::default());
    let intent = new_request(&f, 122).await;
    let before = f.session.snapshot().await.unwrap().manifest().clone();
    for path in ["mutations", "mutations/preview"] {
        let (status, _) = send(&router, request(&f.reader, path, Some(&intent))).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let mut malformed = request(&f.reader, path, Some(&intent));
        *malformed.body_mut() = Body::from("invalid");
        assert_eq!(send(&router, malformed).await.0, StatusCode::FORBIDDEN);
        let mut missing = request(&f.token, path, Some(&intent));
        missing.headers_mut().remove("authorization");
        assert_eq!(send(&router, missing).await.0, StatusCode::UNAUTHORIZED);
        let mut origin = request(&f.token, path, Some(&intent));
        origin
            .headers_mut()
            .insert("origin", "https://elsewhere.invalid".parse().unwrap());
        assert_eq!(send(&router, origin).await.0, StatusCode::FORBIDDEN);
        let mut invalid = intent.clone();
        invalid["principal"] = json!(ACTOR);
        assert_eq!(
            send(&router, request(&f.token, path, Some(&invalid)))
                .await
                .0,
            StatusCode::BAD_REQUEST
        );
        let mut invalid = intent.clone();
        invalid["command"]["shell"] = json!("ignored?");
        assert_eq!(
            send(&router, request(&f.token, path, Some(&invalid)))
                .await
                .0,
            StatusCode::BAD_REQUEST
        );
        let mut large = request(&f.token, path, Some(&intent));
        *large.body_mut() = Body::from(vec![b' '; 512 * 1024 + 1]);
        assert_eq!(send(&router, large).await.0, StatusCode::PAYLOAD_TOO_LARGE);
        assert_eq!(
            send(
                &router,
                request(&f.token, &format!("{path}?extra=1"), Some(&intent))
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
    assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
    assert_eq!(
        send(&router, request(&f.token, "mutations", Some(&intent)))
            .await
            .0,
        StatusCode::OK
    );
    let after = f.session.snapshot().await.unwrap().manifest().clone();
    let mut stale = intent.clone();
    stale["request_id"] = json!(m::RequestId::new([123; 16]));
    let (status, error) = send(&router, request(&f.token, "mutations", Some(&stale))).await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(error["code"], "stale_snapshot");
    let mut different = intent.clone();
    different["command"]["description"] = json!("different");
    let (status, error) = send(&router, request(&f.token, "mutations", Some(&different))).await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(error["code"], "intent_conflict");
    assert_eq!(
        send(
            &router,
            request(
                &f.reader,
                &format!("mutations/{}", m::RequestId::new([122; 16])),
                None
            )
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(*f.session.snapshot().await.unwrap().manifest(), after);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_mutations_run_all_advertised_commands_and_late_bookmark_retries() {
    let f = Fixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(&f, cache.path(), Config::default(), Protection::default());
    let mut new = new_request(&f, 124).await;
    // Mutation bodies are larger than the separate 4 KiB index-task limit.
    new["command"]["description"] = json!("x".repeat(5000));
    let (status, created) = send(&router, request(&f.token, "mutations", Some(&new))).await;
    assert_eq!(status, StatusCode::OK, "{created}");
    let commit = created["receipt"]["result"]["created"][0].clone();
    let mut rename = new_request(&f, 125).await;
    rename["command"] = json!({"type":"describe", "commit":commit, "description":"renamed"});
    let (status, described) = send(&router, request(&f.token, "mutations", Some(&rename))).await;
    assert_eq!(status, StatusCode::OK, "{described}");
    let target = described["receipt"]["result"]["rewrites"][0]["commit"].clone();
    let mut create = new_request(&f, 126).await;
    create["command"] = json!({"type":"bookmark_create", "name":"topic", "target":target});
    let (status, marked) = send(&router, request(&f.token, "mutations", Some(&create))).await;
    assert_eq!(status, StatusCode::OK, "{marked}");
    let mut moving = new_request(&f, 127).await;
    moving["command"] = json!({"type":"bookmark_move", "name":"topic", "expected":[target], "target":roots::ROOT_COMMIT});
    let (status, moved) = send(&router, request(&f.token, "mutations", Some(&moving))).await;
    assert_eq!(status, StatusCode::OK, "{moved}");
    let mut delete = new_request(&f, 128).await;
    delete["command"] =
        json!({"type":"bookmark_delete", "name":"topic", "expected":[roots::ROOT_COMMIT]});
    let (status, deleted) = send(&router, request(&f.token, "mutations", Some(&delete))).await;
    assert_eq!(status, StatusCode::OK, "{deleted}");
    assert_eq!(
        send(&router, request(&f.token, "mutations", Some(&create)))
            .await
            .1,
        marked
    );
    assert_eq!(
        send(&router, request(&f.token, "mutations", Some(&moving)))
            .await
            .1,
        moved
    );
    assert_eq!(
        f.session.snapshot().await.unwrap().manifest().published,
        m::WalPosition(5)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_mutations_enforce_protection_revocation_and_principal_scoped_status() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    let f = Fixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(&f, cache.path(), Config::default(), Protection::default());
    let new = new_request(&f, 129).await;
    let (_, created) = send(&router, request(&f.token, "mutations", Some(&new))).await;
    let commit = created["receipt"]["result"]["created"][0]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let protected = api(
        &f,
        cache.path(),
        Config::default(),
        Protection {
            commits: BTreeSet::from([commit]),
            ..Default::default()
        },
    );
    let mut rewrite = new_request(&f, 130).await;
    rewrite["command"] = json!({"type":"describe", "commit":commit, "description":"forbidden"});
    let before = f.session.snapshot().await.unwrap().manifest().clone();
    for path in ["mutations", "mutations/preview"] {
        assert_eq!(
            send(&protected, request(&f.token, path, Some(&rewrite)))
                .await
                .0,
            StatusCode::FORBIDDEN
        );
    }
    assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
    let writer = BearerToken::generate().unwrap();
    f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([131; 16]),
            expected: before.snapshot(),
            change: TokenChange::Grant {
                credential: writer.clone(),
                principal: m::PrincipalId::new([131; 16]),
                permission: Permission::Write,
                expires_at: None,
            },
        })
        .await
        .unwrap();
    let status_path = format!("mutations/{}", new["request_id"].as_str().unwrap());
    let (status, result) = send(&router, request(&writer, &status_path, None)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result["status"], "not_known");
    let next = new_request(&f, 132).await;
    assert_eq!(
        send(&router, request(&writer, "mutations/preview", Some(&next)))
            .await
            .0,
        StatusCode::OK
    );
    f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([133; 16]),
            expected: f.session.snapshot().await.unwrap().manifest().snapshot(),
            change: TokenChange::Revoke { token: writer.id() },
        })
        .await
        .unwrap();
    let before = f.session.snapshot().await.unwrap().manifest().clone();
    assert_eq!(
        send(&router, request(&writer, "mutations", Some(&next)))
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        send(&router, request(&writer, &status_path, None)).await.0,
        StatusCode::FORBIDDEN
    );
    let mut other = request(&f.token, &status_path, None);
    *other.uri_mut() = format!(
        "/v1/repos/{}/{status_path}",
        m::RepositoryId::new([231; 16])
    )
    .parse()
    .unwrap();
    assert_eq!(send(&router, other).await.0, StatusCode::FORBIDDEN);
    let mut head = request(&f.token, &status_path, None);
    *head.method_mut() = axum::http::Method::HEAD;
    let response = router.oneshot(head).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        to_bytes(response.into_body(), 1024)
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_mutations_small_result_budget_preserves_unknown_identity_and_recoverable_receipt() {
    let f = Fixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(
        &f,
        cache.path(),
        Config {
            max_json_bytes: 128,
            ..Default::default()
        },
        Protection::default(),
    );
    let new = new_request(&f, 134).await;
    let (status, outcome) = send(&router, request(&f.token, "mutations", Some(&new))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        outcome,
        json!({"status":"outcome_unknown","request_id":new["request_id"]})
    );
    let normal = api(&f, cache.path(), Config::default(), Protection::default());
    let path = format!("mutations/{}", new["request_id"].as_str().unwrap());
    let (status, receipt) = send(&normal, request(&f.token, &path, None)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(receipt["status"], "committed");
    assert_eq!(
        send(&normal, request(&f.token, "mutations", Some(&new)))
            .await
            .1,
        receipt
    );
    assert_eq!(
        f.session.snapshot().await.unwrap().manifest().published,
        m::WalPosition(1)
    );
}

use jj_store_object::{
    ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
};
use std::sync::atomic::{AtomicBool, Ordering};
struct LostAck {
    inner: Arc<MemoryStore>,
    armed: AtomicBool,
    entered: tokio::sync::Notify,
}
#[async_trait::async_trait]
impl ObjectStore for LostAck {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.inner.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        let outcome = self.inner.put(key, condition, upload).await?;
        if key.as_str().ends_with("/manifest.json") && self.armed.swap(false, Ordering::SeqCst) {
            self.entered.notify_one();
            std::future::pending::<()>().await;
        }
        Ok(outcome)
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_mutations_deadline_after_manifest_cas_retains_request_identity() {
    let f = Fixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let store = Arc::new(LostAck {
        inner: f.store.clone(),
        armed: AtomicBool::new(true),
        entered: Default::default(),
    });
    let host = RepositoryHost::new(
        store.clone(),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap();
    let router = HttpApi::new(
        host,
        cache.path().to_owned(),
        Config {
            timeout: std::time::Duration::from_secs(1),
            ..Default::default()
        },
    )
    .unwrap()
    .router();
    let new = new_request(&f, 135).await;
    let pending = request(&f.token, "mutations", Some(&new));
    let job = tokio::spawn(async move { send(&router, pending).await });
    tokio::time::timeout(std::time::Duration::from_secs(3), store.entered.notified())
        .await
        .unwrap();
    let (status, unknown) = job.await.unwrap();
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        unknown,
        json!({"status":"outcome_unknown", "request_id":new["request_id"]})
    );
    let restarted = api(&f, cache.path(), Config::default(), Protection::default());
    let path = format!("mutations/{}", new["request_id"].as_str().unwrap());
    let (_, receipt) = send(&restarted, request(&f.token, &path, None)).await;
    assert_eq!(receipt["status"], "committed");
    assert_eq!(
        send(&restarted, request(&f.token, "mutations", Some(&new)))
            .await
            .1,
        receipt
    );
    assert_eq!(
        f.session.snapshot().await.unwrap().manifest().published,
        m::WalPosition(1)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_reconciliation_preserves_get_heads_and_publishes_only_explicit_apply() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let loader = f
                .stores
                .initialize_cache(&settings(), cache.path())
                .await
                .unwrap();
            let (left, _, _, _) = super::reconciliation_acceptance::seed(&f, &loader).await;
            let expected = f.session.snapshot().await.unwrap().manifest().snapshot();
            let router = api(&f, cache.path(), Config::default(), Protection::default());
            let (status, snapshot) = send(&router, request(&f.reader, "snapshot", None)).await;
            assert_eq!(status, StatusCode::OK);
            assert_eq!(snapshot["heads"].as_array().unwrap().len(), 2);
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                expected
            );
            let mut input = new_request(&f, 148).await;
            input["command"] = json!({"type":"reconcile_operations", "base":oid(left.op_id())});
            let (status, preview) = send(
                &router,
                request(&f.token, "mutations/preview", Some(&input)),
            )
            .await;
            assert_eq!(status, StatusCode::OK, "{preview}");
            assert_eq!(preview["status"], "prepared");
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                expected
            );
            let (status, receipt) =
                send(&router, request(&f.token, "mutations", Some(&input))).await;
            assert_eq!(status, StatusCode::OK, "{receipt}");
            assert_eq!(receipt["status"], "committed");
            let path = format!("mutations/{}", input["request_id"].as_str().unwrap());
            assert_eq!(
                send(&router, request(&f.token, &path, None)).await.1,
                receipt
            );
            assert_eq!(
                send(&router, request(&f.token, "mutations", Some(&input)))
                    .await
                    .1,
                receipt
            );
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().heads.len(),
                1
            );
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().published.0,
                expected.position.0 + 1
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_abandon_previews_native_effects_and_reports_hidden_noop_without_publication() {
    tokio::task::LocalSet::new().run_until(async {
        let f = Fixture::new().await;
        let cache = tempfile::tempdir().unwrap();
        let loader = f.stores.initialize_cache(&settings(), cache.path()).await.unwrap();
        let (_, ids) = super::abandon_acceptance::seed(&f, &loader).await;
        let router = api(&f, cache.path(), Config::default(), Protection::default());
        let mut input = new_request(&f, 159).await;
        input["command"] = json!({
            "type": "abandon", "commits": [ids[1]],
            "retain_bookmarks": false, "restore_descendants": true
        });
        let expected = f.session.snapshot().await.unwrap().manifest().snapshot();
        let (status, preview) = send(&router, request(&f.token, "mutations/preview", Some(&input))).await;
        assert_eq!(status, StatusCode::OK, "{preview}");
        assert_eq!(preview["preview"]["result"]["abandoned"], json!([ids[1]]));
        assert_eq!(f.session.snapshot().await.unwrap().manifest().snapshot(), expected);
        let (status, receipt) = send(&router, request(&f.token, "mutations", Some(&input))).await;
        assert_eq!(status, StatusCode::OK, "{receipt}");
        assert_eq!(receipt["status"], "committed");
        let before = f.session.snapshot().await.unwrap().manifest().snapshot();
        let mut noop = input.clone();
        noop["request_id"] = json!(m::RequestId::new([160; 16]));
        noop["expected"] = json!(before);
        let (status, error) = send(&router, request(&f.token, "mutations", Some(&noop))).await;
        assert_eq!(status, StatusCode::CONFLICT);
        assert_eq!(error["code"], "no_changes");
        let path = format!("mutations/{}", input["request_id"].as_str().unwrap());
        assert_eq!(send(&router, request(&f.token, &path, None)).await.1, receipt);
        assert_eq!(send(&router, request(&f.token, "mutations", Some(&input))).await.1, receipt);
        assert_eq!(f.session.snapshot().await.unwrap().manifest().snapshot(), before);
        if let Some(path) = std::env::var_os("JJ_STORE_ABANDON_CONTRACT_CAPTURE") {
            let samples = [
                json!({"path":"/v1/repos/{repo}/mutations/preview","method":"post","status":200,"media":"application/json","data":preview}),
                json!({"path":"/v1/repos/{repo}/mutations","method":"post","status":200,"media":"application/json","data":receipt}),
                json!({"path":"/v1/repos/{repo}/mutations","method":"post","status":409,"media":"application/json","data":error}),
            ];
            std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
        }
    }).await;
}
