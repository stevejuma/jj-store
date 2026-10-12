//! Operator-defined trusted agents, with caller-isolated journals and scoped UDS access.
mod caller;
mod config;
mod lease;
use crate::agent_execution::{Resources, Supervisor};
pub use config::{Agent, Configuration};
use jj_store_model::{PrincipalId, RepositoryId};
use jj_store_service::{
    administration::RepositoryHost, agent_execution::*, auth::BearerToken, directory::Directory,
    registry::RepositoryRegistry,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Scope {
    principal: PrincipalId,
    conversation: String,
    home: RepositoryId,
    repositories: BTreeSet<RepositoryId>,
    agents: BTreeSet<String>,
}
struct Participation {
    supervisor: Supervisor,
    lease: Arc<lease::Lease>,
    stop: tokio::sync::watch::Sender<bool>,
    rpc: tokio::sync::Mutex<Option<tokio::task::JoinHandle<()>>>,
}
#[derive(Clone)]
pub struct Broker(Arc<Inner>);
struct Inner {
    root: PathBuf,
    config: Arc<std::sync::RwLock<Option<Configuration>>>,
    host: RepositoryHost,
    directory: Arc<dyn Directory>,
    registry: Arc<dyn RepositoryRegistry>,
    scopes: Mutex<BTreeMap<String, Scope>>,
    owners: tokio::sync::Mutex<BTreeMap<String, Arc<Participation>>>,
    workers: Arc<tokio::sync::Semaphore>,
    dispatch: tokio::sync::Mutex<()>,
    changed: tokio::sync::broadcast::Sender<u64>,
    freshness: jj_store_service::freshness::Signals,
    stopping: std::sync::atomic::AtomicBool,
}
fn key(actor: PrincipalId, conversation: &str) -> String {
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(format!(
        "remote-agent/{actor}/{conversation}"
    )))
}
impl Broker {
    pub fn open(
        root: PathBuf,
        config: Configuration,
        host: RepositoryHost,
        directory: Arc<dyn Directory>,
        registry: Arc<dyn RepositoryRegistry>,
    ) -> Result<Self> {
        Self::open_optional(root, Some(config), host, directory, registry)
    }
    pub fn open_optional(
        root: PathBuf,
        config: Option<Configuration>,
        host: RepositoryHost,
        directory: Arc<dyn Directory>,
        registry: Arc<dyn RepositoryRegistry>,
    ) -> Result<Self> {
        if let Some(config) = &config {
            config.validate().map_err(|_| Error::Invalid)?;
        }
        let root = crate::policy::outside_checkout(&root).map_err(|_| Error::Invalid)?;
        private(&root)?;
        private(&root.join("sockets"))?;
        let scopes: BTreeMap<String, Scope> =
            match crate::policy::read_regular(&root.join("scopes.json"), 256 * 1024) {
                Ok(bytes) => serde_json::from_slice(&bytes).map_err(|_| Error::Unavailable)?,
                Err(_) if !root.join("scopes.json").exists() => BTreeMap::new(),
                Err(_) => return Err(Error::Unavailable),
            };
        if scopes.len() > 64
            || scopes.iter().any(|(id, s)| {
                *id != key(s.principal, &s.conversation)
                    || s.conversation.parse::<jj_store_model::RequestId>().is_err()
                    || s.agents.is_empty()
                    || s.agents.len() > 4
                    || s.repositories.len() > 9
                    || !s.repositories.contains(&s.home)
            })
        {
            return Err(Error::Invalid);
        }
        let (changed, _) = tokio::sync::broadcast::channel(32);
        Ok(Self(Arc::new(Inner {
            root,
            config: Arc::new(std::sync::RwLock::new(config)),
            host,
            directory,
            registry,
            scopes: Mutex::new(scopes),
            owners: Default::default(),
            workers: Arc::new(tokio::sync::Semaphore::new(4)),
            changed,
            freshness: Default::default(),
            dispatch: Default::default(),
            stopping: Default::default(),
        })))
    }
    pub fn configuration(&self) -> Result<Configuration> {
        self.0
            .config
            .read()
            .map_err(|_| Error::Unavailable)?
            .clone()
            .ok_or(Error::Denied)
    }
    pub fn reload(&self, config: Option<Configuration>) -> Result<()> {
        if let Some(config) = &config {
            config.validate().map_err(|_| Error::Invalid)?;
        }
        *self.0.config.write().map_err(|_| Error::Unavailable)? = config;
        Ok(())
    }
    pub(crate) async fn revoke_removed(&self) -> Result<()> {
        let owners = self
            .0
            .owners
            .lock()
            .await
            .values()
            .cloned()
            .collect::<Vec<_>>();
        for owner in owners {
            if owner.lease.check().await.is_err() {
                owner.stop().await?;
            }
        }
        self.0.freshness.notify(
            None,
            jj_store_service::freshness::Domain::AgentExecution,
            None,
        );
        Ok(())
    }
    fn save(&self, scopes: &BTreeMap<String, Scope>) -> Result<()> {
        let mut file =
            tempfile::NamedTempFile::new_in(&self.0.root).map_err(|_| Error::Unavailable)?;
        serde_json::to_writer(&mut file, scopes).map_err(|_| Error::Unavailable)?;
        file.flush().map_err(|_| Error::Unavailable)?;
        file.as_file().sync_all().map_err(|_| Error::Unavailable)?;
        file.persist(self.0.root.join("scopes.json"))
            .map_err(|_| Error::Unavailable)?;
        std::fs::File::open(&self.0.root)
            .and_then(|f| f.sync_all())
            .map_err(|_| Error::Unavailable)
    }
    async fn check_scope(&self, scope: &Scope, token: &BearerToken) -> Result<()> {
        let authority = Arc::new(lease::Lease {
            principal: scope.principal,
            token: Mutex::new(Some(token.clone())),
            scope: Mutex::new(scope.clone()),
            directory: self.0.directory.clone(),
            configuration: self.0.config.clone(),
        });
        authority.check().await?;
        for repo in &scope.repositories {
            let request = self
                .0
                .host
                .endpoint(*repo)
                .map_err(|_| Error::Denied)?
                .bind_delegated(authority.clone())
                .map_err(|_| Error::Denied)?;
            request.authorize(true).await.map_err(|_| Error::Denied)?;
            if *repo == scope.home
                && request
                    .into_session()
                    .snapshot()
                    .await
                    .map_err(|_| Error::Denied)?
                    .manifest()
                    .work_root
                    .is_none()
            {
                return Err(Error::Invalid);
            }
        }
        Ok(())
    }
    fn queue_budget(&self, additional: usize) -> Result<()> {
        let owners = self.0.owners.try_lock().map_err(|_| Error::Busy)?;
        if owners
            .values()
            .map(|o| o.supervisor.pending_prompts())
            .sum::<usize>()
            .saturating_add(additional)
            > 64
        {
            return Err(Error::Limit);
        }
        Ok(())
    }
    async fn owner(
        &self,
        scope: Scope,
        token: BearerToken,
        update: bool,
    ) -> Result<Arc<Participation>> {
        if self.0.stopping.load(std::sync::atomic::Ordering::Acquire) {
            return Err(Error::Unavailable);
        }
        let id = key(scope.principal, &scope.conversation);
        let mut owners = self.0.owners.lock().await;
        if let Some(owner) = owners.get(&id).cloned()
            && owner
                .lease
                .token
                .lock()
                .map_err(|_| Error::Unavailable)?
                .is_none()
        {
            owner.stop().await?;
            owners.remove(&id);
        }
        if let Some(owner) = owners.get(&id) {
            if owner.lease.scope()? != scope {
                if !update {
                    return Err(Error::Stale);
                }
                if owner.supervisor.pending_prompts() > 0 {
                    return Err(Error::Busy);
                }
                let candidate = lease::Lease {
                    principal: scope.principal,
                    token: Mutex::new(Some(token.clone())),
                    scope: Mutex::new(scope.clone()),
                    directory: self.0.directory.clone(),
                    configuration: self.0.config.clone(),
                };
                candidate.check().await?;
                let candidate = Arc::new(candidate);
                for repo in &scope.repositories {
                    self.0
                        .host
                        .endpoint(*repo)
                        .map_err(|_| Error::Denied)?
                        .bind_delegated(candidate.clone())
                        .map_err(|_| Error::Denied)?
                        .authorize(true)
                        .await
                        .map_err(|_| Error::Denied)?;
                }
                *owner.lease.scope.lock().map_err(|_| Error::Unavailable)? = scope;
            }
            *owner.lease.token.lock().map_err(|_| Error::Unavailable)? = Some(token);
            return Ok(owner.clone());
        }
        let lease = Arc::new(lease::Lease {
            principal: scope.principal,
            token: Mutex::new(Some(token)),
            scope: Mutex::new(scope.clone()),
            directory: self.0.directory.clone(),
            configuration: self.0.config.clone(),
        });
        lease.check().await?;
        for repository in &scope.repositories {
            self.0
                .host
                .endpoint(*repository)
                .map_err(|_| Error::Denied)?
                .bind_delegated(lease.clone())
                .map_err(|_| Error::Denied)?
                .authorize(true)
                .await
                .map_err(|_| Error::Denied)?;
        }
        let root = self.0.root.join(&id);
        private(&root)?;
        let socket = self
            .0
            .root
            .join("sockets")
            .join(format!("{}.sock", &id[..16]));
        if socket.as_os_str().len() > 100 {
            return Err(Error::Invalid);
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::{FileTypeExt, PermissionsExt};
            if let Ok(m) = std::fs::symlink_metadata(&socket) {
                if !m.file_type().is_socket() {
                    return Err(Error::Invalid);
                }
                std::fs::remove_file(&socket).map_err(|_| Error::Unavailable)?;
            }
            let listener =
                tokio::net::UnixListener::bind(&socket).map_err(|_| Error::Unavailable)?;
            std::fs::set_permissions(&socket, std::fs::Permissions::from_mode(0o600))
                .map_err(|_| Error::Unavailable)?;
            let rpc = jj_store_rpc::server::NativeRpc::delegated(
                self.0.host.clone(),
                Default::default(),
                lease.clone(),
                rustix::process::getuid().as_raw(),
            )
            .map_err(|_| Error::Unavailable)?;
            let (stop, mut stopped) = tokio::sync::watch::channel(false);
            let supervisor = Supervisor::open(Resources {
                actor: scope.principal,
                root,
                executable: self.configuration()?.acp_executable,
                jj_executable: self.configuration()?.jj_executable,
                driver: Arc::new(crate::agent_execution::driver::Driver(
                    jj_store_jj::agent_workspaces::Provider {
                        executable: self.configuration()?.jj_executable,
                    },
                )),
                folders: None,
                workspaces: None,
                verifier: None,
                registry: self.0.registry.clone(),
                host: Some(self.0.host.clone()),
                native_socket: Some(socket.clone()),
            })?;
            supervisor.remote(
                lease.clone(),
                self.0.workers.clone(),
                self.configuration()?.setups(scope.principal),
            )?;
            let serving_lease = lease.clone();
            let rpc = tokio::spawn(async move {
                let _ = tonic::transport::Server::builder()
                    .add_service(rpc.service())
                    .serve_with_incoming_shutdown(
                        tokio_stream::wrappers::UnixListenerStream::new(listener),
                        async {
                            let _ = stopped.changed().await;
                        },
                    )
                    .await;
                serving_lease.revoke();
                let _ = std::fs::remove_file(socket);
            });
            let owner = Arc::new(Participation {
                supervisor,
                lease,
                stop,
                rpc: tokio::sync::Mutex::new(Some(rpc)),
            });
            owners.insert(id, owner.clone());
            self.follow(owner.clone());
            Ok(owner)
        }
        #[cfg(not(unix))]
        {
            Err(Error::Unavailable)
        }
    }
    fn follow(&self, owner: Arc<Participation>) {
        let broker = self.clone();
        let mut stopped = owner.stop.subscribe();
        tokio::spawn(async move {
            let mut changes = owner.supervisor.subscribe();
            let mut timer = tokio::time::interval(std::time::Duration::from_secs(1));
            loop {
                tokio::select! {
                    _ = stopped.changed() => break,
                    change = changes.recv() => { if change.is_ok() { let _ = broker.0.changed.send(1); broker.0.freshness.notify_to(None, jj_store_service::freshness::Domain::AgentExecution, None, Some(owner.lease.principal)); } },
                    _ = timer.tick() => {
                        let mut allowed = owner.lease.check().await.is_ok();
                        for repo in &owner.lease.scope().map(|s| s.repositories).unwrap_or_default() {
                            allowed &= match owner.supervisor.native_request(*repo) {
                                Ok(request) => request.authorize(true).await.is_ok(), Err(_) => false,
                            };
                        }
                        if !allowed {
                            let _ = owner.stop().await;
                            let _ = broker.0.changed.send(1); break;
                        }
                        let _ = owner.supervisor.publish().await;
                    }
                }
            }
        });
    }
}
impl Participation {
    async fn stop(&self) -> Result<()> {
        self.lease.revoke();
        self.stop.send_replace(true);
        self.supervisor.shutdown().await?;
        if let Some(mut task) = self.rpc.lock().await.take() {
            if tokio::time::timeout(std::time::Duration::from_secs(5), &mut task)
                .await
                .is_err()
            {
                task.abort();
                let _ = task.await;
            }
        }
        Ok(())
    }
}
fn private(root: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(root).map_err(|_| Error::Unavailable)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(root, std::fs::Permissions::from_mode(0o700))
            .map_err(|_| Error::Unavailable)?;
    }
    Ok(())
}
#[async_trait::async_trait]
impl jj_store_service::remote_agents::RemoteAgents for Broker {
    fn freshness(
        &self,
    ) -> Option<tokio::sync::broadcast::Receiver<jj_store_service::freshness::Change>> {
        Some(self.0.freshness.subscribe())
    }
    async fn admit(&self, token: BearerToken) -> Result<(PrincipalId, AgentManager)> {
        let principal = self
            .0
            .directory
            .principal(&jj_store_service::directory::Caller::Token(token.clone()))
            .await
            .map_err(|_| Error::Denied)?;
        if self.configuration()?.setups(principal).is_empty() {
            return Err(Error::Denied);
        }
        Ok((
            principal,
            Arc::new(caller::Caller {
                broker: self.clone(),
                principal,
                token,
            }),
        ))
    }
    async fn shutdown(&self) -> Result<()> {
        self.0
            .stopping
            .store(true, std::sync::atomic::Ordering::Release);
        for owner in self.0.owners.lock().await.values() {
            owner.stop().await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod execution_tests;
#[cfg(test)]
mod tests;
