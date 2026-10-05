use super::*;
use std::{
    fs,
    path::{Path as FsPath, PathBuf},
    process::Command,
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
async fn request(
    api: &HttpApi,
    repo: m::RepositoryId,
    value: serde_json::Value,
    origin: &str,
) -> (StatusCode, serde_json::Value) {
    let request = axum::http::Request::builder()
        .method("POST")
        .uri(format!("/v1/local/repos/{repo}/working-tree/observe"))
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
#[tokio::test]
async fn observation_route_rejects_client_paths_stale_bindings_and_foreign_origins() {
    let root = tempfile::tempdir().unwrap();
    let manager = manager(root.path());
    let api = api(root.path(), manager, trusted());
    let repo = m::RepositoryId::new([1; 16]);
    let body = serde_json::json!({"version":1,"binding_generation":6,"operation":m::OperationId::new([2;64])});
    let (status, error) = request(&api, repo, body.clone(), "http://127.0.0.1:7443").await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(error["code"], "stale_working_tree");
    let mut arbitrary = body.clone();
    arbitrary["path"] = serde_json::json!("/arbitrary");
    assert_eq!(
        request(&api, repo, arbitrary, "http://127.0.0.1:7443")
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    let mut version = body.clone();
    version["version"] = serde_json::json!(2);
    assert_eq!(
        request(&api, repo, version, "http://127.0.0.1:7443")
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(&api, repo, body.clone(), "https://foreign.example")
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(
            &api,
            m::RepositoryId::new([9; 16]),
            body,
            "http://127.0.0.1:7443"
        )
        .await
        .0,
        StatusCode::SERVICE_UNAVAILABLE
    );
}
#[tokio::test]
async fn observation_route_requires_administrative_localtrusted_admission() {
    let root = tempfile::tempdir().unwrap();
    let api = api(
        root.path(),
        manager(root.path()),
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Read,
        },
    );
    let body = serde_json::json!({"version":1,"binding_generation":7,"operation":m::OperationId::new([2;64])});
    assert_eq!(
        request(
            &api,
            m::RepositoryId::new([1; 16]),
            body,
            "http://127.0.0.1:7443"
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
}
fn files(root: &FsPath) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &FsPath, path: &FsPath, out: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            let metadata = fs::symlink_metadata(&path).unwrap();
            if metadata.is_dir() {
                visit(root, &path, out);
            } else if metadata.is_file() {
                out.insert(
                    path.strip_prefix(root).unwrap().into(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut out = BTreeMap::new();
    visit(root, root, &mut out);
    out
}
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires explicit existing pinned development JJ executable"]
async fn registered_observation_route_reads_real_checkout_without_publication_or_content_leaks() {
    let binary = PathBuf::from(
        std::env::var_os("JJ_STORE_WORKING_TREE_TEST_JJ").expect("explicit development executable"),
    );
    assert!(binary.is_absolute());
    let fixture = tempfile::tempdir().unwrap();
    let parent = fixture.path().canonicalize().unwrap();
    let root = parent.join("repo");
    let config = parent.join("fixture.toml");
    fs::write(
        &config,
        "[user]\nname='Transport fixture'\nemail='transport@example.test'\n",
    )
    .unwrap();
    let command = |args: &[&str]| {
        let value = Command::new(&binary)
            .args(args)
            .env("JJ_CONFIG", &config)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .current_dir(&parent)
            .output()
            .unwrap();
        assert!(
            value.status.success(),
            "{}",
            String::from_utf8_lossy(&value.stderr)
        );
    };
    command(&["git", "init", "--colocate", root.to_str().unwrap()]);
    fs::write(root.join("tracked"), "before\n").unwrap();
    command(&[
        "-R",
        root.to_str().unwrap(),
        "describe",
        "-m",
        "Transport baseline",
    ]);
    let global = parent.join("empty-ignore");
    fs::write(&global, "").unwrap();
    let path = root.join(".git/config");
    let bytes = fs::read_to_string(&path).unwrap();
    fs::write(
        path,
        format!("{bytes}\n[core]\nexcludesFile={}\n", global.display()),
    )
    .unwrap();
    fs::write(root.join("tracked"), "private after bytes\n").unwrap();
    fs::write(root.join(".gitignore"), "ignored\n").unwrap();
    fs::write(root.join("ignored"), "obstacle").unwrap();
    let manager = manager(&root);
    let api = api(&parent, manager.clone(), trusted());
    let repo = m::RepositoryId::new([1; 16]);
    let access = binding(&api.0, repo).await.unwrap();
    let heads = api.0.local_reader.heads(access).await.unwrap().heads;
    assert_eq!(heads.len(), 1);
    let before = files(&root);
    let body = serde_json::json!({"version":1,"binding_generation":7,"operation":heads[0]});
    let (status, data) = request(&api, repo, body.clone(), "http://127.0.0.1:7443").await;
    assert_eq!(status, StatusCode::OK, "{data}");
    assert_eq!(data["snapshot"]["operation"], serde_json::json!(heads[0]));
    assert_eq!(
        data["changes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["path"].as_str().unwrap())
            .collect::<Vec<_>>(),
        [".gitignore", "tracked"]
    );
    assert_eq!(data["obstacle_count"], 2);
    let text = serde_json::to_string(&data).unwrap();
    assert!(
        !text.contains(root.to_str().unwrap())
            && !text.contains("private after bytes")
            && !text.contains("empty-ignore")
    );
    assert_eq!(files(&root), before);
    assert_eq!(
        api.0
            .local_reader
            .heads(binding(&api.0, repo).await.unwrap())
            .await
            .unwrap()
            .heads,
        heads
    );
    manager.generation.store(8, Ordering::Release);
    let (stale, error) = request(&api, repo, body, "http://127.0.0.1:7443").await;
    assert_eq!(stale, StatusCode::CONFLICT);
    if let Some(path) = std::env::var_os("JJ_STORE_WORKING_TREE_CONTRACT_CAPTURE") {
        fs::write(path,serde_json::to_vec(&serde_json::json!([
        {"path":"/v1/local/repos/{repo}/working-tree/observe","method":"post","status":200,"media":"application/json","data":data},
        {"path":"/v1/local/repos/{repo}/working-tree/observe","method":"post","status":409,"media":"application/json","data":error},
    ])).unwrap()).unwrap();
    }
}
