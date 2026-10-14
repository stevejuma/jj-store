use jj_store_object::memory::MemoryStore;
use jj_store_server::{
    capture_control_state::CaptureControlJournal,
    capture_management::{CaptureManagementBinding, Manager},
    config::{Config, EXAMPLE},
    installation::Identity,
};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    auth::{self, Permission, Scope, SystemClock, TokenPolicy},
    native::SessionLimits,
};
use serde_json::Value;
use std::{fs, os::unix::fs::PermissionsExt as _, process::Output, sync::Arc};

pub const GENERATION: &str = "abababababababababababababababab";
pub struct Fixture {
    pub root: tempfile::TempDir,
    pub cwd: tempfile::TempDir,
    pub config: Config,
    pub text: String,
    pub base: String,
    stop: Option<tokio::sync::oneshot::Sender<()>>,
    task: Option<tokio::task::JoinHandle<()>>,
}
struct Operators;
#[async_trait::async_trait]
impl OperatorPolicies for Operators {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(TokenPolicy::new(Scope::Operator))
    }
}
impl Fixture {
    pub async fn new(enabled: bool) -> Self {
        let root = tempfile::tempdir().unwrap();
        let text = EXAMPLE.replace(
            "mode = \"authenticated\"\ninstallation = \"./installation.json\"\ncatalog = \"./repositories.json\"",
            "mode = \"local-trusted\"\ninstallation = \"./installation.json\"\ncatalog = \"./repositories.json\"\nrpc_socket = \"./native.sock\"\npermission = \"admin\"",
        ) + &format!("\n[agent_capture]\nenabled = {enabled}\n[[agent_capture.harnesses]]\nprofile = \"codex-cli-jsonl-v1\"\nproduct_version = \"0.156.1\"\n");
        let path = root.path().join("server.toml");
        fs::write(&path, &text).unwrap();
        let config = Config::load(&path).unwrap();
        let identity = Identity {
            version: 1,
            installation: "ab".repeat(16),
        };
        let identity_path = root.path().join("installation.json");
        fs::write(&identity_path, serde_json::to_vec(&identity).unwrap()).unwrap();
        fs::set_permissions(identity_path, fs::Permissions::from_mode(0o600)).unwrap();
        let binding = enabled.then(|| {
            use std::os::unix::fs::DirBuilderExt as _;
            let parent = config.state_dir.join("agent-capture");
            fs::DirBuilder::new()
                .recursive(true)
                .mode(0o700)
                .create(&parent)
                .unwrap();
            let journal =
                CaptureControlJournal::create(&parent.join("control"), [0xab; 16]).unwrap();
            CaptureManagementBinding {
                generation: journal.current().unwrap().generation,
                journal,
                runtime: Default::default(),
                imports: None,
            }
        });
        let manager = Manager::new(&path, &config, &identity, binding).unwrap();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let host = RepositoryHost::new(
            Arc::new(MemoryStore::default()),
            Arc::new(Operators),
            Arc::new(SystemClock),
            Default::default(),
            SessionLimits::default(),
            8 * 1024 * 1024,
        )
        .unwrap();
        let api = jj_store_http::HttpApi::new(
            host,
            root.path().join("http-reads"),
            jj_store_http::Config {
                origin: base.clone(),
                access: jj_store_http::Access::LocalTrusted {
                    principal: identity.principal(b"loopback-http"),
                    permission: Permission::Admin,
                },
                local_capture: Some(Arc::new(manager)),
                ..Default::default()
            },
        )
        .unwrap();
        let (stop, stopped) = tokio::sync::oneshot::channel();
        let task = tokio::spawn(async move {
            axum::serve(listener, api.router())
                .with_graceful_shutdown(async {
                    let _ = stopped.await;
                })
                .await
                .unwrap();
        });
        Self {
            root,
            cwd: tempfile::tempdir().unwrap(),
            config,
            text,
            base,
            stop: Some(stop),
            task: Some(task),
        }
    }
    pub async fn run(&self, args: &[&str]) -> Output {
        run(self.cwd.path(), &self.base, args).await
    }
    pub fn bytes(&self) -> Vec<u8> {
        fs::read(
            self.config
                .state_dir
                .join("agent-capture/control/state.json"),
        )
        .unwrap()
    }
    pub async fn close(mut self) {
        let _ = self.stop.take().unwrap().send(());
        self.task.take().unwrap().await.unwrap();
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}
pub async fn run(cwd: &std::path::Path, base: &str, args: &[&str]) -> Output {
    run_format(cwd, base, "json", args).await
}
pub async fn run_format(cwd: &std::path::Path, base: &str, format: &str, args: &[&str]) -> Output {
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_jj-store-jj"));
    command
        .current_dir(cwd)
        .args([
            "--no-pager",
            "agent",
            "--format",
            format,
            "capture",
            "--server-url",
            base,
        ])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .env("HTTP_PROXY", "http://127.0.0.1:1")
        .env("ALL_PROXY", "http://127.0.0.1:1")
        .kill_on_drop(true);
    tokio::time::timeout(std::time::Duration::from_secs(15), command.output())
        .await
        .unwrap()
        .unwrap()
}
pub fn successful(output: Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}
