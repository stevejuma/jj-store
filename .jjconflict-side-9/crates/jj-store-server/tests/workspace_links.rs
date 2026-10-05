use jj_store_model::RepositoryId;
use jj_store_server::{installation::Identity, workspaces::Workspaces};
use jj_store_service::local_management::*;
use std::sync::{Arc, Mutex};

struct Verifier(Mutex<VerifiedWorkspace>);
impl WorkspaceVerifier for Verifier {
    fn inspect(&self, path: &str) -> Result<VerifiedWorkspace, Error> {
        let mut value = self.0.lock().unwrap().clone();
        value.path = path.into();
        Ok(value)
    }
}
fn fixture() -> (tempfile::TempDir, Arc<Verifier>, Workspaces) {
    let directory = tempfile::tempdir().unwrap();
    let verifier = Arc::new(Verifier(Mutex::new(VerifiedWorkspace {
        path: "/workspace".into(),
        metadata: "/workspace/.jj/repo".into(),
        repository: RepositoryId::new([1; 16]),
        profile: WorkspaceProfile::Native,
    })));
    let registry = Workspaces::new(
        directory.path().into(),
        Identity {
            version: 1,
            installation: "ab".repeat(16),
        },
        verifier.clone(),
    )
    .unwrap();
    (directory, verifier, registry)
}
fn request(path: &str, revision: u64) -> LinkWorkspace {
    LinkWorkspace {
        version: 1,
        path: path.into(),
        expected_revision: revision,
    }
}

#[tokio::test]
async fn links_are_read_only_until_explicitly_added_and_survive_restart() {
    let (directory, verifier, registry) = fixture();
    let repo = RepositoryId::new([1; 16]);
    assert_eq!(registry.list(repo).await.unwrap().revision, 0);
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
    let linked = registry.link(repo, request("/workspace", 0)).await.unwrap();
    assert_eq!(linked.revision, 1);
    assert_eq!(linked.links[0].availability, WorkspaceAvailability::Ready);
    let bytes = std::fs::read(directory.path().join("workspace-links.json")).unwrap();
    let retry = registry.link(repo, request("/workspace", 0)).await.unwrap();
    assert_eq!(retry.revision, 1);
    assert_eq!(retry.links[0].id, linked.links[0].id);
    drop(registry);
    let registry = Workspaces::new(
        directory.path().into(),
        Identity {
            version: 1,
            installation: "ab".repeat(16),
        },
        verifier.clone(),
    )
    .unwrap();
    assert_eq!(
        registry.list(repo).await.unwrap().links[0].id,
        linked.links[0].id
    );
    assert!(
        registry
            .list(RepositoryId::new([2; 16]))
            .await
            .unwrap()
            .links
            .is_empty()
    );
    verifier.0.lock().unwrap().metadata = "/rebound/.jj/repo".into();
    assert_eq!(
        registry.list(repo).await.unwrap().links[0].availability,
        WorkspaceAvailability::Unavailable
    );
    assert_eq!(
        registry
            .link(repo, request("/workspace", 1))
            .await
            .unwrap_err(),
        Error::Invalid
    );
    assert_eq!(
        std::fs::read(directory.path().join("workspace-links.json")).unwrap(),
        bytes
    );
}

#[tokio::test]
async fn revisions_repository_scope_and_limits_fail_closed() {
    let (_directory, _verifier, registry) = fixture();
    let repo = RepositoryId::new([1; 16]);
    assert_eq!(
        registry
            .link(RepositoryId::new([2; 16]), request("/workspace", 0))
            .await
            .unwrap_err(),
        Error::Invalid
    );
    assert_eq!(
        registry
            .link(repo, request("relative", 0))
            .await
            .unwrap_err(),
        Error::Invalid
    );
    assert_eq!(
        registry
            .link(
                repo,
                request(&format!("/{}", "x".repeat(MAX_PATH_BYTES)), 0)
            )
            .await
            .unwrap_err(),
        Error::Invalid
    );
    registry.link(repo, request("/workspace", 0)).await.unwrap();
    assert_eq!(
        registry
            .link(repo, request("/second", 0))
            .await
            .unwrap_err(),
        Error::StaleRevision
    );
    for n in 1..MAX_WORKSPACES {
        registry
            .link(repo, request(&format!("/workspace-{n}"), n as u64))
            .await
            .unwrap();
    }
    assert_eq!(
        registry
            .link(repo, request("/overflow", MAX_WORKSPACES as u64))
            .await
            .unwrap_err(),
        Error::Limit
    );
    assert_eq!(
        registry.list(repo).await.unwrap().links.len(),
        MAX_WORKSPACES
    );
}

#[tokio::test]
async fn foreign_corrupt_and_symlinked_registries_are_rejected() {
    let (directory, _verifier, registry) = fixture();
    let repo = RepositoryId::new([1; 16]);
    registry.link(repo, request("/workspace", 0)).await.unwrap();
    let path = directory.path().join("workspace-links.json");
    let bytes = std::fs::read(&path).unwrap();
    let mut document: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    document["installation"] = serde_json::json!("cd".repeat(16));
    std::fs::write(&path, serde_json::to_vec(&document).unwrap()).unwrap();
    assert_eq!(registry.list(repo).await.unwrap_err(), Error::Unavailable);
    std::fs::write(&path, b"not json").unwrap();
    assert_eq!(registry.list(repo).await.unwrap_err(), Error::Unavailable);
    #[cfg(unix)]
    {
        std::fs::remove_file(&path).unwrap();
        let target = directory.path().join("unchanged.json");
        std::fs::write(&target, &bytes).unwrap();
        std::os::unix::fs::symlink(&target, &path).unwrap();
        assert_eq!(registry.list(repo).await.unwrap_err(), Error::Unavailable);
        assert_eq!(
            registry.link(repo, request("/other", 1)).await.unwrap_err(),
            Error::Unavailable
        );
        assert_eq!(std::fs::read(target).unwrap(), bytes);
    }
}
