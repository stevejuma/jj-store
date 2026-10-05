//! Bounded, credential-free physical-client connection records and lazy factories.
use crate::{credentials, remote};
use jj_lib::{backend::BackendLoadError, repo::StoreFactories};
use jj_store_jj::{
    backend::BackendLimits,
    factories::{NativeStores, PROFILE},
    operation_stores::{AuthorityResult, OperationLimits},
};
use jj_store_model::RepositoryId;
use jj_store_rpc::client::{NativeClient, Options};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::{Read as _, Write as _},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};

pub const CONNECTION_FILE: &str = "store-connection.json";
const MAX_CONNECTION: u64 = 64 * 1024;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Connection {
    pub format: u32,
    pub profile: String,
    pub endpoint: String,
    pub repository: RepositoryId,
    pub insecure_loopback: bool,
    pub ca_certificate: Option<PathBuf>,
    pub max_blob_bytes: u64,
    pub timeout_seconds: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unix_socket: Option<PathBuf>,
}
impl Connection {
    pub fn new(
        endpoint: &str,
        repository: RepositoryId,
        insecure_loopback: bool,
        ca_certificate: Option<PathBuf>,
    ) -> AuthorityResult<Self> {
        let value = Self {
            format: 1,
            profile: PROFILE.into(),
            endpoint: canonical_endpoint(endpoint, insecure_loopback)?,
            repository,
            insecure_loopback,
            ca_certificate,
            max_blob_bytes: 32 * 1024 * 1024,
            timeout_seconds: 30,
            unix_socket: None,
        };
        value.validate()?;
        Ok(value)
    }
    pub fn new_local(socket: PathBuf, repository: RepositoryId) -> AuthorityResult<Self> {
        let value = Self {
            format: 2,
            profile: PROFILE.into(),
            endpoint: "local".into(),
            repository,
            insecure_loopback: false,
            ca_certificate: None,
            max_blob_bytes: 32 * 1024 * 1024,
            timeout_seconds: 30,
            unix_socket: Some(socket),
        };
        value.validate()?;
        Ok(value)
    }
    pub fn validate(&self) -> AuthorityResult<()> {
        let authenticated = self.format == 1
            && self.unix_socket.is_none()
            && self.endpoint == canonical_endpoint(&self.endpoint, self.insecure_loopback)?;
        let local = self.format == 2
            && self.endpoint == "local"
            && !self.insecure_loopback
            && self.ca_certificate.is_none()
            && self
                .unix_socket
                .as_ref()
                .is_some_and(|path| path.is_absolute());
        if !(authenticated || local)
            || self.profile != PROFILE
            || !(1..=3600).contains(&self.timeout_seconds)
            || self.max_blob_bytes == 0
            || self.max_blob_bytes
                > jj_store_object::staging::MAX_SINGLE_PUT
                    - jj_store_model::envelope::HEADER_LEN as u64
            || self
                .ca_certificate
                .as_ref()
                .is_some_and(|path| !path.is_absolute())
        {
            return Err("invalid native connection record or profile".into());
        }
        Ok(())
    }
    pub fn read(cache: &Path) -> AuthorityResult<Self> {
        let data = bounded_read(&cache.join(CONNECTION_FILE), MAX_CONNECTION)?;
        let connection: Self =
            serde_json::from_slice(&data).map_err(|_| "invalid native connection document")?;
        connection.validate()?;
        let marker = format!(
            "jj-store-client-cache-v1\n{PROFILE}\n{}\n",
            connection.repository
        );
        for component in ["store", "op_store", "op_heads"] {
            if bounded_read(&cache.join(component).join("native-profile"), 1024)?
                != marker.as_bytes()
            {
                return Err("native cache repository/profile binding mismatch".into());
            }
        }
        Ok(connection)
    }
    pub fn write(&self, cache: &Path) -> AuthorityResult<()> {
        self.validate()?;
        let bytes = serde_json::to_vec_pretty(self)?;
        if bytes.len() as u64 > MAX_CONNECTION {
            return Err("native connection record exceeds limit".into());
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(cache.join(CONNECTION_FILE))?;
        file.write_all(&bytes)?;
        file.sync_all()?;
        Ok(())
    }
    pub async fn connect_with(
        &self,
        token: jj_store_service::auth::BearerToken,
    ) -> AuthorityResult<NativeClient> {
        self.validate()?;
        if self.unix_socket.is_some() {
            return Err("local trusted connections do not accept bearer credentials".into());
        }
        let ca_certificate_pem = self
            .ca_certificate
            .as_ref()
            .map(|path| bounded_read(path, 1024 * 1024))
            .transpose()?;
        Ok(NativeClient::connect(
            &self.endpoint,
            self.repository,
            token,
            Options {
                insecure_loopback: self.insecure_loopback,
                ca_certificate_pem,
                object_bytes: self.max_blob_bytes + jj_store_model::envelope::HEADER_LEN as u64,
                timeout: Duration::from_secs(self.timeout_seconds),
            },
        )
        .await?)
    }
    pub async fn connect(&self) -> AuthorityResult<NativeClient> {
        if let Some(socket) = &self.unix_socket {
            return Ok(NativeClient::connect_unix(
                socket,
                self.repository,
                Options {
                    object_bytes: self.max_blob_bytes + jj_store_model::envelope::HEADER_LEN as u64,
                    timeout: Duration::from_secs(self.timeout_seconds),
                    ..Options::default()
                },
            )
            .await?);
        }
        self.connect_with(credentials::resolve(self)?).await
    }
    pub async fn stores(&self) -> AuthorityResult<NativeStores> {
        let client = self.connect().await?;
        remote::stores(
            client,
            self.max_blob_bytes,
            BackendLimits::default(),
            OperationLimits::default(),
        )
    }
}
pub fn canonical_endpoint(endpoint: &str, insecure_loopback: bool) -> AuthorityResult<String> {
    if endpoint.len() > 4096 {
        return Err("native endpoint exceeds limit".into());
    }
    let parsed = url::Url::parse(endpoint).map_err(|_| "invalid native endpoint URL")?;
    let loopback = match parsed.host() {
        Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
        Some(url::Host::Ipv6(ip)) => ip.is_loopback(),
        _ => false,
    };
    if !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
        || parsed.path() != "/"
        || !(parsed.scheme() == "https"
            || (parsed.scheme() == "http" && insecure_loopback && loopback))
    {
        return Err("native endpoint requires TLS or explicit literal loopback, with no credentials or path".into());
    }
    Ok(parsed.to_string())
}
pub fn bounded_read(path: &Path, limit: u64) -> AuthorityResult<Vec<u8>> {
    let mut bytes = Vec::new();
    fs::File::open(path)?
        .take(limit + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err("local metadata exceeds byte limit".into());
    }
    Ok(bytes)
}
/// One cache per CLI invocation. Captured closures never install process-global
/// principal or repository state, and upstream factories handle other backends.
#[derive(Clone, Default)]
pub struct FactoryRegistry {
    entries: Arc<Mutex<BTreeMap<PathBuf, (NativeStores, remote::BridgeContext)>>>,
}
impl FactoryRegistry {
    pub fn context(&self, path: &Path) -> AuthorityResult<remote::BridgeContext> {
        self.entries
            .lock()
            .map_err(|_| "native factory lock poisoned")?
            .get(&path.canonicalize()?)
            .map(|(_, context)| context.clone())
            .ok_or_else(|| "native factory context missing".into())
    }
    pub fn factories(&self) -> StoreFactories {
        let loaded = self.entries.clone();
        let load = move |component: &Path| -> AuthorityResult<NativeStores> {
            let cache = component
                .parent()
                .ok_or("missing native cache directory")?
                .canonicalize()?;
            let mut entries = loaded.lock().map_err(|_| "native factory lock poisoned")?;
            if let Some((stores, _)) = entries.get(&cache) {
                return Ok(stores.clone());
            }
            let connection = Connection::read(&cache)?;
            let client = futures::executor::block_on(connection.connect())?;
            let context = remote::BridgeContext::new(client);
            let stores = remote::stores_with_context(
                context.clone(),
                connection.max_blob_bytes,
                BackendLimits::default(),
                OperationLimits::default(),
            )?;
            entries.insert(cache, (stores.clone(), context));
            Ok(stores)
        };
        let mut factories = StoreFactories::empty();
        let backend = load.clone();
        factories.add_backend(
            "jj-store",
            Box::new(move |settings, path| {
                backend(path)
                    .map_err(BackendLoadError)?
                    .factories()
                    .load_backend(settings, path)
                    .map_err(|e| BackendLoadError(e.into()))
            }),
        );
        let operations = load.clone();
        factories.add_op_store(
            "jj-store",
            Box::new(move |settings, path, root| {
                operations(path)
                    .map_err(BackendLoadError)?
                    .factories()
                    .load_op_store(settings, path, root)
                    .map_err(|e| BackendLoadError(e.into()))
            }),
        );
        factories.add_op_heads_store(
            "jj-store",
            Box::new(move |settings, path| {
                load(path)
                    .map_err(BackendLoadError)?
                    .factories()
                    .load_op_heads_store(settings, path)
                    .map_err(|e| BackendLoadError(e.into()))
            }),
        );
        factories
    }
}
pub fn lazy_factories() -> StoreFactories {
    FactoryRegistry::default().factories()
}
