use super::*;
use crate::{capture_control_state::CaptureControlSnapshot, capture_worker::CaptureWorkerPhase};

pub(in crate::process) struct ReadyCapture {
    supervisor: CodexCaptureSupervisor,
    journal: CaptureControlJournal,
    generation: [u8; 16],
    identity_path: PathBuf,
    identity: Identity,
    runtime: std::sync::Arc<std::sync::OnceLock<crate::capture_runtime::CaptureRuntimeControl>>,
    imports: Option<Arc<crate::capture_management::CaptureImportManagement>>,
    recovery: std::sync::OnceLock<crate::capture_management::CaptureImportRecovery>,
}
impl ReadyCapture {
    pub(super) fn new(
        mut supervisor: CodexCaptureSupervisor,
        journal: CaptureControlJournal,
        selected: CaptureControlSnapshot,
        identity_path: PathBuf,
        identity: Identity,
        imports: Option<Arc<crate::capture_management::CaptureImportManagement>>,
    ) -> Self {
        if selected.paused {
            supervisor.pause();
        }
        Self {
            supervisor,
            journal,
            generation: selected.generation,
            identity_path,
            identity,
            runtime: Default::default(),
            imports,
            recovery: Default::default(),
        }
    }
    pub(super) fn start(mut self) -> Result<RunningCodexCapture> {
        if Identity::load(&self.identity_path)? != self.identity {
            return Err(Error::Service("capture installation changed"));
        }
        let selected = self
            .journal
            .current()
            .map_err(|_| Error::Service("capture control state unavailable"))?;
        if selected.generation != self.generation {
            return Err(Error::Service("capture control generation changed"));
        }
        if selected.paused {
            self.supervisor.pause();
        } else if self.supervisor.phase() == CaptureWorkerPhase::Paused {
            self.supervisor.resume()?;
        }
        if let Some(recovery) = self.recovery.into_inner() {
            self.supervisor.retain_import_recovery(recovery)?;
        }
        let runtime = RunningCodexCapture::start(self.supervisor)?;
        self.runtime
            .set(runtime.controls())
            .map_err(|_| Error::Service("capture control binding already selected"))?;
        Ok(runtime)
    }
    pub(super) fn management(&self) -> crate::capture_management::CaptureManagementBinding {
        crate::capture_management::CaptureManagementBinding {
            journal: self.journal.clone(),
            generation: self.generation,
            runtime: self.runtime.clone(),
            imports: self.imports.clone(),
        }
    }
    pub(super) fn bind_recovery(&self, manager: &crate::capture_management::Manager) {
        if let Some(recovery) = manager.import_recovery() {
            self.recovery.get_or_init(|| recovery);
        }
    }
}
