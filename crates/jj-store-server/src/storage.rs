//! Ownership boundary for authoritative storage runtimes.
use crate::{
    Error, Result,
    config::{Config, CredentialReference, Storage},
    installation::{self, Identity, InternalCredentials, Ownership},
    policy,
};
use jj_store_object::{
    Condition, Key, ObjectStore as _, StagedUpload, WriteOutcome,
    s3::{self, S3Store},
};
use serde::Serialize;
use std::{fs::File, net::SocketAddr, sync::Arc, time::Duration};

mod embedded;
pub use embedded::{EmbeddedStorageConfig, EmbeddedStorageRuntime};

pub struct AuthoritativeStorage {
    store: Arc<S3Store>,
    embedded: Option<EmbeddedStorageRuntime>,
    _lock: Option<File>,
}

impl AuthoritativeStorage {
    pub async fn start(config: &Config) -> Result<Self> {
        config.validate()?;
        match config.storage {
            Storage::EmbeddedRustfs { .. } if !config.state_dir.is_dir() => {
                return Err(Error::Config("managed installation state is missing"));
            }
            Storage::ExternalS3 { .. } => std::fs::create_dir_all(&config.state_dir)?,
            _ => {}
        }
        match &config.storage {
            Storage::ExternalS3 { credentials, .. } => {
                let store = Arc::new(
                    S3Store::new(config.s3_config(resolve_credentials(credentials)?)?)
                        .map_err(|_| Error::Config("storage client"))?,
                );
                Ok(Self {
                    store,
                    embedded: None,
                    _lock: None,
                })
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
                let lock = policy::exclusive_lock(&config.state_dir.join("server.lock"))?;
                let identity: Identity = installation::read_private_json(installation)?;
                let credentials: InternalCredentials =
                    installation::read_private_json(credentials)?;
                let owner: Ownership = installation::read_private_json(ownership)?;
                validate_owned_state(&identity, &credentials, &owner, volume, bucket, region)?;
                let address = concrete_address(address)?;
                let runtime = EmbeddedStorageRuntime::start(EmbeddedStorageConfig {
                    runtime_executable: runtime_executable.clone(),
                    volume: volume.clone(),
                    address: address.to_string(),
                    access_key: credentials.access_key,
                    secret_key: credentials.secret_key,
                    region: region.clone(),
                    bucket: bucket.clone(),
                    max_object_bytes: (config.limits.max_blob_bytes
                        + jj_store_model::envelope::HEADER_LEN as u64)
                        .max(16 * 1024 * 1024),
                    timeout: Duration::from_secs(config.limits.request_seconds),
                })
                .await?;
                if let Err(error) =
                    verify_sentinel(runtime.store(), &identity, &owner, ownership).await
                {
                    runtime.shutdown().await;
                    return Err(error);
                }
                Ok(Self {
                    store: runtime.store(),
                    embedded: Some(runtime),
                    _lock: Some(lock),
                })
            }
        }
    }

    pub fn store(&self) -> Arc<S3Store> {
        self.store.clone()
    }

    pub async fn shutdown(mut self) -> Result<()> {
        if let Some(runtime) = self.embedded.take() {
            runtime.shutdown().await;
        }
        Ok(())
    }
}

fn resolve_credentials(reference: &CredentialReference) -> Result<s3::Credentials> {
    let CredentialReference::Environment {
        access_key,
        secret_key,
        session_token,
    } = reference
    else {
        return Err(Error::Config(
            "OS credential resolution is unavailable in this build",
        ));
    };
    let required = |name: &str| {
        std::env::var(name)
            .ok()
            .filter(|value| !value.is_empty() && value.len() <= 4096)
            .ok_or(Error::Config("server storage credentials unavailable"))
    };
    let session_token = session_token
        .as_ref()
        .and_then(|name| std::env::var(name).ok());
    if session_token
        .as_ref()
        .is_some_and(|token| token.is_empty() || token.len() > 16_384)
    {
        return Err(Error::Config("temporary storage credential length"));
    }
    Ok(s3::Credentials {
        access_key: required(access_key)?,
        secret_key: required(secret_key)?,
        session_token,
    })
}

fn concrete_address(value: &str) -> Result<SocketAddr> {
    let mut address: SocketAddr = value
        .parse()
        .map_err(|_| Error::Config("embedded storage address"))?;
    if !address.ip().is_loopback() {
        return Err(Error::Config("embedded storage must bind loopback"));
    }
    if address.port() == 0 {
        let listener = std::net::TcpListener::bind(address)?;
        address.set_port(listener.local_addr()?.port());
    }
    Ok(address)
}

fn validate_owned_state(
    identity: &Identity,
    credentials: &InternalCredentials,
    owner: &Ownership,
    volume: &std::path::Path,
    bucket: &str,
    region: &str,
) -> Result<()> {
    let valid_id = identity.installation.len() == 32
        && identity
            .installation
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit());
    if identity.version != 1
        || credentials.version != 1
        || credentials.access_key.is_empty()
        || credentials.secret_key.is_empty()
        || owner.version != 1
        || !valid_id
        || owner.installation != identity.installation
        || owner.volume != volume
        || owner.bucket != bucket
        || owner.region != region
        || !volume.is_dir()
    {
        return Err(Error::Config(
            "managed storage identity, credentials, volume or ownership",
        ));
    }
    Ok(())
}

#[derive(Serialize)]
struct Sentinel<'a> {
    version: u32,
    installation: &'a str,
    bucket: &'a str,
    region: &'a str,
}

async fn verify_sentinel(
    store: Arc<S3Store>,
    identity: &Identity,
    owner: &Ownership,
    ownership_path: &std::path::Path,
) -> Result<()> {
    let key = Key::new("_jj_store/v1/installation-sentinel")
        .map_err(|_| Error::Config("storage sentinel key"))?;
    let bytes = serde_json::to_vec(&Sentinel {
        version: 1,
        installation: &identity.installation,
        bucket: &owner.bucket,
        region: &owner.region,
    })
    .map_err(|_| Error::Config("storage sentinel serialization"))?;
    if owner.sentinel_installed {
        if !store
            .bucket_exists()
            .await
            .map_err(|_| Error::Service("managed storage bucket unavailable"))?
        {
            return Err(Error::Service("managed storage bucket is missing"));
        }
    } else {
        store
            .ensure_bucket()
            .await
            .map_err(|_| Error::Service("managed storage bucket unavailable"))?;
        let upload = StagedUpload::from_bytes(&bytes, 4096)
            .await
            .map_err(|_| Error::Service("storage sentinel staging"))?;
        match store
            .put(&key, Condition::Absent, &upload)
            .await
            .map_err(|_| Error::Service("storage sentinel publication"))?
        {
            WriteOutcome::Applied(_) | WriteOutcome::PreconditionFailed | WriteOutcome::Unknown => {
            }
        }
    }
    let observed = store
        .get(&key, None)
        .await
        .map_err(|_| Error::Service("managed storage sentinel is missing"))?
        .read_bounded(4096)
        .await
        .map_err(|_| Error::Service("managed storage sentinel is invalid"))?;
    if observed != bytes {
        return Err(Error::Service("managed storage sentinel does not match"));
    }
    if !owner.sentinel_installed {
        let mut updated = owner.clone();
        updated.sentinel_installed = true;
        installation::persist_ownership(ownership_path, &updated)?;
    }
    Ok(())
}
