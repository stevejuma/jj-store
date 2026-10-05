use super::{
    jj,
    managed_http::{request, server_with_provider},
};
use jj_store_server::{config::Config, installation};
use jj_store_service::local_browser::*;
use serde_json::json;
use std::{
    fs,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering::SeqCst},
    },
};

struct CountingFolders(Arc<AtomicUsize>);
impl FolderProvider for CountingFolders {
    fn inspect(&self, path: &str) -> Result<LocalFolder> {
        self.0.fetch_add(1, SeqCst);
        jj_store_cli::local_browser::LocalFolders.inspect(path)
    }
    fn open(
        &self,
        folder: &LocalFolder,
    ) -> Result<jj_store_service::local_repository::LocalRepository> {
        jj_store_cli::local_browser::LocalFolders.open(folder)
    }
}
#[tokio::test]
async fn local_metadata_reads_bound_registration_inspections() {
    let temp = tempfile::tempdir().unwrap();
    let folder = temp.path().join("repo");
    jj(
        temp.path(),
        &["git", "init", "--colocate", folder.to_str().unwrap()],
    );
    for i in 0..40 {
        fs::write(folder.join(format!("file-{i:02}")), "body").unwrap();
    }
    jj(&folder, &["commit", "-m", "Files"]);
    for _ in 0..3 {
        fs::write(
            folder.join("unrelated"),
            format!("{:?}", std::time::Instant::now()),
        )
        .unwrap();
        jj(&folder, &["commit", "-m", "Other"]);
    }
    let operation = jj(&folder, &["op", "log", "--no-graph", "-n", "1", "-T", "id"]);
    let commit = jj(
        &folder,
        &[
            "log",
            "--ignore-working-copy",
            "-r",
            "@",
            "--no-graph",
            "-T",
            "commit_id",
        ],
    );
    let identity = installation::initialize_filesystem(&temp.path().join("state")).unwrap();
    let mut config = Config::load(&identity.config).unwrap();
    let address = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap();
    config.bind = address;
    config.public_url = format!("http://{address}/");
    let inspections = Arc::new(AtomicUsize::new(0));
    let (stop, running) =
        server_with_provider(&config, Arc::new(CountingFolders(inspections.clone()))).await;
    let client = reqwest::Client::new();
    let (status, registered) = request(&client, address, "/local/repositories", Some(&json!({"version":1,"request_id":"46464646464646464646464646464646","expected_revision":0,"path":folder,"name":"Admission count"})), false).await;
    assert_eq!(status, 200, "{registered}");
    let id = registered["binding"]["repository"].as_str().unwrap();
    let before = super::preparation_acceptance::files(&folder.join(".jj/repo"));
    let initial = inspections.load(SeqCst);
    let query = json!({"operation":operation,"cursor":null,"request":{"kind":"paths","query":{"commit":commit,"path":"","recursive":false,"include_last_change":true,"limit":50,"anchor":null,"direction":"forward"}}});
    let (status, response) = request(
        &client,
        address,
        &format!("/local/repos/{id}/query"),
        Some(&query),
        false,
    )
    .await;
    let count = inspections.load(SeqCst) - initial;
    stop.send(()).unwrap();
    running.await.unwrap();
    assert_eq!(status, 200, "{response}");
    assert_eq!(
        response["data"]["data"]["entries"]
            .as_array()
            .unwrap()
            .len(),
        41
    );
    assert_eq!(
        before,
        super::preparation_acceptance::files(&folder.join(".jj/repo"))
    );
    eprintln!("40 rows across 3 unrelated edits: {count} physical binding inspections");
    assert!(
        count <= 20,
        "physical inspections must scale with request boundaries, not rows: {count}"
    );
}

