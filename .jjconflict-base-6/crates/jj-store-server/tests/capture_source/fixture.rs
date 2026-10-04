use jj_store_capture::{
    agent_capture::AgentCaptureSpool,
    codex_activity::{ActiveCodexSource, ActivityPoll, CodexActivityProbe},
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner},
    profile::CaptureProfile,
    source_registry::{QualifiedCodexSource, SourceRegistry},
};
use jj_store_model::{EpochId, RepositoryId};
use jj_store_object::memory::MemoryStore;
use jj_store_server::{
    capture_admission::CaptureAdmission,
    capture_authority::CaptureAuthority,
    capture_source::{CapturePrivacyRoots, CaptureStepContext},
    config::{Config, EXAMPLE},
    installation::Identity,
    registry::{Catalog, Registry},
};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    auth::{self, Permission, Scope, SystemClock, TokenPolicy},
    local_management::{
        VerifiedCaptureWorkspace, VerifiedWorkspace, WorkspaceProfile, WorkspaceVerifier,
    },
    native::SessionLimits,
    registry::RepositoryEntry,
};
use serde_json::{Value, json};
use std::{
    fs,
    io::Write as _,
    os::unix::fs::PermissionsExt as _,
    path::{Path, PathBuf},
    sync::Arc,
};
pub const A: RepositoryId = RepositoryId::new([1; 16]);
pub const B: RepositoryId = RepositoryId::new([2; 16]);
pub const SECRET: &str = "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";
struct Operators;
#[async_trait::async_trait]
impl OperatorPolicies for Operators {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(TokenPolicy::new(Scope::Operator))
    }
}
struct Native {
    root: PathBuf,
}
impl WorkspaceVerifier for Native {
    fn inspect(
        &self,
        _: &str,
    ) -> Result<VerifiedWorkspace, jj_store_service::local_management::Error> {
        Err(jj_store_service::local_management::Error::Unavailable)
    }
    fn inspect_capture_directory(
        &self,
        directory: &str,
    ) -> Result<VerifiedCaptureWorkspace, jj_store_service::local_management::Error> {
        for (name, repository) in [("workspace-a", A), ("workspace-b", B)] {
            let path = self.root.join(name);
            if Path::new(directory).starts_with(&path) {
                return Ok(VerifiedCaptureWorkspace {
                    directory: directory.into(),
                    workspace: VerifiedWorkspace {
                        path: path.to_str().unwrap().into(),
                        metadata: path.join(".jj/repo").to_str().unwrap().into(),
                        repository,
                        profile: WorkspaceProfile::Native,
                    },
                    binding_digest: [repository.as_bytes()[0]; 32],
                });
            }
        }
        Err(jj_store_service::local_management::Error::Invalid)
    }
}
pub struct Fixture {
    pub root: tempfile::TempDir,
    pub authority: CaptureAuthority,
    pub host: RepositoryHost,
    pub admission: CaptureAdmission,
    pub sources: Arc<SourceRegistry>,
    pub spool: AgentCaptureSpool,
    pub transcript: PathBuf,
    identity: Identity,
    catalog_path: PathBuf,
}
impl Fixture {
    pub async fn new(max_bytes: u64) -> Self {
        let root = tempfile::tempdir().unwrap();
        let config_path = root.path().join("server.toml");
        fs::write(&config_path, EXAMPLE.replace("[access]\nmode = \"authenticated\"\ninstallation = \"./installation.json\"\ncatalog = \"./repositories.json\"", "[access]\nmode = \"local-trusted\"\ninstallation = \"./installation.json\"\ncatalog = \"./repositories.json\"\nrpc_socket = \"./native.sock\"\npermission = \"write\"") + r#"
[agent_capture]
enabled = true
[[agent_capture.harnesses]]
profile = "codex-cli-jsonl-v1"
product_version = "0.156.1"
"#).unwrap();
        let identity = Identity {
            version: 1,
            installation: "ab".repeat(16),
        };
        let path = root.path().join("installation.json");
        fs::write(&path, serde_json::to_vec(&identity).unwrap()).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
        let authority = CaptureAuthority::prepare(&Config::load(&config_path).unwrap())
            .unwrap()
            .unwrap();
        let catalog_path = root.path().join("repositories.json");
        fs::write(
            &catalog_path,
            serde_json::to_vec(&Catalog::empty(&identity)).unwrap(),
        )
        .unwrap();
        fs::set_permissions(&catalog_path, fs::Permissions::from_mode(0o600)).unwrap();
        let registry = Arc::new(Registry::new(catalog_path.clone(), identity.clone()).unwrap());
        let admission = CaptureAdmission::new(
            registry,
            Arc::new(Native {
                root: root.path().to_owned(),
            }),
        );
        let host = RepositoryHost::new(
            Arc::new(MemoryStore::default()),
            Arc::new(Operators),
            Arc::new(SystemClock),
            Default::default(),
            SessionLimits::default(),
            8 * 1024 * 1024,
        )
        .unwrap();
        for repository in [A, B] {
            host.create_local_trusted(
                repository,
                EpochId::new([repository.as_bytes()[0] + 2; 16]),
                Permission::Admin,
            )
            .await
            .unwrap();
        }
        let sources =
            Arc::new(SourceRegistry::open(&root.path().join("sources"), [0xab; 16], 4).unwrap());
        let spool = AgentCaptureSpool::open(&root.path().join("spool"), max_bytes).unwrap();
        let transcripts = root.path().join("transcripts");
        fs::create_dir(&transcripts).unwrap();
        let transcript = transcripts.join("session.jsonl");
        let this = Self {
            root,
            authority,
            host,
            admission,
            sources,
            spool,
            transcript,
            identity,
            catalog_path,
        };
        this.catalog(&[A, B]);
        fs::write(&this.transcript, format!("{}\n",json!({"type":"session_meta","payload":{"id":"00000000-0000-0000-0000-000000000001","cwd":this.workspace(A),"cli_version":"0.156.1","source":"cli","originator":"codex_cli_rs"}}))).unwrap();
        this
    }
    pub fn catalog(&self, repositories: &[RepositoryId]) {
        let mut catalog = Catalog::empty(&self.identity);
        catalog.repositories = repositories
            .iter()
            .map(|id| RepositoryEntry {
                id: *id,
                name: format!("repo-{id}"),
            })
            .collect();
        fs::write(&self.catalog_path, serde_json::to_vec(&catalog).unwrap()).unwrap();
    }
    pub fn without_native_authority(&mut self) {
        self.host = RepositoryHost::new(
            Arc::new(MemoryStore::default()),
            Arc::new(Operators),
            Arc::new(SystemClock),
            Default::default(),
            SessionLimits::default(),
            8 * 1024 * 1024,
        )
        .unwrap();
    }
    pub fn workspace(&self, repository: RepositoryId) -> PathBuf {
        self.root.path().join(if repository == A {
            "workspace-a"
        } else {
            "workspace-b"
        })
    }
    pub fn privacy(&self) -> CapturePrivacyRoots {
        CapturePrivacyRoots {
            home: self.root.path().join("home"),
            temporary: self.root.path().join("temporary"),
            cache: std::env::var_os("JJ_STORE_TRANSCRIPT_CACHE")
                .map(PathBuf::from)
                .unwrap_or_else(|| self.root.path().join("rules")),
        }
    }
    pub fn context(&mut self) -> CaptureStepContext<'_> {
        CaptureStepContext {
            authority: &self.authority,
            host: &self.host,
            admission: &self.admission,
            spool: &mut self.spool,
        }
    }
    pub fn append(&self, value: Value) {
        self.append_bytes(format!("{value}\n").as_bytes());
    }
    pub fn append_bytes(&self, bytes: &[u8]) {
        fs::OpenOptions::new()
            .append(true)
            .open(&self.transcript)
            .unwrap()
            .write_all(bytes)
            .unwrap();
    }
    pub fn qualified(&self) -> QualifiedCodexSource {
        let mut scanner = CodexDirectoryScanner::new(
            ApprovedCodexRoot::open(self.transcript.parent().unwrap())
                .unwrap()
                .unwrap(),
        )
        .unwrap();
        QualifiedCodexSource::inspect(
            CaptureProfile::CodexCliJsonlV1,
            scanner.step(100, 100).unwrap().candidates.remove(0),
        )
        .unwrap()
        .unwrap()
    }
    pub fn start(&self) -> ActiveCodexSource {
        let mut probe = CodexActivityProbe::arm(self.qualified()).unwrap();
        assert_eq!(probe.poll().unwrap(), ActivityPoll::Waiting);
        self.append(
            json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"first"}}),
        );
        assert_eq!(probe.poll().unwrap(), ActivityPoll::Active);
        probe.proof().unwrap().unwrap()
    }
    pub fn message(&self, id: &str, text: &str) {
        self.append(json!({"type":"response_item","payload":{"type":"message","id":id,"role":"assistant","content":[{"type":"output_text","text":text}]}}));
    }
}
