//! Stable management forwarding while the capture writer drains and changes.
use super::*;
use crate::{capture_admission::CaptureAdmission, capture_management::Manager};
use jj_store_model as m;
use jj_store_service::administration::RepositoryHost;
use jj_store_service::capture_management::*;
use std::path::Path;
pub(super) struct CaptureOwner {
    pub manager: Arc<tokio::sync::RwLock<Manager>>,
    runtime: tokio::sync::Mutex<Option<capture::RunningCapture>>,
    retention: std::sync::Mutex<Option<crate::storage::CaptureStorageLifetime>>,
    host: RepositoryHost,
    store: Arc<jj_store_object::s3::S3Store>,
    admission: CaptureAdmission,
    identity: Identity,
    path: std::path::PathBuf,
    effective: std::sync::RwLock<Config>,
    changes: jj_store_service::freshness::Signals,
    bridge: std::sync::Mutex<Option<tokio::task::JoinHandle<()>>>,
}
impl CaptureOwner {
    pub fn new(
        manager: Manager,
        storage: &AuthoritativeStorage,
        host: RepositoryHost,
        admission: CaptureAdmission,
        identity: Identity,
        path: &Path,
        config: &Config,
    ) -> Arc<Self> {
        let owner = Arc::new(Self {
            manager: Arc::new(tokio::sync::RwLock::new(manager)),
            runtime: Default::default(),
            retention: std::sync::Mutex::new(Some(storage.capture_lifetime())),
            host,
            store: storage.store(),
            admission,
            identity,
            path: path.to_owned(),
            effective: std::sync::RwLock::new(config.clone()),
            changes: Default::default(),
            bridge: Default::default(),
        });
        if let Ok(manager) = owner.manager.try_read() {
            owner.bridge(&manager);
        }
        owner
    }
    fn bridge(&self, manager: &Manager) {
        if let Ok(mut previous) = self.bridge.lock() {
            if let Some(task) = previous.take() {
                task.abort();
            }
            if let Some(mut changes) = manager.freshness() {
                let target = self.changes.clone();
                *previous = Some(tokio::spawn(async move {
                    loop {
                        match changes.recv().await {
                            Ok(change) => target.send(change),
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => target
                                .notify(None, jj_store_service::freshness::Domain::Capture, None),
                            Err(_) => break,
                        }
                    }
                }));
            }
        }
    }
    pub async fn attach(&self, runtime: capture::RunningCapture) {
        *self.runtime.lock().await = Some(runtime);
    }
    pub async fn reload(&self, config: &Config) -> crate::Result<()> {
        let mut manager = self.manager.write().await;
        let _gate = manager
            .reload_permit()
            .await
            .map_err(|_| crate::Error::Service("capture operation busy"))?;
        let mut runtime = self.runtime.lock().await;
        if let Some(old) = runtime.as_mut() {
            old.shutdown().await?;
        }
        let previous = self
            .effective
            .read()
            .map_err(|_| crate::Error::Service("capture configuration"))?
            .clone();
        match self.prepare(config) {
            Ok((next_runtime, next)) => {
                *runtime = Some(next_runtime);
                self.bridge(&next);
                *manager = next;
                *self
                    .effective
                    .write()
                    .map_err(|_| crate::Error::Service("capture configuration"))? = config.clone();
                Ok(())
            }
            Err(error) => {
                if let Ok((previous_runtime, previous_manager)) = self.prepare(&previous) {
                    *runtime = Some(previous_runtime);
                    self.bridge(&previous_manager);
                    *manager = previous_manager;
                }
                Err(error)
            }
        }
    }
    fn prepare(&self, config: &Config) -> crate::Result<(capture::RunningCapture, Manager)> {
        let prepared = if config.agent_capture.enabled {
            let lifetime = self
                .retention
                .lock()
                .map_err(|_| crate::Error::Service("capture retention"))?
                .as_ref()
                .ok_or(crate::Error::Service("capture stopped"))?
                .fork();
            capture::PreparedCapture::Ready(Box::new(capture::prepare_retained(
                config,
                &self.identity,
                &self.host,
                self.admission.clone(),
                self.store.clone(),
                lifetime,
                capture::CaptureLocations::environment,
                true,
            )?))
        } else {
            capture::PreparedCapture::Disabled
        };
        let manager = prepared.management(&self.path, config, &self.identity)?;
        let runtime = prepared.start();
        if matches!(runtime, capture::RunningCapture::Unavailable) {
            return Err(crate::Error::Service("capture replacement unavailable"));
        }
        Ok((runtime, manager))
    }
    pub async fn shutdown(&self) -> crate::Result<()> {
        if let Some(task) = self
            .bridge
            .lock()
            .map_err(|_| crate::Error::Service("capture bridge"))?
            .take()
        {
            task.abort();
        }
        let result = if let Some(runtime) = self.runtime.lock().await.as_mut() {
            runtime.shutdown().await
        } else {
            Ok(())
        };
        *self.runtime.lock().await = None;
        self.retention
            .lock()
            .map_err(|_| crate::Error::Service("capture retention"))?
            .take();
        result
    }
}
#[async_trait::async_trait]
impl CaptureManager for CaptureOwner {
    fn freshness(
        &self,
    ) -> Option<tokio::sync::broadcast::Receiver<jj_store_service::freshness::Change>> {
        Some(self.changes.subscribe())
    }
    async fn settings(
        &self,
        a: m::PrincipalId,
    ) -> std::result::Result<LocalCaptureSettings, jj_store_service::capture_management::Error>
    {
        self.manager.read().await.settings(a).await
    }
    async fn save_settings(
        &self,
        a: m::PrincipalId,
        r: SaveCaptureSettings,
    ) -> std::result::Result<LocalCaptureSettings, jj_store_service::capture_management::Error>
    {
        self.manager.read().await.save_settings(a, r).await
    }
    async fn import(
        &self,
        a: m::PrincipalId,
        r: CaptureImportCommand,
    ) -> std::result::Result<CaptureImportOutcome, jj_store_service::capture_management::Error>
    {
        self.manager.read().await.import(a, r).await
    }
    async fn import_status(
        &self,
        a: m::PrincipalId,
        r: CaptureImportQuery,
    ) -> std::result::Result<CaptureImportStatus, jj_store_service::capture_management::Error> {
        self.manager.read().await.import_status(a, r).await
    }
    async fn status(
        &self,
        a: m::PrincipalId,
    ) -> std::result::Result<CaptureStatus, jj_store_service::capture_management::Error> {
        self.manager.read().await.status(a).await
    }
    async fn sources(
        &self,
        a: m::PrincipalId,
        r: CaptureSourceQuery,
    ) -> std::result::Result<CaptureSourcePage, jj_store_service::capture_management::Error> {
        self.manager.read().await.sources(a, r).await
    }
    async fn control(
        &self,
        a: m::PrincipalId,
        r: CaptureControlCommand,
    ) -> std::result::Result<CaptureControlOutcome, jj_store_service::capture_management::Error>
    {
        self.manager.read().await.control(a, r).await
    }
    async fn rescan(
        &self,
        a: m::PrincipalId,
        r: CaptureRescanCommand,
    ) -> std::result::Result<CaptureRescanOutcome, jj_store_service::capture_management::Error>
    {
        self.manager.read().await.rescan(a, r).await
    }
}
