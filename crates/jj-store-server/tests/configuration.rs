use jj_store_model::{PrincipalId, RepositoryId};
use jj_store_server::{
    config::{Config, EXAMPLE},
    policy::{PolicyFile, exclusive_lock, outside_checkout},
};
use jj_store_service::{
    administration::OperatorPolicies as _,
    auth::{BearerToken, Permission, Scope, TokenPolicy},
};
use std::fs;
const PRINCIPAL: PrincipalId = PrincipalId::new([31; 16]);
fn load(root: &std::path::Path, text: &str) -> jj_store_server::Result<Config> {
    let path = root.join("server.toml");
    fs::write(&path, text).unwrap();
    Config::load(&path)
}
#[test]
fn configuration_is_bounded_strict_and_resolves_paths_without_creating_state() {
    let root = tempfile::tempdir().unwrap();
    let config = load(root.path(), EXAMPLE).unwrap();
    assert_eq!(
        config.state_dir,
        root.path().canonicalize().unwrap().join("state")
    );
    assert!(!config.state_dir.exists());
    for text in [
        EXAMPLE.replace("format = 2", "format = 3"),
        EXAMPLE.replace(
            "format = 2",
            "secret_key = 'must-not-echo-secret'\nformat = 2",
        ),
        "a".repeat(65537),
        EXAMPLE.replace("request_seconds = 30", "request_seconds = 0"),
        EXAMPLE.replace("concurrent_requests = 8", "concurrent_requests = 257"),
        EXAMPLE.replace("max_blob_bytes = 33554432", "max_blob_bytes = 0"),
        EXAMPLE.replace("inventory_nodes = 100000", "inventory_nodes = 1000001"),
    ] {
        let error = load(root.path(), &text).unwrap_err().to_string();
        assert!(!error.contains("must-not-echo-secret"));
    }
}

#[test]
fn older_configuration_formats_are_explicitly_rejected() {
    let root = tempfile::tempdir().unwrap();
    let error = load(root.path(), "format = 1\n").unwrap_err().to_string();
    assert!(
        error.contains("unsupported configuration format"),
        "unexpected error: {error}"
    );
}

