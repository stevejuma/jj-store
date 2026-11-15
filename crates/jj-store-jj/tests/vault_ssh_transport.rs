#![cfg(feature = "vault-backup")]
use jj_store_jj::vault_workspace::{
    Binding, HostSshIdentity, PushOutcome, RemoteTarget, RemoteTransfer, Session,
};
use std::{fs, path::Path, process::Command};

fn binding(base: &Path, name: &str) -> Binding {
    let root = base.join(name);
    fs::create_dir(&root).unwrap();
    Binding {
        vault_id: name.into(),
        root,
        state: base.join("private").join(name),
    }
}
fn git(args: &[&str]) -> String {
    let result = Command::new("git")
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(result.status.success(), "fixture Git command failed");
    String::from_utf8(result.stdout).unwrap().trim().into()
}

#[tokio::test]
#[ignore = "requires the disposable sshd/agent runner: node scripts/test-vault-ssh.mjs"]
async fn real_ssh_preserves_files_rejects_host_key_changes_and_recovers_lost_push_responses() {
    let base = std::path::PathBuf::from(
        std::env::var_os("JJ_STORE_SSH_FIXTURE").expect("use the owned SSH fixture runner"),
    )
    .canonicalize()
    .unwrap();
    let target = RemoteTarget {
        url: std::env::var("JJ_STORE_SSH_ENDPOINT").unwrap(),
        branch: "main".into(),
    };
    let origin = base.join("remote.git");
    let a = binding(&base, "vault-a");
    fs::write(a.root.join("Note.md"), "saved first").unwrap();
    let mut session = Session::open(a.clone()).await.unwrap();
    let first = session
        .capture(&[], "First saved checkpoint")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    drop(session);
    let remote = RemoteTransfer::connect_ssh(a.clone(), target.clone(), HostSshIdentity::Agent)
        .await
        .unwrap()
        .with_known_hosts(&base.join("known 'hosts'"))
        .unwrap();
    assert_eq!(remote.observe().await.unwrap(), None);
    let session = Session::open(a.clone()).await.unwrap();
    let intent = session
        .prepare_push(&remote, "ssh-first", None)
        .await
        .unwrap();
    drop(session);
    assert_eq!(
        remote.publish(&intent).await.unwrap(),
        PushOutcome::Complete
    );
    assert_eq!(
        remote.observe().await.unwrap().as_deref(),
        Some(first.commit.as_str())
    );
    git(&[
        "--git-dir",
        origin.to_str().unwrap(),
        "update-ref",
        "refs/heads/unrelated",
        &first.commit,
    ]);

    let b = binding(&base, "vault-b");
    let mut session = Session::open(b.clone()).await.unwrap();
    session.capture(&[], "Empty receiver").await.unwrap();
    drop(session);
    let receiver = RemoteTransfer::connect_ssh(b.clone(), target.clone(), HostSshIdentity::Agent)
        .await
        .unwrap()
        .with_known_hosts(&base.join("known 'hosts'"))
        .unwrap();
    let fetched = receiver.fetch().await.unwrap().unwrap();
    let mut session = Session::open(b.clone()).await.unwrap();
    assert_eq!(
        session.import_fetched(&fetched).await.unwrap().commit,
        first.commit
    );
    assert!(
        !b.root.join("Note.md").exists(),
        "SSH fetch/import cannot write working files"
    );
    drop(session);
    let untrusted = RemoteTransfer::connect_ssh(b.clone(), target, HostSshIdentity::Agent)
        .await
        .unwrap()
        .with_known_hosts(&base.join("wrong-known-hosts"))
        .unwrap();
    assert!(
        untrusted.observe().await.is_err(),
        "changed host key must be rejected"
    );

    fs::write(a.root.join("Note.md"), "saved second").unwrap();
    let mut session = Session::open(a.clone()).await.unwrap();
    let second = session
        .capture(&[], "Second save")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    let intent = session
        .prepare_push(&remote, "ssh-lost-response", Some(first.commit.clone()))
        .await
        .unwrap();
    drop(session);
    fs::write(base.join("drop-next-response"), b"fixture").unwrap();
    assert!(
        remote.publish(&intent).await.is_err(),
        "fixture drops the accepted push response and its first observation"
    );
    assert_eq!(remote.pending_push().unwrap(), Some(intent.clone()));
    assert_eq!(
        git(&[
            "--git-dir",
            origin.to_str().unwrap(),
            "rev-parse",
            "refs/heads/main"
        ]),
        second.commit
    );
    let receives = fs::read_to_string(base.join("receive-count")).unwrap();
    assert_eq!(
        remote.reconcile("ssh-lost-response").await.unwrap(),
        PushOutcome::Complete
    );
    assert!(remote.pending_push().unwrap().is_none());
    assert_eq!(
        fs::read_to_string(base.join("receive-count")).unwrap(),
        receives,
        "observation cannot publish again"
    );
    assert_eq!(
        git(&[
            "--git-dir",
            origin.to_str().unwrap(),
            "rev-parse",
            "refs/heads/unrelated"
        ]),
        first.commit
    );
    assert_eq!(
        fs::read_to_string(a.root.join("Note.md")).unwrap(),
        "saved second"
    );
}
