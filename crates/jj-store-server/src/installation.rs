//! Installation-owned state for local access and managed RustFS authority.
use crate::{Error, Result, config, policy};
use jj_store_service::auth::{Scope, TokenPolicy};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest as _, Sha256};
use std::{
    fs::{self, File},
    io::Write as _,
    path::{Path, PathBuf},
};

pub const INSTALLATION_FILE: &str = "installation.json";
pub const CATALOG_FILE: &str = "repositories.json";
pub const CREDENTIALS_FILE: &str = "rustfs-credentials.json";
pub const OWNERSHIP_FILE: &str = "storage-owner.json";
pub const VOLUME_DIRECTORY: &str = "rustfs";
pub const CONFIG_FILE: &str = "server.toml";
pub const RPC_SOCKET_FILE: &str = "native.sock";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Identity {
    pub version: u32,
    pub installation: String,
}

impl Identity {
    pub fn load(path: &Path) -> Result<Self> {
        let identity: Self = read_private_json(path)?;
        if identity.version != 1
            || identity.installation.len() != 32
            || identity
                .installation
                .bytes()
                .any(|byte| !byte.is_ascii_digit() && !(b'a'..=b'f').contains(&byte))
        {
            return Err(Error::Config("invalid local installation identity"));
        }
        Ok(identity)
    }

