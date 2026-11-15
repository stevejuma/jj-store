#![cfg(feature = "vault-backup")]
use jj_store_jj::vault_workspace::RemoteTarget;

#[test]
fn admits_host_managed_ssh_endpoints() {
    for url in [
        "ssh://git@example.test/team/notes.git",
        "git@example.test:team/notes.git",
    ] {
        assert!(
            RemoteTarget {
                url: url.into(),
                branch: "main".into()
            }
            .validate()
            .is_ok(),
            "{url}"
        );
    }
}

#[test]
fn rejects_embedded_secrets_and_executable_transports() {
    for url in [
        "https://user:secret@example.test/repo",
        "ext::sh -c command",
        "file:///private/repo",
        "ssh://git:secret@example.test/repo",
        "ssh://git@example.test/repo?token=secret",
        "ssh://git@-option/repo",
        "git@example.test:-option",
    ] {
        assert!(
            RemoteTarget {
                url: url.into(),
                branch: "main".into()
            }
            .validate()
            .is_err()
        );
    }
}

#[tokio::test]
async fn authentication_protocol_mismatch_refuses_before_initializing_storage() {
    use jj_store_jj::vault_workspace::{Binding, GitCredentials, HostSshIdentity, RemoteTransfer};
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("vault");
    std::fs::create_dir(&root).unwrap();
    let binding = Binding {
        vault_id: "fixture".into(),
        root,
        state: temp.path().join("private"),
    };
    let https = RemoteTarget {
        url: "https://example.test/team/notes.git".into(),
        branch: "main".into(),
    };
    let ssh = RemoteTarget {
        url: "ssh://git@example.test/team/notes.git".into(),
        branch: "main".into(),
    };
    assert!(
        RemoteTransfer::connect(
            binding.clone(),
            ssh.clone(),
            GitCredentials::token("synthetic").unwrap()
        )
        .await
        .is_err()
    );
    assert!(
        RemoteTransfer::connect_anonymous(binding.clone(), ssh)
            .await
            .is_err()
    );
    assert!(
        RemoteTransfer::connect_ssh(binding.clone(), https, HostSshIdentity::Agent)
            .await
            .is_err()
    );
    assert!(
        !binding.state.exists(),
        "protocol rejection must not initialize backup storage"
    );
}
