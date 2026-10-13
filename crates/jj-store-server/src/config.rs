use crate::{Error, Result, policy};
use jj_store_jj::policy::Protection;
use jj_store_model as m;
use jj_store_object::s3::{self, S3Store};
use jj_store_service::{administration::RepositoryHost, auth::SystemClock, native::SessionLimits};
use std::{path::Path, sync::Arc, time::Duration};

mod types;
pub use types::*;

pub const EXAMPLE: &str = include_str!("config/format-2-example.toml");

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
        if format != 2 {
            return Err(Error::Config("unsupported configuration format"));
        }
        let mut config = toml::from_str::<Config>(text)
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

    pub(crate) fn resolve_paths(&mut self, base: &Path) -> Result<()> {
        self.state_dir = policy::outside_checkout(&base.join(&self.state_dir))?;
        self.operator_policy = policy::outside_checkout(&base.join(&self.operator_policy))?;
        if let Some(tls) = &mut self.tls {
            tls.certificate = base.join(&tls.certificate);
            tls.private_key = policy::outside_checkout(&base.join(&tls.private_key))?;
        }
        self.web_root = self.web_root.take().map(|root| base.join(root));
        self.agent_capture.resolve_paths(base);
        if let Some(remote) = &mut self.remote_agents {
            remote.acp_executable = base.join(&remote.acp_executable);
            remote.jj_executable = base.join(&remote.jj_executable);
        }
        for replication in &mut self.replication {
            replication.ca_certificate = replication
                .ca_certificate
                .take()
                .map(|certificate| base.join(certificate));
        }
        match &mut self.access {
            Access::Authenticated {
                installation,
                catalog,
            } => {
                *installation = policy::outside_checkout(&base.join(&*installation))?;
                *catalog = policy::outside_checkout(&base.join(&*catalog))?;
            }
            Access::LocalTrusted {
                installation,
                catalog,
                rpc_socket,
                ..
            } => {
                *installation = policy::outside_checkout(&base.join(&*installation))?;
                *catalog = policy::outside_checkout(&base.join(&*catalog))?;
                *rpc_socket = policy::outside_checkout(&base.join(&*rpc_socket))?;
            }
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
        if self.workflow_hosts.len() > 64 {
            return Err(Error::Config("workflow host admission limit"));
        }
        self.validate_common()?;
        if let Some(remote) = &self.remote_agents {
            remote.validate()?;
            if !matches!(self.access, Access::Authenticated { .. })
                || matches!(self.storage, Storage::Filesystem)
            {
                return Err(Error::Config(
                    "remote execution requires authenticated native authority",
                ));
            }
        }
        self.agent_capture.validate(&self.access, &self.storage)?;
        match &self.storage {
            Storage::Filesystem => {
                if !matches!(
                    self.access,
                    Access::LocalTrusted {
                        permission: jj_store_service::auth::Permission::Admin,
                        ..
                    }
                ) || !self.bind.ip().is_loopback()
                    || !self.backup.is_empty()
                    || !self.search.is_empty()
                    || !self.replication.is_empty()
                {
                    return Err(Error::Config(
                        "filesystem mode requires loopback LocalTrusted admin without native adapters",
                    ));
                }
            }
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
        validate_replication(&self.replication)?;
        Ok(())
    }

    fn validate_common(&self) -> Result<()> {
        if self.development_browser_annotations
            && (!self.bind.ip().is_loopback()
                || !matches!(self.access, Access::LocalTrusted { .. })
                || self.web_root.is_none())
        {
            return Err(Error::Config(
                "development browser annotations require a loopback LocalTrusted browser bundle",
            ));
        }
        for path in [&self.state_dir, &self.operator_policy] {
            if !path.is_absolute() {
                return Err(Error::Config("control paths must be absolute"));
            }
            policy::outside_checkout(path)?;
        }
        let (installation, catalog) = match &self.access {
            Access::Authenticated {
                installation,
                catalog,
            }
            | Access::LocalTrusted {
                installation,
                catalog,
                ..
            } => (installation, catalog),
        };
        for path in [installation, catalog] {
            if !path.is_absolute() {
                return Err(Error::Config("access registry paths must be absolute"));
            }
            policy::outside_checkout(path)?;
        }
        if let Access::LocalTrusted {
            rpc_socket,
            permission,
            ..
        } = &self.access
        {
            if !rpc_socket.is_absolute() {
                return Err(Error::Config("local access paths must be absolute"));
            }
            policy::outside_checkout(rpc_socket)?;
            if !self.bind.ip().is_loopback()
                || !matches!(
                    permission,
                    jj_store_service::auth::Permission::Read
                        | jj_store_service::auth::Permission::Write
                        | jj_store_service::auth::Permission::Admin
                )
            {
                return Err(Error::Config("local access transport or permission"));
            }
        }
        if let Some(tls) = &self.tls {
            policy::outside_checkout(&tls.private_key)?;
        }
        if self.format != 2
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
        crate::search::validate(
            &self.search,
            matches!(self.access, Access::Authenticated { .. }),
        )
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
        let host = RepositoryHost::new(
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
        .map_err(|_| Error::Config("native host limits"))?;
        let mut runtime = host.runtime_policy().current();
        runtime.concurrent = self.limits.concurrent_requests;
        host.runtime_policy().replace(runtime);
        Ok(host)
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

fn validate_replication(entries: &[Replication]) -> Result<()> {
    if entries.len() > 8 {
        return Err(Error::Config("replication repository limit"));
    }
    let mut repositories = std::collections::BTreeSet::new();
    let mut credential_names = std::collections::BTreeSet::new();
    for entry in entries {
        let parsed = url::Url::parse(&entry.upstream)
            .map_err(|_| Error::Config("replication upstream URL"))?;
        let loopback = match parsed.host() {
            Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
            Some(url::Host::Ipv6(ip)) => ip.is_loopback(),
            _ => false,
        };
        let suffix = entry
            .token_environment
            .strip_prefix("JJ_STORE_REPLICATION_TOKEN_");
        let credential = credential_names.insert(&entry.token_environment)
            && entry.token_environment.len() <= 128
            && suffix.is_some_and(|suffix| {
                !suffix.is_empty()
                    && suffix.bytes().all(|byte| {
                        byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_'
                    })
            });
        let limits = entry.limits;
        if !repositories.insert(entry.repository)
            || !credential
            || !parsed.username().is_empty()
            || parsed.password().is_some()
            || parsed.query().is_some()
            || parsed.fragment().is_some()
            || parsed.path() != "/"
            || !(parsed.scheme() == "https"
                || (parsed.scheme() == "http" && entry.insecure_loopback && loopback))
            || !(1..=128).contains(&limits.max_batch_origins)
            || !(1..=1_000_000).contains(&limits.max_closure_entries)
            || !(1..=1_000_000).contains(&limits.max_objects_per_step)
            || limits.max_bytes_per_step == 0
            || limits.max_bytes_per_step > 2 * 1024 * 1024 * 1024
            || limits.max_native_object_bytes < m::envelope::HEADER_LEN as u64
            || limits.max_native_object_bytes > jj_store_object::staging::MAX_SINGLE_PUT
            || !(10..=60_000).contains(&limits.poll_millis)
            || !(1..=60_000).contains(&limits.retry_initial_millis)
            || !(1..=3600).contains(&limits.retry_max_seconds)
            || Duration::from_millis(limits.retry_initial_millis)
                > Duration::from_secs(limits.retry_max_seconds)
            || !(1..=1024).contains(&limits.circuit_failures)
            || !(1..=3600).contains(&limits.circuit_cooldown_seconds)
            || !(1..=300).contains(&limits.timeout_seconds)
        {
            return Err(Error::Config(
                "replication repository, credential, transport or worker limits",
            ));
        }
    }
    Ok(())
}
