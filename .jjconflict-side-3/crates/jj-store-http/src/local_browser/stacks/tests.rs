use super::*;
use std::{
    path::Path as FsPath,
    sync::atomic::{AtomicU64, Ordering},
};
use tower::ServiceExt as _;
struct Manager {
    folder: management::LocalFolder,
    generation: AtomicU64,
}
#[async_trait::async_trait]
impl management::LocalRepositories for Manager {
    fn inspect(&self, _: &str) -> management::Result<management::LocalFolder> {
        Err(management::Error::Unsupported)
    }
    fn list(&self) -> management::Result<management::LocalBindings> {
        Err(management::Error::Unsupported)
    }
    fn folder(&self, repo: m::RepositoryId) -> management::Result<(management::LocalFolder, u64)> {
        if Some(repo) != self.folder.repository {
            return Err(management::Error::Unavailable);
        }
        Ok((self.folder.clone(), self.generation.load(Ordering::Acquire)))
    }
    fn open(
        &self,
        _: m::RepositoryId,
    ) -> management::Result<jj_store_service::local_repository::LocalRepository> {
        Err(management::Error::Unsupported)
    }
    async fn register(
        &self,
        _: management::RegisterFolder,
    ) -> management::Result<management::RegistrationOutcome> {
        Err(management::Error::Unsupported)
    }
    async fn status(&self, _: m::RequestId) -> management::Result<management::RegistrationOutcome> {
        Err(management::Error::Unsupported)
    }
    fn remove(
        &self,
        _: m::RepositoryId,
        _: management::RemoveBinding,
    ) -> management::Result<management::LocalBindings> {
        Err(management::Error::Unsupported)
    }
    fn reconnect(
        &self,
        _: m::RepositoryId,
        _: management::ReconnectFolder,
    ) -> management::Result<management::LocalBindings> {
        Err(management::Error::Unsupported)
    }
}
fn manager(root: &FsPath) -> Arc<Manager> {
    Arc::new(Manager {
        folder: management::LocalFolder {
            path: root.to_str().unwrap().into(),
            metadata: root.join(".jj/repo").to_str().unwrap().into(),
            repository: Some(m::RepositoryId::new([1; 16])),
        },
        generation: AtomicU64::new(7),
    })
}
fn api(root: &FsPath, manager: Arc<Manager>, access: Access) -> HttpApi {
    HttpApi::with_search(
        None,
        root.join("cache"),
        Config {
            access,
            local_repositories: Some(manager),
            ..Default::default()
        },
        BTreeMap::new(),
    )
    .unwrap()
}
fn trusted() -> Access {
    Access::LocalTrusted {
        principal: m::PrincipalId::new([1; 16]),
        permission: Permission::Admin,
    }
}
async fn request_at(
    api: &HttpApi,
    repo: m::RepositoryId,
    value: serde_json::Value,
    origin: &str,
    suffix: &str,
) -> (StatusCode, serde_json::Value) {
    let request = axum::http::Request::builder()
        .method("POST")
        .uri(format!("/v1/local/repos/{repo}/stacks{suffix}"))
        .header("host", "127.0.0.1:7443")
        .header("origin", origin)
        .header("content-type", "application/json")
        .header("x-jj-store-protocol", "1")
        .body(axum::body::Body::from(serde_json::to_vec(&value).unwrap()))
        .unwrap();
    let response = api.router().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 4 * 1024 * 1024)
        .await
        .unwrap();
    (status, serde_json::from_slice(&bytes).unwrap())
}
async fn request(
    api: &HttpApi,
    repo: m::RepositoryId,
    value: serde_json::Value,
    origin: &str,
) -> (StatusCode, serde_json::Value) {
    request_at(api, repo, value, origin, "").await
}
#[tokio::test]
async fn preview_retains_registration_and_same_origin_admission() {
    let root = tempfile::tempdir().unwrap();
    let manager = manager(root.path());
    let api = api(root.path(), manager, trusted());
    let repo = m::RepositoryId::new([1; 16]);
    let body = serde_json::json!({"version":1,"binding_generation":6,"operation":null,"revision":null,"command":{"kind":"commit","branch":"topic","message":"title","selections":[]}});
    assert_eq!(
        request_at(&api, repo, body, "https://foreign.example", "/preview")
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert!(!root.path().join(".jj/jj-store").exists());
}
#[tokio::test]
async fn route_admits_only_registered_local_generations_and_typed_commands() {
    let root = tempfile::tempdir().unwrap();
    let manager = manager(root.path());
    let api = api(root.path(), manager, trusted());
    let repo = m::RepositoryId::new([1; 16]);
    let body = serde_json::json!({"version":1,"binding_generation":6,"operation":null,"revision":null,"command":{"kind":"refresh"}});
    assert_eq!(
        request(&api, repo, body.clone(), "http://127.0.0.1:7443")
            .await
            .0,
        StatusCode::CONFLICT
    );
    let mut arbitrary = body.clone();
    arbitrary["path"] = serde_json::json!("/arbitrary");
    assert_eq!(
        request(&api, repo, arbitrary, "http://127.0.0.1:7443")
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(&api, repo, body, "https://foreign.example").await.0,
        StatusCode::FORBIDDEN
    );
}
