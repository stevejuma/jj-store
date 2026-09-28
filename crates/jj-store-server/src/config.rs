use crate::{Error, Result, policy};
use jj_store_jj::policy::Protection;
use jj_store_model as m;
use jj_store_object::s3::{self, S3Store};
use jj_store_service::{administration::RepositoryHost, auth::SystemClock, native::SessionLimits};
use serde::Deserialize;
use std::{
    fs::{self, File, OpenOptions},
    io::Write as _,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

mod types;
pub use types::*;

pub const FORMAT_1_EXAMPLE: &str = include_str!("config/format-1-example.toml");
pub const EXAMPLE: &str = include_str!("config/format-2-example.toml");

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FormatOne {
    format: u32,
    bind: std::net::SocketAddr,
    public_url: String,
    #[serde(default)]
    insecure_loopback: bool,
    state_dir: PathBuf,
    operator_policy: PathBuf,
    tls: Option<Tls>,
    web_root: Option<PathBuf>,
    storage: FormatOneStorage,
    #[serde(default)]
    limits: Limits,
    #[serde(default)]
    protection: Protected,
    #[serde(default)]
    search: Vec<crate::search::Repository>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FormatOneStorage {
    endpoint: String,
    bucket: String,
    region: String,
    #[serde(default)]
    insecure_loopback: bool,
}

impl From<FormatOne> for Config {
    fn from(value: FormatOne) -> Self {
        Self {
            format: value.format,
            bind: value.bind,
            public_url: value.public_url,
            insecure_loopback: value.insecure_loopback,
            state_dir: value.state_dir,
            operator_policy: value.operator_policy,
            tls: value.tls,
            web_root: value.web_root,
            access: Access::Authenticated,
            storage: Storage::ExternalS3 {
                endpoint: value.storage.endpoint,
                bucket: value.storage.bucket,
                region: value.storage.region,
                insecure_loopback: value.storage.insecure_loopback,
                credentials: CredentialReference::jj_store_s3_environment(),
            },
            backup: Vec::new(),
            limits: value.limits,
            protection: value.protection,
            search: value.search,
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let bytes = policy::read_regular(path, 64 * 1024)?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|_| Error::Config("configuration must be UTF-8"))?;
        let value: toml::Value = toml::from_str(text)
            .map_err(|_| Error::Config("malformed or unknown TOML configuration field"))?;
        let format = value
            .get("format")
            .and_then(toml::Value::as_integer)
            .ok_or(Error::Config("configuration format"))?;
        let mut config = match format {
            1 => toml::from_str::<FormatOne>(text).map(Config::from),
            2 => toml::from_str::<Config>(text),
            _ => return Err(Error::Config("unsupported configuration format")),
        }
        .map_err(|_| Error::Config("malformed or unknown TOML configuration field"))?;
        let base = path
            .canonicalize()?
            .parent()
            .ok_or(Error::Config("configuration path"))?
            .to_owned();
        config.resolve_paths(&base)?;
        config.validate()?;
        Ok(config)
    }

    pub fn migrate(source: &Path, destination: &Path) -> Result<()> {
        if source == destination || destination.exists() {
            return Err(Error::Config(
                "migration destination must be a new separate file",
            ));
        }
        let mut config = Self::load(source)?;
        config.format = 2;
        let text = toml::to_string_pretty(&config)
            .map_err(|_| Error::Config("configuration serialization"))?;
        persist_new_private(destination, text.as_bytes())
    }

    fn resolve_paths(&mut self, base: &Path) -> Result<()> {
        self.state_dir = policy::outside_checkout(&base.join(&self.state_dir))?;
        self.operator_policy = policy::outside_checkout(&base.join(&self.operator_policy))?;
        if let Some(tls) = &mut self.tls {
            tls.certificate = base.join(&tls.certificate);
            tls.private_key = policy::outside_checkout(&base.join(&tls.private_key))?;
        }
        self.web_root = self.web_root.take().map(|root| base.join(root));
        if let Access::LocalTrusted {
            installation,
            catalog,
        } = &mut self.access
        {
            *installation = policy::outside_checkout(&base.join(&*installation))?;
            *catalog = policy::outside_checkout(&base.join(&*catalog))?;
        }
        if let Storage::EmbeddedRustfs {
            runtime_executable,
            volume,
            installation,
            credentials,
            ownership,
            ..
        } = &mut self.storage
        {
            *runtime_executable = base.join(&*runtime_executable);
            *volume = policy::outside_checkout(&base.join(&*volume))?;
            *installation = policy::outside_checkout(&base.join(&*installation))?;
            *credentials = policy::outside_checkout(&base.join(&*credentials))?;
            *ownership = policy::outside_checkout(&base.join(&*ownership))?;
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        self.validate_common()?;
        match &self.storage {
            Storage::ExternalS3 {
                endpoint,
                bucket,
                region,
                insecure_loopback,
                credentials,
            } => {
                validate_credentials(credentials)?;
                validate_s3(self, endpoint, bucket, region, *insecure_loopback)?;
            }
            Storage::EmbeddedRustfs {
                runtime_executable,
                volume,
                installation,
                credentials,
                ownership,
                address,
                bucket,
                region,
            } => {
                if !runtime_executable.is_absolute()
                    || [volume, installation, credentials, ownership]
                        .iter()
                        .any(|path| !path.is_absolute())
                {
                    return Err(Error::Config("embedded storage paths must be absolute"));
                }
                for path in [volume, installation, credentials, ownership] {
                    policy::outside_checkout(path)?;
                }
                let address: std::net::SocketAddr = address
                    .parse()
                    .map_err(|_| Error::Config("embedded storage address"))?;
                if !address.ip().is_loopback() {
                    return Err(Error::Config("embedded storage must bind loopback"));
                }
                validate_s3(
                    self,
                    &format!("http://{}", concrete_validation_address(address)),
                    bucket,
                    region,
                    true,
                )?;
            }
        }
        for backup in &self.backup {
            let Backup::WalCopy {
                name,
                endpoint,
                bucket,
                region,
                insecure_loopback,
                credentials,
            } = backup;
            if !bounded_name(name) {
                return Err(Error::Config("backup target name"));
            }
            validate_credentials(credentials)?;
            validate_s3(self, endpoint, bucket, region, *insecure_loopback)?;
        }
        let names: std::collections::BTreeSet<_> = self.backup.iter().map(Backup::name).collect();
        if names.len() != self.backup.len() || self.backup.len() > 16 {
            return Err(Error::Config("backup target bounds or duplicate name"));
        }
        if self.format == 1
            && (!matches!(self.access, Access::Authenticated)
                || !matches!(self.storage, Storage::ExternalS3 { .. })
                || !self.backup.is_empty())
        {
            return Err(Error::Config("format one deployment semantics"));
        }
        Ok(())
    }

    fn validate_common(&self) -> Result<()> {
        for path in [&self.state_dir, &self.operator_policy] {
            if !path.is_absolute() {
                return Err(Error::Config("control paths must be absolute"));
            }
            policy::outside_checkout(path)?;
        }
        if let Access::LocalTrusted {
            installation,
            catalog,
        } = &self.access
        {
            for path in [installation, catalog] {
                if !path.is_absolute() {
                    return Err(Error::Config("local access paths must be absolute"));
                }
                policy::outside_checkout(path)?;
            }
        }
        if let Some(tls) = &self.tls {
            policy::outside_checkout(&tls.private_key)?;
        }
        if !(1..=2).contains(&self.format)
            || !(1..=256).contains(&self.limits.concurrent_requests)
            || !(1..=300).contains(&self.limits.request_seconds)
            || !(1..=2 * 1024 * 1024 * 1024).contains(&self.limits.max_blob_bytes)
            || !(1..=1_000_000).contains(&self.limits.inventory_nodes)
            || self.protection.bookmarks.len() > 1024
            || self.protection.commits.len() > 100_000
            || self
                .protection
                .bookmarks
                .iter()
                .any(|name| !bounded_name(name))
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
        validate_public_url(self)?;
        crate::search::validate(&self.search)
    }

    pub(crate) fn s3_config(&self, credentials: s3::Credentials) -> Result<s3::Config> {
        let Storage::ExternalS3 {
            endpoint,
            bucket,
            region,
            insecure_loopback,
            ..
        } = &self.storage
        else {
            return Err(Error::Config("external S3 storage configuration"));
        };
        Ok(s3_config(
            self,
            endpoint,
            bucket,
            region,
            *insecure_loopback,
            credentials,
        ))
    }

    pub(crate) fn host(&self, store: Arc<S3Store>) -> Result<RepositoryHost> {
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

fn validate_public_url(config: &Config) -> Result<()> {
    let url = url::Url::parse(&config.public_url).map_err(|_| Error::Config("public URL"))?;
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
        || (config.tls.is_some() && url.scheme() != "https")
        || (config.tls.is_none()
            && !(url.scheme() == "http" && loopback && config.insecure_loopback))
    {
        return Err(Error::Config("public URL transport or authority"));
    }
    Ok(())
}

fn validate_s3(
    config: &Config,
    endpoint: &str,
    bucket: &str,
    region: &str,
    insecure: bool,
) -> Result<()> {
    S3Store::new(s3_config(
        config,
        endpoint,
        bucket,
        region,
        insecure,
        s3::Credentials {
            access_key: "validation".into(),
            secret_key: "validation".into(),
            session_token: None,
        },
    ))
    .map(|_| ())
    .map_err(|_| Error::Config("storage endpoint, region or bucket"))
}

fn s3_config(
    config: &Config,
    endpoint: &str,
    bucket: &str,
    region: &str,
    insecure: bool,
    credentials: s3::Credentials,
) -> s3::Config {
    s3::Config {
        endpoint: endpoint.into(),
        bucket: bucket.into(),
        region: region.into(),
        credentials,
        allow_http_loopback: insecure,
        max_object_bytes: (config.limits.max_blob_bytes + m::envelope::HEADER_LEN as u64)
            .max(16 * 1024 * 1024),
        timeout: Duration::from_secs(config.limits.request_seconds),
    }
}

fn concrete_validation_address(mut address: std::net::SocketAddr) -> std::net::SocketAddr {
    if address.port() == 0 {
        address.set_port(1);
    }
    address
}

fn bounded_name(value: &str) -> bool {
    !value.is_empty() && value.len() <= 1024 && !value.chars().any(char::is_control)
}

fn validate_credentials(reference: &CredentialReference) -> Result<()> {
    match reference {
        CredentialReference::Environment {
            access_key,
            secret_key,
            session_token,
        } => {
            for name in [Some(access_key), Some(secret_key), session_token.as_ref()]
                .into_iter()
                .flatten()
            {
                if !name.starts_with("JJ_STORE_")
                    || name.len() > 128
                    || name.bytes().any(|byte| {
                        !byte.is_ascii_uppercase() && !byte.is_ascii_digit() && byte != b'_'
                    })
                {
                    return Err(Error::Config("credential environment reference"));
                }
            }
        }
        CredentialReference::OsCredential { service, account } => {
            if !bounded_name(service) || !bounded_name(account) {
                return Err(Error::Config("OS credential reference"));
            }
        }
    }
    Ok(())
}

fn persist_new_private(path: &Path, bytes: &[u8]) -> Result<()> {
    let path = policy::outside_checkout(path)?;
    let parent = path
        .parent()
        .ok_or(Error::Config("configuration destination"))?;
    fs::create_dir_all(parent)?;
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let mut file = options.open(&path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    #[cfg(unix)]
    File::open(parent)?.sync_all()?;
    Ok(())
}
