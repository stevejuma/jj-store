use jj_store_model::{PrincipalId, RepositoryId};
use jj_store_server::{
    config::{Config, EXAMPLE},
    policy::{PolicyFile, exclusive_lock, outside_checkout, outside_checkout_for_development},
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
fn development_browser_annotations_require_explicit_loopback_local_trusted_assets() {
    use jj_store_server::config::Access;
    let root = tempfile::tempdir().unwrap();
    let mut config = load(root.path(), EXAMPLE).unwrap();
    assert!(!config.development_browser_annotations);
    config.development_browser_annotations = true;
    config.web_root = Some(root.path().join("browser"));
    assert!(
        config.validate().is_err(),
        "authenticated deployment must reject it"
    );
    config.access = Access::LocalTrusted {
        installation: root.path().join("installation.json"),
        catalog: root.path().join("repositories.json"),
        rpc_socket: root.path().join("native.sock"),
        permission: Permission::Admin,
    };
    config.validate().unwrap();
    config.bind = "0.0.0.0:7443".parse().unwrap();
    assert!(config.validate().is_err());
    config.bind = "127.0.0.1:7443".parse().unwrap();
    config.web_root = None;
    assert!(config.validate().is_err());
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
fn automatic_capture_configuration_is_explicit_local_writable_and_bounded() {
    let root = tempfile::tempdir().unwrap();
    let local = EXAMPLE.replace(
        "[access]\nmode = \"authenticated\"\ninstallation = \"./installation.json\"\ncatalog = \"./repositories.json\"",
        "[access]\nmode = \"local-trusted\"\ninstallation = \"./installation.json\"\ncatalog = \"./repositories.json\"\nrpc_socket = \"./native.sock\"\npermission = \"write\"",
    );
    let capture = r#"
[agent_capture]
enabled = true
max_readers = 4
rescan_seconds = 30
max_sources = 1024
max_pending_bytes = 67108864
[[agent_capture.harnesses]]
profile = "codex-cli-jsonl-v1"
product_version = "0.156.1"
roots = ["./transcripts"]
"#;
    let config = load(root.path(), &format!("{local}{capture}")).unwrap();
    assert!(config.agent_capture.enabled);
    assert_eq!(config.agent_capture.max_readers, 4);
    assert_eq!(
        config.agent_capture.harnesses[0].roots,
        [root.path().canonicalize().unwrap().join("./transcripts")]
    );
    assert_eq!(
        config.agent_capture.harnesses[0]
            .codex_roots(None, Some(std::path::Path::new("invalid-ambient")))
            .unwrap(),
        config.agent_capture.harnesses[0].roots,
    );
    assert!(
        !config.state_dir.exists(),
        "config check must create no state"
    );
    let desktop = capture.replace("codex-cli-jsonl-v1", "codex-desktop-jsonl-v1");
    assert!(load(root.path(), &format!("{local}{desktop}")).is_err());
    let desktop = desktop.replace("0.156.1", "0.159.2");
    let desktop_config = load(root.path(), &format!("{local}{desktop}")).unwrap();
    assert_eq!(
        desktop_config.agent_capture.harnesses[0].product_version,
        "0.159.2"
    );
    for bad in [
        capture.replace("max_readers = 4", "max_readers = 5"),
        capture.replace("rescan_seconds = 30", "rescan_seconds = 31"),
        capture.replace("max_sources = 1024", "max_sources = 0"),
        capture.replace("max_pending_bytes = 67108864", "max_pending_bytes = 0"),
        capture.replace("codex-cli-jsonl-v1", "unknown-profile"),
        capture.replace("0.156.1", "unknown-version"),
        capture.replace(
            "enabled = true",
            "enabled = true\ntoken = 'must-not-echo-secret'",
        ),
        capture.replace(
            "[[agent_capture.harnesses]]",
            "[[agent_capture.harnesses]]\nunknown = true",
        ),
        format!(
            "{capture}{}",
            &capture[capture.find("[[agent_capture.harnesses]]").unwrap()..]
        ),
        "\n[agent_capture]\nenabled = true\n".into(),
    ] {
        let error = load(root.path(), &format!("{local}{bad}"))
            .unwrap_err()
            .to_string();
        assert!(!error.contains("must-not-echo-secret"));
    }
    assert!(load(root.path(), &format!("{EXAMPLE}{capture}")).is_err());
    assert!(
        load(
            root.path(),
            &format!(
                "{}{capture}",
                local.replace("permission = \"write\"", "permission = \"read\"")
            )
        )
        .is_err()
    );
}

#[test]
fn configured_codex_locations_use_qualified_versions_and_explicit_override_precedence() {
    use jj_store_capture::profile::CaptureProfile;
    use jj_store_server::capture_config::HarnessConfig;
    let root = tempfile::tempdir().unwrap();
    let home = root.path().join("home");
    let data = root.path().join("codex-data");
    let mut harness = HarnessConfig {
        profile: CaptureProfile::CodexDesktopJsonlV1,
        product_version: "0.159.2".into(),
        roots: Vec::new(),
        endpoint: None,
    };
    assert_eq!(
        harness.codex_roots(Some(&home), None).unwrap(),
        vec![home.join(".codex/sessions")]
    );
    assert_eq!(
        harness.codex_roots(None, Some(&data)).unwrap(),
        vec![data.join("sessions")]
    );
    harness.roots.push(root.path().join("explicit"));
    assert_eq!(harness.codex_roots(None, None).unwrap(), harness.roots);
    harness.product_version = "0.156.1".into();
    assert!(harness.codex_roots(None, None).is_err());
    assert!(!home.exists());
    assert!(!data.exists());
}

#[test]
fn capture_api_config_rejects_credentials_remote_hosts_and_profile_confusion() {
    use jj_store_capture::profile::CaptureProfile;
    use jj_store_server::{
        capture_config::HarnessConfig,
        config::{Access, Storage},
    };
    let root = tempfile::tempdir().unwrap();
    let mut config = load(root.path(), EXAMPLE).unwrap();
    let original = toml::to_string(&config).unwrap();
    assert!(
        !original.contains("agent_capture"),
        "omitted config preserves old output"
    );
    config.access = Access::LocalTrusted {
        installation: root.path().join("installation.json"),
        catalog: root.path().join("repositories.json"),
        rpc_socket: root.path().join("native.sock"),
        permission: Permission::Admin,
    };
    config.agent_capture.enabled = true;
    config.agent_capture.harnesses.push(HarnessConfig {
        profile: CaptureProfile::OpencodeReadOnlyApiV1,
        product_version: "1.18.30".into(),
        roots: Vec::new(),
        endpoint: Some("http://127.0.0.1:4096/".into()),
    });
    config.validate().unwrap();
    for bad in [
        "http://localhost:4096/",
        "https://remote.example/",
        "http://user:must-not-echo-secret@127.0.0.1:4096/",
        "http://127.0.0.1:4096/?token=must-not-echo-secret",
        "http://127.0.0.1:4096/#fragment",
        "http://127.0.0.1:4096/session",
    ] {
        config.agent_capture.harnesses[0].endpoint = Some(bad.into());
        let error = config.validate().unwrap_err().to_string();
        assert!(!error.contains("must-not-echo-secret"));
    }
    config.agent_capture.harnesses[0].endpoint = Some("http://[::1]:4096/".into());
    config.validate().unwrap();
    config.agent_capture.harnesses[0]
        .roots
        .push(root.path().join("transcripts"));
    assert!(config.validate().is_err());
    config.agent_capture.harnesses[0].roots.clear();
    config.storage = Storage::Filesystem;
    assert!(
        config.validate().is_err(),
        "automatic native capture needs native authority"
    );
    config.agent_capture.enabled = false;
    config.validate().unwrap();
}

#[tokio::test]
async fn unsupported_capture_reader_fails_before_server_state_creation() {
    use jj_store_capture::profile::CaptureProfile;
    use jj_store_server::{capture_config::HarnessConfig, config::Access, process::PreparedServer};
    let root = tempfile::tempdir().unwrap();
    let mut config = load(root.path(), EXAMPLE).unwrap();
    config.access = Access::LocalTrusted {
        installation: root.path().join("installation.json"),
        catalog: root.path().join("repositories.json"),
        rpc_socket: root.path().join("native.sock"),
        permission: Permission::Write,
    };
    config.agent_capture.enabled = true;
    config.agent_capture.harnesses.push(HarnessConfig {
        profile: CaptureProfile::OpencodeExportJsonV1,
        product_version: CaptureProfile::OpencodeExportJsonV1
            .product_version()
            .into(),
        roots: Vec::new(),
        endpoint: None,
    });
    let error = match PreparedServer::prepare(&config, true).await {
        Ok(_) => panic!("unsupported reader must fail closed"),
        Err(error) => error,
    };
    assert!(error.to_string().contains("capture reader unavailable"));
    assert!(!config.state_dir.exists());
    assert!(!root.path().join("native.sock").exists());
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

#[test]
fn explicit_ignored_development_root_allows_only_its_descendants() {
    let root = tempfile::tempdir().unwrap();
    let checkout = root.path().join("repo");
    fs::create_dir(&checkout).unwrap();
    fs::create_dir(checkout.join(".jj")).unwrap();
    let development = checkout.join(".local/dev-demo");
    fs::create_dir_all(&development).unwrap();

    assert_eq!(
        outside_checkout_for_development(&development.join("state"), Some(&development)).unwrap(),
        development.canonicalize().unwrap().join("state")
    );
    assert!(outside_checkout_for_development(&checkout.join("other"), Some(&development)).is_err());
    assert!(
        outside_checkout_for_development(
            &checkout.join("state"),
            Some(&checkout.join("not-local"))
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
    let local = EXAMPLE.replace(
        "[access]\nmode = \"authenticated\"\ninstallation = \"./installation.json\"\ncatalog = \"./repositories.json\"",
        &local_access,
    );
    let local_search = search.replace(
        "token_environment = \"JJ_STORE_SEARCH_TOKEN_EXAMPLE\"\n",
        "",
    );
    let config = load(root.path(), &format!("{local}{local_search}")).unwrap();
    assert_eq!(config.search[0].token_environment, None);
    assert!(load(root.path(), &format!("{EXAMPLE}{local_search}")).is_err());
    assert!(load(root.path(), &format!("{local}{search}")).is_err());
}

#[test]
fn replication_configuration_is_one_upstream_per_repository_bounded_and_secret_free() {
    let root = tempfile::tempdir().unwrap();
    let repository = RepositoryId::new([92; 16]);
    let replication = format!(
        r#"
[[replication]]
repository = "{repository}"
upstream = "https://hub.example.test/"
token_environment = "JJ_STORE_REPLICATION_TOKEN_MAIN"
ca_certificate = "./hub-ca.pem"

[replication.limits]
max_batch_origins = 64
poll_millis = 250
retry_initial_millis = 100
retry_max_seconds = 10
timeout_seconds = 15
"#
    );
    let config = load(root.path(), &format!("{EXAMPLE}{replication}")).unwrap();
    assert_eq!(config.replication.len(), 1);
    assert_eq!(config.replication[0].repository, repository);
    assert_eq!(
        config.replication[0].ca_certificate.as_deref(),
        Some(
            root.path()
                .canonicalize()
                .unwrap()
                .join("hub-ca.pem")
                .as_path()
        )
    );
    let redacted = toml::to_string(&config).unwrap();
    assert!(redacted.contains("JJ_STORE_REPLICATION_TOKEN_MAIN"));
    assert!(!redacted.contains("jst1_"));

    for bad in [
        format!("{replication}{replication}"),
        replication.replace("https://hub.example.test/", "http://hub.example.test/"),
        replication.replace(
            "JJ_STORE_REPLICATION_TOKEN_MAIN",
            "JJ_STORE_REPLICATION_TOKEN_main",
        ),
        replication.replace("max_batch_origins = 64", "max_batch_origins = 0"),
        replication.replace("poll_millis = 250", "poll_millis = 60001"),
        replication.replace("timeout_seconds = 15", "token = 'must-not-echo-secret'"),
    ] {
        let error = load(root.path(), &format!("{EXAMPLE}{bad}"))
            .unwrap_err()
            .to_string();
        assert!(!error.contains("must-not-echo-secret"));
    }
}

#[test]
fn documentation_configuration_examples_use_production_validation() {
    let source = include_str!("../../../docs/site/reference/configuration.md");
    let root = tempfile::tempdir().unwrap();
    let mut count = 0;
    for example in source.split("<!-- config-example:").skip(1) {
        let text = example
            .split_once("```toml\n")
            .unwrap()
            .1
            .split_once("\n```")
            .unwrap()
            .0;
        let text = text
            .replace("/DOCS_STATE", root.path().to_str().unwrap())
            .replace("/DOCS_WEB", root.path().to_str().unwrap());
        let config = load(root.path(), &text).unwrap();
        assert_eq!(config.format, 2);
        assert!(
            load(
                root.path(),
                &format!("unknown_documented_key = true\n{text}")
            )
            .is_err()
        );
        count += 1;
    }
    assert!(
        count > 0,
        "At least one complete validated example is required"
    );
}
