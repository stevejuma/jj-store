use jj_store_object::{Condition, Key, ObjectStore as _, StagedUpload};
use jj_store_server::storage::{EmbeddedStorageConfig, EmbeddedStorageRuntime};
use jj_store_server::{
    config::{Config, Storage},
    installation::initialize_local,
    process::PreparedServer,
    storage::AuthoritativeStorage,
};
use std::time::Duration;
use tokio::io::AsyncReadExt as _;

fn resident_kib(pid: u32) -> u64 {
    let output = std::process::Command::new("ps")
        .args(["-o", "rss=", "-p", &pid.to_string()])
        .output()
        .unwrap();
    String::from_utf8(output.stdout)
        .unwrap()
        .trim()
        .parse()
        .unwrap()
}

#[tokio::test]
#[ignore = "requires the separately built pinned embedded RustFS runtime"]
async fn embedded_rustfs_preserves_objects_across_graceful_restart() {
    let root = tempfile::tempdir().unwrap();
    std::fs::create_dir(root.path().join("volume")).unwrap();
    let config = EmbeddedStorageConfig {
        runtime_executable: std::fs::canonicalize(
            std::env::var_os("JJ_STORE_RUSTFS_RUNTIME")
                .expect("set JJ_STORE_RUSTFS_RUNTIME to the built embedded runtime"),
        )
        .unwrap(),
        volume: root.path().join("volume"),
        address: format!(
            "127.0.0.1:{}",
            std::net::TcpListener::bind("127.0.0.1:0")
                .unwrap()
                .local_addr()
                .unwrap()
                .port()
        ),
        access_key: "embedded-access-key-qualification".into(),
        secret_key: "embedded-secret-key-qualification".into(),
        region: "us-east-1".into(),
        bucket: "jj-store-embedded-test".into(),
        max_object_bytes: 1024 * 1024,
        timeout: Duration::from_secs(30),
    };
    let key = Key::new("qualification/persistent-object").unwrap();
    let upload = StagedUpload::from_bytes(b"persistent embedded authority", 1024)
        .await
        .unwrap();
    let debug = format!("{config:?}");
    assert!(!debug.contains(&config.access_key));
    assert!(!debug.contains(&config.secret_key));

    let binary_bytes = std::fs::metadata(&config.runtime_executable).unwrap().len();
    let started = std::time::Instant::now();
    let first = EmbeddedStorageRuntime::start(config.clone()).await.unwrap();
    let first_startup_ms = started.elapsed().as_millis();
    let first_rss_kib = resident_kib(first.process_id().unwrap());
    first.store().ensure_bucket().await.unwrap();
    assert!(matches!(
        first
            .store()
            .put(&key, Condition::Absent, &upload)
            .await
            .unwrap(),
        jj_store_object::WriteOutcome::Applied(_)
    ));
    first.shutdown().await;

    let started = std::time::Instant::now();
    let second = EmbeddedStorageRuntime::start(config).await.unwrap();
    let second_startup_ms = started.elapsed().as_millis();
    let second_rss_kib = resident_kib(second.process_id().unwrap());
    let mut download = second.store().get(&key, None).await.unwrap();
    let mut bytes = Vec::new();
    download.reader.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, b"persistent embedded authority");
    println!(
        "embedded_rustfs binary_bytes={binary_bytes} first_startup_ms={first_startup_ms} first_rss_kib={first_rss_kib} second_startup_ms={second_startup_ms} second_rss_kib={second_rss_kib}"
    );
    second.shutdown().await;
}

