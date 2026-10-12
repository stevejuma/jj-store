//! Runtime replacements retain listeners, storage and interactive supervisors.
use super::*;
use crate::{
    replication::{PreparedReplication, RunningReplication},
    runtime_reload::Activation,
    search_management::Manager as SearchManager,
};
use jj_store_service::administration::RepositoryHost;
use jj_store_service::runtime_reload::{self as r, Domain, State};
pub(super) struct Owners {
    pub effective: Arc<std::sync::RwLock<Config>>,
    pub policy: Arc<jj_store_service::runtime_policy::RuntimePolicy>,
    pub capture: Option<Arc<super::reload_capture::CaptureOwner>>,
    pub remote: Option<crate::remote_agents::Broker>,
    pub agents: Option<crate::agent_execution::Supervisor>,
    pub search: Option<Arc<SearchManager>>,
    pub replication: Option<PreparedReplication>,
    pub host: Option<RepositoryHost>,
    pub store: Option<Arc<jj_store_object::s3::S3Store>>,
    pub path: std::path::PathBuf,
    pub gate: tokio::sync::Mutex<()>,
    pub stopping: std::sync::atomic::AtomicBool,
    pub running_replication: tokio::sync::Mutex<Option<RunningReplication>>,
}
impl Owners {
    pub async fn attach(
        &self,
        capture: capture::RunningCapture,
        replication: Option<RunningReplication>,
    ) {
        if let Some(owner) = &self.capture {
            owner.attach(capture).await;
        }
        *self.running_replication.lock().await = replication;
    }
    pub async fn shutdown(&self) -> Result<()> {
        self.stopping
            .store(true, std::sync::atomic::Ordering::SeqCst);
        let _gate = self.gate.lock().await;
        if let Some(capture) = &self.capture {
            capture.shutdown().await?;
        }
        if let Some(replication) = self.running_replication.lock().await.as_mut() {
            replication.shutdown().await?;
        }
        Ok(())
    }
    fn differs(&self, config: &Config, domain: Domain) -> std::result::Result<bool, r::Error> {
        let old = serde_json::to_value(&*self.effective.read().map_err(|_| r::Error::Unavailable)?)
            .map_err(|_| r::Error::Invalid)?;
        let next = serde_json::to_value(config).map_err(|_| r::Error::Invalid)?;
        let key = match domain {
            Domain::Service => "limits",
            Domain::Protection => "protection",
            Domain::Sync => "replication",
            Domain::Capture => "agent_capture",
            Domain::Search => "search",
            Domain::RemoteAgents => "remote_agents",
            _ => return Ok(true),
        };
        Ok(old[key] != next[key])
    }
}
#[async_trait::async_trait]
impl Activation for Owners {
    fn supported(&self, domain: Domain) -> bool {
        match domain {
            Domain::Service => true,
            Domain::Protection => self.host.is_some(),
            Domain::Sync => self.replication.is_some(),
            Domain::Capture => self.capture.is_some(),
            Domain::Search => self.search.is_some(),
            Domain::RemoteAgents => self.remote.is_some(),
            Domain::Agents | Domain::Compute => self.agents.is_some(),
        }
    }
    async fn deferred(&self, domain: Domain) -> bool {
        if let Some(a) = &self.agents {
            let result = match domain {
                Domain::Agents => a.reload_launch_settings().await,
                Domain::Compute => a.reload_compute_settings().await,
                _ => return false,
            };
            return result.is_ok_and(|(s, _)| s == State::Deferred);
        }
        false
    }
    async fn activate(
        &self,
        domain: Domain,
        config: &Config,
    ) -> std::result::Result<(State, Option<String>), r::Error> {
        let _gate = self.gate.lock().await;
        if self.stopping.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(r::Error::Unavailable);
        }
        if !self.differs(config, domain)? {
            return Ok((State::Current, None));
        }
        match domain {
            Domain::Service | Domain::Protection => {
                let _guard = self.policy.publication.write().await;
                let mut current = self.policy.current();
                if domain == Domain::Service {
                    current.concurrent = config.limits.concurrent_requests;
                    current.max_blob = config.limits.max_blob_bytes;
                    current.limits.inventory_nodes = config.limits.inventory_nodes;
                    let timeout = Duration::from_secs(config.limits.request_seconds);
                    current.limits.timeout = timeout;
                    current.limits.publication.timeout = timeout;
                    current.limits.policy.timeout = timeout;
                    if let Some(store) = &self.store {
                        store
                            .reload_limits(
                                (current.max_blob + jj_store_model::envelope::HEADER_LEN as u64)
                                    .max(16 * 1024 * 1024),
                                timeout,
                            )
                            .map_err(|_| r::Error::Invalid)?;
                    }
                } else {
                    current.protection = jj_store_jj::policy::Protection {
                        bookmarks: config.protection.bookmarks.clone(),
                        commits: config.protection.commits.clone(),
                    };
                }
                self.policy.replace(current);
            }
            Domain::RemoteAgents => {
                let _guard = self.policy.publication.write().await;
                let broker = self.remote.as_ref().ok_or(r::Error::Unavailable)?;
                broker
                    .reload(config.remote_agents.clone())
                    .map_err(|_| r::Error::Invalid)?;
                drop(_guard);
                if broker.revoke_removed().await.is_err() {
                    self.effective
                        .write()
                        .map_err(|_| r::Error::Unavailable)?
                        .remote_agents = config.remote_agents.clone();
                    return Ok((State::Deferred,Some("Sharing is active. Revoked workers are draining; retained commands are not replayed.".into())));
                }
            }
            Domain::Capture => self
                .capture
                .as_ref()
                .ok_or(r::Error::Unavailable)?
                .reload(config)
                .await
                .map_err(|_| r::Error::Unavailable)?,
            Domain::Sync => {
                let host = self.host.as_ref().ok_or(r::Error::Unavailable)?;
                let store = self.store.as_ref().ok_or(r::Error::Unavailable)?;
                let identity_path = match &config.access {
                    Access::LocalTrusted { installation, .. }
                    | Access::Authenticated { installation, .. } => installation,
                };
                let principal = Identity::load(identity_path)
                    .map_err(|_| r::Error::Denied)?
                    .principal(b"replication-worker");
                let next = PreparedReplication::new(
                    host,
                    store.clone(),
                    &config.replication,
                    principal,
                    Default::default(),
                )
                .map_err(|_| r::Error::Invalid)?;
                let mut running = self.running_replication.lock().await;
                if let Some(old) = running.as_mut() {
                    old.shutdown().await.map_err(|_| r::Error::Unavailable)?;
                }
                *running = Some(
                    self.replication
                        .as_ref()
                        .ok_or(r::Error::Unavailable)?
                        .replace(next)
                        .map_err(|_| r::Error::Unavailable)?,
                );
            }
            Domain::Search => {
                let bytes = policy::read_regular(&self.path, 64 * 1024)
                    .map_err(|_| r::Error::Unavailable)?;
                use sha2::Digest;
                let revision = hex::encode(sha2::Sha256::digest(bytes));
                self.search
                    .as_ref()
                    .ok_or(r::Error::Unavailable)?
                    .reload(config, &revision)
                    .await
                    .map_err(|_| r::Error::Unavailable)?;
            }
            Domain::Agents => {
                return match &self.agents {
                    Some(a) => a
                        .reload_launch_settings()
                        .await
                        .map_err(|_| r::Error::Unavailable),
                    None => Ok((State::Current, None)),
                };
            }
            Domain::Compute => {
                return match &self.agents {
                    Some(a) => a
                        .reload_compute_settings()
                        .await
                        .map_err(|_| r::Error::Unavailable),
                    None => Ok((State::Current, None)),
                };
            }
        }
        let mut running = self.effective.write().map_err(|_| r::Error::Unavailable)?;
        match domain {
            Domain::Service => running.limits = config.limits.clone(),
            Domain::Protection => running.protection = config.protection.clone(),
            Domain::Sync => running.replication = config.replication.clone(),
            Domain::Capture => running.agent_capture = config.agent_capture.clone(),
            Domain::RemoteAgents => running.remote_agents = config.remote_agents.clone(),
            Domain::Search => running.search = config.search.clone(),
            _ => {}
        }
        Ok((State::Applied, None))
    }
}