    pub fn principal(&self, peer: &[u8]) -> jj_store_model::PrincipalId {
        let mut digest = Sha256::new_with_prefix(b"jj-store/local-principal/v1");
        digest.update(self.installation.as_bytes());
        digest.update((peer.len() as u64).to_be_bytes());
        digest.update(peer);
        let mut id = [0; 16];
        id.copy_from_slice(&digest.finalize()[..16]);
        jj_store_model::PrincipalId::new(id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct InternalCredentials {
    pub version: u32,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Ownership {
    pub version: u32,
    pub installation: String,
    pub volume: PathBuf,
    pub bucket: String,
    pub region: String,
    pub sentinel_installed: bool,
}

pub use crate::registry::Catalog;

#[derive(Clone, Debug, Serialize)]
pub struct InitializedLocal {
    pub version: u32,
    pub installation: String,
    pub state_dir: PathBuf,
    pub config: PathBuf,
}

#[derive(Clone, Debug, Serialize)]
pub struct InitializedRegistry {
    pub version: u32,
    pub installation: String,
    pub state_dir: PathBuf,
    pub installation_file: PathBuf,
    pub catalog: PathBuf,
}

pub fn initialize_registry(state_dir: &Path) -> Result<InitializedRegistry> {
    let state_dir = policy::outside_checkout(state_dir)?;
    if state_dir.exists() {
        return Err(Error::Config(
            "registry state directory must not already exist",
        ));
    }
    let parent = state_dir
        .parent()
        .ok_or(Error::Config("registry state directory parent"))?;
    private_directory(parent)?;
    let staging = tempfile::Builder::new()
        .prefix(".jj-store-registry-")
        .tempdir_in(parent)?;
    set_private_directory(staging.path())?;
    let identity = Identity {
        version: 1,
        installation: random_hex::<16>()?,
    };
    write_json(staging.path().join(INSTALLATION_FILE), &identity)?;
    write_json(
        staging.path().join(CATALOG_FILE),
        &Catalog::empty(&identity),
    )?;
    sync_directory(staging.path())?;
    fs::rename(staging.path(), &state_dir)?;
    let _moved = staging.keep();
    sync_directory(parent)?;
    Ok(InitializedRegistry {
        version: 1,
        installation: identity.installation,
        installation_file: state_dir.join(INSTALLATION_FILE),
        catalog: state_dir.join(CATALOG_FILE),
        state_dir,
    })
}

/// Create all configuration before atomic installation publication; no native credentials.
pub fn initialize_filesystem(state_dir: &Path) -> Result<InitializedLocal> {
    let state = policy::outside_checkout(state_dir)?;
    if state.exists() {
        return Err(Error::Config(
            "local state directory must not already exist",
        ));
    }
    let parent = state
        .parent()
        .ok_or(Error::Config("local state directory parent"))?;
    private_directory(parent)?;
    let staging = tempfile::Builder::new()
        .prefix(".jj-store-filesystem-")
        .tempdir_in(parent)?;
    set_private_directory(staging.path())?;
    let identity = Identity {
        version: 1,
        installation: random_hex::<16>()?,
    };
    write_json(staging.path().join(INSTALLATION_FILE), &identity)?;
    write_json(
        staging.path().join(CATALOG_FILE),
        &Catalog::empty(&identity),
    )?;
    write_private(
        &staging.path().join("operators.json"),
        &TokenPolicy::new(Scope::Operator)
            .encode()
            .map_err(|_| Error::Config("operator policy"))?,
    )?;
    let config = local_config(&state, config::Storage::Filesystem);
    let text = toml::to_string_pretty(&config)
        .map_err(|_| Error::Config("filesystem configuration serialization"))?;
    write_private(&staging.path().join(CONFIG_FILE), text.as_bytes())?;
    sync_directory(staging.path())?;
    fs::rename(staging.path(), &state)?;
    let _moved = staging.keep();
    sync_directory(parent)?;
    Ok(InitializedLocal {
        version: 1,
        installation: identity.installation,
        config: state.join(CONFIG_FILE),
        state_dir: state,
    })
}

pub fn initialize_local(state_dir: &Path, runtime_executable: &Path) -> Result<InitializedLocal> {
    let state_dir = policy::outside_checkout(state_dir)?;
    if state_dir.exists() {
        return Err(Error::Config(
            "local state directory must not already exist",
        ));
    }
    let runtime_executable = runtime_executable.canonicalize()?;
    if !runtime_executable.is_file() {
        return Err(Error::Config("embedded RustFS runtime executable"));
    }
    let parent = state_dir
        .parent()
        .ok_or(Error::Config("local state directory parent"))?;
    private_directory(parent)?;
    let staging = tempfile::Builder::new()
        .prefix(".jj-store-init-")
        .tempdir_in(parent)?;
    set_private_directory(staging.path())?;
    let volume = staging.path().join(VOLUME_DIRECTORY);
    fs::create_dir(&volume)?;
    set_private_directory(&volume)?;

    let installation = random_hex::<16>()?;
    let bucket = format!("jj-store-local-{installation}");
    let identity = Identity {
        version: 1,
        installation: installation.clone(),
    };
    let credentials = InternalCredentials {
        version: 1,
        access_key: random_hex::<16>()?,
        secret_key: random_hex::<32>()?,
    };
    let ownership = Ownership {
        version: 1,
        installation: installation.clone(),
        volume: state_dir.join(VOLUME_DIRECTORY),
        bucket: bucket.clone(),
        region: "us-east-1".into(),
        sentinel_installed: false,
    };
    write_json(staging.path().join(INSTALLATION_FILE), &identity)?;
    write_json(
        staging.path().join(CATALOG_FILE),
        &Catalog::empty(&identity),
    )?;
    write_json(staging.path().join(CREDENTIALS_FILE), &credentials)?;
    write_json(staging.path().join(OWNERSHIP_FILE), &ownership)?;
    write_private(
        &staging.path().join("operators.json"),
        &TokenPolicy::new(Scope::Operator)
            .encode()
            .map_err(|_| Error::Config("local operator policy"))?,
    )?;

    let config = local_config(
        &state_dir,
        config::Storage::EmbeddedRustfs {
            runtime_executable,
            volume: state_dir.join(VOLUME_DIRECTORY),
            installation: state_dir.join(INSTALLATION_FILE),
            credentials: state_dir.join(CREDENTIALS_FILE),
            ownership: state_dir.join(OWNERSHIP_FILE),
            address: "127.0.0.1:0".into(),
            bucket,
            region: "us-east-1".into(),
        },
    );
    let text = toml::to_string_pretty(&config)
        .map_err(|_| Error::Config("local configuration serialization"))?;
    write_private(&staging.path().join(CONFIG_FILE), text.as_bytes())?;
    sync_directory(&volume)?;
    sync_directory(staging.path())?;
    fs::rename(staging.path(), &state_dir)?;
    let _moved = staging.keep();
    sync_directory(parent)?;
    Ok(InitializedLocal {
        version: 1,
        installation,
        config: state_dir.join(CONFIG_FILE),
        state_dir,
    })
}

fn local_config(state: &Path, storage: config::Storage) -> config::Config {
    config::Config {
        format: 2,
        bind: "127.0.0.1:7443".parse().expect("constant address"),
        public_url: "http://127.0.0.1:7443/".into(),
        insecure_loopback: true,
        state_dir: state.into(),
        operator_policy: state.join("operators.json"),
        tls: None,
        web_root: None,
        development_browser_annotations: false,
        access: config::Access::LocalTrusted {
            installation: state.join(INSTALLATION_FILE),
            catalog: state.join(CATALOG_FILE),
            rpc_socket: state.join(RPC_SOCKET_FILE),
            permission: jj_store_service::auth::Permission::Admin,
        },
        storage,
        backup: Vec::new(),
        limits: config::Limits::default(),
        protection: config::Protected::default(),
        search: Vec::new(),
        replication: Vec::new(),
    }
}

pub(crate) fn read_private_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let bytes = read_private(path, 64 * 1024)?;
    serde_json::from_slice(&bytes).map_err(|_| Error::Config("invalid local installation state"))
}

pub(crate) fn persist_ownership(path: &Path, ownership: &Ownership) -> Result<()> {
    persist_private_json(path, ownership)
}

fn persist_private_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let parent = path.parent().ok_or(Error::Config("ownership path"))?;
    let bytes = serde_json::to_vec(value)
        .map_err(|_| Error::Config("local installation state serialization"))?;
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    set_private_file(temp.as_file())?;
    temp.write_all(&bytes)?;
    temp.as_file().sync_all()?;
    temp.persist(path).map_err(|error| Error::Io(error.error))?;
    sync_directory(parent)
}

fn read_private(path: &Path, limit: u64) -> Result<Vec<u8>> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        if fs::symlink_metadata(path)?.permissions().mode() & 0o077 != 0 {
            return Err(Error::Config("local installation file permissions"));
        }
    }
    policy::read_regular(path, limit)
}

fn write_json(path: PathBuf, value: &impl Serialize) -> Result<()> {
    let bytes =
        serde_json::to_vec(value).map_err(|_| Error::Config("local installation serialization"))?;
    write_private(&path, &bytes)
}

fn write_private(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}

fn private_directory(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
        set_private_directory(path)?;
    }
    Ok(())
}

fn set_private_directory(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    }
    Ok(())
}

fn set_private_file(file: &File) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        file.set_permissions(fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}

fn sync_directory(path: &Path) -> Result<()> {
    #[cfg(unix)]
    File::open(path)?.sync_all()?;
    Ok(())
}

fn random_hex<const N: usize>() -> Result<String> {
    let mut bytes = [0_u8; N];
    getrandom::fill(&mut bytes).map_err(|_| Error::Service("secure randomness unavailable"))?;
    Ok(hex::encode(bytes))
}