#[test]
fn format_two_backup_targets_are_tagged_bounded_and_strict() {
    let root = tempfile::tempdir().unwrap();
    let backup = r#"
[[backup]]
driver = "wal-copy"
name = "offsite"
endpoint = "https://backup.example.test/"
bucket = "jj-store-backup"
region = "us-east-1"

[backup.credentials]
source = "environment"
access_key = "JJ_STORE_BACKUP_ACCESS_KEY"
secret_key = "JJ_STORE_BACKUP_SECRET_KEY"
"#;
    let config = load(root.path(), &format!("{EXAMPLE}{backup}")).unwrap();
    assert_eq!(config.backup.len(), 1);
    assert_eq!(config.backup[0].name(), "offsite");
    assert!(load(root.path(), &format!("{EXAMPLE}{backup}{backup}")).is_err());
    assert!(
        load(
            root.path(),
            &format!("{EXAMPLE}{}", backup.replace("wal-copy", "replicate"))
        )
        .is_err()
    );
}
#[test]
fn insecure_shared_endpoints_credentials_and_checkout_control_paths_are_rejected() {
    let root = tempfile::tempdir().unwrap();
    for text in [
        EXAMPLE.replace("127.0.0.1:7443", "0.0.0.0:7443"),
        EXAMPLE.replace("insecure_loopback = true", "insecure_loopback = false"),
        EXAMPLE.replace(
            "http://127.0.0.1:7443/",
            "http://user:password@127.0.0.1:7443/",
        ),
        EXAMPLE.replace("http://127.0.0.1:9000/", "http://127.0.0.1:9000/path"),
        EXAMPLE.replace("bucket = \"jj-store\"", "bucket = \"../escape\""),
    ] {
        assert!(load(root.path(), &text).is_err());
    }
    let checkout = root.path().join("repo");
    fs::create_dir(&checkout).unwrap();
    fs::create_dir(checkout.join(".jj")).unwrap();
    assert!(outside_checkout(&checkout.join("nested/state")).is_err());
    assert!(load(root.path(), &EXAMPLE.replace("./state", "./repo/state")).is_err());
    assert!(
        load(
            root.path(),
            &EXAMPLE.replace("./operators.json", "./repo/operators.json")
        )
        .is_err()
    );
}
#[tokio::test]
async fn operator_policy_persists_only_verifiers_and_reload_observes_revocation() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("operators.json");
    let file = PolicyFile::new(path.clone()).unwrap();
    let token = BearerToken::generate().unwrap();
    file.initialize(PRINCIPAL, &token).unwrap();
    file.initialize(PRINCIPAL, &token).unwrap();
    assert!(
        file.initialize(PRINCIPAL, &BearerToken::generate().unwrap())
            .is_err()
    );
    assert!(
        file.current()
            .await
            .unwrap()
            .authorize(&token, Permission::CreateRepository, 0)
            .is_ok()
    );
    let bytes = fs::read(&path).unwrap();
    assert!(!String::from_utf8_lossy(&bytes).contains(&token.expose()[38..]));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        assert_eq!(fs::metadata(&path).unwrap().permissions().mode() & 0o077, 0);
    }
    assert!(file.revoke(token.id()).unwrap());
    assert!(!file.revoke(token.id()).unwrap());
    assert!(
        file.current()
            .await
            .unwrap()
            .authorize(&token, Permission::CreateRepository, 0)
            .is_err()
    );
    let mut wrong = TokenPolicy::new(Scope::Repository(RepositoryId::new([1; 16])));
    wrong.issue(PRINCIPAL, Permission::Admin, None).unwrap();
    fs::write(&path, wrong.encode().unwrap()).unwrap();
    assert!(file.current().await.is_err());
}
#[test]
fn operator_updates_serialize_and_state_locks_release_with_the_owner() {
    let root = tempfile::tempdir().unwrap();
    let file = PolicyFile::new(root.path().join("operators.json")).unwrap();
    file.initialize(PRINCIPAL, &BearerToken::generate().unwrap())
        .unwrap();
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let file = file.clone();
            std::thread::spawn(move || {
                let token = BearerToken::generate().unwrap();
                for _ in 0..200 {
                    if file.grant(PRINCIPAL, &token).is_ok() {
                        return;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                panic!("operator lock did not become available");
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(file.read().unwrap().tokens().len(), 9);
    let path = root.path().join("server.lock");
    let owner = exclusive_lock(&path).unwrap();
    assert!(exclusive_lock(&path).is_err());
    drop(owner);
    assert!(exclusive_lock(&path).is_ok());
}
#[cfg(unix)]
#[test]
fn configuration_and_policy_symlinks_cannot_bypass_private_path_rules() {
    use std::os::unix::fs::symlink;
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("target");
    fs::write(&path, EXAMPLE).unwrap();
    symlink(&path, root.path().join("config.toml")).unwrap();
    assert!(Config::load(&root.path().join("config.toml")).is_err());
    assert!(PolicyFile::new(root.path().join("config.toml")).is_err());
    let checkout = root.path().join("repo");
    fs::create_dir(&checkout).unwrap();
    fs::write(checkout.join(".git"), "gitdir: elsewhere").unwrap();
    symlink(&checkout, root.path().join("alias")).unwrap();
    assert!(outside_checkout(&root.path().join("alias/state/nested")).is_err());
}

#[test]
fn programmatic_configuration_cannot_bypass_private_path_validation() {
    let root = tempfile::tempdir().unwrap();
    let mut config = load(root.path(), EXAMPLE).unwrap();
    let repo = root.path().join("repo");
    fs::create_dir(&repo).unwrap();
    fs::create_dir(repo.join(".jj")).unwrap();
    config.state_dir = repo.join("state");
    assert!(config.validate().is_err());
}

#[test]
fn search_configuration_is_explicit_bounded_and_contains_no_credentials() {
    let root = tempfile::tempdir().unwrap();
    let search = format!(
        r#"
[[search]]
repository = "{}"
token_environment = "JJ_STORE_SEARCH_TOKEN_EXAMPLE"
poll_millis = 100
[search.source.scopes.main]
kind = "bookmark"
name = "main"
"#,
        RepositoryId::new([71; 16])
    );
    let config = load(root.path(), &format!("{EXAMPLE}{search}")).unwrap();
    assert_eq!(config.search.len(), 1);
    assert_eq!(config.search[0].source.scopes.len(), 1);
    assert_eq!(config.search[0].index_bytes, 512 * 1024 * 1024);
    assert_eq!(config.search[0].snapshot_every, 32);
    assert_eq!(
        config.search[0].token_environment.as_deref(),
        Some("JJ_STORE_SEARCH_TOKEN_EXAMPLE")
    );
    assert!(!config.state_dir.exists());
    for bad in [
        search.replace("JJ_STORE_SEARCH_TOKEN_EXAMPLE", "JJ_STORE_TOKEN"),
        search.replace("poll_millis = 100", "poll_millis = 0"),
        search.replace("poll_millis = 100", "index_bytes = 0"),
        search.replace("poll_millis = 100", "timeout_seconds = 301"),
        search.replace("poll_millis = 100", "snapshot_every = 1000001"),
        search.replace("poll_millis = 100", "token = 'must-not-echo-secret'"),
        search.replace(
            "[search.source.scopes.main]",
            "[search.source]\nunknown = true\n[search.source.scopes.main]",
        ),
        format!("{search}{search}"),
    ] {
        let error = load(root.path(), &format!("{EXAMPLE}{bad}"))
            .unwrap_err()
            .to_string();
        assert!(!error.contains("must-not-echo-secret"));
    }

    let local_access = format!(
        r#"[access]
mode = "local-trusted"
installation = "{}"
catalog = "{}"
rpc_socket = "{}"
permission = "admin""#,
        root.path().join("installation.json").display(),
        root.path().join("repositories.json").display(),
        root.path().join("native.sock").display()
    );
    let local = EXAMPLE.replace("[access]\nmode = \"authenticated\"", &local_access);
    let local_search = search.replace(
        "token_environment = \"JJ_STORE_SEARCH_TOKEN_EXAMPLE\"\n",
        "",
    );
    let config = load(root.path(), &format!("{local}{local_search}")).unwrap();
    assert_eq!(config.search[0].token_environment, None);
    assert!(load(root.path(), &format!("{EXAMPLE}{local_search}")).is_err());
    assert!(load(root.path(), &format!("{local}{search}")).is_err());
}
