use jj_store_model as m;
use jj_store_service::auth::Permission;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, net::SocketAddr, path::PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub format: u32,
    pub bind: SocketAddr,
    pub public_url: String,
    #[serde(default)]
    pub insecure_loopback: bool,
    pub state_dir: PathBuf,
    pub operator_policy: PathBuf,
    pub tls: Option<Tls>,
    pub web_root: Option<PathBuf>,
    #[serde(default)]
    pub development_browser_annotations: bool,
    pub access: Access,
    pub storage: Storage,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub backup: Vec<Backup>,
    #[serde(default)]
    pub limits: Limits,
    #[serde(default)]
    pub protection: Protected,
    #[serde(default)]
    pub search: Vec<crate::search::Repository>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replication: Vec<Replication>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Replication {
    pub repository: m::RepositoryId,
    pub upstream: String,
    #[serde(default)]
    pub insecure_loopback: bool,
    pub token_environment: String,
    pub ca_certificate: Option<PathBuf>,
    #[serde(default)]
    pub limits: ReplicationLimits,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ReplicationLimits {
    pub max_batch_origins: usize,
    pub max_closure_entries: usize,
    pub max_objects_per_step: usize,
    pub max_bytes_per_step: u64,
    pub max_native_object_bytes: u64,
    pub poll_millis: u64,
    pub retry_initial_millis: u64,
    pub retry_max_seconds: u64,
    pub circuit_failures: u32,
    pub circuit_cooldown_seconds: u64,
    pub timeout_seconds: u64,
}

impl Default for ReplicationLimits {
    fn default() -> Self {
        Self {
            max_batch_origins: 128,
            max_closure_entries: 100_000,
            max_objects_per_step: 100_000,
            max_bytes_per_step: 1024 * 1024 * 1024,
            max_native_object_bytes: 64 * 1024 * 1024,
            poll_millis: 5_000,
            retry_initial_millis: 500,
            retry_max_seconds: 60,
            circuit_failures: 8,
            circuit_cooldown_seconds: 60,
            timeout_seconds: 30,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Access {
    Authenticated {
        installation: PathBuf,
        catalog: PathBuf,
    },
    LocalTrusted {
        installation: PathBuf,
        catalog: PathBuf,
        rpc_socket: PathBuf,
        permission: Permission,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Storage {
    ExternalS3 {
        endpoint: String,
        bucket: String,
        region: String,
        #[serde(default)]
        insecure_loopback: bool,
        credentials: CredentialReference,
    },
    EmbeddedRustfs {
        runtime_executable: PathBuf,
        volume: PathBuf,
        installation: PathBuf,
        credentials: PathBuf,
        ownership: PathBuf,
        address: String,
        bucket: String,
        region: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "kebab-case", deny_unknown_fields)]
pub enum CredentialReference {
    Environment {
        access_key: String,
        secret_key: String,
        session_token: Option<String>,
    },
    OsCredential {
        service: String,
        account: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "driver", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Backup {
    WalCopy {
        name: String,
        endpoint: String,
        bucket: String,
        region: String,
        #[serde(default)]
        insecure_loopback: bool,
        credentials: CredentialReference,
    },
}

impl Backup {
    pub fn name(&self) -> &str {
        match self {
            Self::WalCopy { name, .. } => name,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tls {
    pub certificate: PathBuf,
    pub private_key: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Limits {
    pub concurrent_requests: usize,
    pub request_seconds: u64,
    pub max_blob_bytes: u64,
    pub inventory_nodes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            concurrent_requests: 8,
            request_seconds: 30,
            max_blob_bytes: 32 * 1024 * 1024,
            inventory_nodes: 100_000,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Protected {
    pub bookmarks: BTreeSet<String>,
    pub commits: BTreeSet<m::CommitId>,
}
