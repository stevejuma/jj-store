//! Current installation Admin authority and durable policy; no drain or storage owner.
use crate::{
    capture_control_state::{CaptureControlChange, CaptureControlJournal, CaptureControlSelection},
    capture_runtime::CaptureRuntimeControl,
};
use jj_store_model::PrincipalId;
use jj_store_service::capture_management::*;
use std::{
    path::Path,
    sync::{Arc, OnceLock},
};
pub(crate) mod authority;
mod import_dispatch;
mod import_policy;
mod import_progress;
mod import_recovery;
mod import_renewal;
mod import_settlement;
mod imports;
mod observation;
mod report;
mod rescan;
mod settings;
mod sources;
pub use import_dispatch::CaptureImportDispatch;
pub use import_policy::CaptureImportPolicy;
pub use import_recovery::CaptureImportRecovery;
pub(crate) use import_renewal::CaptureImportRenewal;
pub use import_settlement::CaptureImportSettlement;

#[derive(Clone)]
pub struct CaptureManagementBinding {
    pub journal: CaptureControlJournal,
    pub generation: [u8; 16],
    pub runtime: Arc<OnceLock<CaptureRuntimeControl>>,
    pub imports: Option<Arc<CaptureImportManagement>>,
}
/// Metadata handle sharing the existing spool's FIFO group; no execution owner.
pub struct CaptureImportManagement {
    pub journal: crate::capture_import_state::CaptureImportJournal,
    spool: jj_store_capture::agent_capture::AgentCaptureSpool,
    spool_binding: jj_store_capture::CaptureSpoolBinding,
}
impl CaptureImportManagement {
    pub fn new(
        journal: crate::capture_import_state::CaptureImportJournal,
        spool: &jj_store_capture::agent_capture::AgentCaptureSpool,
    ) -> Result<Self, Error> {
        Ok(Self {
            journal,
            spool: spool.fork_handle(),
            spool_binding: spool.binding().map_err(|_| Error::Unavailable)?,
        })
    }
    fn check(&self) -> Result<(), Error> {
        if self.spool.binding().map_err(|_| Error::Unavailable)? != self.spool_binding {
            return Err(Error::Unavailable);
        }
        Ok(())
    }
}
struct Inner {
    changes: jj_store_service::freshness::Signals,
    freshness_started: std::sync::OnceLock<()>,
    authority: authority::Authority,
    settings: settings::SettingsFile,
    binding: Option<CaptureManagementBinding>,
    gate: Arc<tokio::sync::Mutex<()>>,
    #[cfg(test)]
    after_selection: std::sync::Mutex<Option<Box<dyn FnOnce() + Send>>>,
}
#[derive(Clone)]
pub struct Manager(Arc<Inner>);
impl Manager {
    /// Only configuration path selection; no capture environment/state I/O.
    pub fn new(
        path: &Path,
        config: &crate::config::Config,
        identity: &crate::installation::Identity,
        binding: Option<CaptureManagementBinding>,
    ) -> Result<Self, Error> {
        Ok(Self(Arc::new(Inner {
            changes: Default::default(),
            freshness_started: Default::default(),
            authority: authority::Authority::new(path, config, identity)?,
            settings: settings::SettingsFile::new(path, &config.agent_capture)?,
            binding,
            gate: Arc::new(tokio::sync::Mutex::new(())),
            #[cfg(test)]
            after_selection: Default::default(),
        })))
    }
    pub(crate) async fn reload_permit(&self) -> Result<tokio::sync::OwnedMutexGuard<()>, Error> {
        self.observation_permit().await
    }
    fn status_selected(
        &self,
        selected: Option<crate::capture_control_state::CaptureControlSnapshot>,
    ) -> CaptureStatus {
        CaptureStatus {
            version: 1,
            enabled: selected.is_some(),
            policy: selected.map(report::policy),
            runtime: self
                .0
                .binding
                .as_ref()
                .and_then(|b| b.runtime.get())
                .and_then(|r| {
                    r.snapshot()
                        .ok()
                        .map(|snapshot| report::runtime(snapshot, r.configured_roots()))
                }),
        }
    }
    fn disabled() -> CaptureStatus {
        CaptureStatus {
            version: 1,
            enabled: false,
            policy: None,
            runtime: None,
        }
    }
    fn binding(&self) -> Result<&CaptureManagementBinding, Error> {
        self.0.binding.as_ref().ok_or(Error::Unavailable)
    }
    fn apply(
        &self,
        actor: PrincipalId,
        selected: &CaptureControlSelection,
        check: &dyn Fn() -> crate::capture_control_state::Result<()>,
    ) -> Result<CaptureControlOutcome, Error> {
        #[cfg(test)]
        let hook = self.0.after_selection.lock().unwrap().take();
        #[cfg(test)]
        if let Some(hook) = hook {
            hook();
        }
        if self.0.authority.admit(actor) != Ok(true) {
            return Err(Error::Unknown);
        }
        check().map_err(|_| Error::Unknown)?;
        let binding = self.binding()?;
        let requested = binding.runtime.get().is_some_and(|runtime| {
            (if selected.current.paused {
                runtime.pause()
            } else {
                runtime.resume()
            })
            .is_ok()
        });
        self.0.changes.notify(
            None,
            jj_store_service::freshness::Domain::Capture,
            Some(selected.receipt.request.to_string()),
        );
        Ok(CaptureControlOutcome {
            version: 1,
            receipt: CapturePolicyReceipt {
                generation: hex::encode(selected.current.generation),
                request_id: selected.receipt.request,
                expected_revision: selected.receipt.expected_revision,
                revision: selected.receipt.revision,
                paused: selected.receipt.paused,
            },
            replayed: selected.replayed,
            runtime_requested: requested,
            status: self.status_selected(Some(selected.current)),
        })
    }
}

