use jj_store_model::{PrincipalId, RequestId};
use jj_store_object::memory::MemoryStore;
use jj_store_server::{
    directory::Manager,
    installation::{Identity, initialize_registry},
    registry::Registry,
};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    auth::{self, BearerToken, Permission, Scope, SystemClock, TokenPolicy},
    directory::{Caller, Directory},
    native::SessionLimits,
};
use serde_json::{Value, json};
use std::sync::Arc;
struct Operators(TokenPolicy);
#[async_trait::async_trait]
impl OperatorPolicies for Operators {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(self.0.clone())
    }
}
async fn start(
    local: bool,
) -> (
    tempfile::TempDir,
    String,
    Arc<Manager>,
    BearerToken,
    PrincipalId,
    tokio::task::JoinHandle<()>,
) {
    let root = tempfile::tempdir().unwrap();
    let installed = initialize_registry(&root.path().join("state")).unwrap();
    let identity = Identity::load(&installed.installation_file).unwrap();
    let actor = identity.principal(b"loopback-http");
    let registry = Arc::new(Registry::new(installed.catalog, identity.clone()).unwrap());
    let store = Arc::new(MemoryStore::default());
    let directory = Arc::new(
        Manager::new(store.clone(), identity.installation, local.then_some(actor)).unwrap(),
    );
    let token = BearerToken::generate().unwrap();
    let mut policy = TokenPolicy::new(Scope::Operator);
    policy
        .grant(
            &token,
            PrincipalId::new([9; 16]),
            Permission::ManageIdentities,
            None,
        )
        .unwrap();
    let operators = Arc::new(Operators(policy));
    let host = RepositoryHost::new(
        store,
        operators.clone(),
        Arc::new(SystemClock),
        Default::default(),
        SessionLimits::default(),
        16 * 1024 * 1024,
    )
    .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let api = jj_store_http::HttpApi::new(
        host,
        root.path().join("reads"),
        jj_store_http::Config {
            origin: base.clone(),
            access: if local {
                jj_store_http::Access::LocalTrusted {
                    principal: actor,
                    permission: Permission::Admin,
                }
            } else {
                jj_store_http::Access::Authenticated
            },
            directory: Some(directory.clone()),
            directory_operators: Some(operators),
            registry: Some(registry),
            ..Default::default()
        },
    )
    .unwrap();
    let task = tokio::spawn(async move { axum::serve(listener, api.router()).await.unwrap() });
    (root, base, directory, token, actor, task)
}
async fn request(
    base: &str,
    token: Option<&BearerToken>,
    method: &str,
    path: &str,
    body: Option<Value>,
) -> (u16, Value) {
    let client = reqwest::Client::new();
    let mut r = client
        .request(method.parse().unwrap(), format!("{base}/v1/{path}"))
        .header("x-jj-store-protocol", "1");
    if let Some(token) = token {
        r = r.bearer_auth(token.expose().as_str());
    }
    if let Some(body) = body {
        r = r
            .header("content-type", "application/json")
            .body(serde_json::to_vec(&body).unwrap());
    }
    let r = r.send().await.unwrap();
    let status = r.status().as_u16();
    assert_eq!(r.headers()["cache-control"], "no-store");
    (
        status,
        serde_json::from_slice(&r.bytes().await.unwrap()).unwrap(),
    )
}
#[tokio::test]
async fn authenticated_directory_is_private_independent_of_repo_grants_and_revocable() {
    let (_root, base, directory, operator, _, task) = start(false).await;
    assert_eq!(
        request(&base, None, "GET", "directory/profile", None)
            .await
            .0,
        401
    );
    assert_eq!(
        request(&base, Some(&operator), "GET", "directory/profile", None)
            .await
            .0,
        403
    );
    let user = BearerToken::generate().unwrap();
    let other = BearerToken::generate().unwrap();
    let principal = PrincipalId::new([2; 16]);
    for (id, token, actor) in [
        (1, &user, principal),
        (2, &other, PrincipalId::new([3; 16])),
    ] {
        assert_eq!(request(&base,Some(&operator),"POST","directory/identities",Some(json!({"request_id":RequestId::new([id;16]),"principal":actor,"credential":token.expose().as_str(),"expires_at":null}))).await.0,200);
    }
    assert_eq!(
        request(&base, Some(&user), "GET", "directory/identities", None)
            .await
            .0,
        403
    );
    let (status, repos) = request(&base, Some(&user), "GET", "repositories", None).await;
    assert_eq!(status, 200);
    assert_eq!(repos["repositories"], json!([]));
    let (status,saved)=request(&base,Some(&user),"POST","directory/commands",Some(json!({"request_id":RequestId::new([3;16]),"command":{"type":"save_profile","expected":null,"details":{"display_name":"User","description":"Description","email":"private@example.com","avatar":null}}}))).await;
    assert_eq!(status, 200);
    assert_eq!(
        saved["result"]["profile"]["details"]["email"],
        "private@example.com"
    );
    let (status, public) = request(
        &base,
        Some(&other),
        "GET",
        &format!("directory/profiles?ids={principal}"),
        None,
    )
    .await;
    assert_eq!(status, 200);
    assert!(!public.to_string().contains("email"));
    assert!(!public.to_string().contains("private@example.com"));
    assert!(
        public[0]["avatar"]
            .as_str()
            .unwrap()
            .contains("gravatar.com")
    );
    assert_eq!(
        request(
            &base,
            Some(&other),
            "GET",
            &format!("directory/commands/{}", RequestId::new([3; 16])),
            None
        )
        .await,
        (200, Value::Null)
    );
    assert_eq!(
        request(
            &base,
            Some(&operator),
            "POST",
            "directory/identity-revoke",
            Some(json!({"request_id":RequestId::new([4;16]),"token":user.id()}))
        )
        .await
        .0,
        200
    );
    assert_eq!(
        request(&base, Some(&user), "GET", "directory/profile", None)
            .await
            .0,
        403
    );
    directory.audit().await.unwrap();
    task.abort();
    let _ = task.await;
}
#[tokio::test]
async fn local_admin_uses_the_same_revision_conditional_profile_api() {
    let (_root, base, directory, _, actor, task) = start(true).await;
    let (status, p) = request(&base, None, "GET", "directory/profile", None).await;
    assert_eq!(status, 200);
    assert_eq!(p["principal"], actor.to_string());
    let body = json!({"request_id":RequestId::new([1;16]),"command":{"type":"save_profile","expected":null,"details":{"display_name":"Local admin","description":"Local profile","email":null,"avatar":"https://example.com/avatar.png"}}});
    assert_eq!(
        request(
            &base,
            None,
            "POST",
            "directory/commands",
            Some(body.clone())
        )
        .await
        .0,
        200
    );
    assert_eq!(
        request(
            &base,
            None,
            "POST",
            "directory/commands",
            Some(body.clone())
        )
        .await
        .0,
        200
    );
    let mut stale = body;
    stale["request_id"] = json!(RequestId::new([2; 16]));
    assert_eq!(
        request(&base, None, "POST", "directory/commands", Some(stale))
            .await
            .0,
        409
    );
    assert_eq!(
        directory
            .profile(&Caller::Local(actor))
            .await
            .unwrap()
            .details
            .display_name,
        "Local admin"
    );
    task.abort();
    let _ = task.await;
}