#[tokio::test]
#[ignore = "requires the separately built pinned embedded RustFS runtime"]
async fn managed_local_storage_locks_verifies_and_fails_closed() {
    let root = tempfile::tempdir().unwrap();
    let runtime = std::fs::canonicalize(
        std::env::var_os("JJ_STORE_RUSTFS_RUNTIME")
            .expect("set JJ_STORE_RUSTFS_RUNTIME to the built embedded runtime"),
    )
    .unwrap();
    let initialized = initialize_local(&root.path().join("state"), &runtime).unwrap();
    let config = Config::load(&initialized.config).unwrap();
    let key = Key::new("qualification/managed-persistent-object").unwrap();
    let upload = StagedUpload::from_bytes(b"managed embedded authority", 1024)
        .await
        .unwrap();

    let first = AuthoritativeStorage::start(&config).await.unwrap();
    let locked = AuthoritativeStorage::start(&config)
        .await
        .unwrap_err_or_display();
    assert!(locked.contains("lock"), "{locked}");
    assert!(matches!(
        first
            .store()
            .put(&key, Condition::Absent, &upload)
            .await
            .unwrap(),
        jj_store_object::WriteOutcome::Applied(_)
    ));
    first.shutdown().await.unwrap();
    let ownership =
        std::fs::read_to_string(initialized.state_dir.join("storage-owner.json")).unwrap();
    assert!(ownership.contains("\"sentinel_installed\":true"));

    let second = AuthoritativeStorage::start(&config).await.unwrap();
    assert_eq!(
        second
            .store()
            .get(&key, None)
            .await
            .unwrap()
            .read_bounded(1024)
            .await
            .unwrap(),
        b"managed embedded authority"
    );
    drop(second);
    tokio::time::sleep(Duration::from_millis(300)).await;
    let after_abrupt = AuthoritativeStorage::start(&config).await.unwrap();
    assert_eq!(
        after_abrupt
            .store()
            .get(&key, None)
            .await
            .unwrap()
            .read_bounded(1024)
            .await
            .unwrap(),
        b"managed embedded authority"
    );
    after_abrupt.shutdown().await.unwrap();

    let volume = initialized.state_dir.join("rustfs");
    let missing = initialized.state_dir.join("rustfs-missing");
    std::fs::rename(&volume, &missing).unwrap();
    assert!(
        AuthoritativeStorage::start(&config)
            .await
            .unwrap_err_or_display()
            .contains("managed storage identity, credentials, volume or ownership")
    );
    std::fs::rename(&missing, &volume).unwrap();

    let identity = initialized.state_dir.join("installation.json");
    let identity_bytes = std::fs::read(&identity).unwrap();
    std::fs::remove_file(&identity).unwrap();
    assert!(
        AuthoritativeStorage::start(&config)
            .await
            .unwrap_err_or_display()
            .contains("managed storage installation identity is missing")
    );
    std::fs::write(&identity, identity_bytes).unwrap();
    set_private(&identity);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let credentials = initialized.state_dir.join("rustfs-credentials.json");
        std::fs::set_permissions(&credentials, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(
            AuthoritativeStorage::start(&config)
                .await
                .unwrap_err_or_display()
                .contains("local installation file permissions")
        );
        std::fs::set_permissions(&credentials, std::fs::Permissions::from_mode(0o600)).unwrap();
    }

    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let mut blocked = config.clone();
    let Storage::EmbeddedRustfs { address, .. } = &mut blocked.storage else {
        panic!("local init must select managed RustFS")
    };
    *address = listener.local_addr().unwrap().to_string();
    let conflict = tokio::time::timeout(
        Duration::from_secs(10),
        AuthoritativeStorage::start(&blocked),
    )
    .await;
    assert!(
        matches!(conflict, Ok(Err(error)) if error.to_string().contains("embedded storage startup failed"))
    );

    let mut served = config.clone();
    served.bind = "127.0.0.1:0".parse().unwrap();
    let server = PreparedServer::prepare(&served, true).await.unwrap();
    assert!(server.address().unwrap().ip().is_loopback());
    server.run(async {}).await.unwrap();
    let after_server = AuthoritativeStorage::start(&config).await.unwrap();
    after_server.shutdown().await.unwrap();

    let absent = initialize_local(&root.path().join("absent"), &runtime).unwrap();
    let owner = absent.state_dir.join("storage-owner.json");
    let mut value: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&owner).unwrap()).unwrap();
    value["sentinel_installed"] = serde_json::Value::Bool(true);
    std::fs::write(&owner, serde_json::to_vec(&value).unwrap()).unwrap();
    set_private(&owner);
    let absent = Config::load(&absent.config).unwrap();
    assert!(
        AuthoritativeStorage::start(&absent)
            .await
            .unwrap_err_or_display()
            .contains("managed storage bucket is missing")
    );
}

trait ResultErrorDisplay<T> {
    fn unwrap_err_or_display(self) -> String;
}

impl<T> ResultErrorDisplay<T> for jj_store_server::Result<T> {
    fn unwrap_err_or_display(self) -> String {
        match self {
            Ok(_) => panic!("expected managed storage startup to fail"),
            Err(error) => error.to_string(),
        }
    }
}

