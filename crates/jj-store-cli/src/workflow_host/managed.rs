//! CLI-owned optional host runtime. Reload never drops an active poll/job.
use super::*;
use jj_store_service::{
    runtime_reload::{Error, State},
    workflow_host::{ManagedHost, Runtime, RuntimeStatus},
};
use std::sync::{
    Arc, RwLock,
    atomic::{AtomicBool, Ordering},
};
pub struct ManagedRuntime {
    host: tokio::sync::Mutex<Option<Host>>,
    desired: RwLock<Option<ManagedHost>>,
    running: AtomicBool,
    busy: AtomicBool,
    stopping: AtomicBool,
    message: RwLock<Option<String>>,
    wake: tokio::sync::Notify,
}
impl ManagedRuntime {
    pub fn new(initial: Option<ManagedHost>) -> Arc<Self> {
        Arc::new(Self {
            host: Default::default(),
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
                        if host.is_none() {
                            let config = Config(spec.configuration);
                            if config.validate().is_ok() {
                                *host = Host::open(config).await.ok();
                            }
                        }
                        owner.running.store(host.is_some(), Ordering::SeqCst);
                        owner.busy.store(true, Ordering::SeqCst);
                        let result = match host.as_mut() {
                            Some(host) => host.tick().await,
                            None => {
                                Err("Host configuration or native attachment is unavailable".into())
                            }
                        };
                        owner.busy.store(false, Ordering::SeqCst);
                        *owner.message.write().expect("workflow status") = result.err().map(|_| "Host polling paused; native attachment, admission and retained request may need attention.".into());
                    }
                }
                let delay = owner
                    .desired
                    .read()
                    .expect("workflow desired")
                    .as_ref()
                    .map_or(1000, |v| v.configuration.poll_millis);
                tokio::select! { _ = owner.wake.notified() => {}, _ = tokio::time::sleep(Duration::from_millis(delay)) => {} }
            }
        })
    }
    pub async fn stop(&self) {
        self.stopping.store(true, Ordering::SeqCst);
        self.wake.notify_one();
        // Held across each tick, including execution and final reporting.
        self.host.lock().await.take();
        self.running.store(false, Ordering::SeqCst);
    }
}
#[async_trait::async_trait]
impl Runtime for ManagedRuntime {
    fn status(&self) -> RuntimeStatus {
        RuntimeStatus {
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
            .as_ref()
            .is_some_and(|h| h.journal.active.is_some() || h.journal.pending.is_some())
        {
            return Ok((State::Deferred, Some("The host retains an uncertain attempt or original reporting request. Reconcile it before replacing configuration.".into())));
        }
        if let Some(spec) = &next {
            spec.configuration.validate().map_err(|_| Error::Invalid)?;
            if spec.enabled {
                Config(spec.configuration.clone())
                    .validate()
                    .map_err(|_| Error::Invalid)?;
            }
        }
        if next.as_ref().is_some_and(|v| v.enabled) {
            let cfg = Config(next.as_ref().expect("enabled host").configuration.clone());
            if let Some(host) = current.as_mut() {
                if host.config.host == cfg.host
                    && host.config.principal == cfg.principal
                    && host.config.home == cfg.home
                    && host.config.state_dir == cfg.state_dir
                {
                    host.config = cfg;
                } else {
                    *current = Some(Host::open(cfg).await.map_err(|_| Error::Unavailable)?);
                }
            } else {
                *current = Some(Host::open(cfg).await.map_err(|_| Error::Unavailable)?);
            }
        } else {
            *current = None;
        }
        *self.desired.write().map_err(|_| Error::Unavailable)? = next;
        self.running.store(current.is_some(), Ordering::SeqCst);
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
        let runtime = ManagedRuntime::new(None);
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
        let runtime = ManagedRuntime::new(None);
        assert_eq!(runtime.reload(None).await.unwrap().0, State::Applied);
        assert!(!runtime.status().busy);
        assert!(runtime.host.lock().await.is_none());
    }
}
