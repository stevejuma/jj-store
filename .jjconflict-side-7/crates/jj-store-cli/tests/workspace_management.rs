use jj_store_cli::{binding::Connection, workspace_management::NativeWorkspaceVerifier};
use jj_store_model::RepositoryId;
use jj_store_service::local_management::{Error, WorkspaceProfile, WorkspaceVerifier};
use std::{fs, path::Path};

fn native(root: &Path, socket: &Path) {
    let repo = root.join(".jj/repo");
    fs::create_dir_all(root.join(".jj/working_copy")).unwrap();
    for component in ["store", "op_store", "op_heads"] {
        fs::create_dir_all(repo.join(component)).unwrap();
        fs::write(
            repo.join(component).join("native-profile"),
            format!(
                "jj-store-client-cache-v1\njj-native-0.45.1-v1\n{}\n",
                RepositoryId::new([1; 16])
            ),
        )
        .unwrap();
    }
    fs::write(repo.join("store/type"), "jj-store").unwrap();
    Connection::new_local(socket.to_owned(), RepositoryId::new([1; 16]))
        .unwrap()
        .write(&repo)
        .unwrap();
}

#[test]
fn discovery_is_nonmutating_and_binds_the_exact_installation() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("workspace");
    let socket = directory.path().join("native.sock");
    native(&root, &socket);
    let verifier = NativeWorkspaceVerifier::new(&socket).unwrap();
    let proof = verifier.inspect(root.to_str().unwrap()).unwrap();
    assert_eq!(proof.path, root.canonicalize().unwrap().to_str().unwrap());
    assert_eq!(proof.profile, WorkspaceProfile::Native);
    assert_eq!(proof.repository, RepositoryId::new([1; 16]));
    assert!(!socket.exists());
    assert!(!root.join(".jj/repo/op_heads/heads").exists());
    let other = NativeWorkspaceVerifier::new(&directory.path().join("other.sock")).unwrap();
    assert_eq!(
        other.inspect(root.to_str().unwrap()).unwrap_err(),
        Error::Invalid
    );
    assert_eq!(
        verifier
            .inspect(root.join(".jj").to_str().unwrap())
            .unwrap_err(),
        Error::Invalid
    );
    let linked = directory.path().join("linked");
    fs::create_dir_all(linked.join(".jj/working_copy")).unwrap();
    fs::write(linked.join(".jj/repo"), proof.metadata.as_bytes()).unwrap();
    assert_eq!(
        verifier.inspect(linked.to_str().unwrap()).unwrap().metadata,
        proof.metadata
    );
}

#[test]
fn unqualified_git_corrupt_and_symlink_metadata_fail_closed() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("workspace");
    let socket = directory.path().join("native.sock");
    native(&root, &socket);
    let verifier = NativeWorkspaceVerifier::new(&socket).unwrap();
    fs::write(root.join(".jj/repo/store/type"), "git").unwrap();
    assert_eq!(
        verifier.inspect(root.to_str().unwrap()).unwrap_err(),
        Error::UnsupportedProfile
    );
    assert!(!root.join(".jj/repo/store/lapis").exists());
    fs::write(root.join(".jj/repo/store/type"), "jj-store").unwrap();
    fs::write(
        root.join(".jj/repo/op_store/native-profile"),
        "another repository",
    )
    .unwrap();
    assert_eq!(
        verifier.inspect(root.to_str().unwrap()).unwrap_err(),
        Error::Invalid
    );
    #[cfg(unix)]
    {
        let linked = directory.path().join("linked");
        fs::create_dir(&linked).unwrap();
        std::os::unix::fs::symlink(root.join(".jj"), linked.join(".jj")).unwrap();
        assert_eq!(
            verifier.inspect(linked.to_str().unwrap()).unwrap_err(),
            Error::Invalid
        );
    }
}

