use super::*;
use jj_store_service::search_management as search;
#[derive(Default)]
struct Manager {
    calls: std::sync::atomic::AtomicUsize,
}
impl Manager {
    fn report(&self) -> search::LocalSearchSettings {
        search::LocalSearchSettings {
            version: 1,
            revision: "ab".repeat(32),
            desired: Default::default(),
            effective: Default::default(),
            operation: search::SearchOperation::Idle,
            error: None,
        }
    }
}
#[async_trait::async_trait]
impl search::SearchManager for Manager {
    async fn status(
        &self,
        _: m::RepositoryId,
    ) -> std::result::Result<search::LocalSearchSettings, search::Error> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(self.report())
    }
    async fn save(
        &self,
        repo: m::RepositoryId,
        body: search::SaveSearchSettings,
    ) -> std::result::Result<search::LocalSearchSettings, search::Error> {
        if body.version != 1 {
            return Err(search::Error::Invalid);
        }
        self.status(repo).await
    }
    async fn control(
        &self,
        repo: m::RepositoryId,
        body: search::SearchCommand,
    ) -> std::result::Result<search::LocalSearchSettings, search::Error> {
        if body.version != 1 {
            return Err(search::Error::Invalid);
        }
        self.status(repo).await
    }
}
fn request(suffix: &str, body: Option<Value>) -> Request<Body> {
    let mut builder = Request::builder()
        .uri(format!("/v1/repos/{REPO}/local/search{suffix}"))
        .header("host", "127.0.0.1:7443")
        .header("x-jj-store-protocol", "1");
    let body = if let Some(body) = body {
        builder = builder
            .method("POST")
            .header("content-type", "application/json");
        Body::from(body.to_string())
    } else {
        Body::empty()
    };
    builder.body(body).unwrap()
}
pub(super) async fn capture(f: &ContentFixture, cache: &std::path::Path, samples: &mut Vec<Value>) {
    let manager = Arc::new(Manager::default());
    let control =
        serde_json::json!({"version":1,"expected_revision":"ab".repeat(32),"action":"apply"});
    let save = serde_json::json!({"version":1,"expected_revision":"ab".repeat(32),"configuration":search::SearchConfiguration::default()});
    let local = api(
        f,
        cache,
        Config {
            access: Access::LocalTrusted {
                principal: ACTOR,
                permission: Permission::Admin,
            },
            local_search: Some(manager.clone()),
            ..Default::default()
        },
    );
    for (suffix, body) in [
        ("", None),
        ("/save", Some(save.clone())),
        ("/control", Some(control.clone())),
    ] {
        let method = if body.is_some() { "post" } else { "get" };
        let response = local.clone().oneshot(request(suffix, body)).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let data: Value =
            serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
                .unwrap();
        samples.push(serde_json::json!({"path":format!("/v1/repos/{{repo}}/local/search{suffix}"),"method":method,"status":200,"media":"application/json","data":data}));
    }
    for body in [
        serde_json::json!({"version":2,"expected_revision":"ab","action":"apply"}),
        serde_json::json!({"version":1,"expected_revision":"ab","action":"delete"}),
        serde_json::json!({"version":1,"expected_revision":"ab","action":"apply","path":"/tmp/evil"}),
    ] {
        assert_eq!(
            local
                .clone()
                .oneshot(request("/control", Some(body)))
                .await
                .unwrap()
                .status(),
            StatusCode::BAD_REQUEST
        );
    }
    let calls = manager.calls.load(Ordering::SeqCst);
    for access in [
        Access::Authenticated,
        Access::LocalTrusted {
            principal: ACTOR,
            permission: Permission::Read,
        },
    ] {
        let bearer = matches!(access, Access::Authenticated);
        let router = api(
            f,
            cache,
            Config {
                access,
                local_search: Some(manager.clone()),
                ..Default::default()
            },
        );
        for (suffix, body) in [
            ("", None),
            ("/save", Some(save.clone())),
            ("/control", Some(control.clone())),
        ] {
            let mut req = request(suffix, body);
            if bearer {
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
    let mut req = request("/save", Some(save));
    req.headers_mut()
        .insert("origin", "http://evil.example".parse().unwrap());
    assert_eq!(
        local.oneshot(req).await.unwrap().status(),
        StatusCode::FORBIDDEN
    );
    assert_eq!(calls, manager.calls.load(Ordering::SeqCst));
}
