#![cfg(feature = "vault-backup")]
use jj_store_jj::{
    local_browser::{self, Access as _},
    original_remotes::{Error, MutationAccess, OriginalRepository},
    vault_workspace::{HostSshIdentity, RemoteTarget, RemoteTransfer},
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
struct Access {
    metadata: PathBuf,
    write: AtomicBool,
}
#[async_trait::async_trait]
impl local_browser::Access for Access {
    fn repository(&self) -> jj_store_model::RepositoryId {
        jj_store_model::RepositoryId::new([73; 16])
    }
    fn generation(&self) -> u64 {
        1
    }
    fn metadata_path(&self) -> &Path {
        &self.metadata
    }
    async fn authorize(&self) -> local_browser::Result<()> {
        if self.metadata.is_dir() {
            Ok(())
        } else {
            Err(local_browser::Error::Denied)
        }
    }
}
#[async_trait::async_trait]
impl MutationAccess for Access {
    async fn authorize_mutation(&self) -> local_browser::Result<()> {
        self.authorize().await?;
        if self.write.load(Ordering::SeqCst) {
            Ok(())
        } else {
            Err(local_browser::Error::Denied)
        }
    }
}
fn jj(root: &Path, args: &[&str]) -> String {
    let result = Command::new("jj")
        .current_dir(root)
        .args([
            "--no-pager",
            "--config",
            "user.name='Original fixture'",
            "--config",
            "user.email='fixture@invalid'",
            "--config",
            "signing.behavior='drop'",
            "--config",
            "git.sign-on-push=false",
        ])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "fixture JJ command failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    String::from_utf8(result.stdout).unwrap().trim().into()
}
fn setup(base: &Path, name: &str) -> (PathBuf, Arc<Access>) {
    let root = base.join(name);
    fs::create_dir(&root).unwrap();
    jj(&root, &["git", "init", "--no-colocate", "."]);
    let access = Arc::new(Access {
        metadata: root.join(".jj/repo").canonicalize().unwrap(),
        write: AtomicBool::new(true),
    });
    (root, access)
}
#[tokio::test]
async fn discovery_is_read_only_and_never_returns_embedded_credentials() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let (root, access) = setup(&base, "original");
    jj(
        &root,
        &[
            "git",
            "remote",
            "add",
            "origin",
            "https://forge.example/notes.git",
        ],
    );
    jj(
        &root,
        &[
            "git",
            "remote",
            "add",
            "unsafe",
            "https://user:synthetic-secret@forge.example/notes.git",
        ],
    );
    let repository = OriginalRepository::open(access, root.clone())
        .await
        .unwrap();
    let before = repository.snapshot().await.unwrap();
    assert_eq!(
        before
            .remotes
            .iter()
            .find(|remote| remote.name == "origin")
            .unwrap()
            .url
            .as_deref(),
        Some("https://forge.example/notes.git")
    );
    assert!(
        before
            .remotes
            .iter()
            .find(|remote| remote.name == "unsafe")
            .unwrap()
            .url
            .is_none()
    );
    assert!(!format!("{before:?}").contains("synthetic-secret"));
    let private = base.join("private");
    fs::create_dir(&private).unwrap();
    let binding = repository.transport_binding(&private).unwrap();
    assert!(
        !binding.state.exists(),
        "discovery/binding selection must not initialize transport storage"
    );
    assert!(repository.transport_binding(&root.join("private")).is_err());
    assert_eq!(
        repository.snapshot().await.unwrap().operation,
        before.operation
    );
}
#[tokio::test]
async fn mutation_admission_and_backup_cache_isolation_refuse_before_network() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let (root, access) = setup(&base, "original");
    let url = "https://forge.example/notes.git";
    jj(&root, &["git", "remote", "add", "origin", url]);
    let mut repository = OriginalRepository::open(access.clone(), root)
        .await
        .unwrap();
    let before = repository.snapshot().await.unwrap();
    let private = base.join("private");
    fs::create_dir(&private).unwrap();
    let binding = repository.transport_binding(&private).unwrap();
    let transfer = RemoteTransfer::connect_anonymous(
        binding.clone(),
        RemoteTarget {
            url: url.into(),
            branch: "main".into(),
        },
    )
    .await
    .unwrap();
    access.write.store(false, Ordering::SeqCst);
    assert!(matches!(
        repository.fetch("origin", &transfer).await,
        Err(Error::Denied)
    ));
    access.write.store(true, Ordering::SeqCst);
    let mut backup = binding;
    backup.vault_id = "managed-backup".into();
    backup.state = private.join("backup");
    let backup = RemoteTransfer::connect_anonymous(
        backup,
        RemoteTarget {
            url: url.into(),
            branch: "main".into(),
        },
    )
    .await
    .unwrap();
    assert!(matches!(
        repository.fetch("origin", &backup).await,
        Err(Error::Binding)
    ));
    assert_eq!(
        repository.snapshot().await.unwrap().operation,
        before.operation
    );
}
#[tokio::test]
#[ignore = "requires the disposable sshd/agent runner with --original"]
async fn original_fetch_moves_tracked_bookmarks_without_capturing_or_writing_files() {
    let base = PathBuf::from(std::env::var_os("JJ_STORE_SSH_FIXTURE").unwrap())
        .canonicalize()
        .unwrap();
    let endpoint = std::env::var("JJ_STORE_SSH_ENDPOINT").unwrap();
    let (publisher, _) = setup(&base, "publisher");
    fs::write(publisher.join("Note.md"), "first remote version").unwrap();
    jj(&publisher, &["commit", "-m", "First recorded version"]);
    let first = jj(
        &publisher,
        &["log", "-r", "@-", "--no-graph", "-T", "commit_id"],
    );
    let publish = |commit: &str| {
        let result = Command::new("git")
            .arg("--git-dir")
            .arg(publisher.join(".jj/repo/store/git"))
            .arg("push")
            .arg(base.join("remote.git"))
            .arg(format!("{commit}:refs/heads/main"))
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .output()
            .unwrap();
        assert!(result.status.success(), "local fixture publication failed");
    };
    publish(&first);
    let (root, access) = setup(&base, "receiver");
    jj(&root, &["git", "remote", "add", "origin", &endpoint]);
    let mut repository = OriginalRepository::open(access.clone(), root.clone())
        .await
        .unwrap();
    let private = base.join("original-private");
    fs::create_dir(&private).unwrap();
    let binding = repository.transport_binding(&private).unwrap();
    let transfer = RemoteTransfer::connect_ssh(
        binding,
        RemoteTarget {
            url: endpoint,
            branch: "main".into(),
        },
        HostSshIdentity::Agent,
    )
    .await
    .unwrap()
    .with_known_hosts(&base.join("known 'hosts'"))
    .unwrap();
    let before = repository.snapshot().await.unwrap();
    let fetched = repository.fetch("origin", &transfer).await.unwrap();
    assert_eq!(fetched.working_copy, before.working_copy);
    assert!(!root.join("Note.md").exists());
    drop(repository);
    jj(&root, &["bookmark", "track", "main@origin"]);
    let mut repository = OriginalRepository::open(access, root.clone())
        .await
        .unwrap();
    assert_eq!(
        repository
            .snapshot()
            .await
            .unwrap()
            .bookmarks
            .iter()
            .find(|bookmark| bookmark.name == "main")
            .unwrap()
            .commit
            .as_deref(),
        Some(first.as_str())
    );
    fs::write(
        root.join("Unsnapshotted.md"),
        "saved on disk, never captured",
    )
    .unwrap();
    fs::write(publisher.join("Note.md"), "second remote version").unwrap();
    jj(&publisher, &["commit", "-m", "Second recorded version"]);
    let second = jj(
        &publisher,
        &["log", "-r", "@-", "--no-graph", "-T", "commit_id"],
    );
    publish(&second);
    let before = repository.snapshot().await.unwrap();
    let fetched = repository.fetch("origin", &transfer).await.unwrap();
    assert_ne!(fetched.operation, before.operation);
    assert_eq!(fetched.working_copy, before.working_copy);
    assert_eq!(
        fetched
            .bookmarks
            .iter()
            .find(|bookmark| bookmark.name == "main")
            .unwrap()
            .commit
            .as_deref(),
        Some(second.as_str())
    );
    assert_eq!(
        fs::read_to_string(root.join("Unsnapshotted.md")).unwrap(),
        "saved on disk, never captured"
    );
    assert!(!root.join("Note.md").exists());
    let unchanged = repository.fetch("origin", &transfer).await.unwrap();
    assert_eq!(
        unchanged.operation, fetched.operation,
        "unchanged fetch must not fabricate a native operation"
    );
}