#[test]
fn capture_discovery_resolves_nested_and_linked_workspaces_without_mutation() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("workspace");
    let socket = directory.path().join("native.sock");
    native(&root, &socket);
    let nested = root.join("src/nested");
    fs::create_dir_all(&nested).unwrap();
    fs::write(nested.join("dirty.rs"), "preexisting dirty content").unwrap();
    let verifier = NativeWorkspaceVerifier::new(&socket).unwrap();
    let proof = verifier
        .inspect_capture_directory(nested.to_str().unwrap())
        .unwrap();
    assert_eq!(
        proof.workspace.path,
        root.canonicalize().unwrap().to_str().unwrap()
    );
    let root_proof = verifier
        .inspect_capture_directory(root.to_str().unwrap())
        .unwrap();
    assert_eq!(proof.workspace, root_proof.workspace);
    assert_eq!(proof.binding_digest, root_proof.binding_digest);
    assert_eq!(proof.directory, nested.to_str().unwrap());
    fs::write(nested.join("dirty.rs"), "model changed this file").unwrap();
    assert_eq!(
        proof,
        verifier
            .inspect_capture_directory(nested.to_str().unwrap())
            .unwrap()
    );
    let linked = directory.path().join("linked");
    fs::create_dir_all(linked.join(".jj/working_copy")).unwrap();
    fs::create_dir_all(linked.join("src")).unwrap();
    fs::write(linked.join(".jj/repo"), proof.workspace.metadata.as_bytes()).unwrap();
    let linked_proof = verifier
        .inspect_capture_directory(linked.join("src").to_str().unwrap())
        .unwrap();
    assert_eq!(
        linked_proof.workspace.repository,
        proof.workspace.repository
    );
    assert_eq!(linked_proof.workspace.metadata, proof.workspace.metadata);
    assert_ne!(linked_proof.binding_digest, proof.binding_digest);
    assert!(!socket.exists());
    assert!(!root.join(".jj/repo/op_heads/heads").exists());
    assert_eq!(
        fs::read(nested.join("dirty.rs")).unwrap(),
        b"model changed this file"
    );
    let foreign = NativeWorkspaceVerifier::new(&directory.path().join("other.sock")).unwrap();
    assert!(
        foreign
            .inspect_capture_directory(nested.to_str().unwrap())
            .is_err()
    );
}

#[test]
fn capture_discovery_stops_at_inner_boundaries_metadata_and_depth_limit() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("workspace");
    let socket = directory.path().join("native.sock");
    native(&root, &socket);
    let verifier = NativeWorkspaceVerifier::new(&socket).unwrap();
    let vendor = root.join("vendor/another-repo");
    fs::create_dir_all(vendor.join("src")).unwrap();
    fs::write(vendor.join(".git"), "gitdir: other").unwrap();
    assert_eq!(
        verifier
            .inspect_capture_directory(vendor.join("src").to_str().unwrap())
            .unwrap_err(),
        Error::UnsupportedProfile
    );
    fs::create_dir(vendor.join(".jj")).unwrap();
    assert_eq!(
        verifier
            .inspect_capture_directory(vendor.join("src").to_str().unwrap())
            .unwrap_err(),
        Error::Invalid
    );
    assert_eq!(
        verifier
            .inspect_capture_directory(root.join(".jj/working_copy").to_str().unwrap())
            .unwrap_err(),
        Error::Invalid
    );
    let deep = root.join("nested/".repeat(70));
    fs::create_dir_all(&deep).unwrap();
    assert_eq!(
        verifier
            .inspect_capture_directory(deep.to_str().unwrap())
            .unwrap_err(),
        Error::Limit
    );
    let absent = directory.path().join("absent");
    assert!(
        verifier
            .inspect_capture_directory(absent.to_str().unwrap())
            .is_err()
    );
    assert!(!absent.exists());
    assert!(!root.join(".jj/repo/op_heads/heads").exists());
}

#[test]
fn capture_binding_changes_on_connection_or_physical_replacement() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("workspace");
    let socket = directory.path().join("native.sock");
    native(&root, &socket);
    let verifier = NativeWorkspaceVerifier::new(&socket).unwrap();
    let before = verifier
        .inspect_capture_directory(root.to_str().unwrap())
        .unwrap();
    let mut connection = Connection::read(&root.join(".jj/repo")).unwrap();
    connection.timeout_seconds += 1;
    fs::write(
        root.join(".jj/repo/store-connection.json"),
        serde_json::to_vec(&connection).unwrap(),
    )
    .unwrap();
    let changed = verifier
        .inspect_capture_directory(root.to_str().unwrap())
        .unwrap();
    assert_eq!(before.workspace, changed.workspace);
    assert_ne!(before.binding_digest, changed.binding_digest);
    fs::rename(&root, directory.path().join("retired")).unwrap();
    native(&root, &socket);
    let replacement = verifier
        .inspect_capture_directory(root.to_str().unwrap())
        .unwrap();
    assert_eq!(before.workspace, replacement.workspace);
    assert_ne!(before.binding_digest, replacement.binding_digest);
}

