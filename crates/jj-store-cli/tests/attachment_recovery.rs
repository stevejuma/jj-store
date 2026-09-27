//! Resumed physical attachment after a server-accepted publication whose client
//! acknowledgement is lost. No test-only failpoints enter production commands.
use jj_lib::{
    config::{ConfigLayer, ConfigSource, StackedConfig},
    settings::UserSettings,
};
use jj_store_cli::{attachment::Attachment, binding::Connection};
use jj_store_jj::policy::Protection;
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_rpc::{
    client::NativeClient,
    server::{Limits, NativeRpc},
};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    auth::*,
    native::SessionLimits,
};
use std::{
    fs,
    process::{Command, Output},
    sync::{Arc, Mutex},
};
const REPO: m::RepositoryId = m::RepositoryId::new([121; 16]);
struct Policies(Mutex<TokenPolicy>);
#[async_trait::async_trait]
impl OperatorPolicies for Policies {
    async fn current(&self) -> jj_store_service::auth::Result<TokenPolicy> {
        Ok(self.0.lock().unwrap().clone())
    }
}
struct Fixture {
    root: tempfile::TempDir,
    connection: Connection,
    token: BearerToken,
    client: NativeClient,
    server: tokio::task::JoinHandle<()>,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.server.abort();
    }
}
impl Fixture {
    async fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        let mut policy = TokenPolicy::new(Scope::Operator);
        let operator = policy
            .issue(
                m::PrincipalId::new([122; 16]),
                Permission::CreateRepository,
                None,
            )
            .unwrap();
        let host = RepositoryHost::new(
            Arc::new(MemoryStore::default()),
            Arc::new(Policies(Mutex::new(policy))),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
            32 * 1024 * 1024,
        )
        .unwrap();
        let token = BearerToken::generate().unwrap();
        host.create(&operator, REPO, m::EpochId::new([123; 16]), &token)
            .await
            .unwrap();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let connection = Connection::new(
            &format!("http://{}", listener.local_addr().unwrap()),
            REPO,
            true,
            None,
        )
        .unwrap();
        let server = tokio::spawn(async move {
            NativeRpc::hosted(host, Limits::default())
                .unwrap()
                .serve(listener, None, true, std::future::pending())
                .await
                .unwrap();
        });
        let client = connection.connect_with(token.clone()).await.unwrap();
        fs::write(
            root.path().join("jj.toml"),
            "[user]\nname='Attachment Test'\nemail='attachment@example.invalid'\n[operation]\nhostname='attachment-test'\nusername='test'\n",
        )
        .unwrap();
        Self {
            root,
            connection,
            token,
            client,
            server,
        }
    }
    fn settings(&self) -> UserSettings {
        let mut config = StackedConfig::with_defaults();
        for layer in jj_cli::config::default_config_layers() {
            config.add_layer(layer);
        }
        config.add_layer(
            ConfigLayer::parse(
                ConfigSource::User,
                &fs::read_to_string(self.root.path().join("jj.toml")).unwrap(),
            )
            .unwrap(),
        );
        UserSettings::from_config(config).unwrap()
    }
    async fn attachment(&self, name: &str) -> Attachment {
        Attachment::open(
            self.connection.clone(),
            self.token.clone(),
            self.root.path().join(name),
            Some(self.root.path().join(format!("{name}-cache"))),
            Some(name.into()),
            Some(self.root.path().join(format!("{name}-state"))),
        )
        .await
        .unwrap()
    }
    fn cli(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_jj"))
            .current_dir(self.root.path())
            .env("JJ_CONFIG", self.root.path().join("jj.toml"))
            .env("JJ_STORE_TOKEN", self.token.expose().as_str())
            .env_remove("JJ_STORE_PAGER")
            .args(["--no-pager", "--color=never"])
            .args(args)
            .output()
            .unwrap()
    }
    fn retry(&self, name: &str) -> Output {
        self.cli(&[
            "store",
            "attach",
            name,
            "--endpoint",
            &self.connection.endpoint,
            "--repo-id",
            &REPO.to_string(),
            "--insecure-loopback",
            "--workspace",
            name,
            "--cache",
            &format!("{name}-cache"),
            "--state-dir",
            &format!("{name}-state"),
            "--format=json",
        ])
    }
}
fn success(output: Output) -> String {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn accepted_attachment_retries_original_operation_after_lost_ack_and_later_publication() {
    let f = Fixture::new().await;
    let settings = f.settings();
    let mut attachment = f.attachment("first").await;
    attachment.prepare(&settings).await.unwrap();
    let pending = attachment.pending().unwrap().clone();
    f.client.publish(pending.old, pending.new).await.unwrap();
    // Discard acknowledgement: journal still says unpublished, and no .jj exists.
    drop(attachment);
    assert!(!f.root.path().join("first/.jj").exists());
    let mut other = f.attachment("second").await;
    other.prepare(&settings).await.unwrap();
    other.finish(&settings).await.unwrap();
    drop(other);
    let heads = f.client.heads().await.unwrap();
    fs::create_dir_all(f.root.path().join("first")).unwrap();
    fs::write(
        f.root.path().join("first/local.txt"),
        "preserve unsnapshotted bytes",
    )
    .unwrap();
    let result: serde_json::Value = serde_json::from_str(&success(f.retry("first"))).unwrap();
    assert_eq!(result["operation"], pending.new.to_string());
    assert_eq!(f.client.heads().await.unwrap(), heads);
    assert_eq!(
        fs::read(f.root.path().join("first/local.txt")).unwrap(),
        b"preserve unsnapshotted bytes"
    );
    success(f.cli(&["-R", "first", "describe", "-m", "after resumed attachment"]));
    let changed = f.client.heads().await.unwrap();
    success(f.retry("first"));
    assert_eq!(f.client.heads().await.unwrap(), changed);
    let journal = fs::read_to_string(f.root.path().join("first-state/attachment.json")).unwrap();
    assert!(!journal.contains(f.token.expose().as_str()));
    assert!(!f.root.path().join("first/.git").exists());
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn prepared_attachment_survives_cache_loss_and_rejects_changed_retry_identity() {
    let f = Fixture::new().await;
    let mut attachment = f.attachment("work").await;
    attachment.prepare(&f.settings()).await.unwrap();
    let pending = attachment.pending().unwrap().clone();
    // A second process cannot own this journal concurrently.
    assert!(
        Attachment::open(
            f.connection.clone(),
            f.token.clone(),
            f.root.path().join("work"),
            None,
            None,
            Some(f.root.path().join("work-state"))
        )
        .await
        .is_err()
    );
    drop(attachment);
    fs::remove_dir_all(f.root.path().join("work-cache")).unwrap();
    assert!(
        Attachment::open(
            f.connection.clone(),
            f.token.clone(),
            f.root.path().join("work"),
            None,
            Some("different".into()),
            Some(f.root.path().join("work-state"))
        )
        .await
        .is_err()
    );
    let value: serde_json::Value = serde_json::from_str(&success(f.retry("work"))).unwrap();
    assert_eq!(value["operation"], pending.new.to_string());
    let before = f.client.heads().await.unwrap();
    let journal = f.root.path().join("work-state/attachment.json");
    let mut record: serde_json::Value =
        serde_json::from_slice(&fs::read(&journal).unwrap()).unwrap();
    record["complete"] = false.into();
    fs::write(&journal, serde_json::to_vec(&record).unwrap()).unwrap();
    success(f.retry("work"));
    assert_eq!(f.client.heads().await.unwrap(), before);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn revoked_attachment_cannot_replay_receipt_or_expose_physical_metadata() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    let f = Fixture::new().await;
    let mut attachment = f.attachment("revoked").await;
    attachment.prepare(&f.settings()).await.unwrap();
    let pending = attachment.pending().unwrap().clone();
    f.client.publish(pending.old, pending.new).await.unwrap();
    drop(attachment);
    let other = BearerToken::generate().unwrap();
    let snapshot = f.client.tokens().await.unwrap().snapshot;
    let receipt = f
        .client
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([124; 16]),
            expected: snapshot,
            change: TokenChange::Grant {
                credential: other.clone(),
                principal: m::PrincipalId::new([125; 16]),
                permission: Permission::Admin,
                expires_at: None,
            },
        })
        .await
        .unwrap()
        .unwrap();
    let admin = f.connection.connect_with(other).await.unwrap();
    admin
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([126; 16]),
            expected: receipt.accepted,
            change: TokenChange::Revoke {
                token: f.token.id(),
            },
        })
        .await
        .unwrap()
        .unwrap();
    let heads = admin.heads().await.unwrap();
    assert!(!f.retry("revoked").status.success());
    assert!(!f.root.path().join("revoked/.jj").exists());
    assert_eq!(admin.heads().await.unwrap(), heads);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn recovery_preserves_unowned_physical_metadata_stage() {
    let f = Fixture::new().await;
    let mut attachment = f.attachment("work").await;
    attachment.prepare(&f.settings()).await.unwrap();
    let stage = f.root.path().join("work/.jj-store-bootstrap");
    fs::create_dir_all(&stage).unwrap();
    fs::write(stage.join("user-data"), "must survive").unwrap();
    assert!(attachment.finish(&f.settings()).await.is_err());
    assert_eq!(fs::read(stage.join("user-data")).unwrap(), b"must survive");
    assert!(!f.root.path().join("work/.jj").exists());
}
