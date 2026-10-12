//! Restart-bound remote definitions; fresh operator authorization surrounds every save.
use crate::{config::Config, policy};
use jj_store_service::{
    agent_administration as a,
    auth::{BearerToken, Clock, Permission, SystemClock},
    directory::{Caller, Directory},
    server_settings::Error,
};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Clone)]
pub struct Manager {
    path: PathBuf,
    policy: policy::PolicyFile,
    effective: Option<a::Configuration>,
    directory: Arc<dyn Directory>,
}
fn wire<T: serde::de::DeserializeOwned>(value: &impl serde::Serialize) -> Result<T, Error> {
    serde_json::from_value(serde_json::to_value(value).map_err(|_| Error::Invalid)?)
        .map_err(|_| Error::Invalid)
}
impl Manager {
    pub fn new(path: &Path, config: &Config, directory: Arc<dyn Directory>) -> crate::Result<Self> {
        if !matches!(config.access, crate::config::Access::Authenticated { .. })
            || matches!(config.storage, crate::config::Storage::Filesystem)
        {
            return Err(crate::Error::Config(
                "agent administration requires authenticated native storage",
            ));
        }
        Ok(Self {
            path: policy::outside_checkout(path)?,
            policy: policy::PolicyFile::new(config.operator_policy.clone())?,
            effective: wire(&config.remote_agents)
                .map_err(|_| crate::Error::Config("agent administration configuration"))?,
            directory,
        })
    }
    fn authorize(&self, token: &BearerToken) -> Result<jj_store_model::PrincipalId, Error> {
        self.policy
            .read()
            .map_err(|_| Error::Denied)?
            .authorize(
                token,
                Permission::ManageAgents,
                SystemClock.unix_seconds().map_err(|_| Error::Denied)?,
            )
            .map(|p| p.id())
            .map_err(|_| Error::Denied)
    }
    fn read(&self) -> Result<(String, Config, Vec<u8>), Error> {
        let bytes = policy::read_regular(&self.path, 64 * 1024).map_err(|_| Error::Unavailable)?;
        let config = Config::load(&self.path).map_err(|_| Error::Invalid)?;
        if policy::read_regular(&self.path, 64 * 1024).map_err(|_| Error::Unavailable)? != bytes {
            return Err(Error::Stale);
        }
        Ok((hex::encode(Sha256::digest(&bytes)), config, bytes))
    }
    fn persist(&self, token: &BearerToken, request: a::Save) -> Result<(), Error> {
        self.authorize(token)?;
        if request.expected_revision.len() != 64
            || !request
                .expected_revision
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(Error::Invalid);
        }
        let mut lock = self.path.as_os_str().to_owned();
        lock.push(".settings.lock");
        let _lock = policy::exclusive_lock(Path::new(&lock)).map_err(|_| Error::Busy)?;
        let (revision, mut config, bytes) = self.read()?;
        if revision != request.expected_revision {
            return Err(Error::Stale);
        }
        let remote: Option<crate::remote_agents::Configuration> = wire(&request.configuration)?;
        if let Some(remote) = &remote {
            if !Path::new(&remote.acp_executable).is_absolute()
                || !Path::new(&remote.jj_executable).is_absolute()
            {
                return Err(Error::Invalid);
            }
        }
        config.remote_agents = remote;
        config.validate().map_err(|_| Error::Invalid)?;
        let mut doc: toml::Value =
            toml::from_str(std::str::from_utf8(&bytes).map_err(|_| Error::Invalid)?)
                .map_err(|_| Error::Invalid)?;
        let table = doc.as_table_mut().ok_or(Error::Invalid)?;
        if let Some(value) = request.configuration {
            table.insert(
                "remote_agents".into(),
                toml::Value::try_from(value).map_err(|_| Error::Invalid)?,
            );
        } else {
            table.remove("remote_agents");
        }
        let output = toml::to_string(&doc).map_err(|_| Error::Invalid)?;
        if output.len() > 64 * 1024 {
            return Err(Error::Invalid);
        }
        let parent = self.path.parent().ok_or(Error::Invalid)?;
        let mut file = tempfile::NamedTempFile::new_in(parent).map_err(|_| Error::Unavailable)?;
        file.write_all(output.as_bytes())
            .and_then(|_| file.as_file().sync_all())
            .map_err(|_| Error::Unavailable)?;
        self.authorize(token)?;
        if self.read()?.0 != revision {
            return Err(Error::Stale);
        }
        file.persist(&self.path).map_err(|_| Error::Unknown)?;
        File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|_| Error::Unknown)?;
        self.authorize(token).map_err(|_| Error::Unknown)?;
        Ok(())
    }
}
#[async_trait::async_trait]
impl a::Administration for Manager {
    async fn status(&self, token: BearerToken) -> Result<a::Status, Error> {
        let this = self.clone();
        let actor = this.authorize(&token)?;
        let (revision, config, _) = tokio::task::spawn_blocking(move || this.read())
            .await
            .map_err(|_| Error::Unavailable)??;
        let principals = self
            .directory
            .identities(&Caller::Operator(actor))
            .await
            .map_err(|_| Error::Unavailable)?
            .into_iter()
            .filter(|b| {
                !b.revoked
                    && b.expires_at
                        .is_none_or(|at| SystemClock.unix_seconds().is_ok_and(|now| now < at))
            })
            .map(|b| b.principal)
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        self.authorize(&token)?;
        let desired: Option<a::Configuration> = wire(&config.remote_agents)?;
        Ok(a::Status {
            revision,
            restart_required: desired != self.effective,
            desired,
            effective: self.effective.clone(),
            principals,
        })
    }
    async fn save(&self, token: BearerToken, request: a::Save) -> Result<a::Status, Error> {
        let this = self.clone();
        let credential = token.clone();
        tokio::task::spawn_blocking(move || this.persist(&credential, request))
            .await
            .map_err(|_| Error::Unknown)??;
        self.status(token).await.map_err(|_| Error::Unknown)
    }
}

#[cfg(test)]
mod tests;
