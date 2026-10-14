//! CLI-owned optional host runtime. Reload never drops an active poll/job.
use super::*;
use jj_store_service::{
    runtime_reload::{Error, State},
    workflow_host::{HostSettings, ManagedHost, Runtime, RuntimeStatus},
};
use std::sync::{
    Arc, RwLock,
    atomic::{AtomicBool, Ordering},
};
pub struct ManagedRuntime {
    host: tokio::sync::Mutex<Vec<Host>>,
    discovery: Option<Discovery>,
    desired: RwLock<Option<ManagedHost>>,
    running: AtomicBool,
    busy: AtomicBool,
    stopping: AtomicBool,
    message: RwLock<Option<String>>,
    wake: tokio::sync::Notify,
}
impl ManagedRuntime {
    pub fn new(initial: Option<ManagedHost>, discovery: Option<Discovery>) -> Arc<Self> {
        Arc::new(Self {
            host: Default::default(),
            discovery,
            desired: RwLock::new(initial),
            running: Default::default(),
            busy: Default::default(),
            stopping: Default::default(),
            message: Default::default(),
            wake: Default::default(),
        })
    }
    pub fn start(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let owner = self.clone();
        tokio::spawn(async move {
            loop {
                if owner.stopping.load(Ordering::SeqCst) {
                    return;
                }
                {
                    let mut host = owner.host.lock().await;
                    if owner.stopping.load(Ordering::SeqCst) {
                        return;
                    }
                    // Read under the same lock used by reload, so a queued poll
                    // cannot reopen configuration that has just been disabled.
                    let desired = owner.desired.read().expect("workflow desired").clone();
                    if let Some(spec) = desired.filter(|v| v.enabled) {
                        owner.busy.store(true, Ordering::SeqCst);
                        let result = owner.poll(&mut host, &spec.configuration).await;
                        owner.busy.store(false, Ordering::SeqCst);
                        owner.running.store(!host.is_empty(), Ordering::SeqCst);
                        *owner.message.write().expect("workflow status") = result.err().map(|_| "Host polling paused; native attachment, admission and retained request may need attention.".into());
                    }
                }
                let delay = owner
                    .desired
                    .read()
                    .expect("workflow desired")
                    .as_ref()
                    .map_or(1000, |v| v.configuration.poll_millis());
                tokio::select! { _ = owner.wake.notified() => {}, _ = tokio::time::sleep(Duration::from_millis(delay)) => {} }
            }
        })
    }
    async fn configurations(&self, settings: &HostSettings) -> Result<Vec<Config>> {
        match settings {
            HostSettings::Legacy(value) => {
                let config = Config(value.clone());
                config.validate()?;
                Ok(vec![config])
            }
            HostSettings::Registered(environment) => {
                self.discovery
                    .as_ref()
                    .ok_or("Native workspace discovery is unavailable")?
                    .configurations(environment)
                    .await
            }
        }
    }
    async fn install(current: &mut Vec<Host>, next: Vec<Config>) -> Result<()> {
        // Open replacements before changing existing configuration or releasing locks.
        let same = |host: &Host, config: &Config| {
            host.config.host == config.host
                && host.config.principal == config.principal
                && host.config.home == config.home
                && host.config.state_dir == config.state_dir
        };
        let mut fresh = Vec::new();
        for config in &next {
            if !current.iter().any(|host| same(host, config)) {
                fresh.push(Host::open(config.clone()).await?);
            }
        }
        current.retain(|host| next.iter().any(|config| same(host, config)));
        for config in next {
            if let Some(host) = current.iter_mut().find(|host| same(host, &config)) {
                host.config = config;
            }
        }
        current.extend(fresh);
        Ok(())
    }
    async fn poll(&self, hosts: &mut Vec<Host>, settings: &HostSettings) -> Result<()> {
        for host in hosts.iter_mut() {
            host.recover().await?;
        }
        Self::install(hosts, self.configurations(settings).await?).await?;
        for host in hosts {
            host.tick().await?;
        }
        Ok(())
    }
    pub async fn stop(&self) {
        self.stopping.store(true, Ordering::SeqCst);
        self.wake.notify_one();
        // Held across each tick, including execution and final reporting.
        self.host.lock().await.clear();
        self.running.store(false, Ordering::SeqCst);
    }
}
#[async_trait::async_trait]
impl Runtime for ManagedRuntime {
    fn status(&self) -> RuntimeStatus {
        RuntimeStatus {
            principal: self.discovery.as_ref().map(|v| v.principal),
            available: true,
            running: self.running.load(Ordering::SeqCst),
            busy: self.busy.load(Ordering::SeqCst),
            message: self.message.read().expect("workflow status").clone(),
        }
    }
    async fn reload(
        &self,
        next: Option<ManagedHost>,
    ) -> std::result::Result<(State, Option<String>), Error> {
        if self.stopping.load(Ordering::SeqCst) {
            return Err(Error::Unavailable);
        }
        let Ok(mut current) = self.host.try_lock() else {
            return Ok((State::Deferred, Some("The host is settling a poll or active job. Its configuration and journal are retained; reload again when idle.".into())));
        };
        if current
            .iter()
            .any(|h| h.journal.active.is_some() || h.journal.pending.is_some())
        {
            return Ok((State::Deferred, Some("The host retains an uncertain attempt or original reporting request. Reconcile it before replacing configuration.".into())));
        }
        if let Some(spec) = &next {
            spec.configuration.validate().map_err(|_| Error::Invalid)?;
        }
        if let Some(spec) = next.as_ref().filter(|v| v.enabled) {
            let configurations = self
                .configurations(&spec.configuration)
                .await
                .map_err(|_| Error::Unavailable)?;
            Self::install(&mut current, configurations)
                .await
                .map_err(|_| Error::Unavailable)?;
        } else {
            current.clear();
        }
        *self.desired.write().map_err(|_| Error::Unavailable)? = next;
        self.running.store(!current.is_empty(), Ordering::SeqCst);
        *self.message.write().map_err(|_| Error::Unavailable)? = None;
        self.wake.notify_one();
        Ok((State::Applied, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn idle_disabled_runtime_is_reloadable_and_shutdown_drains_poll() {
        let runtime = ManagedRuntime::new(None, None);
        let task = runtime.start();
        let poll = runtime.host.lock().await;
        assert_eq!(runtime.reload(None).await.unwrap().0, State::Deferred);
        let stopping = {
            let runtime = runtime.clone();
            tokio::spawn(async move { runtime.stop().await })
        };
        tokio::task::yield_now().await;
        assert!(!stopping.is_finished());
        drop(poll);
        stopping.await.unwrap();
        task.await.unwrap();
        assert!(!runtime.status().running);
        assert!(matches!(
            runtime.reload(None).await,
            Err(Error::Unavailable)
        ));
    }
    #[tokio::test]
    async fn disabled_host_does_not_start_polling() {
        let runtime = ManagedRuntime::new(None, None);
        assert_eq!(runtime.reload(None).await.unwrap().0, State::Applied);
        assert!(!runtime.status().busy);
        assert!(runtime.host.lock().await.is_empty());
    }
}
