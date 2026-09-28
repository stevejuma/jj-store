use crate::{Error, Result, policy};
use jj_store_jj::policy::Protection;
use jj_store_model as m;
use jj_store_object::s3::{self, S3Store};
use jj_store_service::{administration::RepositoryHost, auth::SystemClock, native::SessionLimits};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

pub const EXAMPLE: &str = r#"format = 1
bind = "127.0.0.1:7443"
public_url = "http://127.0.0.1:7443/"
insecure_loopback = true
state_dir = "./state"
operator_policy = "./operators.json"
# Optional browser bundle built with pnpm web:build:
# web_root = "./web/dist"
# Shared deployments require both files and an https public_url:
# [tls]
# certificate = "./server.crt"
# private_key = "./server.key"

[storage]
endpoint = "http://127.0.0.1:9000/"
bucket = "jj-store"
region = "us-east-1"
insecure_loopback = true
# Credentials: JJ_STORE_S3_ACCESS_KEY / JJ_STORE_S3_SECRET_KEY.
# Optional temporary credential: JJ_STORE_S3_SESSION_TOKEN.

[limits]
concurrent_requests = 8
request_seconds = 30
max_blob_bytes = 33554432
inventory_nodes = 100000

[protection]
bookmarks = []
commits = []

# Optional background search. Repeat for each repository (maximum eight).
# [[search]]
# repository = "<full repository ID>"
# token_environment = "JJ_STORE_SEARCH_TOKEN_EXAMPLE"
# poll_millis = 1000
# timeout_seconds = 60
# index_bytes = 536870912
# snapshot_every = 32 # committed source positions; zero disables snapshots
# [search.source]
# index_source = true
# exclude_prefixes = ["private"]
# [search.source.scopes.main]
# kind = "bookmark"
# name = "main"
"#;
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
    pub storage: Storage,
    #[serde(default)]
    pub limits: Limits,
    #[serde(default)]
    pub protection: Protected,
    #[serde(default)]
    pub search: Vec<crate::search::Repository>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tls {
    pub certificate: PathBuf,
    pub private_key: PathBuf,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Storage {
    pub endpoint: String,
    pub bucket: String,
    pub region: String,
    #[serde(default)]
    pub insecure_loopback: bool,
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
impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let bytes = policy::read_regular(path, 64 * 1024)?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|_| Error::Config("configuration must be UTF-8"))?;
        let mut config: Self = toml::from_str(text)
            .map_err(|_| Error::Config("malformed or unknown TOML configuration field"))?;
        let base = path
            .canonicalize()?
            .parent()
            .ok_or(Error::Config("configuration path"))?
            .to_owned();
        config.state_dir = policy::outside_checkout(&base.join(&config.state_dir))?;
        config.operator_policy = policy::outside_checkout(&base.join(&config.operator_policy))?;
        if let Some(tls) = &mut config.tls {
            tls.certificate = base.join(&tls.certificate);
            tls.private_key = policy::outside_checkout(&base.join(&tls.private_key))?;
        }
        config.web_root = config.web_root.map(|root| base.join(root));
        config.validate()?;
        Ok(config)
    }
    pub fn validate(&self) -> Result<()> {
        for path in [&self.state_dir, &self.operator_policy] {
            if !path.is_absolute() {
                return Err(Error::Config(
                    "control paths must be absolute after configuration loading",
                ));
            }
            policy::outside_checkout(path)?;
        }
        if let Some(tls) = &self.tls {
            policy::outside_checkout(&tls.private_key)?;
        }
        if self.format != 1
            || !(1..=256).contains(&self.limits.concurrent_requests)
            || !(1..=300).contains(&self.limits.request_seconds)
            || !(1..=2 * 1024 * 1024 * 1024).contains(&self.limits.max_blob_bytes)
            || !(1..=1_000_000).contains(&self.limits.inventory_nodes)
            || self.protection.bookmarks.len() > 1024
            || self.protection.commits.len() > 100_000
            || self.protection.bookmarks.iter().any(|name| {
                name.is_empty() || name.len() > 1024 || name.chars().any(char::is_control)
            })
        {
            return Err(Error::Config(
                "format, resource budgets or protection rules",
            ));
        }
        if self.tls.is_none() && !(self.insecure_loopback && self.bind.ip().is_loopback()) {
            return Err(Error::Config(
                "TLS required except explicitly enabled loopback",
            ));
        }
        let url = url::Url::parse(&self.public_url).map_err(|_| Error::Config("public URL"))?;
        let loopback = match url.host() {
            Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
            Some(url::Host::Ipv6(ip)) => ip.is_loopback(),
            _ => false,
        };
        if !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || url.path() != "/"
            || (self.tls.is_some() && url.scheme() != "https")
            || (self.tls.is_none()
                && !(url.scheme() == "http" && loopback && self.insecure_loopback))
        {
            return Err(Error::Config(
                "public URL must match transport security and contain no credentials or path",
            ));
        }
        crate::search::validate(&self.search)?;
        // Constructor validation is pure; this dummy credential never reaches I/O.
        S3Store::new(self.storage_config(s3::Credentials {
            access_key: "validation".into(),
            secret_key: "validation".into(),
            session_token: None,
        }))
        .map_err(|_| Error::Config("storage endpoint, region or bucket"))?;
        Ok(())
    }
    fn storage_config(&self, credentials: s3::Credentials) -> s3::Config {
        s3::Config {
            endpoint: self.storage.endpoint.clone(),
            bucket: self.storage.bucket.clone(),
            region: self.storage.region.clone(),
            credentials,
            allow_http_loopback: self.storage.insecure_loopback,
            max_object_bytes: (self.limits.max_blob_bytes + m::envelope::HEADER_LEN as u64)
                .max(16 * 1024 * 1024),
            timeout: Duration::from_secs(self.limits.request_seconds),
        }
    }
    pub fn host(&self) -> Result<RepositoryHost> {
        self.validate()?;
        let required = |key: &str| {
            std::env::var(key)
                .ok()
                .filter(|v| !v.is_empty() && v.len() <= 4096)
                .ok_or(Error::Config("server storage credentials unavailable"))
        };
        let session_token = std::env::var("JJ_STORE_S3_SESSION_TOKEN").ok();
        if session_token
            .as_ref()
            .is_some_and(|t| t.is_empty() || t.len() > 16384)
        {
            return Err(Error::Config("temporary storage credential length"));
        }
        let store = Arc::new(
            S3Store::new(self.storage_config(s3::Credentials {
                access_key: required("JJ_STORE_S3_ACCESS_KEY")?,
                secret_key: required("JJ_STORE_S3_SECRET_KEY")?,
                session_token,
            }))
            .map_err(|_| Error::Config("storage client"))?,
        );
        let timeout = Duration::from_secs(self.limits.request_seconds);
        let mut limits = SessionLimits {
            inventory_nodes: self.limits.inventory_nodes,
            timeout,
            ..SessionLimits::default()
        };
        limits.publication.timeout = timeout;
        limits.policy.timeout = timeout;
        RepositoryHost::new(
            store,
            Arc::new(policy::PolicyFile::new(self.operator_policy.clone())?),
            Arc::new(SystemClock),
            Protection {
                commits: self.protection.commits.clone(),
                bookmarks: self.protection.bookmarks.clone(),
            },
            limits,
            self.limits.max_blob_bytes,
        )
        .map_err(|_| Error::Config("native host limits"))
    }
}
