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
