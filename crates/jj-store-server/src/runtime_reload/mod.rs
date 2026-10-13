//! One retained activation intent, independent of listener and storage ownership.
mod journal;
use crate::{
    config::{Access, Config},
    installation::Identity,
    policy,
};
use jj_store_service::{
    auth::{Clock, Permission, SystemClock},
    directory::{Caller as DirectoryCaller, Directory},
    runtime_reload::*,
};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};
#[async_trait::async_trait]
pub(crate) trait Activation: Send + Sync {
    fn supported(&self, _domain: Domain) -> bool {
        true
    }
    async fn deferred(&self, _domain: Domain) -> bool {
        false
    }
    async fn activate(
        &self,
        domain: Domain,
        config: &Config,
    ) -> Result<(State, Option<String>), Error>;
}
pub struct Coordinator {
    owner: std::sync::Weak<Coordinator>,
    path: PathBuf,
    identity: Identity,
    startup: Config,
    pub(crate) effective: Arc<RwLock<Config>>,
    policy: policy::PolicyFile,
    directory: Option<Arc<dyn Directory>>,
    activation: Arc<dyn Activation>,
    journal: std::sync::Mutex<journal::Journal>,
    gate: Arc<tokio::sync::Mutex<()>>,
    changes: jj_store_service::freshness::Signals,
}
impl Coordinator {
    pub(crate) fn new(
        path: &Path,
        config: &Config,
        identity: &Identity,
        effective: Arc<RwLock<Config>>,
        directory: Option<Arc<dyn Directory>>,
        activation: Arc<dyn Activation>,
    ) -> crate::Result<Arc<Self>> {
        let path = policy::outside_checkout(path)?;
        let policy = policy::PolicyFile::new(config.operator_policy.clone())?;
        let journal = journal::Journal::open(&config.state_dir)?;
        Ok(Arc::new_cyclic(|owner| Self {
            owner: owner.clone(),
            path,
            startup: config.clone(),
            identity: identity.clone(),
            effective,
            policy,
            directory,
            activation,
            journal: std::sync::Mutex::new(journal),
            gate: Arc::new(Default::default()),
            changes: Default::default(),
        }))
    }
    async fn authority(
        &self,
        caller: &Caller,
    ) -> Result<(jj_store_model::PrincipalId, Vec<Domain>), Error> {
        let available = |domains: Vec<Domain>| {
            domains
                .into_iter()
                .filter(|d| self.activation.supported(*d))
                .collect::<Vec<_>>()
        };
        let all = available(vec![
            Domain::Workflows,
            Domain::Service,
            Domain::Protection,
            Domain::Sync,
            Domain::Capture,
            Domain::Search,
            Domain::Agents,
            Domain::Compute,
        ]);
        match (&self.startup.access, caller) {
            (
                Access::LocalTrusted {
                    installation,
                    permission: Permission::Admin,
                    ..
                },
                Caller::Local(actor),
            ) if *actor == self.identity.principal(b"loopback-http")
                && Identity::load(installation).map_err(|_| Error::Denied)? == self.identity =>
            {
                Ok((*actor, all))
            }
            (Access::Authenticated { installation, .. }, Caller::Token(token)) => {
                if Identity::load(installation).map_err(|_| Error::Denied)? != self.identity {
                    return Err(Error::Denied);
                }
                let actor = self
                    .directory
                    .as_ref()
                    .ok_or(Error::Denied)?
                    .principal(&DirectoryCaller::Token(token.clone()))
                    .await
                    .map_err(|_| Error::Denied)?;
                let policy = self.policy.read().map_err(|_| Error::Denied)?;
                let now = SystemClock.unix_seconds().map_err(|_| Error::Denied)?;
                if policy
                    .authorize(token, Permission::ManageConfiguration, now)
                    .is_ok_and(|p| p.id() == actor)
                {
                    Ok((
                        actor,
                        available(vec![
                            Domain::Service,
                            Domain::Protection,
                            Domain::Sync,
                            Domain::Search,
                            Domain::RemoteAgents,
                        ]),
                    ))
                } else if policy
                    .authorize(token, Permission::ManageAgents, now)
                    .is_ok_and(|p| p.id() == actor)
                {
                    Ok((actor, available(vec![Domain::RemoteAgents])))
                } else {
                    Err(Error::Denied)
                }
            }
            _ => Err(Error::Denied),
        }
    }
    fn saved(&self) -> Result<(String, Config), Error> {
        let bytes = policy::read_regular(&self.path, 64 * 1024).map_err(|_| Error::Unavailable)?;
        let config = Config::load(&self.path).map_err(|_| Error::Invalid)?;
        if policy::read_regular(&self.path, 64 * 1024).map_err(|_| Error::Unavailable)? != bytes {
            return Err(Error::Stale);
        }
        Ok((hex::encode(Sha256::digest(bytes)), config))
    }
    fn compatible(&self, config: &Config) -> Result<(), Error> {
        fn infrastructure(config: &Config) -> Result<serde_json::Value, Error> {
            let mut v = serde_json::to_value(config).map_err(|_| Error::Invalid)?;
            for key in [
                "workflow_hosts",
                "workflow_host",
                "limits",
                "protection",
                "replication",
                "search",
                "agent_capture",
                "remote_agents",
            ] {
                v.as_object_mut().ok_or(Error::Invalid)?.remove(key);
            }
            Ok(v)
        }
        if infrastructure(config)? != infrastructure(&self.startup)? {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    fn pending(&self, config: &Config, domains: &[Domain]) -> Result<Vec<Domain>, Error> {
        let effective = self.effective.read().map_err(|_| Error::Unavailable)?;
        let saved = serde_json::to_value(config).map_err(|_| Error::Invalid)?;
        let running = serde_json::to_value(&*effective).map_err(|_| Error::Invalid)?;
        Ok(domains
            .iter()
            .filter(|d| match d {
                Domain::Agents | Domain::Compute => false,
                Domain::Workflows => {
                    saved["workflow_hosts"] != running["workflow_hosts"]
                        || saved["workflow_host"] != running["workflow_host"]
                }
                _ => saved[field(**d)] != running[field(**d)],
            })
            .copied()
            .collect())
    }
    pub async fn start(
        self: &Arc<Self>,
        caller: Caller,
        command: Command,
    ) -> Result<Outcome, Error> {
        let (actor, supported) = self.authority(&caller).await?;
        validate(&command, &supported)?;
        if let Some(old) = self
            .journal
            .lock()
            .map_err(|_| Error::Unavailable)?
            .find(actor, &command)?
        {
            return Ok(old);
        }
        let permit = self
            .gate
            .clone()
            .try_lock_owned()
            .map_err(|_| Error::Busy)?;
        let mut lock = self.path.as_os_str().to_owned();
        lock.push(".settings.lock");
        let file_lock = policy::exclusive_lock(Path::new(&lock)).map_err(|_| Error::Busy)?;
        let (revision, config) = self.saved()?;
        if revision != command.expected_revision {
            return Err(Error::Stale);
        }
        self.compatible(&config)?;
        self.authority(&caller).await?;
        let outcome = self
            .journal
            .lock()
            .map_err(|_| Error::Unavailable)?
            .begin(actor, command.clone())?;
        let this = self.clone();
        tokio::spawn(async move {
            let _permit = permit;
            let _file_lock = file_lock;
            for domain in &command.domains {
                let result = if this.authority(&caller).await.is_err() {
                    Err(Error::Denied)
                } else {
                    this.activation.activate(*domain, &config).await
                };
                let (state, reason) = result.unwrap_or((
                    State::Failed,
                    Some(
                        "Activation failed; previous owners and original receipts are retained."
                            .into(),
                    ),
                ));
                if let Ok(mut journal) = this.journal.lock() {
                    if journal
                        .progress(actor, command.request_id, *domain, state, reason)
                        .is_err()
                    {
                        break;
                    }
                } else {
                    break;
                }
                this.changes.notify(
                    None,
                    jj_store_service::freshness::Domain::Configuration,
                    None,
                );
            }
            if let Ok(mut journal) = this.journal.lock() {
                let _ = journal.finish(actor, command.request_id);
            }
            this.changes.notify(
                None,
                jj_store_service::freshness::Domain::Configuration,
                None,
            );
        });
        Ok(outcome)
    }
}
fn field(domain: Domain) -> &'static str {
    match domain {
        Domain::Workflows => "workflow_host",
        Domain::Service => "limits",
        Domain::Protection => "protection",
        Domain::Sync => "replication",
        Domain::Capture => "agent_capture",
        Domain::RemoteAgents => "remote_agents",
        Domain::Search => "search",
        _ => "",
    }
}
fn validate(command: &Command, supported: &[Domain]) -> Result<(), Error> {
    if command.version != 1
        || command.expected_revision.len() != 64
        || !command
            .expected_revision
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        || command.domains.is_empty()
        || command.domains.len() > 8
        || command.domains.iter().collect::<BTreeSet<_>>().len() != command.domains.len()
    {
        return Err(Error::Invalid);
    }
    if command.domains.iter().any(|d| !supported.contains(d)) {
        return Err(Error::Denied);
    }
    Ok(())
}
#[async_trait::async_trait]
impl Manager for Coordinator {
    fn freshness(
        &self,
    ) -> Option<tokio::sync::broadcast::Receiver<jj_store_service::freshness::Change>> {
        Some(self.changes.subscribe())
    }
    async fn status(&self, caller: Caller) -> Result<Status, Error> {
        let (actor, supported) = self.authority(&caller).await?;
        let (saved_revision, config) = self.saved()?;
        self.compatible(&config)?;
        let mut pending = self.pending(&config, &supported)?;
        for domain in &supported {
            if self.activation.deferred(*domain).await {
                pending.push(*domain);
            }
        }
        let journal = self.journal.lock().map_err(|_| Error::Unavailable)?;
        let (original, outcome) = journal
            .active(actor)
            .filter(|(c, _)| c.domains.iter().all(|d| supported.contains(d)))
            .map(|(c, o)| (Some(c), Some(o)))
            .unwrap_or((None, None));
        Ok(Status {
            original,
            outcome,
            saved_revision,
            supported,
            pending,
            generation: journal.generation,
        })
    }
    async fn reload(&self, caller: Caller, command: Command) -> Result<Outcome, Error> {
        self.owner
            .upgrade()
            .ok_or(Error::Unavailable)?
            .start(caller, command)
            .await
    }
    async fn outcome(&self, caller: Caller, command: Command) -> Result<Outcome, Error> {
        let (actor, supported) = self.authority(&caller).await?;
        validate(&command, &supported)?;
        let mut result = self
            .journal
            .lock()
            .map_err(|_| Error::Unavailable)?
            .find(actor, &command)?
            .ok_or(Error::Unavailable)?;
        if result.state == State::Unknown {
            let (revision, config) = self.saved()?;
            if revision == command.expected_revision
                && self.pending(&config, &command.domains)?.is_empty()
            {
                for r in &mut result.results {
                    if matches!(r.state, State::Unknown | State::Running) {
                        r.state = if self.activation.deferred(r.domain).await {
                            State::Deferred
                        } else {
                            State::Current
                        };
                    }
                }
                result.state = if result.results.iter().any(|r| r.state == State::Failed) {
                    State::Failed
                } else if result.results.iter().any(|r| r.state == State::Deferred) {
                    State::Deferred
                } else {
                    State::Current
                };
            }
            self.authority(&caller).await?;
            self.journal
                .lock()
                .map_err(|_| Error::Unavailable)?
                .observed(actor, result.clone())?;
        }
        Ok(result)
    }
}
#[cfg(test)]
mod tests;