#[cfg(all(test, unix))]
mod tests;
#[async_trait::async_trait]
impl CaptureManager for Manager {
    fn freshness(
        &self,
    ) -> Option<tokio::sync::broadcast::Receiver<jj_store_service::freshness::Change>> {
        if let Some(runtime) = self.0.binding.as_ref().and_then(|b| b.runtime.get()) {
            self.0
                .freshness_started
                .get_or_init(|| runtime.follow_freshness(self.0.changes.clone()));
        }
        Some(self.0.changes.subscribe())
    }
    async fn settings(&self, actor: PrincipalId) -> Result<LocalCaptureSettings, Error> {
        self.read_settings(actor).await
    }
    async fn save_settings(
        &self,
        actor: PrincipalId,
        request: SaveCaptureSettings,
    ) -> Result<LocalCaptureSettings, Error> {
        self.write_settings(actor, request).await
    }
    async fn import(
        &self,
        actor: PrincipalId,
        request: CaptureImportCommand,
    ) -> Result<CaptureImportOutcome, Error> {
        let result = self.request_import(actor, request).await?;
        self.0
            .changes
            .notify(None, jj_store_service::freshness::Domain::Capture, None);
        Ok(result)
    }
    async fn import_status(
        &self,
        actor: PrincipalId,
        query: CaptureImportQuery,
    ) -> Result<CaptureImportStatus, Error> {
        self.read_import_status(actor, query).await
    }
    async fn sources(
        &self,
        actor: PrincipalId,
        query: CaptureSourceQuery,
    ) -> Result<CaptureSourcePage, Error> {
        self.read_sources(actor, query).await
    }
    async fn rescan(
        &self,
        actor: PrincipalId,
        request: CaptureRescanCommand,
    ) -> Result<CaptureRescanOutcome, Error> {
        self.request_rescan(actor, request).await
    }
    async fn status(&self, actor: PrincipalId) -> Result<CaptureStatus, Error> {
        let this = self.clone();
        let permit = self.observation_permit().await?;
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            if !this.0.authority.admit(actor)? {
                return Ok(Self::disabled());
            }
            let binding = this.binding()?;
            let selected = binding.journal.current().map_err(report::failure)?;
            if selected.generation != binding.generation {
                return Err(Error::Unavailable);
            }
            if this.0.authority.admit(actor) != Ok(true) {
                return Err(Error::Denied);
            }
            Ok(this.status_selected(Some(selected)))
        })
        .await
        .map_err(|_| Error::Unavailable)?
    }
    async fn control(
        &self,
        actor: PrincipalId,
        request: CaptureControlCommand,
    ) -> Result<CaptureControlOutcome, Error> {
        if request.version != 1
            || request.request_id.as_bytes() == &[0; 16]
            || request.generation.len() != 32
            || request
                .generation
                .bytes()
                .any(|b| !b.is_ascii_digit() && !(b'a'..=b'f').contains(&b))
        {
            return Err(Error::Invalid);
        }
        let generation: [u8; 16] = hex::decode(&request.generation)
            .map_err(|_| Error::Invalid)?
            .try_into()
            .map_err(|_| Error::Invalid)?;
        if generation == [0; 16] {
            return Err(Error::Invalid);
        }
        let this = self.clone();
        let permit = self
            .0
            .gate
            .clone()
            .try_lock_owned()
            .map_err(|_| Error::Busy)?;
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            if !this.0.authority.admit(actor)? {
                return Err(Error::Disabled);
            }
            let binding = this.binding()?;
            if generation != binding.generation {
                return Err(Error::Stale);
            }
            let (_, applied) = binding
                .journal
                .record_then(
                    &CaptureControlChange {
                        generation,
                        principal: actor,
                        request: request.request_id,
                        expected_revision: request.expected_revision,
                        paused: request.action == CaptureControlAction::Pause,
                    },
                    |selected, check| this.apply(actor, selected, check),
                )
                .map_err(report::failure)?;
            applied
        })
        .await
        .map_err(|_| Error::Unknown)?
    }
}
