//! Actual registered HTTP handlers over disposable native authority.
use super::*;
use axum::{
    body::{Body, to_bytes},
    http::{Request as HttpRequest, StatusCode},
};
use jj_store_http::{Access, Config, HttpApi};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    collab_model::*,
};
use serde_json::{Value, json};
use tower::ServiceExt as _;
struct NoOperators;
#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        panic!("HTTP collaboration must not access operator credentials")
    }
}
fn api(f: &Fixture, cache: &std::path::Path, config: Config) -> axum::Router {
    let host = RepositoryHost::new(
        f.store.clone(),
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
fn local(permission: Permission) -> Config {
    Config {
        access: Access::LocalTrusted {
            principal: ACTOR,
            permission,
        },
        ..Default::default()
    }
}
fn request(path: &str, body: Option<&Value>) -> HttpRequest<Body> {
    let r = HttpRequest::builder()
        .uri(format!("/v1/repos/{REPO}/{path}"))
        .header("host", "127.0.0.1:7443")
        .header("x-jj-store-protocol", "1");
    match body {
        Some(body) => r
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(body).unwrap()))
            .unwrap(),
        None => r.body(Body::empty()).unwrap(),
    }
}
fn binary(path: &str, bytes: Vec<u8>) -> HttpRequest<Body> {
    HttpRequest::builder()
        .uri(format!("/v1/repos/{REPO}/{path}"))
        .method("POST")
        .header("host", "127.0.0.1:7443")
        .header("x-jj-store-protocol", "1")
        .header("content-type", "application/octet-stream")
        .body(Body::from(bytes))
        .unwrap()
}
async fn send(router: &axum::Router, req: HttpRequest<Body>) -> (StatusCode, Value) {
    let response = router.clone().oneshot(req).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 4 * 1024 * 1024)
        .await
        .unwrap();
    (status, serde_json::from_slice(&bytes).unwrap())
}
async fn get(router: &axum::Router, path: &str) -> Value {
    let (s, v) = send(router, request(path, None)).await;
    assert_eq!(s, StatusCode::OK, "{v}");
    v
}
async fn post(router: &axum::Router, path: &str, body: &Value) -> Value {
    let (s, v) = send(router, request(path, Some(body))).await;
    assert_eq!(s, StatusCode::OK, "{v}");
    v
}
fn command(n: u8, command: Value) -> Value {
    json!({"version":1,"request_id":m::RequestId::new([n;16]),"command":command})
}
fn create(n: u8) -> Value {
    command(
        n,
        json!(ThreadCommandV1::Create {
            thread_id: ThreadId::new([n; 16]),
            title: format!("Thread {n}"),
            baseline: None
        }),
    )
}
async fn home(f: &Fixture) {
    use jj_store_service::work::*;
    WorkCommandService
        .apply(
            f.session.clone(),
            &Request {
                version: 1,
                request_id: m::RequestId::new([199; 16]),
                command: WorkCommandV1::CreateProject {
                    id: ProjectId::new([199; 16]),
                    title: "Collaboration".into(),
                    body: String::new(),
                },
            },
        )
        .await
        .unwrap();
}
fn png() -> Vec<u8> {
    hex::decode("89504e470d0a1a0a0000000d49484452000000010000000108060000001f15c4890000000d49444154789c63f8cfc0f01f00050001ff89993d1d0000000049454e44ae426082").unwrap()
}
fn record(samples: &mut Vec<Value>, route: &str, method: &str, value: &Value) {
    samples.push(json!({"path":format!("/v1/repos/{{repo}}/{route}"),"method":method,"status":200,"media":"application/json","data":value}));
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn http_threads_reactions_keep_exact_targets_assets_signed_roots_and_receipts() {
    let f = Fixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(&f, cache.path(), local(Permission::Admin));
    let before = f.session.snapshot().await.unwrap();
    let empty = get(&router, "threads").await;
    assert!(empty["items"]["threads"].as_array().unwrap().is_empty());
    assert_eq!(
        f.session.snapshot().await.unwrap().digest(),
        before.digest()
    );
    let (s, denied_code) = send(&router, request("threads/commands", Some(&create(201)))).await;
    assert_eq!(s, StatusCode::CONFLICT);
    assert_eq!(denied_code["code"], "collaboration_replication_unsupported");
    assert_eq!(
        f.session.snapshot().await.unwrap().digest(),
        before.digest()
    );
    home(&f).await;
    let frozen = f.session.snapshot().await.unwrap();
    let first_req = create(201);
    let accepted = post(&router, "threads/commands", &first_req).await;
    let second = post(&router, "threads/commands", &create(202)).await;
    let page = get(&router, "threads?limit=1").await;
    let cursor = page["next_cursor"].as_str().unwrap();
    post(&router, "threads/commands", &create(203)).await;
    let resumed = get(&router, &format!("threads?limit=1&cursor={cursor}")).await;
    assert_eq!(resumed["snapshot"], page["snapshot"]);
    assert!(resumed["next_cursor"].is_null());
    assert_eq!(
        send(
            &router,
            request(&format!("threads?limit=2&cursor={cursor}"), None)
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        send(
            &router,
            request(&format!("reaction-emojis?limit=1&cursor={cursor}"), None)
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let thread_id = ThreadId::new([201; 16]);
    let expected_thread: CollabObjectRef =
        serde_json::from_value(accepted["receipt"]["result"]["result"]["version"].clone()).unwrap();
    let message_id = HumanMessageId::new([204; 16]);
    let message = post(
        &router,
        "threads/commands",
        &command(
            204,
            json!(ThreadCommandV1::Message {
                thread_id,
                expected_thread,
                message_id,
                expected_revision: None,
                body: Some("First human revision".into()),
            }),
        ),
    )
    .await;
    let result = &message["receipt"]["result"]["result"];
    let revision: CollabObjectRef = serde_json::from_value(result["message"].clone()).unwrap();
    let target = ReactionTargetV1::HumanMessage {
        thread_id,
        message_id,
        revision,
    };
    let reaction_req = command(
        205,
        json!(ReactionCommandV1::Set {
            target: target.clone(),
            observed: BTreeSet::new(),
            value: ReactionValueV1::Emoji {
                sequence: "❤️".into()
            }
        }),
    );
    let reacted = post(&router, "reactions/commands", &reaction_req).await;
    let query = json!({"target":target, "limit":1, "cursor":null});
    let reaction_page = post(&router, "reactions/query", &query).await;
    assert_eq!(reaction_page["items"]["reactions"][0]["count"], 1);
    let edited = post(
        &router,
        "threads/commands",
        &command(
            206,
            json!(ThreadCommandV1::Message {
                thread_id,
                expected_thread: serde_json::from_value(result["version"].clone()).unwrap(),
                message_id,
                expected_revision: Some(revision),
                body: Some("Second revision".into()),
            }),
        ),
    )
    .await;
    let earlier = post(&router, "reactions/query", &query).await;
    assert_eq!(earlier["items"]["reactions"][0]["count"], 1);
    let new_target = json!({"type":"human_message","thread_id":thread_id,"message_id":message_id,"revision":edited["receipt"]["result"]["result"]["message"]});
    let current = post(
        &router,
        "reactions/query",
        &json!({"target":new_target,"limit":1}),
    )
    .await;
    assert!(current["items"]["reactions"].as_array().unwrap().is_empty());
    let exact = get(
        &router,
        &format!(
            "threads/{thread_id}/messages/{message_id}/revisions/{}",
            revision.id
        ),
    )
    .await;
    assert_eq!(exact["message"]["body"], "First human revision");
    let detail = get(&router, &format!("threads/{thread_id}")).await;
    let timeline = get(&router, &format!("threads/{thread_id}/timeline?limit=1")).await;
    let next = timeline["next_cursor"].as_str().unwrap();
    let older_events = get(
        &router,
        &format!("threads/{thread_id}/timeline?limit=1&cursor={next}"),
    )
    .await;
    assert_eq!(older_events["snapshot"], timeline["snapshot"]);
    let emoji_id = CustomEmojiId::new([207; 16]);
    let upload_path = format!(
        "reaction-emojis/{emoji_id}?version=1&request_id={}&shortcode=heart&label=Heart",
        m::RequestId::new([207; 16])
    );
    let (s, defined) = send(&router, binary(&upload_path, png())).await;
    assert_eq!(s, StatusCode::OK, "{defined}");
    let library = get(&router, "reaction-emojis").await;
    let definition = &library["items"]["definitions"][0];
    let custom: CollabObjectRef =
        serde_json::from_value(defined["receipt"]["result"]["version"].clone()).unwrap();
    let custom_reacted = post(
        &router,
        "reactions/commands",
        &command(
            208,
            json!(ReactionCommandV1::Set {
                target: ReactionTargetV1::Thread { thread_id },
                observed: BTreeSet::new(),
                value: ReactionValueV1::Custom {
                    emoji_id,
                    version: custom
                }
            }),
        ),
    )
    .await;
    let retired = post(&router, "reaction-emojis/commands", &json!({"version":1,"request_id":m::RequestId::new([209;16]),"emoji_id":emoji_id,"expected":custom.id})).await;
    let retained = post(
        &router,
        "reactions/query",
        &json!({"target":{"type":"thread","thread_id":thread_id}}),
    )
    .await;
    assert_eq!(retained["items"]["reactions"][0]["count"], 1);
    for kind in ["asset", "preview"] {
        let asset_id = definition[kind]["id"].as_str().unwrap();
        let mut ranged = request(&format!("reaction-assets/{asset_id}"), None);
        ranged
            .headers_mut()
            .insert("range", "bytes=0-1".parse().unwrap());
        assert_eq!(send(&router, ranged).await.0, StatusCode::BAD_REQUEST);
        let mut head = request(&format!("reaction-assets/{asset_id}"), None);
        *head.method_mut() = axum::http::Method::HEAD;
        let response = router.clone().oneshot(head).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            to_bytes(response.into_body(), MAX_IMAGE_BYTES)
                .await
                .unwrap()
                .is_empty()
        );
        let response = router
            .clone()
            .oneshot(request(&format!("reaction-assets/{asset_id}"), None))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["content-type"], "image/png");
        assert_eq!(response.headers()["x-content-type-options"], "nosniff");
        assert_eq!(response.headers()["cache-control"], "no-store");
        assert_eq!(response.headers()["x-jj-store-reaction-asset"], asset_id);
        let bytes = to_bytes(response.into_body(), MAX_IMAGE_BYTES)
            .await
            .unwrap();
        assert!(!bytes.is_empty());
        if kind == "asset" {
            assert_eq!(bytes.as_ref(), png());
        }
    }
    let fresh = api(&f, cache.path(), local(Permission::Admin));
    let durable = get(
        &fresh,
        &format!("threads/commands/{}", m::RequestId::new([201; 16])),
    )
    .await;
    assert_eq!(durable, accepted);
    assert_eq!(post(&fresh, "threads/commands", &first_req).await, accepted);
    assert_eq!(
        send(
            &fresh,
            request(&format!("threads?limit=1&cursor={cursor}"), None)
        )
        .await
        .1["code"],
        "cursor_reset"
    );
    let reaction_status = get(
        &fresh,
        &format!("reactions/commands/{}", m::RequestId::new([205; 16])),
    )
    .await;
    assert_eq!(reaction_status, reacted);
    let emoji_status = get(
        &fresh,
        &format!("reaction-emojis/commands/{}", m::RequestId::new([207; 16])),
    )
    .await;
    assert_eq!(emoji_status, defined);
    let after = f.session.snapshot().await.unwrap();
    assert_eq!(after.manifest().heads, frozen.manifest().heads);
    assert_eq!(
        after.manifest().replication_root,
        frozen.manifest().replication_root
    );
    let mut samples = vec![];
    for (route, method, data) in [
        ("threads", "get", empty),
        ("threads", "get", page),
        ("threads", "get", resumed),
        ("threads/commands", "post", accepted),
        ("threads/commands", "post", second),
        ("threads/commands", "post", message),
        ("threads/commands/{request_id}", "get", durable),
        ("threads/{thread_id}", "get", detail),
        ("threads/{thread_id}/timeline", "get", timeline),
        ("threads/{thread_id}/timeline", "get", older_events),
        (
            "threads/{thread_id}/messages/{message_id}/revisions/{message_revision}",
            "get",
            exact,
        ),
        ("reactions/commands", "post", reacted),
        ("reactions/commands/{request_id}", "get", reaction_status),
        ("reactions/commands", "post", custom_reacted),
        ("reactions/query", "post", reaction_page),
        ("reactions/query", "post", earlier),
        ("reactions/query", "post", current),
        ("reaction-emojis", "get", library),
        ("reaction-emojis/{emoji_id}", "post", defined),
        ("reaction-emojis/commands", "post", retired),
        ("reaction-emojis/commands/{request_id}", "get", emoji_status),
    ] {
        record(&mut samples, route, method, &data);
    }
    samples.push(json!({"path":"/v1/repos/{repo}/reaction-assets/{asset}","method":"get","status":200,"media":"image/png","data":hex::encode(png())}));
    if let Some(path) = std::env::var_os("JJ_STORE_THREADS_REACTIONS_CONTRACT_CAPTURE") {
        std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn http_collab_refuses_injection_permissions_media_ranges_limits_and_preserves_unknown_identity()
 {
    let f = Fixture::new().await;
    home(&f).await;
    let cache = tempfile::tempdir().unwrap();
    let router = api(&f, cache.path(), local(Permission::Admin));
    let reader = api(&f, cache.path(), local(Permission::Read));
    let before = f.session.snapshot().await.unwrap().digest();
    let mut injected = create(211);
    injected["actor"] = json!(ACTOR);
    for req in [
        request("threads/commands", Some(&injected)),
        request("threads?extra=1", None),
        request("threads?limit=0", None),
        request(
            "reactions/query",
            Some(&json!({"target":{"type":"future"}})),
        ),
        request("reactions/commands", Some(&create(211))),
    ] {
        assert_eq!(send(&router, req).await.0, StatusCode::BAD_REQUEST);
    }
    assert_eq!(
        send(&reader, request("threads/commands", Some(&create(211))))
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        send(
            &reader,
            request(
                &format!("threads/commands/{}", m::RequestId::new([211; 16])),
                None
            )
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    let path = format!(
        "reaction-emojis/{}?version=1&request_id={}&shortcode=x&label=X",
        CustomEmojiId::new([212; 16]),
        m::RequestId::new([212; 16])
    );
    assert_eq!(
        send(&reader, binary(&path, png())).await.0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        send(&router, binary(&path, vec![0; MAX_IMAGE_BYTES + 1]))
            .await
            .0,
        StatusCode::PAYLOAD_TOO_LARGE
    );
    assert_eq!(
        send(&router, binary(&path, b"<svg/>".to_vec())).await.0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        send(&router, request(&path, Some(&json!({"bytes":png()}))))
            .await
            .0,
        StatusCode::UNSUPPORTED_MEDIA_TYPE
    );
    assert_eq!(f.session.snapshot().await.unwrap().digest(), before);
    let small = api(
        &f,
        cache.path(),
        Config {
            max_json_bytes: 128,
            ..local(Permission::Admin)
        },
    );
    let req = create(213);
    let unknown = post(&small, "threads/commands", &req).await;
    assert_eq!(
        unknown,
        json!({"status":"outcome_unknown","request_id":req["request_id"]})
    );
    let committed = get(
        &router,
        &format!("threads/commands/{}", m::RequestId::new([213; 16])),
    )
    .await;
    assert_eq!(committed["status"], "committed");
    assert_eq!(post(&router, "threads/commands", &req).await, committed);
    let authenticated = api(&f, cache.path(), Config::default());
    let mut req = request("threads", None);
    req.headers_mut().insert(
        "authorization",
        format!("Bearer {}", f.reader.expose().as_str())
            .parse()
            .unwrap(),
    );
    assert_eq!(send(&authenticated, req).await.0, StatusCode::OK);
    assert_eq!(
        send(&authenticated, request("threads", None)).await.0,
        StatusCode::UNAUTHORIZED
    );
}
