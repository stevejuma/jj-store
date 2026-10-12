//! Conditional configuration saves, independent of runtime control gates.
mod inventory;
use crate::{
    capture_management::authority::Authority, config::Config, installation::Identity, policy,
};
use jj_store_model::PrincipalId;
use jj_store_service::server_settings::*;
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};
struct Inner {
    changes: jj_store_service::freshness::Signals,
    path: PathBuf,
    effective: Arc<std::sync::RwLock<Config>>,
    authority: Authority,
}
#[derive(Clone)]
pub struct Manager(Arc<Inner>);
fn convert<T: serde::de::DeserializeOwned>(value: &impl serde::Serialize) -> Result<T, Error> {
    serde_json::from_value(serde_json::to_value(value).map_err(|_| Error::Invalid)?)
        .map_err(|_| Error::Invalid)
}
fn wire(config: &Config) -> Result<ServerConfiguration, Error> {
    Ok(ServerConfiguration {
        service: convert(&config.limits)?,
        protection: convert(&config.protection)?,
        sync: convert(&config.replication)?,
    })
}
impl Manager {
    pub fn new(path: &Path, config: &Config, identity: &Identity) -> crate::Result<Self> {
        Ok(Self(Arc::new(Inner {
            changes: Default::default(),
            path: policy::outside_checkout(path)?,
            effective: Arc::new(std::sync::RwLock::new(config.clone())),
            authority: Authority::new(path, config, identity)
                .map_err(|_| crate::Error::Config("local settings authority"))?,
        })))
    }
    pub(crate) fn with_effective(mut self, effective: Arc<std::sync::RwLock<Config>>) -> Self {
        Arc::get_mut(&mut self.0)
            .expect("unshared settings")
            .effective = effective;
        self
    }
    fn admit(&self, actor: PrincipalId) -> Result<(), Error> {
        self.0
            .authority
            .configuration(actor)
            .map(|_| ())
            .map_err(|e| match e {
                jj_store_service::capture_management::Error::Denied => Error::Denied,
                _ => Error::Unavailable,
            })
    }
    fn bytes(&self) -> Result<Vec<u8>, Error> {
        policy::read_regular(&self.0.path, 64 * 1024).map_err(|_| Error::Unavailable)
    }
    fn read(&self) -> Result<(String, Config), Error> {
        let bytes = self.bytes()?;
        let config = Config::load(&self.0.path).map_err(|_| Error::Invalid)?;
        if self.bytes()? != bytes {
            return Err(Error::Stale);
        }
        Ok((hex::encode(Sha256::digest(&bytes)), config))
    }
    fn report(&self) -> Result<LocalServerSettings, Error> {
        let (revision, config) = self.read()?;
        let desired = wire(&config)?;
        let effective = wire(&*self.0.effective.read().map_err(|_| Error::Unavailable)?)?;
        Ok(LocalServerSettings {
            version: 1,
            revision,
            restart_required: desired != effective,
            desired,
            effective,
            sync_defaults: convert(&crate::config::ReplicationLimits::default())?,
            inventory: inventory::values(
                &config,
                &*self.0.effective.read().map_err(|_| Error::Unavailable)?,
            )?,
        })
    }
    fn persist(
        &self,
        actor: PrincipalId,
        command: SaveServerSettings,
    ) -> Result<LocalServerSettings, Error> {
        if command.version != 1
            || command.expected_revision.len() != 64
            || !command
                .expected_revision
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(Error::Invalid);
        }
        let mut lock = self.0.path.as_os_str().to_owned();
        lock.push(".settings.lock");
        let _lock = policy::exclusive_lock(Path::new(&lock)).map_err(|_| Error::Busy)?;
        self.admit(actor)?;
        let (revision, mut config) = self.read()?;
        if revision != command.expected_revision {
            return Err(Error::Stale);
        }
        let (key, value) = match command.update {
            ServerSettingsUpdate::Service(value) => {
                config.limits = convert(&value)?;
                ("limits", toml::Value::try_from(value))
            }
            ServerSettingsUpdate::Protection(value) => {
                config.protection = convert(&value)?;
                ("protection", toml::Value::try_from(value))
            }
            ServerSettingsUpdate::Sync(value) => {
                for entry in &value {
                    if entry.ca_certificate.as_ref().is_some_and(|s| {
                        !Path::new(s).is_absolute() || s.chars().any(char::is_control)
                    }) {
                        return Err(Error::Invalid);
                    }
                }
                config.replication = convert(&value)?;
                ("replication", toml::Value::try_from(value))
            }
        };
        config.validate().map_err(|_| Error::Invalid)?;
        let bytes = self.bytes()?;
        if hex::encode(Sha256::digest(&bytes)) != revision {
            return Err(Error::Stale);
        }
        let mut doc: toml::Value =
            toml::from_str(std::str::from_utf8(&bytes).map_err(|_| Error::Invalid)?)
                .map_err(|_| Error::Invalid)?;
        doc.as_table_mut()
            .ok_or(Error::Invalid)?
            .insert(key.into(), value.map_err(|_| Error::Invalid)?);
        let output = toml::to_string(&doc).map_err(|_| Error::Invalid)?;
        if output.len() > 64 * 1024 {
            return Err(Error::Invalid);
        }
        let parent = self.0.path.parent().ok_or(Error::Invalid)?;
        let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|_| Error::Unavailable)?;
        temp.write_all(output.as_bytes())
            .and_then(|()| temp.as_file().sync_all())
            .map_err(|_| Error::Unavailable)?;
        self.admit(actor)?;
        if self.read()?.0 != revision {
            return Err(Error::Stale);
        }
        temp.persist(&self.0.path).map_err(|_| Error::Unknown)?;
        File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|_| Error::Unknown)?;
        self.admit(actor).map_err(|_| Error::Unknown)?;
        self.report().map_err(|_| Error::Unknown)
    }
}
#[async_trait::async_trait]
impl ServerSettingsManager for Manager {
    fn freshness_witness(&self) -> Option<String> {
        Some(match self.bytes() {
            Ok(bytes) => hex::encode(Sha256::digest(bytes)),
            Err(_) => "unavailable".into(),
        })
    }
    fn freshness(
        &self,
    ) -> Option<tokio::sync::broadcast::Receiver<jj_store_service::freshness::Change>> {
        Some(self.0.changes.subscribe())
    }
    async fn status(&self, actor: PrincipalId) -> Result<LocalServerSettings, Error> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
            this.admit(actor)?;
            let report = this.report()?;
            this.admit(actor)?;
            Ok(report)
        })
        .await
        .map_err(|_| Error::Unavailable)?
    }
    async fn save(
        &self,
        actor: PrincipalId,
        request: SaveServerSettings,
    ) -> Result<LocalServerSettings, Error> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
            let result = this.persist(actor, request);
            // The blocking owner can settle after the HTTP caller has gone away.
            if result.is_ok() || matches!(result, Err(Error::Unknown)) {
                this.0.changes.send(jj_store_service::freshness::Change {
                    repository: None,
                    domain: jj_store_service::freshness::Domain::Configuration,
                    witness: this
                        .freshness_witness()
                        .unwrap_or_else(|| "unavailable".into()),
                    identity: None,
                    audience: None,
                });
            }
            result
        })
        .await
        .map_err(|_| Error::Unknown)?
    }
}