fn set_private(path: &std::path::Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).unwrap();
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires the separately built pinned embedded RustFS runtime"]
async fn injectable_native_stores_replay_exact_receipts_after_cold_rustfs_restart() {
    use jj_store_jj::{objects::NativeObjects, policy::Protection};
    use jj_store_model as m;
    use jj_store_service::{
        auth::{Permission, SystemClock},
        collab_mutations as collab,
        native::{NativeSession, SessionLimits, create_repository_local_trusted},
        review_model as r, review_mutations as review, work,
    };
    use std::{collections::BTreeSet, sync::Arc};
    const REPO: m::RepositoryId = m::RepositoryId::new([71; 16]);
    const ACTOR: m::PrincipalId = m::PrincipalId::new([72; 16]);
    let root = tempfile::tempdir().unwrap();
    std::fs::create_dir(root.path().join("volume")).unwrap();
    let config = EmbeddedStorageConfig {
        runtime_executable: std::fs::canonicalize(
            std::env::var_os("JJ_STORE_RUSTFS_RUNTIME").expect("set JJ_STORE_RUSTFS_RUNTIME"),
        )
        .unwrap(),
        volume: root.path().join("volume"),
        address: format!(
            "127.0.0.1:{}",
            std::net::TcpListener::bind("127.0.0.1:0")
                .unwrap()
                .local_addr()
                .unwrap()
                .port()
        ),
        access_key: "injectable-qualification-access".into(),
        secret_key: "injectable-qualification-secret".into(),
        region: "us-east-1".into(),
        bucket: "injectable-native-qualification".into(),
        max_object_bytes: 8 * 1024 * 1024,
        timeout: Duration::from_secs(30),
    };
    let first = EmbeddedStorageRuntime::start(config.clone()).await.unwrap();
    first.store().ensure_bucket().await.unwrap();
    let objects = Arc::new(NativeObjects::new(first.store(), REPO, 8 * 1024 * 1024).unwrap());
    create_repository_local_trusted(
        first.store(),
        objects.clone(),
        Permission::Admin,
        m::EpochId::new([73; 16]),
        SessionLimits::default().publication,
    )
    .await
    .unwrap();
    let session = Arc::new(
        NativeSession::new_local_trusted(
            first.store(),
            objects.clone(),
            ACTOR,
            Permission::Admin,
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    let work_request = work::Request {
        version: 1,
        request_id: m::RequestId::new([74; 16]),
        command: work::WorkCommandV1::CreateProject {
            id: work::ProjectId::new([74; 16]),
            title: "Cold library home".into(),
            body: String::new(),
        },
    };
    let work_receipt = work::WorkCommandService
        .apply(session.clone(), &work_request)
        .await
        .unwrap();
    let thread_request = collab::Request {
        version: 1,
        request_id: m::RequestId::new([75; 16]),
        command: jj_collab::CollabCommandV1::Thread {
            intent: jj_collab::ThreadCommandV1::Create {
                thread_id: jj_collab::ThreadId::new([75; 16]),
                title: "Cold library thread".into(),
                baseline: None,
            },
        },
    };
    let thread_receipt = collab::CollabCommandService
        .apply(session.clone(), &thread_request)
        .await
        .unwrap();
    let collab::Outcome::Committed { receipt: thread } = &thread_receipt else {
        panic!("committed thread")
    };
    let jj_collab::CollabCommandResultV1::Thread { result: thread } = &thread.result else {
        panic!("thread result")
    };
    let observations: Vec<serde_json::Value> =
        serde_json::from_slice(include_bytes!("fixtures/injectable-codex-hooks.json")).unwrap();
    let batch = jj_agent::SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: jj_agent::CaptureBatchId::new([76; 16]),
        integration: jj_agent::IntegrationId::Codex,
        source_instance_id: jj_agent::SourceInstanceId::new([76; 16]),
        sanitizer_version: "agent-redaction@0.1.0".into(),
        representation: jj_agent::SanitizedRepresentation::SanitizedNative,
        observations: observations
            .into_iter()
            .enumerate()
            .map(|(i, value)| jj_agent::SanitizedObservationV1 {
                observation_id: jj_agent::ObservationId::new([i as u8 + 1; 16]),
                source_sequence: i as u64 + 1,
                occurred_at_millis: Some(1725000000000 + i as i64),
                source_kind: jj_agent::SourceKind::Hook,
                source_trust: jj_agent::SourceTrust::LocalUserHook,
                payload: jj_agent::SanitizedPayloadV1::new(
                    jj_agent::SanitizedMediaType::Json,
                    &serde_json::to_vec(&value).unwrap(),
                )
                .unwrap(),
            })
            .collect(),
    };
    let prepared = session
        .prepare_agent_capture_with_binding(
            m::RequestId::new([77; 16]),
            m::PublicationId::new([78; 16]),
            &jj_agent::NormalizerProfileV1::codex_0_156_1(),
            &batch,
            b"opaque retained result".to_vec(),
            Some(&jj_collab::ThreadBindingV1 {
                thread_id: thread.thread_id,
                observed_thread: thread.version,
                observed_assignment: None,
            }),
        )
        .await
        .unwrap();
    let retained_preparation = serde_json::to_vec(&prepared).unwrap();
    let agent_receipt = session.publish_agent_delta(&prepared).await.unwrap();
    let review_request = review::Request {
        version: 1,
        request_id: m::RequestId::new([79; 16]),
        command: r::ReviewCommandV1::CreateThread {
            anchor: Box::new(r::ArtifactAnchorV1 {
                schema_version: 1,
                repository_id: REPO,
                target: r::ArtifactTargetV1::Repository,
                created_context: r::CreatedContextV1 {
                    publication_position: None,
                    code_state_version: None,
                    agent_root: None,
                },
                sensitivity: r::SensitivityV1::Repository,
            }),
            body: r::ReviewCommentBodyV1 {
                schema_version: 1,
                format: r::ReviewBodyFormatV1::PlainText,
                text: "Cold review".into(),
                sanitizer_version: "plain@1".into(),
            },
        },
    };
    let review_receipt = review::ReviewCommandService
        .apply(session.clone(), &review_request)
        .await
        .unwrap();
    let reaction_request = collab::Request {
        version: 1,
        request_id: m::RequestId::new([80; 16]),
        command: jj_collab::CollabCommandV1::Reaction {
            intent: jj_collab::ReactionCommandV1::Set {
                emoji: None,
                target: jj_collab::ReactionTargetV1::Thread {
                    thread_id: thread.thread_id,
                },
                value: jj_collab::ReactionValueV1::Like,
                observed: BTreeSet::new(),
            },
        },
    };
    let reaction_receipt = collab::CollabCommandService
        .apply(session.clone(), &reaction_request)
        .await
        .unwrap();
    let selected = session.snapshot().await.unwrap().manifest().clone();
    assert_eq!(selected.agent_state_version, 1);
    assert_eq!(selected.thread_state_version, 2);
    assert_eq!(selected.work_state_version, 1);
    assert_eq!(selected.review_state_version, 1);
    assert_eq!(selected.reaction_state_version, 1);
    drop(session);
    drop(objects);
    first.shutdown().await;
    let second = EmbeddedStorageRuntime::start(config).await.unwrap();
    let objects = Arc::new(NativeObjects::new(second.store(), REPO, 8 * 1024 * 1024).unwrap());
    let cold = Arc::new(
        NativeSession::new_local_trusted(
            second.store(),
            objects,
            ACTOR,
            Permission::Admin,
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    assert_eq!(cold.snapshot().await.unwrap().manifest(), &selected);
    assert_eq!(
        work::WorkCommandService
            .apply(cold.clone(), &work_request)
            .await
            .unwrap(),
        work_receipt
    );
    assert_eq!(
        review::ReviewCommandService
            .apply(cold.clone(), &review_request)
            .await
            .unwrap(),
        review_receipt
    );
    assert_eq!(
        collab::CollabCommandService
            .apply(cold.clone(), &thread_request)
            .await
            .unwrap(),
        thread_receipt
    );
    assert_eq!(
        collab::CollabCommandService
            .apply(cold.clone(), &reaction_request)
            .await
            .unwrap(),
        reaction_receipt
    );
    let retained = serde_json::from_slice(&retained_preparation).unwrap();
    assert_eq!(
        cold.publish_agent_delta(&retained).await.unwrap(),
        agent_receipt
    );
    assert_eq!(cold.snapshot().await.unwrap().manifest(), &selected);
    let stores = cold.collaboration_stores().await.unwrap();
    assert_eq!(stores.agent().unwrap().snapshot().await.unwrap().version, 1);
    assert_eq!(stores.work().unwrap().snapshot().await.unwrap().version, 1);
    assert_eq!(
        stores.review().unwrap().snapshot().await.unwrap().version,
        1
    );
    assert_eq!(
        stores.thread().unwrap().snapshot().await.unwrap().version,
        2
    );
    assert_eq!(
        stores.reaction().unwrap().snapshot().await.unwrap().version,
        1
    );
    second.shutdown().await;
}
