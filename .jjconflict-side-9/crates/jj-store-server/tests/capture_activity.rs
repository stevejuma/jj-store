#![cfg(unix)]
use jj_store_capture::{
    codex_activity::{ActivityPoll, CodexActivityProbe},
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner},
    profile::CaptureProfile,
    source_registry::{QualifiedCodexSource, SourceRegistry},
};
use jj_store_model::RepositoryId;
use jj_store_server::{
    capture_admission::{CaptureAdmission, Error},
    installation::Identity,
    registry::{Catalog, Registry},
};
use jj_store_service::{
    local_management::{
        VerifiedCaptureWorkspace, VerifiedWorkspace, WorkspaceProfile, WorkspaceVerifier,
    },
    registry::RepositoryRegistry,
};
use serde_json::json;
use std::{fs, io::Write as _, sync::Arc};
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
        if directory != "/workspace/b/nested" {
            return Err(jj_store_service::local_management::Error::Unavailable);
        }
        Ok(VerifiedCaptureWorkspace {
            directory: directory.into(),
            workspace: VerifiedWorkspace {
                path: "/workspace/b".into(),
                metadata: "/workspace/b/.jj/repo".into(),
                repository: RepositoryId::new([2; 16]),
                profile: WorkspaceProfile::Native,
            },
            binding_digest: [2; 32],
        })
    }
}
#[tokio::test]
async fn unbound_activity_does_not_allocate_and_current_turn_directory_controls_admission() {
    use std::os::unix::fs::PermissionsExt as _;
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let identity = Identity {
        version: 1,
        installation: "ab".repeat(16),
    };
    let catalog_path = private.path().join("catalog.json");
    let catalog = Catalog::empty(&identity);
    fs::write(&catalog_path, serde_json::to_vec(&catalog).unwrap()).unwrap();
    fs::set_permissions(&catalog_path, fs::Permissions::from_mode(0o600)).unwrap();
    let registry = Arc::new(Registry::new(catalog_path.clone(), identity).unwrap());
    let admission = CaptureAdmission::new(registry.clone(), Arc::new(Native));
    let path = root.path().join("session.jsonl");
    let mut file = fs::File::create(&path).unwrap();
    writeln!(
        file,
        "{}",
        json!({"type":"session_meta","payload":{
            "id":"00000000-0000-0000-0000-000000000001","cwd":"/workspace/a",
            "cli_version":"0.156.1","source":"cli","originator":"codex_cli_rs"
        }})
    )
    .unwrap();
    let mut scanner =
        CodexDirectoryScanner::new(ApprovedCodexRoot::open(root.path()).unwrap().unwrap()).unwrap();
    let qualified = QualifiedCodexSource::inspect(
        CaptureProfile::CodexCliJsonlV1,
        scanner.step(100, 100).unwrap().candidates.remove(0),
    )
    .unwrap()
    .unwrap();
    let mut probe = CodexActivityProbe::arm(qualified).unwrap();
    assert_eq!(probe.poll().unwrap(), ActivityPoll::Waiting);
    writeln!(
        file,
        "{}",
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b/nested"}})
    )
    .unwrap();
    writeln!(
        file,
        "{}",
        json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"one"}})
    )
    .unwrap();
    assert_eq!(probe.poll().unwrap(), ActivityPoll::Active);
    let state = private.path().join("sources");
    let sources = Arc::new(SourceRegistry::open(&state, [0xab; 16], 4).unwrap());
    let before = fs::read(state.join("sources.json")).unwrap();
    let result = admission
        .acquire_active_source(sources.clone(), probe.proof().unwrap().unwrap())
        .await;
    assert!(matches!(result, Err(Error::Unbound)));
    assert_eq!(fs::read(state.join("sources.json")).unwrap(), before);
    assert_eq!(fs::read_dir(&state).unwrap().count(), 2);
    registry
        .register(RepositoryId::new([2; 16]), "second".into(), None)
        .await
        .unwrap();
    let (source, proof) = admission
        .acquire_active_source(sources.clone(), probe.proof().unwrap().unwrap())
        .await
        .unwrap();
    assert_eq!(proof.directory, "/workspace/b/nested");
    assert_eq!(proof.workspace.repository, RepositoryId::new([2; 16]));
    admission.revalidate(&proof).await.unwrap();
    drop(source);
    let accepted = fs::read(state.join("sources.json")).unwrap();
    fs::write(&catalog_path, serde_json::to_vec(&catalog).unwrap()).unwrap();
    assert!(matches!(
        admission
            .acquire_active_source(sources.clone(), probe.proof().unwrap().unwrap())
            .await,
        Err(Error::Unbound)
    ));
    assert_eq!(fs::read(state.join("sources.json")).unwrap(), accepted);
    // Source admission is retained evidence, not current repository membership.
    let mut scanner =
        CodexDirectoryScanner::new(ApprovedCodexRoot::open(root.path()).unwrap().unwrap()).unwrap();
    let qualified = QualifiedCodexSource::inspect(
        CaptureProfile::CodexCliJsonlV1,
        scanner.step(100, 100).unwrap().candidates.remove(0),
    )
    .unwrap()
    .unwrap();
    assert!(sources.previously_admitted(&qualified).unwrap());
}
