use super::*;
use jj_store_service::local_management::{
    self as management, VerifiedWorkspace, WorkspaceProfile, WorkspaceVerifier,
};
use std::sync::atomic::{AtomicUsize, Ordering};

struct Verifier(AtomicUsize);
impl WorkspaceVerifier for Verifier {
    fn inspect(&self, path: &str) -> std::result::Result<VerifiedWorkspace, management::Error> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(VerifiedWorkspace {
            path: path.into(),
            metadata: "/fixture/.jj/repo".into(),
            repository: REPO,
            profile: WorkspaceProfile::Native,
        })
    }
}

fn link_request(path: &str, revision: u64) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(format!("/v1/repos/{REPO}/local/workspaces/link"))
        .header("host", "127.0.0.1:7443")
        .header("x-jj-store-protocol", "1")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({"version":1,"expected_revision":revision,"path":path}).to_string(),
        ))
        .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn local_workspace_management_requires_local_admin_and_captures_contracts() {
    let f = ContentFixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let verifier = Arc::new(Verifier(AtomicUsize::new(0)));
    let manager = Arc::new(
        jj_store_server::workspaces::Workspaces::new(
            state.path().into(),
            jj_store_server::installation::Identity {
                version: 1,
                installation: "ab".repeat(16),
            },
            verifier.clone(),
        )
        .unwrap(),
    );
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let local = api(
        &f,
        cache.path(),
        Config {
            access: Access::LocalTrusted {
                principal: ACTOR,
                permission: Permission::Admin,
            },
            local_workspaces: Some(manager.clone()),
            ..Config::default()
        },
    );
    let mut samples = Vec::new();
    for (method, request, expected) in [
        (
            "get",
            protected(&format!("/v1/repos/{REPO}/local/workspaces")),
            StatusCode::OK,
        ),
        ("post", link_request("/workspace", 0), StatusCode::OK),
        ("post", link_request("/workspace", 0), StatusCode::OK),
        ("post", link_request("/second", 0), StatusCode::CONFLICT),
    ] {
        let response = local.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), expected);
        let data: Value =
            serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
                .unwrap();
        samples.push(serde_json::json!({"path":if method == "get" {"/v1/repos/{repo}/local/workspaces"} else {"/v1/repos/{repo}/local/workspaces/link"},"method":method,"status":expected.as_u16(),"media":"application/json","data":data}));
    }
    assert_eq!(samples[0]["data"]["links"], serde_json::json!([]));
    assert_eq!(samples[1]["data"]["links"][0]["availability"], "ready");
    assert_eq!(samples[1]["data"], samples[2]["data"]);
    let calls = verifier.0.load(Ordering::SeqCst);
    for access in [
        Access::Authenticated,
        Access::LocalTrusted {
            principal: ACTOR,
            permission: Permission::Read,
        },
    ] {
        let authenticated = matches!(access, Access::Authenticated);
        let router = api(
            &f,
            cache.path(),
            Config {
                access,
                local_workspaces: Some(manager.clone()),
                ..Config::default()
            },
        );
        for mut req in [
            link_request("/forbidden", 1),
            protected(&format!("/v1/repos/{REPO}/local/workspaces")),
        ] {
            if authenticated {
                req.headers_mut().insert(
                    "authorization",
                    format!("Bearer {}", f.f.token.expose().as_str())
                        .parse()
                        .unwrap(),
                );
            }
            assert_eq!(
                router.clone().oneshot(req).await.unwrap().status(),
                StatusCode::FORBIDDEN
            );
        }
    }
    let mut foreign = link_request("/forbidden", 1);
    foreign
        .headers_mut()
        .insert("origin", "http://evil.example".parse().unwrap());
    assert_eq!(
        local.clone().oneshot(foreign).await.unwrap().status(),
        StatusCode::FORBIDDEN
    );
    let mut invalid = link_request("/forbidden", 1);
    *invalid.body_mut() = Body::from(
        r#"{"version":1,"expected_revision":1,"path":"/forbidden","executable":"/bin/sh"}"#,
    );
    assert_eq!(
        local.oneshot(invalid).await.unwrap().status(),
        StatusCode::BAD_REQUEST
    );
    assert_eq!(verifier.0.load(Ordering::SeqCst), calls);
    assert_eq!(f.f.session.snapshot().await.unwrap().manifest(), &before);
    if let Ok(path) = std::env::var("JJ_STORE_LOCAL_MANAGEMENT_CONTRACT_CAPTURE") {
        std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
}
