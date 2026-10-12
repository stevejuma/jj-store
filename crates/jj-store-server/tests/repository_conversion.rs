use jj_store_jj::local_metadata::{Limits as MetadataLimits, MetadataStore};
use jj_store_model as m;
use jj_store_object::{self as object, ObjectStore, memory::MemoryStore};
use jj_store_server::{
    installation::{Identity, initialize_registry},
    local_repositories::DeviceRepositories,
    registry::Registry,
};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    auth::{self, Permission, Scope, SystemClock, TokenPolicy},
    conversion::contracts::*,
    local_browser::*,
    local_repository::*,
    native::SessionLimits,
    registry::RepositoryRegistry,
};
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
struct Operators;
#[async_trait::async_trait]
impl OperatorPolicies for Operators {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(TokenPolicy::new(Scope::Operator))
    }
}
struct Folders;
impl FolderProvider for Folders {
    fn inspect(&self, path: &str) -> Result<LocalFolder> {
        let root = Path::new(path)
            .canonicalize()
            .map_err(|_| Error::Unavailable)?;
        let metadata = MetadataStore::open(&root.join(".git"), MetadataLimits::default())
            .map_err(|_| Error::Unsupported)?;
        Ok(LocalFolder {
            path: root.to_str().unwrap().into(),
            metadata: root.join(".jj/repo").to_str().unwrap().into(),
            repository: metadata
                .read()
                .map_err(|_| Error::Unavailable)?
                .map(|s| s.repository()),
        })
    }
    fn open(&self, folder: &LocalFolder) -> Result<LocalRepository> {
        if self.inspect(&folder.path)? != *folder {
            return Err(Error::IdentityChanged);
        }
        Ok(LocalRepository::new(
            Path::new(&folder.metadata),
            MetadataStore::open(
                &Path::new(&folder.path).join(".git"),
                MetadataLimits::default(),
            )
            .map_err(|_| Error::Unsupported)?,
        ))
    }
}
struct LostFinal {
    store: Arc<MemoryStore>,
    armed: AtomicBool,
}
#[async_trait::async_trait]
impl ObjectStore for LostFinal {
    async fn head(&self, key: &object::Key) -> object::Result<object::Metadata> {
        self.store.head(key).await
    }
    async fn get(
        &self,
        key: &object::Key,
        range: Option<object::ByteRange>,
    ) -> object::Result<object::Download> {
        self.store.get(key, range).await
    }
    async fn put(
        &self,
        key: &object::Key,
        condition: object::Condition,
        upload: &object::StagedUpload,
    ) -> object::Result<object::WriteOutcome> {
        let result = self.store.put(key, condition, upload).await?;
        if key.as_str().starts_with("repos/")
            && key.as_str().ends_with("/manifest.json")
            && self.armed.swap(false, Ordering::SeqCst)
        {
            return Err(object::Error::Transport);
        }
        Ok(result)
    }
}
struct Fixture {
    root: tempfile::TempDir,
    state: PathBuf,
    identity: Identity,
    registry: Arc<Registry>,
    host: RepositoryHost,
    store: Arc<MemoryStore>,
    fault: Arc<LostFinal>,
}
impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        let installed = initialize_registry(&root.path().join("state")).unwrap();
        let identity = Identity::load(&installed.installation_file).unwrap();
        let registry =
            Arc::new(Registry::new(installed.catalog.clone(), identity.clone()).unwrap());
        let store = Arc::new(MemoryStore::default());
        let fault = Arc::new(LostFinal {
            store: store.clone(),
            armed: AtomicBool::new(false),
        });
        let host = RepositoryHost::new(
            fault.clone(),
            Arc::new(Operators),
            Arc::new(SystemClock),
            Default::default(),
            SessionLimits::default(),
            16 * 1024 * 1024,
        )
        .unwrap();
        Self {
            root,
            state: installed.state_dir,
            identity,
            registry,
            host,
            store,
            fault,
        }
    }
    fn manager(&self) -> DeviceRepositories {
        DeviceRepositories::new(
            self.state.clone(),
            self.identity.clone(),
            Arc::new(Folders),
            self.registry.clone(),
        )
        .unwrap()
        .with_native_host(Some(self.host.clone()))
    }
    async fn local(&self) -> (m::RepositoryId, m::OperationId, PathBuf) {
        let path = self.root.path().join("source");
        stock(
            path.parent().unwrap(),
            &["git", "init", "--colocate", path.to_str().unwrap()],
        );
        std::fs::write(path.join("committed.txt"), "committed content").unwrap();
        stock(&path, &["describe", "-m", "source state"]);
        let outcome = self
            .manager()
            .register(RegisterFolder {
                version: 1,
                request_id: m::RequestId::new([10; 16]),
                expected_revision: 0,
                path: path.to_str().unwrap().into(),
                name: "Source".into(),
            })
            .await
            .unwrap();
        let RegistrationOutcome::Committed { binding, .. } = outcome else {
            panic!()
        };
        let operation = stock(&path, &["op", "log", "--no-graph", "-n", "1", "-T", "id"])
            .trim()
            .parse()
            .unwrap();
        std::fs::write(path.join("dirty.txt"), "keep dirty").unwrap();
        (binding.repository, operation, path)
    }
}
fn stock(path: &Path, args: &[&str]) -> String {
    let output = std::process::Command::new("/opt/homebrew/bin/jj")
        .current_dir(path)
        .args(["--no-pager"])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
fn intent(
    source: m::RepositoryId,
    operation: m::OperationId,
    profile: ConversionProfile,
    name: &str,
    path: Option<String>,
) -> ConversionIntent {
    ConversionIntent {
        version: 1,
        source,
        source_profile: profile,
        operation,
        destination_profile: if profile == ConversionProfile::Native {
            ConversionProfile::LocalJjGit
        } else {
            ConversionProfile::Native
        },
        name: name.into(),
        path,
    }
}
async fn start(
    f: &Fixture,
    intent: ConversionIntent,
    id: u8,
) -> (StartConversion, ConversionOutcome) {
    let preview = f
        .manager()
        .preview_conversion(intent.clone())
        .await
        .unwrap();
    assert!(preview.blocker.is_none(), "{:?}", preview.blocker);
    let request = StartConversion {
        request_id: m::RequestId::new([id; 16]),
        intent,
        prepared_source: preview.prepared_source.unwrap(),
    };
    let result = f.manager().start_conversion(request.clone()).await.unwrap();
    (request, result)
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn conversion_round_trip_preserves_source_and_publishes_new_colocated_folder() {
    let f = Fixture::new();
    let (source, operation, path) = f.local().await;
    let selected = f
        .manager()
        .open(source)
        .unwrap()
        .metadata_store()
        .read()
        .unwrap()
        .unwrap()
        .commit();
    let (request, outcome) = start(
        &f,
        intent(
            source,
            operation,
            ConversionProfile::LocalJjGit,
            "Server copy",
            None,
        ),
        20,
    )
    .await;
    let ConversionOutcome::Committed { report } = outcome else {
        panic!("not committed")
    };
    assert_ne!(report.destination, source);
    assert!(!report.code_mappings["commits"].is_empty());
    assert_eq!(
        f.manager()
            .open(source)
            .unwrap()
            .metadata_store()
            .read()
            .unwrap()
            .unwrap()
            .commit(),
        selected
    );
    assert_eq!(
        std::fs::read_to_string(path.join("dirty.txt")).unwrap(),
        "keep dirty"
    );
    let replay = f.manager().start_conversion(request.clone()).await.unwrap();
    assert_eq!(
        serde_json::to_value(replay).unwrap(),
        serde_json::to_value(ConversionOutcome::Committed {
            report: report.clone()
        })
        .unwrap()
    );
    let destination = f.root.path().join("exported");
    let (_, exported) = start(
        &f,
        intent(
            report.destination,
            report.operation,
            ConversionProfile::Native,
            "Local copy",
            Some(destination.to_str().unwrap().into()),
        ),
        21,
    )
    .await;
    let ConversionOutcome::Committed {
        report: local_report,
    } = exported
    else {
        panic!("export not committed")
    };
    assert_eq!(
        std::fs::read_to_string(destination.join("committed.txt")).unwrap(),
        "committed content"
    );
    assert!(!destination.join("dirty.txt").exists());
    assert!(destination.join(".git").is_dir());
    assert_eq!(
        f.manager()
            .open(local_report.destination)
            .unwrap()
            .repository()
            .unwrap(),
        Some(local_report.destination)
    );
    assert!(
        stock(
            &destination,
            &["op", "log", "--no-graph", "-n", "1", "-T", "id"]
        )
        .contains(&local_report.operation.to_string())
    );
    let mut changed = request;
    changed.intent.name = "Changed".into();
    assert_eq!(
        f.manager().start_conversion(changed).await.unwrap_err(),
        Error::Conflict
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn lost_final_manifest_ack_recovers_after_source_removal_and_restart() {
    let f = Fixture::new();
    let (source, operation, path) = f.local().await;
    f.fault.armed.store(true, Ordering::SeqCst);
    let (request, outcome) = start(
        &f,
        intent(
            source,
            operation,
            ConversionProfile::LocalJjGit,
            "Recovered",
            None,
        ),
        30,
    )
    .await;
    assert!(matches!(outcome, ConversionOutcome::OutcomeUnknown { .. }));
    assert!(
        f.registry
            .snapshot(None)
            .await
            .unwrap()
            .repositories
            .is_empty()
    );
    std::fs::remove_dir_all(path).unwrap();
    let resumed = f
        .manager()
        .conversion_status(request.request_id)
        .await
        .unwrap();
    let ConversionOutcome::Committed { report } = resumed else {
        panic!("not recovered")
    };
    assert_eq!(report.archive_digest, request.prepared_source);
    assert_eq!(
        f.registry.snapshot(None).await.unwrap().repositories.len(),
        1
    );
    assert_eq!(
        serde_json::to_value(f.manager().start_conversion(request).await.unwrap()).unwrap(),
        serde_json::to_value(ConversionOutcome::Committed { report }).unwrap()
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unfinished_preparation_retries_the_saved_archive_after_source_removal() {
    let f = Fixture::new();
    let (source, operation, path) = f.local().await;
    f.store.inject(object::memory::Fault::BeforeWrite);
    let (request, outcome) = start(
        &f,
        intent(
            source,
            operation,
            ConversionProfile::LocalJjGit,
            "Prepared later",
            None,
        ),
        31,
    )
    .await;
    assert!(matches!(outcome, ConversionOutcome::OutcomeUnknown { .. }));
    std::fs::remove_dir_all(path).unwrap();
    let journal = f
        .state
        .join(format!("conversion-{}.json", request.request_id));
    let before = std::fs::read(&journal).unwrap();
    for _ in 0..3 {
        assert!(matches!(
            f.manager()
                .conversion_status(request.request_id)
                .await
                .unwrap(),
            ConversionOutcome::OutcomeUnknown { .. }
        ));
        assert_eq!(std::fs::read(&journal).unwrap(), before);
    }
    assert!(matches!(
        f.manager().start_conversion(request).await.unwrap(),
        ConversionOutcome::Committed { .. }
    ));
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn conversion_preflight_rejects_occupied_folders_and_changed_source() {
    let f = Fixture::new();
    let (source, operation, path) = f.local().await;
    let original = intent(
        source,
        operation,
        ConversionProfile::LocalJjGit,
        "Stale",
        None,
    );
    let preview = f
        .manager()
        .preview_conversion(original.clone())
        .await
        .unwrap();
    // A committed WorkStore edit changes the accepted companion selection.
    let local = f.manager().open(source).unwrap();
    local
        .apply_work_command(&jj_store_service::work::Request {
            version: 1,
            request_id: m::RequestId::new([51; 16]),
            command: jj_store_service::work::WorkCommandV1::CreateProject {
                id: jj_store_service::work::ProjectId::new([52; 16]),
                title: "New".into(),
                body: String::new(),
            },
        })
        .unwrap();
    assert_eq!(
        f.manager()
            .start_conversion(StartConversion {
                request_id: m::RequestId::new([53; 16]),
                intent: original,
                prepared_source: preview.prepared_source.unwrap()
            })
            .await
            .unwrap_err(),
        Error::Stale
    );
    let (_, native) = start(
        &f,
        intent(
            source,
            operation,
            ConversionProfile::LocalJjGit,
            "For export",
            None,
        ),
        54,
    )
    .await;
    let ConversionOutcome::Committed { report } = native else {
        panic!()
    };
    let occupied = f.root.path().join("occupied");
    std::fs::create_dir(&occupied).unwrap();
    std::fs::write(occupied.join("keep"), "untouched").unwrap();
    assert_eq!(
        f.manager()
            .preview_conversion(intent(
                report.destination,
                report.operation,
                ConversionProfile::Native,
                "Occupied",
                Some(occupied.to_str().unwrap().into())
            ))
            .await
            .unwrap_err(),
        Error::Conflict
    );
    assert_eq!(
        std::fs::read_to_string(occupied.join("keep")).unwrap(),
        "untouched"
    );
    assert_eq!(
        std::fs::read_to_string(path.join("dirty.txt")).unwrap(),
        "keep dirty"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn conversion_http_derives_admin_authority_and_captures_contracts() {
    let f = Fixture::new();
    let (source, operation, _) = f.local().await;
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
            local_repositories: Some(Arc::new(f.manager())),
            ..Default::default()
        },
    )
    .unwrap();
    let task = tokio::spawn(async move { axum::serve(listener, api.router()).await.unwrap() });
    let client = reqwest::Client::new();
    let mut samples = Vec::new();
    async fn request(
        client: &reqwest::Client,
        base: &str,
        samples: &mut Vec<serde_json::Value>,
        method: &str,
        path: &str,
        body: Option<serde_json::Value>,
        expected: u16,
    ) -> serde_json::Value {
        let mut req = client
            .request(
                method.to_uppercase().parse::<reqwest::Method>().unwrap(),
                format!("{base}{path}"),
            )
            .header("x-jj-store-protocol", "1");
        if let Some(body) = body {
            req = req
                .header("origin", base)
                .header("content-type", "application/json")
                .body(serde_json::to_vec(&body).unwrap());
        }
        let response = req.send().await.unwrap();
        assert_eq!(response.status().as_u16(), expected);
        let data: serde_json::Value =
            serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();
        let contract_path = if path.contains("/conversion/requests/") {
            "/v1/local/repositories/conversion/requests/{request_id}"
        } else {
            path
        };
        samples.push(serde_json::json!({"method":method,"path":contract_path,"status":expected,"media":"application/json","data":data}));
        data
    }
    let intent = intent(
        source,
        operation,
        ConversionProfile::LocalJjGit,
        "HTTP server copy",
        None,
    );
    let preview = request(
        &client,
        &base,
        &mut samples,
        "post",
        "/v1/local/repositories/conversion/preview",
        Some(serde_json::to_value(&intent).unwrap()),
        200,
    )
    .await;
    assert!(preview["blocker"].is_null());
    let start = StartConversion {
        request_id: m::RequestId::new([60; 16]),
        intent,
        prepared_source: preview["prepared_source"].as_str().unwrap().into(),
    };
    let committed = request(
        &client,
        &base,
        &mut samples,
        "post",
        "/v1/local/repositories/conversion",
        Some(serde_json::to_value(&start).unwrap()),
        200,
    )
    .await;
    assert_eq!(committed["status"], "committed");
    assert_eq!(
        request(
            &client,
            &base,
            &mut samples,
            "get",
            &format!(
                "/v1/local/repositories/conversion/requests/{}",
                start.request_id
            ),
            None,
            200
        )
        .await,
        committed
    );
    let unknown = request(
        &client,
        &base,
        &mut samples,
        "get",
        &format!(
            "/v1/local/repositories/conversion/requests/{}",
            m::RequestId::new([61; 16])
        ),
        None,
        200,
    )
    .await;
    assert_eq!(unknown["status"], "not_known");
    let mut injected = serde_json::to_value(&start).unwrap();
    injected["actor"] = serde_json::json!(m::PrincipalId::new([62; 16]));
    request(
        &client,
        &base,
        &mut samples,
        "post",
        "/v1/local/repositories/conversion",
        Some(injected),
        400,
    )
    .await;
    let mut unsupported = start.intent.clone();
    unsupported.operation = m::OperationId::new([63; 64]);
    let blocked = request(
        &client,
        &base,
        &mut samples,
        "post",
        "/v1/local/repositories/conversion/preview",
        Some(serde_json::to_value(unsupported).unwrap()),
        200,
    )
    .await;
    assert!(!blocked["blocker"].is_null());
    assert!(blocked["prepared_source"].is_null());
    if let Some(path) = std::env::var_os("JJ_STORE_CONVERSION_CONTRACT_CAPTURE") {
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
                local_repositories: Some(Arc::new(f.manager())),
                ..Default::default()
            },
        )
        .unwrap();
        let task = tokio::spawn(async move { axum::serve(listener, api.router()).await.unwrap() });
        for (method, path, body) in [
            (
                "post",
                "/v1/local/repositories/conversion/preview".into(),
                Some(serde_json::to_value(&start.intent).unwrap()),
            ),
            (
                "post",
                "/v1/local/repositories/conversion".into(),
                Some(serde_json::to_value(&start).unwrap()),
            ),
            (
                "get",
                format!(
                    "/v1/local/repositories/conversion/requests/{}",
                    start.request_id
                ),
                None,
            ),
        ] {
            request(&client, &base, &mut vec![], method, &path, body, 403).await;
        }
        task.abort();
    }
}

fn batch() -> jj_store_wal::publication::SanitizedObservationBatchV1 {
    use jj_store_wal::publication::*;
    let values: Vec<serde_json::Value> = serde_json::from_slice(include_bytes!(
        "../../jj-agent/tests/fixtures/codex-0.156.1-hooks.json"
    ))
    .unwrap();
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([70; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([71; 16]),
        sanitizer_version: "agent-redaction@0.1.0".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: values
            .into_iter()
            .enumerate()
            .map(|(i, value)| SanitizedObservationV1 {
                observation_id: ObservationId::new([i as u8 + 1; 16]),
                source_sequence: i as u64 + 1,
                occurred_at_millis: Some(1_725_000_000_000 + i as i64),
                source_kind: SourceKind::Hook,
                source_trust: SourceTrust::LocalUserHook,
                payload: SanitizedPayloadV1::new(
                    SanitizedMediaType::Json,
                    &serde_json::to_vec(&value).unwrap(),
                )
                .unwrap(),
            })
            .collect(),
    }
}

async fn collaboration_fixture(local: &LocalRepository) {
    use jj_collab::*;
    use jj_store_service::work::*;
    use jj_store_wal::publication::*;
    let project = ProjectId::new([81; 16]);
    for (id, command) in [
        (
            81,
            WorkCommandV1::CreateProject {
                id: project,
                title: "Original project".into(),
                body: String::new(),
            },
        ),
        (
            82,
            WorkCommandV1::UpdateProject {
                id: project,
                expected_version: 1,
                title: "Renamed project".into(),
                body: "History retained".into(),
            },
        ),
    ] {
        local
            .apply_work_command(&jj_store_service::work::Request {
                version: 1,
                request_id: m::RequestId::new([id; 16]),
                command,
            })
            .unwrap();
    }
    local
        .apply_agent_command(&LocalAgentRequest {
            version: 1,
            request_id: m::RequestId::new([83; 16]),
            command: LocalAgentCommand::Capture {
                profile: NormalizerProfileV1::codex_0_156_1(),
                batch: Box::new(batch()),
            },
        })
        .await
        .unwrap();
    local
        .apply_review_command(&LocalReviewRequest {
            version: 1,
            request_id: m::RequestId::new([84; 16]),
            command: LocalReviewCommand::RepositoryThread {
                body: ReviewCommentBodyV1 {
                    schema_version: 1,
                    format: ReviewBodyFormatV1::PlainText,
                    text: "Retained review".into(),
                    sanitizer_version: "plain-text@1".into(),
                },
            },
        })
        .await
        .unwrap();
    for (id, command) in [
        (
            85,
            LocalCollabCommand::Thread {
                intent: ThreadCommandV1::Create {
                    thread_id: ThreadId::new([85; 16]),
                    title: "Retained thread".into(),
                    baseline: None,
                },
            },
        ),
        (
            86,
            LocalCollabCommand::Reaction {
                intent: ReactionCommandV1::Set {
                    emoji: None,
                    target: ReactionTargetV1::Thread {
                        thread_id: ThreadId::new([85; 16]),
                    },
                    value: ReactionValueV1::Like,
                    observed: Default::default(),
                },
            },
        ),
    ] {
        local
            .apply_collab_command(&LocalCollabRequest {
                version: 1,
                request_id: m::RequestId::new([id; 16]),
                command,
            })
            .await
            .unwrap();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires explicitly selected isolated embedded RustFS runtime"]
async fn cold_rustfs_conversion_round_trip_retains_nonempty_history_across_restart() {
    use jj_store_server::storage::{EmbeddedStorageConfig, EmbeddedStorageRuntime};
    let mut f = Fixture::new();
    let volume = f.root.path().join("volume");
    std::fs::create_dir(&volume).unwrap();
    let config = EmbeddedStorageConfig {
        runtime_executable: std::fs::canonicalize(
            std::env::var_os("JJ_STORE_RUSTFS_RUNTIME").expect("explicit runtime"),
        )
        .unwrap(),
        volume,
        address: format!(
            "127.0.0.1:{}",
            std::net::TcpListener::bind("127.0.0.1:0")
                .unwrap()
                .local_addr()
                .unwrap()
                .port()
        ),
        access_key: "conversion-disposable-access".into(),
        secret_key: "conversion-disposable-secret".into(),
        region: "us-east-1".into(),
        bucket: "conversion-qualification".into(),
        max_object_bytes: 16 * 1024 * 1024,
        timeout: std::time::Duration::from_secs(30),
    };
    let first = EmbeddedStorageRuntime::start(config.clone()).await.unwrap();
    first.store().ensure_bucket().await.unwrap();
    f.host = f.host.with_conversion_store(first.store());
    let (source, operation, path) = f.local().await;
    collaboration_fixture(&f.manager().open(source).unwrap()).await;
    let source_archive = f
        .manager()
        .open(source)
        .unwrap()
        .conversion_archive(operation)
        .await
        .unwrap();
    let selected = f
        .manager()
        .open(source)
        .unwrap()
        .metadata_store()
        .read()
        .unwrap()
        .unwrap()
        .commit();
    let (request, outcome) = start(
        &f,
        intent(
            source,
            operation,
            ConversionProfile::LocalJjGit,
            "RustFS native",
            None,
        ),
        90,
    )
    .await;
    let ConversionOutcome::Committed { report } = outcome else {
        panic!("native conversion")
    };
    println!("Native import committed; restarting isolated RustFS.");
    first.shutdown().await;
    let second = EmbeddedStorageRuntime::start(config).await.unwrap();
    f.host = f.host.with_conversion_store(second.store());
    assert_eq!(
        serde_json::to_value(
            f.manager()
                .conversion_status(request.request_id)
                .await
                .unwrap()
        )
        .unwrap(),
        serde_json::to_value(ConversionOutcome::Committed {
            report: report.clone()
        })
        .unwrap()
    );
    let native = f
        .host
        .endpoint(report.destination)
        .unwrap()
        .bind_local_trusted(f.identity.principal(b"loopback-http"), Permission::Admin)
        .unwrap()
        .into_session();
    let native_archive = native.conversion_archive(report.operation).await.unwrap();
    assert!(native_archive.collaboration.len() >= source_archive.collaboration.len());
    let destination = f.root.path().join("rustfs-export");
    let (_, outcome) = start(
        &f,
        intent(
            report.destination,
            report.operation,
            ConversionProfile::Native,
            "RustFS local",
            Some(destination.to_str().unwrap().into()),
        ),
        91,
    )
    .await;
    let ConversionOutcome::Committed { report: exported } = outcome else {
        panic!("local export")
    };
    let local = f.manager().open(exported.destination).unwrap();
    assert_eq!(local.thread_store().await.unwrap().state.threads.len(), 1);
    assert!(
        !local
            .reaction_query(
                &jj_collab::ReactionTargetV1::Thread {
                    thread_id: jj_collab::ThreadId::new([85; 16])
                },
                None,
                20,
                None
            )
            .await
            .unwrap()
            .rows
            .is_empty()
    );
    assert_eq!(
        local
            .agent_store()
            .await
            .unwrap()
            .reads()
            .list_sessions(None, 20)
            .await
            .unwrap()
            .selections
            .len(),
        1
    );
    assert_eq!(
        local
            .review_store()
            .await
            .unwrap()
            .reads()
            .list_threads(None, 20)
            .await
            .unwrap()
            .selections
            .len(),
        1
    );
    let archive = local.conversion_archive(exported.operation).await.unwrap();
    let titles: std::collections::BTreeSet<_> = archive
        .collaboration
        .iter()
        .filter_map(|v| v.work.as_ref())
        .flat_map(|v| v.projects.values().map(|p| p.title.as_str()))
        .collect();
    assert!(titles.contains("Original project"));
    assert!(titles.contains("Renamed project"));
    assert_eq!(
        f.manager()
            .open(source)
            .unwrap()
            .metadata_store()
            .read()
            .unwrap()
            .unwrap()
            .commit(),
        selected
    );
    assert_eq!(
        std::fs::read_to_string(path.join("dirty.txt")).unwrap(),
        "keep dirty"
    );
    assert_eq!(
        std::fs::read_to_string(destination.join("committed.txt")).unwrap(),
        "committed content"
    );
    assert!(!destination.join("dirty.txt").exists());
    assert!(
        stock(
            &destination,
            &["op", "log", "--no-graph", "-n", "1", "-T", "id"]
        )
        .contains(&exported.operation.to_string())
    );
    println!(
        "Cold export verified nonempty history and preserved source; starting production browser."
    );
    if std::env::var_os("JJ_STORE_CONVERSION_BROWSER").is_some() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let api = jj_store_http::HttpApi::new(
            f.host.clone(),
            f.root.path().join("browser-http"),
            jj_store_http::Config {
                origin: base.clone(),
                access: jj_store_http::Access::LocalTrusted {
                    principal: f.identity.principal(b"loopback-http"),
                    permission: Permission::Admin,
                },
                registry: Some(f.registry.clone()),
                local_repositories: Some(Arc::new(f.manager())),
                ..Default::default()
            },
        )
        .unwrap();
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let assets =
            jj_store_http::assets::BrowserAssets::load(&workspace.join("web/dist")).unwrap();
        let task = tokio::spawn(async move {
            axum::serve(listener, api.router().merge(assets.router()))
                .await
                .unwrap()
        });
        let output = tokio::process::Command::new("node")
            .current_dir(&workspace)
            .arg("web/tests/browser/repository-conversion-real.mjs")
            .arg(base)
            .arg(source.to_string())
            .arg(f.root.path().join("browser-export"))
            .arg(operation.to_string())
            .output()
            .await
            .unwrap();
        task.abort();
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        println!("{}", String::from_utf8_lossy(&output.stdout));
        assert_eq!(
            std::fs::read_to_string(path.join("dirty.txt")).unwrap(),
            "keep dirty"
        );
        assert_eq!(
            f.manager()
                .open(source)
                .unwrap()
                .metadata_store()
                .read()
                .unwrap()
                .unwrap()
                .commit(),
            selected
        );
    }
    second.shutdown().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn retained_report_digest_rejects_altered_identity_maps() {
    let f = Fixture::new();
    let (source, operation, _) = f.local().await;
    let (request, outcome) = start(
        &f,
        intent(
            source,
            operation,
            ConversionProfile::LocalJjGit,
            "Digest selected",
            None,
        ),
        92,
    )
    .await;
    assert!(matches!(outcome, ConversionOutcome::Committed { .. }));
    let path = f
        .state
        .join(format!("conversion-{}.prepared", request.request_id));
    let bytes = std::fs::read_to_string(&path).unwrap();
    let value: serde_json::Value = serde_json::from_str(&bytes).unwrap();
    let mapped = value["report"]["code_mappings"]["commits"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap()
        .as_str()
        .unwrap();
    let changed = bytes.replacen(
        &format!("\"{mapped}\""),
        &format!("\"{}\"", "aa".repeat(mapped.len() / 2)),
        1,
    );
    assert_ne!(changed, bytes);
    std::fs::write(path, changed).unwrap();
    assert_eq!(
        f.manager()
            .conversion_status(request.request_id)
            .await
            .unwrap_err(),
        Error::IdentityChanged
    );
}
