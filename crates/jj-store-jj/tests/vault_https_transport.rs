#![cfg(feature = "vault-backup")]
use jj_store_jj::{
    local_browser,
    original_remotes::{MutationAccess, OriginalRepository},
    vault_workspace::{
        Binding, GitCredentials, PushOutcome, RemoteTarget, RemoteTransfer, Session,
    },
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
};

struct Access(PathBuf);
#[async_trait::async_trait]
impl local_browser::Access for Access {
    fn repository(&self) -> jj_store_model::RepositoryId {
        jj_store_model::RepositoryId::new([81; 16])
    }
    fn generation(&self) -> u64 {
        1
    }
    fn metadata_path(&self) -> &Path {
        &self.0
    }
    async fn authorize(&self) -> local_browser::Result<()> {
        if self.0.is_dir() {
            Ok(())
        } else {
            Err(local_browser::Error::Denied)
        }
    }
}
#[async_trait::async_trait]
impl MutationAccess for Access {
    async fn authorize_mutation(&self) -> local_browser::Result<()> {
        local_browser::Access::authorize(self).await
    }
}
fn jj(root: &Path, args: &[&str]) -> String {
    let result = Command::new("jj")
        .current_dir(root)
        .args([
            "--no-pager",
            "--config",
            "user.name='HTTPS fixture'",
            "--config",
            "user.email='fixture@invalid'",
            "--config",
            "signing.behavior='drop'",
        ])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "disposable JJ command failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    String::from_utf8(result.stdout).unwrap().trim().into()
}
fn target(endpoint: &str, repository: &str) -> RemoteTarget {
    RemoteTarget {
        url: format!("{endpoint}/{repository}.git"),
        branch: "main".into(),
    }
}
fn credentials(token: &str) -> GitCredentials {
    GitCredentials::basic("fixture-user", token).unwrap()
}

