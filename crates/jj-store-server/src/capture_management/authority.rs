use super::*;
use crate::config::{Access, Config};
use jj_store_service::auth::Permission;
use std::path::{Path, PathBuf};

pub(super) struct Authority {
    path: PathBuf,
    identity: crate::installation::Identity,
    actor: PrincipalId,
    state: PathBuf,
    installation: PathBuf,
    catalog: PathBuf,
    socket: PathBuf,
    capture: crate::capture_config::AgentCaptureConfig,
    storage: serde_json::Value,
}
impl Authority {
    pub(super) fn actor(&self) -> PrincipalId {
        self.actor
    }
    pub(super) fn import_binding(&self) -> Result<(PathBuf, [u8; 16]), Error> {
        let installation = hex::decode(&self.identity.installation)
            .map_err(|_| Error::Unavailable)?
            .try_into()
            .map_err(|_| Error::Unavailable)?;
        Ok((self.state.join("agent-capture/imports"), installation))
    }
    pub fn new(
        path: &Path,
        config: &Config,
        identity: &crate::installation::Identity,
    ) -> Result<Self, Error> {
        let Access::LocalTrusted {
            installation,
            catalog,
            rpc_socket,
            permission: Permission::Admin,
        } = &config.access
        else {
            return Err(Error::Denied);
        };
        if path.as_os_str().is_empty() {
            return Err(Error::Invalid);
        }
        let path = if path.is_absolute() {
            path.to_owned()
        } else {
            std::env::current_dir()
                .map_err(|_| Error::Unavailable)?
                .join(path)
        };
        Ok(Self {
            path,
            identity: identity.clone(),
            actor: identity.principal(b"loopback-http"),
            state: config.state_dir.clone(),
            installation: installation.clone(),
            catalog: catalog.clone(),
            socket: rpc_socket.clone(),
            capture: config.agent_capture.clone(),
            storage: serde_json::to_value(&config.storage).map_err(|_| Error::Unavailable)?,
        })
    }
    pub fn admit(&self, actor: PrincipalId) -> Result<bool, Error> {
        if actor != self.actor {
            return Err(Error::Denied);
        }
        let config = Config::load(&self.path).map_err(|_| Error::Unavailable)?;
        let Access::LocalTrusted {
            installation,
            catalog,
            rpc_socket,
            permission: Permission::Admin,
        } = &config.access
        else {
            return Err(Error::Denied);
        };
        if config.state_dir != self.state
            || installation != &self.installation
            || catalog != &self.catalog
            || rpc_socket != &self.socket
            || serde_json::to_value(&config.storage).map_err(|_| Error::Unavailable)?
                != self.storage
        {
            return Err(Error::Unavailable);
        }
        if crate::installation::Identity::load(installation).map_err(|_| Error::Unavailable)?
            != self.identity
        {
            return Err(Error::Denied);
        }
        if !config.agent_capture.enabled {
            return Ok(false);
        }
        if config.agent_capture != self.capture {
            return Err(Error::Unavailable);
        }
        Ok(true)
    }
}
