use jj_store_server::{
    capture_admission::{CaptureAdmission, Error},
    installation::Identity,
    registry::{Catalog, Registry},
};
use jj_store_service::local_management::{VerifiedWorkspace, WorkspaceProfile, WorkspaceVerifier};
use std::sync::Arc;
struct RootOnly;
impl WorkspaceVerifier for RootOnly {
    fn inspect(
        &self,
        path: &str,
    ) -> Result<VerifiedWorkspace, jj_store_service::local_management::Error> {
        Ok(VerifiedWorkspace {
            path: path.into(),
            metadata: "/workspace/.jj/repo".into(),
            repository: jj_store_model::RepositoryId::new([1; 16]),
            profile: WorkspaceProfile::Native,
        })
    }
}
#[tokio::test]
async fn root_only_verifiers_cannot_accidentally_admit_capture() {
    let root = tempfile::tempdir().unwrap();
    let identity = Identity {
        version: 1,
        installation: "ab".repeat(16),
    };
    let path = root.path().join("catalog.json");
    std::fs::write(
        &path,
        serde_json::to_vec(&Catalog::empty(&identity)).unwrap(),
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    }
    let registry = Arc::new(Registry::new(path, identity).unwrap());
    let admission = CaptureAdmission::new(registry, Arc::new(RootOnly));
    assert_eq!(
        admission.inspect("/workspace").await.unwrap_err(),
        Error::Unavailable
    );
    assert_eq!(
        admission.inspect(&"x".repeat(4097)).await.unwrap_err(),
        Error::Limit
    );
}

#[tokio::test]
async fn batch_admission_uses_reconstructed_context_and_current_registry_membership() {
    use jj_store_capture::{
        agent_capture::DurableCaptureAck,
        codex_transcript::{CodexTranscriptReader, TranscriptPoll},
        profile::CaptureProfile,
    };
    use jj_store_model::RepositoryId;
    use jj_store_service::{
        local_management::VerifiedCaptureWorkspace,
        registry::{RepositoryEntry, RepositoryRegistry},
    };
    use serde_json::json;
    use std::{fs, io::Write as _};
    struct Native;
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
            let (root, id) = match directory {
                "/workspace/a" => ("/workspace/a", 1),
                "/workspace/b/subdir" => ("/workspace/b", 2),
                _ => return Err(jj_store_service::local_management::Error::Unavailable),
            };
            Ok(VerifiedCaptureWorkspace {
                directory: directory.into(),
                workspace: VerifiedWorkspace {
                    path: root.into(),
                    metadata: format!("{root}/.jj/repo"),
                    repository: RepositoryId::new([id; 16]),
                    profile: WorkspaceProfile::Native,
                },
                binding_digest: [id; 32],
            })
        }
    }
    let root = tempfile::tempdir().unwrap();
    let identity = Identity {
        version: 1,
        installation: "ab".repeat(16),
    };
    let catalog_path = root.path().join("catalog.json");
    let mut catalog = Catalog::empty(&identity);
    catalog.repositories.push(RepositoryEntry {
        id: RepositoryId::new([1; 16]),
        name: "first".into(),
    });
    fs::write(&catalog_path, serde_json::to_vec(&catalog).unwrap()).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(&catalog_path, fs::Permissions::from_mode(0o600)).unwrap();
    }
    let registry = Arc::new(Registry::new(catalog_path.clone(), identity).unwrap());
    let admission = CaptureAdmission::new(registry.clone(), Arc::new(Native));
    let source_path = root.path().join("session.jsonl");
    let mut file = fs::File::create(&source_path).unwrap();
    for record in [
        json!({"type":"session_meta","payload":{"id":"00000000-0000-0000-0000-000000000001","cwd":"/workspace/a","cli_version":"0.156.1","source":"cli","originator":"codex_cli_rs"}}),
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b/subdir"}}),
        json!({"message":"second repository"}),
        json!({"type":"turn_context","payload":{"cwd":"relative-invalid"}}),
    ] {
        file.write_all(&serde_json::to_vec(&record).unwrap())
            .unwrap();
        file.write_all(b"\n").unwrap();
    }
    let mut reader = CodexTranscriptReader::open(
        CaptureProfile::CodexCliJsonlV1,
        &source_path,
        "07".repeat(16).parse().unwrap(),
        None,
    )
    .unwrap();
    let TranscriptPoll::Ready(first) = reader.poll().unwrap() else {
        panic!("header")
    };
    let proof = admission.inspect_batch(&first).await.unwrap();
    assert_eq!(proof.workspace.repository, RepositoryId::new([1; 16]));
    reader
        .acknowledge(&DurableCaptureAck {
            batch_id: first.batch_id,
            position: first.cursor.position,
        })
        .unwrap();
    let TranscriptPoll::Ready(second) = reader.poll().unwrap() else {
        panic!("second directory")
    };
    assert_eq!(
        admission.inspect_batch(&second).await.unwrap_err(),
        Error::Unbound
    );
    registry
        .register(RepositoryId::new([2; 16]), "second".into(), None)
        .await
        .unwrap();
    let proof = admission.inspect_batch(&second).await.unwrap();
    assert_eq!(proof.directory, "/workspace/b/subdir");
    assert_eq!(proof.workspace.repository, RepositoryId::new([2; 16]));
    admission.revalidate(&proof).await.unwrap();
    // Fixture removes discovery membership without changing the directory proof.
    fs::write(&catalog_path, serde_json::to_vec(&catalog).unwrap()).unwrap();
    assert_eq!(
        admission.revalidate(&proof).await.unwrap_err(),
        Error::Unbound
    );
    reader
        .acknowledge(&DurableCaptureAck {
            batch_id: second.batch_id,
            position: second.cursor.position,
        })
        .unwrap();
    let TranscriptPoll::Ready(gap) = reader.poll().unwrap() else {
        panic!("unknown directory")
    };
    assert_eq!(
        admission.inspect_batch(&gap).await.unwrap_err(),
        Error::Unbound
    );
}