#[tokio::test]
async fn an_externally_changed_endpoint_refuses_the_stale_transfer() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let (root, access) = setup(&base, "original");
    let url = "https://forge.example/notes.git";
    jj(&root, &["git", "remote", "add", "origin", url]);
    let mut repository = OriginalRepository::open(access, root.clone())
        .await
        .unwrap();
    let private = base.join("private");
    fs::create_dir(&private).unwrap();
    let transfer = RemoteTransfer::connect_anonymous(
        repository.transport_binding(&private).unwrap(),
        RemoteTarget {
            url: url.into(),
            branch: "main".into(),
        },
    )
    .await
    .unwrap();
    jj(
        &root,
        &[
            "git",
            "remote",
            "set-url",
            "origin",
            "https://other.example/notes.git",
        ],
    );
    assert!(matches!(
        repository.fetch("origin", &transfer).await,
        Err(Error::Stale)
    ));
}

#[tokio::test]
#[ignore = "requires the disposable sshd/agent runner with --original"]
async fn original_push_publishes_one_reviewed_bookmark_and_recovers_without_replay() {
    use jj_store_jj::vault_workspace::PushOutcome;
    let base = PathBuf::from(std::env::var_os("JJ_STORE_SSH_FIXTURE").unwrap())
        .canonicalize()
        .unwrap();
    let endpoint = std::env::var("JJ_STORE_SSH_ENDPOINT").unwrap();
    let (root, access) = setup(&base, "original-publisher");
    jj(&root, &["git", "remote", "add", "origin", &endpoint]);
    fs::write(root.join("Note.md"), "reviewed original bytes").unwrap();
    jj(&root, &["commit", "-m", "First original publication"]);
    let first = jj(&root, &["log", "-r", "@-", "--no-graph", "-T", "commit_id"]);
    jj(&root, &["bookmark", "create", "published", "-r", "@-"]);
    jj(&root, &["bookmark", "create", "never-publish", "-r", "@-"]);
    fs::write(root.join("Unsnapshotted.md"), "must remain unpublished").unwrap();
    let repository = OriginalRepository::open(access.clone(), root.clone())
        .await
        .unwrap();
    let private = base.join("original-push-private");
    fs::create_dir(&private).unwrap();
    let binding = repository.transport_binding(&private).unwrap();
    let transfer = RemoteTransfer::connect_ssh(
        binding.clone(),
        RemoteTarget {
            url: endpoint,
            branch: "selected-publish".into(),
        },
        HostSshIdentity::Agent,
    )
    .await
    .unwrap()
    .with_known_hosts(&base.join("known 'hosts'"))
    .unwrap();
    let before = repository.snapshot().await.unwrap();
    let review = repository
        .review_push("origin", "published", "first-original", &transfer)
        .await
        .unwrap();
    assert_eq!(review.commit(), first);
    assert_eq!(
        transfer.observe().await.unwrap(),
        None,
        "review must not publish"
    );
    assert_eq!(
        repository.snapshot().await.unwrap().operation,
        before.operation
    );
    assert_eq!(
        repository.push(&review, &transfer).await.unwrap(),
        PushOutcome::Complete
    );
    assert_eq!(
        transfer.observe().await.unwrap().as_deref(),
        Some(first.as_str())
    );
    assert_eq!(
        fs::read_to_string(root.join("Unsnapshotted.md")).unwrap(),
        "must remain unpublished"
    );
    let remote_git = |args: &[&str]| {
        Command::new("git")
            .arg("--git-dir")
            .arg(base.join("remote.git"))
            .args(args)
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .output()
            .unwrap()
    };
    assert!(
        !remote_git(&["show-ref", "--verify", "refs/heads/never-publish"])
            .status
            .success()
    );
    assert!(
        !remote_git(&["cat-file", "-e", &format!("{first}:Unsnapshotted.md")])
            .status
            .success()
    );
    drop(repository);
    fs::remove_file(root.join("Unsnapshotted.md")).unwrap();
    fs::write(root.join("Note.md"), "second reviewed original bytes").unwrap();
    jj(&root, &["commit", "-m", "Second original publication"]);
    let second = jj(&root, &["log", "-r", "@-", "--no-graph", "-T", "commit_id"]);
    jj(&root, &["bookmark", "set", "published", "-r", "@-"]);
    fs::write(root.join("Unsnapshotted.md"), "still unpublished").unwrap();
    let repository = OriginalRepository::open(access, root.clone())
        .await
        .unwrap();
    let review = repository
        .review_push("origin", "published", "lost-original", &transfer)
        .await
        .unwrap();
    fs::write(base.join("drop-next-response"), "fixture").unwrap();
    assert!(matches!(
        repository.push(&review, &transfer).await,
        Err(Error::Transport)
    ));
    assert_eq!(
        binding.pending_publication().unwrap().unwrap().request_id,
        "lost-original"
    );
    let receives = fs::read_to_string(base.join("receive-count")).unwrap();
    assert_eq!(
        transfer.reconcile("lost-original").await.unwrap(),
        PushOutcome::Complete
    );
    assert_eq!(
        repository.push(&review, &transfer).await.unwrap(),
        PushOutcome::Complete
    );
    assert_eq!(
        fs::read_to_string(base.join("receive-count")).unwrap(),
        receives
    );
    assert_eq!(
        transfer.observe().await.unwrap().as_deref(),
        Some(second.as_str())
    );
    assert!(binding.pending_publication().unwrap().is_none());
    assert!(matches!(
        repository
            .review_push("origin", "never-publish", "divergent-original", &transfer)
            .await,
        Err(Error::Diverged)
    ));
    assert_eq!(
        fs::read_to_string(base.join("receive-count")).unwrap(),
        receives,
        "divergent review must not publish"
    );
    assert_eq!(
        fs::read_to_string(root.join("Unsnapshotted.md")).unwrap(),
        "still unpublished"
    );
    assert!(
        !remote_git(&["cat-file", "-e", &format!("{second}:Unsnapshotted.md")])
            .status
            .success()
    );
    // A changed native operation cannot execute the retained review again.
    jj(&root, &["bookmark", "delete", "published"]);
    assert!(matches!(
        repository.push(&review, &transfer).await,
        Err(Error::Stale)
    ));
}