struct MissedEventBinding {
    source: super::ReadBinding,
    checks: AtomicUsize,
    boundaries: AtomicUsize,
}
#[async_trait::async_trait]
impl jj_store_jj::local_browser::Access for MissedEventBinding {
    fn repository(&self) -> jj_store_model::RepositoryId {
        self.source.repository()
    }
    fn generation(&self) -> u64 {
        self.source.generation()
    }
    fn metadata_path(&self) -> &std::path::Path {
        self.source.metadata_path()
    }
    async fn authorize(&self) -> jj_store_jj::local_browser::Result<()> {
        self.boundaries.fetch_add(1, SeqCst);
        if self.checks.load(SeqCst) != 0 {
            return Err(jj_store_jj::query::Error::Denied);
        }
        Ok(())
    }
    async fn check(&self) -> jj_store_jj::local_browser::Result<()> {
        // Simulate cached success after an external change whose event was lost.
        self.checks.fetch_add(1, SeqCst);
        Ok(())
    }
}
#[tokio::test]
async fn cached_inner_checks_cannot_bypass_final_physical_validation() {
    use jj_store_jj::local_browser as q;
    let source = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    fs::write(source.path().join("file"), "body").unwrap();
    jj(source.path(), &["describe", "-m", "File"]);
    let commit = jj(
        source.path(),
        &["log", "-r", "@", "--no-graph", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let operation = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let access = Arc::new(MissedEventBinding {
        source: super::ReadBinding(source.path().join(".jj/repo")),
        checks: 0.into(),
        boundaries: 0.into(),
    });
    let result = q::Executor::new(1)
        .unwrap()
        .read(
            access.clone(),
            operation,
            q::Request::Paths(q::paths::Request {
                commit,
                path: String::new(),
                recursive: false,
                include_last_change: true,
                last_change_mode: Default::default(),
                limit: 10,
                anchor: None,
                direction: Default::default(),
            }),
        )
        .await;
    assert!(matches!(result, Err(q::Error::Denied)));
    assert!(access.checks.load(SeqCst) > 0);
    assert_eq!(access.boundaries.load(SeqCst), 2);
}

#[tokio::test]
async fn registration_persistence_invalidates_shared_admission_synchronously() {
    use jj_store_server::{
        installation::{Identity, initialize_registry},
        local_repositories::DeviceRepositories,
        registry::Registry,
    };
    let temp = tempfile::tempdir().unwrap();
    let folder = temp.path().join("repo");
    jj(
        temp.path(),
        &["git", "init", "--colocate", folder.to_str().unwrap()],
    );
    let installed = initialize_registry(&temp.path().join("state")).unwrap();
    let identity = Identity::load(&installed.installation_file).unwrap();
    let manager = DeviceRepositories::new(
        installed.state_dir.clone(),
        identity.clone(),
        Arc::new(jj_store_cli::local_browser::LocalFolders),
        Arc::new(Registry::new(installed.catalog.clone(), identity).unwrap()),
    )
    .unwrap();
    let RegistrationOutcome::Committed {
        binding, revision, ..
    } = manager
        .register(RegisterFolder {
            version: 1,
            request_id: jj_store_model::RequestId::new([49; 16]),
            expected_revision: 0,
            path: folder.to_str().unwrap().into(),
            name: "Admission".into(),
        })
        .await
        .unwrap()
    else {
        panic!()
    };
    let id = binding.repository;
    let admission = manager.read_admission(id).unwrap();
    let (original, generation) = manager.folder(id).unwrap();
    admission
        .validate(&original, generation, true, || manager.folder(id))
        .unwrap();
    assert!(admission.cached(&original, generation));
    let moved = temp.path().join("moved");
    fs::rename(&folder, &moved).unwrap();
    let updated = manager
        .clone()
        .reconnect(
            id,
            ReconnectFolder {
                version: 1,
                expected_revision: revision,
                path: moved.to_str().unwrap().into(),
            },
        )
        .unwrap();
    assert!(!admission.cached(&original, generation));
    let (current, generation) = manager.folder(id).unwrap();
    admission
        .validate(&current, generation, true, || manager.folder(id))
        .unwrap();
    assert!(admission.cached(&current, generation));
    manager
        .clone()
        .remove(
            id,
            RemoveBinding {
                version: 1,
                expected_revision: updated.revision,
            },
        )
        .unwrap();
    assert!(!admission.cached(&current, generation));
    assert!(
        admission
            .validate(&current, generation, true, || manager.folder(id))
            .is_err()
    );
}