#[tokio::test]
#[ignore = "requires the disposable TLS smart-HTTP runner: pnpm test:vault-https"]
async fn real_https_tokens_anonymous_reads_and_original_publication_preserve_scope() {
    let base = PathBuf::from(std::env::var_os("JJ_STORE_HTTPS_FIXTURE").unwrap())
        .canonicalize()
        .unwrap();
    let endpoint = std::env::var("JJ_STORE_HTTPS_ENDPOINT").unwrap();
    let root = base.join("notes");
    fs::create_dir(&root).unwrap();
    let private = base.join("private");
    fs::create_dir(&private).unwrap();
    let binding = Binding {
        vault_id: "https-backup".into(),
        root: root.clone(),
        state: private.join("backup"),
    };
    fs::write(root.join("Note.md"), "first retained bytes").unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    let checkpoint = session
        .capture(&[], "HTTPS fixture checkpoint")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    drop(session);
    let remote = RemoteTransfer::connect(
        binding.clone(),
        target(&endpoint, "token"),
        credentials("first-synthetic-token"),
    )
    .await
    .unwrap();
    assert_eq!(remote.observe().await.unwrap(), None);
    let session = Session::open(binding.clone()).await.unwrap();
    let intent = session
        .prepare_push(&remote, "https-first", None)
        .await
        .unwrap();
    drop(session);
    remote.reserve(&intent).unwrap();
    assert_eq!(
        remote.publish(&intent).await.unwrap(),
        PushOutcome::Complete
    );
    assert_eq!(
        remote.observe().await.unwrap().as_deref(),
        Some(checkpoint.commit.as_str())
    );
    let wrong = RemoteTransfer::connect(
        binding.clone(),
        target(&endpoint, "token"),
        credentials("wrong-synthetic-token"),
    )
    .await
    .unwrap();
    assert!(wrong.observe().await.is_err());
    let unauthenticated =
        RemoteTransfer::connect_anonymous(binding.clone(), target(&endpoint, "token"))
            .await
            .unwrap();
    assert!(unauthenticated.observe().await.is_err());

    // Seed the independent public read endpoint using native fixture-owned objects.
    let publish = Command::new("git")
        .arg("--git-dir")
        .arg(binding.state.join("repo/store/git"))
        .arg("-c")
        .arg("core.hooksPath=/dev/null")
        .arg("push")
        .arg(base.join("exports/anonymous.git"))
        .arg(format!("{}:refs/heads/main", checkpoint.commit))
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .unwrap();
    assert!(
        publish.status.success(),
        "disposable anonymous endpoint seed failed"
    );
    let anonymous =
        RemoteTransfer::connect_anonymous(binding.clone(), target(&endpoint, "anonymous"))
            .await
            .unwrap();
    assert_eq!(
        anonymous.fetch().await.unwrap().unwrap().commit(),
        checkpoint.commit
    );
    let redirect = RemoteTransfer::connect(
        binding.clone(),
        target(&endpoint, "redirect"),
        credentials("first-synthetic-token"),
    )
    .await
    .unwrap();
    assert!(
        redirect.observe().await.is_err(),
        "authenticated requests must not follow redirects"
    );

    let original = base.join("original");
    fs::create_dir(&original).unwrap();
    jj(&original, &["git", "init", "--no-colocate", "."]);
    jj(
        &original,
        &[
            "git",
            "remote",
            "add",
            "origin",
            &target(&endpoint, "token").url,
        ],
    );
    let access = Arc::new(Access(original.join(".jj/repo").canonicalize().unwrap()));
    let mut repository = OriginalRepository::open(access.clone(), original.clone())
        .await
        .unwrap();
    let original_private = base.join("original-private");
    fs::create_dir(&original_private).unwrap();
    let transfer = RemoteTransfer::connect(
        repository.transport_binding(&original_private).unwrap(),
        target(&endpoint, "token"),
        credentials("first-synthetic-token"),
    )
    .await
    .unwrap();
    let before = repository.snapshot().await.unwrap();
    fs::write(
        original.join("Unsnapshotted.md"),
        "fetch must preserve this",
    )
    .unwrap();
    let fetched = repository.fetch("origin", &transfer).await.unwrap();
    assert_eq!(fetched.working_copy, before.working_copy);
    assert!(!original.join("Note.md").exists());
    assert_eq!(
        fs::read_to_string(original.join("Unsnapshotted.md")).unwrap(),
        "fetch must preserve this"
    );
    drop(repository);
    fs::remove_file(original.join("Unsnapshotted.md")).unwrap();
    jj(&original, &["bookmark", "track", "main@origin"]);
    jj(&original, &["new", "main"]);
    fs::write(original.join("Note.md"), "explicit original publication").unwrap();
    jj(&original, &["commit", "-m", "Recorded original change"]);
    jj(&original, &["bookmark", "set", "main", "-r", "@-"]);
    fs::write(
        original.join("Unsnapshotted.md"),
        "push must not capture this",
    )
    .unwrap();
    let repository = OriginalRepository::open(access, original.clone())
        .await
        .unwrap();
    let review = repository
        .review_push("origin", "main", "https-original", &transfer)
        .await
        .unwrap();
    assert_eq!(
        (review.divergence().incoming, review.divergence().outgoing),
        (0, 1)
    );
    assert_eq!(
        review.divergence().remote_commit.as_deref(),
        Some(checkpoint.commit.as_str())
    );
    let proposed = review.commit().to_owned();
    assert_eq!(
        repository.push(&review, &transfer).await.unwrap(),
        PushOutcome::Complete
    );
    assert_eq!(
        remote.observe().await.unwrap().as_deref(),
        Some(proposed.as_str())
    );
    assert_eq!(
        fs::read_to_string(original.join("Unsnapshotted.md")).unwrap(),
        "push must not capture this"
    );
    assert_eq!(
        fs::read_to_string(root.join("Note.md")).unwrap(),
        "first retained bytes"
    );
    assert_eq!(
        anonymous.observe().await.unwrap().as_deref(),
        Some(checkpoint.commit.as_str())
    );

    fs::write(base.join("active-token"), "replacement-synthetic-token").unwrap();
    assert!(
        remote.observe().await.is_err(),
        "the replaced token must stop authorizing reads"
    );
    let replacement = RemoteTransfer::connect(
        binding.clone(),
        target(&endpoint, "token"),
        credentials("replacement-synthetic-token"),
    )
    .await
    .unwrap();
    assert_eq!(
        replacement.observe().await.unwrap().as_deref(),
        Some(proposed.as_str())
    );
    let config = fs::read_to_string(original.join(".jj/repo/store/git/config")).unwrap();
    for secret in ["first-synthetic-token", "replacement-synthetic-token"] {
        assert!(!config.contains(secret));
    }
    let facts: serde_json::Value =
        serde_json::from_slice(&fs::read(base.join("facts.json")).unwrap()).unwrap();
    assert_eq!(
        facts["leaked"], 0,
        "anonymous endpoint must never receive credentials"
    );
    assert_eq!(
        facts["receives"], 2,
        "only the two explicitly reviewed branches publish"
    );
    assert!(facts["refused"].as_u64().unwrap() >= 3);
    assert!(facts["anonymousReads"].as_u64().unwrap() >= 2);
    assert!(facts["redirects"].as_u64().unwrap() >= 1);
}