#[tokio::test]
async fn explicit_remote_save_preserves_origin_and_rejects_stale_configuration_without_transfer() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let (root, access) = setup(&base, "original");
    let origin = "https://forge.example/original.git";
    jj(&root, &["git", "remote", "add", "origin", origin]);
    let repository = OriginalRepository::open(access, root.clone())
        .await
        .unwrap();
    let before = repository.snapshot().await.unwrap();
    let target = RemoteTarget {
        url: "https://unreachable.invalid/selected.git".into(),
        branch: "main".into(),
    };
    let saved = repository
        .save_remote(&before.remote_configuration, "backup-host", &target)
        .await
        .unwrap();
    assert_eq!(saved.operation, before.operation);
    assert_ne!(saved.remote_configuration, before.remote_configuration);
    assert_eq!(
        saved
            .remotes
            .iter()
            .find(|r| r.name == "origin")
            .unwrap()
            .url
            .as_deref(),
        Some(origin)
    );
    assert_eq!(
        saved
            .remotes
            .iter()
            .find(|r| r.name == "backup-host")
            .unwrap()
            .url
            .as_deref(),
        Some(target.url.as_str())
    );
    assert!(matches!(
        repository
            .save_remote(&before.remote_configuration, "origin", &target)
            .await,
        Err(Error::Stale)
    ));
    assert_eq!(
        repository.snapshot().await.unwrap().remote_configuration,
        saved.remote_configuration
    );
    assert!(!root.join("original-transport").exists());
}
