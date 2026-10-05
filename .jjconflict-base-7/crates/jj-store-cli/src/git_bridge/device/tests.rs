use super::*;
#[test]
fn approved_endpoints_reject_credentials_helpers_and_ssh_options() {
    let directory = tempfile::tempdir().unwrap();
    assert_eq!(
        approved_url(directory.path(), ".").unwrap(),
        directory.path().canonicalize().unwrap().to_str().unwrap()
    );
    assert!(approved_url(directory.path(), "https://example.invalid/code.git").is_ok());
    assert!(approved_url(directory.path(), "ssh://git@example.invalid/code.git").is_ok());
    for value in [
        "https://user:secret@example.invalid/code.git",
        "https://user@example.invalid/code.git",
        "ssh://-oProxyCommand@example.invalid/code.git",
        "ssh://user%20-oProxyCommand@example.invalid/code.git",
        "ext::command",
        "file:///tmp/repo",
        "git@example.invalid:code.git",
        "https://example.invalid/repo?secret=x",
        "--upload-pack=command",
        "https://example.invalid/repo\n",
    ] {
        assert!(approved_url(directory.path(), value).is_err(), "{value}");
    }
    let repository = m::RepositoryId::new([1; 16]);
    let remote = m::RequestId::new([2; 16]);
    let approval = m::RequestId::new([3; 16]);
    assert_ne!(
        fingerprint(repository, remote, approval, "a"),
        fingerprint(repository, remote, approval, "b")
    );
    assert_ne!(
        fingerprint(repository, remote, approval, "a"),
        fingerprint(repository, remote, m::RequestId::new([4; 16]), "a")
    );
}
#[cfg(unix)]
#[test]
fn private_state_rejects_symlinks_and_public_permissions() {
    use std::os::unix::fs::{PermissionsExt as _, symlink};
    let parent = tempfile::tempdir().unwrap();
    let path = parent.path().join("private");
    private(&path).unwrap();
    let link = parent.path().join("link");
    symlink(&path, &link).unwrap();
    assert!(private(&link).is_err());
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    assert!(private(&path).is_err());
}