#[tokio::test]
async fn binary_avatar_upload_normalizes_large_sources_replays_and_admits_identity_first() {
    let (_root, base, directory, _, actor, task) = start(true).await;
    let mut image_bytes = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(image::RgbImage::from_pixel(
        2500,
        1800,
        image::Rgb([60, 90, 150]),
    ))
    .write_to(&mut image_bytes, image::ImageFormat::Jpeg)
    .unwrap();
    let mut bytes = image_bytes.into_inner();
    bytes.resize(2 * 1024 * 1024 + 1, 0);
    let client = reqwest::Client::new();
    let path = format!(
        "{base}/v1/directory/image-uploads/{}",
        RequestId::new([72; 16])
    );
    let mut original = Value::Null;
    for _ in 0..2 {
        let response = client
            .post(&path)
            .header("x-jj-store-protocol", "1")
            .header("content-type", "application/octet-stream")
            .body(bytes.clone())
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        let receipt: Value = serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();
        if original.is_null() {
            original = receipt;
        } else {
            assert_eq!(receipt, original);
        }
    }
    let asset = original["result"]["asset"].as_str().unwrap();
    let retained = directory.media(&Caller::Local(actor), asset).await.unwrap();
    let retained = image::load_from_memory(&retained.bytes).unwrap();
    assert_eq!((retained.width(), retained.height()), (512, 369));
    assert_eq!(
        client
            .post(&path)
            .header("x-jj-store-protocol", "1")
            .header("content-type", "application/octet-stream")
            .body("changed")
            .send()
            .await
            .unwrap()
            .status(),
        400
    );
    directory.audit().await.unwrap();
    task.abort();
    let _ = task.await;
    let (_root, base, _, _, _, task) = start(false).await;
    let response = client
        .post(format!(
            "{base}/v1/directory/image-uploads/{}",
            RequestId::new([73; 16])
        ))
        .header("x-jj-store-protocol", "1")
        .header("content-type", "application/octet-stream")
        .body("invalid image")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 401);
    task.abort();
    let _ = task.await;
}
