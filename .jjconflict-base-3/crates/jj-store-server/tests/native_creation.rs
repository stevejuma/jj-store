use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_server::{
    installation::{Identity, initialize_registry},
    local_repositories::DeviceRepositories,
    registry::Registry,
};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    auth::{self, Permission, Scope, SystemClock, TokenPolicy},
    local_browser::*,
    local_repository::LocalRepository,
    native::SessionLimits,
    registry::RepositoryRegistry,
};
use std::sync::Arc;
struct Operators;
#[async_trait::async_trait]
impl OperatorPolicies for Operators {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(TokenPolicy::new(Scope::Operator))
    }
}
struct Folders;
impl FolderProvider for Folders {
    fn inspect(&self, _: &str) -> Result<LocalFolder> {
        Err(Error::Unsupported)
    }
    fn open(&self, _: &LocalFolder) -> Result<LocalRepository> {
        Err(Error::Unsupported)
    }
}
struct Fixture {
    root: tempfile::TempDir,
    host: RepositoryHost,
    store: Arc<MemoryStore>,
    registry: Arc<Registry>,
    identity: Identity,
    state: std::path::PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        let installed = initialize_registry(&root.path().join("state")).unwrap();
        let identity = Identity::load(&installed.installation_file).unwrap();
        let registry =
            Arc::new(Registry::new(installed.catalog.clone(), identity.clone()).unwrap());
        let store = Arc::new(MemoryStore::default());
        let host = RepositoryHost::new(
            store.clone(),
            Arc::new(Operators),
            Arc::new(SystemClock),
            Default::default(),
            SessionLimits::default(),
            16 * 1024 * 1024,
        )
        .unwrap();
        Self {
            root,
            host,
            store,
            registry,
            identity,
            state: installed.state_dir,
        }
    }
    fn manager(&self, native: bool) -> DeviceRepositories {
        DeviceRepositories::new(
            self.state.clone(),
            self.identity.clone(),
            Arc::new(Folders),
            self.registry.clone(),
        )
        .unwrap()
        .with_native_host(native.then(|| self.host.clone()))
    }
}
fn request(name: &str) -> CreateNativeRepository {
    CreateNativeRepository {
        version: 1,
        request_id: m::RequestId::new([1; 16]),
        name: name.into(),
    }
}
#[tokio::test]
async fn native_creation_replays_after_restart_without_a_physical_workspace() {
    let f = Fixture::new();
    let manager = f.manager(true);
    assert!(manager.native_creation());
    let outcome = manager
        .create_native(request("Created repository"))
        .await
        .unwrap();
    let NativeCreationOutcome::Committed { repository, .. } = outcome else {
        panic!("not committed")
    };
    assert_eq!(
        f.manager(true)
            .create_native(request("Created repository"))
            .await
            .unwrap(),
        outcome
    );
    assert_eq!(
        f.manager(true)
            .native_creation_status(request("").request_id)
            .await
            .unwrap(),
        outcome
    );
    assert!(matches!(
        manager.create_native(request("Changed intent")).await,
        Err(Error::Conflict)
    ));
    let catalog = f.registry.snapshot(None).await.unwrap();
    assert_eq!(catalog.repositories.len(), 1);
    assert_eq!(catalog.repositories[0].id, repository);
    assert!(manager.list().unwrap().bindings.is_empty());
    let session = f
        .host
        .endpoint(repository)
        .unwrap()
        .bind_local_trusted(f.identity.principal(b"loopback-http"), Permission::Admin)
        .unwrap()
        .into_session();
    assert_eq!(session.snapshot().await.unwrap().manifest().heads.len(), 1);
    assert!(matches!(
        session.open_read(None, Default::default()).await.unwrap(),
        jj_store_service::reads::OpenRead::Pinned(_)
    ));
    assert!(!f.root.path().join("repository").exists());
    let unknown = m::RequestId::new([9; 16]);
    assert_eq!(
        manager.native_creation_status(unknown).await.unwrap(),
        NativeCreationOutcome::NotKnown {
            request_id: unknown
        }
    );
}
#[tokio::test]
async fn native_creation_rejects_names_and_unavailable_authority_without_publication() {
    let f = Fixture::new();
    assert!(!f.manager(false).native_creation());
    assert!(matches!(
        f.manager(false).create_native(request("Valid")).await,
        Err(Error::Unsupported)
    ));
    for name in ["", "a\nb", "../repo"] {
        assert!(matches!(
            f.manager(true).create_native(request(name)).await,
            Err(Error::Invalid)
        ));
    }
    let first = f
        .manager(true)
        .create_native(request("Taken"))
        .await
        .unwrap();
    let mut another = request("Taken");
    another.request_id = m::RequestId::new([2; 16]);
    assert!(matches!(
        f.manager(true).create_native(another).await,
        Err(Error::Conflict)
    ));
    assert_eq!(
        f.registry.snapshot(None).await.unwrap().repositories.len(),
        1
    );
    assert_eq!(
        f.manager(true)
            .native_creation_status(request("").request_id)
            .await
            .unwrap(),
        first
    );
}
#[tokio::test]
async fn native_creation_recovers_unconfirmed_publication_with_retained_identity() {
    let f = Fixture::new();
    f.store.inject(jj_store_object::memory::Fault::BeforeWrite);
    let first = f.manager(true).create_native(request("Interrupted")).await;
    assert!(matches!(
        first,
        Ok(NativeCreationOutcome::OutcomeUnknown { .. })
    ));
    assert!(
        f.registry
            .snapshot(None)
            .await
            .unwrap()
            .repositories
            .is_empty()
    );
    let journals = std::fs::read_dir(&f.state)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with("native-create-")
        })
        .collect::<Vec<_>>();
    assert_eq!(journals.len(), 1);
    let bytes = std::fs::read(journals[0].path()).unwrap();
    let saved: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let recovered = f
        .manager(true)
        .native_creation_status(request("").request_id)
        .await
        .unwrap();
    let NativeCreationOutcome::Committed { repository, .. } = recovered else {
        panic!("not recovered")
    };
    assert_eq!(saved["repository"], repository.to_string());
    assert_eq!(
        f.manager(true)
            .create_native(request("Interrupted"))
            .await
            .unwrap(),
        recovered
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_creation_http_is_local_admin_only_and_captures_contracts() {
    let f = Fixture::new();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let api = jj_store_http::HttpApi::new(
        f.host.clone(),
        f.root.path().join("http"),
        jj_store_http::Config {
            origin: base.clone(),
            access: jj_store_http::Access::LocalTrusted {
                principal: f.identity.principal(b"loopback-http"),
                permission: Permission::Admin,
            },
            registry: Some(f.registry.clone()),
            local_repositories: Some(Arc::new(f.manager(true))),
            ..Default::default()
        },
    )
    .unwrap();
    let task = tokio::spawn(async move { axum::serve(listener, api.router()).await.unwrap() });
    let client = reqwest::Client::new();
    let mut samples = vec![];
    for (method, path, body, expected) in [
        ("get", "/v1/bootstrap".to_string(), None, 200),
        (
            "post",
            "/v1/local/repositories/native".to_string(),
            Some(serde_json::to_value(request("HTTP created")).unwrap()),
            200,
        ),
        (
            "get",
            format!(
                "/v1/local/repositories/native/requests/{}",
                request("").request_id
            ),
            None,
            200,
        ),
        (
            "get",
            format!(
                "/v1/local/repositories/native/requests/{}",
                m::RequestId::new([9; 16])
            ),
            None,
            200,
        ),
        (
            "post",
            "/v1/local/repositories/native".to_string(),
            Some(serde_json::to_value(request("Changed")).unwrap()),
            409,
        ),
    ] {
        let mut req = client
            .request(
                method.to_uppercase().parse::<reqwest::Method>().unwrap(),
                format!("{base}{path}"),
            )
            .header("x-jj-store-protocol", "1");
        if let Some(body) = body {
            req = req
                .header("origin", &base)
                .header("content-type", "application/json")
                .body(serde_json::to_vec(&body).unwrap());
        }
        let response = req.send().await.unwrap();
        assert_eq!(response.status().as_u16(), expected);
        let response: serde_json::Value =
            serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();
        let contract_path = if path.contains("/native/requests/") {
            "/v1/local/repositories/native/requests/{request_id}"
        } else {
            &path
        };
        samples.push(serde_json::json!({"method":method,"path":contract_path,"status":expected,"media":"application/json","data":response}));
    }
    if let Some(path) = std::env::var_os("JJ_STORE_NATIVE_CREATION_CONTRACT_CAPTURE") {
        std::fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
    task.abort();
    for access in [
        jj_store_http::Access::Authenticated,
        jj_store_http::Access::LocalTrusted {
            principal: f.identity.principal(b"loopback-http"),
            permission: Permission::Read,
        },
    ] {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let api = jj_store_http::HttpApi::new(
            f.host.clone(),
            f.root.path().join("denied"),
            jj_store_http::Config {
                origin: base.clone(),
                access,
                local_repositories: Some(Arc::new(f.manager(true))),
                ..Default::default()
            },
        )
        .unwrap();
        let task = tokio::spawn(async move { axum::serve(listener, api.router()).await.unwrap() });
        let response = client
            .post(format!("{base}/v1/local/repositories/native"))
            .header("x-jj-store-protocol", "1")
            .header("origin", &base)
            .header("content-type", "application/json")
            .body(serde_json::to_vec(&request("Denied")).unwrap())
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::FORBIDDEN);
        task.abort();
    }
}

#[tokio::test]
async fn native_names_reserve_current_and_pending_local_aliases_but_not_removed_history() {
    for (current, pending, blocked) in [
        (true, false, true),
        (false, true, true),
        (false, false, false),
    ] {
        let f = Fixture::new();
        let repository = m::RepositoryId::new([7; 16]);
        let folder = LocalFolder {
            path: "/device/source".into(),
            metadata: "/device/source/.jj/repo".into(),
            repository: Some(repository),
        };
        let local = RegisterFolder {
            version: 1,
            request_id: m::RequestId::new([8; 16]),
            expected_revision: 0,
            path: folder.path.clone(),
            name: "Alias".into(),
        };
        let binding = LocalBinding {
            repository,
            name: local.name.clone(),
            path: folder.path.clone(),
            generation: 1,
            availability: Availability::Ready,
        };
        let value = serde_json::json!({"version":1,"installation":f.identity.installation,"revision":1,
            "bindings":if current {vec![serde_json::json!({"folder":folder,"name":"Alias","generation":1})]} else {vec![]},
            "requests":[{"request":local,"folder":folder,"proposed_repository":repository,
                "result":if pending {None} else {Some(RegistrationOutcome::Committed { request_id:local.request_id,binding,revision:1 })}}]});
        let path = f.state.join("local-repositories.json");
        std::fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        let outcome = f.manager(true).create_native(request("Alias")).await;
        if blocked {
            assert!(matches!(outcome, Err(Error::Conflict)));
        } else {
            assert!(matches!(
                outcome,
                Ok(NativeCreationOutcome::Committed { .. })
            ));
        }
    }
}
