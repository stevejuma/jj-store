#![cfg(unix)]
use jj_store_server::{
    capture_authority::CaptureAuthority,
    config::{Access, Config, EXAMPLE},
    installation::Identity,
};
use jj_store_service::auth::Permission;
use std::{fs, os::unix::fs::PermissionsExt as _, path::Path};
fn config(root: &Path) -> Config {
    let path = root.join("server.toml");
    fs::write(&path, EXAMPLE.replace("[access]\nmode = \"authenticated\"\ninstallation = \"./installation.json\"\ncatalog = \"./repositories.json\"", "[access]\nmode = \"local-trusted\"\ninstallation = \"./installation.json\"\ncatalog = \"./repositories.json\"\nrpc_socket = \"./native.sock\"\npermission = \"write\"") + r#"
[agent_capture]
enabled = true
[[agent_capture.harnesses]]
profile = "codex-cli-jsonl-v1"
product_version = "0.156.1"
"#).unwrap();
    Config::load(&path).unwrap()
}
fn identity(root: &Path, byte: u8) -> Identity {
    let identity = Identity {
        version: 1,
        installation: format!("{byte:02x}").repeat(16),
    };
    let path = root.join("installation.json");
    fs::write(&path, serde_json::to_vec(&identity).unwrap()).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    identity
}
#[test]
fn capture_actor_is_stable_separate_and_disabled_preparation_reads_no_identity() {
    let root = tempfile::tempdir().unwrap();
    let mut config = config(root.path());
    config.agent_capture.enabled = false;
    assert!(CaptureAuthority::prepare(&config).unwrap().is_none());
    assert!(!config.state_dir.exists());
    assert!(!root.path().join("installation.json").exists());
    config.agent_capture.enabled = true;
    assert!(CaptureAuthority::prepare(&config).is_err());
    let installation = identity(root.path(), 0xab);
    let first = CaptureAuthority::prepare(&config)
        .unwrap()
        .unwrap()
        .principal();
    assert_eq!(first, installation.principal(b"agent-capture-worker"));
    for purpose in [
        b"browser".as_slice(),
        b"search-index",
        b"replication-worker",
        b"local-peer",
    ] {
        assert_ne!(first, installation.principal(purpose));
    }
    if let Access::LocalTrusted { permission, .. } = &mut config.access {
        *permission = Permission::Admin;
    }
    config.agent_capture.harnesses[0].roots = vec![root.path().join("other-transcripts")];
    assert_eq!(
        first,
        CaptureAuthority::prepare(&config)
            .unwrap()
            .unwrap()
            .principal()
    );
    identity(root.path(), 0xcd);
    assert_ne!(
        first,
        CaptureAuthority::prepare(&config)
            .unwrap()
            .unwrap()
            .principal()
    );
    if let Access::LocalTrusted { permission, .. } = &mut config.access {
        *permission = Permission::Read;
    }
    assert!(CaptureAuthority::prepare(&config).is_err());
    fs::set_permissions(
        root.path().join("installation.json"),
        fs::Permissions::from_mode(0o644),
    )
    .unwrap();
    if let Access::LocalTrusted { permission, .. } = &mut config.access {
        *permission = Permission::Write;
    }
    assert!(CaptureAuthority::prepare(&config).is_err());
    assert!(!config.state_dir.exists());
}

#[tokio::test]
async fn native_publisher_requires_current_admission_and_retains_receipt_actor_and_code_heads() {
    native_publisher_acceptance(false).await;
}

#[tokio::test]
async fn guarded_publisher_rechecks_directory_and_catalog_after_preparation_and_preserves_exact_replay()
 {
    native_publisher_acceptance(true).await;
}

async fn native_publisher_acceptance(guarded: bool) {
    use jj_store_capture::{
        agent_publication::AgentCapturePublisher,
        codex_transcript::{CodexTranscriptReader, TranscriptPoll},
        profile::CaptureProfile,
    };
    use jj_store_model::{EpochId, PublicationId, RepositoryId, RequestId};
    use jj_store_object::memory::MemoryStore;
    use jj_store_server::{
        capture_admission::CaptureAdmission,
        registry::{Catalog, Registry},
    };
    use jj_store_service::{
        administration::{OperatorPolicies, RepositoryHost},
        auth::{self, Scope, SystemClock, TokenPolicy},
        local_management::{
            VerifiedCaptureWorkspace, VerifiedWorkspace, WorkspaceProfile, WorkspaceVerifier,
        },
        native::SessionLimits,
        registry::RepositoryEntry,
        transport::{NormalizerProfileV1, PublishOutcome},
    };
    use serde_json::json;
    use std::sync::{
        Arc,
        atomic::{AtomicU8, Ordering},
    };
    struct Operators;
    #[async_trait::async_trait]
    impl OperatorPolicies for Operators {
        async fn current(&self) -> auth::Result<TokenPolicy> {
            Ok(TokenPolicy::new(Scope::Operator))
        }
    }
    struct Native(Arc<AtomicU8>);
    impl WorkspaceVerifier for Native {
        fn inspect(
            &self,
            _: &str,
        ) -> Result<VerifiedWorkspace, jj_store_service::local_management::Error> {
            Err(jj_store_service::local_management::Error::Unavailable)
        }
        fn inspect_capture_directory(
            &self,
            directory: &str,
        ) -> Result<VerifiedCaptureWorkspace, jj_store_service::local_management::Error> {
            Ok(VerifiedCaptureWorkspace {
                directory: directory.into(),
                workspace: VerifiedWorkspace {
                    path: "/workspace".into(),
                    metadata: "/workspace/.jj/repo".into(),
                    repository: RepositoryId::new([1; 16]),
                    profile: WorkspaceProfile::Native,
                },
                binding_digest: [self.0.load(Ordering::SeqCst); 32],
            })
        }
    }
    let root = tempfile::tempdir().unwrap();
    let config = config(root.path());
    let installation = identity(root.path(), 0xab);
    let authority = CaptureAuthority::prepare(&config).unwrap().unwrap();
    let repository = RepositoryId::new([1; 16]);
    let catalog_path = root.path().join("repositories.json");
    let mut catalog = Catalog::empty(&installation);
    catalog.repositories.push(RepositoryEntry {
        id: repository,
        name: "capture".into(),
    });
    fs::write(&catalog_path, serde_json::to_vec(&catalog).unwrap()).unwrap();
    fs::set_permissions(&catalog_path, fs::Permissions::from_mode(0o600)).unwrap();
    let registry = Arc::new(Registry::new(catalog_path.clone(), installation.clone()).unwrap());
    let binding_revision = Arc::new(AtomicU8::new(1));
    let admission = CaptureAdmission::new(registry, Arc::new(Native(binding_revision.clone())));
    let proof = admission.inspect("/workspace/subdir").await.unwrap();
    let store = Arc::new(MemoryStore::default());
    let host = RepositoryHost::new(
        store,
        Arc::new(Operators),
        Arc::new(SystemClock),
        Default::default(),
        SessionLimits::default(),
        1024 * 1024,
    )
    .unwrap();
    assert!(
        authority.bind(&host, &admission, &proof).await.is_err(),
        "registry membership does not establish native write authority"
    );
    host.create_local_trusted(repository, EpochId::new([2; 16]), Permission::Admin)
        .await
        .unwrap();
    let request = authority.bind(&host, &admission, &proof).await.unwrap();
    assert!(
        request.tokens().await.is_err(),
        "capture binds Write even when configuration permits Admin"
    );
    let guarded_publisher = authority.publisher(&host, &admission, &proof);
    let publisher: &dyn AgentCapturePublisher = if guarded {
        &guarded_publisher
    } else {
        &request
    };
    let heads = request.heads().await.unwrap();
    let transcript = root.path().join("fixture.jsonl");
    fs::write(
        &transcript,
        format!(
            "{}\n",
            json!({"type":"session_meta","payload":{
                "id":"00000000-0000-0000-0000-000000000001","cwd":"/workspace",
                "cli_version":"0.156.1","source":"cli","originator":"codex_cli_rs"
            }})
        ),
    )
    .unwrap();
    let mut reader = CodexTranscriptReader::open(
        CaptureProfile::CodexCliJsonlV1,
        &transcript,
        "11".repeat(16).parse().unwrap(),
        None,
    )
    .unwrap();
    let TranscriptPoll::Ready(batch) = reader.poll().unwrap() else {
        panic!("fixture")
    };
    let profile = NormalizerProfileV1::codex_0_156_1();
    let safe = batch.sanitize_omitted(repository, &profile).unwrap();
    let prepared = AgentCapturePublisher::prepare_agent_capture(
        publisher,
        RequestId::new([7; 16]),
        PublicationId::new([8; 16]),
        &profile,
        &safe.batch,
        b"capture-result".to_vec(),
    )
    .await
    .unwrap();
    assert_eq!(prepared.principal, authority.principal());
    if guarded {
        let mut foreign_actor = prepared.clone();
        foreign_actor.principal = jj_store_model::PrincipalId::new([99; 16]);
        assert!(publisher.publish_agent_delta(&foreign_actor).await.is_err());
        fs::write(
            &catalog_path,
            serde_json::to_vec(&Catalog::empty(&installation)).unwrap(),
        )
        .unwrap();
        assert!(publisher.publish_agent_delta(&prepared).await.is_err());
        fs::write(&catalog_path, serde_json::to_vec(&catalog).unwrap()).unwrap();
        binding_revision.store(2, Ordering::SeqCst);
        assert!(publisher.publish_agent_delta(&prepared).await.is_err());
        assert!(
            publisher
                .prepare_agent_capture(
                    RequestId::new([9; 16]),
                    PublicationId::new([10; 16]),
                    &profile,
                    &safe.batch,
                    vec![]
                )
                .await
                .is_err()
        );
        assert_eq!(request.heads().await.unwrap(), heads);
        binding_revision.store(1, Ordering::SeqCst);
    }
    let PublishOutcome::Committed(receipt) =
        AgentCapturePublisher::publish_agent_delta(publisher, &prepared)
            .await
            .unwrap()
    else {
        panic!("committed")
    };
    assert_eq!(receipt.principal, authority.principal());
    assert_eq!(request.heads().await.unwrap(), heads);
    let PublishOutcome::Committed(replayed) =
        AgentCapturePublisher::publish_agent_delta(publisher, &prepared)
            .await
            .unwrap()
    else {
        panic!("replay")
    };
    assert_eq!(receipt, replayed);
    let denied = host
        .endpoint(repository)
        .unwrap()
        .bind_local_trusted(authority.principal(), Permission::Read)
        .unwrap();
    assert!(
        AgentCapturePublisher::prepare_agent_capture(
            &denied,
            RequestId::new([9; 16]),
            PublicationId::new([10; 16]),
            &profile,
            &safe.batch,
            vec![]
        )
        .await
        .is_err()
    );
    assert!(
        AgentCapturePublisher::publish_agent_delta(&denied, &prepared)
            .await
            .is_err()
    );
    fs::write(
        &catalog_path,
        serde_json::to_vec(&Catalog::empty(&installation)).unwrap(),
    )
    .unwrap();
    assert!(authority.bind(&host, &admission, &proof).await.is_err());
    if guarded {
        assert!(publisher.publish_agent_delta(&prepared).await.is_err());
    }
}