#[tokio::test]
async fn server_capture_admits_only_published_registry_bindings_and_revalidates_them() {
    use jj_store_server::{
        capture_admission::{CaptureAdmission, Error as AdmissionError},
        installation::Identity,
        registry::{Catalog, Registry},
    };
    use jj_store_service::registry::RepositoryRegistry as _;
    use std::sync::Arc;
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("workspace");
    let socket = directory.path().join("native.sock");
    native(&root, &socket);
    fs::create_dir(root.join("src")).unwrap();
    let repository = RepositoryId::new([1; 16]);
    let identity = Identity {
        version: 1,
        installation: "ab".repeat(16),
    };
    let catalog_path = directory.path().join("repositories.json");
    fs::write(
        &catalog_path,
        serde_json::to_vec(&Catalog::empty(&identity)).unwrap(),
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(&catalog_path, fs::Permissions::from_mode(0o600)).unwrap();
    }
    let registry = Arc::new(Registry::new(catalog_path.clone(), identity.clone()).unwrap());
    let admission = CaptureAdmission::new(
        registry.clone(),
        Arc::new(NativeWorkspaceVerifier::new(&socket).unwrap()),
    );
    let path = root.join("src");
    assert_eq!(
        admission.inspect(path.to_str().unwrap()).await.unwrap_err(),
        AdmissionError::Unbound
    );
    registry
        .reserve(repository, "code/project".into())
        .await
        .unwrap();
    assert_eq!(
        admission.inspect(path.to_str().unwrap()).await.unwrap_err(),
        AdmissionError::Unbound,
        "pending reservation is not published admission"
    );
    registry
        .register(repository, "code/project".into(), None)
        .await
        .unwrap();
    let proof = admission.inspect(path.to_str().unwrap()).await.unwrap();
    admission.revalidate(&proof).await.unwrap();
    let linked = directory.path().join("linked");
    fs::create_dir_all(linked.join(".jj/working_copy")).unwrap();
    fs::write(linked.join(".jj/repo"), proof.workspace.metadata.as_bytes()).unwrap();
    assert_eq!(
        admission
            .inspect(linked.to_str().unwrap())
            .await
            .unwrap()
            .workspace
            .repository,
        repository
    );
    assert!(
        !directory.path().join("workspace-links.json").exists(),
        "a separately registered physical link is unnecessary"
    );
    let mut connection = Connection::read(&root.join(".jj/repo")).unwrap();
    connection.timeout_seconds += 1;
    fs::write(
        root.join(".jj/repo/store-connection.json"),
        serde_json::to_vec(&connection).unwrap(),
    )
    .unwrap();
    assert_eq!(
        admission.revalidate(&proof).await.unwrap_err(),
        AdmissionError::Changed
    );
    let current = admission.inspect(path.to_str().unwrap()).await.unwrap();
    #[cfg(unix)]
    {
        let alias = directory.path().join("session-directory");
        std::os::unix::fs::symlink(&path, &alias).unwrap();
        let alias_proof = admission.inspect(alias.to_str().unwrap()).await.unwrap();
        fs::remove_file(&alias).unwrap();
        std::os::unix::fs::symlink(&linked, &alias).unwrap();
        assert_eq!(
            admission.revalidate(&alias_proof).await.unwrap_err(),
            AdmissionError::Changed,
            "the original session directory alias must be re-resolved"
        );
    }
    fs::write(path.join(".git"), "gitdir: newly-created-inner-repo").unwrap();
    assert_eq!(
        admission.revalidate(&current).await.unwrap_err(),
        AdmissionError::Unbound,
        "revalidation must repeat discovery from the original session directory"
    );
    fs::remove_file(path.join(".git")).unwrap();
    fs::write(
        &catalog_path,
        serde_json::to_vec(&Catalog::empty(&identity)).unwrap(),
    )
    .unwrap();
    assert_eq!(
        admission.revalidate(&current).await.unwrap_err(),
        AdmissionError::Unbound
    );
    assert!(!socket.exists());
    assert!(!root.join(".jj/repo/op_heads/heads").exists());
}
